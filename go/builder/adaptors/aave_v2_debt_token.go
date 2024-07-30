package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// AaveV2DebtTokenAdaptorV2CallBuilder is a builder for AaveV2DebtTokenAdaptorV2 calls.
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Aave/AaveDebtTokenAdaptor.sol
type AaveV2DebtTokenAdaptorV2CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.AaveDebtTokenAdaptorV2
}

func NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor common.Address) *AaveV2DebtTokenAdaptorV2CallBuilder {
	return &AaveV2DebtTokenAdaptorV2CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.AaveDebtTokenAdaptorV2, 0),
	}
}

func (b *AaveV2DebtTokenAdaptorV2CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_AaveDebtTokenV2Calls{
			AaveDebtTokenV2Calls: &steward_proto.AaveDebtTokenAdaptorV2Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *AaveV2DebtTokenAdaptorV2CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *AaveV2DebtTokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveDebtTokenAdaptorV2{
		Function: &steward_proto.AaveDebtTokenAdaptorV2_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

func (b *AaveV2DebtTokenAdaptorV2CallBuilder) BorrowFromAave(token common.Address, amount *big.Int) *AaveV2DebtTokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveDebtTokenAdaptorV2{
		Function: &steward_proto.AaveDebtTokenAdaptorV2_BorrowFromAave_{
			BorrowFromAave: &steward_proto.AaveDebtTokenAdaptorV2_BorrowFromAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV2DebtTokenAdaptorV2CallBuilder) RepayAaveDebt(token common.Address, amount *big.Int) *AaveV2DebtTokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveDebtTokenAdaptorV2{
		Function: &steward_proto.AaveDebtTokenAdaptorV2_RepayAaveDebt_{
			RepayAaveDebt: &steward_proto.AaveDebtTokenAdaptorV2_RepayAaveDebt{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}
