package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

const (
	_ = iota
	BalancerSwapKindGivenIn
	BalancerSwapKindGivenOut
)

// BalancerPoolAdaptorV1CallBuilder is a builder for BalancerPoolAdaptorV1 calls
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Balancer/BalancerPoolAdaptor.sol
type BalancerPoolAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.BalancerPoolAdaptorV1
}

func NewBalancerPoolAdaptorV1CallBuilder(adaptor common.Address) *BalancerPoolAdaptorV1CallBuilder {
	return &BalancerPoolAdaptorV1CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.BalancerPoolAdaptorV1, 0),
	}
}

func (b *BalancerPoolAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_BalancerPoolV1Calls{
			BalancerPoolV1Calls: &steward_proto.BalancerPoolAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *BalancerPoolAdaptorV1CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *BalancerPoolAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

type BalancerSingleSwap struct {
	poolId   string
	kind     int
	assetIn  common.Address
	assetOut common.Address
	amount   *big.Int
	userData []byte
}

type BalancerSwapData struct {
	minAmountsForSwaps []*big.Int
	swapDeadlines      []*big.Int
}

func (b *BalancerPoolAdaptorV1CallBuilder) JoinPool(targetBpt common.Address, swapsBeforeJoin []*BalancerSingleSwap, swapData BalancerSwapData, minimumBPT *big.Int) *BalancerPoolAdaptorV1CallBuilder {
	singleSwaps := make([]*steward_proto.BalancerPoolAdaptorV1_SingleSwap, len(swapsBeforeJoin))
	for i, swap := range swapsBeforeJoin {
		singleSwaps[i] = &steward_proto.BalancerPoolAdaptorV1_SingleSwap{
			PoolId:   swap.poolId[:],
			Kind:     steward_proto.BalancerPoolAdaptorV1_SwapKind(swap.kind),
			AssetIn:  swap.assetIn.Hex(),
			AssetOut: swap.assetOut.Hex(),
			Amount:   swap.amount.String(),
			UserData: swap.userData,
		}
	}
	data := &steward_proto.BalancerPoolAdaptorV1_SwapData{
		MinAmountsForSwaps: make([]string, len(swapData.minAmountsForSwaps)),
		SwapDeadlines:      make([]string, len(swapData.swapDeadlines)),
	}
	for i, amount := range swapData.minAmountsForSwaps {
		data.MinAmountsForSwaps[i] = amount.String()
	}
	for i, deadline := range swapData.swapDeadlines {
		data.SwapDeadlines[i] = deadline.String()
	}

	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_JoinPool_{
			JoinPool: &steward_proto.BalancerPoolAdaptorV1_JoinPool{
				TargetBpt:       targetBpt.Hex(),
				SwapsBeforeJoin: singleSwaps,
				SwapData:        data,
				MinimumBpt:      minimumBPT.String(),
			},
		},
	})

	return b
}

type BalancerExitPoolRequest struct {
	assets            []common.Address
	minAmountsOut     []big.Int
	userData          []byte
	toInternalBalance bool
}

func (b *BalancerPoolAdaptorV1CallBuilder) ExitPool(targetBpt common.Address, swapsAfterExit []*BalancerSingleSwap, swapData BalancerSwapData, exitRequest *BalancerExitPoolRequest) *BalancerPoolAdaptorV1CallBuilder {
	singleSwaps := make([]*steward_proto.BalancerPoolAdaptorV1_SingleSwap, len(swapsAfterExit))
	for i, swap := range swapsAfterExit {
		singleSwaps[i] = &steward_proto.BalancerPoolAdaptorV1_SingleSwap{
			PoolId:   swap.poolId[:],
			Kind:     steward_proto.BalancerPoolAdaptorV1_SwapKind(swap.kind),
			AssetIn:  swap.assetIn.Hex(),
			AssetOut: swap.assetOut.Hex(),
			Amount:   swap.amount.String(),
			UserData: swap.userData,
		}
	}
	data := &steward_proto.BalancerPoolAdaptorV1_SwapData{
		MinAmountsForSwaps: make([]string, len(swapData.minAmountsForSwaps)),
		SwapDeadlines:      make([]string, len(swapData.swapDeadlines)),
	}
	for i, amount := range swapData.minAmountsForSwaps {
		data.MinAmountsForSwaps[i] = amount.String()
	}
	for i, deadline := range swapData.swapDeadlines {
		data.SwapDeadlines[i] = deadline.String()
	}

	assets := make([]string, len(exitRequest.assets))
	minAmountsOut := make([]string, len(exitRequest.minAmountsOut))
	for i, asset := range exitRequest.assets {
		assets[i] = asset.Hex()
	}
	for i, amount := range exitRequest.minAmountsOut {
		minAmountsOut[i] = amount.String()
	}

	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_ExitPool_{
			ExitPool: &steward_proto.BalancerPoolAdaptorV1_ExitPool{
				TargetBpt:      targetBpt.Hex(),
				SwapsAfterExit: singleSwaps,
				SwapData:       data,
				Request: &steward_proto.BalancerPoolAdaptorV1_ExitPoolRequest{
					Assets:            assets,
					MinAmountsOut:     minAmountsOut,
					UserData:          exitRequest.userData,
					ToInternalBalance: exitRequest.toInternalBalance,
				},
			},
		},
	})

	return b
}

func (b *BalancerPoolAdaptorV1CallBuilder) StakeBpt(bpt common.Address, liquidityGauge common.Address, amountIn *big.Int) *BalancerPoolAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_StakeBpt{
			StakeBpt: &steward_proto.BalancerPoolAdaptorV1_StakeBPT{
				Bpt:            bpt.Hex(),
				LiquidityGauge: liquidityGauge.Hex(),
				AmountIn:       amountIn.String(),
			},
		},
	})

	return b
}

func (b *BalancerPoolAdaptorV1CallBuilder) UnstakeBpt(bpt common.Address, liquidityGauge common.Address, amountOut *big.Int) *BalancerPoolAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_UnstakeBpt{
			UnstakeBpt: &steward_proto.BalancerPoolAdaptorV1_UnstakeBPT{
				Bpt:            bpt.Hex(),
				LiquidityGauge: liquidityGauge.Hex(),
				AmountOut:      amountOut.String(),
			},
		},
	})

	return b
}

func (b *BalancerPoolAdaptorV1CallBuilder) ClaimRewards(gauge common.Address) *BalancerPoolAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1{
		Function: &steward_proto.BalancerPoolAdaptorV1_ClaimRewards_{
			ClaimRewards: &steward_proto.BalancerPoolAdaptorV1_ClaimRewards{
				Gauge: gauge.Hex(),
			},
		},
	})

	return b
}
