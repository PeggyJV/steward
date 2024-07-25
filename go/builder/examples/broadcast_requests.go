package examples

import (
	"context"
	"fmt"
	"sync"

	"github.com/cosmos/cosmos-sdk/client"
	pubsub "github.com/peggyjv/sommelier/v7/x/pubsub/types"
)

// Requests to schedule a contract call must be sent to many Steward instances. This is required because calls that do not have enough votes of approval from the validator set will not be
// executed. Right now there is no single endpoint that handles broadcastic the call to each
// Steward instance, so we must query the endpoints and then send the request to each one.
//
// This example includes a concurrent loop that sends the request to each subscriber, which is the optimal way to broadcast in production due to the time constraint of the scheduled height.
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
	conn, client, err := CreateTlsClient()
	if err != nil {
		panic(err)
	}

	defer conn.Close()

	// Build request
	request, err := BuildMulticallRequest()
	if err != nil {
		panic(err)
	}

	// Send request to each subscriber concurrently
	var wg sync.WaitGroup

	for _, subscriber := range subscribers {
		wg.Add(1)
		go func(pushUrl string) {
			defer wg.Done()

			response, err := client.Schedule(context.Background(), request)
			if err != nil {
				fmt.Printf("Error sending request to %s: %s\n", pushUrl, err)
				return
			}

			fmt.Print("Sent request to ", pushUrl, " with response: ", response, "\n")
		}(subscriber.PushUrl)
	}

	wg.Wait()
}
