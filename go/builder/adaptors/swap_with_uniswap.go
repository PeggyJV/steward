package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// SwapWithUniswapAdaptorV1CallBuilder is a builder for SwapWithUniswapAdaptorV1 calls
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Uniswap/SwapWithUniswapAdaptor.sol
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

func (b *SwapWithUniswapAdaptorV1CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *SwapWithUniswapAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.SwapWithUniswapAdaptorV1{
		Function: &steward_proto.SwapWithUniswapAdaptorV1_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
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
