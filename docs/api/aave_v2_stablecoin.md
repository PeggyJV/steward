# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [aave_v2_stablecoin.proto](#aave_v2_stablecoin-proto)
    - [AaveV2Stablecoin](#steward-v1-AaveV2Stablecoin)
    - [AaveV2Stablecoin.AccrueFees](#steward-v1-AaveV2Stablecoin-AccrueFees)
    - [AaveV2Stablecoin.ClaimAndUnstake](#steward-v1-AaveV2Stablecoin-ClaimAndUnstake)
    - [AaveV2Stablecoin.EnterPosition](#steward-v1-AaveV2Stablecoin-EnterPosition)
    - [AaveV2Stablecoin.Rebalance](#steward-v1-AaveV2Stablecoin-Rebalance)
    - [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v1-AaveV2Stablecoin-Rebalance-SwapParams)
    - [AaveV2Stablecoin.Reinvest](#steward-v1-AaveV2Stablecoin-Reinvest)
    - [AaveV2Stablecoin.Sweep](#steward-v1-AaveV2Stablecoin-Sweep)
    - [AaveV2Stablecoin.TransferFees](#steward-v1-AaveV2Stablecoin-TransferFees)
  
- [Scalar Value Types](#scalar-value-types)



<a name="aave_v2_stablecoin-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## aave_v2_stablecoin.proto



<a name="steward-v1-AaveV2Stablecoin"></a>

### AaveV2Stablecoin
Represents a function call to the Aave V2 Stablecoin cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| accrue_fees | [AaveV2Stablecoin.AccrueFees](#steward-v1-AaveV2Stablecoin-AccrueFees) |  | Represents function `accruePlatformFees()`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L702) |
| claim_and_unstake | [AaveV2Stablecoin.ClaimAndUnstake](#steward-v1-AaveV2Stablecoin-ClaimAndUnstake) |  | Represents function `claimAndUnstake()`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L941) |
| enter_position | [AaveV2Stablecoin.EnterPosition](#steward-v1-AaveV2Stablecoin-EnterPosition) |  | Represents function `enterPosition()`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L820) |
| rebalance | [AaveV2Stablecoin.Rebalance](#steward-v1-AaveV2Stablecoin-Rebalance) |  | Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L844) |
| reinvest | [AaveV2Stablecoin.Reinvest](#steward-v1-AaveV2Stablecoin-Reinvest) |  | Represents function `reinvest(uint256 minAssetsOut)`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L901) |
| sweep | [AaveV2Stablecoin.Sweep](#steward-v1-AaveV2Stablecoin-Sweep) |  | Represents function `sweep(address)`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L961) |
| transfer_fees | [AaveV2Stablecoin.TransferFees](#steward-v1-AaveV2Stablecoin-TransferFees) |  | Represents function `transferFees()`: [link](https://github.com/PeggyJV/cellar-contracts/blob/b93b4393299aee35421f83ef4ca2689256e5b354/contracts/AaveV2StablecoinCellar.sol#L754) |






<a name="steward-v1-AaveV2Stablecoin-AccrueFees"></a>

### AaveV2Stablecoin.AccrueFees
Take platform fees and performance fees off of cellar&#39;s active assets.

Represents function `accruePlatformFees()`






<a name="steward-v1-AaveV2Stablecoin-ClaimAndUnstake"></a>

### AaveV2Stablecoin.ClaimAndUnstake
Claim rewards from Aave and begin cooldown period to unstake them.

Represents function `claimAndUnstake()`






<a name="steward-v1-AaveV2Stablecoin-EnterPosition"></a>

### AaveV2Stablecoin.EnterPosition
Enters inactive assets into the current Aave stablecoin position.

Represents function `enterPosition()`






<a name="steward-v1-AaveV2Stablecoin-Rebalance"></a>

### AaveV2Stablecoin.Rebalance
Rebalances current assets into a new asset position.

Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`

This function is based on the Curve Pool Registry exchange_multiple() function:
https://github.com/curvefi/curve-pool-registry/blob/16a8664952cf61d7fed06acca79ad5ac696f4b20/contracts/Swaps.vy#L461-L489


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| route | [string](#string) | repeated | An array of up to 9 addresses (4 swaps) representing a token swap route, where each triplet of addresses is a single swap route (ex. in token address, pool address, out token address) |
| swap_params | [AaveV2Stablecoin.Rebalance.SwapParams](#steward-v1-AaveV2Stablecoin-Rebalance-SwapParams) | repeated | An array of up to 4 swap params. Attempting more than four swaps will fail. |
| min_assets_out | [uint64](#uint64) |  | Minimum acceptable assets to be received from the swap (slippage parameter) |






<a name="steward-v1-AaveV2Stablecoin-Rebalance-SwapParams"></a>

### AaveV2Stablecoin.Rebalance.SwapParams
Represents parameters for a single swap. Each swap needs the indeces in Rebalance.route of the in/out token addresses and the swap type. See the Curve contract linked above for more detail.


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| in_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s input token address |
| out_index | [uint64](#uint64) |  | Index in the `route` array of the swap&#39;s output token address |
| swap_type | [uint64](#uint64) |  | 1 - stableswap `exchange` 2 - stableswap `exchange_underlying` 3 - cryptoswap `exchange` 4 - cryptoswap `exchange_underlying` 5 - Polygon factory metapools `exchange_underlying` See the Curve Pool Registry exchange_multiple() function for more information. |






<a name="steward-v1-AaveV2Stablecoin-Reinvest"></a>

### AaveV2Stablecoin.Reinvest
Reinvest rewards back into cellar&#39;s current position. Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.

Represents function `reinvest(uint256 minAssetsOut)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min_assets_out | [uint64](#uint64) |  | Minimum acceptable assets to be received from the swap (slippage parameter) |






<a name="steward-v1-AaveV2Stablecoin-Sweep"></a>

### AaveV2Stablecoin.Sweep
Sweep tokens sent here that are not managed by the cellar. This may be used in case the wrong tokens are accidentally sent to this contract.

Represents function `sweep(address)`


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| token | [string](#string) |  | The address of the token to be transferred out of the Cellar |
| to | [string](#string) |  | The address to which the tokens should be transferred |






<a name="steward-v1-AaveV2Stablecoin-TransferFees"></a>

### AaveV2Stablecoin.TransferFees
Transfer accrued fees to the Sommelier Chain to distribute.

Represents function `transferFees()`





 

 

 

 



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

