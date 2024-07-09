package builder

import (
	"context"
	"math/big"
	"testing"
	"time"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/go/builder/adaptors"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

// Integration tests assuming there is a running Steward simulation server on port 5734

// Builds insecure grpc client for testing
func buildInsecureClient() (*grpc.ClientConn, steward_proto.SimulateContractCallServiceClient, error) {
	addr := "localhost:5734"
	conn, err := grpc.NewClient(addr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		return nil, nil, err
	}

	return conn, steward_proto.NewSimulateContractCallServiceClient(conn), nil
}

func newRequest(callData *steward_proto.CellarV2_5) (*steward_proto.SimulateRequest, error) {
	scheduleRequest, err := NewScheduleRequestBuilder().
		WithCellarID(common.HexToAddress("0x9999999999999999999999999999999999999999")).
		WithChainID(1).
		WithCallData(callData).
		WithBlockHeight(1).
		WithDeadline(0).
		Build()
	if err != nil {
		return nil, err
	}

	return &steward_proto.SimulateRequest{
		Request:    scheduleRequest,
		EncodeOnly: true,
	}, nil
}

func TestBuilderIntegration(t *testing.T) {
	conn, client, err := buildInsecureClient()
	require.NoError(t, err)
	defer conn.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	require.NoError(t, err)

	amount := big.NewInt(100)
	adaptor := common.HexToAddress("0x0000000000000000000000000000000000000000")
	token := common.HexToAddress("0x1111111111111111111111111111111111111111")
	spender := common.HexToAddress("0x2222222222222222222222222222222222222222")

	t.Run("call on AaveV2ATokenAdaptorV2", func(t *testing.T) {
		adaptorCalls := adaptors.NewAaveV2ATokenAdaptorV2CallBuilder(adaptor).
			RevokeApproval(token, spender).
			DepositToAave(token, amount).
			WithdrawFromAave(token, amount).
			Build()

		callData, err := NewCallDataBuilder().
			CallOnAdaptor(adaptorCalls).
			Build()

		assert.NoError(t, err)

		request, err := newRequest(callData)
		assert.NoError(t, err)

		response, err := client.Simulate(ctx, request)
		assert.NoError(t, err)

		response.EncodedCall
	})
}
