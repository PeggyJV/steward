# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/aave/debt_token.proto](#adaptors_aave_debt_token-proto)
    - [AaveDebtTokenAdaptorV1](#steward-v3-AaveDebtTokenAdaptorV1)
    - [AaveDebtTokenAdaptorV1.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptorV1-BorrowFromAave)
    - [AaveDebtTokenAdaptorV1.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptorV1-RepayAaveDebt)
    - [AaveDebtTokenAdaptorV1.SwapAndRepay](#steward-v3-AaveDebtTokenAdaptorV1-SwapAndRepay)
    - [AaveDebtTokenAdaptorV1Calls](#steward-v3-AaveDebtTokenAdaptorV1Calls)
    - [AaveDebtTokenAdaptorV2](#steward-v3-AaveDebtTokenAdaptorV2)
    - [AaveDebtTokenAdaptorV2.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptorV2-BorrowFromAave)
    - [AaveDebtTokenAdaptorV2.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptorV2-RepayAaveDebt)
    - [AaveDebtTokenAdaptorV2Calls](#steward-v3-AaveDebtTokenAdaptorV2Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_aave_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/aave/debt_token.proto



<a name="steward-v3-AaveDebtTokenAdaptorV1"></a>

### AaveDebtTokenAdaptorV1
Represents call data for the Aave Debt Token adaptor V1, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveDebtTokenAdaptorV1.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptorV1-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveDebtTokenAdaptorV1.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptorV1-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |
| swap_and_repay | [AaveDebtTokenAdaptorV1.SwapAndRepay](#steward-v3-AaveDebtTokenAdaptorV1-SwapAndRepay) |  | Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |






<a name="steward-v3-AaveDebtTokenAdaptorV1-BorrowFromAave"></a>

### AaveDebtTokenAdaptorV1.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v3-AaveDebtTokenAdaptorV1-RepayAaveDebt"></a>

### AaveDebtTokenAdaptorV1.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v3-AaveDebtTokenAdaptorV1-SwapAndRepay"></a>

### AaveDebtTokenAdaptorV1.SwapAndRepay
Allows strategists to swap assets and repay loans in one call.

Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_in | [string](#string) |  | The address of the token to swap from |
| token_to_repay | [string](#string) |  | The address of the token to swap to and repay with |
| amount_in | [string](#string) |  | The amount to swap |
| exchange | [Exchange](#steward-v3-Exchange) |  | The exchange to make the swap on |
| params | [SwapParams](#steward-v3-SwapParams) |  | The parameters for the swap |






<a name="steward-v3-AaveDebtTokenAdaptorV1Calls"></a>

### AaveDebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveDebtTokenAdaptorV1](#steward-v3-AaveDebtTokenAdaptorV1) | repeated |  |






<a name="steward-v3-AaveDebtTokenAdaptorV2"></a>

### AaveDebtTokenAdaptorV2
Represents call data for the Aave Debt Token adaptor V2, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveDebtTokenAdaptorV2.BorrowFromAave](#steward-v3-AaveDebtTokenAdaptorV2-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveDebtTokenAdaptorV2.RepayAaveDebt](#steward-v3-AaveDebtTokenAdaptorV2-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |






<a name="steward-v3-AaveDebtTokenAdaptorV2-BorrowFromAave"></a>

### AaveDebtTokenAdaptorV2.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v3-AaveDebtTokenAdaptorV2-RepayAaveDebt"></a>

### AaveDebtTokenAdaptorV2.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v3-AaveDebtTokenAdaptorV2Calls"></a>

### AaveDebtTokenAdaptorV2Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveDebtTokenAdaptorV2](#steward-v3-AaveDebtTokenAdaptorV2) | repeated |  |





 

 

 

 



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

