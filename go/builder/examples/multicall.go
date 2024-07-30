package examples

import (
	"context"
	"fmt"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/go/builder"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

func BuildMulticallRequest() (*steward_proto.ScheduleRequest, error) {
	index := uint32(0)
	positionId := uint32(1)
	inDebtArray := false
	configurationData := []byte{0x1}

	// A multicall is used implicitly when more than one function call is added to the CellarCallDataBuilder
	callData, err := builder.NewCellarCallDataBuilder().
		AddPositionToCatalogue(positionId).
		AddPosition(index, positionId, configurationData, inDebtArray).
		Build()
	if err != nil {
		return nil, err
	}

	// Build request
	cellarId := common.HexToAddress("0x0")
	return builder.NewScheduleRequestBuilder().
		WithCellarID(cellarId).
		WithChainID(1).
		WithCallData(callData).
		WithBlockHeight(100).
		Build()
}

func ExampleMulticall() {
	// Get client and context
	conn, client, err := CreateTlsClient()
	if err != nil {
		panic(err)
	}

	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Build call data
	request, err := BuildMulticallRequest()
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
