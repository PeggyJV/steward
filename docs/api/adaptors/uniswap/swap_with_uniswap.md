# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/uniswap/swap_with_uniswap.proto](#adaptors_uniswap_swap_with_uniswap-proto)
    - [SwapWithUniswapAdaptorV1](#steward-v3-SwapWithUniswapAdaptorV1)
    - [SwapWithUniswapAdaptorV1.SwapWithUniV2](#steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV2)
    - [SwapWithUniswapAdaptorV1.SwapWithUniV3](#steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV3)
    - [SwapWithUniswapAdaptorV1Calls](#steward-v3-SwapWithUniswapAdaptorV1Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_uniswap_swap_with_uniswap-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/uniswap/swap_with_uniswap.proto



<a name="steward-v3-SwapWithUniswapAdaptorV1"></a>

### SwapWithUniswapAdaptorV1
Represents call data for the Uniswap V3 adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| swap_with_uni_v2 | [SwapWithUniswapAdaptorV1.SwapWithUniV2](#steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV2) |  | Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)` |
| swap_with_uni_v3 | [SwapWithUniswapAdaptorV1.SwapWithUniV3](#steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV3) |  | Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)` |






<a name="steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV2"></a>

### SwapWithUniswapAdaptorV1.SwapWithUniV2
Perform a swap using Uniswap V2.

Represents function `swapWithUniV2(address[] path, uint256 amount, uint256 amountOutMin)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated |  |
| amount | [string](#string) |  |  |
| amount_out_min | [string](#string) |  |  |






<a name="steward-v3-SwapWithUniswapAdaptorV1-SwapWithUniV3"></a>

### SwapWithUniswapAdaptorV1.SwapWithUniV3
Perform a swap using Uniswap V3.

Represents function `Represents function `swapWithUniV3(address[] path, uint24[] poolFees, uint256 amount, uint256 amountOutMin)``


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| path | [string](#string) | repeated |  |
| pool_fees | [uint32](#uint32) | repeated |  |
| amount | [string](#string) |  |  |
| amount_out_min | [string](#string) |  |  |






<a name="steward-v3-SwapWithUniswapAdaptorV1Calls"></a>

### SwapWithUniswapAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [SwapWithUniswapAdaptorV1](#steward-v3-SwapWithUniswapAdaptorV1) | repeated |  |





 

 

 

 



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

