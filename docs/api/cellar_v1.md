# Protocol Documentation
<a name="top"></a>

## Table of Contents

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
  
- [Scalar Value Types](#scalar-value-types)



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

