package integration_tests

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"math/big"
	"time"

	"github.com/cosmos/cosmos-sdk/types/query"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	gravityTypes "github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
	corkTypes "github.com/peggyjv/sommelier/v4/x/cork/types"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

// Steward/Cork integration tests follow the same basic pattern:
// 1. Query the cork module to ensure the relevant cellar ID is approved
// 2. Wait for the next cork vote period to begin
// 3. Construct and send the relevant cork request to each steward instance
// 4. Wait for the voting period to end so the cork will be submitted to the gravity module as a ContractCallTx
// 5. Query the gravity module to ensure the ContractCallTx was created
// 6. Wait for the LogicCallSubmitted event to occur in the gravity contract, indicating that the orchestrators
//	  relayed the ContractCallTx successfully.
// 7. Wait for the presumed Cellar contract event indicating a successful decoding of the function call corked by Steward.

const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"
const ONE_ADDRESS = "0x1111111111111111111111111111111111111111"

// AaveV2Stablecoin rebalance test values used in multiple methods
const T_AAVE_DAI = "0x6B175474E89094C44Da98b954EedeAC495271d0F"
const T_AAVE_POOL = "0xE340B25fE32B1011616bb8EC495A4d503e322177"
const T_AAVE_USDC = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"

// 1*10^18
const T_AAVE_MIN_ASSETS_OUT = "1000000000000000000"

