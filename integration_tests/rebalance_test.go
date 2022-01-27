package integration_tests

import (
	"context"
	"crypto/tls"
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
		initialTickRange, err := s.getFirstTickRange()
		s.Require().NoError(err)
		s.Require().Equal(int32(600), initialTickRange.Upper)
		s.Require().Equal(int32(300), initialTickRange.Lower)
		s.Require().Equal(uint32(900), initialTickRange.Weight)
		expectedTickRange := &types.TickRange{
			Upper:  198840,
			Lower:  192180,
			Weight: 100,
		}
		cellar := &types.Cellar{
			Id: hardhatCellar.String(),
			TickRanges: []*types.TickRange{
				expectedTickRange,
			},
		}

		s.T().Logf("checking that test cellar exists in the chain")
		val := s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			queryClient, err := val.GetQueryClient()
			s.Require().NoError(err, "error getting query client")

			res, err := queryClient.QueryCellars(context.Background(), &types.QueryCellarsRequest{})
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

		// replace vote period check - sending commits with grpc call to steward
		s.T().Logf("request rebalance start")
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

				ctx, cancel := context.WithTimeout(context.Background(), time.Second)
				defer cancel()

				c := NewUniswapV3CellarAllocatorClient(conn)
				data := []*Position{
					{
						UpperPrice: expectedTickRange.Upper,
						LowerPrice: expectedTickRange.Lower,
						Weight:     expectedTickRange.Weight,
					},
				}
				s.T().Logf("sending request to %s", s.chain.validators[i].keyInfo.GetAddress())
				cellarId := fmt.Sprintf("%s:%s", "ethereum", hardhatCellar.String())
				request := RebalanceRequest{CellarId: cellarId, Data: data}
				_, err = c.Rebalance(ctx, &request)
				s.Require().NoError(err)
			}

			return true
		}, 100*time.Second, 1*time.Second, "rebalance request took too long")

		s.T().Logf("wait for new vote period start")
		val = s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			queryClient, err := val.GetQueryClient()
			s.Require().NoError(err, "error getting query client")

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

		s.T().Logf("checking pre-commits for validators")
		precommits := make([]types.AllocationPrecommit, 4)
		for _, val := range s.chain.validators {
			address := val.keyInfo.GetAddress()
			operator := sdk.ValAddress(address)
			s.Require().Eventuallyf(func() bool {
				queryClient, err := val.GetQueryClient()
				s.Require().NoError(err, "error getting query client")

				res, err := queryClient.QueryAllocationPrecommit(context.Background(), &types.QueryAllocationPrecommitRequest{
					Validator: operator.String(),
					Cellar:    hardhatCellar.String(),
				})
				if err != nil {
					return false
				}
				if res == nil {
					return false
				}
				s.Require().Equal(res.Precommit.CellarId, cellar.Id, "cellar ids unequal")

				precommits[val.index] = *res.Precommit
				return true
			},
				30*time.Second,
				2*time.Second,
				"pre-commit not found for validator %s",
				val.keyInfo.GetAddress().String())
		}

		s.T().Logf("checking commits for validators")
		for _, val := range s.chain.validators {
			address := val.keyInfo.GetAddress()
			operator := sdk.ValAddress(address)
			s.Require().Eventuallyf(func() bool {
				queryClient, err := val.GetQueryClient()
				s.Require().NoError(err, "error getting query client")

				res, err := queryClient.QueryAllocationCommit(context.Background(), &types.QueryAllocationCommitRequest{Validator: operator.String(), Cellar: hardhatCellar.String()})
				if err != nil {
					return false
				}
				if res == nil {
					return false
				}
				s.Require().Equal(res.Commit.Vote.Cellar.Id, cellar.Id, "cellar ids unequal")
				commitHash, err := res.Commit.Vote.Hash(res.Commit.Salt, operator)
				s.Require().NoError(err, "unable to create commit hash for comparison")
				s.Require().Equal(commitHash, precommits[val.index].Hash, "commit and precommit hashes do not match")

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
			queryClient, err := val.GetQueryClient()
			s.Require().NoError(err, "error getting query client")

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

		s.T().Logf("checking for updated tick ranges in cellar")
		s.Require().Eventuallyf(func() bool {
			actualTickRange, err := s.getFirstTickRange()
			if err != nil {
				s.T().Logf("got error %e querying ticks", err)
				return false
			}
			if initialTickRange.Upper != actualTickRange.Upper {
				s.T().Logf("wrong upper %s", actualTickRange.String())
				return false
			}
			if initialTickRange.Lower != actualTickRange.Lower {
				s.T().Logf("wrong lower %s", actualTickRange.String())
				return false
			}
			if initialTickRange.Weight != actualTickRange.Weight {
				s.T().Logf("wrong weight %s", actualTickRange.String())
				return false
			}

			return true
		}, 5*time.Minute, 5*time.Second, "cellar ticks never updated")

		s.T().Logf("checking to see if hooks updated cellars on chain")
		val = s.chain.validators[0]
		s.Require().Eventuallyf(func() bool {
			queryClient, err := val.GetQueryClient()
			s.Require().NoError(err, "error getting query client")

			res, err := queryClient.QueryCellars(context.Background(), &types.QueryCellarsRequest{})
			if err != nil {
				return false
			}
			s.Require().Len(res.Cellars, 1, "incorrect number of cellars on chain")
			s.T().Logf("cellars %s", res.Cellars)
			if !res.Cellars[0].Equals(*cellar) {
				s.T().Logf("unequal cellars %s %s", res.Cellars[0].String(), cellar.String())
				return false
			}
			return true
		}, 100*time.Second, 10*time.Second, "on chain cellars never updated")

	})
}
