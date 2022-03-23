package integration_tests

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"time"

	allocationTypes "github.com/peggyjv/sommelier/v3/x/allocation/types"
	corkTypes "github.com/peggyjv/sommelier/v3/x/cork/types"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func (s *IntegrationTestSuite) TestCork() {
	s.Run("Bring up chain, and submit a cork", func() {
		expectedTickRange := &allocationTypes.TickRange{
			Upper:  12000,
			Lower:  7000,
			Weight: 100,
		}

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

				ctx, cancel := context.WithTimeout(context.Background(), time.Second)
				defer cancel()

				c := NewContractCallClient(conn)
				s.T().Logf("sending request to %s", s.chain.validators[i].keyInfo.GetAddress())
				cellarId := hardhatCellar.String()
				request := SubmitRequest{
					CellarId: cellarId,
					CallData: &SubmitRequest_Uniswapv3Rebalance{
						&UniswapV3RebalanceParams{
							CellarTickInfo: []*UniswapV3Position{
								{
									UpperPrice: expectedTickRange.Upper,
									LowerPrice: expectedTickRange.Lower,
									Weight:     int32(expectedTickRange.Weight),
								},
							},
							CurrentPrice: 10000,
						},
					},
				}
				_, err = c.Submit(ctx, &request)
				// s.Require().NoError(err)
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

		s.T().Logf("checking for updated tick ranges in cellar")
		s.Require().Eventuallyf(func() bool {
			actualTickRange, err := s.getFirstTickRange()
			if err != nil {
				s.T().Logf("got error %e querying ticks", err)
				return false
			}
			if expectedTickRange.Upper != actualTickRange.Upper {
				s.T().Logf("wrong upper %s", actualTickRange.String())
				return false
			}
			if expectedTickRange.Lower != actualTickRange.Lower {
				s.T().Logf("wrong lower %s", actualTickRange.String())
				return false
			}
			if expectedTickRange.Weight != actualTickRange.Weight {
				s.T().Logf("wrong weight %s", actualTickRange.String())
				return false
			}

			return true
		}, 5*time.Minute, 5*time.Second, "cellar ticks never updated")
	})
}
