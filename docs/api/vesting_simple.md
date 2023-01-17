# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/vesting_simple.proto](#adaptors_vesting_simple-proto)
    - [VestingSimpleAdaptor](#steward-v3-VestingSimpleAdaptor)
    - [VestingSimpleAdaptor.DepositToVesting](#steward-v3-VestingSimpleAdaptor-DepositToVesting)
    - [VestingSimpleAdaptor.WithdrawAllFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawAllFromVesting)
    - [VestingSimpleAdaptor.WithdrawAnyFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawAnyFromVesting)
    - [VestingSimpleAdaptor.WithdrawFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawFromVesting)
    - [VestingSimpleAdaptorCalls](#steward-v3-VestingSimpleAdaptorCalls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_vesting_simple-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/vesting_simple.proto



<a name="steward-v3-VestingSimpleAdaptor"></a>

### VestingSimpleAdaptor
Represents call data for the Vesting Simple adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| deposit_to_vesting | [VestingSimpleAdaptor.DepositToVesting](#steward-v3-VestingSimpleAdaptor-DepositToVesting) |  | Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)` |
| withdraw_from_vesting | [VestingSimpleAdaptor.WithdrawFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawFromVesting) |  | Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)` |
| withdraw_any_from_vesting | [VestingSimpleAdaptor.WithdrawAnyFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawAnyFromVesting) |  | Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)` |
| withdraw_all_from_vesting | [VestingSimpleAdaptor.WithdrawAllFromVesting](#steward-v3-VestingSimpleAdaptor-WithdrawAllFromVesting) |  | Represents function `withdrawAllFromVesting(VestingSimple vestingContract)` |






<a name="steward-v3-VestingSimpleAdaptor-DepositToVesting"></a>

### VestingSimpleAdaptor.DepositToVesting
Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
deposit its entire balance (appropriate in most cases).

Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v3-VestingSimpleAdaptor-WithdrawAllFromVesting"></a>

### VestingSimpleAdaptor.WithdrawAllFromVesting
Withdraw a certain amount of tokens from vesting, from any deposit. This will not affect the cellar&#39;s TVL because any deposit must
already have vested, and will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.

Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |






<a name="steward-v3-VestingSimpleAdaptor-WithdrawAnyFromVesting"></a>

### VestingSimpleAdaptor.WithdrawAnyFromVesting
Withdraw a single deposit from vesting. This will not affect the cellar&#39;s TVL because any deposit must already have vested, and
will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.

Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v3-VestingSimpleAdaptor-WithdrawFromVesting"></a>

### VestingSimpleAdaptor.WithdrawFromVesting
Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
deposit its entire balance (appropriate in most cases).

Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| vesting_contract | [string](#string) |  |  |
| deposit_id | [string](#string) |  |  |
| amount | [string](#string) |  |  |






<a name="steward-v3-VestingSimpleAdaptorCalls"></a>

### VestingSimpleAdaptorCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [VestingSimpleAdaptor](#steward-v3-VestingSimpleAdaptor) | repeated |  |





 

 

 

 



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

