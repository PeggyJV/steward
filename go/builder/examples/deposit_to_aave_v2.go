package examples

import (
	"context"
	"fmt"
	"math/big"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/go/builder"
	"github.com/peggyjv/steward/go/builder/adaptors"
)

func ExampleDepositToAaveRequest() {
	// Get client and context
	client, err := CreateTlsClient()
	if err != nil {
		panic(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Build adaptor call
	adaptor := common.HexToAddress("0x1234567890000000000000000000000000000000")
	token := common.HexToAddress("0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9")
	amount := big.NewInt(100000)
	adaptorCall := adaptors.NewAaveV2ATokenAdaptorV2CallBuilder(adaptor).
		DepositToAave(token, amount).
		Build()

	// Build call data
	callData, err := builder.NewCallDataBuilder().
		CallOnAdaptor(adaptorCall).
		Build()
	if err != nil {
		panic(err)
	}

	// Build request
	cellarId := common.HexToAddress("0x0")
	request, err := builder.NewScheduleRequestBuilder().
		WithCellarID(cellarId).
		WithChainID(1).
		WithCallData(callData).
		WithBlockHeight(100).
		Build()
	if err != nil {
		panic(err)
	}

	// Send request
	response, err := client.Schedule(ctx, request)
	if err != nil {
		panic(err)
	}

	fmt.Print(response)
}
