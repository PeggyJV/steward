//
// Protos for function calls to the Aave AToken adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: adaptors/aave/a_token.proto

package steward_proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// Represents call data for the Aave AToken adaptor V1, used to manage lending positions on Aave
type AaveATokenAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*AaveATokenAdaptorV1_Swap
	//	*AaveATokenAdaptorV1_OracleSwap
	//	*AaveATokenAdaptorV1_RevokeApproval
	//	*AaveATokenAdaptorV1_DepositToAave_
	//	*AaveATokenAdaptorV1_WithdrawFromAave_
	Function isAaveATokenAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *AaveATokenAdaptorV1) Reset() {
	*x = AaveATokenAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV1) ProtoMessage() {}

func (x *AaveATokenAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV1.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV1) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{0}
}

func (m *AaveATokenAdaptorV1) GetFunction() isAaveATokenAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *AaveATokenAdaptorV1) GetSwap() *Swap {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV1_Swap); ok {
		return x.Swap
	}
	return nil
}

func (x *AaveATokenAdaptorV1) GetOracleSwap() *OracleSwap {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV1_OracleSwap); ok {
		return x.OracleSwap
	}
	return nil
}

func (x *AaveATokenAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *AaveATokenAdaptorV1) GetDepositToAave() *AaveATokenAdaptorV1_DepositToAave {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV1_DepositToAave_); ok {
		return x.DepositToAave
	}
	return nil
}

func (x *AaveATokenAdaptorV1) GetWithdrawFromAave() *AaveATokenAdaptorV1_WithdrawFromAave {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV1_WithdrawFromAave_); ok {
		return x.WithdrawFromAave
	}
	return nil
}

type isAaveATokenAdaptorV1_Function interface {
	isAaveATokenAdaptorV1_Function()
}

type AaveATokenAdaptorV1_Swap struct {
	// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
	Swap *Swap `protobuf:"bytes,1,opt,name=swap,proto3,oneof"`
}

type AaveATokenAdaptorV1_OracleSwap struct {
	// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
	OracleSwap *OracleSwap `protobuf:"bytes,2,opt,name=oracle_swap,json=oracleSwap,proto3,oneof"`
}

type AaveATokenAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,3,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type AaveATokenAdaptorV1_DepositToAave_ struct {
	// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
	DepositToAave *AaveATokenAdaptorV1_DepositToAave `protobuf:"bytes,4,opt,name=deposit_to_aave,json=depositToAave,proto3,oneof"`
}

type AaveATokenAdaptorV1_WithdrawFromAave_ struct {
	// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
	WithdrawFromAave *AaveATokenAdaptorV1_WithdrawFromAave `protobuf:"bytes,5,opt,name=withdraw_from_aave,json=withdrawFromAave,proto3,oneof"`
}

func (*AaveATokenAdaptorV1_Swap) isAaveATokenAdaptorV1_Function() {}

func (*AaveATokenAdaptorV1_OracleSwap) isAaveATokenAdaptorV1_Function() {}

func (*AaveATokenAdaptorV1_RevokeApproval) isAaveATokenAdaptorV1_Function() {}

func (*AaveATokenAdaptorV1_DepositToAave_) isAaveATokenAdaptorV1_Function() {}

func (*AaveATokenAdaptorV1_WithdrawFromAave_) isAaveATokenAdaptorV1_Function() {}

// Represents call data for the Aave AToken adaptor V2, used to manage lending positions on Aave
type AaveATokenAdaptorV2 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*AaveATokenAdaptorV2_RevokeApproval
	//	*AaveATokenAdaptorV2_DepositToAave_
	//	*AaveATokenAdaptorV2_WithdrawFromAave_
	Function isAaveATokenAdaptorV2_Function `protobuf_oneof:"function"`
}

func (x *AaveATokenAdaptorV2) Reset() {
	*x = AaveATokenAdaptorV2{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV2) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV2) ProtoMessage() {}

func (x *AaveATokenAdaptorV2) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV2.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV2) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{1}
}

func (m *AaveATokenAdaptorV2) GetFunction() isAaveATokenAdaptorV2_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *AaveATokenAdaptorV2) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV2_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *AaveATokenAdaptorV2) GetDepositToAave() *AaveATokenAdaptorV2_DepositToAave {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV2_DepositToAave_); ok {
		return x.DepositToAave
	}
	return nil
}

