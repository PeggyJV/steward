package examples

import (
	"context"
	"fmt"
	"time"

	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
)

// When running the production server, Steward exposes a Status/Version method that shows the current
// running version of Steward. It can be useful for verifying connectivity and compatability. TLS authentication
// is required to access this endpoint.
func CreateTlsStatusClient() (steward_proto.StatusServiceClient, error) {
	// This example uses fake file paths for the auth materials
	creds, err := buildCredentials("client.crt", "client.key", "server_ca.crt")
	if err != nil {
		return nil, err
	}

	addr := "localhost:5734"
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(creds))
	if err != nil {
		return nil, err
	}

	defer conn.Close()

	client := steward_proto.NewStatusServiceClient(conn)

	return client, nil
}

func ExampleCheckStewardStatus() {
	// Get client and context
	client, err := CreateTlsStatusClient()
	if err != nil {
		panic(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Send request
	response, err := client.Version(ctx, &steward_proto.VersionRequest{})
	if err != nil {
		panic(err)
	}

	fmt.Print(response)
}
