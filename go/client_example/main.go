package main

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"math/big"
	"os"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/go/builder"
	"github.com/peggyjv/steward/go/builder/adaptors"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func main() {
	/// 1. Build credentials object for two-way TLS authentication

	// Replace these paths with your own client cert and key, and server CA paths
	creds, err := buildCredentials("client.crt", "client.key", "server_ca.crt")
	if err != nil {
		fmt.Printf("failed to build credentials: %v", err)
		return
	}

	/// 2. Construct context and client

	conn, client := buildClient(creds)
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	/// 3. Construct adaptor calls and final request using the builder package

	request := buildScheduleRequest()

	/// 4. Send the request

	_, err = client.Schedule(ctx, request)
}

func buildCredentials(clientCertPath string, clientKeyPath string, serverCAPath string) (credentials.TransportCredentials, error) {
	clientCert, err := tls.LoadX509KeyPair(clientCertPath, clientKeyPath)
	if err != nil {
		return nil, err
	}

	rootPool := x509.NewCertPool()
	serverCA, err := os.ReadFile(serverCAPath)
	if err != nil {
		return nil, err
	}
	rootPool.AppendCertsFromPEM(serverCA)

	tlsConfig := &tls.Config{
		Certificates: []tls.Certificate{clientCert},
		RootCAs:      rootPool,
	}

	return credentials.NewTLS(tlsConfig), nil
}

func getConnection(addr string, creds credentials.TransportCredentials) (*grpc.ClientConn, error) {
	conn, err := grpc.Dial(
		addr,
		grpc.WithBlock(),
		grpc.WithTransportCredentials(creds),
	)
	if err != nil {
		return nil, err
	}

	return conn, nil
}

func buildClient(creds credentials.TransportCredentials) (*grpc.ClientConn, steward_proto.ContractCallServiceClient) {
	addr := fmt.Sprintf("localhost:5734")
	conn, err := getConnection(addr, creds)
	if err != nil {
		panic("failed to get connection")
	}
	defer conn.Close()

	return conn, steward_proto.NewContractCallServiceClient(conn)
}

func buildScheduleRequest() *steward_proto.ScheduleRequest {
	////// A. Build some adaptor calls

	// This call to the Aave V2 A Token adaptor will have two function calls in it
	aaveV2Address := common.HexToAddress("0x1111111111111111111111111111111111111111")
	token := common.HexToAddress("0x2222222222222222222222222222222222222222")

	aaveV2Call := adaptors.NewAaveV2ATokenAdaptorV2CallBuilder(aaveV2Address).
		DepositToAave(token, big.NewInt(1000000000000000000)).
		WithdrawFromAave(token, big.NewInt(1000000000000000000)).
		Build()

	// This call to revoke swapping on Balancer will have one call in it
	balancerAddress := common.HexToAddress("0x3333333333333333333333333333333333333333")
	spenderAddress := common.HexToAddress("0x4444444444444444444444444444444444444444")

	balancerCall := adaptors.NewBalancerPoolAdaptorV1CallBuilder(balancerAddress).
		RevokeApproval(token, spenderAddress).
		Build()

	////// B. Build the Cellar function call and put the adaptor calls in it

	// This will implicitly be a multicall to the Cellar contract since it includes two adaptors calls.
	// We'll call some peripheral functions and then the adaptors. This is a nonsensical series of function
	// calls simply meant to exhibit the use of the builder API.
	configurationData := []byte("0xdeadbeef")
	cellarCall, err := builder.NewCallDataBuilder().
		LiftShutdown().
		AddPosition(uint32(1), uint32(1), configurationData, false).
		CallOnAdaptor(aaveV2Call).
		CallOnAdaptor(balancerCall).
		Build()

	if err != nil {
		panic("failed to build cellar call")
	}

	////// C. Build the final schedule request

	cellarId := "0x0000000000000000000000000000000000000000"
	chainId := uint64(1)
	blockHeight := uint64(100)

	scheduleRequest, err := builder.NewScheduleRequestBuilder().
		WithCellarID(common.HexToAddress(cellarId)).
		WithChainID(int(chainId)).
		WithBlockHeight(int(blockHeight)).
		WithCallData(cellarCall).
		Build()

	if err != nil {
		panic("failed to build schedule request")
	}

	return scheduleRequest
}
