package integration_tests

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"math/big"
	"reflect"
	"time"

	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/ethclient"
	gravityTypes "github.com/peggyjv/gravity-bridge/module/x/gravity/types"
	allocationTypes "github.com/peggyjv/sommelier/v3/x/allocation/types"
	corkTypes "github.com/peggyjv/sommelier/v3/x/cork/types"
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

			res, err := queryClient.QueryCellars(context.Background(), &allocationTypes.QueryCellarsRequest{})
			if err != nil {
				return false
			}
			if res == nil {
				return false
			}
			for _, c := range res.Cellars {
				if c.Id == hardhatCellar.String() {
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
				addr := fmt.Sprintf("localhost:%v", port+i)
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
				request := SubmitRequest{
					CellarId: cellarId,
					CallData: &SubmitRequest_AaveV2Stablecoin{
						&AaveV2Stablecoin{
							Function: &AaveV2Stablecoin_ClaimAndUnstake{
								&ClaimAndUnstake{},
							},
						},
					},
				}
				_, err = c.Submit(ctx, &request)
				s.Require().NoError(err)
			}

			return true
		}, 100*time.Second, 1*time.Second, "rebalance request took too long")

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

		methodName := "claimAndUnstake"
		method, err := aave_abi.Pack(methodName)
		invalidationScope := crypto.Keccak256(method)
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
			logs, err := ethClient.FilterLogs(context.Background(), query)
			s.Require().NoError(err)
			s.T().Logf("got %v gravity.submitLogicCall logs", len(logs))
			ethClient.Close()

			gravity_abi, err := GravityMetaData.GetAbi()
			s.Require().NoError(err)

			for i, log := range logs {
				if len(log.Data) > 0 {
					out, err := gravity_abi.Unpack("LogicCallEvent", log.Data)
					if err != nil {
						s.T().Logf("log%v: failed to unpack log (%v) to event structure", i, log.Data)
						return false
					}

					event := &GravityLogicCallEvent{
						InvalidationId:    out[0].([32]byte),
						InvalidationNonce: out[1].(*big.Int),
						ReturnData:        out[2].([]byte),
						EventNonce:        out[3].(*big.Int),
					}
					s.T().Logf("log%v: comparing invalidation parameters", i)
					eventInvalidationId := event.InvalidationId[:]
					if reflect.DeepEqual(eventInvalidationId, invalidationScope) && int(event.InvalidationNonce.Int64()) == invalidationNonce {
						s.T().Log("logic call executed!")
						return true
					}

					s.T().Logf("log%v: invalidation parameters did not match up", i)
				} else {
					s.T().Logf("log%v: no data in log", i)
				}
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
			eventSignature := []byte("mockClaimAndUnstake()")
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
			s.T().Logf("got %v logs", len(logs))
			ethClient.Close()

			if len(logs) > 0 {
				s.T().Log("saw mock function event!")
				return true
			}

			return false
		}, 2*time.Minute, 5*time.Second, "cellar event never seen")
	})
}
