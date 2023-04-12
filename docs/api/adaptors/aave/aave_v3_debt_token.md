# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/aave/aave_v3_debt_token.proto](#adaptors_aave_aave_v3_debt_token-proto)
    - [AaveV3DebtTokenAdaptorV1](#steward-v3-AaveV3DebtTokenAdaptorV1)
    - [AaveV3DebtTokenAdaptorV1.AdaptorCallForAaveV3Flashloan](#steward-v3-AaveV3DebtTokenAdaptorV1-AdaptorCallForAaveV3Flashloan)
    - [AaveV3DebtTokenAdaptorV1.BorrowFromAave](#steward-v3-AaveV3DebtTokenAdaptorV1-BorrowFromAave)
    - [AaveV3DebtTokenAdaptorV1.FlashLoan](#steward-v3-AaveV3DebtTokenAdaptorV1-FlashLoan)
    - [AaveV3DebtTokenAdaptorV1.RepayAaveDebt](#steward-v3-AaveV3DebtTokenAdaptorV1-RepayAaveDebt)
    - [AaveV3DebtTokenAdaptorV1.RepayWithATokens](#steward-v3-AaveV3DebtTokenAdaptorV1-RepayWithATokens)
    - [AaveV3DebtTokenAdaptorV1Calls](#steward-v3-AaveV3DebtTokenAdaptorV1Calls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_aave_aave_v3_debt_token-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/aave/aave_v3_debt_token.proto



<a name="steward-v3-AaveV3DebtTokenAdaptorV1"></a>

### AaveV3DebtTokenAdaptorV1
Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| borrow_from_aave | [AaveV3DebtTokenAdaptorV1.BorrowFromAave](#steward-v3-AaveV3DebtTokenAdaptorV1-BorrowFromAave) |  | Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)` |
| repay_aave_debt | [AaveV3DebtTokenAdaptorV1.RepayAaveDebt](#steward-v3-AaveV3DebtTokenAdaptorV1-RepayAaveDebt) |  | Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)` |
| repay_with_a_tokens | [AaveV3DebtTokenAdaptorV1.RepayWithATokens](#steward-v3-AaveV3DebtTokenAdaptorV1-RepayWithATokens) |  | Represents function `repayWithATokens(ERC20 underlying, uint256 amount)` |
| flash_loan | [AaveV3DebtTokenAdaptorV1.FlashLoan](#steward-v3-AaveV3DebtTokenAdaptorV1-FlashLoan) |  | Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)` |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1-AdaptorCallForAaveV3Flashloan"></a>

### AaveV3DebtTokenAdaptorV1.AdaptorCallForAaveV3Flashloan
Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| adaptor | [string](#string) |  | Address of the adaptor |
| uniswap_v3_v1_calls | [UniswapV3AdaptorV1Calls](#steward-v3-UniswapV3AdaptorV1Calls) |  | Represents function calls to the UniswapV3Adaptor V1 |
| aave_a_token_v1_calls | [AaveATokenAdaptorV1Calls](#steward-v3-AaveATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenAdaptor V1 |
| aave_debt_token_v1_calls | [AaveDebtTokenAdaptorV1Calls](#steward-v3-AaveDebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenAdaptor V1 |
| compound_c_token_v2_calls | [CompoundCTokenAdaptorV2Calls](#steward-v3-CompoundCTokenAdaptorV2Calls) |  | Represents function calls to the CompoundCTokenAdaptor V2 |
| aave_a_token_v2_calls | [AaveATokenAdaptorV2Calls](#steward-v3-AaveATokenAdaptorV2Calls) |  | Represents function calls to the AaveATokenV2Adaptor |
| aave_debt_token_v2_calls | [AaveDebtTokenAdaptorV2Calls](#steward-v3-AaveDebtTokenAdaptorV2Calls) |  | Represents function calls to the AavaDebtTokenV2Adaptor |
| aave_v3_a_token_v1_calls | [AaveV3ATokenAdaptorV1Calls](#steward-v3-AaveV3ATokenAdaptorV1Calls) |  | Represents function calls to the AaveATokenV1Adaptor |
| aave_v3_debt_token_v1_calls | [AaveV3DebtTokenAdaptorV1Calls](#steward-v3-AaveV3DebtTokenAdaptorV1Calls) |  | Represents function calls to the AavaDebtTokenV1Adaptor |
| one_inch_v1_calls | [OneInchAdaptorV1Calls](#steward-v3-OneInchAdaptorV1Calls) |  | Represents function calls to the OneInchAdaptorV1 |
| fees_and_reserves_v1_calls | [FeesAndReservesAdaptorV1Calls](#steward-v3-FeesAndReservesAdaptorV1Calls) |  | Represents function calls to the FeesAndReservesAdaptorV1 |
| zero_x_v1_calls | [ZeroXAdaptorV1Calls](#steward-v3-ZeroXAdaptorV1Calls) |  | Represents functionc alls to the ZeroXAdaptorV1 |
| swap_with_uniswap_v1_calls | [SwapWithUniswapAdaptorV1Calls](#steward-v3-SwapWithUniswapAdaptorV1Calls) |  | Represents function calls to the SwapWithUniswapAdaptorV1 |
| vesting_simple_v2_calls | [VestingSimpleAdaptorV2Calls](#steward-v3-VestingSimpleAdaptorV2Calls) |  | Represents function calls to VestingSimpleAdaptor |
| cellar_v1_calls | [CellarAdaptorV1Calls](#steward-v3-CellarAdaptorV1Calls) |  | Represents function calls to the CellarAdaptor |
| uniswap_v3_v2_calls | [UniswapV3AdaptorV2Calls](#steward-v3-UniswapV3AdaptorV2Calls) |  | Represents function calls to the UniswapV3Adaptor V2 |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1-BorrowFromAave"></a>

### AaveV3DebtTokenAdaptorV1.BorrowFromAave
Allows strategists to borrow assets from Aave.

Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to borrow |
| amount | [string](#string) |  | The amount to borrow |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1-FlashLoan"></a>

### AaveV3DebtTokenAdaptorV1.FlashLoan
Allows strategists to have Cellars take out flash loans

Represents function `flashLoan(address[] loanToken, uint256[] loanAmount, bytes params)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| loan_tokens | [string](#string) | repeated | The addresses of the ERC20 tokens to borrow |
| loan_amounts | [string](#string) | repeated | The amounts to borrow |
| params | [AaveV3DebtTokenAdaptorV1.AdaptorCallForAaveV3Flashloan](#steward-v3-AaveV3DebtTokenAdaptorV1-AdaptorCallForAaveV3Flashloan) | repeated | The params to pass to the flash loan callback. |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1-RepayAaveDebt"></a>

### AaveV3DebtTokenAdaptorV1.RepayAaveDebt
Allows strategists to repay loan debt on Aave.

Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1-RepayWithATokens"></a>

### AaveV3DebtTokenAdaptorV1.RepayWithATokens
Allows strategist to use aTokens to repay debt tokens with the same underlying.

Represents function `repayWithATokens(ERC20 underlying, uint256 amount)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| underlying_token | [string](#string) |  | The address of the underlying ERC20 token to repay |
| amount | [string](#string) |  | The amount to repay |






<a name="steward-v3-AaveV3DebtTokenAdaptorV1Calls"></a>

### AaveV3DebtTokenAdaptorV1Calls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [AaveV3DebtTokenAdaptorV1](#steward-v3-AaveV3DebtTokenAdaptorV1) | repeated |  |





 

 

 

 



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

