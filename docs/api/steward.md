# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [steward.proto](#steward-proto)
    - [SubmitRequest](#steward-v3-SubmitRequest)
    - [SubmitResponse](#steward-v3-SubmitResponse)
  
    - [ContractCall](#steward-v3-ContractCall)
  
- [Scalar Value Types](#scalar-value-types)



<a name="steward-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## steward.proto



<a name="steward-v3-SubmitRequest"></a>

### SubmitRequest
Represents a single function call on a particular Cellar


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| cellar_id | [string](#string) |  | The ID (currently simply an Ethereum address) of the target Cellar |
| aave_v2_stablecoin | [AaveV2Stablecoin](#steward-v3-AaveV2Stablecoin) |  |  |
| cellar_v1 | [CellarV1](#steward-v3-CellarV1) |  |  |
| cellar_v2 | [CellarV2](#steward-v3-CellarV2) |  |  |
| cellar_v2dot2 | [CellarV2_2](#steward-v3-CellarV2_2) |  |  |






<a name="steward-v3-SubmitResponse"></a>

### SubmitResponse






 

 

 


<a name="steward-v3-ContractCall"></a>

### ContractCall
Service for handling Cellar contract calls

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| Submit | [SubmitRequest](#steward-v3-SubmitRequest) | [SubmitResponse](#steward-v3-SubmitResponse) | Handles simple contract call submission |

 



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

