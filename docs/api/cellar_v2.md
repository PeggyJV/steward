# Protocol Documentation
<a name="top"></a>

## Table of Contents

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
  
- [Scalar Value Types](#scalar-value-types)



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

