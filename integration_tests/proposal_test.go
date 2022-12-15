package integration_tests

import (
	"context"
	"fmt"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	govtypes "github.com/cosmos/cosmos-sdk/x/gov/types"
	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/peggyjv/sommelier/v4/x/cork/types"
	corktypes "github.com/peggyjv/sommelier/v4/x/cork/types"
)

func (s *IntegrationTestSuite) TestScheduledCorkProposal() {
	s.checkCellarExists(vaultCellar)

	orch := s.chain.orchestrators[0]
	orchClientCtx, err := s.chain.clientContext("tcp://localhost:26657", orch.keyring, "orch", orch.keyInfo.GetAddress())
	s.Require().NoError(err)
	currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
	s.Require().NoError(err)
	protoJson := `
	{
		"call": {
			"CellarV1": {
				"function": {
					"TrustPosition" : {
						"position": {
							"Erc20Address": "0x0000000000000000000000000000000000000000"
						}
					}
				}
			}
		}
	}
	`
	targetBlockHeight := currentHeight + 120
	proposal := corktypes.NewScheduledCorkProposal(
		"scheduled cork proposal test",
		"description",
		uint64(targetBlockHeight),
		vaultCellar.String(),
		protoJson,
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

	s.T().Log("Submit proposal")
	submitProposalResponse, err := s.chain.sendMsgs(*orchClientCtx, proposalMsg)
	s.Require().NoError(err)
	s.Require().Zero(submitProposalResponse.Code, "raw log: %s", submitProposalResponse.RawLog)

	s.T().Log("Check proposal was submitted correctly")
	govQueryClient := govtypes.NewQueryClient(orchClientCtx)
	s.Require().Eventually(func() bool {
		proposalsQueryResponse, err := govQueryClient.Proposals(context.Background(), &govtypes.QueryProposalsRequest{})
		if err != nil {
			s.T().Logf("error querying proposals: %e", err)
			return false
		}

		s.Require().NotEmpty(proposalsQueryResponse.Proposals)
		s.Require().Equal(uint64(1), proposalsQueryResponse.Proposals[0].ProposalId, "not proposal id 1")
		s.Require().Equal(govtypes.StatusVotingPeriod, proposalsQueryResponse.Proposals[0].Status, "proposal not in voting period")

		return true
	}, time.Second*30, time.Second*5, "proposal submission was never found")

	s.T().Log("Vote for proposal")
	for _, val := range s.chain.validators {
		kr, err := val.keyring()
		s.Require().NoError(err)
		localClientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kr, "val", val.keyInfo.GetAddress())
		s.Require().NoError(err)

		voteMsg := govtypes.NewMsgVote(val.keyInfo.GetAddress(), 1, govtypes.OptionYes)
		voteResponse, err := s.chain.sendMsgs(*localClientCtx, voteMsg)
		s.Require().NoError(err)
		s.Require().Zero(voteResponse.Code, "Vote error: %s", voteResponse.RawLog)
	}

	s.T().Log("Waiting for proposal to be approved..")
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := govQueryClient.Proposal(context.Background(), &govtypes.QueryProposalRequest{ProposalId: 1})
		return govtypes.StatusPassed == proposalQueryResponse.Proposal.Status
	}, time.Second*30, time.Second*5, "proposal was never accepted")
	s.T().Log("Proposal approved!")

	s.T().Log("Waiting for scheduled cork to be created by steward")
	corkQueryClient := corktypes.NewQueryClient(orchClientCtx)
	s.Require().Eventually(func() bool {
		proposalQueryResponse, _ := corkQueryClient.QueryScheduledCorks(context.Background(), &corktypes.QueryScheduledCorksRequest{})
		return len(proposalQueryResponse.Corks) > 0
	}, time.Second*120, time.Second*2, "corks never scheduled")

	s.T().Log("wait for scheduled height")
	s.Require().Eventuallyf(func() bool {
		currentHeight, err := s.GetLatestBlockHeight(orchClientCtx)
		s.Require().NoError(err)
		if currentHeight >= targetBlockHeight {
			return true
		} else {
			res, err := corkQueryClient.QueryScheduledCorks(context.Background(), &types.QueryScheduledCorksRequest{})
			if err != nil {
				s.T().Logf("error: %s", err)
				return false
			}

			// verify that the scheduled corks have not yet been consumed
			s.Require().Len(res.Corks, len(s.chain.validators))
		}

		return false
	}, 3*time.Minute, 1*time.Second, "never reached scheduled height")

	s.T().Logf("checking for cellar event")
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying cellar events...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		if err != nil {
			return false
		}

		// For non-anonymous events, the first log topic is a keccak256 hash of the
		// event signature.
		eventSignature := []byte("TrustChanged(address,bool)")
		mockEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
		query := ethereum.FilterQuery{
			FromBlock: nil,
			ToBlock:   nil,
			Addresses: []common.Address{
				vaultCellar,
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

		vault_abi, err := CellarMetaData.GetAbi()
		s.Require().NoError(err)

		s.T().Logf("got %v logs: %v", len(logs), logs)
		if len(logs) > 0 {
			for _, log := range logs {
				if len(log.Data) > 0 {
					var event CellarTrustChanged
					err := vault_abi.UnpackIntoInterface(&event, "TrustChanged", log.Data)
					s.Require().NoError(err, "failed to unpack TrustChanged event from log data")

					return true
				}
			}
		}

		return false
	}, 3*time.Minute, 10*time.Second, "cellar event never seen")
}
