package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// Builder for AaveV3ATokenAdaptorV1 adaptor
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Aave/V3/AaveV3ATokenAdaptor.sol
type AaveV3ATokenAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.AaveV3ATokenAdaptorV1
}

func NewAaveV3ATokenAdaptorV1CallBuilder(adaptor common.Address) *AaveV3ATokenAdaptorV1CallBuilder {
	return &AaveV3ATokenAdaptorV1CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.AaveV3ATokenAdaptorV1, 0),
	}
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_AaveV3ATokenV1Calls{
			AaveV3ATokenV1Calls: &steward_proto.AaveV3ATokenAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *AaveV3ATokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3ATokenAdaptorV1{
		Function: &steward_proto.AaveV3ATokenAdaptorV1_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) DepositToAave(token common.Address, amount *big.Int) *AaveV3ATokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3ATokenAdaptorV1{
		Function: &steward_proto.AaveV3ATokenAdaptorV1_DepositToAave_{
			DepositToAave: &steward_proto.AaveV3ATokenAdaptorV1_DepositToAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) WithdrawFromAave(token common.Address, amount *big.Int) *AaveV3ATokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3ATokenAdaptorV1{
		Function: &steward_proto.AaveV3ATokenAdaptorV1_WithdrawFromAave_{
			WithdrawFromAave: &steward_proto.AaveV3ATokenAdaptorV1_WithdrawFromAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) AdjustIsolationModeAssetAsCollateral(asset common.Address, useAsCollateral bool) *AaveV3ATokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3ATokenAdaptorV1{
		Function: &steward_proto.AaveV3ATokenAdaptorV1_AdjustIsolationModeAssetAsCollateral_{
			AdjustIsolationModeAssetAsCollateral: &steward_proto.AaveV3ATokenAdaptorV1_AdjustIsolationModeAssetAsCollateral{
				Asset:           asset.Hex(),
				UseAsCollateral: useAsCollateral,
			},
		},
	})

	return b
}

func (b *AaveV3ATokenAdaptorV1CallBuilder) ChangeEMode(categoryId uint32) *AaveV3ATokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3ATokenAdaptorV1{
		Function: &steward_proto.AaveV3ATokenAdaptorV1_ChangeEmode{
			ChangeEmode: &steward_proto.AaveV3ATokenAdaptorV1_ChangeEMode{
				CategoryId: categoryId,
			},
		},
	})

	return b
}
