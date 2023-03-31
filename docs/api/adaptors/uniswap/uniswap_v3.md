# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [adaptors/uniswap/uniswap_v3.proto](#adaptors_uniswap_uniswap_v3-proto)
    - [UniswapV3Adaptor](#steward-v3-UniswapV3Adaptor)
    - [UniswapV3Adaptor.AddToPosition](#steward-v3-UniswapV3Adaptor-AddToPosition)
    - [UniswapV3Adaptor.ClosePosition](#steward-v3-UniswapV3Adaptor-ClosePosition)
    - [UniswapV3Adaptor.CollectFees](#steward-v3-UniswapV3Adaptor-CollectFees)
    - [UniswapV3Adaptor.OpenPosition](#steward-v3-UniswapV3Adaptor-OpenPosition)
    - [UniswapV3Adaptor.PurgeAllZeroLiquidityPositions](#steward-v3-UniswapV3Adaptor-PurgeAllZeroLiquidityPositions)
    - [UniswapV3Adaptor.PurgeSinglePosition](#steward-v3-UniswapV3Adaptor-PurgeSinglePosition)
    - [UniswapV3Adaptor.RemoveUnownedPositionFromTracker](#steward-v3-UniswapV3Adaptor-RemoveUnownedPositionFromTracker)
    - [UniswapV3Adaptor.TakeFromPosition](#steward-v3-UniswapV3Adaptor-TakeFromPosition)
    - [UniswapV3AdaptorCalls](#steward-v3-UniswapV3AdaptorCalls)
  
- [Scalar Value Types](#scalar-value-types)



<a name="adaptors_uniswap_uniswap_v3-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## adaptors/uniswap/uniswap_v3.proto



<a name="steward-v3-UniswapV3Adaptor"></a>

### UniswapV3Adaptor
Represents call data for the Uniswap V3 adaptor


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| swap | [Swap](#steward-v3-Swap) |  | Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)` |
| oracle_swap | [OracleSwap](#steward-v3-OracleSwap) |  | Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)` |
| revoke_approval | [RevokeApproval](#steward-v3-RevokeApproval) |  | Represents function `revokeApproval(ERC20 asset, address spender)` |
| open_position | [UniswapV3Adaptor.OpenPosition](#steward-v3-UniswapV3Adaptor-OpenPosition) |  | Represents function `openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)` |
| close_position | [UniswapV3Adaptor.ClosePosition](#steward-v3-UniswapV3Adaptor-ClosePosition) |  | Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)` |
| add_to_position | [UniswapV3Adaptor.AddToPosition](#steward-v3-UniswapV3Adaptor-AddToPosition) |  | Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)` |
| take_from_position | [UniswapV3Adaptor.TakeFromPosition](#steward-v3-UniswapV3Adaptor-TakeFromPosition) |  | Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)` |
| collect_fees | [UniswapV3Adaptor.CollectFees](#steward-v3-UniswapV3Adaptor-CollectFees) |  | Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)` |
| purge_all_zero_liquidity_positions | [UniswapV3Adaptor.PurgeAllZeroLiquidityPositions](#steward-v3-UniswapV3Adaptor-PurgeAllZeroLiquidityPositions) |  | Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)` |
| purge_single_position | [UniswapV3Adaptor.PurgeSinglePosition](#steward-v3-UniswapV3Adaptor-PurgeSinglePosition) |  | Represents function `purgeSinglePosition(uint256 tokenId)` |
| remove_unowned_position_from_tracker | [UniswapV3Adaptor.RemoveUnownedPositionFromTracker](#steward-v3-UniswapV3Adaptor-RemoveUnownedPositionFromTracker) |  | Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)` |






<a name="steward-v3-UniswapV3Adaptor-AddToPosition"></a>

### UniswapV3Adaptor.AddToPosition
Allows strategist to add to existing Uniswap V3 positions.

Represents function `addToPosition(uint256 tokenId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-ClosePosition"></a>

### UniswapV3Adaptor.ClosePosition
Allows strategist to close Uniswap V3 positions.

Represents function `closePosition(uint256 tokenId, uint256 min0, uint256 min1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-CollectFees"></a>

### UniswapV3Adaptor.CollectFees
Allows strategist to collect fees from existing Uniswap V3 positions.

Represents function `collectFees(uint256 tokenId, uint128 amount0, uint128 amount1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-OpenPosition"></a>

### UniswapV3Adaptor.OpenPosition
Allows strategist to open up arbritray Uniswap V3 positions.

Represents function openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |
| pool_fee | [uint32](#uint32) |  |  |
| amount_0 | [string](#string) |  |  |
| amount_1 | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |
| tick_lower | [int32](#int32) |  |  |
| tick_upper | [int32](#int32) |  |  |






<a name="steward-v3-UniswapV3Adaptor-PurgeAllZeroLiquidityPositions"></a>

### UniswapV3Adaptor.PurgeAllZeroLiquidityPositions
Allows strategist to purge zero liquidity LP positions from tracker.

Represents function `purgeAllZeroLiquidityPositions(ERC20 token0, ERC20 token1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-PurgeSinglePosition"></a>

### UniswapV3Adaptor.PurgeSinglePosition
Allows strategist to purge a single zero liquidity LP position from tracker.

Represents function `purgeSinglePosition(uint256 tokenId)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-RemoveUnownedPositionFromTracker"></a>

### UniswapV3Adaptor.RemoveUnownedPositionFromTracker
Allows strategist to remove tracked positions that are not owned by the cellar.

Represents function `removeUnOwnedPositionFromTracker(uint256 tokenId, ERC20 token0, ERC20 token1)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| token_0 | [string](#string) |  |  |
| token_1 | [string](#string) |  |  |






<a name="steward-v3-UniswapV3Adaptor-TakeFromPosition"></a>

### UniswapV3Adaptor.TakeFromPosition
Allows strategist to take from existing Uniswap V3 positions.

Represents function `takeFromPosition(uint256 tokenId, uint128 liquidity, uint256 min0, uint256 min1, bool takeFees)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token_id | [string](#string) |  |  |
| liquidity | [string](#string) |  |  |
| min_0 | [string](#string) |  |  |
| min_1 | [string](#string) |  |  |
| take_fees | [bool](#bool) |  |  |






<a name="steward-v3-UniswapV3AdaptorCalls"></a>

### UniswapV3AdaptorCalls



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| calls | [UniswapV3Adaptor](#steward-v3-UniswapV3Adaptor) | repeated |  |





 

 

 

 



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

