package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/stretchr/testify/assert"
)

func TestNewSwapWithUniswapAdaptorV1CallBuilder(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	builder := NewSwapWithUniswapAdaptorV1CallBuilder(adaptor)

	assert.Equal(t, adaptor, builder.adaptor)
	assert.NotNil(t, builder.calls)
}

func TestSwapWithUniswapAdaptorV1Build(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	builder := NewSwapWithUniswapAdaptorV1CallBuilder(adaptor)

	call := builder.Build()
	assert.Equal(t, adaptor.Hex(), call.Adaptor)
	assert.NotNil(t, call.CallData)
}

func TestSwapWithUniV2(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	path := []common.Address{
		common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
		common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678"),
	}
	amount := big.NewInt(1000)
	amountOutMin := big.NewInt(500)

	builder := NewSwapWithUniswapAdaptorV1CallBuilder(adaptor)
	builder.SwapWithUniV2(path, amount, amountOutMin)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetSwapWithUniV2()
	assert.Equal(t, len(path), len(call.Path))
	for i, addr := range path {
		assert.Equal(t, addr.Hex(), call.Path[i])
	}
	assert.Equal(t, amount.String(), call.Amount)
	assert.Equal(t, amountOutMin.String(), call.AmountOutMin)
}

func TestSwapWithUniV3(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678")
	path := []common.Address{
		common.HexToAddress("0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef"),
		common.HexToAddress("0x1234567890abcdef1234567890abcdef12345678"),
	}
	poolFees := []uint32{3000, 500}
	amount := big.NewInt(1000)
	amountOutMin := big.NewInt(500)

	builder := NewSwapWithUniswapAdaptorV1CallBuilder(adaptor)
	builder.SwapWithUniV3(path, poolFees, amount, amountOutMin)

	assert.Len(t, builder.calls, 1)
	call := builder.calls[0].GetSwapWithUniV3()
	assert.Equal(t, len(path), len(call.Path))
	for i, addr := range path {
		assert.Equal(t, addr.Hex(), call.Path[i])
	}
	assert.Equal(t, len(poolFees), len(call.PoolFees))
	for i, fee := range poolFees {
		assert.Equal(t, fee, call.PoolFees[i])
	}
	assert.Equal(t, amount.String(), call.Amount)
	assert.Equal(t, amountOutMin.String(), call.AmountOutMin)
}
