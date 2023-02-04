# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/base.proto](#adaptors_base-proto)
    - [OracleSwap](#steward-v3-OracleSwap)
    - [RevokeApproval](#steward-v3-RevokeApproval)
    - [Swap](#steward-v3-Swap)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_base-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/base.proto



<a name="steward-v3-OracleSwap"></a>

### OracleSwap
Helper function to make safe &#34;blind&#34; Uniswap Swaps by comparing value in vs value out of the swap.

Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset_in | [string](#string) |  | Asset to swap from |
| asset_out | [string](#string) |  | Asset to swap to |
| amount_in | [string](#string) |  | Amount to swap |
| exchange | [Exchange](#steward-v3-Exchange) |  | The exchange to make the swap on |
| params | [OracleSwapParams](#steward-v3-OracleSwapParams) |  | The parameters for the swap |
| slippage | [uint64](#uint64) |  | The slippage allowed for the swap |






<a name="steward-v3-RevokeApproval"></a>

### RevokeApproval
Allows strategists to zero out an approval for a given `asset`.

Represents function `revokeApproval(ERC20 asset, address spender)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset | [string](#string) |  | ERC20 Asset to revoke spender&#39;s approval for |
| spender | [string](#string) |  | The spender to revoke approval of asset for |






<a name="steward-v3-Swap"></a>

### Swap
Helper function that allows swaps using the Swap Router

Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset_in | [string](#string) |  | Asset to swap from |
| asset_out | [string](#string) |  | Asset to swap to |
| amount_in | [string](#string) |  | Amount to swap |
| exchange | [Exchange](#steward-v3-Exchange) |  | The exchange to make the swap on |
| params | [SwapParams](#steward-v3-SwapParams) |  | The parameters for the swap |





 

 

 

 



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

