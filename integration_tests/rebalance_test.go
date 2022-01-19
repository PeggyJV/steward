package integration_tests

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"time"

	"github.com/peggyjv/sommelier/x/allocation/types"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

const (
	port = 5734
)

func (s *IntegrationTestSuite) TestRebalance() {
	s.Run("Bring up chain, and submit a re-balance", func() {
		tickRange, err := s.getFirstTickRange()
		s.Require().NoError(err)
		s.Require().Equal(int32(600), tickRange.Upper)
		s.Require().Equal(int32(300), tickRange.Lower)
		s.Require().Equal(uint32(900), tickRange.Weight)

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
				ServerName:   "",
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
						UpperPrice: commit.Vote.Cellar.TickRanges[0].Upper,
						LowerPrice: commit.Vote.Cellar.TickRanges[0].Lower,
						Weight:     commit.Vote.Cellar.TickRanges[0].Weight,
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

		s.T().Logf("checking for updated tick ranges in cellar")
		s.Require().Eventuallyf(func() bool {
			tickRange, err = s.getFirstTickRange()
			if err != nil {
				s.T().Logf("got error %e querying ticks", err)
				return false
			}
			if commit.Vote.Cellar.TickRanges[0].Upper != tickRange.Upper {
				s.T().Logf("wrong upper %s", tickRange.String())
				return false
			}
			if commit.Vote.Cellar.TickRanges[0].Lower != tickRange.Lower {
				s.T().Logf("wrong lower %s", tickRange.String())
				return false
			}
			if commit.Vote.Cellar.TickRanges[0].Weight != tickRange.Weight {
				s.T().Logf("wrong weight %s", tickRange.String())
				return false
			}

			return true
		}, 5*time.Minute, 5*time.Second, "cellar ticks never updated")

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
