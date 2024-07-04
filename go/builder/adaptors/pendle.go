package adaptors

import (
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// Builder
type PendleAdaptorCallBuilder struct {
	adaptor common.Address
	calls   []*steward_proto.PendleAdaptorV1
}

// Constructor
func NewPendleAdaptorCall(adaptor common.Address) *PendleAdaptorCallBuilder {
	return &PendleAdaptorCallBuilder{
		adaptor: adaptor,
		calls:   make([]*steward_proto.PendleAdaptorV1, 0),
	}
}

// Build
func (b *PendleAdaptorCallBuilder) Build() *steward_proto.AdaptorCall {
	return &steward_proto.AdaptorCall{
		Adaptor: b.adaptor.Hex(),
		CallData: &steward_proto.AdaptorCall_PendleV1Calls{
			PendleV1Calls: &steward_proto.PendleAdaptorV1Calls{
				Calls: b.calls,
			},
		},
	}
}

type PendleSwapData struct {
	SwapType    int32
	ExtRouter   common.Address
	ExtCallData string
	NeedScale   bool
}

func convertPendleSwapData(data PendleSwapData) *steward_proto.PendleAdaptorV1_SwapData {
	return &steward_proto.PendleAdaptorV1_SwapData{
		SwapType:    data.SwapType,
		ExtRouter:   data.ExtRouter.Hex(),
		ExtCalldata: data.ExtCallData,
		NeedScale:   data.NeedScale,
	}
}

type PendleTokenInput struct {
	TokenIn     common.Address
	NetTokenIn  big.Int
	TokenMintSy common.Address
	PendleSwap  common.Address
	SwapData    PendleSwapData
}

func convertPendleTokenInput(input PendleTokenInput) *steward_proto.PendleAdaptorV1_TokenInput {
	return &steward_proto.PendleAdaptorV1_TokenInput{
		TokenIn:     input.TokenIn.Hex(),
		NetTokenIn:  input.NetTokenIn.String(),
		TokenMintSy: input.TokenMintSy.Hex(),
		PendleSwap:  input.PendleSwap.Hex(),
		SwapData:    convertPendleSwapData(input.SwapData),
	}
}

type PendleApproxParams struct {
	GuessMin      big.Int
	GuessMax      big.Int
	GuessOffchain big.Int
	MaxIteration  big.Int
	Eps           big.Int
}

func convertPendleApproxParams(params PendleApproxParams) *steward_proto.PendleAdaptorV1_ApproxParams {
	return &steward_proto.PendleAdaptorV1_ApproxParams{
		GuessMin:      params.GuessMin.String(),
		GuessMax:      params.GuessMax.String(),
		GuessOffchain: params.GuessOffchain.String(),
		MaxIteration:  params.MaxIteration.String(),
		Eps:           params.Eps.String(),
	}
}

type PendleTokenOutput struct {
	TokenOut      common.Address
	MinTokenOut   big.Int
	TokenRedeemSy common.Address
	PendleSwap    common.Address
	SwapData      PendleSwapData
}

func convertPendleTokenOutput(output PendleTokenOutput) *steward_proto.PendleAdaptorV1_TokenOutput {
	return &steward_proto.PendleAdaptorV1_TokenOutput{
		TokenOut:      output.TokenOut.Hex(),
		MinTokenOut:   output.MinTokenOut.String(),
		TokenRedeemSy: output.TokenRedeemSy.Hex(),
		PendleSwap:    output.PendleSwap.Hex(),
		SwapData:      convertPendleSwapData(output.SwapData),
	}
}

func (b *PendleAdaptorCallBuilder) MintSyFromToken(market common.Address, minSyOut big.Int, tokenInput PendleTokenInput) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_MintSyFromToken_{
			MintSyFromToken: &steward_proto.PendleAdaptorV1_MintSyFromToken{
				Market:   market.Hex(),
				MinSyOut: minSyOut.String(),
				Input:    convertPendleTokenInput(tokenInput),
			},
		},
	})

	return b
}

func (b *PendleAdaptorCallBuilder) MintPyFromSy(market common.Address, netSyIn big.Int, minPyOut big.Int) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_MintPyFromSy_{
			MintPyFromSy: &steward_proto.PendleAdaptorV1_MintPyFromSy{
				Market:   market.Hex(),
				NetSyIn:  netSyIn.String(),
				MinPyOut: minPyOut.String(),
			},
		},
	})

	return b
}

func (b *PendleAdaptorCallBuilder) SwapExactPtForYt(market common.Address, exactPtIn big.Int, minYtOut big.Int, guessTotalYtToSwap PendleApproxParams) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_SwapExactPtForYt_{
			SwapExactPtForYt: &steward_proto.PendleAdaptorV1_SwapExactPtForYt{
				Market:             market.Hex(),
				ExactPtIn:          exactPtIn.String(),
				MinYtOut:           minYtOut.String(),
				GuessTotalYtToSwap: convertPendleApproxParams(guessTotalYtToSwap),
			},
		},
	})

	return b
}

// SwapExactYtForPt
func (b *PendleAdaptorCallBuilder) SwapExactYtForPt(market common.Address, exactYtIn big.Int, minPtOut big.Int, guessTotalPtToSwap PendleApproxParams) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_SwapExactYtForPt_{
			SwapExactYtForPt: &steward_proto.PendleAdaptorV1_SwapExactYtForPt{
				Market:             market.Hex(),
				ExactYtIn:          exactYtIn.String(),
				MinPtOut:           minPtOut.String(),
				GuessTotalPtToSwap: convertPendleApproxParams(guessTotalPtToSwap),
			},
		},
	})

	return b
}

// AddLiquidityDualSyAndPt
func (b *PendleAdaptorCallBuilder) AddLiquidityDualSyAndPt(market common.Address, netSyDesired big.Int, netPtDesired big.Int, minLpOut big.Int) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_AddLiquidityDualSyAndPt_{
			AddLiquidityDualSyAndPt: &steward_proto.PendleAdaptorV1_AddLiquidityDualSyAndPt{
				Market:       market.Hex(),
				NetSyDesired: netSyDesired.String(),
				NetPtDesired: netPtDesired.String(),
				MinLpOut:     minLpOut.String(),
			},
		},
	})

	return b
}

// RedeemPyToSy
func (b *PendleAdaptorCallBuilder) RedeemPyToSy(market common.Address, netPyIn big.Int, minSyOut big.Int) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_RedeemPyToSy_{
			RedeemPyToSy: &steward_proto.PendleAdaptorV1_RedeemPyToSy{
				Market:   market.Hex(),
				NetPyIn:  netPyIn.String(),
				MinSyOut: minSyOut.String(),
			},
		},
	})

	return b
}

// RedeemSyToToken
func (b *PendleAdaptorCallBuilder) RedeemSyToToken(market common.Address, netSyIn big.Int, tokenOutput PendleTokenOutput) *PendleAdaptorCallBuilder {
	b.calls = append(b.calls, &steward_proto.PendleAdaptorV1{
		Function: &steward_proto.PendleAdaptorV1_RedeemSyToToken_{
			RedeemSyToToken: &steward_proto.PendleAdaptorV1_RedeemSyToToken{
				Market:  market.Hex(),
				NetSyIn: netSyIn.String(),
				Output:  convertPendleTokenOutput(tokenOutput),
			},
		},
	})

	return b
}
