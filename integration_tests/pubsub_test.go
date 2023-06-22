package integration_tests

import (
	"bytes"
	"context"
	"fmt"
	"io/ioutil"
	"math/big"
	"strings"
	"time"

	"github.com/cosmos/cosmos-sdk/client"
	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ory/dockertest/v3/docker"
	pubsubtypes "github.com/peggyjv/sommelier/v4/x/pubsub/types"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// TODO: This test is for development purposes. Pubsub interaction is probably something that will need to be added to the cork test
// since it is prerequisite to any operations triggered by a strategy provider.
func (s *IntegrationTestSuite) TestPubsub() {
	s.T().Log("starting pubsub test")
	// Verify no default subscriptions
	val := s.chain.validators[0]
	kb, err := val.keyring()
	s.Require().NoError(err)
	clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
	s.Require().NoError(err)
	pubsubClient := pubsubtypes.NewQueryClient(clientCtx)

	// Verify cannot make authenticated call to Steward
	request := &steward_proto.ScheduleRequest{
		CellarId:    "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707",
		BlockHeight: 0,
		CallData:    nil,
	}
	// steward0 := s.chain.stewards[0]

	// s.Require().Eventually(func() bool {
	// 	_, err := steward0.schedule(request)
	// 	s.Require().Error(err)

	// 	return true
	// }, time.Second*10, time.Second, "schedule call took too long")

	// s.watchForLog("steward0", "Sending fatal alert HandshakeFailure")

	// AddDefaulSubscriptionProposal
	orch := s.chain.orchestrators[0]
	orchClientCtx, err := s.chain.clientContext("tcp://localhost:26657", orch.keyring, "orch", orch.keyInfo.GetAddress())
	s.Require().NoError(err)
	proposal := pubsubtypes.NewAddDefaultSubscriptionProposal(
		"add default subscription",
		"adds a default subscription for steward",
		aaveCellar.Hex(),
		"localhost",
	)
	proposalMsg, err := govtypes.NewMsgSubmitProposal(
		proposal,
		sdk.Coins{
			{
				Denom:  testDenom,
				Amount: sdk.NewInt(1000000),
			},
		},
		orch.keyInfo.GetAddress(),
	)
	s.Require().NoError(err, "Unable to create governance proposal")

	s.runProposal(orchClientCtx, proposalMsg, "/pubsub.v1.AddDefaultSubscriptionProposal")

	caCert, err := ioutil.ReadFile("integration_tests/tls/client/test_client_ca.crt")
	s.Require().NoError(err)

	publisherProposal := pubsubtypes.NewAddPublisherProposal(
		"add publisher",
		"add localhost publisher",
		"localhost",
		orch.keyInfo.GetAddress().String(),
		"https://localhost/somm1p78uz3z9sgav0m325y0tmzzx3a56h583t3x7cx/cacert.pem",
		string(caCert),
	)
	proposalMsg, err = govtypes.NewMsgSubmitProposal(
		publisherProposal,
		sdk.Coins{
			{
				Denom:  testDenom,
				Amount: sdk.NewInt(1000000),
			},
		},
		orch.keyInfo.GetAddress(),
	)
	s.Require().NoError(err, "Unable to create governance proposal")

	s.runProposal(orchClientCtx, proposalMsg, "/pubsub.v1.AddPublisherProposal")

	// Verify default subscription added
	response, err := pubsubClient.QueryDefaultSubscriptions(context.Background(), &pubsubtypes.QueryDefaultSubscriptionsRequest{})
	s.Require().NoError(err)
	s.Require().Greater(len(response.DefaultSubscriptions), 0)

	// Make an authenticated call to Steward
	s.checkCellarExists(aaveCellar)

	cellarId := aaveCellar.String()

	currentHeight, err := s.GetLatestBlockHeight(clientCtx)
	s.Require().NoError(err)
	scheduledHeight := currentHeight + 30

	// Create the cork request to send to Steward
	route := []string{T_AAVE_DAI, T_AAVE_POOL, T_AAVE_USDC, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS}
	swapParams := []*steward_proto.AaveV2Stablecoin_Rebalance_SwapParams{
		{InIndex: 0, OutIndex: 2, SwapType: 1},
		{InIndex: 0, OutIndex: 0, SwapType: 0},
		{InIndex: 0, OutIndex: 0, SwapType: 0},
		{InIndex: 0, OutIndex: 0, SwapType: 0},
	}
	request = &steward_proto.ScheduleRequest{
		CellarId: cellarId,
		CallData: &steward_proto.ScheduleRequest_AaveV2Stablecoin{
			AaveV2Stablecoin: &steward_proto.AaveV2Stablecoin{
				Function: &steward_proto.AaveV2Stablecoin_Rebalance_{
					Rebalance: &steward_proto.AaveV2Stablecoin_Rebalance{
						Route:        route,
						SwapParams:   swapParams,
						MinAssetsOut: T_AAVE_MIN_ASSETS_OUT,
					},
				},
			},
		},
		BlockHeight: uint64(scheduledHeight),
	}

	s.executeStewardCalls(request)

	// Construct invalidation scope and nonce for gravity query
	aave_abi, err := AaveV2MetaData.GetAbi()
	s.Require().NoError(err)

	methodName := "rebalance"
	zeroAddress := common.HexToAddress(ZERO_ADDRESS)
	solRoute := [9]common.Address{common.HexToAddress(T_AAVE_DAI), common.HexToAddress(T_AAVE_POOL), common.HexToAddress(T_AAVE_USDC), zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress}
	zero := big.NewInt(0)
	solSwapParams := &[4][3]*big.Int{{zero, big.NewInt(2), big.NewInt(1)}, {zero, zero, zero}, {zero, zero, zero}, {zero, zero, zero}}
	minAssetsOut := big.NewInt(1000000000000000000)
	method, err := aave_abi.Pack(methodName, solRoute, solSwapParams, minAssetsOut)
	addr := common.HexToAddress(aaveCellar.String())
	invalidationScope := crypto.Keccak256Hash(
		bytes.Join(
			[][]byte{addr.Bytes(), method},
			[]byte{},
		)).Bytes()
	invalidationNonce := 1
	s.Require().NoError(err)

	s.queryLogicCallTransaction(clientCtx, invalidationScope, invalidationNonce)

	// For non-anonymous events, the first log topic is a keccak256 hash o	f the
	// event signature.
	eventSignature := []byte("LogicCallEvent(bytes32,uint256,bytes,uint256)")
	logicCallEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
	s.waitForGravityLogicCallEvent(logicCallEventSignatureTopic, invalidationScope, invalidationNonce)

	s.T().Logf("checking for cellar event")
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying cellar events...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// For non-anonymous events, the first log topic is a keccak256 hash of the
		// event signature.
		eventSignature := []byte("mockRebalance(address[9],uint256[3][4],uint256)")
		mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
		query := ethereum.FilterQuery{
			FromBlock: nil,
			ToBlock:   nil,
			Addresses: []common.Address{
				aaveCellar,
			},
			Topics: [][]common.Hash{
				{
					mockEventSignatureTopic,
				},
			},
		}

		logs, err := ethClient.FilterLogs(context.Background(), query)
		if err != nil {
			ethClient.Close()
			return false
		}

		s.T().Logf("got %v logs", len(logs))
		if len(logs) == 1 {
			s.T().Log("saw mock function event!")

			log := logs[0]
			if len(log.Data) > 0 {
				var event AaveV2MockRebalance
				err := aave_abi.UnpackIntoInterface(&event, "mockRebalance", log.Data)
				s.Require().NoError(err, "failed to unpack mockRebalance event from log data")
				s.Require().Equal(event.MinAssetsOut, minAssetsOut)
				return true
			}
		}

		return false
	}, 2*time.Minute, 5*time.Second, "cellar event never seen")
}

