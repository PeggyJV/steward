package adaptors

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder builds calls for the AaveV2EnableAssetAsCollateralAdaptorV1 adaptor
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Aave/AaveV2EnableAssetAsCollateralAdaptor.sol
type AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1
}

func NewAaveV2EnableAssetAsCollateralAdaptorV1CallBuilder(adaptor common.Address) *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder {
	return &AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1, 0),
	}
}

func (b *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_AaveV2EnableAssetAsCollateralV1Calls{
			AaveV2EnableAssetAsCollateralV1Calls: &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

func (b *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder) RevokeApproval(asset common.Address, spender common.Address) *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1{
		Function: &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1_RevokeApproval{
			RevokeApproval: &steward_proto.RevokeApproval{
				Asset:   asset.Hex(),
				Spender: spender.Hex(),
			},
		},
	})

	return b
}

func (b *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder) SetUserUseReserveAsCollateral(asset common.Address, useAsCollateral bool) *AaveV2EnableAssetAsCollateralAdaptorV1CallBuilder {
	b.calls = append(b.calls, &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1{
		Function: &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_{
			SetUserUseReserveAsCollateral: &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral{
				Asset:           asset.Hex(),
				UseAsCollateral: useAsCollateral,
			},
		},
	})

	return b
}