func (x *AaveATokenAdaptorV2) GetWithdrawFromAave() *AaveATokenAdaptorV2_WithdrawFromAave {
	if x, ok := x.GetFunction().(*AaveATokenAdaptorV2_WithdrawFromAave_); ok {
		return x.WithdrawFromAave
	}
	return nil
}

type isAaveATokenAdaptorV2_Function interface {
	isAaveATokenAdaptorV2_Function()
}

type AaveATokenAdaptorV2_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type AaveATokenAdaptorV2_DepositToAave_ struct {
	// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
	DepositToAave *AaveATokenAdaptorV2_DepositToAave `protobuf:"bytes,2,opt,name=deposit_to_aave,json=depositToAave,proto3,oneof"`
}

type AaveATokenAdaptorV2_WithdrawFromAave_ struct {
	// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
	WithdrawFromAave *AaveATokenAdaptorV2_WithdrawFromAave `protobuf:"bytes,3,opt,name=withdraw_from_aave,json=withdrawFromAave,proto3,oneof"`
}

func (*AaveATokenAdaptorV2_RevokeApproval) isAaveATokenAdaptorV2_Function() {}

func (*AaveATokenAdaptorV2_DepositToAave_) isAaveATokenAdaptorV2_Function() {}

func (*AaveATokenAdaptorV2_WithdrawFromAave_) isAaveATokenAdaptorV2_Function() {}

type AaveATokenAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*AaveATokenAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *AaveATokenAdaptorV1Calls) Reset() {
	*x = AaveATokenAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV1Calls) ProtoMessage() {}

func (x *AaveATokenAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{2}
}

func (x *AaveATokenAdaptorV1Calls) GetCalls() []*AaveATokenAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

type AaveATokenAdaptorV2Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*AaveATokenAdaptorV2 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *AaveATokenAdaptorV2Calls) Reset() {
	*x = AaveATokenAdaptorV2Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV2Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV2Calls) ProtoMessage() {}

func (x *AaveATokenAdaptorV2Calls) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV2Calls.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV2Calls) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{3}
}

func (x *AaveATokenAdaptorV2Calls) GetCalls() []*AaveATokenAdaptorV2 {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to lend assets on Aave.
//
// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
type AaveATokenAdaptorV1_DepositToAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to deposit
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to deposit
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveATokenAdaptorV1_DepositToAave) Reset() {
	*x = AaveATokenAdaptorV1_DepositToAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV1_DepositToAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV1_DepositToAave) ProtoMessage() {}

func (x *AaveATokenAdaptorV1_DepositToAave) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV1_DepositToAave.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV1_DepositToAave) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *AaveATokenAdaptorV1_DepositToAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveATokenAdaptorV1_DepositToAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Allows strategists to withdraw assets from Aave.
//
// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
type AaveATokenAdaptorV1_WithdrawFromAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to withdraw
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to withdraw
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveATokenAdaptorV1_WithdrawFromAave) Reset() {
	*x = AaveATokenAdaptorV1_WithdrawFromAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV1_WithdrawFromAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV1_WithdrawFromAave) ProtoMessage() {}

func (x *AaveATokenAdaptorV1_WithdrawFromAave) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV1_WithdrawFromAave.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV1_WithdrawFromAave) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *AaveATokenAdaptorV1_WithdrawFromAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveATokenAdaptorV1_WithdrawFromAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Allows strategists to lend assets on Aave.
//
// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
type AaveATokenAdaptorV2_DepositToAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to deposit
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to deposit
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveATokenAdaptorV2_DepositToAave) Reset() {
	*x = AaveATokenAdaptorV2_DepositToAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV2_DepositToAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV2_DepositToAave) ProtoMessage() {}

func (x *AaveATokenAdaptorV2_DepositToAave) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV2_DepositToAave.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV2_DepositToAave) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{1, 0}
}

func (x *AaveATokenAdaptorV2_DepositToAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveATokenAdaptorV2_DepositToAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

//
// Allows strategists to withdraw assets from Aave.
//
// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
type AaveATokenAdaptorV2_WithdrawFromAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to withdraw
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to withdraw
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveATokenAdaptorV2_WithdrawFromAave) Reset() {
	*x = AaveATokenAdaptorV2_WithdrawFromAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_aave_a_token_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveATokenAdaptorV2_WithdrawFromAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveATokenAdaptorV2_WithdrawFromAave) ProtoMessage() {}

