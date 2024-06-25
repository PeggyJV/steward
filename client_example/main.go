package main

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"fmt"
	"os"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func main() {
        // Replace these paths with your own client cert and key, and server CA paths
		creds, err := buildCredentials("client.crt", "client.key", "server_ca.crt")
        if err != nil {
            fmt.Printf("failed to build credentials: %v", err)
            return
        }

        cellarId := common.HexToAddress("")
        chainId := uint(1)
        adaptorContract := common.HexToAddress("")
        blockHeight := uint(1)

        request := BuildUniswapV3ScheduleRequest(cellarId, chainId, adaptorContract, blockHeight)

        addr := fmt.Sprintf("localhost:5734")
        conn, err := getConnection(addr, creds) 
        if err != nil {
            fmt.Printf("failed to get connection: %v", err)
            return
        }
        defer conn.Close()

        ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
        defer cancel()

        c := steward_proto.NewContractCallServiceClient(conn)
        _, err = c.Schedule(ctx, request)
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
