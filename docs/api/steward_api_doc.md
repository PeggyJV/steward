# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [aave_v2_stablecoin.proto](#aave_v2_stablecoin-proto)
    - [AaveV2Stablecoin](#steward-v3-AaveV2Stablecoin)
    - [AaveV2Stablecoin.Accrue](#steward-v3-AaveV2Stablecoin-Accrue)
    - [AaveV2Stablecoin.ClaimAndUnstake](#steward-v3-AaveV2Stablecoin-ClaimAndUnstake)
    - [AaveV2Stablecoin.EnterPosition](#steward-v3-AaveV2Stablecoin-EnterPosition)
    - [AaveV2Stablecoin.EnterPositionWithAssets](#steward-v3-AaveV2Stablecoin-EnterPositionWithAssets)
    - [AaveV2Stablecoin.ExitPosition](#steward-v3-AaveV2Stablecoin-ExitPosition)
    - [AaveV2Stablecoin.ExitPositionWithAssets](#steward-v3-AaveV2Stablecoin-ExitPositionWithAssets)
    - [AaveV2Stablecoin.Rebalance](#steward-v3-AaveV2Stablecoin-Rebalance)
    - [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v3-AaveV2Stablecoin-Rebalance-SwapParams)
    - [AaveV2Stablecoin.Reinvest](#steward-v3-AaveV2Stablecoin-Reinvest)
    - [AaveV2Stablecoin.SendFees](#steward-v3-AaveV2Stablecoin-SendFees)
    - [AaveV2Stablecoin.SetAccrualPeriod](#steward-v3-AaveV2Stablecoin-SetAccrualPeriod)
    - [AaveV2Stablecoin.SetDepositLimit](#steward-v3-AaveV2Stablecoin-SetDepositLimit)
    - [AaveV2Stablecoin.SetLiquidityLimit](#steward-v3-AaveV2Stablecoin-SetLiquidityLimit)
    - [AaveV2StablecoinGovernance](#steward-v3-AaveV2StablecoinGovernance)
    - [AaveV2StablecoinGovernance.InitiateShutdown](#steward-v3-AaveV2StablecoinGovernance-InitiateShutdown)
    - [AaveV2StablecoinGovernance.LiftShutdown](#steward-v3-AaveV2StablecoinGovernance-LiftShutdown)
    - [AaveV2StablecoinGovernance.SetFeesDistributor](#steward-v3-AaveV2StablecoinGovernance-SetFeesDistributor)
    - [AaveV2StablecoinGovernance.SetTrust](#steward-v3-AaveV2StablecoinGovernance-SetTrust)
    - [AaveV2StablecoinGovernance.Sweep](#steward-v3-AaveV2StablecoinGovernance-Sweep)
  
- [cellar_v1.proto](#cellar_v1-proto)
    - [CellarV1](#steward-v3-CellarV1)
    - [CellarV1.AddPosition](#steward-v3-CellarV1-AddPosition)
    - [CellarV1.PushPosition](#steward-v3-CellarV1-PushPosition)
    - [CellarV1.Rebalance](#steward-v3-CellarV1-Rebalance)
    - [CellarV1.RemovePosition](#steward-v3-CellarV1-RemovePosition)
    - [CellarV1.SetDepositLimit](#steward-v3-CellarV1-SetDepositLimit)
    - [CellarV1.SetHoldingPosition](#steward-v3-CellarV1-SetHoldingPosition)
    - [CellarV1.SetLiquidityLimit](#steward-v3-CellarV1-SetLiquidityLimit)
    - [CellarV1.SetRebalanceDeviation](#steward-v3-CellarV1-SetRebalanceDeviation)
    - [CellarV1.SetShareLockPeriod](#steward-v3-CellarV1-SetShareLockPeriod)
    - [CellarV1.SetStrategistPayoutAddress](#steward-v3-CellarV1-SetStrategistPayoutAddress)
    - [CellarV1.SetWithdrawType](#steward-v3-CellarV1-SetWithdrawType)
    - [CellarV1.SwapPositions](#steward-v3-CellarV1-SwapPositions)
    - [CellarV1Governance](#steward-v3-CellarV1Governance)
    - [CellarV1Governance.InitiateShutdown](#steward-v3-CellarV1Governance-InitiateShutdown)
    - [CellarV1Governance.LiftShutdown](#steward-v3-CellarV1Governance-LiftShutdown)
    - [CellarV1Governance.ResetHighWatermark](#steward-v3-CellarV1Governance-ResetHighWatermark)
    - [CellarV1Governance.SetFeesDistributor](#steward-v3-CellarV1Governance-SetFeesDistributor)
    - [CellarV1Governance.SetPerformanceFee](#steward-v3-CellarV1Governance-SetPerformanceFee)
    - [CellarV1Governance.SetPlatformFee](#steward-v3-CellarV1Governance-SetPlatformFee)
    - [CellarV1Governance.SetStrategistPerformanceCut](#steward-v3-CellarV1Governance-SetStrategistPerformanceCut)
    - [CellarV1Governance.SetStrategistPlatformCut](#steward-v3-CellarV1Governance-SetStrategistPlatformCut)
    - [CellarV1Governance.TrustPosition](#steward-v3-CellarV1Governance-TrustPosition)
  
    - [CellarV1.WithdrawType](#steward-v3-CellarV1-WithdrawType)
  
- [cellar_v2.proto](#cellar_v2-proto)
    - [CellarV2](#steward-v3-CellarV2)
    - [CellarV2.AdaptorCall](#steward-v3-CellarV2-AdaptorCall)
    - [CellarV2.AddPosition](#steward-v3-CellarV2-AddPosition)
    - [CellarV2.CallOnAdaptor](#steward-v3-CellarV2-CallOnAdaptor)
    - [CellarV2.RemovePosition](#steward-v3-CellarV2-RemovePosition)
    - [CellarV2.SetHoldingPosition](#steward-v3-CellarV2-SetHoldingPosition)
    - [CellarV2.SetRebalanceDeviation](#steward-v3-CellarV2-SetRebalanceDeviation)
    - [CellarV2.SetShareLockPeriod](#steward-v3-CellarV2-SetShareLockPeriod)
    - [CellarV2.SetStrategistPayoutAddress](#steward-v3-CellarV2-SetStrategistPayoutAddress)
    - [CellarV2.SwapPositions](#steward-v3-CellarV2-SwapPositions)
    - [CellarV2Governance](#steward-v3-CellarV2Governance)
    - [CellarV2Governance.InitiateShutdown](#steward-v3-CellarV2Governance-InitiateShutdown)
    - [CellarV2Governance.LiftShutdown](#steward-v3-CellarV2Governance-LiftShutdown)
    - [CellarV2Governance.SetPlatformFee](#steward-v3-CellarV2Governance-SetPlatformFee)
    - [CellarV2Governance.SetStrategistPlatformCut](#steward-v3-CellarV2Governance-SetStrategistPlatformCut)
    - [CellarV2Governance.SetupAdaptor](#steward-v3-CellarV2Governance-SetupAdaptor)
  
- [common.proto](#common-proto)
    - [OracleSwapParams](#steward-v3-OracleSwapParams)
    - [SwapParams](#steward-v3-SwapParams)
    - [UniV2OracleSwapParams](#steward-v3-UniV2OracleSwapParams)
    - [UniV2SwapParams](#steward-v3-UniV2SwapParams)
    - [UniV3OracleSwapParams](#steward-v3-UniV3OracleSwapParams)
    - [UniV3SwapParams](#steward-v3-UniV3SwapParams)
  
    - [Exchange](#steward-v3-Exchange)
  
- [governance.proto](#governance-proto)
    - [GovernanceCall](#steward-v3-GovernanceCall)
  
- [steward.proto](#steward-proto)
    - [ScheduleRequest](#steward-v3-ScheduleRequest)
    - [ScheduleResponse](#steward-v3-ScheduleResponse)
  
    - [ContractCall](#steward-v3-ContractCall)
  
- [Scalar Value Types](#scalar-value-types)



<a name="aave_v2_stablecoin-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v2_stablecoin.proto



<a name="steward-v3-AaveV2Stablecoin"></a>

### AaveV2Stablecoin
Represents a function call to the Aave V2 Stablecoin cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| accrue | [AaveV2Stablecoin.Accrue](#steward-v3-AaveV2Stablecoin-Accrue) |  | Represents function `accruePlatformFees()` |
| claim_and_unstake | [AaveV2Stablecoin.ClaimAndUnstake](#steward-v3-AaveV2Stablecoin-ClaimAndUnstake) |  | Represents function `claimAndUnstake()` |
| enter_position | [AaveV2Stablecoin.EnterPosition](#steward-v3-AaveV2Stablecoin-EnterPosition) |  | Represents function `enterPosition()` |
| enter_position_with_assets | [AaveV2Stablecoin.EnterPositionWithAssets](#steward-v3-AaveV2Stablecoin-EnterPositionWithAssets) |  | Represents function `enterPosition(uint256 assets)` |
| exit_position | [AaveV2Stablecoin.ExitPosition](#steward-v3-AaveV2Stablecoin-ExitPosition) |  | Represents function `exitPosition()` |
| exit_position_with_assets | [AaveV2Stablecoin.ExitPositionWithAssets](#steward-v3-AaveV2Stablecoin-ExitPositionWithAssets) |  | Represents function `exitPosition(uint256 assets)` |
| rebalance | [AaveV2Stablecoin.Rebalance](#steward-v3-AaveV2Stablecoin-Rebalance) |  | Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)` |
| reinvest | [AaveV2Stablecoin.Reinvest](#steward-v3-AaveV2Stablecoin-Reinvest) |  | Represents function `reinvest(uint256 minAssetsOut)` |
| set_accrual_period | [AaveV2Stablecoin.SetAccrualPeriod](#steward-v3-AaveV2Stablecoin-SetAccrualPeriod) |  | Represents function `setAccrualPeriod(uint32 newAccrualPeriod)` |
| set_deposit_limit | [AaveV2Stablecoin.SetDepositLimit](#steward-v3-AaveV2Stablecoin-SetDepositLimit) |  | Represents function `setDepositLimit(uint256 limit)` |
| set_liquidity_limit | [AaveV2Stablecoin.SetLiquidityLimit](#steward-v3-AaveV2Stablecoin-SetLiquidityLimit) |  | Represents function `setLiquidityLimit(uint256 limit)` |
| send_fees | [AaveV2Stablecoin.SendFees](#steward-v3-AaveV2Stablecoin-SendFees) |  | Represents function `transferFees()` |






<a name="steward-v3-AaveV2Stablecoin-Accrue"></a>

### AaveV2Stablecoin.Accrue
Accrue yield, platform fees, and performance fees..

Represents function `accrue()`






<a name="steward-v3-AaveV2Stablecoin-ClaimAndUnstake"></a>

### AaveV2Stablecoin.ClaimAndUnstake
Claim rewards from Aave and begin cooldown period to unstake them.

Represents function `claimAndUnstake()`






<a name="steward-v3-AaveV2Stablecoin-EnterPosition"></a>

### AaveV2Stablecoin.EnterPosition
Pushes total assets into the current Aave lending position.

Represents function `enterPosition()`






<a name="steward-v3-AaveV2Stablecoin-EnterPositionWithAssets"></a>

### AaveV2Stablecoin.EnterPositionWithAssets
Pushes assets into the current Aave lending position.

Represents function `enterPosition(uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| assets | [string](#string) |  | amount of assets to enter into the current position |






<a name="steward-v3-AaveV2Stablecoin-ExitPosition"></a>

### AaveV2Stablecoin.ExitPosition
Pulls total assets from the current Aave lending position.

Represents function `enterPosition()`






<a name="steward-v3-AaveV2Stablecoin-ExitPositionWithAssets"></a>

### AaveV2Stablecoin.ExitPositionWithAssets
Pulls assets from the current Aave lending position.

Represents function `exitPosition(uint256 assets)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| assets | [string](#string) |  | amount of assets to exit from the current position |






<a name="steward-v3-AaveV2Stablecoin-Rebalance"></a>

### AaveV2Stablecoin.Rebalance
Rebalances current assets into a new asset position.

Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`

This function is based on the Curve Pool Registry exchange_multiple() function:
https://github.com/curvefi/curve-pool-registry/blob/16a8664952cf61d7fed06acca79ad5ac696f4b20/contracts/Swaps.vy#L461-L489


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| route | [string](#string) | repeated | array of [initial token, pool, token, pool, token, ...] that specifies the swap route on Curve. |
| swap_params | [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v3-AaveV2Stablecoin-Rebalance-SwapParams) | repeated | An array of up to 4 swap params. Attempting more than four swaps will fail. |
| min_assets_out | [string](#string) |  | Minimum acceptable assets to be received from the swap (slippage parameter). Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v3-AaveV2Stablecoin-Rebalance-SwapParams"></a>

### AaveV2Stablecoin.Rebalance.SwapParams
Represents parameters for a single swap. Each swap needs the indeces in Rebalance.route of the in/out token addresses and the swap type. See the Curve contract linked above for more detail.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| in_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s input token address |
| out_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s output token address |
| swap_type | [uint64](#uint64) |  | 1 - stableswap `exchange` 2 - stableswap `exchange_underlying` 3 - cryptoswap `exchange` 4 - cryptoswap `exchange_underlying` 5 - Polygon factory metapools `exchange_underlying` See the Curve Pool Registry exchange_multiple() function for more information. |






<a name="steward-v3-AaveV2Stablecoin-Reinvest"></a>

### AaveV2Stablecoin.Reinvest
Reinvest rewards back into cellar&#39;s current position. Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.

Represents function `reinvest(uint256 minAssetsOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_assets_out | [string](#string) |  | Minimum acceptable assets to be received from the swap (slippage parameter). Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v3-AaveV2Stablecoin-SendFees"></a>

### AaveV2Stablecoin.SendFees
Transfer accrued fees to the Sommelier Chain to distribute.

Represents function `sendFees()`






<a name="steward-v3-AaveV2Stablecoin-SetAccrualPeriod"></a>

### AaveV2Stablecoin.SetAccrualPeriod
Set the accrual period over which yield is distributed.

Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_accrual_period | [uint32](#uint32) |  |  |






<a name="steward-v3-AaveV2Stablecoin-SetDepositLimit"></a>

### AaveV2Stablecoin.SetDepositLimit
Set the per-wallet deposit limit. Uses the same decimals as the current asset.

Represents function `setDepositLimit(uint256 limit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [string](#string) |  | Amount of assets to set as the new limit. Must be parsable as an unsigned 256-bit integer. |






<a name="steward-v3-AaveV2Stablecoin-SetLiquidityLimit"></a>

### AaveV2Stablecoin.SetLiquidityLimit
Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.

Represents function `setLiquidityLimit(uint256 limit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [string](#string) |  | Amount of assets to set as the new limit |






<a name="steward-v3-AaveV2StablecoinGovernance"></a>

### AaveV2StablecoinGovernance
Represents a function call initiated by governance


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| set_fees_distributor | [AaveV2StablecoinGovernance.SetFeesDistributor](#steward-v3-AaveV2StablecoinGovernance-SetFeesDistributor) |  | Represents function `setFeesDistributor(bytes32)` |
| initiate_shutdown | [AaveV2StablecoinGovernance.InitiateShutdown](#steward-v3-AaveV2StablecoinGovernance-InitiateShutdown) |  | Represents function `initiateShutdown(bool)` |
| lift_shutdown | [AaveV2StablecoinGovernance.LiftShutdown](#steward-v3-AaveV2StablecoinGovernance-LiftShutdown) |  | Represents function `liftShutdown()` |
| set_trust | [AaveV2StablecoinGovernance.SetTrust](#steward-v3-AaveV2StablecoinGovernance-SetTrust) |  | Represents function `setTrust(address, bool)` |
| sweep | [AaveV2StablecoinGovernance.Sweep](#steward-v3-AaveV2StablecoinGovernance-Sweep) |  | Represents function `sweep(address, address)` |






<a name="steward-v3-AaveV2StablecoinGovernance-InitiateShutdown"></a>

### AaveV2StablecoinGovernance.InitiateShutdown
Represents function `initiateShutdown(bool)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| empty_position | [bool](#bool) |  | Whether to empty the position |






<a name="steward-v3-AaveV2StablecoinGovernance-LiftShutdown"></a>

### AaveV2StablecoinGovernance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v3-AaveV2StablecoinGovernance-SetFeesDistributor"></a>

### AaveV2StablecoinGovernance.SetFeesDistributor
Represents function `setFeesDistributor(bytes32)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_fees_distributor | [string](#string) |  | The new fees distributor |






<a name="steward-v3-AaveV2StablecoinGovernance-SetTrust"></a>

### AaveV2StablecoinGovernance.SetTrust
Represents function `setTrust(address, bool)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [string](#string) |  | The position to set trust for |
| trust | [bool](#bool) |  | Whether to trust the address |






<a name="steward-v3-AaveV2StablecoinGovernance-Sweep"></a>

### AaveV2StablecoinGovernance.Sweep
Represents function `sweep(address, address)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to sweep |
| recipient | [string](#string) |  | The recipient of the sweep |





 

 

 

 



<a name="cellar_v1-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cellar_v1.proto



<a name="steward-v3-CellarV1"></a>

### CellarV1
Represents a function call to a cellar that implements Cellar.sol


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV1.AddPosition](#steward-v3-CellarV1-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| push_position | [CellarV1.PushPosition](#steward-v3-CellarV1-PushPosition) |  | Represents function `pushPosition(address position)` |
| remove_position | [CellarV1.RemovePosition](#steward-v3-CellarV1-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV1.SetHoldingPosition](#steward-v3-CellarV1-SetHoldingPosition) |  | Represents function `setHoldingPosition(address newHoldingPosition)` |
| rebalance | [CellarV1.Rebalance](#steward-v3-CellarV1-Rebalance) |  | Represents function `rebalance(address fromPosition, address toPosition, uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)` |
| set_strategist_payout_address | [CellarV1.SetStrategistPayoutAddress](#steward-v3-CellarV1-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| set_withdraw_type | [CellarV1.SetWithdrawType](#steward-v3-CellarV1-SetWithdrawType) |  | Represents function `setWithdrawType(WithdrawType newWithdrawType)` |
| swap_positions | [CellarV1.SwapPositions](#steward-v3-CellarV1-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_deposit_limit | [CellarV1.SetDepositLimit](#steward-v3-CellarV1-SetDepositLimit) |  | Represents function `setDepositLimit()` |
| set_liquidity_limit | [CellarV1.SetLiquidityLimit](#steward-v3-CellarV1-SetLiquidityLimit) |  | Represents function `setLiquidityLimit()` |
| set_share_lock_period | [CellarV1.SetShareLockPeriod](#steward-v3-CellarV1-SetShareLockPeriod) |  | Represents function `setShareLockPeriod()` |
| set_rebalance_deviation | [CellarV1.SetRebalanceDeviation](#steward-v3-CellarV1-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint265)` |






<a name="steward-v3-CellarV1-AddPosition"></a>

### CellarV1.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint256 index, address position)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [string](#string) |  | Index at which to add the position |
| position | [string](#string) |  | Address of the position to add |






<a name="steward-v3-CellarV1-PushPosition"></a>

### CellarV1.PushPosition
Push a trusted position to the end of the list of positions used by the cellar. If you
know you are going to add a position to the end of the array, this is more efficient then
`addPosition`.

Represents function `pushPosition(address position)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position | [string](#string) |  | Address of the position to push |






<a name="steward-v3-CellarV1-Rebalance"></a>

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
| exchange | [Exchange](#steward-v3-Exchange) |  |  |
| params | [SwapParams](#steward-v3-SwapParams) |  |  |






<a name="steward-v3-CellarV1-RemovePosition"></a>

### CellarV1.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint256 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [string](#string) |  | Index at which to remove the position |






<a name="steward-v3-CellarV1-SetDepositLimit"></a>

### CellarV1.SetDepositLimit
Set the per-wallet deposit limit. Uses the same decimals as the current asset.

Represents function `setDepositLimit()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_limit | [string](#string) |  |  |






<a name="steward-v3-CellarV1-SetHoldingPosition"></a>

### CellarV1.SetHoldingPosition
Set the holding position used by the cellar.

Represents function `setHoldingPosition(address newHoldingPosition)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_holding_position | [string](#string) |  | Address of the new holding position to use |






<a name="steward-v3-CellarV1-SetLiquidityLimit"></a>

### CellarV1.SetLiquidityLimit
Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.

Represents function `setLiquidityLimit()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_limit | [string](#string) |  |  |






<a name="steward-v3-CellarV1-SetRebalanceDeviation"></a>

### CellarV1.SetRebalanceDeviation
Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v3-CellarV1-SetShareLockPeriod"></a>

### CellarV1.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v3-CellarV1-SetStrategistPayoutAddress"></a>

### CellarV1.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v3-CellarV1-SetWithdrawType"></a>

### CellarV1.SetWithdrawType
Set the withdraw type used by the cellar.

Represents function `setWithdrawType(WithdrawType newWithdrawType)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_withdraw_type | [CellarV1.WithdrawType](#steward-v3-CellarV1-WithdrawType) |  | The withdraw type to use for the cellar |






<a name="steward-v3-CellarV1-SwapPositions"></a>

### CellarV1.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint256 index1, uint256 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [string](#string) |  | Index of the first position |
| index_2 | [string](#string) |  | Index of the second position |






<a name="steward-v3-CellarV1Governance"></a>

### CellarV1Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiate_shutdown | [CellarV1Governance.InitiateShutdown](#steward-v3-CellarV1Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV1Governance.LiftShutdown](#steward-v3-CellarV1Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| reset_high_watermark | [CellarV1Governance.ResetHighWatermark](#steward-v3-CellarV1Governance-ResetHighWatermark) |  | Represents function `resetHighWatermark()` |
| set_fees_distributor | [CellarV1Governance.SetFeesDistributor](#steward-v3-CellarV1Governance-SetFeesDistributor) |  | Represents function `setFeesDistributor(address)` |
| set_performance_fee | [CellarV1Governance.SetPerformanceFee](#steward-v3-CellarV1Governance-SetPerformanceFee) |  | Represents function `setPerformanceFee(uint256)` |
| set_platform_fee | [CellarV1Governance.SetPlatformFee](#steward-v3-CellarV1Governance-SetPlatformFee) |  | Represents function `setPlatformFee(uint256)` |
| set_strategist_performance_cut | [CellarV1Governance.SetStrategistPerformanceCut](#steward-v3-CellarV1Governance-SetStrategistPerformanceCut) |  | Represents function `setStrategistPerformanceCut(uint256)` |
| set_strategist_platform_cut | [CellarV1Governance.SetStrategistPlatformCut](#steward-v3-CellarV1Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(address)` |
| trust_position | [CellarV1Governance.TrustPosition](#steward-v3-CellarV1Governance-TrustPosition) |  | Represents function `trustPosition(address)` |






<a name="steward-v3-CellarV1Governance-InitiateShutdown"></a>

### CellarV1Governance.InitiateShutdown
Represents function `initiateShutdown()`






<a name="steward-v3-CellarV1Governance-LiftShutdown"></a>

### CellarV1Governance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v3-CellarV1Governance-ResetHighWatermark"></a>

### CellarV1Governance.ResetHighWatermark
Represents function `resetHighWatermark()`






<a name="steward-v3-CellarV1Governance-SetFeesDistributor"></a>

### CellarV1Governance.SetFeesDistributor
Represents function `setFeesDistributor(bytes32)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_fees_distributor | [string](#string) |  | Cosmos address of the new fees distributor |






<a name="steward-v3-CellarV1Governance-SetPerformanceFee"></a>

### CellarV1Governance.SetPerformanceFee
Represents function `setPerformanceFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New performance fee |






<a name="steward-v3-CellarV1Governance-SetPlatformFee"></a>

### CellarV1Governance.SetPlatformFee
Represents function `setPlatformFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New platform fee |






<a name="steward-v3-CellarV1Governance-SetStrategistPerformanceCut"></a>

### CellarV1Governance.SetStrategistPerformanceCut
Represents function `setStrategistPerformanceCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist performance cut |






<a name="steward-v3-CellarV1Governance-SetStrategistPlatformCut"></a>

### CellarV1Governance.SetStrategistPlatformCut
Represents function `setStrategistPlatformCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist platform cut |






<a name="steward-v3-CellarV1Governance-TrustPosition"></a>

### CellarV1Governance.TrustPosition
Represents function `trustPosition(address)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| erc20_address | [string](#string) |  |  |
| erc4626_address | [string](#string) |  |  |
| cellar_address | [string](#string) |  |  |





 


<a name="steward-v3-CellarV1-WithdrawType"></a>

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



<a name="steward-v3-CellarV2"></a>

### CellarV2
Represents a function call to a cellar that implements Cellar.sol


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV2.AddPosition](#steward-v3-CellarV2-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2.CallOnAdaptor](#steward-v3-CellarV2-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2.RemovePosition](#steward-v3-CellarV2-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV2.SetHoldingPosition](#steward-v3-CellarV2-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| set_strategist_payout_address | [CellarV2.SetStrategistPayoutAddress](#steward-v3-CellarV2-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| swap_positions | [CellarV2.SwapPositions](#steward-v3-CellarV2-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_rebalance_deviation | [CellarV2.SetRebalanceDeviation](#steward-v3-CellarV2-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint265)` |
| set_share_lock_period | [CellarV2.SetShareLockPeriod](#steward-v3-CellarV2-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |






<a name="steward-v3-CellarV2-AdaptorCall"></a>

### CellarV2.AdaptorCall
Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| uniswap_v3_calls | [UniswapV3AdaptorCalls](#steward-v3-UniswapV3AdaptorCalls) |  | Represents function calls to the UniswapV3Adaptor |
| aave_a_token_calls | [AaveATokenAdaptorCalls](#steward-v3-AaveATokenAdaptorCalls) |  | Represents function calls to the AaveATokenAdaptor |
| aave_debt_token_calls | [AaveDebtTokenAdaptorCalls](#steward-v3-AaveDebtTokenAdaptorCalls) |  | Represents function calls to the AavaDebtTokenAdaptor |
| compound_c_token_calls | [CompoundCTokenAdaptorCalls](#steward-v3-CompoundCTokenAdaptorCalls) |  | Represents function calls to the CompoundCTokenAdaptor |
| vesting_simple_calls | [VestingSimpleAdaptorCalls](#steward-v3-VestingSimpleAdaptorCalls) |  | Represents function calls to the VestingSimpleAdaptor |






<a name="steward-v3-CellarV2-AddPosition"></a>

### CellarV2.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v3-CellarV2-CallOnAdaptor"></a>

### CellarV2.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [CellarV2.AdaptorCall](#steward-v3-CellarV2-AdaptorCall) | repeated |  |






<a name="steward-v3-CellarV2-RemovePosition"></a>

### CellarV2.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v3-CellarV2-SetHoldingPosition"></a>

### CellarV2.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v3-CellarV2-SetRebalanceDeviation"></a>

### CellarV2.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v3-CellarV2-SetShareLockPeriod"></a>

### CellarV2.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v3-CellarV2-SetStrategistPayoutAddress"></a>

### CellarV2.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v3-CellarV2-SwapPositions"></a>

### CellarV2.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v3-CellarV2Governance"></a>

### CellarV2Governance
Represent a function call initiated through a governance proposal


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| initiate_shutdown | [CellarV2Governance.InitiateShutdown](#steward-v3-CellarV2Governance-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2Governance.LiftShutdown](#steward-v3-CellarV2Governance-LiftShutdown) |  | Represents function `liftShutdown()` |
| set_platform_fee | [CellarV2Governance.SetPlatformFee](#steward-v3-CellarV2Governance-SetPlatformFee) |  | Represents function `setPlatformFee(uint256)` |
| set_strategist_platform_cut | [CellarV2Governance.SetStrategistPlatformCut](#steward-v3-CellarV2Governance-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(address)` |
| setup_adaptor | [CellarV2Governance.SetupAdaptor](#steward-v3-CellarV2Governance-SetupAdaptor) |  | Represents function `setupAdaptor(address adaptor)` |






<a name="steward-v3-CellarV2Governance-InitiateShutdown"></a>

### CellarV2Governance.InitiateShutdown
Represents function `initiateShutdown()`






<a name="steward-v3-CellarV2Governance-LiftShutdown"></a>

### CellarV2Governance.LiftShutdown
Represents function `liftShutdown()`






<a name="steward-v3-CellarV2Governance-SetPlatformFee"></a>

### CellarV2Governance.SetPlatformFee
Represents function `setPlatformFee(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New platform fee |






<a name="steward-v3-CellarV2Governance-SetStrategistPlatformCut"></a>

### CellarV2Governance.SetStrategistPlatformCut
Represents function `setStrategistPlatformCut(uint64)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [uint64](#uint64) |  | New strategist platform cut |






<a name="steward-v3-CellarV2Governance-SetupAdaptor"></a>

### CellarV2Governance.SetupAdaptor
Allows owner to add new adaptors for the cellar to use.

Represents function `setupAdaptor(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |





 

 

 

 



<a name="common-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## common.proto



<a name="steward-v3-OracleSwapParams"></a>

### OracleSwapParams
Represents swap params for BaseAdaptor.oracleSwap()


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| univ2_params | [UniV2OracleSwapParams](#steward-v3-UniV2OracleSwapParams) |  |  |
| univ3_params | [UniV3OracleSwapParams](#steward-v3-UniV3OracleSwapParams) |  |  |






<a name="steward-v3-SwapParams"></a>

### SwapParams
Represents swap parameters for an exchange


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| univ2_params | [UniV2SwapParams](#steward-v3-UniV2SwapParams) |  | Params for a Uniswap V2 swap |
| univ3_params | [UniV3SwapParams](#steward-v3-UniV3SwapParams) |  | Params for a Uniswap V3 swap |






<a name="steward-v3-UniV2OracleSwapParams"></a>

### UniV2OracleSwapParams
Represents oracle swap parameters for UniswapV2


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |






<a name="steward-v3-UniV2SwapParams"></a>

### UniV2SwapParams
Represents swap parameters for UniswapV2


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| amount | [string](#string) |  | Amount of the first asset in the path to swap |
| amount_out_min | [string](#string) |  | The minimum amount of the last asset in the path to receive |






<a name="steward-v3-UniV3OracleSwapParams"></a>

### UniV3OracleSwapParams
Represents oracle swap parameters for UniswapV3


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| pool_fees | [uint32](#uint32) | repeated | Array of pool fees dictating what swap pools to use |






<a name="steward-v3-UniV3SwapParams"></a>

### UniV3SwapParams
Represents swap parameters for UniswapV3


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated | Array of addresses dictating what swap path to follow |
| pool_fees | [uint32](#uint32) | repeated | Array of pool fees dictating what swap pools to use |
| amount | [string](#string) |  | Amount of the first asset in the path to swap |
| amount_out_min | [string](#string) |  | The minimum amount of the last asset in the path to receive |





 


<a name="steward-v3-Exchange"></a>

### Exchange
Exchange selector

| Name | Number | Description |
| ---- | ------ | ----------- |
| EXCHANGE_UNSPECIFIED | 0 |  |
| EXCHANGE_UNIV2 | 1 | Represents Uniswap V2 |
| EXCHANGE_UNIV3 | 2 | Represents Uniswap V3 |


 

 

 



<a name="governance-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## governance.proto



<a name="steward-v3-GovernanceCall"></a>

### GovernanceCall
Represents a governance-executed cellar function call. Used for Scheduled Cork Proposals in Sommelier.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| aave_v2_stablecoin | [AaveV2StablecoinGovernance](#steward-v3-AaveV2StablecoinGovernance) |  | Governance function calls to the AaveV2Stablecoin cellar |
| cellar_v1 | [CellarV1Governance](#steward-v3-CellarV1Governance) |  | Governance function calls to V1 cellars |
| cellar_v2 | [CellarV2Governance](#steward-v3-CellarV2Governance) |  | Governance function calls to V2 cellars |





 

 

 

 



<a name="steward-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## steward.proto



<a name="steward-v3-ScheduleRequest"></a>

### ScheduleRequest
Represents a single, scheduled function call to a particular Cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar_id | [string](#string) |  | The ID (currently simply an Ethereum address) of the target Cellar |
| block_height | [uint64](#uint64) |  | The block height at which to schedule the contract call |
| aave_v2_stablecoin | [AaveV2Stablecoin](#steward-v3-AaveV2Stablecoin) |  |  |
| cellar_v1 | [CellarV1](#steward-v3-CellarV1) |  |  |
| cellar_v2 | [CellarV2](#steward-v3-CellarV2) |  |  |






<a name="steward-v3-ScheduleResponse"></a>

### ScheduleResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  | The hex encoded ID of the scheduled cork |





 

 

 


<a name="steward-v3-ContractCall"></a>

### ContractCall
Service for handling Cellar contract calls

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Schedule | [ScheduleRequest](#steward-v3-ScheduleRequest) | [ScheduleResponse](#steward-v3-ScheduleResponse) | Handles scheduled contract call submission |

 



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

