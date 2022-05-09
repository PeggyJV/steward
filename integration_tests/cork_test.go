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

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	gravityTypes "github.com/peggyjv/gravity-bridge/module/v2/x/gravity/types"
	corkTypes "github.com/peggyjv/sommelier/v4/x/cork/types"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func (s *IntegrationTestSuite) TestCork() {
	s.Run("Bring up chain, and submit a cork", func() {
		s.T().Logf("checking that test cellar exists in the chain")
		val := s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			queryClient, err := val.GetQueryClient()
			s.Require().NoError(err, "error getting query client")

			res, err := queryClient.QueryCellarIDs(context.Background(), &corkTypes.QueryCellarIDsRequest{})
			if err != nil {
				return false
			}
			if res == nil {
				return false
			}
			for _, c := range res.CellarIds {
				if c == hardhatCellar.String() {
					return true
				}
			}
			return false
		}, 30*time.Second, 2*time.Second, "hardhat cellar not found in chain")

		s.T().Logf("wait for new vote period start")
		s.Require().Eventuallyf(func() bool {
			kb, err := val.keyring()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
			s.Require().NoError(err)

			queryClient := corkTypes.NewQueryClient(clientCtx)
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
		}, 200*time.Second, 1*time.Second, "new vote period never seen")

		s.T().Logf("submit contract call data to steward")
		dai := "0x6B175474E89094C44Da98b954EedeAC495271d0F"
		pool := "0xE340B25fE32B1011616bb8EC495A4d503e322177"
		usdc := "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
		zeroAddr := "0x0000000000000000000000000000000000000000"
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
				addr := fmt.Sprintf("localhost:%v", 5734+i)
				conn, err := grpc.Dial(
					addr,
					grpc.WithBlock(),
					grpc.WithTransportCredentials(creds),
				)
				s.Require().NoError(err)
				defer conn.Close()

				ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
				defer cancel()

				c := NewContractCallClient(conn)
				s.T().Logf("sending request to %s", s.chain.validators[i].keyInfo.GetAddress())
				cellarId := hardhatCellar.String()
				route := []string{dai, pool, usdc, zeroAddr, zeroAddr, zeroAddr, zeroAddr, zeroAddr, zeroAddr}
				swapParams := []*AaveV2Stablecoin_Rebalance_SwapParams{
					{InIndex: 0, OutIndex: 2, SwapType: 1},
					{InIndex: 0, OutIndex: 0, SwapType: 0},
					{InIndex: 0, OutIndex: 0, SwapType: 0},
					{InIndex: 0, OutIndex: 0, SwapType: 0},
				}
				minAssetsOut := uint64(1000)
				request := SubmitRequest{
					CellarId: cellarId,
					CallData: &SubmitRequest_AaveV2Stablecoin{
						&AaveV2Stablecoin{
							Function: &AaveV2Stablecoin_Rebalance_{
								&AaveV2Stablecoin_Rebalance{
									Route:        route,
									SwapParams:   swapParams,
									MinAssetsOut: minAssetsOut,
								},
							},
						},
					},
				}
				_, err = c.Submit(ctx, &request)
				s.Require().NoError(err)
			}

			return true
		}, 100*time.Second, 1*time.Second, "Cork request took too long")

		s.T().Logf("waiting for end of vote period, endblocker to run")
		val = s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			kb, err := val.keyring()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
			s.Require().NoError(err)

			queryClient := corkTypes.NewQueryClient(clientCtx)
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
		}, 105*time.Second, 1*time.Second, "new vote period never seen")

		s.T().Log("querying gravity module for logic call transaction")
		aave_abi, err := AaveV2MetaData.GetAbi()
		s.Require().NoError(err)

		methodName := "rebalance"
		zeroAddress := common.HexToAddress(zeroAddr)
		route := [9]common.Address{common.HexToAddress(dai), common.HexToAddress(pool), common.HexToAddress(usdc), zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress, zeroAddress}
		zero := big.NewInt(0)
		swapParams := &[4][3]*big.Int{{zero, big.NewInt(2), big.NewInt(1)}, {zero, zero, zero}, {zero, zero, zero}, {zero, zero, zero}}
		minAssetsOut := big.NewInt(1000)
		method, err := aave_abi.Pack(methodName, route, swapParams, minAssetsOut)
		addr := common.HexToAddress(hardhatCellar.String())
		invalidationScope := crypto.Keccak256Hash(
			bytes.Join(
				[][]byte{addr.Bytes(), method},
				[]byte{},
			)).Bytes()
		invalidationNonce := 1
		s.Require().NoError(err)

		kb, err := val.keyring()
		s.Require().NoError(err)
		clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
		s.Require().NoError(err)

		gravityQueryClient := gravityTypes.NewQueryClient(clientCtx)

		s.Require().Eventuallyf(func() bool {
			request := &gravityTypes.ContractCallTxRequest{
				InvalidationNonce: uint64(invalidationNonce),
				InvalidationScope: invalidationScope,
			}
			response, _ := gravityQueryClient.ContractCallTx(context.Background(), request)

			if response != nil {
				s.T().Logf("found logic call %v!", response.LogicCall)
				return true
			}

			return false
		}, 1*time.Minute, 5*time.Second, "cellar event never seen")

		s.T().Logf("waiting for gravity to submit call to cellar")
		s.Require().Eventuallyf(func() bool {
			s.T().Log("querying gravity logic call events...")
			ethClient, err := ethclient.Dial(fmt.Sprintf("http://%s", s.ethResource.GetHostPort("8545/tcp")))
			s.Require().NoError(err)

			// For non-anonymous events, the first log topic is a keccak256 hash of the
			// event signature.
			eventSignature := []byte("LogicCallEvent(bytes32,uint256,bytes,uint256)")
			logicCallEventSignatureTopic := crypto.Keccak256Hash(eventSignature)
			query := ethereum.FilterQuery{
				FromBlock: nil,
				ToBlock:   nil,
				Addresses: []common.Address{
					gravityContract,
				},
				Topics: [][]common.Hash{
					{
						logicCallEventSignatureTopic,
					},
				},
			}
			result, err := ethClient.FilterLogs(context.Background(), query)
			s.Require().NoError(err)
			ethClient.Close()

			s.T().Logf("got %v LogicCallEvent logs", len(result))
			if len(result) == 0 {
				return false
			}

			// only one LogicCallEvent is expected
			s.Require().Equal(len(result), 1)

			log := result[0]
			gravity_abi, err := GravityMetaData.GetAbi()
			s.Require().NoError(err)

			var event GravityLogicCallEvent
			if len(log.Data) > 0 {
				err := gravity_abi.UnpackIntoInterface(&event, "LogicCallEvent", log.Data)
				s.Require().NoError(err)

				s.T().Log("comparing invalidation parameters")
				eventInvalidationId := event.InvalidationId[:]
				if bytes.Equal(eventInvalidationId, invalidationScope) && int(event.InvalidationNonce.Int64()) == invalidationNonce {
					s.T().Log("logic call executed!")
					return true
				}
				s.T().Log("invalidation parameters did not match up")
			} else {
				s.T().Log("no data in log")
			}

			return false
		}, 1*time.Minute, 3*time.Second, "cellar event never seen")

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
					hardhatCellar,
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
					s.Require().Equal(event.MinAssetsOut, big.NewInt(1000))
					return true
				}
			}

			return false
		}, 2*time.Minute, 5*time.Second, "cellar event never seen")
	})
}