func (s *IntegrationTestSuite) TestAaveV2Stablecoin() {
	s.Run("Submit rebalance request to MockAaveV2Stablecoin", func() {
		s.checkCellarExists(aaveCellar)
		s.waitForVotePeriod()

		cellarId := aaveCellar.String()

		// Create the cork request to send to Steward
		route := []string{T_AAVE_DAI, T_AAVE_POOL, T_AAVE_USDC, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS, ZERO_ADDRESS}
		swapParams := []*steward_proto.AaveV2Stablecoin_Rebalance_SwapParams{
			{InIndex: 0, OutIndex: 2, SwapType: 1},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
			{InIndex: 0, OutIndex: 0, SwapType: 0},
		}
		request := &steward_proto.SubmitRequest{
			CellarId: cellarId,
			CallData: &steward_proto.SubmitRequest_AaveV2Stablecoin{
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
		}
		s.executeStewardCalls(request)
		s.waitForVotePeriod()

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

		s.queryLogicCallTransaction(invalidationScope, invalidationNonce)

		// For non-anonymous events, the first log topic is a keccak256 hash o	f the
		// event signature.
		eventSignature := []byte("LogicCallEvent(bytes32,uint256,bytes,uint256)")
		logicCallEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
		s.waitForGravityLogicCallEvent(logicCallEventSignatureTopic, invalidationScope, invalidationNonce)

		s.T().Logf("checking for cellar event")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying cellar events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			s.Require().NoError(err)

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
			s.Require().NoError(err)
			ethClient.Close()

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
	})
}

func (s *IntegrationTestSuite) TestCellarV1() {
	s.Run("Submit rebalance to MockCellar", func() {
		s.checkCellarExists(vaultCellar)

		// Create the cork requests to send to Steward
		from := ZERO_ADDRESS
		to := ONE_ADDRESS
		assetsFrom := "1000"
		cellarId := vaultCellar.String()
		swapParamsUniV2 := &steward_proto.CellarV1_SwapParams{
			Params: &steward_proto.CellarV1_SwapParams_Univ2Params{
				Univ2Params: &steward_proto.CellarV1_UniV2SwapParams{
					Path:         []string{ZERO_ADDRESS, ONE_ADDRESS},
					Amount:       "1000",
					AmountOutMin: "2000",
				},
			},
		}
		requestUniV2 := &steward_proto.SubmitRequest{
			CellarId: cellarId,
			CallData: &steward_proto.SubmitRequest_CellarV1{
				CellarV1: &steward_proto.CellarV1{
					Function: &steward_proto.CellarV1_Rebalance_{
						Rebalance: &steward_proto.CellarV1_Rebalance{
							FromPosition: from,
							ToPosition:   to,
							AssetsFrom:   assetsFrom,
							Exchange:     steward_proto.CellarV1_EXCHANGE_UNIV2,
							Params:       swapParamsUniV2,
						},
					},
				},
			},
		}
		swapParamsUniV3 := &steward_proto.CellarV1_SwapParams{
			Params: &steward_proto.CellarV1_SwapParams_Univ3Params{
				Univ3Params: &steward_proto.CellarV1_UniV3SwapParams{
					Path:         []string{ZERO_ADDRESS, ONE_ADDRESS},
					PoolFees:     []uint32{0, 0},
					Amount:       "1000",
					AmountOutMin: "2000",
				},
			},
		}
		requestUniV3 := &steward_proto.SubmitRequest{
			CellarId: cellarId,
			CallData: &steward_proto.SubmitRequest_CellarV1{
				CellarV1: &steward_proto.CellarV1{
					Function: &steward_proto.CellarV1_Rebalance_{
						Rebalance: &steward_proto.CellarV1_Rebalance{
							FromPosition: from,
							ToPosition:   to,
							AssetsFrom:   assetsFrom,
							Exchange:     steward_proto.CellarV1_EXCHANGE_UNIV3,
							Params:       swapParamsUniV3,
						},
					},
				},
			},
		}

		s.T().Log("running through two sequences to test rebalance with both UniV2 and Univ3 swap params")
		for sequence, request := range []*steward_proto.SubmitRequest{requestUniV2, requestUniV3} {
			s.T().Log("starting sequence")
			s.waitForVotePeriod()
			s.executeStewardCalls(request)
			s.waitForVotePeriod()

			// Construct invalidation scope and nonce for gravity query
			vault_abi, err := CellarMetaData.GetAbi()
			s.Require().NoError(err)

			methodName := "rebalance"
			addressArrayT, _ := abi.NewType("address[]", "address[]", nil)
			feeArrayT, _ := abi.NewType("uint32[]", "uint32[]", nil)
			uint256T, _ := abi.NewType("uint256", "uint256", nil)

			var args abi.Arguments
			var params []byte
			if sequence == 0 {
				args = abi.Arguments{
					{
						Type: addressArrayT,
					},
					{
						Type: uint256T,
					},
					{
						Type: uint256T,
					},
				}

				params, err = args.Pack(
					[]common.Address{
						common.HexToAddress(ZERO_ADDRESS),
						common.HexToAddress(ONE_ADDRESS),
					},
					big.NewInt(1000),
					big.NewInt(2000),
				)
				s.Require().NoError(err)
			} else {
				args = abi.Arguments{
					{
						Type: addressArrayT,
					},
					{
						Type: feeArrayT,
					},
					{
						Type: uint256T,
					},
					{
						Type: uint256T,
					},
				}

				params, err = args.Pack(
					[]common.Address{
						common.HexToAddress(ZERO_ADDRESS),
						common.HexToAddress(ONE_ADDRESS),
					},
					[]uint32{0, 0},
					big.NewInt(1000),
					big.NewInt(2000),
				)
				s.Require().NoError(err)
			}
			method, err := vault_abi.Pack(methodName, common.HexToAddress(from), common.HexToAddress(to), big.NewInt(1000), uint8(sequence), params)
			s.Require().NoError(err)

			addr := common.HexToAddress(vaultCellar.String())
			invalidationScope := crypto.Keccak256Hash(
				bytes.Join(
					[][]byte{addr.Bytes(), method},
					[]byte{},
				)).Bytes()
			invalidationNonce := sequence + 1
			s.queryLogicCallTransaction(invalidationScope, invalidationNonce)

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("LogicCallEvent(bytes32,uint256,bytes,uint256)")
			logicCallEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			s.waitForGravityLogicCallEvent(logicCallEventSignatureTopic, invalidationScope, invalidationNonce)

			s.T().Logf("checking for cellar event")
			s.Require().Eventuallyf(func() bool {
				s.T().Log("querying cellar events...")
				ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
				s.Require().NoError(err)

				// For non-anonymous events, the first log topic is a keccak256 hash of the
				// event signature.
				eventSignature := []byte("Rebalance(address,address,uint256,uint8)")
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
				s.Require().NoError(err)
				ethClient.Close()

				s.T().Logf("got %v logs: %v", len(logs), logs)
				if len(logs) > 0 {
					for _, log := range logs {
						if len(log.Data) > 0 {
							var event CellarRebalance
							err := vault_abi.UnpackIntoInterface(&event, "Rebalance", log.Data)
							s.Require().NoError(err, "failed to unpack Rebalance event from log data")

							if sequence == 0 {
								if event.Exchange == 0 {
									s.T().Log("saw mock function event!")
									return true
								}
							} else {
								if event.Exchange == 1 {
									s.T().Log("saw mock function event!")
									return true
								}
							}
						}
					}
				}

				return false
			}, 2*time.Minute, 10*time.Second, "cellar event never seen")

			s.T().Log("sequence complete!")
		}
	})
}

func (s *IntegrationTestSuite) checkCellarExists(cellar common.Address) {
	s.T().Logf("checking that cellar %s exists in the chain", cellar.String())
	queryClient, err := s.chain.validators[0].GetQueryClient()
	s.Require().NoError(err, "error getting query client")
	s.Require().Eventuallyf(func() bool {
		res, err := queryClient.QueryCellarIDs(context.Background(), &corkTypes.QueryCellarIDsRequest{})
		if err != nil || res == nil {
			return false
		}

		for _, c := range res.CellarIds {
			if c == cellar.String() {
				return true
			}
		}

		return false
	}, 60*time.Second, 5*time.Second, "cellar %s not found in approved cellars list", cellar.String())
}

func (s *IntegrationTestSuite) waitForVotePeriod() {
	s.T().Logf("wait for new vote period start")
	queryClient, err := s.chain.validators[0].GetQueryClient()
	s.Require().NoError(err, "error getting query client")
	s.Require().Eventuallyf(func() bool {
		res, err := queryClient.QueryCommitPeriod(context.Background(), &corkTypes.QueryCommitPeriodRequest{})
		if err != nil {
			return false
		}
		if res.VotePeriodStart != res.CurrentHeight {
			if res.CurrentHeight%10 == 0 {
				s.T().Logf("current height: %d, period end: %d", res.CurrentHeight, res.VotePeriodEnd)
			}
			return false
		}

		return true
	}, 30*time.Second, 1*time.Second, "new vote period never seen")
}

func (s *IntegrationTestSuite) executeStewardCalls(request *steward_proto.SubmitRequest) {
	s.T().Logf("submit contract call data to steward")
	s.Require().Eventually(func() bool {
		// Load in client TLS config
		// client cert
		clientCert, err := tls.LoadX509KeyPair(
			"integration_tests/tls/client/test_client.crt",
			"integration_tests/tls/client/test_client_key_pkcs8.pem",
		)
		s.Require().NoError(err)

		// server CA
		rootPool := x509.NewCertPool()
		r, err := ioutil.ReadFile("integration_tests/tls/server/test_server_ca.crt")
		s.Require().NoError(err)

		block, _ := pem.Decode(r)
		rootCert, err := x509.ParseCertificate(block.Bytes)
		s.Require().NoError(err)

		rootPool.AddCert(rootCert)

		tlsConfig := &tls.Config{
			Certificates: []tls.Certificate{clientCert},
			RootCAs:      rootPool,
		}
		creds := credentials.NewTLS(tlsConfig)

		// make it rain
		for i := range s.chain.stewards {
			addr := fmt.Sprintf("localhost:%v", STEWARD_PORT+i)
			conn, err := grpc.Dial(
				addr,
				grpc.WithBlock(),
				grpc.WithTransportCredentials(creds),
			)
			s.Require().NoError(err)
			defer conn.Close()

			ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
			defer cancel()

			c := steward_proto.NewContractCallClient(conn)
			s.T().Logf("sending request to %s", s.chain.validators[i].keyInfo.GetAddress())
			_, err = c.Submit(ctx, request)
			s.Require().NoError(err)
		}

		return true
	}, 100*time.Second, 1*time.Second, "Cork request took too long")
}

func (s *IntegrationTestSuite) queryLogicCallTransaction(invalidationScope []byte, invalidationNonce int) {
	s.T().Log("querying gravity module for logic call transaction")
	val := s.chain.validators[0]
	kb, err := val.keyring()
	s.Require().NoError(err)

	clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
	s.Require().NoError(err)

	gravityQueryClient := gravityTypes.NewQueryClient(clientCtx)
	s.Require().Eventuallyf(func() bool {
		request := &gravityTypes.ContractCallTxsRequest{
			Pagination: &query.PageRequest{},
		}
		response, _ := gravityQueryClient.ContractCallTxs(context.Background(), request)
		if response != nil {
			for _, call := range response.Calls {
				if bytes.Equal(call.InvalidationScope, invalidationScope) && call.InvalidationNonce == uint64(invalidationNonce) {
					s.T().Log("logic call found in the gravity module!")
					return true
				}
			}
		}

		return false
	}, 1*time.Minute, 5*time.Second, "cellar event never seen")
	time.Sleep(time.Duration(2000000000))
}

func (s *IntegrationTestSuite) waitForGravityLogicCallEvent(topic common.Hash, invalidationScope []byte, invalidationNonce int) {
	s.T().Logf("waiting for gravity to submit call to cellar")
	gravity_abi, err := GravityMetaData.GetAbi()
	s.Require().NoError(err)
	s.Require().Eventuallyf(func() bool {
		s.T().Log("querying gravity logic call events...")
		ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
		s.Require().NoError(err)

		query := ethereum.FilterQuery{
			FromBlock: nil,
			ToBlock:   nil,
			Addresses: []common.Address{
				gravityContract,
			},
			Topics: [][]common.Hash{
				{
					topic,
				},
			},
		}
		logs, err := ethClient.FilterLogs(context.Background(), query)
		s.Require().NoError(err)
		ethClient.Close()

		s.T().Logf("got %v LogicCallEvent logs", len(logs))
		if len(logs) == 0 {
			return false
		}

		var event GravityLogicCallEvent
		for _, log := range logs {
			if len(log.Data) > 0 {
				err := gravity_abi.UnpackIntoInterface(&event, "LogicCallEvent", log.Data)
				if err != nil {
					continue
				}

				eventInvalidationId := event.InvalidationId[:]
				if bytes.Equal(eventInvalidationId, invalidationScope) && int(event.InvalidationNonce.Int64()) == invalidationNonce {
					s.T().Log("logic call executed!")
					return true
				}
			} else {
				s.T().Log("no data in log")
			}
		}
		return false
	}, 1*time.Minute, 5*time.Second, "cellar event never seen")
}
