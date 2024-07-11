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

	// Pendle args
	market := common.HexToAddress("0x4444444444444444444444444444444444444444")
	netSyDesired := big.NewInt(1000)
	netPtDesired := big.NewInt(500)
	minLpOut := big.NewInt(50)
	netPyIn := big.NewInt(1000)
	minSyOut := big.NewInt(500)
	tokenInput := adaptors.PendleTokenInput{
		TokenIn:     token,
		NetTokenIn:  amount,
		TokenMintSy: token,
		PendleSwap:  token,
		SwapData: adaptors.PendleSwapData{
			SwapType:    1,
			ExtRouter:   token,
			ExtCallData: "FFFF",
			NeedScale:   true,
		},
	}
	approxParams := adaptors.PendleApproxParams{
		GuessMin:      big.NewInt(1000),
		GuessMax:      big.NewInt(2000),
		GuessOffchain: big.NewInt(1500),
		MaxIteration:  big.NewInt(10),
		Eps:           big.NewInt(1),
	}
	tokenOutput := adaptors.PendleTokenOutput{
		TokenOut:      token,
		MinTokenOut:   amount,
		TokenRedeemSy: token,
		PendleSwap:    token,
		SwapData: adaptors.PendleSwapData{
			SwapType:    1,
			ExtRouter:   token,
			ExtCallData: "FFFF",
			NeedScale:   true,
		},
	}

	// CallOnAdaptor
	testCases := []struct {
		message string
		calls   *steward_proto.AdaptorCall
	}{
		{
			message: "call on AaveV2ATokenAdaptorV2",
			calls: adaptors.NewAaveV2ATokenAdaptorV2CallBuilder(adaptor).
				RevokeApproval(token, spender).
				DepositToAave(token, amount).
				WithdrawFromAave(token, amount).
				Build(),
		},
		{
			message: "call on AaveV2DebtTokenAdaptorV2",
			calls: adaptors.NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor).
				RevokeApproval(token, spender).
				BorrowFromAave(token, amount).
				RepayAaveDebt(token, amount).
				Build(),
		},
		{
			message: "call on AaveV3ATokenAdaptorV1",
			calls: adaptors.NewAaveV3ATokenAdaptorV1CallBuilder(adaptor).
				RevokeApproval(token, spender).
				DepositToAave(token, amount).
				WithdrawFromAave(token, amount).
				AdjustIsolationModeAssetAsCollateral(token, true).
				ChangeEMode(uint32(1)).
				Build(),
		},
		{
			message: "call on AaveV3DebtTokenAdaptorV2",
			calls: adaptors.NewAaveV3DebtTokenAdaptorV1CallBuilder(adaptor).
				RevokeApproval(token, spender).
				BorrowFromAave(token, amount).
				RepayAaveDebt(token, amount).
				RepayWithATokens(token, amount).
				Build(),
		},
		{
			message: "call on AaveV2EnableAssetAsCollateralAdaptorV1",
			calls: adaptors.NewAaveV2EnableAssetAsCollateralAdaptorV1CallBuilder(adaptor).
				RevokeApproval(token, spender).
				SetUserUseReserveAsCollateral(token, true).
				Build(),
		},
		{
			message: "call on aave v3 flash loan",
			calls: adaptors.NewAaveV3DebtTokenFlashLoanAdaptorV1CallBuilder(adaptor).
				FlashLoan([]common.Address{token}, []*big.Int{amount}, []*steward_proto.AdaptorCall{adaptors.NewAaveV2EnableAssetAsCollateralAdaptorV1CallBuilder(adaptor).
					RevokeApproval(token, spender).
					SetUserUseReserveAsCollateral(token, false).
					Build()}).Build(),
		},
		{
			message: "call on balancer flash loan",
			calls: adaptors.NewBalancerPoolFlashLoanAdaptorV1CallBuilder(adaptor).
				FlashLoan([]common.Address{token}, []*big.Int{amount}, []*steward_proto.AdaptorCall{adaptors.NewAaveV2EnableAssetAsCollateralAdaptorV1CallBuilder(adaptor).
					RevokeApproval(token, spender).
					SetUserUseReserveAsCollateral(token, false).
					Build()}).Build(),
		},
		{
			message: "call on PendleAdaptorV1",
			calls: adaptors.NewPendleAdaptorV1CallBuilder(adaptor).
				RevokeApproval(token, spender).
				MintSyFromToken(market, netSyDesired, tokenInput).
				MintPyFromSy(market, netSyDesired, netPtDesired).
				SwapExactPtForYt(market, amount, amount, approxParams).
				SwapExactYtForPt(market, amount, amount, approxParams).
				AddLiquidityDualSyAndPt(market, netSyDesired, netPtDesired, minLpOut).
				RedeemPyToSy(market, netPyIn, minSyOut).
				RedeemSyToToken(market, amount, tokenOutput).
				Build(),
		},
		{
			message: "call swap with uniswap",
			calls: adaptors.NewSwapWithUniswapAdaptorV1CallBuilder(adaptor).
				RevokeApproval(token, spender).
				SwapWithUniV2([]common.Address{token}, amount, amount).
				SwapWithUniV3([]common.Address{token}, []uint32{1}, amount, amount).
				Build(),
		},
	}

	for _, c := range testCases {
		t.Run(c.message, func(t *testing.T) {
			callData, err := NewCellarCallDataBuilder().
				CallOnAdaptor(c.calls).
				Build()

			assert.NoError(t, err)

			request, err := newRequest(callData)
			assert.NoError(t, err)

			_, err = client.Simulate(ctx, request)
			assert.NoError(t, err)
		})
	}

	// Missing function calls are either not added to the builder or will cause the test to fail
	// because of the hardcoded position/adaptor whitelist in steward.
	t.Run("other cellar functions", func(t *testing.T) {
		callData, err := NewCellarCallDataBuilder().
			AddPosition(uint32(0), uint32(1), []byte{0x00}, false).
			SwapPositions(uint32(1), uint32(2), false).
			RemovePosition(uint32(0), false).
			SetHoldingPosition(uint32(2)).
			SetStrategistPayoutAddress(spender).
			SetShareLockPeriod(big.NewInt(100)).
			InitiateShutdown().
			LiftShutdown().
			RemovePositionFromCatalogue(uint32(3)).
			RemoveAdaptorFromCatalogue(adaptor).
			IncreaseShareSupplyCap(big.NewInt(100)).
			Build()

		assert.NoError(t, err)

		request, err := newRequest(callData)
		assert.NoError(t, err)

		_, err = client.Simulate(ctx, request)
		assert.NoError(t, err)
	})

}
