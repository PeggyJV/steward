# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/aave/a_token.proto](#adaptors_aave_a_token-proto)
    - [AaveATokenAdaptorV1](#steward-v3-AaveATokenAdaptorV1)
    - [AaveATokenAdaptorV1.DepositToAave](#steward-v3-AaveATokenAdaptorV1-DepositToAave)
    - [AaveATokenAdaptorV1.WithdrawFromAave](#steward-v3-AaveATokenAdaptorV1-WithdrawFromAave)
    - [AaveATokenAdaptorV1Calls](#steward-v3-AaveATokenAdaptorV1Calls)
    - [AaveATokenAdaptorV2](#steward-v3-AaveATokenAdaptorV2)
    - [AaveATokenAdaptorV2.DepositToAave](#steward-v3-AaveATokenAdaptorV2-DepositToAave)
    - [AaveATokenAdaptorV2.WithdrawFromAave](#steward-v3-AaveATokenAdaptorV2-WithdrawFromAave)
    - [AaveATokenAdaptorV2Calls](#steward-v3-AaveATokenAdaptorV2Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_aave_a_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/aave/a_token.proto



<a name="steward-v3-AaveATokenAdaptorV1"></a>

### AaveATokenAdaptorV1
Represents call data for the Aave AToken adaptor V1, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave | [AaveATokenAdaptorV1.DepositToAave](#steward-v3-AaveATokenAdaptorV1-DepositToAave) |  | Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave | [AaveATokenAdaptorV1.WithdrawFromAave](#steward-v3-AaveATokenAdaptorV1-WithdrawFromAave) |  | Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |






<a name="steward-v3-AaveATokenAdaptorV1-DepositToAave"></a>

### AaveATokenAdaptorV1.DepositToAave
Allows strategists to lend assets on Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to deposit |
| amount | [string](#string) |  | The amount to deposit |






<a name="steward-v3-AaveATokenAdaptorV1-WithdrawFromAave"></a>

### AaveATokenAdaptorV1.WithdrawFromAave
Allows strategists to withdraw assets from Aave.

Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to withdraw |
| amount | [string](#string) |  | The amount to withdraw |






<a name="steward-v3-AaveATokenAdaptorV1Calls"></a>

### AaveATokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveATokenAdaptorV1](#steward-v3-AaveATokenAdaptorV1) | repeated |  |






<a name="steward-v3-AaveATokenAdaptorV2"></a>

### AaveATokenAdaptorV2
Represents call data for the Aave AToken adaptor V2, used to manage lending positions on Aave


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_aave | [AaveATokenAdaptorV2.DepositToAave](#steward-v3-AaveATokenAdaptorV2-DepositToAave) |  | Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)` |
| withdraw_from_aave | [AaveATokenAdaptorV2.WithdrawFromAave](#steward-v3-AaveATokenAdaptorV2-WithdrawFromAave) |  | Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)` |






<a name="steward-v3-AaveATokenAdaptorV2-DepositToAave"></a>

### AaveATokenAdaptorV2.DepositToAave
Allows strategists to lend assets on Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to deposit |
| amount | [string](#string) |  | The amount to deposit |






<a name="steward-v3-AaveATokenAdaptorV2-WithdrawFromAave"></a>

### AaveATokenAdaptorV2.WithdrawFromAave
Allows strategists to withdraw assets from Aave.

Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to withdraw |
| amount | [string](#string) |  | The amount to withdraw |






<a name="steward-v3-AaveATokenAdaptorV2Calls"></a>

### AaveATokenAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveATokenAdaptorV2](#steward-v3-AaveATokenAdaptorV2) | repeated |  |





 

 

 

 



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

