package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the BalancerPoolAdaptorCallBuilder constructor
func TestNewBalancerPoolAdaptorCall(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewBalancerPoolAdaptorV1CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))
}

// Test JoinPool function
func TestJoinPool(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewBalancerPoolAdaptorV1CallBuilder(adaptor)

	swapData := BalancerSwapData{
		MinAmountsForSwaps: []*big.Int{big.NewInt(100)},
		SwapDeadlines:      []*big.Int{big.NewInt(1000)},
	}

	singleSwap := BalancerSingleSwap{
		PoolId:   "0x3333333333333333333333333333333333333333",
		Kind:     1,
		AssetIn:  common.HexToAddress("0x0000000000000000000000000000000000000000"),
		AssetOut: common.HexToAddress("0x1111111111111111111111111111111111111111"),
		Amount:   big.NewInt(100),
		UserData: []byte{0x01, 0x02, 0x03},
	}

	targetAddress := common.HexToAddress("0x4")

	builder.JoinPool(targetAddress, []*BalancerSingleSwap{&singleSwap}, swapData, big.NewInt(42))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.BalancerPoolAdaptorV1_JoinPool_{}, builder.calls[0].Function)
}

// Test ExitPool function
func TestExitPool(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewBalancerPoolAdaptorV1CallBuilder(adaptor)

	swapData := BalancerSwapData{
		MinAmountsForSwaps: []*big.Int{big.NewInt(100)},
		SwapDeadlines:      []*big.Int{big.NewInt(1000)},
	}

	singleSwap := BalancerSingleSwap{
		PoolId:   "0x3333333333333333333333333333333333333333",
		Kind:     1,
		AssetIn:  common.HexToAddress("0x0000000000000000000000000000000000000000"),
		AssetOut: common.HexToAddress("0x1111111111111111111111111111111111111111"),
		Amount:   big.NewInt(100),
		UserData: []byte{0x01, 0x02, 0x03},
	}
	targetAddress := common.HexToAddress("0x4")
	exitPoolRequest := BalancerExitPoolRequest{
		Assets:        []common.Address{common.HexToAddress("0x1")},
		MinAmountsOut: []*big.Int{big.NewInt(100)},
	}

	builder.ExitPool(targetAddress, []*BalancerSingleSwap{&singleSwap}, swapData, &exitPoolRequest)

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.BalancerPoolAdaptorV1_ExitPool_{}, builder.calls[0].Function)
}
