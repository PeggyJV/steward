package builder

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// CellarCallDataBuilder is the builder for cellar call data. It can be used to string together multiple function calls to one Cellar. Function calls will execute in the order they are added to the builder.
type CellarCallDataBuilder struct {
	functionCalls []*steward_proto.CellarV2_5_FunctionCall
}

// NewCellarCallDataBuilder creates a new CallDataBuilder
func NewCellarCallDataBuilder() *CellarCallDataBuilder {
	return &CellarCallDataBuilder{
		functionCalls: make([]*steward_proto.CellarV2_5_FunctionCall, 0),
	}
}

func (cdb *CellarCallDataBuilder) GetFunctionCalls() []*steward_proto.CellarV2_5_FunctionCall {
	return cdb.functionCalls
}

// Build returns the CallData struct with the function calls added to the builder
func (cdb *CellarCallDataBuilder) Build() (*steward_proto.CellarV2_5, error) {
	if len(cdb.functionCalls) == 0 {
		return nil, fmt.Errorf("no function calls added to CallDataBuilder")
	} else if len(cdb.functionCalls) == 1 {
		return &steward_proto.CellarV2_5{
			CallType: &steward_proto.CellarV2_5_FunctionCall_{
				FunctionCall: cdb.functionCalls[0],
			},
		}, nil
	} else {
		return &steward_proto.CellarV2_5{
			CallType: &steward_proto.CellarV2_5_Multicall_{
				Multicall: &steward_proto.CellarV2_5_Multicall{
					FunctionCalls: cdb.functionCalls,
				},
			},
		}, nil
	}
}

// CallOnAdaptor adds call data for the CallOnAdaptor function to the builder
func (cdb *CellarCallDataBuilder) CallOnAdaptor(adaptorCalls ...*steward_proto.AdaptorCall) *CellarCallDataBuilder {
	// search functionCalls for type steward_proto.CellarV2_5_FunctionCall_CallOnAdaptor and if
	// it already exists, append to CallOnAdaptor.Data instead of creating a new CallOnAdaptor struct
	found := false
	for _, call := range cdb.functionCalls {
		if call.GetCallOnAdaptor() != nil {
			found = true
			call.GetCallOnAdaptor().Data = append(call.GetCallOnAdaptor().Data, adaptorCalls...)
			break
		}
	}

	if !found {
		cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
			Function: &steward_proto.CellarV2_5_FunctionCall_CallOnAdaptor{
				CallOnAdaptor: &steward_proto.CellarV2_5_CallOnAdaptor{
					Data: adaptorCalls,
				},
			},
		})
	}

	return cdb
}

// AddPosition adds call data for the AddPosition function to the builder
func (cdb *CellarCallDataBuilder) AddPosition(index uint32, positionId uint32, configurationData []byte, inDebtArray bool) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_AddPosition{
			AddPosition: &steward_proto.CellarV2_5_AddPosition{
				Index:             index,
				PositionId:        positionId,
				ConfigurationData: configurationData,
				InDebtArray:       inDebtArray,
			},
		},
	})

	return cdb
}

// RemovePosition adds call data for the RemovePosition function to the builder
func (cdb *CellarCallDataBuilder) RemovePosition(index uint32, inDebtArray bool) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_RemovePosition{
			RemovePosition: &steward_proto.CellarV2_5_RemovePosition{
				Index:       index,
				InDebtArray: inDebtArray,
			},
		},
	})

	return cdb
}

// SetHoldingPosition adds call data for the SetHoldingPosition function to the builder
func (cdb *CellarCallDataBuilder) SetHoldingPosition(positionId uint32) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetHoldingPosition{
			SetHoldingPosition: &steward_proto.CellarV2_5_SetHoldingPosition{
				PositionId: positionId,
			},
		},
	})

	return cdb
}

// SetStrategistPayoutAddress adds call data for the SetStrategistPayoutAddress function to the builder
func (cdb *CellarCallDataBuilder) SetStrategistPayoutAddress(payout common.Address) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetStrategistPayoutAddress{
			SetStrategistPayoutAddress: &steward_proto.CellarV2_5_SetStrategistPayoutAddress{
				Payout: payout.Hex(),
			},
		},
	})

	return cdb
}

// SwapPositions adds call data for the SwapPositions function to the builder
func (cdb *CellarCallDataBuilder) SwapPositions(index1, index2 uint32, inDebtArray bool) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SwapPositions{
			SwapPositions: &steward_proto.CellarV2_5_SwapPositions{
				Index_1:     index1,
				Index_2:     index2,
				InDebtArray: inDebtArray,
			},
		},
	})

	return cdb
}

// SetShareLockPeriod adds call data for the SetShareLockPeriod function to the builder
func (cdb *CellarCallDataBuilder) SetShareLockPeriod(newLock *big.Int) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetShareLockPeriod{
			SetShareLockPeriod: &steward_proto.CellarV2_5_SetShareLockPeriod{
				NewLock: newLock.String(),
			},
		},
	})

	return cdb
}

