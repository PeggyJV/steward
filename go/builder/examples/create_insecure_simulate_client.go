package examples

import (
	"context"
	"fmt"
	"time"

	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func CreateInsecureSimulateClient() {
	addr := "localhost:5734"
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		panic(err)
	}

	defer conn.Close()

	client := steward_proto.NewSimulateContractCallServiceClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Use the client
	resp, err := client.Simulate(ctx, nil)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Printf("Response: %s", resp)
}