func (x *AaveATokenAdaptorV2_WithdrawFromAave) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_aave_a_token_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveATokenAdaptorV2_WithdrawFromAave.ProtoReflect.Descriptor instead.
func (*AaveATokenAdaptorV2_WithdrawFromAave) Descriptor() ([]byte, []int) {
	return file_adaptors_aave_a_token_proto_rawDescGZIP(), []int{1, 1}
}

func (x *AaveATokenAdaptorV2_WithdrawFromAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveATokenAdaptorV2_WithdrawFromAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

var File_adaptors_aave_a_token_proto protoreflect.FileDescriptor

var file_adaptors_aave_a_token_proto_rawDesc = []byte{
	0x0a, 0x1b, 0x61, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x61, 0x61, 0x76, 0x65, 0x2f,
	0x61, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x13, 0x61, 0x64, 0x61, 0x70, 0x74,
	0x6f, 0x72, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x87,
	0x04, 0x0a, 0x13, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x26, 0x0a, 0x04, 0x73, 0x77, 0x61, 0x70, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x34, 0x2e, 0x53, 0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x04, 0x73, 0x77, 0x61, 0x70, 0x12, 0x39,
	0x0a, 0x0b, 0x6f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x5f, 0x73, 0x77, 0x61, 0x70, 0x18, 0x02, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x4f, 0x72, 0x61, 0x63, 0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x0a, 0x6f,
	0x72, 0x61, 0x63, 0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76,
	0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e,
	0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00,
	0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c,
	0x12, 0x57, 0x0a, 0x0f, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x61,
	0x61, 0x76, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65,
	0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x44, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x0d, 0x64, 0x65, 0x70, 0x6f,
	0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x12, 0x60, 0x0a, 0x12, 0x77, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46,
	0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x10, 0x77, 0x69, 0x74, 0x68, 0x64,
	0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x1a, 0x3d, 0x0a, 0x0d, 0x44,
	0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x12, 0x14, 0x0a, 0x05,
	0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b,
	0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x40, 0x0a, 0x10, 0x57, 0x69,
	0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x12, 0x14,
	0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74,
	0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x42, 0x0a, 0x0a, 0x08,
	0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa4, 0x03, 0x0a, 0x13, 0x41, 0x61, 0x76,
	0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32,
	0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f,
	0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70,
	0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41,
	0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x57, 0x0a, 0x0f, 0x64, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x2d, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61,
	0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56,
	0x32, 0x2e, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65, 0x48,
	0x00, 0x52, 0x0d, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41, 0x61, 0x76, 0x65,
	0x12, 0x60, 0x0a, 0x12, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66, 0x72, 0x6f,
	0x6d, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54,
	0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x2e, 0x57, 0x69,
	0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x48, 0x00,
	0x52, 0x10, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61,
	0x76, 0x65, 0x1a, 0x3d, 0x0a, 0x0d, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x41,
	0x61, 0x76, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e,
	0x74, 0x1a, 0x40, 0x0a, 0x10, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f,
	0x6d, 0x41, 0x61, 0x76, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22,
	0x51, 0x0a, 0x18, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x35, 0x0a, 0x05, 0x63,
	0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x73, 0x74, 0x65,
	0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63, 0x61, 0x6c,
	0x6c, 0x73, 0x22, 0x51, 0x0a, 0x18, 0x41, 0x61, 0x76, 0x65, 0x41, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x35,
	0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x41,
	0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x52, 0x05,
	0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_adaptors_aave_a_token_proto_rawDescOnce sync.Once
	file_adaptors_aave_a_token_proto_rawDescData = file_adaptors_aave_a_token_proto_rawDesc
)

func file_adaptors_aave_a_token_proto_rawDescGZIP() []byte {
	file_adaptors_aave_a_token_proto_rawDescOnce.Do(func() {
		file_adaptors_aave_a_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_adaptors_aave_a_token_proto_rawDescData)
	})
	return file_adaptors_aave_a_token_proto_rawDescData
}

