package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

func TestNewPendleAdaptorV1CallBuilder(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	builder := NewPendleAdaptorV1CallBuilder(adaptor)

	assert.Equal(t, adaptor, builder.adaptor)
	assert.NotNil(t, builder.calls)
}

func TestBuild(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	builder := NewPendleAdaptorV1CallBuilder(adaptor)

	call := builder.Build()
	assert.Equal(t, adaptor.Hex(), call.Adaptor)
	assert.NotNil(t, call.CallData)
}

func TestAddLiquidityDualSyAndPt(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netSyDesired := big.NewInt(1000)
	netPtDesired := big.NewInt(500)
	minLpOut := big.NewInt(50)

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.AddLiquidityDualSyAndPt(market, netSyDesired, netPtDesired, minLpOut)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetAddLiquidityDualSyAndPt()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netSyDesired.String(), call.NetSyDesired)
	assert.Equal(t, netPtDesired.String(), call.NetPtDesired)
	assert.Equal(t, minLpOut.String(), call.MinLpOut)
	assert.IsType(t, &steward_proto.PendleAdaptorV1_AddLiquidityDualSyAndPt{}, call)
}

func TestRedeemPyToSy(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netPyIn := big.NewInt(1000)
	minSyOut := big.NewInt(500)

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.RedeemPyToSy(market, netPyIn, minSyOut)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetRedeemPyToSy()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netPyIn.String(), call.NetPyIn)
	assert.Equal(t, minSyOut.String(), call.MinSyOut)
	assert.IsType(t, &steward_proto.PendleAdaptorV1_RedeemPyToSy{}, call)
}

func TestRedeemSyToToken(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netSyIn := big.NewInt(1000)
	tokenOutput := PendleTokenOutput{
		// Initialize the fields of PendleTokenOutput as needed
	}

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.RedeemSyToToken(market, netSyIn, tokenOutput)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetRedeemSyToToken()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netSyIn.String(), call.NetSyIn)
	assert.IsType(t, &steward_proto.PendleAdaptorV1_RedeemSyToToken{}, call)
}

func TestConvertPendleSwapData(t *testing.T) {
	data := PendleSwapData{
		SwapType:    1,
		ExtRouter:   common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
		ExtCallData: "0x1234",
		NeedScale:   true,
	}

	result := convertPendleSwapData(data)

	assert.Equal(t, data.SwapType, result.SwapType)
	assert.Equal(t, data.ExtRouter.Hex(), result.ExtRouter)
	assert.Equal(t, data.ExtCallData, result.ExtCalldata)
	assert.Equal(t, data.NeedScale, result.NeedScale)
}

func TestConvertPendleTokenInput(t *testing.T) {
	input := PendleTokenInput{
		TokenIn:     common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
		NetTokenIn:  big.NewInt(1000),
		TokenMintSy: common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678"),
		PendleSwap:  common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
		SwapData: PendleSwapData{
			SwapType:    1,
			ExtRouter:   common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
			ExtCallData: "0x1234",
			NeedScale:   true,
		},
	}

	result := convertPendleTokenInput(input)

	assert.Equal(t, input.TokenIn.Hex(), result.TokenIn)
	assert.Equal(t, input.NetTokenIn.String(), result.NetTokenIn)
	assert.Equal(t, input.TokenMintSy.Hex(), result.TokenMintSy)
	assert.Equal(t, input.PendleSwap.Hex(), result.PendleSwap)
	assert.Equal(t, input.SwapData.SwapType, result.SwapData.SwapType)
	assert.Equal(t, input.SwapData.ExtRouter.Hex(), result.SwapData.ExtRouter)
	assert.Equal(t, input.SwapData.ExtCallData, result.SwapData.ExtCalldata)
	assert.Equal(t, input.SwapData.NeedScale, result.SwapData.NeedScale)
}

func TestConvertPendleApproxParams(t *testing.T) {
	params := PendleApproxParams{
		GuessMin:      big.NewInt(1000),
		GuessMax:      big.NewInt(2000),
		GuessOffchain: big.NewInt(1500),
		MaxIteration:  big.NewInt(10),
		Eps:           big.NewInt(1),
	}

	result := convertPendleApproxParams(params)

	assert.Equal(t, params.GuessMin.String(), result.GuessMin)
	assert.Equal(t, params.GuessMax.String(), result.GuessMax)
	assert.Equal(t, params.GuessOffchain.String(), result.GuessOffchain)
	assert.Equal(t, params.MaxIteration.String(), result.MaxIteration)
	assert.Equal(t, params.Eps.String(), result.Eps)
}

func TestPendleAdaptorCallBuilder_AddLiquidityDualSyAndPt(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netSyDesired := big.NewInt(1000)
	netPtDesired := big.NewInt(500)
	minLpOut := big.NewInt(50)

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.AddLiquidityDualSyAndPt(market, netSyDesired, netPtDesired, minLpOut)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetAddLiquidityDualSyAndPt()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netSyDesired.String(), call.NetSyDesired)
	assert.Equal(t, netPtDesired.String(), call.NetPtDesired)
	assert.Equal(t, minLpOut.String(), call.MinLpOut)
}

func TestPendleAdaptorCallBuilder_RedeemPyToSy(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netPyIn := big.NewInt(1000)
	minSyOut := big.NewInt(500)

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.RedeemPyToSy(market, netPyIn, minSyOut)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetRedeemPyToSy()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netPyIn.String(), call.NetPyIn)
	assert.Equal(t, minSyOut.String(), call.MinSyOut)
}

func TestPendleAdaptorCallBuilder_RedeemSyToToken(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	market := common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
	netSyIn := big.NewInt(1000)
	tokenOutput := PendleTokenOutput{
		// Initialize the fields of PendleTokenOutput as needed
	}

	builder := NewPendleAdaptorV1CallBuilder(adaptor)
	builder.RedeemSyToToken(market, netSyIn, tokenOutput)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetRedeemSyToToken()
	assert.Equal(t, market.Hex(), call.Market)
	assert.Equal(t, netSyIn.String(), call.NetSyIn)
	// Add more assertions for tokenOutput fields if needed
}
