# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [cellar_v2.proto](#cellar_v2-proto)
    - [AdaptorCall](#steward-v3-AdaptorCall)
    - [CellarV2](#steward-v3-CellarV2)
    - [CellarV2.AddPosition](#steward-v3-CellarV2-AddPosition)
    - [CellarV2.CallOnAdaptor](#steward-v3-CellarV2-CallOnAdaptor)
    - [CellarV2.InitiateShutdown](#steward-v3-CellarV2-InitiateShutdown)
    - [CellarV2.LiftShutdown](#steward-v3-CellarV2-LiftShutdown)
    - [CellarV2.RemovePosition](#steward-v3-CellarV2-RemovePosition)
    - [CellarV2.SetHoldingPosition](#steward-v3-CellarV2-SetHoldingPosition)
    - [CellarV2.SetPlatformFee](#steward-v3-CellarV2-SetPlatformFee)
    - [CellarV2.SetRebalanceDeviation](#steward-v3-CellarV2-SetRebalanceDeviation)
    - [CellarV2.SetShareLockPeriod](#steward-v3-CellarV2-SetShareLockPeriod)
    - [CellarV2.SetStrategistPayoutAddress](#steward-v3-CellarV2-SetStrategistPayoutAddress)
    - [CellarV2.SetupAdaptor](#steward-v3-CellarV2-SetupAdaptor)
    - [CellarV2.SwapPositions](#steward-v3-CellarV2-SwapPositions)
    - [CellarV2_2](#steward-v3-CellarV2_2)
    - [CellarV2_2.AddAdaptorToCatalogue](#steward-v3-CellarV2_2-AddAdaptorToCatalogue)
    - [CellarV2_2.AddPosition](#steward-v3-CellarV2_2-AddPosition)
    - [CellarV2_2.AddPositionToCatalogue](#steward-v3-CellarV2_2-AddPositionToCatalogue)
    - [CellarV2_2.CallOnAdaptor](#steward-v3-CellarV2_2-CallOnAdaptor)
    - [CellarV2_2.FunctionCall](#steward-v3-CellarV2_2-FunctionCall)
    - [CellarV2_2.InitiateShutdown](#steward-v3-CellarV2_2-InitiateShutdown)
    - [CellarV2_2.LiftShutdown](#steward-v3-CellarV2_2-LiftShutdown)
    - [CellarV2_2.Multicall](#steward-v3-CellarV2_2-Multicall)
    - [CellarV2_2.RemovePosition](#steward-v3-CellarV2_2-RemovePosition)
    - [CellarV2_2.SetHoldingPosition](#steward-v3-CellarV2_2-SetHoldingPosition)
    - [CellarV2_2.SetRebalanceDeviation](#steward-v3-CellarV2_2-SetRebalanceDeviation)
    - [CellarV2_2.SetShareLockPeriod](#steward-v3-CellarV2_2-SetShareLockPeriod)
    - [CellarV2_2.SetStrategistPayoutAddress](#steward-v3-CellarV2_2-SetStrategistPayoutAddress)
    - [CellarV2_2.SetStrategistPlatformCut](#steward-v3-CellarV2_2-SetStrategistPlatformCut)
    - [CellarV2_2.SwapPositions](#steward-v3-CellarV2_2-SwapPositions)
  
- [Scalar Value Types](#scalar-value-types)



<a name="cellar_v2-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## cellar_v2.proto



<a name="steward-v3-AdaptorCall"></a>

### AdaptorCall
Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| uniswap_v3_v1_calls | [UniswapV3AdaptorV1Calls](#steward-v3-UniswapV3AdaptorV1Calls) |  | Represents function calls to the UniswapV3Adaptor V1 |
| aave_a_token_v1_calls | [AaveATokenAdaptorV1Calls](#steward-v3-AaveATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenAdaptor V1 |
| aave_debt_token_v1_calls | [AaveDebtTokenAdaptorV1Calls](#steward-v3-AaveDebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenAdaptor V1 |
| compound_c_token_v2_calls | [CompoundCTokenAdaptorV2Calls](#steward-v3-CompoundCTokenAdaptorV2Calls) |  | Represents function calls to the CompoundCTokenAdaptor V2 |
| aave_a_token_v2_calls | [AaveATokenAdaptorV2Calls](#steward-v3-AaveATokenAdaptorV2Calls) |  | Represents function calls to the AaveATokenV2Adaptor |
| aave_debt_token_v2_calls | [AaveDebtTokenAdaptorV2Calls](#steward-v3-AaveDebtTokenAdaptorV2Calls) |  | Represents function calls to the AavaDebtTokenV2Adaptor |
| aave_v3_a_token_v1_calls | [AaveV3ATokenAdaptorV1Calls](#steward-v3-AaveV3ATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenV1Adaptor |
| aave_v3_debt_token_v1_calls | [AaveV3DebtTokenAdaptorV1Calls](#steward-v3-AaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenV1Adaptor |
| one_inch_v1_calls | [OneInchAdaptorV1Calls](#steward-v3-OneInchAdaptorV1Calls) |  | Represents function calls to the OneInchAdaptorV1 |
| fees_and_reserves_v1_calls | [FeesAndReservesAdaptorV1Calls](#steward-v3-FeesAndReservesAdaptorV1Calls) |  | Represents function calls to the FeesAndReservesAdaptorV1 |
| zero_x_v1_calls | [ZeroXAdaptorV1Calls](#steward-v3-ZeroXAdaptorV1Calls) |  | Represents functionc alls to the ZeroXAdaptorV1 |
| swap_with_uniswap_v1_calls | [SwapWithUniswapAdaptorV1Calls](#steward-v3-SwapWithUniswapAdaptorV1Calls) |  | Represents function calls to the SwapWithUniswapAdaptorV1 |
| vesting_simple_v2_calls | [VestingSimpleAdaptorV2Calls](#steward-v3-VestingSimpleAdaptorV2Calls) |  | Represents function calls to VestingSimpleAdaptor |
| cellar_v1_calls | [CellarAdaptorV1Calls](#steward-v3-CellarAdaptorV1Calls) |  | Represents function calls to the CellarAdaptor |
| uniswap_v3_v2_calls | [UniswapV3AdaptorV2Calls](#steward-v3-UniswapV3AdaptorV2Calls) |  | Represents function calls to the UniswapV3Adaptor V2 |
| aave_v2_enable_asset_as_collateral_v1_calls | [AaveV2EnableAssetAsCollateralAdaptorV1Calls](#steward-v3-AaveV2EnableAssetAsCollateralAdaptorV1Calls) |  | Represents function calls to the AaveV2EnableAssetAsCollatorAdaptor V1 |
| f_token_v1_calls | [FTokenAdaptorV1Calls](#steward-v3-FTokenAdaptorV1Calls) |  | Represents function calls to the FTokenAdaptor V1 |
| morpho_aave_v2_a_token_v1_calls | [MorphoAaveV2ATokenAdaptorV1Calls](#steward-v3-MorphoAaveV2ATokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2AToken V1 |
| morpho_aave_v2_debt_token_v1_calls | [MorphoAaveV2DebtTokenAdaptorV1Calls](#steward-v3-MorphoAaveV2DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV2DebtToken V1 |
| morpho_aave_v3_a_token_collateral_v1_calls | [MorphoAaveV3ATokenCollateralAdaptorV1Calls](#steward-v3-MorphoAaveV3ATokenCollateralAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenCollateral V1 |
| morpho_aave_v3_a_token_p2p_v1_calls | [MorphoAaveV3ATokenP2PAdaptorV1Calls](#steward-v3-MorphoAaveV3ATokenP2PAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3ATokenP2P V1 |
| morpho_aave_v3_debt_token_v1_calls | [MorphoAaveV3DebtTokenAdaptorV1Calls](#steward-v3-MorphoAaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the MorphoAaveV3DebtToken V1 |






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
| setup_adaptor | [CellarV2.SetupAdaptor](#steward-v3-CellarV2-SetupAdaptor) |  | Represents function `setupAdaptor(address _adaptor)` |
| set_platform_fee | [CellarV2.SetPlatformFee](#steward-v3-CellarV2-SetPlatformFee) |  | Represents function `setPlatformFee(uint64 platformFee)` |
| initiate_shutdown | [CellarV2.InitiateShutdown](#steward-v3-CellarV2-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| lift_shutdown | [CellarV2.LiftShutdown](#steward-v3-CellarV2-LiftShutdown) |  | Represents function `liftShutdown()` |






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
| data | [AdaptorCall](#steward-v3-AdaptorCall) | repeated |  |






<a name="steward-v3-CellarV2-InitiateShutdown"></a>

### CellarV2.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v3-CellarV2-LiftShutdown"></a>

### CellarV2.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






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

Represents function `setHoldingPosition(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v3-CellarV2-SetPlatformFee"></a>

### CellarV2.SetPlatformFee
Allows owner to set the platform fee.

Represents function `setPlatformFee(uint64 platformFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| platform_fee | [uint64](#uint64) |  |  |






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

Represents function `setShareLockPeriod(uint256 newLock)`


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






<a name="steward-v3-CellarV2-SetupAdaptor"></a>

### CellarV2.SetupAdaptor
Allows owner to add new adaptors for the cellar to use.

Represents function `setupAdaptor(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |






<a name="steward-v3-CellarV2-SwapPositions"></a>

### CellarV2.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |






<a name="steward-v3-CellarV2_2"></a>

### CellarV2_2



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_call | [CellarV2_2.FunctionCall](#steward-v3-CellarV2_2-FunctionCall) |  | Represents a single function call |
| multicall | [CellarV2_2.Multicall](#steward-v3-CellarV2_2-Multicall) |  | Represents multiple, ordered function calls |






<a name="steward-v3-CellarV2_2-AddAdaptorToCatalogue"></a>

### CellarV2_2.AddAdaptorToCatalogue
Allows the owner to add an adaptor to the Cellar&#39;s adaptor catalogue

Represents function `addAdaptorToCatalogue(address adaptor)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  |  |






<a name="steward-v3-CellarV2_2-AddPosition"></a>

### CellarV2_2.AddPosition
Insert a trusted position to the list of positions used by the cellar at a given index.

Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to add the position |
| position_id | [uint32](#uint32) |  | The position&#39;s ID in the cellar registry |
| configuration_data | [bytes](#bytes) |  | Data used to configure how the position behaves |
| in_debt_array | [bool](#bool) |  | Whether to add position in the debt array, or the credit array. |






<a name="steward-v3-CellarV2_2-AddPositionToCatalogue"></a>

### CellarV2_2.AddPositionToCatalogue
Allows the owner to add a position to the Cellar&#39;s position catalogue

Represents function `addPositionToCatalogue(uint32 positionId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  |  |






<a name="steward-v3-CellarV2_2-CallOnAdaptor"></a>

### CellarV2_2.CallOnAdaptor
Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.

Represents function `callOnAdaptor(AdaptorCall[] memory data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [AdaptorCall](#steward-v3-AdaptorCall) | repeated |  |






<a name="steward-v3-CellarV2_2-FunctionCall"></a>

### CellarV2_2.FunctionCall
The function you wish to execute on the target cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| add_position | [CellarV2_2.AddPosition](#steward-v3-CellarV2_2-AddPosition) |  | Represents function `addPosition(uint256 index, address position)` |
| call_on_adaptor | [CellarV2_2.CallOnAdaptor](#steward-v3-CellarV2_2-CallOnAdaptor) |  | Represents function `callOnAdaptor(AdaptorCall[] memory data)` |
| remove_position | [CellarV2_2.RemovePosition](#steward-v3-CellarV2_2-RemovePosition) |  | Represents function `removePosition(uint256 index)` |
| set_holding_position | [CellarV2_2.SetHoldingPosition](#steward-v3-CellarV2_2-SetHoldingPosition) |  | Represents function `setHoldingPosition(uint32 position_id)` |
| set_strategist_payout_address | [CellarV2_2.SetStrategistPayoutAddress](#steward-v3-CellarV2_2-SetStrategistPayoutAddress) |  | Represents function `setStrategistPayoutAddress(address payout)` |
| swap_positions | [CellarV2_2.SwapPositions](#steward-v3-CellarV2_2-SwapPositions) |  | Represents function `swapPositions(uint256 index1, uint256 index2)` |
| set_rebalance_deviation | [CellarV2_2.SetRebalanceDeviation](#steward-v3-CellarV2_2-SetRebalanceDeviation) |  | Represents function `setRebalanceDeviation(uint265)` |
| set_share_lock_period | [CellarV2_2.SetShareLockPeriod](#steward-v3-CellarV2_2-SetShareLockPeriod) |  | Represents function `setShareLockPeriod(uint256 newLock)` |
| initiate_shutdown | [CellarV2_2.InitiateShutdown](#steward-v3-CellarV2_2-InitiateShutdown) |  | Represents function `initiateShutdown()` |
| set_strategist_platform_cut | [CellarV2_2.SetStrategistPlatformCut](#steward-v3-CellarV2_2-SetStrategistPlatformCut) |  | Represents function `setStrategistPlatformCut(uint64 cut)` |
| lift_shutdown | [CellarV2_2.LiftShutdown](#steward-v3-CellarV2_2-LiftShutdown) |  | Represents function `liftShutdown()` |
| add_adaptor_to_catalogue | [CellarV2_2.AddAdaptorToCatalogue](#steward-v3-CellarV2_2-AddAdaptorToCatalogue) |  | Represents function `addAdaptorToCatalogue(address adaptor)` |
| add_position_to_catalogue | [CellarV2_2.AddPositionToCatalogue](#steward-v3-CellarV2_2-AddPositionToCatalogue) |  | Represents function `addPositionToCatalogue(uint32 positionId)` |






<a name="steward-v3-CellarV2_2-InitiateShutdown"></a>

### CellarV2_2.InitiateShutdown
Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.

Represents function `initiateShutdown()`






<a name="steward-v3-CellarV2_2-LiftShutdown"></a>

### CellarV2_2.LiftShutdown
Allows the owner to restart a shut down Cellar

Represents function `liftShutdown()`






<a name="steward-v3-CellarV2_2-Multicall"></a>

### CellarV2_2.Multicall
Allows caller to call multiple functions in a single TX.

Represents function `multicall(bytes[] data)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| function_calls | [CellarV2_2.FunctionCall](#steward-v3-CellarV2_2-FunctionCall) | repeated |  |






<a name="steward-v3-CellarV2_2-RemovePosition"></a>

### CellarV2_2.RemovePosition
Remove the position at a given index from the list of positions used by the cellar.

Represents function `removePosition(uint32 index, bool inDebtArray)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index | [uint32](#uint32) |  | Index at which to remove the position |
| in_debt_array | [bool](#bool) |  | Whether to remove position from the debt array, or the credit array. |






<a name="steward-v3-CellarV2_2-SetHoldingPosition"></a>

### CellarV2_2.SetHoldingPosition
Set the holding position used of the cellar.

Represents function `setHoldingIndex(uint8 index)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| position_id | [uint32](#uint32) |  | ID (index) of the new holding position to use |






<a name="steward-v3-CellarV2_2-SetRebalanceDeviation"></a>

### CellarV2_2.SetRebalanceDeviation
Changes the cellar&#39;s allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.

Represents function `setRebalanceDeviation(uint256)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_deviation | [string](#string) |  |  |






<a name="steward-v3-CellarV2_2-SetShareLockPeriod"></a>

### CellarV2_2.SetShareLockPeriod
Allows share lock period to be updated.

Represents function `setShareLockPeriod()`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_lock | [string](#string) |  |  |






<a name="steward-v3-CellarV2_2-SetStrategistPayoutAddress"></a>

### CellarV2_2.SetStrategistPayoutAddress
Sets the Strategists payout address.

Represents function `setStrategistPayoutAddress(address payout)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| payout | [string](#string) |  |  |






<a name="steward-v3-CellarV2_2-SetStrategistPlatformCut"></a>

### CellarV2_2.SetStrategistPlatformCut
Allows strategist to set the platform cut for the cellar.

Represents function `setStrategistPlatformCut(uint64 cut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_cut | [uint64](#uint64) |  | The new strategist platform cut |






<a name="steward-v3-CellarV2_2-SwapPositions"></a>

### CellarV2_2.SwapPositions
Swap the positions at two given indeces.

Represents function `swapPositions(uint32 index1, uint32 index2)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| index_1 | [uint32](#uint32) |  | Index of the first position |
| index_2 | [uint32](#uint32) |  | Index of the second position |
| in_debt_array | [bool](#bool) |  | Whether to switch positions in the debt array, or the credit array. |





 

 

 

 



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

