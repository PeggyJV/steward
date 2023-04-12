# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/fees_and_reserves.proto](#adaptors_fees_and_reserves-proto)
    - [FeesAndReservesAdaptorV1](#steward-v3-FeesAndReservesAdaptorV1)
    - [FeesAndReservesAdaptorV1.AddAssetsToReserves](#steward-v3-FeesAndReservesAdaptorV1-AddAssetsToReserves)
    - [FeesAndReservesAdaptorV1.ChangeUpkeepFrequency](#steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency)
    - [FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas](#steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas)
    - [FeesAndReservesAdaptorV1.PrepareFees](#steward-v3-FeesAndReservesAdaptorV1-PrepareFees)
    - [FeesAndReservesAdaptorV1.SetupMetaData](#steward-v3-FeesAndReservesAdaptorV1-SetupMetaData)
    - [FeesAndReservesAdaptorV1.UpdateManagementFees](#steward-v3-FeesAndReservesAdaptorV1-UpdateManagementFees)
    - [FeesAndReservesAdaptorV1.UpdatePerformanceFees](#steward-v3-FeesAndReservesAdaptorV1-UpdatePerformanceFees)
    - [FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves](#steward-v3-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves)
    - [FeesAndReservesAdaptorV1Calls](#steward-v3-FeesAndReservesAdaptorV1Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_fees_and_reserves-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/fees_and_reserves.proto



<a name="steward-v3-FeesAndReservesAdaptorV1"></a>

### FeesAndReservesAdaptorV1
Represents call data for the FeesAndReserves and FeesAndReservesAdaptor contracts.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| update_performance_fees | [FeesAndReservesAdaptorV1.UpdatePerformanceFees](#steward-v3-FeesAndReservesAdaptorV1-UpdatePerformanceFees) |  | Represents function `updatePerformanceFee(uint32 performanceFee)` |
| update_management_fees | [FeesAndReservesAdaptorV1.UpdateManagementFees](#steward-v3-FeesAndReservesAdaptorV1-UpdateManagementFees) |  | Represents function `updateManagementFee(uint32 managementFee)` |
| change_upkeep_frequency | [FeesAndReservesAdaptorV1.ChangeUpkeepFrequency](#steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency) |  | Represents function `changeUpkeepFrequency(uint64 newFrequency)` |
| change_upkeep_max_gas | [FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas](#steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas) |  | Represents function `changeUpkeepMaxGas(uint64 newMaxGas)` |
| setup_meta_data | [FeesAndReservesAdaptorV1.SetupMetaData](#steward-v3-FeesAndReservesAdaptorV1-SetupMetaData) |  | Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)` |
| add_assets_to_reserves | [FeesAndReservesAdaptorV1.AddAssetsToReserves](#steward-v3-FeesAndReservesAdaptorV1-AddAssetsToReserves) |  | Represents function `addAssetsToReserves(uint256 amount)` |
| withdraw_assets_from_reserves | [FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves](#steward-v3-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves) |  | Represents function `withdrawAssetsFromReserves(uint256 amount)` |
| prepare_fees | [FeesAndReservesAdaptorV1.PrepareFees](#steward-v3-FeesAndReservesAdaptorV1-PrepareFees) |  | Represents function `prepareFees(uint256 amount)` |






<a name="steward-v3-FeesAndReservesAdaptorV1-AddAssetsToReserves"></a>

### FeesAndReservesAdaptorV1.AddAssetsToReserves
Allows the owner to add assets to the Cellar&#39;s reserves

Represents function `addAssetsToReserves(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepFrequency"></a>

### FeesAndReservesAdaptorV1.ChangeUpkeepFrequency
Allows the owner to update a Cellar&#39;s upkeep frequency.

Represents function `changeUpkeepFrequency(uint64 newFrequency)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_frequency | [uint64](#uint64) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-ChangeUpkeepMaxGas"></a>

### FeesAndReservesAdaptorV1.ChangeUpkeepMaxGas
Allows the owner to update a Cellar&#39;s upkeep max gas.

Represents function `changeUpkeepMaxGas(uint64 newMaxGas)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_max_gas | [uint64](#uint64) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-PrepareFees"></a>

### FeesAndReservesAdaptorV1.PrepareFees
Allows the owner to prepare fees to be split between the platform, strategist, and protocol

Represents function `prepareFees(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-SetupMetaData"></a>

### FeesAndReservesAdaptorV1.SetupMetaData
Allows the owner to set the Cellar&#39;s fee metadata

Represents function `setupMetaData(uint32 managementFee, uint32 performanceFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| management_fee | [uint32](#uint32) |  |  |
| performance_fee | [uint32](#uint32) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-UpdateManagementFees"></a>

### FeesAndReservesAdaptorV1.UpdateManagementFees
Allows the owner to update a Cellar&#39;s management fee.

Represents function `updateManagementFee(uint32 managementFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| management_fee | [uint32](#uint32) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-UpdatePerformanceFees"></a>

### FeesAndReservesAdaptorV1.UpdatePerformanceFees
Allows the owner to update a Cellar&#39;s performance fee.

Represents function `updatePerformanceFee(uint32 performanceFee)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| performance_fee | [uint32](#uint32) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1-WithdrawAssetsFromReserves"></a>

### FeesAndReservesAdaptorV1.WithdrawAssetsFromReserves
Allows the owner to withdraw assets from the Cellar&#39;s reserves

Represents function `withdrawAssetsFromReserves(uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| amount | [string](#string) |  |  |






<a name="steward-v3-FeesAndReservesAdaptorV1Calls"></a>

### FeesAndReservesAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [FeesAndReservesAdaptorV1](#steward-v3-FeesAndReservesAdaptorV1) | repeated |  |





 

 

 

 



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

