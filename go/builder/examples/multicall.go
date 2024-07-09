package examples

import (
	"context"
	"fmt"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/go/builder"
)

func ExampleMulticall() {
	// Get client and context
	client, err := CreateTlsClient()
	if err != nil {
		panic(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Build call data
	index := uint32(0)
	positionId := uint32(1)
	inDebtArray := false
	configurationData := []byte{0x1}

	// A multicall is used implicitly when more than one function call is added to the CellarCallDataBuilder
	callData, err := builder.NewCallDataBuilder().
		AddPositionToCatalogue(positionId).
		AddPosition(index, positionId, configurationData, inDebtArray).
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
