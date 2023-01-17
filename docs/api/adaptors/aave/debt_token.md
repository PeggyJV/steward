# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/aave/debt_token.proto](#adaptors_aave_debt_token-proto)
    - [AaveDebtTokenAdaptor](#steward-v3-AaveDebtTokenAdaptor)
    - [AaveDebtTokenAdaptor.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptor-BorrowFromAave)
    - [AaveDebtTokenAdaptor.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptor-RepayAaveDebt)
    - [AaveDebtTokenAdaptor.SwapAndRepay](#steward-v3-AaveDebtTokenAdaptor-SwapAndRepay)
    - [AaveDebtTokenAdaptorCalls](#steward-v3-AaveDebtTokenAdaptorCalls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_aave_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/aave/debt_token.proto



<a name="steward-v3-AaveDebtTokenAdaptor"></a>

### AaveDebtTokenAdaptor
Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveDebtTokenAdaptor.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptor-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveDebtTokenAdaptor.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptor-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |
| swap_and_repay | [AaveDebtTokenAdaptor.SwapAndRepay](#steward-v3-AaveDebtTokenAdaptor-SwapAndRepay) |  | Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |






<a name="steward-v3-AaveDebtTokenAdaptor-BorrowFromAave"></a>

### AaveDebtTokenAdaptor.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v3-AaveDebtTokenAdaptor-RepayAaveDebt"></a>

### AaveDebtTokenAdaptor.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v3-AaveDebtTokenAdaptor-SwapAndRepay"></a>

### AaveDebtTokenAdaptor.SwapAndRepay
Allows strategists to swap assets and repay loans in one call.

Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  | The address of the token to swap from |
| token_to_repay | [string](#string) |  | The address of the token to swap to and repay with |
| amount_in | [string](#string) |  | The amount to swap |
| exchange | [Exchange](#steward-v3-Exchange) |  | The exchange to make the swap on |
| params | [SwapParams](#steward-v3-SwapParams) |  | The parameters for the swap |






<a name="steward-v3-AaveDebtTokenAdaptorCalls"></a>

### AaveDebtTokenAdaptorCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveDebtTokenAdaptor](#steward-v3-AaveDebtTokenAdaptor) | repeated |  |





 

 

 

 



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

