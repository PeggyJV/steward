package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

type AaveV2ATokenAdaptorV2CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.AaveATokenAdaptorV2
}

func NewAaveV2ATokenAdaptorV2CallBuilder(adaptor common.Address) *AaveV2ATokenAdaptorV2CallBuilder {
	return &AaveV2ATokenAdaptorV2CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.AaveATokenAdaptorV2, 0),
	}
}

func (b *AaveV2ATokenAdaptorV2CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_AaveATokenV2Calls{
			AaveATokenV2Calls: &steward_proto.AaveATokenAdaptorV2Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *AaveV2ATokenAdaptorV2CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *AaveV2ATokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveATokenAdaptorV2{
		Function: &steward_proto.AaveATokenAdaptorV2_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

func (b *AaveV2ATokenAdaptorV2CallBuilder) DepositToAave(token common.Address, amount *big.Int) *AaveV2ATokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveATokenAdaptorV2{
		Function: &steward_proto.AaveATokenAdaptorV2_DepositToAave_{
			DepositToAave: &steward_proto.AaveATokenAdaptorV2_DepositToAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}

func (b *AaveV2ATokenAdaptorV2CallBuilder) WithdrawFromAave(token common.Address, amount *big.Int) *AaveV2ATokenAdaptorV2CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveATokenAdaptorV2{
		Function: &steward_proto.AaveATokenAdaptorV2_WithdrawFromAave_{
			WithdrawFromAave: &steward_proto.AaveATokenAdaptorV2_WithdrawFromAave{
				Token:  token.Hex(),
				Amount: amount.String(),
			},
		},
	})

	return b
}
