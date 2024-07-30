package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// AaveV3DebtTokenAdaptorV1CallBuilder is a builder for AaveV3DebtTokenAdaptorV1 calls
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Aave/V3/AaveV3DebtTokenAdaptor.sol
type AaveV3DebtTokenAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.AaveV3DebtTokenAdaptorV1
}

func NewAaveV3DebtTokenAdaptorV1CallBuilder(adaptor common.Address) *AaveV3DebtTokenAdaptorV1CallBuilder {
	return &AaveV3DebtTokenAdaptorV1CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.AaveV3DebtTokenAdaptorV1, 0),
	}
}

func (b *AaveV3DebtTokenAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_AaveV3DebtTokenV1Calls{
			AaveV3DebtTokenV1Calls: &steward_proto.AaveV3DebtTokenAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *AaveV3DebtTokenAdaptorV1CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *AaveV3DebtTokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3DebtTokenAdaptorV1{
		Function: &steward_proto.AaveV3DebtTokenAdaptorV1_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

func (b *AaveV3DebtTokenAdaptorV1CallBuilder) BorrowFromAave(token common.Address, amount *big.Int) *AaveV3DebtTokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3DebtTokenAdaptorV1{
		Function: &steward_proto.AaveV3DebtTokenAdaptorV1_BorrowFromAave_{
			BorrowFromAave: &steward_proto.AaveV3DebtTokenAdaptorV1_BorrowFromAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV3DebtTokenAdaptorV1CallBuilder) RepayAaveDebt(token common.Address, amount *big.Int) *AaveV3DebtTokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3DebtTokenAdaptorV1{
		Function: &steward_proto.AaveV3DebtTokenAdaptorV1_RepayAaveDebt_{
			RepayAaveDebt: &steward_proto.AaveV3DebtTokenAdaptorV1_RepayAaveDebt{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV3DebtTokenAdaptorV1CallBuilder) RepayWithATokens(underlyingToken common.Address, amount *big.Int) *AaveV3DebtTokenAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV3DebtTokenAdaptorV1{
		Function: &steward_proto.AaveV3DebtTokenAdaptorV1_RepayWithATokens_{
			RepayWithATokens: &steward_proto.AaveV3DebtTokenAdaptorV1_RepayWithATokens{
				UnderlyingToken: underlyingToken.Hex(),
				Amount:          amount.String(),
			},
		},
	})

	return b
}