func (s *IntegrationTestSuite) watchForLog(container string, expectedString string) {
	stewardLogOutput := bytes.Buffer{}
	err := s.dockerPool.Client.Logs(docker.LogsOptions{
		Container:    container,
		OutputStream: &stewardLogOutput,
		Stdout:       true,
	})
	s.Require().NoError(err, "error getting steward0 logs")

	s.Require().Eventuallyf(func() bool {
		for _, line := range strings.Split(stewardLogOutput.String(), "\n") {
			if strings.Contains(line, expectedString) {
				return true
			}
		}

		return false
	}, time.Second*20, time.Second*2, "unable to retrieve tls warning from logs")
}

func (s *IntegrationTestSuite) runProposal(clientCtx *client.Context, proposal *govtypes.MsgSubmitProposal, typeUrl string) {
	s.T().Log("submitting proposal")
	submitProposalResponse, err := s.chain.sendMsgs(*clientCtx, proposal)
	s.Require().NoError(err)
	s.Require().Zero(submitProposalResponse.Code, "raw log: %s", submitProposalResponse.RawLog)

	s.T().Log("verifying proposal was submitted correctly")
	govQueryClient := govtypes.NewQueryClient(clientCtx)
	var proposalID uint64
	s.Require().Eventually(func() bool {
		proposalsQueryResponse, err := govQueryClient.Proposals(context.Background(), &govtypes.QueryProposalsRequest{})
		if err != nil {
			s.T().Logf("error querying proposals: %e", err)
			return false
		}

		s.Require().NotEmpty(proposalsQueryResponse.Proposals)
		for _, p := range proposalsQueryResponse.Proposals {
			if p.Content.TypeUrl == typeUrl {
				s.Require().Equal(govtypes.StatusVotingPeriod, p.Status, "proposal not in voting period")
				proposalID = p.ProposalId
			}
		}

		return proposalID > 0
	}, time.Second*30, time.Second*5, "proposal submission was never found")

	s.T().Log("voting for proposal")
	// wait so the client for val0 will be aware of the latest tx sequence
	time.Sleep(time.Second * 10)
	for _, val := range s.chain.validators {
		kr, err := val.keyring()
		s.Require().NoError(err)
		localClientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kr, "val", val.keyInfo.GetAddress())
		s.Require().NoError(err)

		voteMsg := govtypes.NewMsgVote(val.keyInfo.GetAddress(), proposalID, govtypes.OptionYes)
		voteResponse, err := s.chain.sendMsgs(*localClientCtx, voteMsg)
		s.Require().NoError(err)
		s.Require().Zero(voteResponse.Code, "Vote error: %s", voteResponse.RawLog)
	}

	s.T().Log("waiting for proposal to be approved..")
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := govQueryClient.Proposal(context.Background(), &govtypes.QueryProposalRequest{ProposalId: proposalID})
		return govtypes.StatusPassed == proposalQueryResponse.Proposal.Status
	}, time.Second*30, time.Second*5, "proposal was never accepted")
	s.T().Log("proposal approved!")
}
