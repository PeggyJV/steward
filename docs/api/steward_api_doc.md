# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [a_token.proto](#a_token-proto)
    - [AaveATokenAdaptorV1](#steward-v4-AaveATokenAdaptorV1)
    - [AaveATokenAdaptorV1.DepositToAave](#steward-v4-AaveATokenAdaptorV1-DepositToAave)
    - [AaveATokenAdaptorV1.WithdrawFromAave](#steward-v4-AaveATokenAdaptorV1-WithdrawFromAave)
    - [AaveATokenAdaptorV1Calls](#steward-v4-AaveATokenAdaptorV1Calls)
    - [AaveATokenAdaptorV2](#steward-v4-AaveATokenAdaptorV2)
    - [AaveATokenAdaptorV2.DepositToAave](#steward-v4-AaveATokenAdaptorV2-DepositToAave)
    - [AaveATokenAdaptorV2.WithdrawFromAave](#steward-v4-AaveATokenAdaptorV2-WithdrawFromAave)
    - [AaveATokenAdaptorV2Calls](#steward-v4-AaveATokenAdaptorV2Calls)
  
- [aave_v2_enable_asset_as_collateral_adaptor.proto](#aave_v2_enable_asset_as_collateral_adaptor-proto)
    - [AaveV2EnableAssetAsCollateralAdaptorV1](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1)
    - [AaveV2EnableAssetAsCollateralAdaptorV1.SetUserUseReserveAsCollateral](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1-SetUserUseReserveAsCollateral)
    - [AaveV2EnableAssetAsCollateralAdaptorV1Calls](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1Calls)
  
- [aave_v2_stablecoin.proto](#aave_v2_stablecoin-proto)
    - [AaveV2Stablecoin](#steward-v4-AaveV2Stablecoin)
    - [AaveV2Stablecoin.Accrue](#steward-v4-AaveV2Stablecoin-Accrue)
    - [AaveV2Stablecoin.ClaimAndUnstake](#steward-v4-AaveV2Stablecoin-ClaimAndUnstake)
    - [AaveV2Stablecoin.EnterPosition](#steward-v4-AaveV2Stablecoin-EnterPosition)
    - [AaveV2Stablecoin.EnterPositionWithAssets](#steward-v4-AaveV2Stablecoin-EnterPositionWithAssets)
    - [AaveV2Stablecoin.ExitPosition](#steward-v4-AaveV2Stablecoin-ExitPosition)
    - [AaveV2Stablecoin.ExitPositionWithAssets](#steward-v4-AaveV2Stablecoin-ExitPositionWithAssets)
    - [AaveV2Stablecoin.Rebalance](#steward-v4-AaveV2Stablecoin-Rebalance)
    - [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v4-AaveV2Stablecoin-Rebalance-SwapParams)
    - [AaveV2Stablecoin.Reinvest](#steward-v4-AaveV2Stablecoin-Reinvest)
    - [AaveV2Stablecoin.SendFees](#steward-v4-AaveV2Stablecoin-SendFees)
    - [AaveV2Stablecoin.SetAccrualPeriod](#steward-v4-AaveV2Stablecoin-SetAccrualPeriod)
    - [AaveV2Stablecoin.SetDepositLimit](#steward-v4-AaveV2Stablecoin-SetDepositLimit)
    - [AaveV2Stablecoin.SetLiquidityLimit](#steward-v4-AaveV2Stablecoin-SetLiquidityLimit)
    - [AaveV2StablecoinGovernance](#steward-v4-AaveV2StablecoinGovernance)
    - [AaveV2StablecoinGovernance.InitiateShutdown](#steward-v4-AaveV2StablecoinGovernance-InitiateShutdown)
    - [AaveV2StablecoinGovernance.LiftShutdown](#steward-v4-AaveV2StablecoinGovernance-LiftShutdown)
    - [AaveV2StablecoinGovernance.SetFeesDistributor](#steward-v4-AaveV2StablecoinGovernance-SetFeesDistributor)
    - [AaveV2StablecoinGovernance.SetTrust](#steward-v4-AaveV2StablecoinGovernance-SetTrust)
    - [AaveV2StablecoinGovernance.Sweep](#steward-v4-AaveV2StablecoinGovernance-Sweep)
  
- [aave_v3_a_token.proto](#aave_v3_a_token-proto)
    - [AaveV3ATokenAdaptorV1](#steward-v4-AaveV3ATokenAdaptorV1)
    - [AaveV3ATokenAdaptorV1.AdjustIsolationModeAssetAsCollateral](#steward-v4-AaveV3ATokenAdaptorV1-AdjustIsolationModeAssetAsCollateral)
    - [AaveV3ATokenAdaptorV1.ChangeEMode](#steward-v4-AaveV3ATokenAdaptorV1-ChangeEMode)
    - [AaveV3ATokenAdaptorV1.DepositToAave](#steward-v4-AaveV3ATokenAdaptorV1-DepositToAave)
    - [AaveV3ATokenAdaptorV1.WithdrawFromAave](#steward-v4-AaveV3ATokenAdaptorV1-WithdrawFromAave)
    - [AaveV3ATokenAdaptorV1Calls](#steward-v4-AaveV3ATokenAdaptorV1Calls)
  
- [aave_v3_debt_token.proto](#aave_v3_debt_token-proto)
    - [AaveV3DebtTokenAdaptorV1](#steward-v4-AaveV3DebtTokenAdaptorV1)
    - [AaveV3DebtTokenAdaptorV1.BorrowFromAave](#steward-v4-AaveV3DebtTokenAdaptorV1-BorrowFromAave)
    - [AaveV3DebtTokenAdaptorV1.RepayAaveDebt](#steward-v4-AaveV3DebtTokenAdaptorV1-RepayAaveDebt)
    - [AaveV3DebtTokenAdaptorV1.RepayWithATokens](#steward-v4-AaveV3DebtTokenAdaptorV1-RepayWithATokens)
    - [AaveV3DebtTokenAdaptorV1Calls](#steward-v4-AaveV3DebtTokenAdaptorV1Calls)
  
- [aave_v3_debt_token_flash_loan.proto](#aave_v3_debt_token_flash_loan-proto)
    - [AaveV3DebtTokenAdaptorV1FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan)
    - [AaveV3DebtTokenAdaptorV1FlashLoan.AdaptorCallForAaveV3FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-AdaptorCallForAaveV3FlashLoan)
    - [AaveV3DebtTokenAdaptorV1FlashLoan.FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-FlashLoan)
    - [AaveV3DebtTokenAdaptorV1FlashLoanCalls](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoanCalls)
  
- [aura_erc4626.proto](#aura_erc4626-proto)
    - [AuraERC4626AdaptorV1](#steward-v4-AuraERC4626AdaptorV1)
    - [AuraERC4626AdaptorV1.GetRewards](#steward-v4-AuraERC4626AdaptorV1-GetRewards)
    - [AuraERC4626AdaptorV1Calls](#steward-v4-AuraERC4626AdaptorV1Calls)
  
- [balancer_pool.proto](#balancer_pool-proto)
    - [BalancerPoolAdaptorV1](#steward-v4-BalancerPoolAdaptorV1)
    - [BalancerPoolAdaptorV1.ClaimRewards](#steward-v4-BalancerPoolAdaptorV1-ClaimRewards)
    - [BalancerPoolAdaptorV1.ExitPool](#steward-v4-BalancerPoolAdaptorV1-ExitPool)
    - [BalancerPoolAdaptorV1.ExitPoolRequest](#steward-v4-BalancerPoolAdaptorV1-ExitPoolRequest)
    - [BalancerPoolAdaptorV1.JoinPool](#steward-v4-BalancerPoolAdaptorV1-JoinPool)
    - [BalancerPoolAdaptorV1.SingleSwap](#steward-v4-BalancerPoolAdaptorV1-SingleSwap)
    - [BalancerPoolAdaptorV1.StakeBPT](#steward-v4-BalancerPoolAdaptorV1-StakeBPT)
    - [BalancerPoolAdaptorV1.SwapData](#steward-v4-BalancerPoolAdaptorV1-SwapData)
    - [BalancerPoolAdaptorV1.UnstakeBPT](#steward-v4-BalancerPoolAdaptorV1-UnstakeBPT)
    - [BalancerPoolAdaptorV1Calls](#steward-v4-BalancerPoolAdaptorV1Calls)
  
    - [BalancerPoolAdaptorV1.SwapKind](#steward-v4-BalancerPoolAdaptorV1-SwapKind)
  
- [balancer_pool_flash_loan.proto](#balancer_pool_flash_loan-proto)
    - [BalancerPoolAdaptorV1FlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan)
    - [BalancerPoolAdaptorV1FlashLoan.AdaptorCallForBalancerPoolFlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan-AdaptorCallForBalancerPoolFlashLoan)
    - [BalancerPoolAdaptorV1FlashLoan.MakeFlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan-MakeFlashLoan)
    - [BalancerPoolAdaptorV1FlashLoanCalls](#steward-v4-BalancerPoolAdaptorV1FlashLoanCalls)
  
- [base.proto](#base-proto)
    - [OracleSwap](#steward-v4-OracleSwap)
    - [RevokeApproval](#steward-v4-RevokeApproval)
    - [Swap](#steward-v4-Swap)
  
- [c_token.proto](#c_token-proto)
    - [CompoundCTokenAdaptorV2](#steward-v4-CompoundCTokenAdaptorV2)
    - [CompoundCTokenAdaptorV2.ClaimComp](#steward-v4-CompoundCTokenAdaptorV2-ClaimComp)
    - [CompoundCTokenAdaptorV2.DepositToCompound](#steward-v4-CompoundCTokenAdaptorV2-DepositToCompound)
    - [CompoundCTokenAdaptorV2.WithdrawFromCompound](#steward-v4-CompoundCTokenAdaptorV2-WithdrawFromCompound)
    - [CompoundCTokenAdaptorV2Calls](#steward-v4-CompoundCTokenAdaptorV2Calls)
  
- [cellar_adaptor.proto](#cellar_adaptor-proto)
    - [CellarAdaptorV1](#steward-v4-CellarAdaptorV1)
    - [CellarAdaptorV1.DepositToCellar](#steward-v4-CellarAdaptorV1-DepositToCellar)
    - [CellarAdaptorV1.WithdrawFromCellar](#steward-v4-CellarAdaptorV1-WithdrawFromCellar)
    - [CellarAdaptorV1Calls](#steward-v4-CellarAdaptorV1Calls)
  
- [cellar_v1.proto](#cellar_v1-proto)
    - [CellarV1](#steward-v4-CellarV1)
    - [CellarV1.AddPosition](#steward-v4-CellarV1-AddPosition)
    - [CellarV1.PushPosition](#steward-v4-CellarV1-PushPosition)
    - [CellarV1.Rebalance](#steward-v4-CellarV1-Rebalance)
    - [CellarV1.RemovePosition](#steward-v4-CellarV1-RemovePosition)
    - [CellarV1.SetDepositLimit](#steward-v4-CellarV1-SetDepositLimit)
    - [CellarV1.SetHoldingPosition](#steward-v4-CellarV1-SetHoldingPosition)
    - [CellarV1.SetLiquidityLimit](#steward-v4-CellarV1-SetLiquidityLimit)
    - [CellarV1.SetRebalanceDeviation](#steward-v4-CellarV1-SetRebalanceDeviation)
    - [CellarV1.SetShareLockPeriod](#steward-v4-CellarV1-SetShareLockPeriod)
    - [CellarV1.SetStrategistPayoutAddress](#steward-v4-CellarV1-SetStrategistPayoutAddress)
    - [CellarV1.SetWithdrawType](#steward-v4-CellarV1-SetWithdrawType)
    - [CellarV1.SwapPositions](#steward-v4-CellarV1-SwapPositions)
    - [CellarV1Governance](#steward-v4-CellarV1Governance)
    - [CellarV1Governance.InitiateShutdown](#steward-v4-CellarV1Governance-InitiateShutdown)
    - [CellarV1Governance.LiftShutdown](#steward-v4-CellarV1Governance-LiftShutdown)
    - [CellarV1Governance.ResetHighWatermark](#steward-v4-CellarV1Governance-ResetHighWatermark)
    - [CellarV1Governance.SetFeesDistributor](#steward-v4-CellarV1Governance-SetFeesDistributor)
    - [CellarV1Governance.SetPerformanceFee](#steward-v4-CellarV1Governance-SetPerformanceFee)
    - [CellarV1Governance.SetPlatformFee](#steward-v4-CellarV1Governance-SetPlatformFee)
    - [CellarV1Governance.SetStrategistPerformanceCut](#steward-v4-CellarV1Governance-SetStrategistPerformanceCut)
    - [CellarV1Governance.SetStrategistPlatformCut](#steward-v4-CellarV1Governance-SetStrategistPlatformCut)
    - [CellarV1Governance.TrustPosition](#steward-v4-CellarV1Governance-TrustPosition)
  
    - [CellarV1.WithdrawType](#steward-v4-CellarV1-WithdrawType)
  
- [cellar_v2.proto](#cellar_v2-proto)
    - [AdaptorCall](#steward-v4-AdaptorCall)
    - [CellarV2](#steward-v4-CellarV2)
    - [CellarV2.AddPosition](#steward-v4-CellarV2-AddPosition)
    - [CellarV2.CallOnAdaptor](#steward-v4-CellarV2-CallOnAdaptor)
    - [CellarV2.InitiateShutdown](#steward-v4-CellarV2-InitiateShutdown)
    - [CellarV2.LiftShutdown](#steward-v4-CellarV2-LiftShutdown)
    - [CellarV2.RemovePosition](#steward-v4-CellarV2-RemovePosition)
    - [CellarV2.SetHoldingPosition](#steward-v4-CellarV2-SetHoldingPosition)
    - [CellarV2.SetPlatformFee](#steward-v4-CellarV2-SetPlatformFee)
    - [CellarV2.SetRebalanceDeviation](#steward-v4-CellarV2-SetRebalanceDeviation)
    - [CellarV2.SetShareLockPeriod](#steward-v4-CellarV2-SetShareLockPeriod)
    - [CellarV2.SetStrategistPayoutAddress](#steward-v4-CellarV2-SetStrategistPayoutAddress)
    - [CellarV2.SetStrategistPlatformCut](#steward-v4-CellarV2-SetStrategistPlatformCut)
    - [CellarV2.SetupAdaptor](#steward-v4-CellarV2-SetupAdaptor)
    - [CellarV2.SwapPositions](#steward-v4-CellarV2-SwapPositions)
    - [CellarV2Governance](#steward-v4-CellarV2Governance)
    - [CellarV2Governance.InitiateShutdown](#steward-v4-CellarV2Governance-InitiateShutdown)
    - [CellarV2Governance.LiftShutdown](#steward-v4-CellarV2Governance-LiftShutdown)
    - [CellarV2Governance.SetPlatformFee](#steward-v4-CellarV2Governance-SetPlatformFee)
    - [CellarV2Governance.SetRebalanceDeviation](#steward-v4-CellarV2Governance-SetRebalanceDeviation)
    - [CellarV2Governance.SetStrategistPlatformCut](#steward-v4-CellarV2Governance-SetStrategistPlatformCut)
    - [CellarV2Governance.SetupAdaptor](#steward-v4-CellarV2Governance-SetupAdaptor)
    - [CellarV2_2](#steward-v4-CellarV2_2)
    - [CellarV2_2.AddAdaptorToCatalogue](#steward-v4-CellarV2_2-AddAdaptorToCatalogue)
    - [CellarV2_2.AddPosition](#steward-v4-CellarV2_2-AddPosition)
    - [CellarV2_2.AddPositionToCatalogue](#steward-v4-CellarV2_2-AddPositionToCatalogue)
    - [CellarV2_2.CachePriceRouter](#steward-v4-CellarV2_2-CachePriceRouter)
    - [CellarV2_2.CallOnAdaptor](#steward-v4-CellarV2_2-CallOnAdaptor)
    - [CellarV2_2.FunctionCall](#steward-v4-CellarV2_2-FunctionCall)
    - [CellarV2_2.InitiateShutdown](#steward-v4-CellarV2_2-InitiateShutdown)
    - [CellarV2_2.LiftShutdown](#steward-v4-CellarV2_2-LiftShutdown)
    - [CellarV2_2.Multicall](#steward-v4-CellarV2_2-Multicall)
    - [CellarV2_2.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_2-RemoveAdaptorFromCatalogue)
    - [CellarV2_2.RemovePosition](#steward-v4-CellarV2_2-RemovePosition)
    - [CellarV2_2.RemovePositionFromCatalogue](#steward-v4-CellarV2_2-RemovePositionFromCatalogue)
    - [CellarV2_2.SetHoldingPosition](#steward-v4-CellarV2_2-SetHoldingPosition)
    - [CellarV2_2.SetRebalanceDeviation](#steward-v4-CellarV2_2-SetRebalanceDeviation)
    - [CellarV2_2.SetShareLockPeriod](#steward-v4-CellarV2_2-SetShareLockPeriod)
    - [CellarV2_2.SetStrategistPayoutAddress](#steward-v4-CellarV2_2-SetStrategistPayoutAddress)
    - [CellarV2_2.SetStrategistPlatformCut](#steward-v4-CellarV2_2-SetStrategistPlatformCut)
    - [CellarV2_2.SwapPositions](#steward-v4-CellarV2_2-SwapPositions)
    - [CellarV2_2Governance](#steward-v4-CellarV2_2Governance)
    - [CellarV2_2Governance.AddAdaptorToCatalogue](#steward-v4-CellarV2_2Governance-AddAdaptorToCatalogue)
    - [CellarV2_2Governance.AddPosition](#steward-v4-CellarV2_2Governance-AddPosition)
    - [CellarV2_2Governance.AddPositionToCatalogue](#steward-v4-CellarV2_2Governance-AddPositionToCatalogue)
    - [CellarV2_2Governance.CachePriceRouter](#steward-v4-CellarV2_2Governance-CachePriceRouter)
    - [CellarV2_2Governance.CallOnAdaptor](#steward-v4-CellarV2_2Governance-CallOnAdaptor)
    - [CellarV2_2Governance.ForcePositionOut](#steward-v4-CellarV2_2Governance-ForcePositionOut)
    - [CellarV2_2Governance.FunctionCall](#steward-v4-CellarV2_2Governance-FunctionCall)
    - [CellarV2_2Governance.InitiateShutdown](#steward-v4-CellarV2_2Governance-InitiateShutdown)
    - [CellarV2_2Governance.LiftShutdown](#steward-v4-CellarV2_2Governance-LiftShutdown)
    - [CellarV2_2Governance.Multicall](#steward-v4-CellarV2_2Governance-Multicall)
    - [CellarV2_2Governance.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_2Governance-RemoveAdaptorFromCatalogue)
    - [CellarV2_2Governance.RemovePosition](#steward-v4-CellarV2_2Governance-RemovePosition)
    - [CellarV2_2Governance.RemovePositionFromCatalogue](#steward-v4-CellarV2_2Governance-RemovePositionFromCatalogue)
    - [CellarV2_2Governance.SetHoldingPosition](#steward-v4-CellarV2_2Governance-SetHoldingPosition)
    - [CellarV2_2Governance.SetRebalanceDeviation](#steward-v4-CellarV2_2Governance-SetRebalanceDeviation)
    - [CellarV2_2Governance.SetShareLockPeriod](#steward-v4-CellarV2_2Governance-SetShareLockPeriod)
    - [CellarV2_2Governance.SetStrategistPlatformCut](#steward-v4-CellarV2_2Governance-SetStrategistPlatformCut)
    - [CellarV2_2Governance.SwapPositions](#steward-v4-CellarV2_2Governance-SwapPositions)
    - [CellarV2_2Governance.ToggleIgnorePause](#steward-v4-CellarV2_2Governance-ToggleIgnorePause)
    - [CellarV2_5](#steward-v4-CellarV2_5)
    - [CellarV2_5.AddAdaptorToCatalogue](#steward-v4-CellarV2_5-AddAdaptorToCatalogue)
    - [CellarV2_5.AddPosition](#steward-v4-CellarV2_5-AddPosition)
    - [CellarV2_5.AddPositionToCatalogue](#steward-v4-CellarV2_5-AddPositionToCatalogue)
    - [CellarV2_5.CachePriceRouter](#steward-v4-CellarV2_5-CachePriceRouter)
    - [CellarV2_5.CallOnAdaptor](#steward-v4-CellarV2_5-CallOnAdaptor)
    - [CellarV2_5.DecreaseShareSupplyCap](#steward-v4-CellarV2_5-DecreaseShareSupplyCap)
    - [CellarV2_5.DropAlternativeAssetData](#steward-v4-CellarV2_5-DropAlternativeAssetData)
    - [CellarV2_5.FunctionCall](#steward-v4-CellarV2_5-FunctionCall)
    - [CellarV2_5.IncreaseShareSupplyCap](#steward-v4-CellarV2_5-IncreaseShareSupplyCap)
    - [CellarV2_5.InitiateShutdown](#steward-v4-CellarV2_5-InitiateShutdown)
    - [CellarV2_5.LiftShutdown](#steward-v4-CellarV2_5-LiftShutdown)
    - [CellarV2_5.Multicall](#steward-v4-CellarV2_5-Multicall)
    - [CellarV2_5.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_5-RemoveAdaptorFromCatalogue)
    - [CellarV2_5.RemovePosition](#steward-v4-CellarV2_5-RemovePosition)
    - [CellarV2_5.RemovePositionFromCatalogue](#steward-v4-CellarV2_5-RemovePositionFromCatalogue)
    - [CellarV2_5.SetAlternativeAssetData](#steward-v4-CellarV2_5-SetAlternativeAssetData)
    - [CellarV2_5.SetHoldingPosition](#steward-v4-CellarV2_5-SetHoldingPosition)
    - [CellarV2_5.SetRebalanceDeviation](#steward-v4-CellarV2_5-SetRebalanceDeviation)
    - [CellarV2_5.SetShareLockPeriod](#steward-v4-CellarV2_5-SetShareLockPeriod)
    - [CellarV2_5.SetSharePriceOracle](#steward-v4-CellarV2_5-SetSharePriceOracle)
    - [CellarV2_5.SetStrategistPayoutAddress](#steward-v4-CellarV2_5-SetStrategistPayoutAddress)
    - [CellarV2_5.SetStrategistPlatformCut](#steward-v4-CellarV2_5-SetStrategistPlatformCut)
    - [CellarV2_5.SwapPositions](#steward-v4-CellarV2_5-SwapPositions)
    - [CellarV2_5Governance](#steward-v4-CellarV2_5Governance)
    - [CellarV2_5Governance.AddAdaptorToCatalogue](#steward-v4-CellarV2_5Governance-AddAdaptorToCatalogue)
    - [CellarV2_5Governance.AddPosition](#steward-v4-CellarV2_5Governance-AddPosition)
    - [CellarV2_5Governance.AddPositionToCatalogue](#steward-v4-CellarV2_5Governance-AddPositionToCatalogue)
    - [CellarV2_5Governance.CachePriceRouter](#steward-v4-CellarV2_5Governance-CachePriceRouter)
    - [CellarV2_5Governance.CallOnAdaptor](#steward-v4-CellarV2_5Governance-CallOnAdaptor)
    - [CellarV2_5Governance.DecreaseShareSupplyCap](#steward-v4-CellarV2_5Governance-DecreaseShareSupplyCap)
    - [CellarV2_5Governance.ForcePositionOut](#steward-v4-CellarV2_5Governance-ForcePositionOut)
    - [CellarV2_5Governance.FunctionCall](#steward-v4-CellarV2_5Governance-FunctionCall)
    - [CellarV2_5Governance.IncreaseShareSupplyCap](#steward-v4-CellarV2_5Governance-IncreaseShareSupplyCap)
    - [CellarV2_5Governance.InitiateShutdown](#steward-v4-CellarV2_5Governance-InitiateShutdown)
    - [CellarV2_5Governance.LiftShutdown](#steward-v4-CellarV2_5Governance-LiftShutdown)
    - [CellarV2_5Governance.Multicall](#steward-v4-CellarV2_5Governance-Multicall)
    - [CellarV2_5Governance.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_5Governance-RemoveAdaptorFromCatalogue)
    - [CellarV2_5Governance.RemovePosition](#steward-v4-CellarV2_5Governance-RemovePosition)
    - [CellarV2_5Governance.RemovePositionFromCatalogue](#steward-v4-CellarV2_5Governance-RemovePositionFromCatalogue)
    - [CellarV2_5Governance.SetAutomationActions](#steward-v4-CellarV2_5Governance-SetAutomationActions)
    - [CellarV2_5Governance.SetHoldingPosition](#steward-v4-CellarV2_5Governance-SetHoldingPosition)
    - [CellarV2_5Governance.SetRebalanceDeviation](#steward-v4-CellarV2_5Governance-SetRebalanceDeviation)
    - [CellarV2_5Governance.SetSharePriceOracle](#steward-v4-CellarV2_5Governance-SetSharePriceOracle)
    - [CellarV2_5Governance.SetStrategistPlatformCut](#steward-v4-CellarV2_5Governance-SetStrategistPlatformCut)
    - [CellarV2_5Governance.ToggleIgnorePause](#steward-v4-CellarV2_5Governance-ToggleIgnorePause)
  
- [collateral_f_token.proto](#collateral_f_token-proto)
    - [CollateralFTokenAdaptorV1](#steward-v4-CollateralFTokenAdaptorV1)
    - [CollateralFTokenAdaptorV1.AddCollateral](#steward-v4-CollateralFTokenAdaptorV1-AddCollateral)
    - [CollateralFTokenAdaptorV1.RemoveCollateral](#steward-v4-CollateralFTokenAdaptorV1-RemoveCollateral)
    - [CollateralFTokenAdaptorV1Calls](#steward-v4-CollateralFTokenAdaptorV1Calls)
  
- [common.proto](#common-proto)
    - [MarketParams](#steward-v4-MarketParams)
    - [OracleSwapParams](#steward-v4-OracleSwapParams)
    - [SwapParams](#steward-v4-SwapParams)
    - [UniV2OracleSwapParams](#steward-v4-UniV2OracleSwapParams)
    - [UniV2SwapParams](#steward-v4-UniV2SwapParams)
    - [UniV3OracleSwapParams](#steward-v4-UniV3OracleSwapParams)
    - [UniV3SwapParams](#steward-v4-UniV3SwapParams)
  
    - [Exchange](#steward-v4-Exchange)
  
- [convex_curve.proto](#convex_curve-proto)
    - [ConvexCurveAdaptorV1](#steward-v4-ConvexCurveAdaptorV1)
    - [ConvexCurveAdaptorV1.DepositLPTInConvexAndStake](#steward-v4-ConvexCurveAdaptorV1-DepositLPTInConvexAndStake)
    - [ConvexCurveAdaptorV1.GetRewards](#steward-v4-ConvexCurveAdaptorV1-GetRewards)
    - [ConvexCurveAdaptorV1.WithdrawFromBaseRewardPoolAsLPT](#steward-v4-ConvexCurveAdaptorV1-WithdrawFromBaseRewardPoolAsLPT)
    - [ConvexCurveAdaptorV1Calls](#steward-v4-ConvexCurveAdaptorV1Calls)
  
- [curve.proto](#curve-proto)
    - [CurveAdaptorV1](#steward-v4-CurveAdaptorV1)
    - [CurveAdaptorV1.AddLiquidity](#steward-v4-CurveAdaptorV1-AddLiquidity)
    - [CurveAdaptorV1.AddLiquidityETH](#steward-v4-CurveAdaptorV1-AddLiquidityETH)
    - [CurveAdaptorV1.ClaimRewards](#steward-v4-CurveAdaptorV1-ClaimRewards)
    - [CurveAdaptorV1.RemoveLiquidity](#steward-v4-CurveAdaptorV1-RemoveLiquidity)
    - [CurveAdaptorV1.RemoveLiquidityETH](#steward-v4-CurveAdaptorV1-RemoveLiquidityETH)
    - [CurveAdaptorV1.StakeInGauge](#steward-v4-CurveAdaptorV1-StakeInGauge)
    - [CurveAdaptorV1.UnstakeFromGauge](#steward-v4-CurveAdaptorV1-UnstakeFromGauge)
    - [CurveAdaptorV1Calls](#steward-v4-CurveAdaptorV1Calls)
  
- [debt_f_token.proto](#debt_f_token-proto)
    - [DebtFTokenAdaptorV1](#steward-v4-DebtFTokenAdaptorV1)
    - [DebtFTokenAdaptorV1.BorrowFromFraxlend](#steward-v4-DebtFTokenAdaptorV1-BorrowFromFraxlend)
    - [DebtFTokenAdaptorV1.CallAddInterest](#steward-v4-DebtFTokenAdaptorV1-CallAddInterest)
    - [DebtFTokenAdaptorV1.RepayFraxlendDebt](#steward-v4-DebtFTokenAdaptorV1-RepayFraxlendDebt)
    - [DebtFTokenAdaptorV1Calls](#steward-v4-DebtFTokenAdaptorV1Calls)
  
- [debt_token.proto](#debt_token-proto)
    - [AaveDebtTokenAdaptorV1](#steward-v4-AaveDebtTokenAdaptorV1)
    - [AaveDebtTokenAdaptorV1.BorrowFromAave](#steward-v4-AaveDebtTokenAdaptorV1-BorrowFromAave)
    - [AaveDebtTokenAdaptorV1.RepayAaveDebt](#steward-v4-AaveDebtTokenAdaptorV1-RepayAaveDebt)
    - [AaveDebtTokenAdaptorV1.SwapAndRepay](#steward-v4-AaveDebtTokenAdaptorV1-SwapAndRepay)
    - [AaveDebtTokenAdaptorV1Calls](#steward-v4-AaveDebtTokenAdaptorV1Calls)
    - [AaveDebtTokenAdaptorV2](#steward-v4-AaveDebtTokenAdaptorV2)
    - [AaveDebtTokenAdaptorV2.BorrowFromAave](#steward-v4-AaveDebtTokenAdaptorV2-BorrowFromAave)
    - [AaveDebtTokenAdaptorV2.RepayAaveDebt](#steward-v4-AaveDebtTokenAdaptorV2-RepayAaveDebt)
    - [AaveDebtTokenAdaptorV2Calls](#steward-v4-AaveDebtTokenAdaptorV2Calls)
  
- [erc4626.proto](#erc4626-proto)
    - [ERC4626AdaptorV1](#steward-v4-ERC4626AdaptorV1)
    - [ERC4626AdaptorV1.DepositToVault](#steward-v4-ERC4626AdaptorV1-DepositToVault)
    - [ERC4626AdaptorV1.WithdrawFromVault](#steward-v4-ERC4626AdaptorV1-WithdrawFromVault)
    - [ERC4626AdaptorV1Calls](#steward-v4-ERC4626AdaptorV1Calls)
  
- [f_token.proto](#f_token-proto)
    - [FTokenAdaptorV1](#steward-v4-FTokenAdaptorV1)
    - [FTokenAdaptorV1.CallAddInterest](#steward-v4-FTokenAdaptorV1-CallAddInterest)
    - [FTokenAdaptorV1.LendFrax](#steward-v4-FTokenAdaptorV1-LendFrax)
    - [FTokenAdaptorV1.RedeemFraxShare](#steward-v4-FTokenAdaptorV1-RedeemFraxShare)
    - [FTokenAdaptorV1.WithdrawFrax](#steward-v4-FTokenAdaptorV1-WithdrawFrax)
    - [FTokenAdaptorV1Calls](#steward-v4-FTokenAdaptorV1Calls)
  
- [fees_and_reserves.proto](#fees_and_reserves-proto)
    - [FeesAndReservesAdaptorV1](#steward-v4-FeesAndReservesAdaptorV1)
    - [FeesAndReservesAdaptorV1.AddAssetsToReserves](#steward-v4-FeesAndReservesAdaptorV1-AddAssetsToReserves)
    - [FeesAndReservesAdaptorV1.ChangeUpkeepFrequency](#steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency)
    - [FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas](#steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas)
    - [FeesAndReservesAdaptorV1.PrepareFees](#steward-v4-FeesAndReservesAdaptorV1-PrepareFees)
    - [FeesAndReservesAdaptorV1.SetupMetaData](#steward-v4-FeesAndReservesAdaptorV1-SetupMetaData)
    - [FeesAndReservesAdaptorV1.UpdateManagementFees](#steward-v4-FeesAndReservesAdaptorV1-UpdateManagementFees)
    - [FeesAndReservesAdaptorV1.UpdatePerformanceFees](#steward-v4-FeesAndReservesAdaptorV1-UpdatePerformanceFees)
    - [FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves](#steward-v4-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves)
    - [FeesAndReservesAdaptorV1Calls](#steward-v4-FeesAndReservesAdaptorV1Calls)
  
- [governance.proto](#governance-proto)
    - [GovernanceCall](#steward-v4-GovernanceCall)
  
- [legacy_cellar_adaptor.proto](#legacy_cellar_adaptor-proto)
    - [LegacyCellarAdaptorV1](#steward-v4-LegacyCellarAdaptorV1)
    - [LegacyCellarAdaptorV1.DepositToCellar](#steward-v4-LegacyCellarAdaptorV1-DepositToCellar)
    - [LegacyCellarAdaptorV1.WithdrawFromCellar](#steward-v4-LegacyCellarAdaptorV1-WithdrawFromCellar)
    - [LegacyCellarAdaptorV1Calls](#steward-v4-LegacyCellarAdaptorV1Calls)
  
- [morpho_aave_v2_a_token.proto](#morpho_aave_v2_a_token-proto)
    - [MorphoAaveV2ATokenAdaptorV1](#steward-v4-MorphoAaveV2ATokenAdaptorV1)
    - [MorphoAaveV2ATokenAdaptorV1.DepositToAaveV2Morpho](#steward-v4-MorphoAaveV2ATokenAdaptorV1-DepositToAaveV2Morpho)
    - [MorphoAaveV2ATokenAdaptorV1.WithdrawFromAaveV2Morpho](#steward-v4-MorphoAaveV2ATokenAdaptorV1-WithdrawFromAaveV2Morpho)
    - [MorphoAaveV2ATokenAdaptorV1Calls](#steward-v4-MorphoAaveV2ATokenAdaptorV1Calls)
  
- [morpho_aave_v2_debt_token.proto](#morpho_aave_v2_debt_token-proto)
    - [MorphoAaveV2DebtTokenAdaptorV1](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1)
    - [MorphoAaveV2DebtTokenAdaptorV1.BorrowFromAaveV2Morpho](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1-BorrowFromAaveV2Morpho)
    - [MorphoAaveV2DebtTokenAdaptorV1.RepayAaveV2MorphoDebt](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1-RepayAaveV2MorphoDebt)
    - [MorphoAaveV2DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1Calls)
  
- [morpho_aave_v3_a_token_collateral.proto](#morpho_aave_v3_a_token_collateral-proto)
    - [MorphoAaveV3ATokenCollateralAdaptorV1](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1)
    - [MorphoAaveV3ATokenCollateralAdaptorV1.DepositToAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-DepositToAaveV3Morpho)
    - [MorphoAaveV3ATokenCollateralAdaptorV1.WithdrawFromAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-WithdrawFromAaveV3Morpho)
    - [MorphoAaveV3ATokenCollateralAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1Calls)
  
- [morpho_aave_v3_a_token_p2p.proto](#morpho_aave_v3_a_token_p2p-proto)
    - [MorphoAaveV3ATokenP2PAdaptorV1](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1)
    - [MorphoAaveV3ATokenP2PAdaptorV1.DepositToAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-DepositToAaveV3Morpho)
    - [MorphoAaveV3ATokenP2PAdaptorV1.WithdrawFromAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-WithdrawFromAaveV3Morpho)
    - [MorphoAaveV3ATokenP2PAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1Calls)
  
- [morpho_aave_v3_debt_token.proto](#morpho_aave_v3_debt_token-proto)
    - [MorphoAaveV3DebtTokenAdaptorV1](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1)
    - [MorphoAaveV3DebtTokenAdaptorV1.BorrowFromAaveV3Morpho](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1-BorrowFromAaveV3Morpho)
    - [MorphoAaveV3DebtTokenAdaptorV1.RepayAaveV3MorphoDebt](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1-RepayAaveV3MorphoDebt)
    - [MorphoAaveV3DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1Calls)
  
- [morpho_blue_collateral.proto](#morpho_blue_collateral-proto)
    - [MorphoBlueCollateralAdaptorV1](#steward-v4-MorphoBlueCollateralAdaptorV1)
    - [MorphoBlueCollateralAdaptorV1.AddCollateral](#steward-v4-MorphoBlueCollateralAdaptorV1-AddCollateral)
    - [MorphoBlueCollateralAdaptorV1.RemoveCollateral](#steward-v4-MorphoBlueCollateralAdaptorV1-RemoveCollateral)
    - [MorphoBlueCollateralAdaptorV1Calls](#steward-v4-MorphoBlueCollateralAdaptorV1Calls)
  
- [morpho_blue_debt.proto](#morpho_blue_debt-proto)
    - [MorphoBlueDebtAdaptorV1](#steward-v4-MorphoBlueDebtAdaptorV1)
    - [MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue](#steward-v4-MorphoBlueDebtAdaptorV1-BorrowFromMorphoBlue)
    - [MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt](#steward-v4-MorphoBlueDebtAdaptorV1-RepayMorphoBlueDebt)
    - [MorphoBlueDebtAdaptorV1Calls](#steward-v4-MorphoBlueDebtAdaptorV1Calls)
  
- [morpho_blue_supply.proto](#morpho_blue_supply-proto)
    - [MorphoBlueSupplyAdaptorV1](#steward-v4-MorphoBlueSupplyAdaptorV1)
    - [MorphoBlueSupplyAdaptorV1.LendToMorphoBlue](#steward-v4-MorphoBlueSupplyAdaptorV1-LendToMorphoBlue)
    - [MorphoBlueSupplyAdaptorV1.WithdrawFromMorphoBlue](#steward-v4-MorphoBlueSupplyAdaptorV1-WithdrawFromMorphoBlue)
    - [MorphoBlueSupplyAdaptorV1Calls](#steward-v4-MorphoBlueSupplyAdaptorV1Calls)
  
- [morpho_reward_handler.proto](#morpho_reward_handler-proto)
    - [Claim](#steward-v4-Claim)
  
- [oneinch.proto](#oneinch-proto)
    - [OneInchAdaptorV1](#steward-v4-OneInchAdaptorV1)
    - [OneInchAdaptorV1.SwapWithOneInch](#steward-v4-OneInchAdaptorV1-SwapWithOneInch)
    - [OneInchAdaptorV1Calls](#steward-v4-OneInchAdaptorV1Calls)
  
- [pendle.proto](#pendle-proto)
    - [PendleAdaptorV1](#steward-v4-PendleAdaptorV1)
    - [PendleAdaptorV1.AddLiquidityDualSyAndPt](#steward-v4-PendleAdaptorV1-AddLiquidityDualSyAndPt)
    - [PendleAdaptorV1.ApproxParams](#steward-v4-PendleAdaptorV1-ApproxParams)
    - [PendleAdaptorV1.MintPyFromSy](#steward-v4-PendleAdaptorV1-MintPyFromSy)
    - [PendleAdaptorV1.MintSyFromToken](#steward-v4-PendleAdaptorV1-MintSyFromToken)
    - [PendleAdaptorV1.RedeemPyToSy](#steward-v4-PendleAdaptorV1-RedeemPyToSy)
    - [PendleAdaptorV1.RedeemSyToToken](#steward-v4-PendleAdaptorV1-RedeemSyToToken)
    - [PendleAdaptorV1.RemoveLiquidityDualSyAndPt](#steward-v4-PendleAdaptorV1-RemoveLiquidityDualSyAndPt)
    - [PendleAdaptorV1.SwapData](#steward-v4-PendleAdaptorV1-SwapData)
    - [PendleAdaptorV1.SwapExactPtForYt](#steward-v4-PendleAdaptorV1-SwapExactPtForYt)
    - [PendleAdaptorV1.SwapExactYtForPt](#steward-v4-PendleAdaptorV1-SwapExactYtForPt)
    - [PendleAdaptorV1.TokenInput](#steward-v4-PendleAdaptorV1-TokenInput)
    - [PendleAdaptorV1.TokenOutput](#steward-v4-PendleAdaptorV1-TokenOutput)
    - [PendleAdaptorV1Calls](#steward-v4-PendleAdaptorV1Calls)
  
- [staking.proto](#staking-proto)
    - [StakingAdaptorV1](#steward-v4-StakingAdaptorV1)
    - [StakingAdaptorV1.CancelBurn](#steward-v4-StakingAdaptorV1-CancelBurn)
    - [StakingAdaptorV1.CompleteBurn](#steward-v4-StakingAdaptorV1-CompleteBurn)
    - [StakingAdaptorV1.Mint](#steward-v4-StakingAdaptorV1-Mint)
    - [StakingAdaptorV1.MintERC20](#steward-v4-StakingAdaptorV1-MintERC20)
    - [StakingAdaptorV1.RemoveClaimedRequest](#steward-v4-StakingAdaptorV1-RemoveClaimedRequest)
    - [StakingAdaptorV1.RequestBurn](#steward-v4-StakingAdaptorV1-RequestBurn)
    - [StakingAdaptorV1.Unwrap](#steward-v4-StakingAdaptorV1-Unwrap)
    - [StakingAdaptorV1.Wrap](#steward-v4-StakingAdaptorV1-Wrap)
    - [StakingAdaptorV1Calls](#steward-v4-StakingAdaptorV1Calls)
  
- [steward.proto](#steward-proto)
    - [EncodeRequest](#steward-v4-EncodeRequest)
    - [EncodeResponse](#steward-v4-EncodeResponse)
    - [ScheduleRequest](#steward-v4-ScheduleRequest)
    - [ScheduleResponse](#steward-v4-ScheduleResponse)
    - [SimulateRequest](#steward-v4-SimulateRequest)
    - [SimulateResponse](#steward-v4-SimulateResponse)
    - [StatusRequest](#steward-v4-StatusRequest)
    - [StatusResponse](#steward-v4-StatusResponse)
    - [VersionRequest](#steward-v4-VersionRequest)
    - [VersionResponse](#steward-v4-VersionResponse)
  
    - [ContractCallService](#steward-v4-ContractCallService)
    - [EncodingService](#steward-v4-EncodingService)
    - [SimulateContractCallService](#steward-v4-SimulateContractCallService)
    - [StatusService](#steward-v4-StatusService)
  
- [swap_with_uniswap.proto](#swap_with_uniswap-proto)
    - [SwapWithUniswapAdaptorV1](#steward-v4-SwapWithUniswapAdaptorV1)
    - [SwapWithUniswapAdaptorV1.SwapWithUniV2](#steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV2)
    - [SwapWithUniswapAdaptorV1.SwapWithUniV3](#steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV3)
    - [SwapWithUniswapAdaptorV1Calls](#steward-v4-SwapWithUniswapAdaptorV1Calls)
  
- [uniswap_v3.proto](#uniswap_v3-proto)
    - [UniswapV3AdaptorV2](#steward-v4-UniswapV3AdaptorV2)
    - [UniswapV3AdaptorV2.AddToPosition](#steward-v4-UniswapV3AdaptorV2-AddToPosition)
    - [UniswapV3AdaptorV2.ClosePosition](#steward-v4-UniswapV3AdaptorV2-ClosePosition)
    - [UniswapV3AdaptorV2.CollectFees](#steward-v4-UniswapV3AdaptorV2-CollectFees)
    - [UniswapV3AdaptorV2.OpenPosition](#steward-v4-UniswapV3AdaptorV2-OpenPosition)
    - [UniswapV3AdaptorV2.PurgeAllZeroLiquidityPositions](#steward-v4-UniswapV3AdaptorV2-PurgeAllZeroLiquidityPositions)
    - [UniswapV3AdaptorV2.PurgeSinglePosition](#steward-v4-UniswapV3AdaptorV2-PurgeSinglePosition)
    - [UniswapV3AdaptorV2.RemoveUnownedPositionFromTracker](#steward-v4-UniswapV3AdaptorV2-RemoveUnownedPositionFromTracker)
    - [UniswapV3AdaptorV2.TakeFromPosition](#steward-v4-UniswapV3AdaptorV2-TakeFromPosition)
    - [UniswapV3AdaptorV2Calls](#steward-v4-UniswapV3AdaptorV2Calls)
  
- [vesting_simple.proto](#vesting_simple-proto)
    - [VestingSimpleAdaptorV2](#steward-v4-VestingSimpleAdaptorV2)
    - [VestingSimpleAdaptorV2.DepositToVesting](#steward-v4-VestingSimpleAdaptorV2-DepositToVesting)
    - [VestingSimpleAdaptorV2.WithdrawAllFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawAllFromVesting)
    - [VestingSimpleAdaptorV2.WithdrawAnyFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawAnyFromVesting)
    - [VestingSimpleAdaptorV2.WithdrawFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawFromVesting)
    - [VestingSimpleAdaptorV2Calls](#steward-v4-VestingSimpleAdaptorV2Calls)
  
- [zero_x.proto](#zero_x-proto)
    - [ZeroXAdaptorV1](#steward-v4-ZeroXAdaptorV1)
    - [ZeroXAdaptorV1.SwapWith0x](#steward-v4-ZeroXAdaptorV1-SwapWith0x)
    - [ZeroXAdaptorV1Calls](#steward-v4-ZeroXAdaptorV1Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="a_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## a_token.proto



<a name="steward-v4-AaveATokenAdaptorV1"></a>

### AaveATokenAdaptorV1
Represents call data for the Aave AToken adaptor V1, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v4-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v4-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave | [AaveATokenAdaptorV1.DepositToAave](#steward-v4-AaveATokenAdaptorV1-DepositToAave) |  | Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave | [AaveATokenAdaptorV1.WithdrawFromAave](#steward-v4-AaveATokenAdaptorV1-WithdrawFromAave) |  | Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |






<a name="steward-v4-AaveATokenAdaptorV1-DepositToAave"></a>

### AaveATokenAdaptorV1.DepositToAave
Allows strategists to lend assets on Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to deposit |
| amount | [string](#string) |  | The amount to deposit |






<a name="steward-v4-AaveATokenAdaptorV1-WithdrawFromAave"></a>

### AaveATokenAdaptorV1.WithdrawFromAave
Allows strategists to withdraw assets from Aave.

Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to withdraw |
| amount | [string](#string) |  | The amount to withdraw |






<a name="steward-v4-AaveATokenAdaptorV1Calls"></a>

### AaveATokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveATokenAdaptorV1](#steward-v4-AaveATokenAdaptorV1) | repeated |  |






<a name="steward-v4-AaveATokenAdaptorV2"></a>

### AaveATokenAdaptorV2
Represents call data for the Aave AToken adaptor V2, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave | [AaveATokenAdaptorV2.DepositToAave](#steward-v4-AaveATokenAdaptorV2-DepositToAave) |  | Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave | [AaveATokenAdaptorV2.WithdrawFromAave](#steward-v4-AaveATokenAdaptorV2-WithdrawFromAave) |  | Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |






<a name="steward-v4-AaveATokenAdaptorV2-DepositToAave"></a>

### AaveATokenAdaptorV2.DepositToAave
Allows strategists to lend assets on Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to deposit |
| amount | [string](#string) |  | The amount to deposit |






<a name="steward-v4-AaveATokenAdaptorV2-WithdrawFromAave"></a>

### AaveATokenAdaptorV2.WithdrawFromAave
Allows strategists to withdraw assets from Aave.

Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to withdraw |
| amount | [string](#string) |  | The amount to withdraw |






<a name="steward-v4-AaveATokenAdaptorV2Calls"></a>

### AaveATokenAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveATokenAdaptorV2](#steward-v4-AaveATokenAdaptorV2) | repeated |  |





 

 

 

 



<a name="aave_v2_enable_asset_as_collateral_adaptor-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v2_enable_asset_as_collateral_adaptor.proto



<a name="steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1"></a>

### AaveV2EnableAssetAsCollateralAdaptorV1
Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| set_user_use_reserve_as_collateral | [AaveV2EnableAssetAsCollateralAdaptorV1.SetUserUseReserveAsCollateral](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1-SetUserUseReserveAsCollateral) |  | Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)` |






<a name="steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1-SetUserUseReserveAsCollateral"></a>

### AaveV2EnableAssetAsCollateralAdaptorV1.SetUserUseReserveAsCollateral
Allows a strategist to choose to use an asset as collateral or not.

Represents function `setUserUseReserveAsCollateral(address asset, bool useAsCollateral)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset | [string](#string) |  | The address of the asset to set as collateral |
| use_as_collateral | [bool](#bool) |  | Whether to use the asset as collateral |






<a name="steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1Calls"></a>

### AaveV2EnableAssetAsCollateralAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveV2EnableAssetAsCollateralAdaptorV1](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1) | repeated |  |





 

 

 

 



<a name="aave_v2_stablecoin-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v2_stablecoin.proto



<a name="steward-v4-AaveV2Stablecoin"></a>

### AaveV2Stablecoin
Represents a function call to the Aave V2 Stablecoin cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| accrue | [AaveV2Stablecoin.Accrue](#steward-v4-AaveV2Stablecoin-Accrue) |  | Represents function `accruePlatformFees()` |
| claim_and_unstake | [AaveV2Stablecoin.ClaimAndUnstake](#steward-v4-AaveV2Stablecoin-ClaimAndUnstake) |  | Represents function `claimAndUnstake()` |
| enter_position | [AaveV2Stablecoin.EnterPosition](#steward-v4-AaveV2Stablecoin-EnterPosition) |  | Represents function `enterPosition()` |
| enter_position_with_assets | [AaveV2Stablecoin.EnterPositionWithAssets](#steward-v4-AaveV2Stablecoin-EnterPositionWithAssets) |  | Represents function `enterPosition(uint256 assets)` |
| exit_position | [AaveV2Stablecoin.ExitPosition](#steward-v4-AaveV2Stablecoin-ExitPosition) |  | Represents function `exitPosition()` |
| exit_position_with_assets | [AaveV2Stablecoin.ExitPositionWithAssets](#steward-v4-AaveV2Stablecoin-ExitPositionWithAssets) |  | Represents function `exitPosition(uint256 assets)` |
| rebalance | [AaveV2Stablecoin.Rebalance](#steward-v4-AaveV2Stablecoin-Rebalance) |  | Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)` |
| reinvest | [AaveV2Stablecoin.Reinvest](#steward-v4-AaveV2Stablecoin-Reinvest) |  | Represents function `reinvest(uint256 minAssetsOut)` |
| set_accrual_period | [AaveV2Stablecoin.SetAccrualPeriod](#steward-v4-AaveV2Stablecoin-SetAccrualPeriod) |  | Represents function `setAccrualPeriod(uint32 newAccrualPeriod)` |
| set_deposit_limit | [AaveV2Stablecoin.SetDepositLimit](#steward-v4-AaveV2Stablecoin-SetDepositLimit) |  | Represents function `setDepositLimit(uint256 limit)` |
| set_liquidity_limit | [AaveV2Stablecoin.SetLiquidityLimit](#steward-v4-AaveV2Stablecoin-SetLiquidityLimit) |  | Represents function `setLiquidityLimit(uint256 limit)` |
| send_fees | [AaveV2Stablecoin.SendFees](#steward-v4-AaveV2Stablecoin-SendFees) |  | Represents function `transferFees()` |






<a name="steward-v4-AaveV2Stablecoin-Accrue"></a>

### AaveV2Stablecoin.Accrue
Accrue yield, platform fees, and performance fees..

Represents function `accrue()`






<a name="steward-v4-AaveV2Stablecoin-ClaimAndUnstake"></a>

### AaveV2Stablecoin.ClaimAndUnstake
Claim rewards from Aave and begin cooldown period to unstake them.

Represents function `claimAndUnstake()`






<a name="steward-v4-AaveV2Stablecoin-EnterPosition"></a>

### AaveV2Stablecoin.EnterPosition
Pushes total assets into the current Aave lending position.

Represents function `enterPosition()`






<a name="steward-v4-AaveV2Stablecoin-EnterPositionWithAssets"></a>

### AaveV2Stablecoin.EnterPositionWithAssets
Pushes assets into the current Aave lending position.

Represents function `enterPosition(uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| assets | [string](#string) |  | amount of assets to enter into the current position |






<a name="steward-v4-AaveV2Stablecoin-ExitPosition"></a>

### AaveV2Stablecoin.ExitPosition
Pulls total assets from the current Aave lending position.

Represents function `enterPosition()`






<a name="steward-v4-AaveV2Stablecoin-ExitPositionWithAssets"></a>

### AaveV2Stablecoin.ExitPositionWithAssets
Pulls assets from the current Aave lending position.

Represents function `exitPosition(uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| assets | [string](#string) |  | amount of assets to exit from the current position |






<a name="steward-v4-AaveV2Stablecoin-Rebalance"></a>

### AaveV2Stablecoin.Rebalance
Rebalances current assets into a new asset position.

Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`

This function is based on the Curve Pool Registry exchange_multiple() function:
https://github.com/curvefi/curve-pool-registry/blob/16a8664952cf61d7fed06acca79ad5ac696f4b20/contracts/Swaps.vy#L461-L489


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| route | [string](#string) | repeated | array of [initial token, pool, token, pool, token, ...] that specifies the swap route on Curve. |
| swap_params | [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v4-AaveV2Stablecoin-Rebalance-SwapParams) | repeated | An array of up to 4 swap params. Attempting more than four swaps will fail. |
| min_assets_out | [string](#string) |  | Minimum acceptable assets to be received from the swap (slippage parameter). Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v4-AaveV2Stablecoin-Rebalance-SwapParams"></a>

### AaveV2Stablecoin.Rebalance.SwapParams
Represents parameters for a single swap. Each swap needs the indeces in Rebalance.route of the in/out token addresses and the swap type. See the Curve contract linked above for more detail.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| in_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s input token address |
| out_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s output token address |
| swap_type | [uint64](#uint64) |  | 1 - stableswap `exchange` 2 - stableswap `exchange_underlying` 3 - cryptoswap `exchange` 4 - cryptoswap `exchange_underlying` 5 - Polygon factory metapools `exchange_underlying` See the Curve Pool Registry exchange_multiple() function for more information. |






<a name="steward-v4-AaveV2Stablecoin-Reinvest"></a>

### AaveV2Stablecoin.Reinvest
Reinvest rewards back into cellar&#39;s current position. Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.

Represents function `reinvest(uint256 minAssetsOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_assets_out | [string](#string) |  | Minimum acceptable assets to be received from the swap (slippage parameter). Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v4-AaveV2Stablecoin-SendFees"></a>

### AaveV2Stablecoin.SendFees
Transfer accrued fees to the Sommelier Chain to distribute.

Represents function `sendFees()`






<a name="steward-v4-AaveV2Stablecoin-SetAccrualPeriod"></a>

### AaveV2Stablecoin.SetAccrualPeriod
Set the accrual period over which yield is distributed.

Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_accrual_period | [uint32](#uint32) |  |  |






<a name="steward-v4-AaveV2Stablecoin-SetDepositLimit"></a>

### AaveV2Stablecoin.SetDepositLimit
Set the per-wallet deposit limit. Uses the same decimals as the current asset.

Represents function `setDepositLimit(uint256 limit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [string](#string) |  | Amount of assets to set as the new limit. Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v4-AaveV2Stablecoin-SetLiquidityLimit"></a>

### AaveV2Stablecoin.SetLiquidityLimit
Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.

Represents function `setLiquidityLimit(uint256 limit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [string](#string) |  | Amount of assets to set as the new limit |






<a name="steward-v4-AaveV2StablecoinGovernance"></a>

### AaveV2StablecoinGovernance
Represents a function call initiated by governance


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| set_fees_distributor | [AaveV2StablecoinGovernance.SetFeesDistributor](#steward-v4-AaveV2StablecoinGovernance-SetFeesDistributor) |  | Represents function `setFeesDistributor(bytes32)` |
| initiate_shutdown | [AaveV2StablecoinGovernance.InitiateShutdown](#steward-v4-AaveV2StablecoinGovernance-InitiateShutdown) |  | Represents function `initiateShutdown(bool)` |
| lift_shutdown | [AaveV2StablecoinGovernance.LiftShutdown](#steward-v4-AaveV2StablecoinGovernance-LiftShutdown) |  | Represents function `liftShutdown()` |
| set_trust | [AaveV2StablecoinGovernance.SetTrust](#steward-v4-AaveV2StablecoinGovernance-SetTrust) |  | Represents function `setTrust(address, bool)` |
| sweep | [AaveV2StablecoinGovernance.Sweep](#steward-v4-AaveV2StablecoinGovernance-Sweep) |  | Represents function `sweep(address, address)` |






<a name="steward-v4-AaveV2StablecoinGovernance-InitiateShutdown"></a>

### AaveV2StablecoinGovernance.InitiateShutdown
Represents function `initiateShutdown(bool)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| empty_position | [bool](#bool) |  | Whether to empty the position |






<a name="steward-v4-AaveV2StablecoinGovernance-LiftShutdown"></a>

### AaveV2StablecoinGovernance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v4-AaveV2StablecoinGovernance-SetFeesDistributor"></a>

### AaveV2StablecoinGovernance.SetFeesDistributor
Represents function `setFeesDistributor(bytes32)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_fees_distributor | [string](#string) |  | The new fees distributor |






<a name="steward-v4-AaveV2StablecoinGovernance-SetTrust"></a>

### AaveV2StablecoinGovernance.SetTrust
Represents function `setTrust(address, bool)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [string](#string) |  | The position to set trust for |
| trust | [bool](#bool) |  | Whether to trust the address |






<a name="steward-v4-AaveV2StablecoinGovernance-Sweep"></a>

### AaveV2StablecoinGovernance.Sweep
Represents function `sweep(address, address)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to sweep |
| recipient | [string](#string) |  | The recipient of the sweep |





 

 

 

 



<a name="aave_v3_a_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v3_a_token.proto



<a name="steward-v4-AaveV3ATokenAdaptorV1"></a>

### AaveV3ATokenAdaptorV1
Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave | [AaveV3ATokenAdaptorV1.DepositToAave](#steward-v4-AaveV3ATokenAdaptorV1-DepositToAave) |  | Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave | [AaveV3ATokenAdaptorV1.WithdrawFromAave](#steward-v4-AaveV3ATokenAdaptorV1-WithdrawFromAave) |  | Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |
| adjust_isolation_mode_asset_as_collateral | [AaveV3ATokenAdaptorV1.AdjustIsolationModeAssetAsCollateral](#steward-v4-AaveV3ATokenAdaptorV1-AdjustIsolationModeAssetAsCollateral) |  | Represents function `adjustIsolationModeAssetAsCollateral(ERC20 asset, bool useAsCollateral)` |
| change_emode | [AaveV3ATokenAdaptorV1.ChangeEMode](#steward-v4-AaveV3ATokenAdaptorV1-ChangeEMode) |  | Represents function `changeEMode(uint8 categoryId)` |






<a name="steward-v4-AaveV3ATokenAdaptorV1-AdjustIsolationModeAssetAsCollateral"></a>

### AaveV3ATokenAdaptorV1.AdjustIsolationModeAssetAsCollateral
Allows strategists to adjust an asset&#39;s isolation mode.

Represents function `adjustIsolationModeAssetAsCollateral(ERC20 asset, bool useAsCollateral)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset | [string](#string) |  | The address of the ERC20 token |
| use_as_collateral | [bool](#bool) |  | Whether to use the asset as collateral |






<a name="steward-v4-AaveV3ATokenAdaptorV1-ChangeEMode"></a>

### AaveV3ATokenAdaptorV1.ChangeEMode
Allows strategists to enter different EModes.

Represents function `changeEMode(uint8 categoryId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| category_id | [uint32](#uint32) |  | The category ID |






<a name="steward-v4-AaveV3ATokenAdaptorV1-DepositToAave"></a>

### AaveV3ATokenAdaptorV1.DepositToAave
Allows strategists to lend assets on Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to deposit |
| amount | [string](#string) |  | The amount to deposit |






<a name="steward-v4-AaveV3ATokenAdaptorV1-WithdrawFromAave"></a>

### AaveV3ATokenAdaptorV1.WithdrawFromAave
Allows strategists to withdraw assets from Aave.

Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to withdraw |
| amount | [string](#string) |  | The amount to withdraw |






<a name="steward-v4-AaveV3ATokenAdaptorV1Calls"></a>

### AaveV3ATokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveV3ATokenAdaptorV1](#steward-v4-AaveV3ATokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="aave_v3_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v3_debt_token.proto



<a name="steward-v4-AaveV3DebtTokenAdaptorV1"></a>

### AaveV3DebtTokenAdaptorV1
Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveV3DebtTokenAdaptorV1.BorrowFromAave](#steward-v4-AaveV3DebtTokenAdaptorV1-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveV3DebtTokenAdaptorV1.RepayAaveDebt](#steward-v4-AaveV3DebtTokenAdaptorV1-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |
| repay_with_a_tokens | [AaveV3DebtTokenAdaptorV1.RepayWithATokens](#steward-v4-AaveV3DebtTokenAdaptorV1-RepayWithATokens) |  | Represents function `repayWithATokens(ERC20 underlying, uint256 amount)` |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1-BorrowFromAave"></a>

### AaveV3DebtTokenAdaptorV1.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1-RepayAaveDebt"></a>

### AaveV3DebtTokenAdaptorV1.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1-RepayWithATokens"></a>

### AaveV3DebtTokenAdaptorV1.RepayWithATokens
Allows strategist to use aTokens to repay debt tokens with the same underlying.

Represents function `repayWithATokens(ERC20 underlying, uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| underlying_token | [string](#string) |  | The address of the underlying ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1Calls"></a>

### AaveV3DebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveV3DebtTokenAdaptorV1](#steward-v4-AaveV3DebtTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="aave_v3_debt_token_flash_loan-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v3_debt_token_flash_loan.proto



<a name="steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan"></a>

### AaveV3DebtTokenAdaptorV1FlashLoan
Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| flash_loan | [AaveV3DebtTokenAdaptorV1FlashLoan.FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-FlashLoan) |  | Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)` |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-AdaptorCallForAaveV3FlashLoan"></a>

### AaveV3DebtTokenAdaptorV1FlashLoan.AdaptorCallForAaveV3FlashLoan
Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| aave_a_token_v1_calls | [AaveATokenAdaptorV1Calls](#steward-v4-AaveATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenAdaptor V1 |
| aave_debt_token_v1_calls | [AaveDebtTokenAdaptorV1Calls](#steward-v4-AaveDebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenAdaptor V1 |
| compound_c_token_v2_calls | [CompoundCTokenAdaptorV2Calls](#steward-v4-CompoundCTokenAdaptorV2Calls) |  | Represents function calls to the CompoundCTokenAdaptor V2 |
| aave_a_token_v2_calls | [AaveATokenAdaptorV2Calls](#steward-v4-AaveATokenAdaptorV2Calls) |  | Represents function calls to the AaveATokenV2Adaptor |
| aave_debt_token_v2_calls | [AaveDebtTokenAdaptorV2Calls](#steward-v4-AaveDebtTokenAdaptorV2Calls) |  | Represents function calls to the AavaDebtTokenV2Adaptor |
| aave_v3_a_token_v1_calls | [AaveV3ATokenAdaptorV1Calls](#steward-v4-AaveV3ATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenV1Adaptor |
| one_inch_v1_calls | [OneInchAdaptorV1Calls](#steward-v4-OneInchAdaptorV1Calls) |  | Represents function calls to the OneInchAdaptorV1 |
| fees_and_reserves_v1_calls | [FeesAndReservesAdaptorV1Calls](#steward-v4-FeesAndReservesAdaptorV1Calls) |  | Represents function calls to the FeesAndReservesAdaptorV1 |
| zero_x_v1_calls | [ZeroXAdaptorV1Calls](#steward-v4-ZeroXAdaptorV1Calls) |  | Represents functionc alls to the ZeroXAdaptorV1 |
| swap_with_uniswap_v1_calls | [SwapWithUniswapAdaptorV1Calls](#steward-v4-SwapWithUniswapAdaptorV1Calls) |  | Represents function calls to the SwapWithUniswapAdaptorV1 |
| vesting_simple_v2_calls | [VestingSimpleAdaptorV2Calls](#steward-v4-VestingSimpleAdaptorV2Calls) |  | Represents function calls to VestingSimpleAdaptor |
| cellar_v1_calls | [CellarAdaptorV1Calls](#steward-v4-CellarAdaptorV1Calls) |  | Represents function calls to the CellarAdaptor |
| uniswap_v3_v2_calls | [UniswapV3AdaptorV2Calls](#steward-v4-UniswapV3AdaptorV2Calls) |  | Represents function calls to the UniswapV3Adaptor V2 |
| aave_v2_enable_asset_as_collateral_v1_calls | [AaveV2EnableAssetAsCollateralAdaptorV1Calls](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1Calls) |  | Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1 |
| f_token_v1_calls | [FTokenAdaptorV1Calls](#steward-v4-FTokenAdaptorV1Calls) |  | Represents function calls to the FTokenAdaptor V1 |
| morpho_aave_v2_a_token_v1_calls | [MorphoAaveV2ATokenAdaptorV1Calls](#steward-v4-MorphoAaveV2ATokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2AToken V1 |
| morpho_aave_v2_debt_token_v1_calls | [MorphoAaveV2DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2DebtToken V1 |
| morpho_aave_v3_a_token_collateral_v1_calls | [MorphoAaveV3ATokenCollateralAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenCollateral V1 |
| morpho_aave_v3_a_token_p2p_v1_calls | [MorphoAaveV3ATokenP2PAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenP2P V1 |
| morpho_aave_v3_debt_token_v1_calls | [MorphoAaveV3DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3DebtToken V1 |
| balancer_pool_v1_calls | [BalancerPoolAdaptorV1Calls](#steward-v4-BalancerPoolAdaptorV1Calls) |  | Represents function calls to the BalancerPoolAdaptor V1 |
| legacy_cellar_v1_calls | [LegacyCellarAdaptorV1Calls](#steward-v4-LegacyCellarAdaptorV1Calls) |  | Represents function calls to the LegacyCellarAdaptor V1 |
| debt_f_token_v1_calls | [DebtFTokenAdaptorV1Calls](#steward-v4-DebtFTokenAdaptorV1Calls) |  | Represents function calls to the DebtFTokenAdaptor V1 |
| collateral_f_token_v1_calls | [CollateralFTokenAdaptorV1Calls](#steward-v4-CollateralFTokenAdaptorV1Calls) |  | Represents function calls to the CollateralFTokenAdaptor V1 |
| convex_curve_v1_calls | [ConvexCurveAdaptorV1Calls](#steward-v4-ConvexCurveAdaptorV1Calls) |  | Represents function calls for the ConvexCurveAdaptorV1 |
| curve_v1_calls | [CurveAdaptorV1Calls](#steward-v4-CurveAdaptorV1Calls) |  | Represents function calls for the CurveAdaptorV1 |
| aura_erc4626_v1_calls | [AuraERC4626AdaptorV1Calls](#steward-v4-AuraERC4626AdaptorV1Calls) |  | Represents function calls for the AuraERC4626AdaptorV1 |
| morpho_blue_collateral_v1_calls | [MorphoBlueCollateralAdaptorV1Calls](#steward-v4-MorphoBlueCollateralAdaptorV1Calls) |  | Represents function calls for the MorphoBlueCollateralAdaptorV1 |
| morpho_blue_debt_v1_calls | [MorphoBlueDebtAdaptorV1Calls](#steward-v4-MorphoBlueDebtAdaptorV1Calls) |  | Represents function calls for the MorphoBlueDebtAdaptorV1 |
| morpho_blue_supply_v1_calls | [MorphoBlueSupplyAdaptorV1Calls](#steward-v4-MorphoBlueSupplyAdaptorV1Calls) |  | Represents function calls for the MorphoBlueSupplyAdaptorV1 |
| erc4626_v1_calls | [ERC4626AdaptorV1Calls](#steward-v4-ERC4626AdaptorV1Calls) |  | Represents function calls for the ERC4626AdaptorV1 |
| staking_v1_calls | [StakingAdaptorV1Calls](#steward-v4-StakingAdaptorV1Calls) |  | Represents function calls for the StakingAdaptorV1 |
| aave_v3_debt_token_v1_calls | [AaveV3DebtTokenAdaptorV1Calls](#steward-v4-AaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls for the AaveV3DebtTokenAdaptor V1 |
| pendle_v1_calls | [PendleAdaptorV1Calls](#steward-v4-PendleAdaptorV1Calls) |  | Represents function calls for the PendleAdaptorV1 |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-FlashLoan"></a>

### AaveV3DebtTokenAdaptorV1FlashLoan.FlashLoan
Allows strategists to have Cellars take out flash loans

Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| loan_tokens | [string](#string) | repeated | The addresses of the ERC20 tokens to borrow |
| loan_amounts | [string](#string) | repeated | The amounts to borrow |
| params | [AaveV3DebtTokenAdaptorV1FlashLoan.AdaptorCallForAaveV3FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan-AdaptorCallForAaveV3FlashLoan) | repeated | The params to pass to the flash loan callback. |






<a name="steward-v4-AaveV3DebtTokenAdaptorV1FlashLoanCalls"></a>

### AaveV3DebtTokenAdaptorV1FlashLoanCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveV3DebtTokenAdaptorV1FlashLoan](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoan) | repeated |  |





 

 

 

 



<a name="aura_erc4626-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aura_erc4626.proto



<a name="steward-v4-AuraERC4626AdaptorV1"></a>

### AuraERC4626AdaptorV1
Represents call data for the Aura ERC4626 adaptor V1


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| get_rewards | [AuraERC4626AdaptorV1.GetRewards](#steward-v4-AuraERC4626AdaptorV1-GetRewards) |  | Represents function `getRewards(IBaseRewardPool _auraPool, bool _claimExtras)` |






<a name="steward-v4-AuraERC4626AdaptorV1-GetRewards"></a>

### AuraERC4626AdaptorV1.GetRewards
Allows strategist to get rewards for an Aura pool.

Represents function `getRewards(IBaseRewardPool _auraPool, bool _claimExtras)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| aura_pool | [string](#string) |  | The address of the Aura pool to get rewards for |
| claim_extras | [bool](#bool) |  | Whether to claim extra rewards associated with the pool |






<a name="steward-v4-AuraERC4626AdaptorV1Calls"></a>

### AuraERC4626AdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AuraERC4626AdaptorV1](#steward-v4-AuraERC4626AdaptorV1) | repeated |  |





 

 

 

 



<a name="balancer_pool-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## balancer_pool.proto



<a name="steward-v4-BalancerPoolAdaptorV1"></a>

### BalancerPoolAdaptorV1
Represents call data for the Balancer Pool adaptor V1, for managing pool positions on Balancer.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| join_pool | [BalancerPoolAdaptorV1.JoinPool](#steward-v4-BalancerPoolAdaptorV1-JoinPool) |  | Represents function `relayerJoinPool(ERC20[] tokensIn, uint256[] amountsIn, ERC20 btpOut, bytes[] memory callData)` |
| exit_pool | [BalancerPoolAdaptorV1.ExitPool](#steward-v4-BalancerPoolAdaptorV1-ExitPool) |  | Represents function `relayerExitPool(ERC20 bptIn, uint256 amountIn, ERC20[] memory tokensOut, bytes[] memory callData)` |
| stake_bpt | [BalancerPoolAdaptorV1.StakeBPT](#steward-v4-BalancerPoolAdaptorV1-StakeBPT) |  | Represents function `stakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountIn)` |
| unstake_bpt | [BalancerPoolAdaptorV1.UnstakeBPT](#steward-v4-BalancerPoolAdaptorV1-UnstakeBPT) |  | Represents function `unstakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountOut)` |
| claim_rewards | [BalancerPoolAdaptorV1.ClaimRewards](#steward-v4-BalancerPoolAdaptorV1-ClaimRewards) |  | Represents function `claimRewards(address gauge)` |






<a name="steward-v4-BalancerPoolAdaptorV1-ClaimRewards"></a>

### BalancerPoolAdaptorV1.ClaimRewards
Claim rewards ($BAL) from LP position

Represents `function claimRewards(address gauge)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gauge | [string](#string) |  | The gauge to claim rewards from |






<a name="steward-v4-BalancerPoolAdaptorV1-ExitPool"></a>

### BalancerPoolAdaptorV1.ExitPool
Call `BalancerRelayer` on mainnet to carry out exit txs

Represents function `exitPool(ERC20 targetBpt, IVault.SingleSwap[] memory swapsBeforeJoin, SwapData memory swapData, IVault.ExitPoolRequest request)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target_bpt | [string](#string) |  | The target pool |
| swaps_after_exit | [BalancerPoolAdaptorV1.SingleSwap](#steward-v4-BalancerPoolAdaptorV1-SingleSwap) | repeated | Swaps to execute after exiting pool |
| swap_data | [BalancerPoolAdaptorV1.SwapData](#steward-v4-BalancerPoolAdaptorV1-SwapData) |  | Data for swaps |
| request | [BalancerPoolAdaptorV1.ExitPoolRequest](#steward-v4-BalancerPoolAdaptorV1-ExitPoolRequest) |  |  |






<a name="steward-v4-BalancerPoolAdaptorV1-ExitPoolRequest"></a>

### BalancerPoolAdaptorV1.ExitPoolRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| assets | [string](#string) | repeated |  |
| min_amounts_out | [string](#string) | repeated |  |
| user_data | [bytes](#bytes) |  |  |
| to_internal_balance | [bool](#bool) |  |  |






<a name="steward-v4-BalancerPoolAdaptorV1-JoinPool"></a>

### BalancerPoolAdaptorV1.JoinPool
Allows strategists to join Balancer pools using EXACT_TOKENS_IN_FOR_BPT_OUT joins

Represents function `joinPool(ERC20 targetBpt, IVault.SingleSwap[] memory swapsBeforeJoin, SwapData memory swapData, uint256 minimumBpt)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target_bpt | [string](#string) |  | The target pool |
| swaps_before_join | [BalancerPoolAdaptorV1.SingleSwap](#steward-v4-BalancerPoolAdaptorV1-SingleSwap) | repeated | Swap to execute before joining pool |
| swap_data | [BalancerPoolAdaptorV1.SwapData](#steward-v4-BalancerPoolAdaptorV1-SwapData) |  | Data for swaps |
| minimum_bpt | [string](#string) |  | The minimum BPT to mint |






<a name="steward-v4-BalancerPoolAdaptorV1-SingleSwap"></a>

### BalancerPoolAdaptorV1.SingleSwap
Data for a single swap executed by `swap`. `amount` is either `amountIn` or `amountOut` depending on the `kind` value.
Represents the SingleSwap struct defined here:
https://github.com/PeggyJV/cellar-contracts/blob/main/src/interfaces/external/Balancer/IVault.sol


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pool_id | [string](#string) |  | The pool ID (bytes32) |
| kind | [BalancerPoolAdaptorV1.SwapKind](#steward-v4-BalancerPoolAdaptorV1-SwapKind) |  | The swap kind (enum) |
| asset_in | [string](#string) |  | The asset in (address) |
| asset_out | [string](#string) |  | The asset out (address) |
| amount | [string](#string) |  | The amount (uint256) |
| user_data | [bytes](#bytes) |  | The user data (bytes) |






<a name="steward-v4-BalancerPoolAdaptorV1-StakeBPT"></a>

### BalancerPoolAdaptorV1.StakeBPT
Stake (deposit) BPTs into respective pool gauge

Represents `function stakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountIn)``


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bpt | [string](#string) |  | The BPT to stake |
| liquidity_gauge | [string](#string) |  | The liquidity gauge to stake into |
| amount_in | [string](#string) |  | The amount to stake |






<a name="steward-v4-BalancerPoolAdaptorV1-SwapData"></a>

### BalancerPoolAdaptorV1.SwapData
Stores each swaps min amount, and deadline


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_amounts_for_swaps | [string](#string) | repeated | The minimum amounts for swaps |
| swap_deadlines | [string](#string) | repeated | The swap deadlines |






<a name="steward-v4-BalancerPoolAdaptorV1-UnstakeBPT"></a>

### BalancerPoolAdaptorV1.UnstakeBPT
Unstake (withdraw) BPT from respective pool gauge

Represents `function unstakeBPT(ERC20 _bpt, address _liquidityGauge, uint256 _amountOut)``


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| bpt | [string](#string) |  | The BPT to unstake |
| liquidity_gauge | [string](#string) |  | The liquidity gauge to unstake from |
| amount_out | [string](#string) |  | The amount to unstake |






<a name="steward-v4-BalancerPoolAdaptorV1Calls"></a>

### BalancerPoolAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [BalancerPoolAdaptorV1](#steward-v4-BalancerPoolAdaptorV1) | repeated |  |





 


<a name="steward-v4-BalancerPoolAdaptorV1-SwapKind"></a>

### BalancerPoolAdaptorV1.SwapKind
Represents the SwapKind enum defined here:
https://github.com/PeggyJV/cellar-contracts/blob/main/src/interfaces/external/Balancer/IVault.sol

| Name | Number | Description |
| ---- | ------ | ----------- |
| SWAP_KIND_UNSPECIFIED | 0 |  |
| SWAP_KIND_GIVEN_IN | 1 |  |
| SWAP_KIND_GIVEN_OUT | 2 |  |


 

 

 



<a name="balancer_pool_flash_loan-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## balancer_pool_flash_loan.proto



<a name="steward-v4-BalancerPoolAdaptorV1FlashLoan"></a>

### BalancerPoolAdaptorV1FlashLoan
Represents flash loan call data for the Balancer Pool adaptor V1, for managing pool positions on Balancer.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| make_flash_loan | [BalancerPoolAdaptorV1FlashLoan.MakeFlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan-MakeFlashLoan) |  |  |






<a name="steward-v4-BalancerPoolAdaptorV1FlashLoan-AdaptorCallForBalancerPoolFlashLoan"></a>

### BalancerPoolAdaptorV1FlashLoan.AdaptorCallForBalancerPoolFlashLoan
NOTE: Make FlashLoan takes an array of AdaptorCall. cellar_v2.proto defines it, but also imports this file, therefore we can&#39;t import cellar_v2.proto in order to use the AdaptorCall message here. To avoid the recursive import, we duplicate the message definition.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| aave_a_token_v1_calls | [AaveATokenAdaptorV1Calls](#steward-v4-AaveATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenAdaptor V1 |
| aave_debt_token_v1_calls | [AaveDebtTokenAdaptorV1Calls](#steward-v4-AaveDebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenAdaptor V1 |
| compound_c_token_v2_calls | [CompoundCTokenAdaptorV2Calls](#steward-v4-CompoundCTokenAdaptorV2Calls) |  | Represents function calls to the CompoundCTokenAdaptor V2 |
| aave_a_token_v2_calls | [AaveATokenAdaptorV2Calls](#steward-v4-AaveATokenAdaptorV2Calls) |  | Represents function calls to the AaveATokenV2Adaptor |
| aave_debt_token_v2_calls | [AaveDebtTokenAdaptorV2Calls](#steward-v4-AaveDebtTokenAdaptorV2Calls) |  | Represents function calls to the AavaDebtTokenV2Adaptor |
| aave_v3_a_token_v1_calls | [AaveV3ATokenAdaptorV1Calls](#steward-v4-AaveV3ATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenV1Adaptor |
| aave_v3_debt_token_v1_calls | [AaveV3DebtTokenAdaptorV1Calls](#steward-v4-AaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenV1Adaptor |
| one_inch_v1_calls | [OneInchAdaptorV1Calls](#steward-v4-OneInchAdaptorV1Calls) |  | Represents function calls to the OneInchAdaptorV1 |
| fees_and_reserves_v1_calls | [FeesAndReservesAdaptorV1Calls](#steward-v4-FeesAndReservesAdaptorV1Calls) |  | Represents function calls to the FeesAndReservesAdaptorV1 |
| zero_x_v1_calls | [ZeroXAdaptorV1Calls](#steward-v4-ZeroXAdaptorV1Calls) |  | Represents functionc alls to the ZeroXAdaptorV1 |
| swap_with_uniswap_v1_calls | [SwapWithUniswapAdaptorV1Calls](#steward-v4-SwapWithUniswapAdaptorV1Calls) |  | Represents function calls to the SwapWithUniswapAdaptorV1 |
| vesting_simple_v2_calls | [VestingSimpleAdaptorV2Calls](#steward-v4-VestingSimpleAdaptorV2Calls) |  | Represents function calls to VestingSimpleAdaptor |
| cellar_v1_calls | [CellarAdaptorV1Calls](#steward-v4-CellarAdaptorV1Calls) |  | Represents function calls to the CellarAdaptor |
| uniswap_v3_v2_calls | [UniswapV3AdaptorV2Calls](#steward-v4-UniswapV3AdaptorV2Calls) |  | Represents function calls to the UniswapV3Adaptor V2 |
| aave_v2_enable_asset_as_collateral_v1_calls | [AaveV2EnableAssetAsCollateralAdaptorV1Calls](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1Calls) |  | Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1 |
| f_token_v1_calls | [FTokenAdaptorV1Calls](#steward-v4-FTokenAdaptorV1Calls) |  | Represents function calls to the FTokenAdaptor V1 |
| morpho_aave_v2_a_token_v1_calls | [MorphoAaveV2ATokenAdaptorV1Calls](#steward-v4-MorphoAaveV2ATokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2AToken V1 |
| morpho_aave_v2_debt_token_v1_calls | [MorphoAaveV2DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2DebtToken V1 |
| morpho_aave_v3_a_token_collateral_v1_calls | [MorphoAaveV3ATokenCollateralAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenCollateral V1 |
| morpho_aave_v3_a_token_p2p_v1_calls | [MorphoAaveV3ATokenP2PAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenP2P V1 |
| morpho_aave_v3_debt_token_v1_calls | [MorphoAaveV3DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3DebtToken V1 |
| legacy_cellar_v1_calls | [LegacyCellarAdaptorV1Calls](#steward-v4-LegacyCellarAdaptorV1Calls) |  | Represents function calls to the LegacyCellarAdaptor V1 |
| debt_f_token_v1_calls | [DebtFTokenAdaptorV1Calls](#steward-v4-DebtFTokenAdaptorV1Calls) |  | Represents function calls to the DebtFTokenAdaptor V1 |
| collateral_f_token_v1_calls | [CollateralFTokenAdaptorV1Calls](#steward-v4-CollateralFTokenAdaptorV1Calls) |  | Represents function calls to the CollateralFTokenAdaptor V1 |
| convex_curve_v1_calls | [ConvexCurveAdaptorV1Calls](#steward-v4-ConvexCurveAdaptorV1Calls) |  | Represents function calls for the ConvexCurveAdaptorV1 |
| curve_v1_calls | [CurveAdaptorV1Calls](#steward-v4-CurveAdaptorV1Calls) |  | Represents function calls for the CurveAdaptorV1 |
| aura_erc4626_v1_calls | [AuraERC4626AdaptorV1Calls](#steward-v4-AuraERC4626AdaptorV1Calls) |  | Represents function calls for the AuraERC4626AdaptorV1 |
| morpho_blue_collateral_v1_calls | [MorphoBlueCollateralAdaptorV1Calls](#steward-v4-MorphoBlueCollateralAdaptorV1Calls) |  | Represents function calls for the MorphoBlueCollateralAdaptorV1 |
| morpho_blue_debt_v1_calls | [MorphoBlueDebtAdaptorV1Calls](#steward-v4-MorphoBlueDebtAdaptorV1Calls) |  | Represents function calls for the MorphoBlueDebtAdaptorV1 |
| morpho_blue_supply_v1_calls | [MorphoBlueSupplyAdaptorV1Calls](#steward-v4-MorphoBlueSupplyAdaptorV1Calls) |  | Represents function calls for the MorphoBlueSupplyAdaptorV1 |
| erc4626_v1_calls | [ERC4626AdaptorV1Calls](#steward-v4-ERC4626AdaptorV1Calls) |  | Represents function calls for the ERC4626AdaptorV1 |
| staking_v1_calls | [StakingAdaptorV1Calls](#steward-v4-StakingAdaptorV1Calls) |  | Represents function calls for the StakingAdaptorV1 |
| pendle_v1_calls | [PendleAdaptorV1Calls](#steward-v4-PendleAdaptorV1Calls) |  | Represents function calls for the PendleAdaptorV1 |






<a name="steward-v4-BalancerPoolAdaptorV1FlashLoan-MakeFlashLoan"></a>

### BalancerPoolAdaptorV1FlashLoan.MakeFlashLoan
Make a flash loan

Represents `function makeFlashLoan(IERC20[] tokens, uint256[] amounts, bytes memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| tokens | [string](#string) | repeated | The tokens to flash loan |
| amounts | [string](#string) | repeated | The amounts to flash loan |
| data | [BalancerPoolAdaptorV1FlashLoan.AdaptorCallForBalancerPoolFlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan-AdaptorCallForBalancerPoolFlashLoan) | repeated | The data to flash loan |






<a name="steward-v4-BalancerPoolAdaptorV1FlashLoanCalls"></a>

### BalancerPoolAdaptorV1FlashLoanCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [BalancerPoolAdaptorV1FlashLoan](#steward-v4-BalancerPoolAdaptorV1FlashLoan) | repeated |  |





 

 

 

 



<a name="base-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## base.proto



<a name="steward-v4-OracleSwap"></a>

### OracleSwap
Helper function to make safe &#34;blind&#34; Uniswap Swaps by comparing value in vs value out of the swap.

Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset_in | [string](#string) |  | Asset to swap from |
| asset_out | [string](#string) |  | Asset to swap to |
| amount_in | [string](#string) |  | Amount to swap |
| exchange | [Exchange](#steward-v4-Exchange) |  | The exchange to make the swap on |
| params | [OracleSwapParams](#steward-v4-OracleSwapParams) |  | The parameters for the swap |
| slippage | [uint64](#uint64) |  | The slippage allowed for the swap |






<a name="steward-v4-RevokeApproval"></a>

### RevokeApproval
Allows strategists to zero out an approval for a given `asset`.

Represents function `revokeApproval(ERC20 asset, address spender)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset | [string](#string) |  | ERC20 Asset to revoke spender&#39;s approval for |
| spender | [string](#string) |  | The spender to revoke approval of asset for |






<a name="steward-v4-Swap"></a>

### Swap
Helper function that allows swaps using the Swap Router

Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset_in | [string](#string) |  | Asset to swap from |
| asset_out | [string](#string) |  | Asset to swap to |
| amount_in | [string](#string) |  | Amount to swap |
| exchange | [Exchange](#steward-v4-Exchange) |  | The exchange to make the swap on |
| params | [SwapParams](#steward-v4-SwapParams) |  | The parameters for the swap |





 

 

 

 



<a name="c_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## c_token.proto



<a name="steward-v4-CompoundCTokenAdaptorV2"></a>

### CompoundCTokenAdaptorV2
Represents call data for the Compound C Token adaptor V2, managing lending positions on Compound.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_compound | [CompoundCTokenAdaptorV2.DepositToCompound](#steward-v4-CompoundCTokenAdaptorV2-DepositToCompound) |  | Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)` |
| withdraw_from_compound | [CompoundCTokenAdaptorV2.WithdrawFromCompound](#steward-v4-CompoundCTokenAdaptorV2-WithdrawFromCompound) |  | Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)` |
| claim_comp | [CompoundCTokenAdaptorV2.ClaimComp](#steward-v4-CompoundCTokenAdaptorV2-ClaimComp) |  | Represents function `claimComp()` |






<a name="steward-v4-CompoundCTokenAdaptorV2-ClaimComp"></a>

### CompoundCTokenAdaptorV2.ClaimComp
Allows strategists to claim COMP rewards.

Represents function `claimComp()`






<a name="steward-v4-CompoundCTokenAdaptorV2-DepositToCompound"></a>

### CompoundCTokenAdaptorV2.DepositToCompound
Allows strategists to lend assets on Compound.

Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  |  |
| amount_to_deposit | [string](#string) |  |  |






<a name="steward-v4-CompoundCTokenAdaptorV2-WithdrawFromCompound"></a>

### CompoundCTokenAdaptorV2.WithdrawFromCompound
Allows strategists to withdraw assets from Compound.

Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  |  |
| amount_to_withdraw | [string](#string) |  |  |






<a name="steward-v4-CompoundCTokenAdaptorV2Calls"></a>

### CompoundCTokenAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [CompoundCTokenAdaptorV2](#steward-v4-CompoundCTokenAdaptorV2) | repeated |  |





 

 

 

 



<a name="cellar_adaptor-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cellar_adaptor.proto



<a name="steward-v4-CellarAdaptorV1"></a>

### CellarAdaptorV1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| depositToCellar | [CellarAdaptorV1.DepositToCellar](#steward-v4-CellarAdaptorV1-DepositToCellar) |  | Represents function `depositToCellar(Cellar cellar, uint256 assets)` |
| withdrawFromCellar | [CellarAdaptorV1.WithdrawFromCellar](#steward-v4-CellarAdaptorV1-WithdrawFromCellar) |  | Represents function `withdrawFromCellar(Cellar cellar, uint256 assets)` |






<a name="steward-v4-CellarAdaptorV1-DepositToCellar"></a>

### CellarAdaptorV1.DepositToCellar
Allows strategists to deposit into Cellar positions.

Represents function `depositToCellar(Cellar cellar, uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar | [string](#string) |  |  |
| assets | [string](#string) |  |  |






<a name="steward-v4-CellarAdaptorV1-WithdrawFromCellar"></a>

### CellarAdaptorV1.WithdrawFromCellar
Allows strategists to withdraw from Cellar positions.

Represents function `withdrawFromCellar(Cellar cellar, uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar | [string](#string) |  |  |
| assets | [string](#string) |  |  |






<a name="steward-v4-CellarAdaptorV1Calls"></a>

### CellarAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [CellarAdaptorV1](#steward-v4-CellarAdaptorV1) | repeated |  |





 

 

 

 



<a name="cellar_v1-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cellar_v1.proto



<a name="steward-v4-CellarV1"></a>

### CellarV1
Represents a function call to a cellar that implements Cellar.sol


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV1.AddPosition](#steward-v4-CellarV1-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| push_position | [CellarV1.PushPosition](#steward-v4-CellarV1-PushPosition) |  | Represents function `pushPosition(address position)` |
| remove_position | [CellarV1.RemovePosition](#steward-v4-CellarV1-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV1.SetHoldingPosition](#steward-v4-CellarV1-SetHoldingPosition) |  | Represents function `setHoldingPosition(address newHoldingPosition)` |
| rebalance | [CellarV1.Rebalance](#steward-v4-CellarV1-Rebalance) |  | Represents function `rebalance(address fromPosition, address toPosition, uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)` |
| set_strategist_payout_address | [CellarV1.SetStrategistPayoutAddress](#steward-v4-CellarV1-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| set_withdraw_type | [CellarV1.SetWithdrawType](#steward-v4-CellarV1-SetWithdrawType) |  | Represents function `setWithdrawType(WithdrawType newWithdrawType)` |
| swap_positions | [CellarV1.SwapPositions](#steward-v4-CellarV1-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_deposit_limit | [CellarV1.SetDepositLimit](#steward-v4-CellarV1-SetDepositLimit) |  | Represents function `setDepositLimit()` |
| set_liquidity_limit | [CellarV1.SetLiquidityLimit](#steward-v4-CellarV1-SetLiquidityLimit) |  | Represents function `setLiquidityLimit()` |
| set_share_lock_period | [CellarV1.SetShareLockPeriod](#steward-v4-CellarV1-SetShareLockPeriod) |  | Represents function `setShareLockPeriod()` |
| set_rebalance_deviation | [CellarV1.SetRebalanceDeviation](#steward-v4-CellarV1-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |






<a name="steward-v4-CellarV1-AddPosition"></a>

### CellarV1.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint256 index, address position)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [string](#string) |  | Index at which to add the position |
| position | [string](#string) |  | Address of the position to add |






<a name="steward-v4-CellarV1-PushPosition"></a>

### CellarV1.PushPosition
Push a trusted position to the end of the list of positions used by the cellar. If you
know you are going to add a position to the end of the array, this is more efficient then
`addPosition`.

Represents function `pushPosition(address position)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [string](#string) |  | Address of the position to push |






<a name="steward-v4-CellarV1-Rebalance"></a>

### CellarV1.Rebalance
Move assets between positions. To move assets from/to this cellar&#39;s holdings, specify
the address of this cellar as the `fromPosition`/`toPosition`.

Represents function `rebalance(address fromPosition, address toPosition,
 uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| from_position | [string](#string) |  |  |
| to_position | [string](#string) |  |  |
| assets_from | [string](#string) |  |  |
| exchange | [Exchange](#steward-v4-Exchange) |  |  |
| params | [SwapParams](#steward-v4-SwapParams) |  |  |






<a name="steward-v4-CellarV1-RemovePosition"></a>

### CellarV1.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint256 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [string](#string) |  | Index at which to remove the position |






<a name="steward-v4-CellarV1-SetDepositLimit"></a>

### CellarV1.SetDepositLimit
Set the per-wallet deposit limit. Uses the same decimals as the current asset.

Represents function `setDepositLimit()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_limit | [string](#string) |  |  |






<a name="steward-v4-CellarV1-SetHoldingPosition"></a>

### CellarV1.SetHoldingPosition
Set the holding position used by the cellar.

Represents function `setHoldingPosition(address newHoldingPosition)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_holding_position | [string](#string) |  | Address of the new holding position to use |






<a name="steward-v4-CellarV1-SetLiquidityLimit"></a>

### CellarV1.SetLiquidityLimit
Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.

Represents function `setLiquidityLimit()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_limit | [string](#string) |  |  |






<a name="steward-v4-CellarV1-SetRebalanceDeviation"></a>

### CellarV1.SetRebalanceDeviation
Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV1-SetShareLockPeriod"></a>

### CellarV1.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v4-CellarV1-SetStrategistPayoutAddress"></a>

### CellarV1.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v4-CellarV1-SetWithdrawType"></a>

### CellarV1.SetWithdrawType
Set the withdraw type used by the cellar.

Represents function `setWithdrawType(WithdrawType newWithdrawType)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_withdraw_type | [CellarV1.WithdrawType](#steward-v4-CellarV1-WithdrawType) |  | The withdraw type to use for the cellar |






<a name="steward-v4-CellarV1-SwapPositions"></a>

### CellarV1.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint256 index1, uint256 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [string](#string) |  | Index of the first position |
| index_2 | [string](#string) |  | Index of the second position |






<a name="steward-v4-CellarV1Governance"></a>

### CellarV1Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiate_shutdown | [CellarV1Governance.InitiateShutdown](#steward-v4-CellarV1Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV1Governance.LiftShutdown](#steward-v4-CellarV1Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| reset_high_watermark | [CellarV1Governance.ResetHighWatermark](#steward-v4-CellarV1Governance-ResetHighWatermark) |  | Represents function `resetHighWatermark()` |
| set_fees_distributor | [CellarV1Governance.SetFeesDistributor](#steward-v4-CellarV1Governance-SetFeesDistributor) |  | Represents function `setFeesDistributor(address)` |
| set_performance_fee | [CellarV1Governance.SetPerformanceFee](#steward-v4-CellarV1Governance-SetPerformanceFee) |  | Represents function `setPerformanceFee(uint256)` |
| set_platform_fee | [CellarV1Governance.SetPlatformFee](#steward-v4-CellarV1Governance-SetPlatformFee) |  | Represents function `setPlatformFee(uint256)` |
| set_strategist_performance_cut | [CellarV1Governance.SetStrategistPerformanceCut](#steward-v4-CellarV1Governance-SetStrategistPerformanceCut) |  | Represents function `setStrategistPerformanceCut(uint256)` |
| set_strategist_platform_cut | [CellarV1Governance.SetStrategistPlatformCut](#steward-v4-CellarV1Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(address)` |
| trust_position | [CellarV1Governance.TrustPosition](#steward-v4-CellarV1Governance-TrustPosition) |  | Represents function `trustPosition(address)` |






<a name="steward-v4-CellarV1Governance-InitiateShutdown"></a>

### CellarV1Governance.InitiateShutdown
Represents function `initiateShutdown()`






<a name="steward-v4-CellarV1Governance-LiftShutdown"></a>

### CellarV1Governance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v4-CellarV1Governance-ResetHighWatermark"></a>

### CellarV1Governance.ResetHighWatermark
Represents function `resetHighWatermark()`






<a name="steward-v4-CellarV1Governance-SetFeesDistributor"></a>

### CellarV1Governance.SetFeesDistributor
Represents function `setFeesDistributor(bytes32)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_fees_distributor | [string](#string) |  | Cosmos address of the new fees distributor |






<a name="steward-v4-CellarV1Governance-SetPerformanceFee"></a>

### CellarV1Governance.SetPerformanceFee
Represents function `setPerformanceFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New performance fee |






<a name="steward-v4-CellarV1Governance-SetPlatformFee"></a>

### CellarV1Governance.SetPlatformFee
Represents function `setPlatformFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New platform fee |






<a name="steward-v4-CellarV1Governance-SetStrategistPerformanceCut"></a>

### CellarV1Governance.SetStrategistPerformanceCut
Represents function `setStrategistPerformanceCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist performance cut |






<a name="steward-v4-CellarV1Governance-SetStrategistPlatformCut"></a>

### CellarV1Governance.SetStrategistPlatformCut
Represents function `setStrategistPlatformCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist platform cut |






<a name="steward-v4-CellarV1Governance-TrustPosition"></a>

### CellarV1Governance.TrustPosition
Represents function `trustPosition(address)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| erc20_address | [string](#string) |  |  |
| erc4626_address | [string](#string) |  |  |
| cellar_address | [string](#string) |  |  |





 


<a name="steward-v4-CellarV1-WithdrawType"></a>

### CellarV1.WithdrawType
Represents the withdraw type to use for the cellar

| Name | Number | Description |
| ---- | ------ | ----------- |
| WITHDRAW_TYPE_UNSPECIFIED | 0 |  |
| WITHDRAW_TYPE_ORDERLY | 1 |  |
| WITHDRAW_TYPE_PROPORTIONAL | 2 |  |


 

 

 



<a name="cellar_v2-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cellar_v2.proto



<a name="steward-v4-AdaptorCall"></a>

### AdaptorCall
Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| aave_a_token_v1_calls | [AaveATokenAdaptorV1Calls](#steward-v4-AaveATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenAdaptor V1 |
| aave_debt_token_v1_calls | [AaveDebtTokenAdaptorV1Calls](#steward-v4-AaveDebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenAdaptor V1 |
| compound_c_token_v2_calls | [CompoundCTokenAdaptorV2Calls](#steward-v4-CompoundCTokenAdaptorV2Calls) |  | Represents function calls to the CompoundCTokenAdaptor V2 |
| aave_a_token_v2_calls | [AaveATokenAdaptorV2Calls](#steward-v4-AaveATokenAdaptorV2Calls) |  | Represents function calls to the AaveATokenV2Adaptor |
| aave_debt_token_v2_calls | [AaveDebtTokenAdaptorV2Calls](#steward-v4-AaveDebtTokenAdaptorV2Calls) |  | Represents function calls to the AavaDebtTokenV2Adaptor |
| aave_v3_a_token_v1_calls | [AaveV3ATokenAdaptorV1Calls](#steward-v4-AaveV3ATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenV1Adaptor |
| aave_v3_debt_token_v1_calls | [AaveV3DebtTokenAdaptorV1Calls](#steward-v4-AaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenV1Adaptor |
| one_inch_v1_calls | [OneInchAdaptorV1Calls](#steward-v4-OneInchAdaptorV1Calls) |  | Represents function calls to the OneInchAdaptorV1 |
| fees_and_reserves_v1_calls | [FeesAndReservesAdaptorV1Calls](#steward-v4-FeesAndReservesAdaptorV1Calls) |  | Represents function calls to the FeesAndReservesAdaptorV1 |
| zero_x_v1_calls | [ZeroXAdaptorV1Calls](#steward-v4-ZeroXAdaptorV1Calls) |  | Represents functionc alls to the ZeroXAdaptorV1 |
| swap_with_uniswap_v1_calls | [SwapWithUniswapAdaptorV1Calls](#steward-v4-SwapWithUniswapAdaptorV1Calls) |  | Represents function calls to the SwapWithUniswapAdaptorV1 |
| vesting_simple_v2_calls | [VestingSimpleAdaptorV2Calls](#steward-v4-VestingSimpleAdaptorV2Calls) |  | Represents function calls to VestingSimpleAdaptor |
| cellar_v1_calls | [CellarAdaptorV1Calls](#steward-v4-CellarAdaptorV1Calls) |  | Represents function calls to the CellarAdaptor |
| uniswap_v3_v2_calls | [UniswapV3AdaptorV2Calls](#steward-v4-UniswapV3AdaptorV2Calls) |  | Represents function calls to the UniswapV3Adaptor V2 |
| aave_v2_enable_asset_as_collateral_v1_calls | [AaveV2EnableAssetAsCollateralAdaptorV1Calls](#steward-v4-AaveV2EnableAssetAsCollateralAdaptorV1Calls) |  | Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1 |
| f_token_v1_calls | [FTokenAdaptorV1Calls](#steward-v4-FTokenAdaptorV1Calls) |  | Represents function calls to the FTokenAdaptor V1 |
| morpho_aave_v2_a_token_v1_calls | [MorphoAaveV2ATokenAdaptorV1Calls](#steward-v4-MorphoAaveV2ATokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2AToken V1 |
| morpho_aave_v2_debt_token_v1_calls | [MorphoAaveV2DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2DebtToken V1 |
| morpho_aave_v3_a_token_collateral_v1_calls | [MorphoAaveV3ATokenCollateralAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenCollateral V1 |
| morpho_aave_v3_a_token_p2p_v1_calls | [MorphoAaveV3ATokenP2PAdaptorV1Calls](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenP2P V1 |
| morpho_aave_v3_debt_token_v1_calls | [MorphoAaveV3DebtTokenAdaptorV1Calls](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3DebtToken V1 |
| balancer_pool_v1_calls | [BalancerPoolAdaptorV1Calls](#steward-v4-BalancerPoolAdaptorV1Calls) |  | Represents function calls to the BalancerPoolAdaptor V1 |
| legacy_cellar_v1_calls | [LegacyCellarAdaptorV1Calls](#steward-v4-LegacyCellarAdaptorV1Calls) |  | Represents function calls to the LegacyCellarAdaptor V1 |
| debt_f_token_v1_calls | [DebtFTokenAdaptorV1Calls](#steward-v4-DebtFTokenAdaptorV1Calls) |  | Represents function calls to the DebtFTokenAdaptor V1 |
| collateral_f_token_v1_calls | [CollateralFTokenAdaptorV1Calls](#steward-v4-CollateralFTokenAdaptorV1Calls) |  | Represents function calls to the CollateralFTokenAdaptor V1 |
| aave_v3_debt_token_v1_flash_loan_calls | [AaveV3DebtTokenAdaptorV1FlashLoanCalls](#steward-v4-AaveV3DebtTokenAdaptorV1FlashLoanCalls) |  | Represents function call for the AaveV3DebtTokenAdaptorV1 |
| balancer_pool_v1_flash_loan_calls | [BalancerPoolAdaptorV1FlashLoanCalls](#steward-v4-BalancerPoolAdaptorV1FlashLoanCalls) |  | Represents function call for the BalancerPoolAdaptorV1 |
| convex_curve_v1_calls | [ConvexCurveAdaptorV1Calls](#steward-v4-ConvexCurveAdaptorV1Calls) |  | Represents function calls for the ConvexCurveAdaptorV1 |
| curve_v1_calls | [CurveAdaptorV1Calls](#steward-v4-CurveAdaptorV1Calls) |  | Represents function calls for the CurveAdaptorV1 |
| aura_erc4626_v1_calls | [AuraERC4626AdaptorV1Calls](#steward-v4-AuraERC4626AdaptorV1Calls) |  | Represents function calls for the AuraERC4626AdaptorV1 |
| morpho_blue_collateral_v1_calls | [MorphoBlueCollateralAdaptorV1Calls](#steward-v4-MorphoBlueCollateralAdaptorV1Calls) |  | Represents function calls for the MorphoBlueCollateralAdaptorV1 |
| morpho_blue_debt_v1_calls | [MorphoBlueDebtAdaptorV1Calls](#steward-v4-MorphoBlueDebtAdaptorV1Calls) |  | Represents function calls for the MorphoBlueDebtAdaptorV1 |
| morpho_blue_supply_v1_calls | [MorphoBlueSupplyAdaptorV1Calls](#steward-v4-MorphoBlueSupplyAdaptorV1Calls) |  | Represents function calls for the MorphoBlueSupplyAdaptorV1 |
| erc4626_v1_calls | [ERC4626AdaptorV1Calls](#steward-v4-ERC4626AdaptorV1Calls) |  | Represents function calls for the ERC4626AdaptorV1 |
| staking_v1_calls | [StakingAdaptorV1Calls](#steward-v4-StakingAdaptorV1Calls) |  | Represents function calls for the StakingAdaptorV1 |
| pendle_v1_calls | [PendleAdaptorV1Calls](#steward-v4-PendleAdaptorV1Calls) |  | Represents function calls for the PendleAdaptorV1 |






<a name="steward-v4-CellarV2"></a>

### CellarV2
Represents a function call to a cellar that implements Cellar.sol


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| call_on_adaptor | [CellarV2.CallOnAdaptor](#steward-v4-CellarV2-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2.RemovePosition](#steward-v4-CellarV2-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV2.SetHoldingPosition](#steward-v4-CellarV2-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| set_strategist_payout_address | [CellarV2.SetStrategistPayoutAddress](#steward-v4-CellarV2-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| swap_positions | [CellarV2.SwapPositions](#steward-v4-CellarV2-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_share_lock_period | [CellarV2.SetShareLockPeriod](#steward-v4-CellarV2-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |
| add_position | [CellarV2.AddPosition](#steward-v4-CellarV2-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| setup_adaptor | [CellarV2.SetupAdaptor](#steward-v4-CellarV2-SetupAdaptor) |  | Represents function `setupAdaptor(address adaptor)` |
| initiate_shutdown | [CellarV2.InitiateShutdown](#steward-v4-CellarV2-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2.LiftShutdown](#steward-v4-CellarV2-LiftShutdown) |  | Represents function `liftShutdown()` |
| set_platform_fee | [CellarV2.SetPlatformFee](#steward-v4-CellarV2-SetPlatformFee) |  | Represents function `setPlatformFee(uint256)` |
| set_strategist_platform_cut | [CellarV2.SetStrategistPlatformCut](#steward-v4-CellarV2-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(address)` |
| set_rebalance_deviation | [CellarV2.SetRebalanceDeviation](#steward-v4-CellarV2-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |






<a name="steward-v4-CellarV2-AddPosition"></a>

### CellarV2.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v4-CellarV2-CallOnAdaptor"></a>

### CellarV2.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v4-AdaptorCall) | repeated |  |






<a name="steward-v4-CellarV2-InitiateShutdown"></a>

### CellarV2.InitiateShutdown
Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2-LiftShutdown"></a>

### CellarV2.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v4-CellarV2-RemovePosition"></a>

### CellarV2.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v4-CellarV2-SetHoldingPosition"></a>

### CellarV2.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v4-CellarV2-SetPlatformFee"></a>

### CellarV2.SetPlatformFee
Represents function `setPlatformFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New platform fee |






<a name="steward-v4-CellarV2-SetRebalanceDeviation"></a>

### CellarV2.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2-SetShareLockPeriod"></a>

### CellarV2.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v4-CellarV2-SetStrategistPayoutAddress"></a>

### CellarV2.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v4-CellarV2-SetStrategistPlatformCut"></a>

### CellarV2.SetStrategistPlatformCut
Represents function `setStrategistPlatformCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist platform cut |






<a name="steward-v4-CellarV2-SetupAdaptor"></a>

### CellarV2.SetupAdaptor
Allows owner to add new adaptors for the cellar to use.

Represents function `setupAdaptor(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |






<a name="steward-v4-CellarV2-SwapPositions"></a>

### CellarV2.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v4-CellarV2Governance"></a>

### CellarV2Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiate_shutdown | [CellarV2Governance.InitiateShutdown](#steward-v4-CellarV2Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2Governance.LiftShutdown](#steward-v4-CellarV2Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| set_platform_fee | [CellarV2Governance.SetPlatformFee](#steward-v4-CellarV2Governance-SetPlatformFee) |  | Represents function `setPlatformFee(uint256)` |
| set_strategist_platform_cut | [CellarV2Governance.SetStrategistPlatformCut](#steward-v4-CellarV2Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(address)` |
| setup_adaptor | [CellarV2Governance.SetupAdaptor](#steward-v4-CellarV2Governance-SetupAdaptor) |  | Represents function `setupAdaptor(address adaptor)` |
| set_rebalance_deviation | [CellarV2Governance.SetRebalanceDeviation](#steward-v4-CellarV2Governance-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |






<a name="steward-v4-CellarV2Governance-InitiateShutdown"></a>

### CellarV2Governance.InitiateShutdown
Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2Governance-LiftShutdown"></a>

### CellarV2Governance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v4-CellarV2Governance-SetPlatformFee"></a>

### CellarV2Governance.SetPlatformFee
Represents function `setPlatformFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New platform fee |






<a name="steward-v4-CellarV2Governance-SetRebalanceDeviation"></a>

### CellarV2Governance.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2Governance-SetStrategistPlatformCut"></a>

### CellarV2Governance.SetStrategistPlatformCut
Represents function `setStrategistPlatformCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist platform cut |






<a name="steward-v4-CellarV2Governance-SetupAdaptor"></a>

### CellarV2Governance.SetupAdaptor
Allows owner to add new adaptors for the cellar to use.

Represents function `setupAdaptor(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |






<a name="steward-v4-CellarV2_2"></a>

### CellarV2_2



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_call | [CellarV2_2.FunctionCall](#steward-v4-CellarV2_2-FunctionCall) |  | Represents a single function call |
| multicall | [CellarV2_2.Multicall](#steward-v4-CellarV2_2-Multicall) |  | Represents multiple, ordered function calls |






<a name="steward-v4-CellarV2_2-AddAdaptorToCatalogue"></a>

### CellarV2_2.AddAdaptorToCatalogue
Allows the owner to add an adaptor to the Cellar&#39;s adaptor catalogue

Represents function `addAdaptorToCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2-AddPosition"></a>

### CellarV2_2.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2-AddPositionToCatalogue"></a>

### CellarV2_2.AddPositionToCatalogue
Allows the owner to add a position to the Cellar&#39;s position catalogue

Represents function `addPositionToCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_2-CachePriceRouter"></a>

### CellarV2_2.CachePriceRouter
Updates the cellar to use the latest price router in the registry.

Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| check_total_assets | [bool](#bool) |  | Whether to check the total assets of the cellar |
| allowable_range | [uint32](#uint32) |  | The allowable range of the cellar&#39;s total assets to deviate between old and new routers |
| expected_price_router | [string](#string) |  | The expected price router address |






<a name="steward-v4-CellarV2_2-CallOnAdaptor"></a>

### CellarV2_2.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v4-AdaptorCall) | repeated |  |






<a name="steward-v4-CellarV2_2-FunctionCall"></a>

### CellarV2_2.FunctionCall
The function you wish to execute on the target cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV2_2.AddPosition](#steward-v4-CellarV2_2-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2_2.CallOnAdaptor](#steward-v4-CellarV2_2-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2_2.RemovePosition](#steward-v4-CellarV2_2-RemovePosition) |  | Represents function `removePosition(uint256 index, bool inDebtArray)` |
| remove_adaptor_from_catalogue | [CellarV2_2.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_2-RemoveAdaptorFromCatalogue) |  | Represents function `removeAdaptorFromCatalogue(address adaptor)` |
| remove_position_from_catalogue | [CellarV2_2.RemovePositionFromCatalogue](#steward-v4-CellarV2_2-RemovePositionFromCatalogue) |  | Represents function `removePositionFromCatalogue(uint32 positionId)` |
| set_holding_position | [CellarV2_2.SetHoldingPosition](#steward-v4-CellarV2_2-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| set_strategist_payout_address | [CellarV2_2.SetStrategistPayoutAddress](#steward-v4-CellarV2_2-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| swap_positions | [CellarV2_2.SwapPositions](#steward-v4-CellarV2_2-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| add_adaptor_to_catalogue | [CellarV2_2.AddAdaptorToCatalogue](#steward-v4-CellarV2_2-AddAdaptorToCatalogue) |  | Represents function `addAdaptorToCatalogue(address adaptor)` |
| add_position_to_catalogue | [CellarV2_2.AddPositionToCatalogue](#steward-v4-CellarV2_2-AddPositionToCatalogue) |  | Represents function `addPositionToCatalogue(uint32 positionId)` |
| set_rebalance_deviation | [CellarV2_2.SetRebalanceDeviation](#steward-v4-CellarV2_2-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |
| set_share_lock_period | [CellarV2_2.SetShareLockPeriod](#steward-v4-CellarV2_2-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |
| set_strategist_platform_cut | [CellarV2_2.SetStrategistPlatformCut](#steward-v4-CellarV2_2-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(uint64 cut)` |
| initiate_shutdown | [CellarV2_2.InitiateShutdown](#steward-v4-CellarV2_2-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2_2.LiftShutdown](#steward-v4-CellarV2_2-LiftShutdown) |  | Represents function `liftShutdown()` |
| cache_price_router | [CellarV2_2.CachePriceRouter](#steward-v4-CellarV2_2-CachePriceRouter) |  | Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)` |






<a name="steward-v4-CellarV2_2-InitiateShutdown"></a>

### CellarV2_2.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2_2-LiftShutdown"></a>

### CellarV2_2.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






<a name="steward-v4-CellarV2_2-Multicall"></a>

### CellarV2_2.Multicall
Allows caller to call multiple functions in a single TX.

Represents function `multicall(bytes[] data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_calls | [CellarV2_2.FunctionCall](#steward-v4-CellarV2_2-FunctionCall) | repeated |  |






<a name="steward-v4-CellarV2_2-RemoveAdaptorFromCatalogue"></a>

### CellarV2_2.RemoveAdaptorFromCatalogue
Allows callers to remove adaptors from this cellar&#39;s catalogue

Represents function `removeAdaptorFromCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2-RemovePosition"></a>

### CellarV2_2.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2-RemovePositionFromCatalogue"></a>

### CellarV2_2.RemovePositionFromCatalogue
Allows caller to remove positions from this cellar&#39;s catalogue

Represents function `removePositionFromCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_2-SetHoldingPosition"></a>

### CellarV2_2.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v4-CellarV2_2-SetRebalanceDeviation"></a>

### CellarV2_2.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2-SetShareLockPeriod"></a>

### CellarV2_2.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2-SetStrategistPayoutAddress"></a>

### CellarV2_2.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2-SetStrategistPlatformCut"></a>

### CellarV2_2.SetStrategistPlatformCut
Allows strategist to set the platform cut for the cellar.

Represents function `setStrategistPlatformCut(uint64 cut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cut | [uint64](#uint64) |  | The new strategist platform cut |






<a name="steward-v4-CellarV2_2-SwapPositions"></a>

### CellarV2_2.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2Governance"></a>

### CellarV2_2Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_call | [CellarV2_2Governance.FunctionCall](#steward-v4-CellarV2_2Governance-FunctionCall) |  | Represents a single function call |
| multicall | [CellarV2_2Governance.Multicall](#steward-v4-CellarV2_2Governance-Multicall) |  | Represents multiple, ordered function calls |






<a name="steward-v4-CellarV2_2Governance-AddAdaptorToCatalogue"></a>

### CellarV2_2Governance.AddAdaptorToCatalogue
Allows the owner to add an adaptor to the Cellar&#39;s adaptor catalogue

Represents function `addAdaptorToCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2Governance-AddPosition"></a>

### CellarV2_2Governance.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2Governance-AddPositionToCatalogue"></a>

### CellarV2_2Governance.AddPositionToCatalogue
Allows the owner to add a position to the Cellar&#39;s position catalogue

Represents function `addPositionToCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_2Governance-CachePriceRouter"></a>

### CellarV2_2Governance.CachePriceRouter
Updates the cellar to use the latest price router in the registry.

Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| check_total_assets | [bool](#bool) |  | Whether to check the total assets of the cellar |
| allowable_range | [uint32](#uint32) |  | The allowable range of the cellar&#39;s total assets to deviate between old and new routers |
| expected_price_router | [string](#string) |  | The expected price router address |






<a name="steward-v4-CellarV2_2Governance-CallOnAdaptor"></a>

### CellarV2_2Governance.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v4-AdaptorCall) | repeated |  |






<a name="steward-v4-CellarV2_2Governance-ForcePositionOut"></a>

### CellarV2_2Governance.ForcePositionOut
Allows caller to force a position out of the cellar

Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  |  |
| position_id | [uint32](#uint32) |  |  |
| in_debt_array | [bool](#bool) |  |  |






<a name="steward-v4-CellarV2_2Governance-FunctionCall"></a>

### CellarV2_2Governance.FunctionCall
The function you wish to execute on the target cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_adaptor_to_catalogue | [CellarV2_2Governance.AddAdaptorToCatalogue](#steward-v4-CellarV2_2Governance-AddAdaptorToCatalogue) |  | Represents function `addAdaptorToCatalogue(address adaptor)` |
| add_position_to_catalogue | [CellarV2_2Governance.AddPositionToCatalogue](#steward-v4-CellarV2_2Governance-AddPositionToCatalogue) |  | Represents function `addPositionToCatalogue(uint32 positionId)` |
| set_rebalance_deviation | [CellarV2_2Governance.SetRebalanceDeviation](#steward-v4-CellarV2_2Governance-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |
| set_share_lock_period | [CellarV2_2Governance.SetShareLockPeriod](#steward-v4-CellarV2_2Governance-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |
| set_strategist_platform_cut | [CellarV2_2Governance.SetStrategistPlatformCut](#steward-v4-CellarV2_2Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(uint64 cut)` |
| initiate_shutdown | [CellarV2_2Governance.InitiateShutdown](#steward-v4-CellarV2_2Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2_2Governance.LiftShutdown](#steward-v4-CellarV2_2Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| force_position_out | [CellarV2_2Governance.ForcePositionOut](#steward-v4-CellarV2_2Governance-ForcePositionOut) |  | Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)` |
| toggle_ignore_pause | [CellarV2_2Governance.ToggleIgnorePause](#steward-v4-CellarV2_2Governance-ToggleIgnorePause) |  | Represents function `toggleIgnorePause(bool ignore)` |
| cache_price_router | [CellarV2_2Governance.CachePriceRouter](#steward-v4-CellarV2_2Governance-CachePriceRouter) |  | Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)` |
| add_position | [CellarV2_2Governance.AddPosition](#steward-v4-CellarV2_2Governance-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2_2Governance.CallOnAdaptor](#steward-v4-CellarV2_2Governance-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2_2Governance.RemovePosition](#steward-v4-CellarV2_2Governance-RemovePosition) |  | Represents function `removePosition(uint256 index, bool inDebtArray)` |
| remove_adaptor_from_catalogue | [CellarV2_2Governance.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_2Governance-RemoveAdaptorFromCatalogue) |  | Represents function `removeAdaptorFromCatalogue(address adaptor)` |
| remove_position_from_catalogue | [CellarV2_2Governance.RemovePositionFromCatalogue](#steward-v4-CellarV2_2Governance-RemovePositionFromCatalogue) |  | Represents function `removePositionFromCatalogue(uint32 positionId)` |
| set_holding_position | [CellarV2_2Governance.SetHoldingPosition](#steward-v4-CellarV2_2Governance-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| swap_positions | [CellarV2_2Governance.SwapPositions](#steward-v4-CellarV2_2Governance-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |






<a name="steward-v4-CellarV2_2Governance-InitiateShutdown"></a>

### CellarV2_2Governance.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2_2Governance-LiftShutdown"></a>

### CellarV2_2Governance.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






<a name="steward-v4-CellarV2_2Governance-Multicall"></a>

### CellarV2_2Governance.Multicall
Allows caller to call multiple functions in a single TX.

Represents function `multicall(bytes[] data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_calls | [CellarV2_2Governance.FunctionCall](#steward-v4-CellarV2_2Governance-FunctionCall) | repeated |  |






<a name="steward-v4-CellarV2_2Governance-RemoveAdaptorFromCatalogue"></a>

### CellarV2_2Governance.RemoveAdaptorFromCatalogue
Allows callers to remove adaptors from this cellar&#39;s catalogue

Represents function `removeAdaptorFromCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2Governance-RemovePosition"></a>

### CellarV2_2Governance.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2Governance-RemovePositionFromCatalogue"></a>

### CellarV2_2Governance.RemovePositionFromCatalogue
Allows caller to remove positions from this cellar&#39;s catalogue

Represents function `removePositionFromCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_2Governance-SetHoldingPosition"></a>

### CellarV2_2Governance.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v4-CellarV2_2Governance-SetRebalanceDeviation"></a>

### CellarV2_2Governance.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2Governance-SetShareLockPeriod"></a>

### CellarV2_2Governance.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v4-CellarV2_2Governance-SetStrategistPlatformCut"></a>

### CellarV2_2Governance.SetStrategistPlatformCut
Allows strategist to set the platform cut for the cellar.

Represents function `setStrategistPlatformCut(uint64 cut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cut | [uint64](#uint64) |  | The new strategist platform cut |






<a name="steward-v4-CellarV2_2Governance-SwapPositions"></a>

### CellarV2_2Governance.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_2Governance-ToggleIgnorePause"></a>

### CellarV2_2Governance.ToggleIgnorePause
Allows caller to toggle the ignorePause flag on the cellar

Represents function `toggleIgnorePause(bool ignore)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ignore | [bool](#bool) |  |  |






<a name="steward-v4-CellarV2_5"></a>

### CellarV2_5



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_call | [CellarV2_5.FunctionCall](#steward-v4-CellarV2_5-FunctionCall) |  | Represents a single function call |
| multicall | [CellarV2_5.Multicall](#steward-v4-CellarV2_5-Multicall) |  | Represents multiple, ordered function calls |






<a name="steward-v4-CellarV2_5-AddAdaptorToCatalogue"></a>

### CellarV2_5.AddAdaptorToCatalogue
Allows the owner to add an adaptor to the Cellar&#39;s adaptor catalogue

Represents function `addAdaptorToCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-AddPosition"></a>

### CellarV2_5.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_5-AddPositionToCatalogue"></a>

### CellarV2_5.AddPositionToCatalogue
Allows the owner to add a position to the Cellar&#39;s position catalogue

Represents function `addPositionToCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_5-CachePriceRouter"></a>

### CellarV2_5.CachePriceRouter
Updates the cellar to use the latest price router in the registry.

Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| check_total_assets | [bool](#bool) |  | Whether to check the total assets of the cellar |
| allowable_range | [uint32](#uint32) |  | The allowable range of the cellar&#39;s total assets to deviate between old and new routers |
| expected_price_router | [string](#string) |  | The expected price router address |






<a name="steward-v4-CellarV2_5-CallOnAdaptor"></a>

### CellarV2_5.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v4-AdaptorCall) | repeated |  |






<a name="steward-v4-CellarV2_5-DecreaseShareSupplyCap"></a>

### CellarV2_5.DecreaseShareSupplyCap
Allows strategist to decrease the share supply cap

Represents function `decreaseShareSupplyCap(uint192)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cap | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-DropAlternativeAssetData"></a>

### CellarV2_5.DropAlternativeAssetData
Allows the strategist to stop an alternative asset from being deposited.

Represents function `dropAlternativeAssetData(ERC20 _alternativeAsset)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alternative_asset | [string](#string) |  | The address of the alternative asset |






<a name="steward-v4-CellarV2_5-FunctionCall"></a>

### CellarV2_5.FunctionCall
The function you wish to execute on the target cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV2_5.AddPosition](#steward-v4-CellarV2_5-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2_5.CallOnAdaptor](#steward-v4-CellarV2_5-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2_5.RemovePosition](#steward-v4-CellarV2_5-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV2_5.SetHoldingPosition](#steward-v4-CellarV2_5-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| set_strategist_payout_address | [CellarV2_5.SetStrategistPayoutAddress](#steward-v4-CellarV2_5-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| swap_positions | [CellarV2_5.SwapPositions](#steward-v4-CellarV2_5-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_share_lock_period | [CellarV2_5.SetShareLockPeriod](#steward-v4-CellarV2_5-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |
| initiate_shutdown | [CellarV2_5.InitiateShutdown](#steward-v4-CellarV2_5-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2_5.LiftShutdown](#steward-v4-CellarV2_5-LiftShutdown) |  | Represents function `liftShutdown()` |
| remove_adaptor_from_catalogue | [CellarV2_5.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_5-RemoveAdaptorFromCatalogue) |  | Represents function `removeAdaptorFromCatalogue(address adaptor)` |
| remove_position_from_catalogue | [CellarV2_5.RemovePositionFromCatalogue](#steward-v4-CellarV2_5-RemovePositionFromCatalogue) |  | Represents function `removePositionFromCatalogue(uint32 positionId)` |
| decrease_share_supply_cap | [CellarV2_5.DecreaseShareSupplyCap](#steward-v4-CellarV2_5-DecreaseShareSupplyCap) |  | Represents function `decreaseShareSupplyCap(uint192) |
| set_alternative_asset_data | [CellarV2_5.SetAlternativeAssetData](#steward-v4-CellarV2_5-SetAlternativeAssetData) |  | Represents function `setAlternativeAssetData(ERC20 _alternativeAsset, uint32 _alternativeHoldingPosition, uint32 _alternativeAssetFee)` |
| drop_alternative_asset_data | [CellarV2_5.DropAlternativeAssetData](#steward-v4-CellarV2_5-DropAlternativeAssetData) |  | Represents function `setDropAlternativeAssetData(ERC20 _alternativeAsset)` |
| add_adaptor_to_catalogue | [CellarV2_5.AddAdaptorToCatalogue](#steward-v4-CellarV2_5-AddAdaptorToCatalogue) |  | Represents function `addAdaptorToCatalogue(address adaptor)` |
| add_position_to_catalogue | [CellarV2_5.AddPositionToCatalogue](#steward-v4-CellarV2_5-AddPositionToCatalogue) |  | Represents function `addPositionToCatalogue(uint32 positionId)` |
| set_rebalance_deviation | [CellarV2_5.SetRebalanceDeviation](#steward-v4-CellarV2_5-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |
| set_strategist_platform_cut | [CellarV2_5.SetStrategistPlatformCut](#steward-v4-CellarV2_5-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(uint64 cut)` |
| set_share_price_oracle | [CellarV2_5.SetSharePriceOracle](#steward-v4-CellarV2_5-SetSharePriceOracle) |  | Represents function `setSharePriceOracle(uint256 _registryId, ERC4626SharePriceOracle _sharePriceOracle)` |
| increase_share_supply_cap | [CellarV2_5.IncreaseShareSupplyCap](#steward-v4-CellarV2_5-IncreaseShareSupplyCap) |  | Represents function `increaseShareSupplyCap(uint192 _newShareSupplyCap)` |
| cache_price_router | [CellarV2_5.CachePriceRouter](#steward-v4-CellarV2_5-CachePriceRouter) |  | Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)` |






<a name="steward-v4-CellarV2_5-IncreaseShareSupplyCap"></a>

### CellarV2_5.IncreaseShareSupplyCap
Allows the caller to increase the share supply cap

Represents function `increaseShareSupplyCap(uint192 _newShareSupplyCap)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cap | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-InitiateShutdown"></a>

### CellarV2_5.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2_5-LiftShutdown"></a>

### CellarV2_5.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






<a name="steward-v4-CellarV2_5-Multicall"></a>

### CellarV2_5.Multicall
Allows caller to call multiple functions in a single TX.

Represents function `multicall(bytes[] data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_calls | [CellarV2_5.FunctionCall](#steward-v4-CellarV2_5-FunctionCall) | repeated |  |






<a name="steward-v4-CellarV2_5-RemoveAdaptorFromCatalogue"></a>

### CellarV2_5.RemoveAdaptorFromCatalogue
Allows callers to remove adaptors from this cellar&#39;s catalogue

Represents function `removeAdaptorFromCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-RemovePosition"></a>

### CellarV2_5.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v4-CellarV2_5-RemovePositionFromCatalogue"></a>

### CellarV2_5.RemovePositionFromCatalogue
Allows caller to remove positions from this cellar&#39;s catalogue

Represents function `removePositionFromCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_5-SetAlternativeAssetData"></a>

### CellarV2_5.SetAlternativeAssetData
Allows the strategist to add, or update an existing alternative asset deposit.

Represents function `setAlternativeAssetData(ERC20 _alternativeAsset, uint32 _alternativeHoldingPosition, uint32 _alternativeAssetFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| alternative_asset | [string](#string) |  | The address of the alternative asset |
| alternative_holding_position | [uint32](#uint32) |  | The holding position to direct alternative asset deposits to |
| alternative_asset_fee | [uint32](#uint32) |  | The fee to charge for depositing this alternative asset |






<a name="steward-v4-CellarV2_5-SetHoldingPosition"></a>

### CellarV2_5.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v4-CellarV2_5-SetRebalanceDeviation"></a>

### CellarV2_5.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-SetShareLockPeriod"></a>

### CellarV2_5.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-SetSharePriceOracle"></a>

### CellarV2_5.SetSharePriceOracle
Allows the caller to set the share price oracle contract

Represents function `setSharePriceOracle(uint256 _registryId, ERC4626SharePriceOracle _sharePriceOracle)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| registry_id | [string](#string) |  | The oracle registry ID |
| share_price_oracle | [string](#string) |  | The oracle contract address |






<a name="steward-v4-CellarV2_5-SetStrategistPayoutAddress"></a>

### CellarV2_5.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5-SetStrategistPlatformCut"></a>

### CellarV2_5.SetStrategistPlatformCut
Allows strategist to set the platform cut for the cellar.

Represents function `setStrategistPlatformCut(uint64 cut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cut | [uint64](#uint64) |  | The new strategist platform cut |






<a name="steward-v4-CellarV2_5-SwapPositions"></a>

### CellarV2_5.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_5Governance"></a>

### CellarV2_5Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_call | [CellarV2_5Governance.FunctionCall](#steward-v4-CellarV2_5Governance-FunctionCall) |  | Represents a single function call |
| multicall | [CellarV2_5Governance.Multicall](#steward-v4-CellarV2_5Governance-Multicall) |  | Represents multiple, ordered function calls |






<a name="steward-v4-CellarV2_5Governance-AddAdaptorToCatalogue"></a>

### CellarV2_5Governance.AddAdaptorToCatalogue
Allows the owner to add an adaptor to the Cellar&#39;s adaptor catalogue

Represents function `addAdaptorToCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5Governance-AddPosition"></a>

### CellarV2_5Governance.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v4-CellarV2_5Governance-AddPositionToCatalogue"></a>

### CellarV2_5Governance.AddPositionToCatalogue
Allows the owner to add a position to the Cellar&#39;s position catalogue

Represents function `addPositionToCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_5Governance-CachePriceRouter"></a>

### CellarV2_5Governance.CachePriceRouter
Updates the cellar to use the latest price router in the registry.

Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| check_total_assets | [bool](#bool) |  | Whether to check the total assets of the cellar |
| allowable_range | [uint32](#uint32) |  | The allowable range of the cellar&#39;s total assets to deviate between old and new routers |
| expected_price_router | [string](#string) |  | The expected price router address |






<a name="steward-v4-CellarV2_5Governance-CallOnAdaptor"></a>

### CellarV2_5Governance.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v4-AdaptorCall) | repeated |  |






<a name="steward-v4-CellarV2_5Governance-DecreaseShareSupplyCap"></a>

### CellarV2_5Governance.DecreaseShareSupplyCap
Allows strategist to decrease the share supply cap

Represents function `decreaseShareSupplyCap(uint192)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cap | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5Governance-ForcePositionOut"></a>

### CellarV2_5Governance.ForcePositionOut
Allows caller to force a position out of the cellar

Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  |  |
| position_id | [uint32](#uint32) |  |  |
| in_debt_array | [bool](#bool) |  |  |






<a name="steward-v4-CellarV2_5Governance-FunctionCall"></a>

### CellarV2_5Governance.FunctionCall
The function you wish to execute on the target cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_adaptor_to_catalogue | [CellarV2_5Governance.AddAdaptorToCatalogue](#steward-v4-CellarV2_5Governance-AddAdaptorToCatalogue) |  | Represents function `addAdaptorToCatalogue(address adaptor)` |
| add_position_to_catalogue | [CellarV2_5Governance.AddPositionToCatalogue](#steward-v4-CellarV2_5Governance-AddPositionToCatalogue) |  | Represents function `addPositionToCatalogue(uint32 positionId)` |
| set_rebalance_deviation | [CellarV2_5Governance.SetRebalanceDeviation](#steward-v4-CellarV2_5Governance-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint256)` |
| set_strategist_platform_cut | [CellarV2_5Governance.SetStrategistPlatformCut](#steward-v4-CellarV2_5Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(uint64 cut)` |
| force_position_out | [CellarV2_5Governance.ForcePositionOut](#steward-v4-CellarV2_5Governance-ForcePositionOut) |  | Represents function `forcePositionOut(uint32 index, uint32 positionId, bool inDebtArray)` |
| toggle_ignore_pause | [CellarV2_5Governance.ToggleIgnorePause](#steward-v4-CellarV2_5Governance-ToggleIgnorePause) |  | Represents function `toggleIgnorePause()` |
| set_share_price_oracle | [CellarV2_5Governance.SetSharePriceOracle](#steward-v4-CellarV2_5Governance-SetSharePriceOracle) |  | Represents function `setSharePriceOracle(uint256 _registryId, ERC4626SharePriceOracle _sharePriceOracle)` |
| increase_share_supply_cap | [CellarV2_5Governance.IncreaseShareSupplyCap](#steward-v4-CellarV2_5Governance-IncreaseShareSupplyCap) |  | Represents function `increaseShareSupplyCap(uint192 _newShareSupplyCap)` |
| set_automation_actions | [CellarV2_5Governance.SetAutomationActions](#steward-v4-CellarV2_5Governance-SetAutomationActions) |  | Represents function `setAutomatiionActions(uint256 _registryId, address _expectedAutomationActions)` |
| cache_price_router | [CellarV2_5Governance.CachePriceRouter](#steward-v4-CellarV2_5Governance-CachePriceRouter) |  | Represents function `cachePriceRouter(bool checkTotalAssets, uint16 allowableRange, address expectedPriceRouter)` |
| initiate_shutdown | [CellarV2_5Governance.InitiateShutdown](#steward-v4-CellarV2_5Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2_5Governance.LiftShutdown](#steward-v4-CellarV2_5Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| remove_adaptor_from_catalogue | [CellarV2_5Governance.RemoveAdaptorFromCatalogue](#steward-v4-CellarV2_5Governance-RemoveAdaptorFromCatalogue) |  | Represents function `removeAdaptorFromCatalogue(address adaptor)` |
| remove_position_from_catalogue | [CellarV2_5Governance.RemovePositionFromCatalogue](#steward-v4-CellarV2_5Governance-RemovePositionFromCatalogue) |  | Represents function `removePositionFromCatalogue(uint32 positionId)` |
| decrease_share_supply_cap | [CellarV2_5Governance.DecreaseShareSupplyCap](#steward-v4-CellarV2_5Governance-DecreaseShareSupplyCap) |  | Represents function `decreaseShareSupplyCap(uint192) |
| set_holding_position | [CellarV2_5Governance.SetHoldingPosition](#steward-v4-CellarV2_5Governance-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| add_position | [CellarV2_5Governance.AddPosition](#steward-v4-CellarV2_5Governance-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2_5Governance.CallOnAdaptor](#steward-v4-CellarV2_5Governance-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2_5Governance.RemovePosition](#steward-v4-CellarV2_5Governance-RemovePosition) |  | Represents function `removePosition(uint256 index)` |






<a name="steward-v4-CellarV2_5Governance-IncreaseShareSupplyCap"></a>

### CellarV2_5Governance.IncreaseShareSupplyCap
Allows the caller to increase the share supply cap

Represents function `increaseShareSupplyCap(uint192 _newShareSupplyCap)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cap | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5Governance-InitiateShutdown"></a>

### CellarV2_5Governance.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v4-CellarV2_5Governance-LiftShutdown"></a>

### CellarV2_5Governance.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






<a name="steward-v4-CellarV2_5Governance-Multicall"></a>

### CellarV2_5Governance.Multicall
Allows caller to call multiple functions in a single TX.

Represents function `multicall(bytes[] data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_calls | [CellarV2_5Governance.FunctionCall](#steward-v4-CellarV2_5Governance-FunctionCall) | repeated |  |






<a name="steward-v4-CellarV2_5Governance-RemoveAdaptorFromCatalogue"></a>

### CellarV2_5Governance.RemoveAdaptorFromCatalogue
Allows callers to remove adaptors from this cellar&#39;s catalogue

Represents function `removeAdaptorFromCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5Governance-RemovePosition"></a>

### CellarV2_5Governance.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v4-CellarV2_5Governance-RemovePositionFromCatalogue"></a>

### CellarV2_5Governance.RemovePositionFromCatalogue
Allows caller to remove positions from this cellar&#39;s catalogue

Represents function `removePositionFromCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v4-CellarV2_5Governance-SetAutomationActions"></a>

### CellarV2_5Governance.SetAutomationActions
Allows caller to set automation actions

Represents function `setAutomatiionActions(uint256 _registryId, address _expectedAutomationActions)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| registry_id | [string](#string) |  | The oracle registry ID |
| expected_automation_actions | [string](#string) |  | The automation actions contract address |






<a name="steward-v4-CellarV2_5Governance-SetHoldingPosition"></a>

### CellarV2_5Governance.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v4-CellarV2_5Governance-SetRebalanceDeviation"></a>

### CellarV2_5Governance.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v4-CellarV2_5Governance-SetSharePriceOracle"></a>

### CellarV2_5Governance.SetSharePriceOracle
Allows the caller to set the share price oracle contract

Represents function `setSharePriceOracle(uint256 _registryId, ERC4626SharePriceOracle _sharePriceOracle)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| registry_id | [string](#string) |  | The oracle registry ID |
| share_price_oracle | [string](#string) |  | The oracle contract address |






<a name="steward-v4-CellarV2_5Governance-SetStrategistPlatformCut"></a>

### CellarV2_5Governance.SetStrategistPlatformCut
Allows strategist to set the platform cut for the cellar.

Represents function `setStrategistPlatformCut(uint64 cut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cut | [uint64](#uint64) |  | The new strategist platform cut |






<a name="steward-v4-CellarV2_5Governance-ToggleIgnorePause"></a>

### CellarV2_5Governance.ToggleIgnorePause
Allows caller to toggle the ignorePause flag on the cellar

Represents function `toggleIgnorePause()`





 

 

 

 



<a name="collateral_f_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## collateral_f_token.proto



<a name="steward-v4-CollateralFTokenAdaptorV1"></a>

### CollateralFTokenAdaptorV1
Represents call data for the Frax Collateral F Token adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| add_collateral | [CollateralFTokenAdaptorV1.AddCollateral](#steward-v4-CollateralFTokenAdaptorV1-AddCollateral) |  | Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)` |
| remove_collateral | [CollateralFTokenAdaptorV1.RemoveCollateral](#steward-v4-CollateralFTokenAdaptorV1-RemoveCollateral) |  | Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)` |






<a name="steward-v4-CollateralFTokenAdaptorV1-AddCollateral"></a>

### CollateralFTokenAdaptorV1.AddCollateral
Allows strategists to add collateral to the respective cellar position on FraxLend, enabling borrowing.

Represents function `addCollateral(IFToken _fraxlendPair, uint256 _collateralToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fraxlend_pair | [string](#string) |  | The FraxLend pair to add collateral to. |
| collateral_to_deposit | [string](#string) |  | The amount of collateral to add to the cellar position. |






<a name="steward-v4-CollateralFTokenAdaptorV1-RemoveCollateral"></a>

### CollateralFTokenAdaptorV1.RemoveCollateral
Allows strategists to remove collateral from the respective cellar position on FraxLend.

Represents function `removeCollateral(uint256 _collateralAmount, IFToken _fraxlendPair)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| collateral_amount | [string](#string) |  | The amount of collateral to remove from the cellar position. |
| fraxlend_pair | [string](#string) |  | The FraxLend pair to remove collateral from. |






<a name="steward-v4-CollateralFTokenAdaptorV1Calls"></a>

### CollateralFTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [CollateralFTokenAdaptorV1](#steward-v4-CollateralFTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="common-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## common.proto



<a name="steward-v4-MarketParams"></a>

### MarketParams
Represents parameters for a Morpho Blue market


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| loan_token | [string](#string) |  | The address of the loan token |
| collateral_token | [string](#string) |  | The address of the collateral token |
| oracle | [string](#string) |  | The address of the oracle |
| irm | [string](#string) |  | The address of the interest rate model |
| lltv | [string](#string) |  | The loan-to-value ratio |






<a name="steward-v4-OracleSwapParams"></a>

### OracleSwapParams
Represents swap params for BaseAdaptor.oracleSwap()


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| univ2_params | [UniV2OracleSwapParams](#steward-v4-UniV2OracleSwapParams) |  |  |
| univ3_params | [UniV3OracleSwapParams](#steward-v4-UniV3OracleSwapParams) |  |  |






<a name="steward-v4-SwapParams"></a>

### SwapParams
Represents swap parameters for an exchange


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| univ2_params | [UniV2SwapParams](#steward-v4-UniV2SwapParams) |  | Params for a Uniswap V2 swap |
| univ3_params | [UniV3SwapParams](#steward-v4-UniV3SwapParams) |  | Params for a Uniswap V3 swap |






<a name="steward-v4-UniV2OracleSwapParams"></a>

### UniV2OracleSwapParams
Represents oracle swap parameters for UniswapV2


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |






<a name="steward-v4-UniV2SwapParams"></a>

### UniV2SwapParams
Represents swap parameters for UniswapV2


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| amount | [string](#string) |  | Amount of the first asset in the path to swap |
| amount_out_min | [string](#string) |  | The minimum amount of the last asset in the path to receive |






<a name="steward-v4-UniV3OracleSwapParams"></a>

### UniV3OracleSwapParams
Represents oracle swap parameters for UniswapV3


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| pool_fees | [uint32](#uint32) | repeated | Array of pool fees dictating what swap pools to use |






<a name="steward-v4-UniV3SwapParams"></a>

### UniV3SwapParams
Represents swap parameters for UniswapV3


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| pool_fees | [uint32](#uint32) | repeated | Array of pool fees dictating what swap pools to use |
| amount | [string](#string) |  | Amount of the first asset in the path to swap |
| amount_out_min | [string](#string) |  | The minimum amount of the last asset in the path to receive |





 


<a name="steward-v4-Exchange"></a>

### Exchange
Exchange selector

| Name | Number | Description |
| ---- | ------ | ----------- |
| EXCHANGE_UNSPECIFIED | 0 |  |
| EXCHANGE_UNIV2 | 1 | Represents Uniswap V2 |
| EXCHANGE_UNIV3 | 2 | Represents Uniswap V3 |


 

 

 



<a name="convex_curve-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## convex_curve.proto



<a name="steward-v4-ConvexCurveAdaptorV1"></a>

### ConvexCurveAdaptorV1
Represents call data for the Convex Curve adaptor V1


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_lpt_in_convex_and_stake | [ConvexCurveAdaptorV1.DepositLPTInConvexAndStake](#steward-v4-ConvexCurveAdaptorV1-DepositLPTInConvexAndStake) |  | Represents function `depositLPTInConvexAndStake(uint256 _pid, address baseRewardPool, ERC20 _lpt, CurvePool _pool, bytes4 _selector, uint256 _amount)` |
| withdraw_from_base_reward_pool_as_lpt | [ConvexCurveAdaptorV1.WithdrawFromBaseRewardPoolAsLPT](#steward-v4-ConvexCurveAdaptorV1-WithdrawFromBaseRewardPoolAsLPT) |  | Represents function `withdrawFromBaseRewardPoolAsLPTaddress(_baseRewardPool, uint256 _amount, bool _claim)` |
| get_rewards | [ConvexCurveAdaptorV1.GetRewards](#steward-v4-ConvexCurveAdaptorV1-GetRewards) |  | Represents function `getRewards(address _baseRewardPool, bool _claimExtras)` |






<a name="steward-v4-ConvexCurveAdaptorV1-DepositLPTInConvexAndStake"></a>

### ConvexCurveAdaptorV1.DepositLPTInConvexAndStake
Allows strategists to deposit and stake LPTs into Convex markets via the respective Convex market Booster contract

Represents function `depositLPTInConvexAndStake(uint256 _pid, address baseRewardPool, ERC20 _lpt, CurvePool _pool, bytes4 _selector, uint256 _amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pid | [string](#string) |  |  |
| base_reward_pool | [string](#string) |  |  |
| lpt | [string](#string) |  |  |
| pool | [string](#string) |  |  |
| selector | [string](#string) |  |  |
| amount_to_deposit | [string](#string) |  |  |






<a name="steward-v4-ConvexCurveAdaptorV1-GetRewards"></a>

### ConvexCurveAdaptorV1.GetRewards
Allows strategists to get rewards for an Convex Booster without withdrawing/unwrapping from Convex market

Represents function `getRewards(address _baseRewardPool, bool _claimExtras)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| base_reward_pool | [string](#string) |  |  |
| claim_extras | [bool](#bool) |  |  |






<a name="steward-v4-ConvexCurveAdaptorV1-WithdrawFromBaseRewardPoolAsLPT"></a>

### ConvexCurveAdaptorV1.WithdrawFromBaseRewardPoolAsLPT
Allows strategists to withdraw from Convex markets via Booster contract w/ or w/o claiming rewards

Represents function `withdrawFromBaseRewardPoolAsLPTaddress(_baseRewardPool, uint256 _amount, bool _claim)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| base_reward_pool | [string](#string) |  |  |
| amount_to_withdraw | [string](#string) |  |  |
| claim | [bool](#bool) |  |  |






<a name="steward-v4-ConvexCurveAdaptorV1Calls"></a>

### ConvexCurveAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [ConvexCurveAdaptorV1](#steward-v4-ConvexCurveAdaptorV1) | repeated |  |





 

 

 

 



<a name="curve-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## curve.proto



<a name="steward-v4-CurveAdaptorV1"></a>

### CurveAdaptorV1
Represents call data for the Curve adaptor V1


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| add_liquidity | [CurveAdaptorV1.AddLiquidity](#steward-v4-CurveAdaptorV1-AddLiquidity) |  | Represents function `addLiquidity(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, CurveGauge gauge, bytes4 selector)` |
| add_liquidity_eth | [CurveAdaptorV1.AddLiquidityETH](#steward-v4-CurveAdaptorV1-AddLiquidityETH) |  | Represents function `addLiquidityETH(address pool, ERC20 lpToken, uint256[] orderedMinimumUnderlyingTokenAmountsOut, uint256 minLPAmount, bool useUnderlying, CurveGauge gauge, bytes4 selector)` |
| remove_liquidity | [CurveAdaptorV1.RemoveLiquidity](#steward-v4-CurveAdaptorV1-RemoveLiquidity) |  | Represents function `removeLiquidity(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, CurveGauge gauge, bytes4 selector)` |
| remove_liquidity_eth | [CurveAdaptorV1.RemoveLiquidityETH](#steward-v4-CurveAdaptorV1-RemoveLiquidityETH) |  | Represents function `removeLiquidityETH(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, bool useUnderlying, CurveGauge gauge, bytes4 selector)` |
| stake_in_gauge | [CurveAdaptorV1.StakeInGauge](#steward-v4-CurveAdaptorV1-StakeInGauge) |  | Represents function `stakeInGauge(ERC20 lpToken, CurveGauge gauge, uint256 amount, CurvePool pool, bytes4 selector)` |
| unstake_from_gauge | [CurveAdaptorV1.UnstakeFromGauge](#steward-v4-CurveAdaptorV1-UnstakeFromGauge) |  | Represents function `unstakeFromGauge(CurveGauge gauge, uint256 amount)` |
| claim_rewards | [CurveAdaptorV1.ClaimRewards](#steward-v4-CurveAdaptorV1-ClaimRewards) |  | Represents function `claimRewards(CurveGauge gauge)` |






<a name="steward-v4-CurveAdaptorV1-AddLiquidity"></a>

### CurveAdaptorV1.AddLiquidity
Allows strategist to add liquidity to Curve pairs that do NOT use the native asset.

Represents function `addLiquidity(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, CurveGauge gauge, bytes4 selector)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pool | [string](#string) |  | Address of the Curve pool |
| lp_token | [string](#string) |  | Address of the LP token |
| ordered_underlying_token_amounts | [string](#string) | repeated | Minimum amount of each underlying token to receive |
| min_lp_amount | [string](#string) |  | Minimum amount of LP tokens to receive |
| gauge | [string](#string) |  | Address of the Curve gauge |
| selector | [string](#string) |  | Selector of the function to call |






<a name="steward-v4-CurveAdaptorV1-AddLiquidityETH"></a>

### CurveAdaptorV1.AddLiquidityETH
Allows strategist to add liquidity to Curve pairs that use the native asset.

Represents function `addLiquidityETH(address pool, ERC20 lpToken, uint256[] orderedUnderlyingTokenAmounts, uint256 minLPAmount, bool useUnderlying, CurveGauge gauge, bytes4 selector)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pool | [string](#string) |  | Address of the Curve pool |
| lp_token | [string](#string) |  | Address of the LP token |
| ordered_underlying_token_amounts | [string](#string) | repeated | Minimum amount of each underlying token to receive |
| min_lp_amount | [string](#string) |  | Minimum amount of LP tokens to receive |
| use_underlying | [bool](#bool) |  | Whether to use the underlying asset or the wrapped asset |
| gauge | [string](#string) |  | Address of the Curve gauge |
| selector | [string](#string) |  | Selector of the function to call |






<a name="steward-v4-CurveAdaptorV1-ClaimRewards"></a>

### CurveAdaptorV1.ClaimRewards
Allows strategist to claim rewards from a gauge.

Represents function `claimRewards(CurveGauge gauge)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gauge | [string](#string) |  | Address of the Curve gauge |






<a name="steward-v4-CurveAdaptorV1-RemoveLiquidity"></a>

### CurveAdaptorV1.RemoveLiquidity
Allows strategist to remove liquidity from Curve pairs that do NOT use the native asset.

Represents function `removeLiquidity(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, CurveGauge gauge, bytes4 selector)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pool | [string](#string) |  | Address of the Curve pool |
| lp_token | [string](#string) |  | Address of the LP token |
| lp_token_amount | [string](#string) |  | Amount of LP tokens to remove |
| ordered_minimum_underlying_token_amounts_out | [string](#string) | repeated | Minimum amount of each underlying token to receive |
| gauge | [string](#string) |  | Address of the Curve gauge |
| selector | [string](#string) |  | Selector of the function to call |






<a name="steward-v4-CurveAdaptorV1-RemoveLiquidityETH"></a>

### CurveAdaptorV1.RemoveLiquidityETH
Allows strategist to remove liquidity from Curve pairs that use the native asset.

Represents function `removeLiquidityETH(address pool, ERC20 lpToken, uint256 lpTokenAmount, uint256[] orderedMinimumUnderlyingTokenAmountsOut, bool useUnderlying, CurveGauge gauge, bytes4 selector)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| pool | [string](#string) |  | Address of the Curve pool |
| lp_token | [string](#string) |  | Address of the LP token |
| lp_token_amount | [string](#string) |  | Amount of LP tokens to remove |
| ordered_minimum_underlying_token_amounts_out | [string](#string) | repeated | Minimum amount of each underlying token to receive |
| use_underlying | [bool](#bool) |  | Whether to use the underlying asset or the wrapped asset |
| gauge | [string](#string) |  | Address of the Curve gauge |
| selector | [string](#string) |  | Selector of the function to call |






<a name="steward-v4-CurveAdaptorV1-StakeInGauge"></a>

### CurveAdaptorV1.StakeInGauge
Allows strategist to stake Curve LP tokens in their gauge.

Represents function `stakeInGauge(ERC20 lpToken, CurveGauge gauge, uint256 amount, CurvePool pool, bytes4 selector)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| lp_token | [string](#string) |  | Address of the LP token |
| gauge | [string](#string) |  | Address of the Curve gauge |
| amount | [string](#string) |  | Amount of LP tokens to stake |
| pool | [string](#string) |  | Address of the Curve pool |
| selector | [string](#string) |  | Selector of the function to call |






<a name="steward-v4-CurveAdaptorV1-UnstakeFromGauge"></a>

### CurveAdaptorV1.UnstakeFromGauge
Allows strategist to unstake Curve LP tokens from their gauge.

Represents function `unstakeFromGauge(CurveGauge gauge, uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| gauge | [string](#string) |  | Address of the Curve gauge |
| amount | [string](#string) |  | Amount of LP tokens to unstake |






<a name="steward-v4-CurveAdaptorV1Calls"></a>

### CurveAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [CurveAdaptorV1](#steward-v4-CurveAdaptorV1) | repeated |  |





 

 

 

 



<a name="debt_f_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## debt_f_token.proto



<a name="steward-v4-DebtFTokenAdaptorV1"></a>

### DebtFTokenAdaptorV1
Represents call data for the Frax adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_fraxlend | [DebtFTokenAdaptorV1.BorrowFromFraxlend](#steward-v4-DebtFTokenAdaptorV1-BorrowFromFraxlend) |  | Represents function `borrowFromFraxlend(IFToken fraxlendPair, uint256 amountToBorrow)` |
| repay_fraxlend_debt | [DebtFTokenAdaptorV1.RepayFraxlendDebt](#steward-v4-DebtFTokenAdaptorV1-RepayFraxlendDebt) |  | Represents function `repayFraxlendDebt(IFToken _fraxlendPair, uint256 _debtTokenRepayAmount)` |
| call_add_interest | [DebtFTokenAdaptorV1.CallAddInterest](#steward-v4-DebtFTokenAdaptorV1-CallAddInterest) |  | Represents function `callAddInterest(IFToken _fraxlendPair)` |






<a name="steward-v4-DebtFTokenAdaptorV1-BorrowFromFraxlend"></a>

### DebtFTokenAdaptorV1.BorrowFromFraxlend
Allows a strategist to borrow assets from Fraxlend 

Represents `function borrowFromFraxlend(IFToken fraxlendPair, uint256 amountToBorrow)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fraxlend_pair | [string](#string) |  | The address of the Frax Pair to borrow from. |
| amount_to_borrow | [string](#string) |  | The amount of the asset to borrow. |






<a name="steward-v4-DebtFTokenAdaptorV1-CallAddInterest"></a>

### DebtFTokenAdaptorV1.CallAddInterest
Allows a strategist to call `addInterest` on a Frax Pair they are using

Represents `function callAddInterest(IFToken _fraxlendPair)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fraxlend_pair | [string](#string) |  | The address of the pair to call addInterest on. |






<a name="steward-v4-DebtFTokenAdaptorV1-RepayFraxlendDebt"></a>

### DebtFTokenAdaptorV1.RepayFraxlendDebt
Allows strategists to repay loan debt on Fraxlend Pair. 
Make sure to call addInterest() beforehand to ensure we are repaying what is required.    

Represents `function repayFraxlendDebt(IFToken _fraxlendPair, uint256 _debtTokenRepayAmount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| fraxlend_pair | [string](#string) |  | The address of the Frax Pair to repay debt on. |
| debt_token_repay_amount | [string](#string) |  | The amount of the debt token to repay. |






<a name="steward-v4-DebtFTokenAdaptorV1Calls"></a>

### DebtFTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [DebtFTokenAdaptorV1](#steward-v4-DebtFTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## debt_token.proto



<a name="steward-v4-AaveDebtTokenAdaptorV1"></a>

### AaveDebtTokenAdaptorV1
Represents call data for the Aave Debt Token adaptor V1, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v4-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v4-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveDebtTokenAdaptorV1.BorrowFromAave](#steward-v4-AaveDebtTokenAdaptorV1-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveDebtTokenAdaptorV1.RepayAaveDebt](#steward-v4-AaveDebtTokenAdaptorV1-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |
| swap_and_repay | [AaveDebtTokenAdaptorV1.SwapAndRepay](#steward-v4-AaveDebtTokenAdaptorV1-SwapAndRepay) |  | Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |






<a name="steward-v4-AaveDebtTokenAdaptorV1-BorrowFromAave"></a>

### AaveDebtTokenAdaptorV1.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v4-AaveDebtTokenAdaptorV1-RepayAaveDebt"></a>

### AaveDebtTokenAdaptorV1.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v4-AaveDebtTokenAdaptorV1-SwapAndRepay"></a>

### AaveDebtTokenAdaptorV1.SwapAndRepay
Allows strategists to swap assets and repay loans in one call.

Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  | The address of the token to swap from |
| token_to_repay | [string](#string) |  | The address of the token to swap to and repay with |
| amount_in | [string](#string) |  | The amount to swap |
| exchange | [Exchange](#steward-v4-Exchange) |  | The exchange to make the swap on |
| params | [SwapParams](#steward-v4-SwapParams) |  | The parameters for the swap |






<a name="steward-v4-AaveDebtTokenAdaptorV1Calls"></a>

### AaveDebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveDebtTokenAdaptorV1](#steward-v4-AaveDebtTokenAdaptorV1) | repeated |  |






<a name="steward-v4-AaveDebtTokenAdaptorV2"></a>

### AaveDebtTokenAdaptorV2
Represents call data for the Aave Debt Token adaptor V2, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveDebtTokenAdaptorV2.BorrowFromAave](#steward-v4-AaveDebtTokenAdaptorV2-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveDebtTokenAdaptorV2.RepayAaveDebt](#steward-v4-AaveDebtTokenAdaptorV2-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |






<a name="steward-v4-AaveDebtTokenAdaptorV2-BorrowFromAave"></a>

### AaveDebtTokenAdaptorV2.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v4-AaveDebtTokenAdaptorV2-RepayAaveDebt"></a>

### AaveDebtTokenAdaptorV2.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v4-AaveDebtTokenAdaptorV2Calls"></a>

### AaveDebtTokenAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveDebtTokenAdaptorV2](#steward-v4-AaveDebtTokenAdaptorV2) | repeated |  |





 

 

 

 



<a name="erc4626-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## erc4626.proto



<a name="steward-v4-ERC4626AdaptorV1"></a>

### ERC4626AdaptorV1
Represents call data for the ERC4626 adaptor V1


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_vault | [ERC4626AdaptorV1.DepositToVault](#steward-v4-ERC4626AdaptorV1-DepositToVault) |  | Represents function `depositToVault(ERC4626 erc4626Vault, uint256 assets)` |
| withdraw_from_vault | [ERC4626AdaptorV1.WithdrawFromVault](#steward-v4-ERC4626AdaptorV1-WithdrawFromVault) |  | Represents function `withdrawFromVault(ERC4626 erc4626Vault, uint256 assets)` |






<a name="steward-v4-ERC4626AdaptorV1-DepositToVault"></a>

### ERC4626AdaptorV1.DepositToVault
Allows strategists to deposit into ERC4626 positions.

Represents function `depositToVault(ERC4626 erc4626Vault, uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| erc4626_vault | [string](#string) |  | The address of the ERC4626 vault |
| assets | [string](#string) |  | The amount of assets to deposit |






<a name="steward-v4-ERC4626AdaptorV1-WithdrawFromVault"></a>

### ERC4626AdaptorV1.WithdrawFromVault
Allows strategists to withdraw from ERC4626 positions.

Represents function `withdrawFromVault(ERC4626 erc4626Vault, uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| erc4626_vault | [string](#string) |  | The address of the ERC4626 vault |
| assets | [string](#string) |  | The amount of assets to withdraw |






<a name="steward-v4-ERC4626AdaptorV1Calls"></a>

### ERC4626AdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [ERC4626AdaptorV1](#steward-v4-ERC4626AdaptorV1) | repeated |  |





 

 

 

 



<a name="f_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## f_token.proto



<a name="steward-v4-FTokenAdaptorV1"></a>

### FTokenAdaptorV1
Represents call data for the Frax adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| lend_frax | [FTokenAdaptorV1.LendFrax](#steward-v4-FTokenAdaptorV1-LendFrax) |  | Represents function `lendFrax(IFToken fToken, uint256 amountToDeposit)` |
| redeem_frax_share | [FTokenAdaptorV1.RedeemFraxShare](#steward-v4-FTokenAdaptorV1-RedeemFraxShare) |  | Represents function `redeemFraxShare(IFToken fToken, uint256 amountToRedeem)` |
| withdraw_frax | [FTokenAdaptorV1.WithdrawFrax](#steward-v4-FTokenAdaptorV1-WithdrawFrax) |  | Represents function `withdrawFrax(IFToken fToken, uint256 amountToWithdraw)` |
| call_add_interest | [FTokenAdaptorV1.CallAddInterest](#steward-v4-FTokenAdaptorV1-CallAddInterest) |  | Represents function `callAddInterest(IFToken fToken)` |






<a name="steward-v4-FTokenAdaptorV1-CallAddInterest"></a>

### FTokenAdaptorV1.CallAddInterest
Allows a strategist to call `addInterest` on a Frax Pair they are using

Represents `function callAddInterest(IFToken fToken)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_token | [string](#string) |  | The address of the fToken to call `addInterest` on. |






<a name="steward-v4-FTokenAdaptorV1-LendFrax"></a>

### FTokenAdaptorV1.LendFrax
Allows strategists to lend FRAX on FraxLend

Represents `function lendFrax(IFToken fToken, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_token | [string](#string) |  | The address of the fToken to lend. |
| amount_to_deposit | [string](#string) |  | The amount of the fToken to lend. |






<a name="steward-v4-FTokenAdaptorV1-RedeemFraxShare"></a>

### FTokenAdaptorV1.RedeemFraxShare
Allows strategists to redeem FRAX shares on FraxLend

Represents `function redeemFraxShare(IFToken fToken, uint256 amountToRedeem)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_token | [string](#string) |  | The address of the fToken to redeem. |
| amount_to_redeem | [string](#string) |  | The amount of the fToken to redeem. |






<a name="steward-v4-FTokenAdaptorV1-WithdrawFrax"></a>

### FTokenAdaptorV1.WithdrawFrax
Allows strategists to withdraw FRAX from FraxLend

Represents `function withdrawFrax(IFToken fToken, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| f_token | [string](#string) |  | The address of the fToken to withdraw. |
| amount_to_withdraw | [string](#string) |  | The amount of the fToken to withdraw. |






<a name="steward-v4-FTokenAdaptorV1Calls"></a>

### FTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [FTokenAdaptorV1](#steward-v4-FTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="fees_and_reserves-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## fees_and_reserves.proto



<a name="steward-v4-FeesAndReservesAdaptorV1"></a>

### FeesAndReservesAdaptorV1
Represents call data for the FeesAndReserves and FeesAndReservesAdaptor contracts.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| update_performance_fees | [FeesAndReservesAdaptorV1.UpdatePerformanceFees](#steward-v4-FeesAndReservesAdaptorV1-UpdatePerformanceFees) |  | Represents function `updatePerformanceFee(uint32 performanceFee)` |
| update_management_fees | [FeesAndReservesAdaptorV1.UpdateManagementFees](#steward-v4-FeesAndReservesAdaptorV1-UpdateManagementFees) |  | Represents function `updateManagementFee(uint32 managementFee)` |
| change_upkeep_frequency | [FeesAndReservesAdaptorV1.ChangeUpkeepFrequency](#steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency) |  | Represents function `changeUpkeepFrequency(uint64 newFrequency)` |
| change_upkeep_max_gas | [FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas](#steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas) |  | Represents function `changeUpkeepMaxGas(uint64 newMaxGas)` |
| setup_meta_data | [FeesAndReservesAdaptorV1.SetupMetaData](#steward-v4-FeesAndReservesAdaptorV1-SetupMetaData) |  | Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)` |
| add_assets_to_reserves | [FeesAndReservesAdaptorV1.AddAssetsToReserves](#steward-v4-FeesAndReservesAdaptorV1-AddAssetsToReserves) |  | Represents function `addAssetsToReserves(uint256 amount)` |
| withdraw_assets_from_reserves | [FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves](#steward-v4-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves) |  | Represents function `withdrawAssetsFromReserves(uint256 amount)` |
| prepare_fees | [FeesAndReservesAdaptorV1.PrepareFees](#steward-v4-FeesAndReservesAdaptorV1-PrepareFees) |  | Represents function `prepareFees(uint256 amount)` |






<a name="steward-v4-FeesAndReservesAdaptorV1-AddAssetsToReserves"></a>

### FeesAndReservesAdaptorV1.AddAssetsToReserves
Allows the owner to add assets to the Cellar&#39;s reserves

Represents function `addAssetsToReserves(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency"></a>

### FeesAndReservesAdaptorV1.ChangeUpkeepFrequency
Allows the owner to update a Cellar&#39;s upkeep frequency.

Represents function `changeUpkeepFrequency(uint64 newFrequency)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_frequency | [uint64](#uint64) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas"></a>

### FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas
Allows the owner to update a Cellar&#39;s upkeep max gas.

Represents function `changeUpkeepMaxGas(uint64 newMaxGas)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_max_gas | [uint64](#uint64) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-PrepareFees"></a>

### FeesAndReservesAdaptorV1.PrepareFees
Allows the owner to prepare fees to be split between the platform, strategist, and protocol

Represents function `prepareFees(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-SetupMetaData"></a>

### FeesAndReservesAdaptorV1.SetupMetaData
Allows the owner to set the Cellar&#39;s fee metadata

Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| management_fee | [uint32](#uint32) |  |  |
| performance_fee | [uint32](#uint32) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-UpdateManagementFees"></a>

### FeesAndReservesAdaptorV1.UpdateManagementFees
Allows the owner to update a Cellar&#39;s management fee.

Represents function `updateManagementFee(uint32 managementFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| management_fee | [uint32](#uint32) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-UpdatePerformanceFees"></a>

### FeesAndReservesAdaptorV1.UpdatePerformanceFees
Allows the owner to update a Cellar&#39;s performance fee.

Represents function `updatePerformanceFee(uint32 performanceFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| performance_fee | [uint32](#uint32) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves"></a>

### FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves
Allows the owner to withdraw assets from the Cellar&#39;s reserves

Represents function `withdrawAssetsFromReserves(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v4-FeesAndReservesAdaptorV1Calls"></a>

### FeesAndReservesAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [FeesAndReservesAdaptorV1](#steward-v4-FeesAndReservesAdaptorV1) | repeated |  |





 

 

 

 



<a name="governance-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## governance.proto



<a name="steward-v4-GovernanceCall"></a>

### GovernanceCall
Represents a governance-executed cellar function call. Used for Scheduled Cork Proposals in Sommelier.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| aave_v2_stablecoin | [AaveV2StablecoinGovernance](#steward-v4-AaveV2StablecoinGovernance) |  | Governance function calls to the AaveV2Stablecoin cellar |
| cellar_v1 | [CellarV1Governance](#steward-v4-CellarV1Governance) |  | Governance function calls to V1 cellars |
| cellar_v2 | [CellarV2Governance](#steward-v4-CellarV2Governance) |  | Governance function calls to V2 cellars |
| cellar_v2_2 | [CellarV2_2Governance](#steward-v4-CellarV2_2Governance) |  | Governance function calls to the V2.2 cellars |
| cellar_v2_5 | [CellarV2_5Governance](#steward-v4-CellarV2_5Governance) |  | Governance function calls to the V2.5 cellars |





 

 

 

 



<a name="legacy_cellar_adaptor-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## legacy_cellar_adaptor.proto



<a name="steward-v4-LegacyCellarAdaptorV1"></a>

### LegacyCellarAdaptorV1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| depositToCellar | [LegacyCellarAdaptorV1.DepositToCellar](#steward-v4-LegacyCellarAdaptorV1-DepositToCellar) |  | Represents function `depositToCellar(Cellar cellar, uint256 assets, address oracle)` |
| withdrawFromCellar | [LegacyCellarAdaptorV1.WithdrawFromCellar](#steward-v4-LegacyCellarAdaptorV1-WithdrawFromCellar) |  | Represents function `withdrawFromCellar(Cellar cellar, uint256 assets, address oracle)` |






<a name="steward-v4-LegacyCellarAdaptorV1-DepositToCellar"></a>

### LegacyCellarAdaptorV1.DepositToCellar
Allows strategists to deposit into Cellar positions.

Represents function `depositToCellar(Cellar cellar, uint256 assets, address oracle)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar | [string](#string) |  |  |
| assets | [string](#string) |  |  |
| oracle | [string](#string) |  |  |






<a name="steward-v4-LegacyCellarAdaptorV1-WithdrawFromCellar"></a>

### LegacyCellarAdaptorV1.WithdrawFromCellar
Allows strategists to withdraw from Cellar positions.

Represents function `withdrawFromCellar(Cellar cellar, uint256 assets, address oracle)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar | [string](#string) |  |  |
| assets | [string](#string) |  |  |
| oracle | [string](#string) |  |  |






<a name="steward-v4-LegacyCellarAdaptorV1Calls"></a>

### LegacyCellarAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [LegacyCellarAdaptorV1](#steward-v4-LegacyCellarAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_aave_v2_a_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_aave_v2_a_token.proto



<a name="steward-v4-MorphoAaveV2ATokenAdaptorV1"></a>

### MorphoAaveV2ATokenAdaptorV1
Represents call data for the Morpho Aave V2 AToken adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave_v2_morpho | [MorphoAaveV2ATokenAdaptorV1.DepositToAaveV2Morpho](#steward-v4-MorphoAaveV2ATokenAdaptorV1-DepositToAaveV2Morpho) |  | Represents function `depositToAaveV2Morpho(IAaveToken aToken, uint256 amountToDeposit)` |
| withdraw_from_aave_v2_morpho | [MorphoAaveV2ATokenAdaptorV1.WithdrawFromAaveV2Morpho](#steward-v4-MorphoAaveV2ATokenAdaptorV1-WithdrawFromAaveV2Morpho) |  | Represents function `withdrawFromAaveV2Morpho(IAaveToken aToken, uint256 amountToWithdraw)` |
| claim | [Claim](#steward-v4-Claim) |  | Represents function `claim(uint256 claimable, bytes32[] memory proof)` |






<a name="steward-v4-MorphoAaveV2ATokenAdaptorV1-DepositToAaveV2Morpho"></a>

### MorphoAaveV2ATokenAdaptorV1.DepositToAaveV2Morpho
Allows strategists to lend assets on Morpho.

Represents function `depositToAaveV2Morpho(IAaveToken aToken, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a_token | [string](#string) |  | The address of the Aave V2 aToken to deposit to. |
| amount_to_deposit | [string](#string) |  | The amount of the asset to deposit. |






<a name="steward-v4-MorphoAaveV2ATokenAdaptorV1-WithdrawFromAaveV2Morpho"></a>

### MorphoAaveV2ATokenAdaptorV1.WithdrawFromAaveV2Morpho
Allows strategists to withdraw assets from Morpho.

Represents function `withdrawFromAaveV2Morpho(IAaveToken aToken, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a_token | [string](#string) |  | The address of the Aave V2 aToken to withdraw from. |
| amount_to_withdraw | [string](#string) |  | The amount of the asset to withdraw. |






<a name="steward-v4-MorphoAaveV2ATokenAdaptorV1Calls"></a>

### MorphoAaveV2ATokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoAaveV2ATokenAdaptorV1](#steward-v4-MorphoAaveV2ATokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_aave_v2_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_aave_v2_debt_token.proto



<a name="steward-v4-MorphoAaveV2DebtTokenAdaptorV1"></a>

### MorphoAaveV2DebtTokenAdaptorV1
Represents call data for the Morpho Aave V2 Debt Token adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave_v2_morpho | [MorphoAaveV2DebtTokenAdaptorV1.BorrowFromAaveV2Morpho](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1-BorrowFromAaveV2Morpho) |  | Represents function `borrowFromAaveV2Morpho(address aToken, uint256 amountToBorrow)` |
| repay_aave_v2_morpho_debt | [MorphoAaveV2DebtTokenAdaptorV1.RepayAaveV2MorphoDebt](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1-RepayAaveV2MorphoDebt) |  | Represents function `repayAaveV2MorphoDebt(IAaveToken aToken, uint256 amountToRepay)` |






<a name="steward-v4-MorphoAaveV2DebtTokenAdaptorV1-BorrowFromAaveV2Morpho"></a>

### MorphoAaveV2DebtTokenAdaptorV1.BorrowFromAaveV2Morpho
Allows strategists to borrow assets from Aave.

Represents function `borrowFromAaveV2Morpho(address aToken, uint256 amountToBorrow)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a_token | [string](#string) |  | The address of the Aave V2 aToken to borrow. |
| amount_to_borrow | [string](#string) |  | The amount of the asset to borrow. |






<a name="steward-v4-MorphoAaveV2DebtTokenAdaptorV1-RepayAaveV2MorphoDebt"></a>

### MorphoAaveV2DebtTokenAdaptorV1.RepayAaveV2MorphoDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveV2MorphoDebt(IAaveToken aToken, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| a_token | [string](#string) |  | The address of the Aave V2 aToken to repay. |
| amount_to_repay | [string](#string) |  | The amount of the asset to repay. |






<a name="steward-v4-MorphoAaveV2DebtTokenAdaptorV1Calls"></a>

### MorphoAaveV2DebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoAaveV2DebtTokenAdaptorV1](#steward-v4-MorphoAaveV2DebtTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_aave_v3_a_token_collateral-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_aave_v3_a_token_collateral.proto



<a name="steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1"></a>

### MorphoAaveV3ATokenCollateralAdaptorV1
Represents call data for the Morpho Aave V3 AToken Collateral adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave_v3_morpho | [MorphoAaveV3ATokenCollateralAdaptorV1.DepositToAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-DepositToAaveV3Morpho) |  | Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave_v3_morpho | [MorphoAaveV3ATokenCollateralAdaptorV1.WithdrawFromAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-WithdrawFromAaveV3Morpho) |  | Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |
| claim | [Claim](#steward-v4-Claim) |  | Represents function `claim(uint256 claimable, bytes32[] memory proof)` |






<a name="steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-DepositToAaveV3Morpho"></a>

### MorphoAaveV3ATokenCollateralAdaptorV1.DepositToAaveV3Morpho
Allows strategists to lend assets on Morpho

Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_to_deposit | [string](#string) |  | The address of the token to deposit |
| amount_to_deposit | [string](#string) |  | The amount of tokens to deposit |






<a name="steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1-WithdrawFromAaveV3Morpho"></a>

### MorphoAaveV3ATokenCollateralAdaptorV1.WithdrawFromAaveV3Morpho
Allows strategists to withdraw assets from Morpho

Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_to_withdraw | [string](#string) |  | The address of the token to withdraw |
| amount_to_withdraw | [string](#string) |  | The amount of tokens to withdraw |






<a name="steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1Calls"></a>

### MorphoAaveV3ATokenCollateralAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoAaveV3ATokenCollateralAdaptorV1](#steward-v4-MorphoAaveV3ATokenCollateralAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_aave_v3_a_token_p2p-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_aave_v3_a_token_p2p.proto



<a name="steward-v4-MorphoAaveV3ATokenP2PAdaptorV1"></a>

### MorphoAaveV3ATokenP2PAdaptorV1
Represents call data for the Morpho Aave V3 A Token P2P adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave_v3_morpho | [MorphoAaveV3ATokenP2PAdaptorV1.DepositToAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-DepositToAaveV3Morpho) |  | Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit, uint256 maxIterations)` |
| withdraw_from_aave_v3_morpho | [MorphoAaveV3ATokenP2PAdaptorV1.WithdrawFromAaveV3Morpho](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-WithdrawFromAaveV3Morpho) |  | Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw, uint256 maxIterations)` |
| claim | [Claim](#steward-v4-Claim) |  | Represents function `claim(uint256 claimable, bytes32[] memory proof)` |






<a name="steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-DepositToAaveV3Morpho"></a>

### MorphoAaveV3ATokenP2PAdaptorV1.DepositToAaveV3Morpho
Allows strategists to lend assets on Morpho

Represents function `depositToAaveV3Morpho(ERC20 tokenToDeposit, uint256 amountToDeposit, uint256 maxIterations)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_to_deposit | [string](#string) |  | The address of the token to deposit |
| amount_to_deposit | [string](#string) |  | The amount of tokens to deposit |
| max_iterations | [string](#string) |  | The maximum number of iterations to run |






<a name="steward-v4-MorphoAaveV3ATokenP2PAdaptorV1-WithdrawFromAaveV3Morpho"></a>

### MorphoAaveV3ATokenP2PAdaptorV1.WithdrawFromAaveV3Morpho
Allows strategists to withdraw assets from Morpho

Represents function `withdrawFromAaveV3Morpho(ERC20 tokenToWithdraw, uint256 amountToWithdraw, uint256 maxIterations)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_to_withdraw | [string](#string) |  | The address of the token to withdraw |
| amount_to_withdraw | [string](#string) |  | The amount of tokens to withdraw |
| max_iterations | [string](#string) |  | The maximum number of iterations to run |






<a name="steward-v4-MorphoAaveV3ATokenP2PAdaptorV1Calls"></a>

### MorphoAaveV3ATokenP2PAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoAaveV3ATokenP2PAdaptorV1](#steward-v4-MorphoAaveV3ATokenP2PAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_aave_v3_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_aave_v3_debt_token.proto



<a name="steward-v4-MorphoAaveV3DebtTokenAdaptorV1"></a>

### MorphoAaveV3DebtTokenAdaptorV1
Represents call data for the Morpho Aave V3 Debt Token adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave_v3_morpho | [MorphoAaveV3DebtTokenAdaptorV1.BorrowFromAaveV3Morpho](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1-BorrowFromAaveV3Morpho) |  | Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)` |
| repay_aave_v3_morpho_debt | [MorphoAaveV3DebtTokenAdaptorV1.RepayAaveV3MorphoDebt](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1-RepayAaveV3MorphoDebt) |  | Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |






<a name="steward-v4-MorphoAaveV3DebtTokenAdaptorV1-BorrowFromAaveV3Morpho"></a>

### MorphoAaveV3DebtTokenAdaptorV1.BorrowFromAaveV3Morpho
Allows strategists to borrow assets from Morpho

Represents function `borrowFromAaveV3Morpho(address underlying, uint256 amountToBorrow, uint256 maxIterations)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| underlying | [string](#string) |  | The underlying asset to borrow |
| amount_to_borrow | [string](#string) |  | The amount of the underlying asset to borrow |
| max_iterations | [string](#string) |  | The maximum number of iterations to perform |






<a name="steward-v4-MorphoAaveV3DebtTokenAdaptorV1-RepayAaveV3MorphoDebt"></a>

### MorphoAaveV3DebtTokenAdaptorV1.RepayAaveV3MorphoDebt
Allows strategists to repay loan debt on Morpho

Represents function `repayAaveV3MorphoDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_to_repay | [string](#string) |  | The token to repay |
| amount_to_repay | [string](#string) |  | The amount of the token to repay |






<a name="steward-v4-MorphoAaveV3DebtTokenAdaptorV1Calls"></a>

### MorphoAaveV3DebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoAaveV3DebtTokenAdaptorV1](#steward-v4-MorphoAaveV3DebtTokenAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_blue_collateral-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_blue_collateral.proto



<a name="steward-v4-MorphoBlueCollateralAdaptorV1"></a>

### MorphoBlueCollateralAdaptorV1
Represents call data for the Morpho Blue Collateral adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| add_collateral | [MorphoBlueCollateralAdaptorV1.AddCollateral](#steward-v4-MorphoBlueCollateralAdaptorV1-AddCollateral) |  | Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)` |
| remove_collateral | [MorphoBlueCollateralAdaptorV1.RemoveCollateral](#steward-v4-MorphoBlueCollateralAdaptorV1-RemoveCollateral) |  | Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)` |






<a name="steward-v4-MorphoBlueCollateralAdaptorV1-AddCollateral"></a>

### MorphoBlueCollateralAdaptorV1.AddCollateral
Allows strategists to add collateral to the respective cellar position on specified MB Market, enabling borrowing.

Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| collateral_to_deposit | [string](#string) |  | The amount of collateral to add |






<a name="steward-v4-MorphoBlueCollateralAdaptorV1-RemoveCollateral"></a>

### MorphoBlueCollateralAdaptorV1.RemoveCollateral
Allows strategists to remove collateral from the respective cellar position on specified MB Market.

Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| collateral_amount | [string](#string) |  | The amount of collateral to remove |






<a name="steward-v4-MorphoBlueCollateralAdaptorV1Calls"></a>

### MorphoBlueCollateralAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoBlueCollateralAdaptorV1](#steward-v4-MorphoBlueCollateralAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_blue_debt-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_blue_debt.proto



<a name="steward-v4-MorphoBlueDebtAdaptorV1"></a>

### MorphoBlueDebtAdaptorV1
Represents call data for the Morpho Blue Debt adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_morpho_blue | [MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue](#steward-v4-MorphoBlueDebtAdaptorV1-BorrowFromMorphoBlue) |  | Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)` |
| repay_morpho_blue_debt | [MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt](#steward-v4-MorphoBlueDebtAdaptorV1-RepayMorphoBlueDebt) |  | Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)` |






<a name="steward-v4-MorphoBlueDebtAdaptorV1-BorrowFromMorphoBlue"></a>

### MorphoBlueDebtAdaptorV1.BorrowFromMorphoBlue
Allows strategists borrow a specific amount of an asset on Morpho Blue

Represents function `borrowFromMorphoBlue(MarketParams memory _market, uint256 _amountToBorrow)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| amount_to_borrow | [string](#string) |  | The amount of the debt token to borrow |






<a name="steward-v4-MorphoBlueDebtAdaptorV1-RepayMorphoBlueDebt"></a>

### MorphoBlueDebtAdaptorV1.RepayMorphoBlueDebt
Allows strategists to repay loan debt on Morph Blue Lending Market. Make sure to call addInterest() beforehand to ensure we are repaying what is required.

Represents function `repayMorphoBlueDebt(MarketParams memory _market, uint256 _debtTokenRepayAmount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| debt_token_repay_amount | [string](#string) |  | The amount of the debt token to repay |






<a name="steward-v4-MorphoBlueDebtAdaptorV1Calls"></a>

### MorphoBlueDebtAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoBlueDebtAdaptorV1](#steward-v4-MorphoBlueDebtAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_blue_supply-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_blue_supply.proto



<a name="steward-v4-MorphoBlueSupplyAdaptorV1"></a>

### MorphoBlueSupplyAdaptorV1
Represents call data for the Morpho Blue Supply adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| lend_to_morpho_blue | [MorphoBlueSupplyAdaptorV1.LendToMorphoBlue](#steward-v4-MorphoBlueSupplyAdaptorV1-LendToMorphoBlue) |  | Represents function `lendToMorphoBlue(MarketParams memory _market, uint256 _assets)` |
| withdraw_from_morpho_blue | [MorphoBlueSupplyAdaptorV1.WithdrawFromMorphoBlue](#steward-v4-MorphoBlueSupplyAdaptorV1-WithdrawFromMorphoBlue) |  | Represents function `withdrawFromMorphoBlue(MarketParams memory _market, uint256 _assets)` |






<a name="steward-v4-MorphoBlueSupplyAdaptorV1-LendToMorphoBlue"></a>

### MorphoBlueSupplyAdaptorV1.LendToMorphoBlue
Allows strategists to lend a specific amount for an asset on Morpho Blue

Represents function `lendToMorphoBlue(MarketParams memory _market, uint256 _assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| assets | [string](#string) |  | The amount of the loan token to lend |






<a name="steward-v4-MorphoBlueSupplyAdaptorV1-WithdrawFromMorphoBlue"></a>

### MorphoBlueSupplyAdaptorV1.WithdrawFromMorphoBlue
Allows strategists to withdraw the underlying asset plus interest

Represents function `withdrawFromMorphoBlue(MarketParams memory _market, uint256 _assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [MarketParams](#steward-v4-MarketParams) |  | Identifier of a Morpho Blue Market |
| assets | [string](#string) |  | The amount of the loan token to lend |






<a name="steward-v4-MorphoBlueSupplyAdaptorV1Calls"></a>

### MorphoBlueSupplyAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [MorphoBlueSupplyAdaptorV1](#steward-v4-MorphoBlueSupplyAdaptorV1) | repeated |  |





 

 

 

 



<a name="morpho_reward_handler-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## morpho_reward_handler.proto



<a name="steward-v4-Claim"></a>

### Claim
Allows Morpho A Token cellars to claim Morpho Rewards

Represents function `claim(uint256 claimable, bytes32[] memory proof)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| claimable | [string](#string) |  | The amount of the asset to withdraw. |
| proof | [bytes](#bytes) | repeated | Proof of claim |





 

 

 

 



<a name="oneinch-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## oneinch.proto



<a name="steward-v4-OneInchAdaptorV1"></a>

### OneInchAdaptorV1
Represents call data for the OneInch adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| swap_with_one_inch | [OneInchAdaptorV1.SwapWithOneInch](#steward-v4-OneInchAdaptorV1-SwapWithOneInch) |  | Represents function `swapWithOneInch(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes swapCallData)` |






<a name="steward-v4-OneInchAdaptorV1-SwapWithOneInch"></a>

### OneInchAdaptorV1.SwapWithOneInch
Allows strategists to make ERC20 swaps using 1Inch.

Represents function `swapWithOneInch(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes swapCallData)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  |  |
| token_out | [string](#string) |  |  |
| amount | [string](#string) |  |  |
| swap_call_data | [bytes](#bytes) |  |  |






<a name="steward-v4-OneInchAdaptorV1Calls"></a>

### OneInchAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [OneInchAdaptorV1](#steward-v4-OneInchAdaptorV1) | repeated |  |





 

 

 

 



<a name="pendle-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## pendle.proto



<a name="steward-v4-PendleAdaptorV1"></a>

### PendleAdaptorV1
Represents call data for the Pendle adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| mint_sy_from_token | [PendleAdaptorV1.MintSyFromToken](#steward-v4-PendleAdaptorV1-MintSyFromToken) |  | Represents function `mintSyFromToken(IPendleMarket market, uint256 minSyOut, TokenInput memory input)` |
| mint_py_from_sy | [PendleAdaptorV1.MintPyFromSy](#steward-v4-PendleAdaptorV1-MintPyFromSy) |  | Represents function `mintPyFromSy(IPendleMarket market, uint256 netSyIn, uint256 minPyOut)` |
| swap_exact_pt_for_yt | [PendleAdaptorV1.SwapExactPtForYt](#steward-v4-PendleAdaptorV1-SwapExactPtForYt) |  | Represents function `swapExactPtForYt(IPendleMarket market, uint256 exactPtIn, uint256 minYtOut, ApproxParams calldata guessTotalYtToSwap)` |
| swap_exact_yt_for_pt | [PendleAdaptorV1.SwapExactYtForPt](#steward-v4-PendleAdaptorV1-SwapExactYtForPt) |  | Represents function `swapExactYtForPt(IPendleMarket market, uint256 exactYtIn, uint256 minPtOut, ApproxParams calldata guessTotalPtToSwap)` |
| add_liquidity_dual_sy_and_pt | [PendleAdaptorV1.AddLiquidityDualSyAndPt](#steward-v4-PendleAdaptorV1-AddLiquidityDualSyAndPt) |  | Represents function `addLiquidityDualSyAndPt(IPendleMarket market, uint256 netSyDesired, uint256 netPtDesired, uint256 minLpOut)` |
| remove_liquidity_dual_sy_and_pt | [PendleAdaptorV1.RemoveLiquidityDualSyAndPt](#steward-v4-PendleAdaptorV1-RemoveLiquidityDualSyAndPt) |  | Represents function `removeLiquidityDualSyAndPt(IPendleMarket market, uint256 netLpToRemove, uint256 minSyOut, uint256 minPtOut)` |
| redeem_py_to_sy | [PendleAdaptorV1.RedeemPyToSy](#steward-v4-PendleAdaptorV1-RedeemPyToSy) |  | Represents function `redeemPyToSy(IPendleMarket market, uint256 netPyIn, uint256 minSyOut)` |
| redeem_sy_to_token | [PendleAdaptorV1.RedeemSyToToken](#steward-v4-PendleAdaptorV1-RedeemSyToToken) |  | Represents function `redeemSyToToken(IPendleMarket market, uint256 netSyIn, TokenOutput memory output)` |






<a name="steward-v4-PendleAdaptorV1-AddLiquidityDualSyAndPt"></a>

### PendleAdaptorV1.AddLiquidityDualSyAndPt
Allows strategist to add liquidity to a Pendle market.

Represents function `addLiquidityDualSyAndPt(IPendleMarket market, uint256 netSyDesired, uint256 netPtDesired, uint256 minLpOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to add liquidity to. |
| net_sy_desired | [string](#string) |  | The net amount of SY to add to the market. |
| net_pt_desired | [string](#string) |  | The net amount of PT to add to the market. |
| min_lp_out | [string](#string) |  | The minimum amount of LP tokens to receive. |






<a name="steward-v4-PendleAdaptorV1-ApproxParams"></a>

### PendleAdaptorV1.ApproxParams
All of these fields are uint256


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| guess_min | [string](#string) |  | The minimum guess for the swap. |
| guess_max | [string](#string) |  | The maximum guess for the swap. |
| guess_offchain | [string](#string) |  | The offchain guess for the swap. |
| max_iteration | [string](#string) |  | The maximum number of iterations for the swap. Every iteration, the diff between guessMin and guessMax will be divided by 2. |
| eps | [string](#string) |  | the max eps between the returned result &amp; the correct result, base 1e18. Normally this number will be set to 1e15 (1e18/1000 = 0.1%) |






<a name="steward-v4-PendleAdaptorV1-MintPyFromSy"></a>

### PendleAdaptorV1.MintPyFromSy
Allows strategist to exchange an SY for a PY.

Represents function `mintPyFromSy(IPendleMarket market, uint256 netSyIn, uint256 minPyOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to mint PY from. |
| net_sy_in | [string](#string) |  | The net amount of SY to exchange for PY. |
| min_py_out | [string](#string) |  | The minimum amount of PY to receive. |






<a name="steward-v4-PendleAdaptorV1-MintSyFromToken"></a>

### PendleAdaptorV1.MintSyFromToken
Allows strategist to exchange a token for an SY. 

Represents function `mintSyFromToken(IPendleMarket market, uint256 minSyOut, TokenInput memory input)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to mint SY from. |
| min_sy_out | [string](#string) |  | The minimum amount of SY to receive. |
| input | [PendleAdaptorV1.TokenInput](#steward-v4-PendleAdaptorV1-TokenInput) |  | The input token address to exchange for SY. |






<a name="steward-v4-PendleAdaptorV1-RedeemPyToSy"></a>

### PendleAdaptorV1.RedeemPyToSy
Allows strategist to redeem PY for SY.

Represents function `redeemPyToSy(IPendleMarket market, uint256 netPyIn, uint256 minSyOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to redeem PY from. |
| net_py_in | [string](#string) |  | The net amount of PY to redeem for SY. |
| min_sy_out | [string](#string) |  | The minimum amount of SY to receive. |






<a name="steward-v4-PendleAdaptorV1-RedeemSyToToken"></a>

### PendleAdaptorV1.RedeemSyToToken
Allows strategist to redeem SY for a token.

Represents function `redeemSyToToken(IPendleMarket market, uint256 netSyIn, TokenOutput memory output)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to redeem SY from. |
| net_sy_in | [string](#string) |  | The net amount of SY to redeem for a token. |
| output | [PendleAdaptorV1.TokenOutput](#steward-v4-PendleAdaptorV1-TokenOutput) |  | The output token address to receive. |






<a name="steward-v4-PendleAdaptorV1-RemoveLiquidityDualSyAndPt"></a>

### PendleAdaptorV1.RemoveLiquidityDualSyAndPt
Allows strategist to remove liquidity from a Pendle market.

Represents function `removeLiquidityDualSyAndPt(IPendleMarket market, uint256 netLpToRemove, uint256 minSyOut, uint256 minPtOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to remove liquidity from. |
| net_lp_to_remove | [string](#string) |  | The net amount of LP tokens to remove from the market. |
| min_sy_out | [string](#string) |  | The minimum amount of SY to receive. |
| min_pt_out | [string](#string) |  | The minimum amount of PT to receive. |






<a name="steward-v4-PendleAdaptorV1-SwapData"></a>

### PendleAdaptorV1.SwapData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap_type | [int32](#int32) |  | The swap type |
| ext_router | [string](#string) |  | The external router address. |
| ext_calldata | [string](#string) |  | The external calldata. |
| need_scale | [bool](#bool) |  | Whether or not scaling is needed. |






<a name="steward-v4-PendleAdaptorV1-SwapExactPtForYt"></a>

### PendleAdaptorV1.SwapExactPtForYt
Allows strategist to exchange PT for YT.

Represents function `swapExactPtForYt(IPendleMarket market, uint256 exactPtIn, uint256 minYtOut, ApproxParams calldata guessTotalYtToSwap)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to swap PT for YT. |
| exact_pt_in | [string](#string) |  | The exact amount of PT to swap for YT. |
| min_yt_out | [string](#string) |  | The minimum amount of YT to receive. |
| guess_total_yt_to_swap | [PendleAdaptorV1.ApproxParams](#steward-v4-PendleAdaptorV1-ApproxParams) |  | The approximate parameters for the swap. |






<a name="steward-v4-PendleAdaptorV1-SwapExactYtForPt"></a>

### PendleAdaptorV1.SwapExactYtForPt
Allows strategist to exchange YT for PT.

Represents function `swapExactYtForPt(IPendleMarket market, uint256 exactYtIn, uint256 minPtOut, ApproxParams calldata guessTotalPtToSwap)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  | The address of the Pendle market to swap YT for PT. |
| exact_yt_in | [string](#string) |  | The exact amount of YT to swap for PT. |
| min_pt_out | [string](#string) |  | The minimum amount of PT to receive. |
| guess_total_pt_to_swap | [PendleAdaptorV1.ApproxParams](#steward-v4-PendleAdaptorV1-ApproxParams) |  | The approximation parameters for the swap. |






<a name="steward-v4-PendleAdaptorV1-TokenInput"></a>

### PendleAdaptorV1.TokenInput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  | The input token address. |
| net_token_in | [string](#string) |  | The net amount of the input token. |
| token_mint_sy | [string](#string) |  | The token address to mint SY. |
| pendle_swap | [string](#string) |  | The Pendle swap address. |
| swap_data | [PendleAdaptorV1.SwapData](#steward-v4-PendleAdaptorV1-SwapData) |  | The swap data. |






<a name="steward-v4-PendleAdaptorV1-TokenOutput"></a>

### PendleAdaptorV1.TokenOutput



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_out | [string](#string) |  | The output token address. |
| min_token_out | [string](#string) |  | The minimum amount of the output token. |
| token_redeem_sy | [string](#string) |  | The token address to redeem SY. |
| pendle_swap | [string](#string) |  | The Pendle swap address. |
| swap_data | [PendleAdaptorV1.SwapData](#steward-v4-PendleAdaptorV1-SwapData) |  | The swap data. |






<a name="steward-v4-PendleAdaptorV1Calls"></a>

### PendleAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [PendleAdaptorV1](#steward-v4-PendleAdaptorV1) | repeated |  |





 

 

 

 



<a name="staking-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## staking.proto



<a name="steward-v4-StakingAdaptorV1"></a>

### StakingAdaptorV1
Represents call data for the Staking adaptor V1


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| mint | [StakingAdaptorV1.Mint](#steward-v4-StakingAdaptorV1-Mint) |  | Represents function `mint(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)` |
| request_burn | [StakingAdaptorV1.RequestBurn](#steward-v4-StakingAdaptorV1-RequestBurn) |  | Represents function `requestBurn(uint256 amount, bytes calldata wildcard)` |
| complete_burn | [StakingAdaptorV1.CompleteBurn](#steward-v4-StakingAdaptorV1-CompleteBurn) |  | Represents function `completeBurn(uint256 id, uint256 minAmountOut, bytes calldata wildcard)` |
| cancel_burn | [StakingAdaptorV1.CancelBurn](#steward-v4-StakingAdaptorV1-CancelBurn) |  | Represents function `cancelBurn(uint256 id, bytes calldata wildcard)` |
| wrap | [StakingAdaptorV1.Wrap](#steward-v4-StakingAdaptorV1-Wrap) |  | Represents function `wrap(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)` |
| unwrap | [StakingAdaptorV1.Unwrap](#steward-v4-StakingAdaptorV1-Unwrap) |  | Represents function `unwrap(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)` |
| mint_erc20 | [StakingAdaptorV1.MintERC20](#steward-v4-StakingAdaptorV1-MintERC20) |  | Represents function `mintERC20(ERC20 depositAsset, uint256 amount, uint256 minAmountOut, bytes calldata wildcard)` |
| remove_claimed_request | [StakingAdaptorV1.RemoveClaimedRequest](#steward-v4-StakingAdaptorV1-RemoveClaimedRequest) |  | Represents function `removeClaimedRequest(uint256, bytes calldata)` |






<a name="steward-v4-StakingAdaptorV1-CancelBurn"></a>

### StakingAdaptorV1.CancelBurn
Allows a strategist to cancel an active burn/withdraw request. 

Represents the function `cancelBurn(uint256 id, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The id of the burn request |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-CompleteBurn"></a>

### StakingAdaptorV1.CompleteBurn
Allows a strategist to complete a burn/withdraw of a derivative asset for a native asset.

Represents the function `completeBurn(uint256 id, uint256 minAmountOut, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The id of the burn request |
| min_amount_out | [string](#string) |  | The minimum amount of the asset to receive |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-Mint"></a>

### StakingAdaptorV1.Mint
Allows a strategist to `mint` a derivative asset using the chains native asset.

Represents the function `mint(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  | The amount of the asset to mint |
| min_amount_out | [string](#string) |  | The minimum amount of the asset to receive |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-MintERC20"></a>

### StakingAdaptorV1.MintERC20
Allows a strategist to mint a derivative asset using an ERC20. 

Represents the function `mintERC20(ERC20 depositAsset, uint256 amount, uint256 minAmountOut, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| deposit_asset | [string](#string) |  | The address of the ERC20 asset to deposit |
| amount | [string](#string) |  | The amount of the asset to mint |
| min_amount_out | [string](#string) |  | The minimum amount of the asset to receive |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-RemoveClaimedRequest"></a>

### StakingAdaptorV1.RemoveClaimedRequest
Allows strategist to remove a request from `requestIds` if it has already been claimed.

Represents the function `removeClaimedRequest(uint256, bytes calldata)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The id of the request to remove |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-RequestBurn"></a>

### StakingAdaptorV1.RequestBurn
Allows a strategist to request to burn/withdraw a derivative for a chains native asset.   

Represents the function `requestBurn(uint256 amount, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  | The amount of the asset to burn |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-Unwrap"></a>

### StakingAdaptorV1.Unwrap
Allows a strategist to unwrap a wrapped derivative asset.

Represents the function `unwrap(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  | The amount of the asset to unwrap |
| min_amount_out | [string](#string) |  | The minimum amount of the asset to receive |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1-Wrap"></a>

### StakingAdaptorV1.Wrap
Allows a strategist to wrap a derivative asset. 

Represents the function `wrap(uint256 amount, uint256 minAmountOut, bytes calldata wildcard)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  | The amount of the asset to wrap |
| min_amount_out | [string](#string) |  | The minimum amount of the asset to receive |
| wildcard | [string](#string) |  | Arbitrary ABI encoded data that can be used by inheriting adaptors |






<a name="steward-v4-StakingAdaptorV1Calls"></a>

### StakingAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [StakingAdaptorV1](#steward-v4-StakingAdaptorV1) | repeated |  |





 

 

 

 



<a name="steward-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## steward.proto



<a name="steward-v4-EncodeRequest"></a>

### EncodeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar_id | [string](#string) |  |  |
| aave_v2_stablecoin | [AaveV2Stablecoin](#steward-v4-AaveV2Stablecoin) |  |  |
| cellar_v1 | [CellarV1](#steward-v4-CellarV1) |  |  |
| cellar_v2 | [CellarV2](#steward-v4-CellarV2) |  |  |
| cellar_v2_2 | [CellarV2_2](#steward-v4-CellarV2_2) |  |  |
| cellar_v2_5 | [CellarV2_5](#steward-v4-CellarV2_5) |  |  |






<a name="steward-v4-EncodeResponse"></a>

### EncodeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| encoded_call | [string](#string) |  | The encoded contract call |






<a name="steward-v4-ScheduleRequest"></a>

### ScheduleRequest
Represents a scheduled function call to a particular Cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar_id | [string](#string) |  | The ID (currently simply an Ethereum address) of the target Cellar |
| block_height | [uint64](#uint64) |  | The block height at which to schedule the contract call |
| aave_v2_stablecoin | [AaveV2Stablecoin](#steward-v4-AaveV2Stablecoin) |  |  |
| cellar_v1 | [CellarV1](#steward-v4-CellarV1) |  |  |
| cellar_v2 | [CellarV2](#steward-v4-CellarV2) |  |  |
| cellar_v2_2 | [CellarV2_2](#steward-v4-CellarV2_2) |  |  |
| cellar_v2_5 | [CellarV2_5](#steward-v4-CellarV2_5) |  |  |
| chain_id | [uint64](#uint64) |  | The ID of the chain on which the target Cellar resides |
| deadline | [uint64](#uint64) |  | The unix timestamp deadline for the contract call to be executed |






<a name="steward-v4-ScheduleResponse"></a>

### ScheduleResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The hex encoded ID of the scheduled cork |
| chain_id | [uint64](#uint64) |  | The ID of the chain on which the target Cellar resides |






<a name="steward-v4-SimulateRequest"></a>

### SimulateRequest
Represents a simulated function call to a particular Cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| request | [ScheduleRequest](#steward-v4-ScheduleRequest) |  |  |
| encode_only | [bool](#bool) |  | Whether to simply encode and return the contract call data, skipping the Tenderly simulation |






<a name="steward-v4-SimulateResponse"></a>

### SimulateResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| encoded_call | [string](#string) |  | The encoded contract call |
| response_body | [string](#string) |  | The response body from the Tenderly simulation |






<a name="steward-v4-StatusRequest"></a>

### StatusRequest
Represents a request for Steward&#39;s current status






<a name="steward-v4-StatusResponse"></a>

### StatusResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [string](#string) |  |  |






<a name="steward-v4-VersionRequest"></a>

### VersionRequest







<a name="steward-v4-VersionResponse"></a>

### VersionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [string](#string) |  |  |





 

 

 


<a name="steward-v4-ContractCallService"></a>

### ContractCallService
Service for handling Cellar contract calls

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Schedule | [ScheduleRequest](#steward-v4-ScheduleRequest) | [ScheduleResponse](#steward-v4-ScheduleResponse) | Handles scheduled contract call submission |


<a name="steward-v4-EncodingService"></a>

### EncodingService
Service for testing contract call encoding. Simply returns the encoded call data in the response.

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Encode | [EncodeRequest](#steward-v4-EncodeRequest) | [EncodeResponse](#steward-v4-EncodeResponse) | Handles contract call encoding |


<a name="steward-v4-SimulateContractCallService"></a>

### SimulateContractCallService
Service for simulating contract calls encoded by Steward using Tenderly

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Simulate | [SimulateRequest](#steward-v4-SimulateRequest) | [SimulateResponse](#steward-v4-SimulateResponse) | Handles simulated contract call submission |


<a name="steward-v4-StatusService"></a>

### StatusService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Version | [VersionRequest](#steward-v4-VersionRequest) | [VersionResponse](#steward-v4-VersionResponse) |  |

 



<a name="swap_with_uniswap-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## swap_with_uniswap.proto



<a name="steward-v4-SwapWithUniswapAdaptorV1"></a>

### SwapWithUniswapAdaptorV1
Represents call data for the Uniswap V3 adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| swap_with_uni_v2 | [SwapWithUniswapAdaptorV1.SwapWithUniV2](#steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV2) |  | Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)` |
| swap_with_uni_v3 | [SwapWithUniswapAdaptorV1.SwapWithUniV3](#steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV3) |  | Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)` |






<a name="steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV2"></a>

### SwapWithUniswapAdaptorV1.SwapWithUniV2
Perform a swap using Uniswap V2.

Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated |  |
| amount | [string](#string) |  |  |
| amount_out_min | [string](#string) |  |  |






<a name="steward-v4-SwapWithUniswapAdaptorV1-SwapWithUniV3"></a>

### SwapWithUniswapAdaptorV1.SwapWithUniV3
Perform a swap using Uniswap V3.

Represents function `Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)``


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated |  |
| pool_fees | [uint32](#uint32) | repeated |  |
| amount | [string](#string) |  |  |
| amount_out_min | [string](#string) |  |  |






<a name="steward-v4-SwapWithUniswapAdaptorV1Calls"></a>

### SwapWithUniswapAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [SwapWithUniswapAdaptorV1](#steward-v4-SwapWithUniswapAdaptorV1) | repeated |  |





 

 

 

 



<a name="uniswap_v3-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## uniswap_v3.proto



<a name="steward-v4-UniswapV3AdaptorV2"></a>

### UniswapV3AdaptorV2
Represents call data for the Uniswap V3 adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| open_position | [UniswapV3AdaptorV2.OpenPosition](#steward-v4-UniswapV3AdaptorV2-OpenPosition) |  | Represents function `openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)` |
| close_position | [UniswapV3AdaptorV2.ClosePosition](#steward-v4-UniswapV3AdaptorV2-ClosePosition) |  | Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)` |
| add_to_position | [UniswapV3AdaptorV2.AddToPosition](#steward-v4-UniswapV3AdaptorV2-AddToPosition) |  | Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)` |
| take_from_position | [UniswapV3AdaptorV2.TakeFromPosition](#steward-v4-UniswapV3AdaptorV2-TakeFromPosition) |  | Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)` |
| collect_fees | [UniswapV3AdaptorV2.CollectFees](#steward-v4-UniswapV3AdaptorV2-CollectFees) |  | Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)` |
| purge_all_zero_liquidity_positions | [UniswapV3AdaptorV2.PurgeAllZeroLiquidityPositions](#steward-v4-UniswapV3AdaptorV2-PurgeAllZeroLiquidityPositions) |  | Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)` |
| purge_single_position | [UniswapV3AdaptorV2.PurgeSinglePosition](#steward-v4-UniswapV3AdaptorV2-PurgeSinglePosition) |  | Represents function `purgeSinglePosition(uint256 tokenId)` |
| remove_unowned_position_from_tracker | [UniswapV3AdaptorV2.RemoveUnownedPositionFromTracker](#steward-v4-UniswapV3AdaptorV2-RemoveUnownedPositionFromTracker) |  | Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)` |






<a name="steward-v4-UniswapV3AdaptorV2-AddToPosition"></a>

### UniswapV3AdaptorV2.AddToPosition
Allows strategist to add to existing Uniswap V3 positions.

Represents function `addToPosition(uint256 tokenId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-ClosePosition"></a>

### UniswapV3AdaptorV2.ClosePosition
Allows strategist to close Uniswap V3 positions.

Represents function `closePosition(uint256 tokenId, uint256 min0, uint256 min1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-CollectFees"></a>

### UniswapV3AdaptorV2.CollectFees
Allows strategist to collect fees from existing Uniswap V3 positions.

Represents function `collectFees(uint256 tokenId, uint128 amount0, uint128 amount1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-OpenPosition"></a>

### UniswapV3AdaptorV2.OpenPosition
Allows strategist to open up arbritray Uniswap V3 positions.

Represents function openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |
| pool_fee | [uint32](#uint32) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |
| tick_lower | [int32](#int32) |  |  |
| tick_upper | [int32](#int32) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-PurgeAllZeroLiquidityPositions"></a>

### UniswapV3AdaptorV2.PurgeAllZeroLiquidityPositions
Allows strategist to purge zero liquidity LP positions from tracker.

Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-PurgeSinglePosition"></a>

### UniswapV3AdaptorV2.PurgeSinglePosition
Allows strategist to purge a single zero liquidity LP position from tracker.

Represents function `purgeSinglePosition(uint256 tokenId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-RemoveUnownedPositionFromTracker"></a>

### UniswapV3AdaptorV2.RemoveUnownedPositionFromTracker
Allows strategist to remove tracked positions that are not owned by the cellar.

Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2-TakeFromPosition"></a>

### UniswapV3AdaptorV2.TakeFromPosition
Allows strategist to take from existing Uniswap V3 positions.

Represents function `takeFromPosition(uint256 tokenId, uint128 liquidity, uint256 min0, uint256 min1, bool takeFees)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| liquidity | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |
| take_fees | [bool](#bool) |  |  |






<a name="steward-v4-UniswapV3AdaptorV2Calls"></a>

### UniswapV3AdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [UniswapV3AdaptorV2](#steward-v4-UniswapV3AdaptorV2) | repeated |  |





 

 

 

 



<a name="vesting_simple-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## vesting_simple.proto



<a name="steward-v4-VestingSimpleAdaptorV2"></a>

### VestingSimpleAdaptorV2
Represents call data for the Vesting Simple adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_vesting | [VestingSimpleAdaptorV2.DepositToVesting](#steward-v4-VestingSimpleAdaptorV2-DepositToVesting) |  | Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)` |
| withdraw_from_vesting | [VestingSimpleAdaptorV2.WithdrawFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawFromVesting) |  | Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)` |
| withdraw_any_from_vesting | [VestingSimpleAdaptorV2.WithdrawAnyFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawAnyFromVesting) |  | Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)` |
| withdraw_all_from_vesting | [VestingSimpleAdaptorV2.WithdrawAllFromVesting](#steward-v4-VestingSimpleAdaptorV2-WithdrawAllFromVesting) |  | Represents function `withdrawAllFromVesting(VestingSimple vestingContract)` |






<a name="steward-v4-VestingSimpleAdaptorV2-DepositToVesting"></a>

### VestingSimpleAdaptorV2.DepositToVesting
Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
deposit its entire balance (appropriate in most cases).

Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v4-VestingSimpleAdaptorV2-WithdrawAllFromVesting"></a>

### VestingSimpleAdaptorV2.WithdrawAllFromVesting
Withdraw a certain amount of tokens from vesting, from any deposit. This will not affect the cellar&#39;s TVL because any deposit must
already have vested, and will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.

Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |






<a name="steward-v4-VestingSimpleAdaptorV2-WithdrawAnyFromVesting"></a>

### VestingSimpleAdaptorV2.WithdrawAnyFromVesting
Withdraw a single deposit from vesting. This will not affect the cellar&#39;s TVL because any deposit must already have vested, and
will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.

Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v4-VestingSimpleAdaptorV2-WithdrawFromVesting"></a>

### VestingSimpleAdaptorV2.WithdrawFromVesting
Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
deposit its entire balance (appropriate in most cases).

Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| deposit_id | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v4-VestingSimpleAdaptorV2Calls"></a>

### VestingSimpleAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [VestingSimpleAdaptorV2](#steward-v4-VestingSimpleAdaptorV2) | repeated |  |





 

 

 

 



<a name="zero_x-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## zero_x.proto



<a name="steward-v4-ZeroXAdaptorV1"></a>

### ZeroXAdaptorV1



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v4-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| swap_with_0x | [ZeroXAdaptorV1.SwapWith0x](#steward-v4-ZeroXAdaptorV1-SwapWith0x) |  | Represents function `swapWith0x(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes memory swapCallData)` |






<a name="steward-v4-ZeroXAdaptorV1-SwapWith0x"></a>

### ZeroXAdaptorV1.SwapWith0x
Allows strategists to make ERC20 swaps using 0x.

Represents function `swapWith0x(ERC20 tokenIn, ERC20 tokenOut, uint256 amount, bytes memory swapCallData)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  |  |
| token_out | [string](#string) |  |  |
| amount | [string](#string) |  |  |
| swap_call_data | [bytes](#bytes) |  |  |






<a name="steward-v4-ZeroXAdaptorV1Calls"></a>

### ZeroXAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [ZeroXAdaptorV1](#steward-v4-ZeroXAdaptorV1) | repeated |  |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

