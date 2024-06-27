package main

import (
	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

func BuildUniswapV3ScheduleRequest(cellarId common.Address, chainId uint, adaptorContract common.Address, blockHeight uint) *steward_proto.ScheduleRequest {
    return &steward_proto.ScheduleRequest{
        CellarId:    cellarId.Hex(), 
        ChainId:     uint64(chainId),
        CallData:   &steward_proto.ScheduleRequest_CellarV2_5{
            CellarV2_5: &steward_proto.CellarV2_5{
                CallType: &steward_proto.CellarV2_5_FunctionCall_{
                    FunctionCall: &steward_proto.CellarV2_5_FunctionCall{
                        Function: &steward_proto.CellarV2_5_FunctionCall_CallOnAdaptor{
                            CallOnAdaptor: &steward_proto.CellarV2_5_CallOnAdaptor{
                                Data: []*steward_proto.AdaptorCall{
                                    SwapWithUniswapV3AdaptorCall(
                                        adaptorContract,
                                        []string{
                                            "0x00000000000000000000000000000000000000000000",
                                            "0x11111111111111111111111111111111111111111111",
                                            "0x22222222222222222222222222222222222222222222",
                                        },
                                        []uint32{1000, 2000, 3000},
                                        "100000000",
                                        "100000000",
                                    ),
                                },
                            },
                        },
                    },
                },
            },
        },
        BlockHeight: uint64(blockHeight),
        Deadline: 0,
    }
}

func SwapWithUniswapV3AdaptorCall(adaptorContract common.Address, path []string, poolFees []uint32, amount string, amountOutMin string) *steward_proto.AdaptorCall {
    return &steward_proto.AdaptorCall{
        Adaptor: adaptorContract.Hex(),
        CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
            SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
                Calls: []*steward_proto.SwapWithUniswapAdaptorV1{
                    {
                        Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
                            SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
                                Path: path, 
                                PoolFees: poolFees,
                                Amount: amount,
                                AmountOutMin: amountOutMin,
                            },
                        },
                    },
                },
            },
        },
    }
}
