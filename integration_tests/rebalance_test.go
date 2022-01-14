package integration_tests

import (
	"context"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"time"

	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/peggyjv/sommelier/x/allocation/types"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

const (
	port = 5734
)

func (s *IntegrationTestSuite) TestRebalance() {
	s.Run("Bring up chain, and submit a re-balance", func() {

		// DO NOT DELETE COMMENTED CODE BELOW
		// It's just commented out to isolate steward run for debugging
		// tickRange, err := s.getFirstTickRange()
		// s.Require().NoError(err)
		// s.Require().Equal(int32(600), tickRange.Upper)
		// s.Require().Equal(int32(300), tickRange.Lower)
		// s.Require().Equal(uint32(900), tickRange.Weight)

		commit := types.Allocation{
			Vote: &types.RebalanceVote{
				Cellar: &types.Cellar{
					Id: hardhatCellar.String(),
					TickRanges: []*types.TickRange{
						{Upper: 198840, Lower: 192180, Weight: 100},
					},
				},
				CurrentPrice: 100,
			},
			Salt: "testsalt",
		}

		s.T().Logf("checking that test cellar exists in the chain")
		val := s.chain.validators[0]
		// s.Require().Eventuallyf(func() bool {
		// 	kb, err := val.keyring()
		// 	s.Require().NoError(err)
		// 	clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
		// 	s.Require().NoError(err)

		// 	queryClient := types.NewQueryClient(clientCtx)
		// 	res, err := queryClient.QueryCellars(context.Background(), &types.QueryCellarsRequest{})
		// 	if err != nil {
		// 		return false
		// 	}
		// 	if res == nil {
		// 		return false
		// 	}
		// 	for _, c := range res.Cellars {
		// 		if c.Id == hardhatCellar.String() {
		// 			return true
		// 		}
		// 	}
		// 	return false
		// }, 30*time.Second, 2*time.Second, "hardhat cellar not found in chain")

		// replace vote period check - sending commits with grpc call to steward
		s.T().Logf("request rebalance start")
		s.Require().Eventually(func() bool {
			data := []*Position{{UpperPrice: 1000, LowerPrice: 1, Weight: 100}}
			request := RebalanceRequest{CellarId: "ethereum:0x0000000000000000000000000000000000000000", Data: data}

			// Load in client TLS config
			r, _ := ioutil.ReadFile("integration_tests/tls/client/test_client.crt")
			block, _ := pem.Decode(r)
			cert, err := x509.ParseCertificate(block.Bytes)
			s.Require().NoError(err)

			pool := x509.NewCertPool()
			pool.AddCert(cert)

			tls := credentials.NewClientTLSFromCert(pool, "")
			s.T().Logf("    loaded client cert")

			for i, steward := range s.chain.stewards {
				// Dial steward and send rebalance request
				addr := fmt.Sprintf("localhost:%v", port+i)
				s.T().Logf("    dialing")
				conn, err := grpc.Dial(addr, grpc.WithBlock(), grpc.WithTransportCredentials(tls))
				if err != nil {
					s.T().Fatalf("failed to connect to steward: %v", err)
				}
				defer conn.Close()

				s.T().Logf("    connection established")
				c := NewUniswapV3CellarAllocatorClient(conn)

				ctx, cancel := context.WithTimeout(context.Background(), time.Second)
				defer cancel()

				s.T().Logf("    sending rebalance")
				_, err = c.Rebalance(ctx, &request)
				if err != nil {
					s.T().Fatalf("rebalance request to %s at %s failed: %v", steward.instanceName(), addr, err)
				}
			}

			return true
		}, 100*time.Second, 1*time.Second, "rebalance request took too long")

		s.T().Logf("checking pre-commits for validators")
		for i, val := range s.chain.validators {
			i := i
			val := val
			s.Require().Eventuallyf(func() bool {
				kb, err := val.keyring()
				s.Require().NoError(err)
				clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
				s.Require().NoError(err)

				queryClient := types.NewQueryClient(clientCtx)
				signerVal := sdk.ValAddress(val.keyInfo.GetAddress())
				res, err := queryClient.QueryAllocationPrecommit(context.Background(), &types.QueryAllocationPrecommitRequest{
					Validator: signerVal.String(),
					Cellar:    hardhatCellar.String(),
				})
				if err != nil {
					return false
				}
				if res == nil {
					return false
				}
				expectedPrecommit, err := types.NewMsgAllocationPrecommit(*commit.Vote, commit.Salt, s.chain.orchestrators[i].keyInfo.GetAddress(), sdk.ValAddress(val.keyInfo.GetAddress()))
				s.Require().NoError(err, "unable to create precommit")
				s.Require().Equal(res.Precommit.CellarId, commit.Vote.Cellar.Id, "cellar ids unequal")
				s.Require().Equal(res.Precommit.Hash, expectedPrecommit.Precommit[0].Hash, "commit hashes unequal")

				return true
			},
				30*time.Second,
				2*time.Second,
				"pre-commit not found for validator %s",
				val.keyInfo.GetAddress().String())
		}

		s.T().Logf("checking commits for validators")
		for _, val := range s.chain.validators {
			val := val
			s.Require().Eventuallyf(func() bool {
				kb, err := val.keyring()
				s.Require().NoError(err)
				clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
				s.Require().NoError(err)

				queryClient := types.NewQueryClient(clientCtx)
				res, err := queryClient.QueryAllocationCommit(context.Background(), &types.QueryAllocationCommitRequest{Validator: sdk.ValAddress(val.keyInfo.GetAddress()).String(), Cellar: hardhatCellar.String()})
				if err != nil {
					return false
				}
				if res == nil {
					return false
				}
				s.Require().Equal(res.Commit.Vote.Cellar.Id, commit.Vote.Cellar.Id, "cellar ids unequal")

				return true
			},
				30*time.Second,
				2*time.Second,
				"commit not found for validator %s",
				val.keyInfo.GetAddress().String())
		}

		s.T().Logf("waiting for end of vote period, endblocker to run")
		val = s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			kb, err := val.keyring()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
			s.Require().NoError(err)

			queryClient := types.NewQueryClient(clientCtx)
			res, err := queryClient.QueryCommitPeriod(context.Background(), &types.QueryCommitPeriodRequest{})
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

		// s.T().Logf("checking for updated tick ranges in cellar")
		// s.Require().Eventuallyf(func() bool {
		// 	tickRange, err = s.getFirstTickRange()
		// 	if err != nil {
		// 		s.T().Logf("got error %e querying ticks", err)
		// 		return false
		// 	}
		// 	if commit.Vote.Cellar.TickRanges[0].Upper != tickRange.Upper {
		// 		s.T().Logf("wrong upper %s", tickRange.String())
		// 		return false
		// 	}
		// 	if commit.Vote.Cellar.TickRanges[0].Lower != tickRange.Lower {
		// 		s.T().Logf("wrong lower %s", tickRange.String())
		// 		return false
		// 	}
		// 	if commit.Vote.Cellar.TickRanges[0].Weight != tickRange.Weight {
		// 		s.T().Logf("wrong weight %s", tickRange.String())
		// 		return false
		// 	}

		// 	return true
		// }, 5*time.Minute, 5*time.Second, "cellar ticks never updated")

		s.T().Logf("checking to see if hooks updated cellars on chain")
		val = s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			kb, err := val.keyring()
			s.Require().NoError(err)
			clientCtx, err := s.chain.clientContext("tcp://localhost:26657", &kb, "val", val.keyInfo.GetAddress())
			s.Require().NoError(err)

			queryClient := types.NewQueryClient(clientCtx)
			res, err := queryClient.QueryCellars(context.Background(), &types.QueryCellarsRequest{})
			if err != nil {
				return false
			}
			s.Require().Len(res.Cellars, 1, "incorrect number of cellars on chain")
			s.T().Logf("cellars %s", res.Cellars)
			if !res.Cellars[0].Equals(*commit.Vote.Cellar) {
				s.T().Logf("unequal cellars %s %s", res.Cellars[0].String(), commit.Vote.Cellar.String())
				return false
			}

			return true
		}, 100*time.Second, 10*time.Second, "on chain cellars never updated")

	})
}
