package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// BalancerPoolAdaptorV1CallBuilder is a builder for using flash loans with the BalancerPoolAdaptorV1 adaptor.
// Contract: https://github.com/PeggyJV/cellar-contracts/blob/main/src/modules/adaptors/Balancer/BalancerPoolAdaptor.sol
type BalancerPoolFlashLoanAdaptorV1CallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.BalancerPoolAdaptorV1FlashLoan
}

// Constructor
func NewBalancerPoolFlashLoanAdaptorV1CallBuilder(adaptor common.Address) *BalancerPoolFlashLoanAdaptorV1CallBuilder {
	return &BalancerPoolFlashLoanAdaptorV1CallBuilder{
		adaptor: adaptor,
	}
}

// Build
func (b *BalancerPoolFlashLoanAdaptorV1CallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_BalancerPoolV1FlashLoanCalls{
			BalancerPoolV1FlashLoanCalls: &steward_proto.BalancerPoolAdaptorV1FlashLoanCalls{
				Calls: b.calls,
			},
		},
	}
}

// FlashLoan
func (b *BalancerPoolFlashLoanAdaptorV1CallBuilder) FlashLoan(loanTokens []common.Address, loanAmounts []*big.Int, userData []*steward_proto.AdaptorCall) *BalancerPoolFlashLoanAdaptorV1CallBuilder {
	// convert loan tokens to strings
	tokensStr := make([]string, len(loanTokens))
	for i, token := range loanTokens {
		tokensStr[i] = token.Hex()
	}
	amountsStr := make([]string, len(loanAmounts))
	for i, amount := range loanAmounts {
		amountsStr[i] = amount.String()
	}
	converted := make([]*steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan, len(userData))
	for i, call := range userData {
		converted[i] = convertToBalancerPoolFlashLoanAdaptorCall(call)
	}
	b.calls = append(b.calls, &steward_proto.BalancerPoolAdaptorV1FlashLoan{
		MakeFlashLoan: &steward_proto.BalancerPoolAdaptorV1FlashLoan_MakeFlashLoan{
			Tokens:  tokensStr,
			Amounts: amountsStr,
			Data:    converted,
		},
	})

	return b
}

func convertToBalancerPoolFlashLoanAdaptorCall(call *steward_proto.AdaptorCall) *steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan {
	switch a := call.CallData.(type) {
	case *steward_proto.AdaptorCall_AaveATokenV2Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AaveATokenV2Calls{
				AaveATokenV2Calls: a.AaveATokenV2Calls,
			},
		}
	case *steward_proto.AdaptorCall_AaveDebtTokenV2Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AaveDebtTokenV2Calls{
				AaveDebtTokenV2Calls: a.AaveDebtTokenV2Calls,
			},
		}
	case *steward_proto.AdaptorCall_AaveV3ATokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AaveV3ATokenV1Calls{
				AaveV3ATokenV1Calls: a.AaveV3ATokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_CompoundCTokenV2Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_CompoundCTokenV2Calls{
				CompoundCTokenV2Calls: a.CompoundCTokenV2Calls,
			},
		}
	case *steward_proto.AdaptorCall_OneInchV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_OneInchV1Calls{
				OneInchV1Calls: a.OneInchV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_FeesAndReservesV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_FeesAndReservesV1Calls{
				FeesAndReservesV1Calls: a.FeesAndReservesV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_ZeroXV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_ZeroXV1Calls{
				ZeroXV1Calls: a.ZeroXV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_SwapWithUniswapV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_SwapWithUniswapV1Calls{
				SwapWithUniswapV1Calls: a.SwapWithUniswapV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_VestingSimpleV2Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_VestingSimpleV2Calls{
				VestingSimpleV2Calls: a.VestingSimpleV2Calls,
			},
		}
	case *steward_proto.AdaptorCall_CellarV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_CellarV1Calls{
				CellarV1Calls: a.CellarV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_UniswapV3V2Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_UniswapV3V2Calls{
				UniswapV3V2Calls: a.UniswapV3V2Calls,
			},
		}
	case *steward_proto.AdaptorCall_AaveV2EnableAssetAsCollateralV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AaveV2EnableAssetAsCollateralV1Calls{
				AaveV2EnableAssetAsCollateralV1Calls: a.AaveV2EnableAssetAsCollateralV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_FTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_FTokenV1Calls{
				FTokenV1Calls: a.FTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoAaveV2ATokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoAaveV2ATokenV1Calls{
				MorphoAaveV2ATokenV1Calls: a.MorphoAaveV2ATokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoAaveV2DebtTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoAaveV2DebtTokenV1Calls{
				MorphoAaveV2DebtTokenV1Calls: a.MorphoAaveV2DebtTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoAaveV3ATokenCollateralV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoAaveV3ATokenCollateralV1Calls{
				MorphoAaveV3ATokenCollateralV1Calls: a.MorphoAaveV3ATokenCollateralV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoAaveV3ATokenP2PV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoAaveV3ATokenP2PV1Calls{
				MorphoAaveV3ATokenP2PV1Calls: a.MorphoAaveV3ATokenP2PV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoAaveV3DebtTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoAaveV3DebtTokenV1Calls{
				MorphoAaveV3DebtTokenV1Calls: a.MorphoAaveV3DebtTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_AaveV3DebtTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AaveV3DebtTokenV1Calls{
				AaveV3DebtTokenV1Calls: a.AaveV3DebtTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_LegacyCellarV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_LegacyCellarV1Calls{
				LegacyCellarV1Calls: a.LegacyCellarV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_DebtFTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_DebtFTokenV1Calls{
				DebtFTokenV1Calls: a.DebtFTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_CollateralFTokenV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_CollateralFTokenV1Calls{
				CollateralFTokenV1Calls: a.CollateralFTokenV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_ConvexCurveV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_ConvexCurveV1Calls{
				ConvexCurveV1Calls: a.ConvexCurveV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_CurveV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_CurveV1Calls{
				CurveV1Calls: a.CurveV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_AuraErc4626V1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_AuraErc4626V1Calls{
				AuraErc4626V1Calls: a.AuraErc4626V1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoBlueCollateralV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoBlueCollateralV1Calls{
				MorphoBlueCollateralV1Calls: a.MorphoBlueCollateralV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoBlueDebtV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoBlueDebtV1Calls{
				MorphoBlueDebtV1Calls: a.MorphoBlueDebtV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_MorphoBlueSupplyV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_MorphoBlueSupplyV1Calls{
				MorphoBlueSupplyV1Calls: a.MorphoBlueSupplyV1Calls,
			},
		}
	case *steward_proto.AdaptorCall_Erc4626V1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_Erc4626V1Calls{
				Erc4626V1Calls: a.Erc4626V1Calls,
			},
		}
	case *steward_proto.AdaptorCall_StakingV1Calls:
		return &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan{
			Adaptor: call.Adaptor,
			CallData: &steward_proto.BalancerPoolAdaptorV1FlashLoan_AdaptorCallForBalancerPoolFlashLoan_StakingV1Calls{
				StakingV1Calls: a.StakingV1Calls,
			},
		}
	default:
		return nil
	}
}