// InitiateShutdown adds call data for the InitiateShutdown function to the builder
func (cdb *CellarCallDataBuilder) InitiateShutdown() *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_InitiateShutdown{
			InitiateShutdown: &steward_proto.CellarV2_5_InitiateShutdown{},
		},
	})

	return cdb
}

// LiftShutdown adds call data for the LiftShutdown function to the builder
func (cdb *CellarCallDataBuilder) LiftShutdown() *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_LiftShutdown{
			LiftShutdown: &steward_proto.CellarV2_5_LiftShutdown{},
		},
	})

	return cdb
}

// AddAdaptorToCatalogue adds call data for the AddAdaptorToCatalogue function to the builder
func (cdb *CellarCallDataBuilder) AddAdaptorToCatalogue(adaptor common.Address) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_AddAdaptorToCatalogue{
			AddAdaptorToCatalogue: &steward_proto.CellarV2_5_AddAdaptorToCatalogue{
				Adaptor: adaptor.Hex(),
			},
		},
	})

	return cdb
}

// RemoveAdaptorFromCatalogue adds call data for the RemoveAdaptorFromCatalogue function to the builder
func (cdb *CellarCallDataBuilder) RemoveAdaptorFromCatalogue(adaptor common.Address) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_RemoveAdaptorFromCatalogue{
			RemoveAdaptorFromCatalogue: &steward_proto.CellarV2_5_RemoveAdaptorFromCatalogue{
				Adaptor: adaptor.Hex(),
			},
		},
	})

	return cdb
}

// RemovePositionFromCatalogue adds call data for the RemovePositionFromCatalogue function to the builder
func (cdb *CellarCallDataBuilder) RemovePositionFromCatalogue(positionId uint32) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_RemovePositionFromCatalogue{
			RemovePositionFromCatalogue: &steward_proto.CellarV2_5_RemovePositionFromCatalogue{
				PositionId: positionId,
			},
		},
	})

	return cdb
}

// DecreaseShareSupplyCap adds call data for the DecreaseShareSupplyCap function to the builder
func (cdb *CellarCallDataBuilder) DecreaseShareSupplyCap(newCap *big.Int) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_DecreaseShareSupplyCap{
			DecreaseShareSupplyCap: &steward_proto.CellarV2_5_DecreaseShareSupplyCap{
				NewCap: newCap.String(),
			},
		},
	})

	return cdb
}

// AddPositionToCatalogue adds call data for the AddPositionToCatalogue function to the builder
func (cdb *CellarCallDataBuilder) AddPositionToCatalogue(positionId uint32) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_AddPositionToCatalogue{
			AddPositionToCatalogue: &steward_proto.CellarV2_5_AddPositionToCatalogue{
				PositionId: positionId,
			},
		},
	})

	return cdb
}

// SetRebalanceDeviation adds call data for the SetRebalanceDeviation function to the builder
func (cdb *CellarCallDataBuilder) SetRebalanceDeviation(newDeviation *big.Int) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetRebalanceDeviation{
			SetRebalanceDeviation: &steward_proto.CellarV2_5_SetRebalanceDeviation{
				NewDeviation: newDeviation.String(),
			},
		},
	})

	return cdb
}

// SetStrategistPlatformCut adds call data for the SetStrategistPlatformCut function to the builder
func (cdb *CellarCallDataBuilder) SetStrategistPlatformCut(newCut uint64) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetStrategistPlatformCut{
			SetStrategistPlatformCut: &steward_proto.CellarV2_5_SetStrategistPlatformCut{
				NewCut: newCut,
			},
		},
	})

	return cdb
}

// IncreaseShareSupplyCap adds call data for the IncreaseShareSupplyCap function to the builder
func (cdb *CellarCallDataBuilder) IncreaseShareSupplyCap(newCap *big.Int) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_IncreaseShareSupplyCap{
			IncreaseShareSupplyCap: &steward_proto.CellarV2_5_IncreaseShareSupplyCap{
				NewCap: newCap.String(),
			},
		},
	})

	return cdb
}

// SetSharePriceOracle adds call data for the SetSharePriceOracle function to the builder
func (cdb *CellarCallDataBuilder) SetSharePriceOracle(registryId *big.Int, sharePriceOracle common.Address) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_SetSharePriceOracle{
			SetSharePriceOracle: &steward_proto.CellarV2_5_SetSharePriceOracle{
				RegistryId:       registryId.String(),
				SharePriceOracle: sharePriceOracle.Hex(),
			},
		},
	})

	return cdb
}

// CachePriceRouter adds call data for the CachePriceRouter function to the builder
func (cdb *CellarCallDataBuilder) CachePriceRouter(checkTotalAssets bool, allowableRange uint32, expectedPriceRouter common.Address) *CellarCallDataBuilder {
	cdb.functionCalls = append(cdb.functionCalls, &steward_proto.CellarV2_5_FunctionCall{
		Function: &steward_proto.CellarV2_5_FunctionCall_CachePriceRouter{
			CachePriceRouter: &steward_proto.CellarV2_5_CachePriceRouter{
				CheckTotalAssets:    checkTotalAssets,
				AllowableRange:      allowableRange,
				ExpectedPriceRouter: expectedPriceRouter.Hex(),
			},
		},
	})

	return cdb
}
