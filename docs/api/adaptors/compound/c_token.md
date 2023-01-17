# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/compound/c_token.proto](#adaptors_compound_c_token-proto)
    - [CompoundCTokenAdaptor](#steward-v3-CompoundCTokenAdaptor)
    - [CompoundCTokenAdaptor.ClaimComp](#steward-v3-CompoundCTokenAdaptor-ClaimComp)
    - [CompoundCTokenAdaptor.ClaimCompAndSwap](#steward-v3-CompoundCTokenAdaptor-ClaimCompAndSwap)
    - [CompoundCTokenAdaptor.DepositToCompound](#steward-v3-CompoundCTokenAdaptor-DepositToCompound)
    - [CompoundCTokenAdaptor.WithdrawFromCompound](#steward-v3-CompoundCTokenAdaptor-WithdrawFromCompound)
    - [CompoundCTokenAdaptorCalls](#steward-v3-CompoundCTokenAdaptorCalls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_compound_c_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/compound/c_token.proto



<a name="steward-v3-CompoundCTokenAdaptor"></a>

### CompoundCTokenAdaptor
Represents call data for the Compound C Token adaptor, managing lending positions on Compound.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_compound | [CompoundCTokenAdaptor.DepositToCompound](#steward-v3-CompoundCTokenAdaptor-DepositToCompound) |  | Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)` |
| withdraw_from_compound | [CompoundCTokenAdaptor.WithdrawFromCompound](#steward-v3-CompoundCTokenAdaptor-WithdrawFromCompound) |  | Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)` |
| claim_comp | [CompoundCTokenAdaptor.ClaimComp](#steward-v3-CompoundCTokenAdaptor-ClaimComp) |  | Represents function `claimComp()` |
| claim_comp_and_swap | [CompoundCTokenAdaptor.ClaimCompAndSwap](#steward-v3-CompoundCTokenAdaptor-ClaimCompAndSwap) |  | Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |






<a name="steward-v3-CompoundCTokenAdaptor-ClaimComp"></a>

### CompoundCTokenAdaptor.ClaimComp
Allows strategists to claim COMP rewards.

Represents function `claimComp()`






<a name="steward-v3-CompoundCTokenAdaptor-ClaimCompAndSwap"></a>

### CompoundCTokenAdaptor.ClaimCompAndSwap
Allows strategists to claim COMP and immediately swap it using oracleSwap.

Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| asset_out | [string](#string) |  |  |
| exchange | [Exchange](#steward-v3-Exchange) |  |  |
| params | [OracleSwapParams](#steward-v3-OracleSwapParams) |  |  |
| slippage | [uint64](#uint64) |  |  |






<a name="steward-v3-CompoundCTokenAdaptor-DepositToCompound"></a>

### CompoundCTokenAdaptor.DepositToCompound
Allows strategists to lend assets on Compound.

Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  |  |
| amount_to_deposit | [string](#string) |  |  |






<a name="steward-v3-CompoundCTokenAdaptor-WithdrawFromCompound"></a>

### CompoundCTokenAdaptor.WithdrawFromCompound
Allows strategists to withdraw assets from Compound.

Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| market | [string](#string) |  |  |
| amount_to_withdraw | [string](#string) |  |  |






<a name="steward-v3-CompoundCTokenAdaptorCalls"></a>

### CompoundCTokenAdaptorCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [CompoundCTokenAdaptor](#steward-v3-CompoundCTokenAdaptor) | repeated |  |





 

 

 

 



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

