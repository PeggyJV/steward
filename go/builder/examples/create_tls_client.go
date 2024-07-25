package examples

import (
	"crypto/tls"
	"crypto/x509"
	"os"

	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

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

func CreateTlsClient() (*grpc.ClientConn, steward_proto.ContractCallServiceClient, error) {
	// This example uses fake file paths for the auth materials
	creds, err := buildCredentials("client.crt", "client.key", "server_ca.crt")
	if err != nil {
		return nil, nil, err
	}

	addr := "localhost:5734"
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(creds))
	if err != nil {
		return nil, nil, err
	}

	client := steward_proto.NewContractCallServiceClient(conn)

	return conn, client, nil
}
