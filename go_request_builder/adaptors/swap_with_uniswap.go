package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// Builder for SwapWithUniswapAdaptorV1
type SwapWithUniswapAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.SwapWithUniswapAdaptorV1
}

// NewSwapWithUniswapAdaptorV1CallBuilder creates a new SwapWithUniswapAdaptorV1CallBuilder
func NewSwapWithUniswapAdaptorV1CallBuilder(adaptor common.Address) *SwapWithUniswapAdaptorV1CallBuilder {
	return &SwapWithUniswapAdaptorV1CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.SwapWithUniswapAdaptorV1, 0),
	}
}

// Build builds the AdaptorCall
func (b *SwapWithUniswapAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
			SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *SwapWithUniswapAdaptorV1CallBuilder) SwapWithUniV2(path []common.Address, amount *big.Int, amountOutMin *big.Int) *SwapWithUniswapAdaptorV1CallBuilder {
	pathStrs := make([]string, 0)
	for _, addr := range path {
		pathStrs = append(pathStrs, addr.Hex())
	}
	b.calls = append(b.calls, &steward_proto.SwapWithUniswapAdaptorV1{
		Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV2_{
			SwapWithUniV2: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV2{
				Path:         pathStrs,
				Amount:       amount.String(),
				AmountOutMin: amountOutMin.String(),
			},
		},
	})

	return b
}

func (b *SwapWithUniswapAdaptorV1CallBuilder) SwapWithUniV3(path []common.Address, poolFees []uint32, amount *big.Int, amountOutMin *big.Int) *SwapWithUniswapAdaptorV1CallBuilder {
	pathStrs := make([]string, 0)
	for _, addr := range path {
		pathStrs = append(pathStrs, addr.Hex())
	}
	b.calls = append(b.calls, &steward_proto.SwapWithUniswapAdaptorV1{
		Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
			SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
				Path:         pathStrs,
				PoolFees:     poolFees,
				Amount:       amount.String(),
				AmountOutMin: amountOutMin.String(),
			},
		},
	})

	return b
}
