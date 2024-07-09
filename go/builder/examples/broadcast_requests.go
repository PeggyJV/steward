package examples

import (
	"context"
	"fmt"

	"github.com/cosmos/cosmos-sdk/client"
	pubsub "github.com/peggyjv/sommelier/v7/x/pubsub/types"
)

// Requests to schedule a contract call must be sent to many Steward instances. This is required because calls that do not have enough votes of approval from the validator set will not be
// executed. Right now there is no single endpoint that handles broadcastic the call to each
// Steward instance, so we must query the endpoints and then send the request to each one.
//
// This example simple sends the requests sequentially in a loop. In production it would be better to
// send the requests concurrently.
func ExampleBroadcastRequests() {
	// Query the Sommelier chain to get the list of Steward instances
	sommCtx := client.Context{}.WithNodeURI("http://localhost:26657")
	sommQueryClient := pubsub.NewQueryClient(sommCtx)

	res, err := sommQueryClient.QuerySubscribers(context.Background(), &pubsub.QuerySubscribersRequest{})
	if err != nil {
		panic(err)
	}

	subscribers := res.Subscribers

	// Get client
	client, err := CreateTlsClient()
	if err != nil {
		panic(err)
	}

	// Build request
	request, err := BuildMulticallRequest()
	if err != nil {
		panic(err)
	}

	// Send request to each subscriber
	for _, subscriber := range subscribers {
		response, err := client.Schedule(context.Background(), request)
		if err != nil {
			fmt.Printf("Error sending request to %s: %s\n", subscriber, err)
		}

		fmt.Print("Sent request to ", subscriber, " with response: ", response, "\n")
	}
}