var file_adaptors_aave_a_token_proto_msgTypes = make([]protoimpl.MessageInfo, 8)
var file_adaptors_aave_a_token_proto_goTypes = []interface{}{
	(*AaveATokenAdaptorV1)(nil),                  // 0: steward.v4.AaveATokenAdaptorV1
	(*AaveATokenAdaptorV2)(nil),                  // 1: steward.v4.AaveATokenAdaptorV2
	(*AaveATokenAdaptorV1Calls)(nil),             // 2: steward.v4.AaveATokenAdaptorV1Calls
	(*AaveATokenAdaptorV2Calls)(nil),             // 3: steward.v4.AaveATokenAdaptorV2Calls
	(*AaveATokenAdaptorV1_DepositToAave)(nil),    // 4: steward.v4.AaveATokenAdaptorV1.DepositToAave
	(*AaveATokenAdaptorV1_WithdrawFromAave)(nil), // 5: steward.v4.AaveATokenAdaptorV1.WithdrawFromAave
	(*AaveATokenAdaptorV2_DepositToAave)(nil),    // 6: steward.v4.AaveATokenAdaptorV2.DepositToAave
	(*AaveATokenAdaptorV2_WithdrawFromAave)(nil), // 7: steward.v4.AaveATokenAdaptorV2.WithdrawFromAave
	(*Swap)(nil),           // 8: steward.v4.Swap
	(*OracleSwap)(nil),     // 9: steward.v4.OracleSwap
	(*RevokeApproval)(nil), // 10: steward.v4.RevokeApproval
}
var file_adaptors_aave_a_token_proto_depIdxs = []int32{
	8,  // 0: steward.v4.AaveATokenAdaptorV1.swap:type_name -> steward.v4.Swap
	9,  // 1: steward.v4.AaveATokenAdaptorV1.oracle_swap:type_name -> steward.v4.OracleSwap
	10, // 2: steward.v4.AaveATokenAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	4,  // 3: steward.v4.AaveATokenAdaptorV1.deposit_to_aave:type_name -> steward.v4.AaveATokenAdaptorV1.DepositToAave
	5,  // 4: steward.v4.AaveATokenAdaptorV1.withdraw_from_aave:type_name -> steward.v4.AaveATokenAdaptorV1.WithdrawFromAave
	10, // 5: steward.v4.AaveATokenAdaptorV2.revoke_approval:type_name -> steward.v4.RevokeApproval
	6,  // 6: steward.v4.AaveATokenAdaptorV2.deposit_to_aave:type_name -> steward.v4.AaveATokenAdaptorV2.DepositToAave
	7,  // 7: steward.v4.AaveATokenAdaptorV2.withdraw_from_aave:type_name -> steward.v4.AaveATokenAdaptorV2.WithdrawFromAave
	0,  // 8: steward.v4.AaveATokenAdaptorV1Calls.calls:type_name -> steward.v4.AaveATokenAdaptorV1
	1,  // 9: steward.v4.AaveATokenAdaptorV2Calls.calls:type_name -> steward.v4.AaveATokenAdaptorV2
	10, // [10:10] is the sub-list for method output_type
	10, // [10:10] is the sub-list for method input_type
	10, // [10:10] is the sub-list for extension type_name
	10, // [10:10] is the sub-list for extension extendee
	0,  // [0:10] is the sub-list for field type_name
}

func init() { file_adaptors_aave_a_token_proto_init() }
func file_adaptors_aave_a_token_proto_init() {
	if File_adaptors_aave_a_token_proto != nil {
		return
	}
	file_adaptors_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_adaptors_aave_a_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV1); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV2); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV1Calls); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV2Calls); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV1_DepositToAave); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV1_WithdrawFromAave); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV2_DepositToAave); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_adaptors_aave_a_token_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveATokenAdaptorV2_WithdrawFromAave); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_adaptors_aave_a_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*AaveATokenAdaptorV1_Swap)(nil),
		(*AaveATokenAdaptorV1_OracleSwap)(nil),
		(*AaveATokenAdaptorV1_RevokeApproval)(nil),
		(*AaveATokenAdaptorV1_DepositToAave_)(nil),
		(*AaveATokenAdaptorV1_WithdrawFromAave_)(nil),
	}
	file_adaptors_aave_a_token_proto_msgTypes[1].OneofWrappers = []interface{}{
		(*AaveATokenAdaptorV2_RevokeApproval)(nil),
		(*AaveATokenAdaptorV2_DepositToAave_)(nil),
		(*AaveATokenAdaptorV2_WithdrawFromAave_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_adaptors_aave_a_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   8,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_adaptors_aave_a_token_proto_goTypes,
		DependencyIndexes: file_adaptors_aave_a_token_proto_depIdxs,
		MessageInfos:      file_adaptors_aave_a_token_proto_msgTypes,
	}.Build()
	File_adaptors_aave_a_token_proto = out.File
	file_adaptors_aave_a_token_proto_rawDesc = nil
	file_adaptors_aave_a_token_proto_goTypes = nil
	file_adaptors_aave_a_token_proto_depIdxs = nil
}
