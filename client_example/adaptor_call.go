package main
			CallData: &steward_proto.ScheduleRequest_CellarV2_2{
				CellarV2_2: &steward_proto.CellarV2_2{
					CallType: &steward_proto.CellarV2_2_Multicall_{
						Multicall: &steward_proto.CellarV2_2_Multicall{
							FunctionCalls: []*steward_proto.CellarV2_2_FunctionCall{
								{
									Function: &steward_proto.CellarV2_2_FunctionCall_CallOnAdaptor{
										CallOnAdaptor: &steward_proto.CellarV2_2_CallOnAdaptor{
											Data: []*steward_proto.AdaptorCall{
												{
													Adaptor: adaptorContract.Hex(),
													CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
														SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
															Calls: []*steward_proto.SwapWithUniswapAdaptorV1{
																{
																	Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
																		SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
																			Path:         []string{"0x1111111111111111111111111111111111111111", "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},
																			PoolFees:     []uint32{1000, 2000},
																			Amount:       "2",
																			AmountOutMin: "2",
																		},
																	},
																},
															},
														},
													},
												},
											},
										},
									},
								},
								{
									Function: &steward_proto.CellarV2_2_FunctionCall_CallOnAdaptor{
										CallOnAdaptor: &steward_proto.CellarV2_2_CallOnAdaptor{
											Data: []*steward_proto.AdaptorCall{
												{
													Adaptor: adaptorContract.Hex(),
													CallData: &steward_proto.AdaptorCall_SwapWithUniswapV1Calls{
														SwapWithUniswapV1Calls: &steward_proto.SwapWithUniswapAdaptorV1Calls{
															Calls: []*steward_proto.SwapWithUniswapAdaptorV1{
																{
																	Function: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3_{
																		SwapWithUniV3: &steward_proto.SwapWithUniswapAdaptorV1_SwapWithUniV3{
																			Path:         []string{"0x1111111111111111111111111111111111111111", "0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},
																			PoolFees:     []uint32{1000, 2000},
																			Amount:       "3",
																			AmountOutMin: "3",
																		},
																	},
																},
															},
														},
													},
												},
											},
										},
									},
								},
							},
						},
					},
				},
			},
	
