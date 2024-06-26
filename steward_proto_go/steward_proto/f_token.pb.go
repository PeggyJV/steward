//
// Protos for function calls to the Frax adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.33.0
// 	protoc        v4.25.3
// source: f_token.proto

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

// Represents call data for the Frax adaptor.
type FTokenAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*FTokenAdaptorV1_RevokeApproval
	//	*FTokenAdaptorV1_LendFrax_
	//	*FTokenAdaptorV1_RedeemFraxShare_
	//	*FTokenAdaptorV1_WithdrawFrax_
	//	*FTokenAdaptorV1_CallAddInterest_
	Function isFTokenAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *FTokenAdaptorV1) Reset() {
	*x = FTokenAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1) ProtoMessage() {}

func (x *FTokenAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{0}
}

func (m *FTokenAdaptorV1) GetFunction() isFTokenAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *FTokenAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*FTokenAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *FTokenAdaptorV1) GetLendFrax() *FTokenAdaptorV1_LendFrax {
	if x, ok := x.GetFunction().(*FTokenAdaptorV1_LendFrax_); ok {
		return x.LendFrax
	}
	return nil
}

func (x *FTokenAdaptorV1) GetRedeemFraxShare() *FTokenAdaptorV1_RedeemFraxShare {
	if x, ok := x.GetFunction().(*FTokenAdaptorV1_RedeemFraxShare_); ok {
		return x.RedeemFraxShare
	}
	return nil
}

func (x *FTokenAdaptorV1) GetWithdrawFrax() *FTokenAdaptorV1_WithdrawFrax {
	if x, ok := x.GetFunction().(*FTokenAdaptorV1_WithdrawFrax_); ok {
		return x.WithdrawFrax
	}
	return nil
}

func (x *FTokenAdaptorV1) GetCallAddInterest() *FTokenAdaptorV1_CallAddInterest {
	if x, ok := x.GetFunction().(*FTokenAdaptorV1_CallAddInterest_); ok {
		return x.CallAddInterest
	}
	return nil
}

type isFTokenAdaptorV1_Function interface {
	isFTokenAdaptorV1_Function()
}

type FTokenAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type FTokenAdaptorV1_LendFrax_ struct {
	// Represents function `lendFrax(IFToken fToken, uint256 amountToDeposit)`
	LendFrax *FTokenAdaptorV1_LendFrax `protobuf:"bytes,2,opt,name=lend_frax,json=lendFrax,proto3,oneof"`
}

type FTokenAdaptorV1_RedeemFraxShare_ struct {
	// Represents function `redeemFraxShare(IFToken fToken, uint256 amountToRedeem)`
	RedeemFraxShare *FTokenAdaptorV1_RedeemFraxShare `protobuf:"bytes,3,opt,name=redeem_frax_share,json=redeemFraxShare,proto3,oneof"`
}

type FTokenAdaptorV1_WithdrawFrax_ struct {
	// Represents function `withdrawFrax(IFToken fToken, uint256 amountToWithdraw)`
	WithdrawFrax *FTokenAdaptorV1_WithdrawFrax `protobuf:"bytes,4,opt,name=withdraw_frax,json=withdrawFrax,proto3,oneof"`
}

type FTokenAdaptorV1_CallAddInterest_ struct {
	// Represents function `callAddInterest(IFToken fToken)`
	CallAddInterest *FTokenAdaptorV1_CallAddInterest `protobuf:"bytes,5,opt,name=call_add_interest,json=callAddInterest,proto3,oneof"`
}

func (*FTokenAdaptorV1_RevokeApproval) isFTokenAdaptorV1_Function() {}

func (*FTokenAdaptorV1_LendFrax_) isFTokenAdaptorV1_Function() {}

func (*FTokenAdaptorV1_RedeemFraxShare_) isFTokenAdaptorV1_Function() {}

func (*FTokenAdaptorV1_WithdrawFrax_) isFTokenAdaptorV1_Function() {}

func (*FTokenAdaptorV1_CallAddInterest_) isFTokenAdaptorV1_Function() {}

type FTokenAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*FTokenAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *FTokenAdaptorV1Calls) Reset() {
	*x = FTokenAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1Calls) ProtoMessage() {}

func (x *FTokenAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{1}
}

func (x *FTokenAdaptorV1Calls) GetCalls() []*FTokenAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists to lend FRAX on FraxLend
//
// Represents `function lendFrax(IFToken fToken, uint256 amountToDeposit)`
type FTokenAdaptorV1_LendFrax struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the fToken to lend.
	FToken string `protobuf:"bytes,1,opt,name=f_token,json=fToken,proto3" json:"f_token,omitempty"`
	// The amount of the fToken to lend.
	AmountToDeposit string `protobuf:"bytes,2,opt,name=amount_to_deposit,json=amountToDeposit,proto3" json:"amount_to_deposit,omitempty"`
}

func (x *FTokenAdaptorV1_LendFrax) Reset() {
	*x = FTokenAdaptorV1_LendFrax{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1_LendFrax) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1_LendFrax) ProtoMessage() {}

func (x *FTokenAdaptorV1_LendFrax) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1_LendFrax.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1_LendFrax) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *FTokenAdaptorV1_LendFrax) GetFToken() string {
	if x != nil {
		return x.FToken
	}
	return ""
}

func (x *FTokenAdaptorV1_LendFrax) GetAmountToDeposit() string {
	if x != nil {
		return x.AmountToDeposit
	}
	return ""
}

// Allows strategists to redeem FRAX shares on FraxLend
//
// Represents `function redeemFraxShare(IFToken fToken, uint256 amountToRedeem)`
type FTokenAdaptorV1_RedeemFraxShare struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the fToken to redeem.
	FToken string `protobuf:"bytes,1,opt,name=f_token,json=fToken,proto3" json:"f_token,omitempty"`
	// The amount of the fToken to redeem.
	AmountToRedeem string `protobuf:"bytes,2,opt,name=amount_to_redeem,json=amountToRedeem,proto3" json:"amount_to_redeem,omitempty"`
}

func (x *FTokenAdaptorV1_RedeemFraxShare) Reset() {
	*x = FTokenAdaptorV1_RedeemFraxShare{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1_RedeemFraxShare) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1_RedeemFraxShare) ProtoMessage() {}

func (x *FTokenAdaptorV1_RedeemFraxShare) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1_RedeemFraxShare.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1_RedeemFraxShare) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *FTokenAdaptorV1_RedeemFraxShare) GetFToken() string {
	if x != nil {
		return x.FToken
	}
	return ""
}

func (x *FTokenAdaptorV1_RedeemFraxShare) GetAmountToRedeem() string {
	if x != nil {
		return x.AmountToRedeem
	}
	return ""
}

// Allows strategists to withdraw FRAX from FraxLend
//
// Represents `function withdrawFrax(IFToken fToken, uint256 amountToWithdraw)`
type FTokenAdaptorV1_WithdrawFrax struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the fToken to withdraw.
	FToken string `protobuf:"bytes,1,opt,name=f_token,json=fToken,proto3" json:"f_token,omitempty"`
	// The amount of the fToken to withdraw.
	AmountToWithdraw string `protobuf:"bytes,2,opt,name=amount_to_withdraw,json=amountToWithdraw,proto3" json:"amount_to_withdraw,omitempty"`
}

func (x *FTokenAdaptorV1_WithdrawFrax) Reset() {
	*x = FTokenAdaptorV1_WithdrawFrax{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1_WithdrawFrax) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1_WithdrawFrax) ProtoMessage() {}

func (x *FTokenAdaptorV1_WithdrawFrax) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1_WithdrawFrax.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1_WithdrawFrax) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{0, 2}
}

func (x *FTokenAdaptorV1_WithdrawFrax) GetFToken() string {
	if x != nil {
		return x.FToken
	}
	return ""
}

func (x *FTokenAdaptorV1_WithdrawFrax) GetAmountToWithdraw() string {
	if x != nil {
		return x.AmountToWithdraw
	}
	return ""
}

// Allows a strategist to call `addInterest` on a Frax Pair they are using
//
// Represents `function callAddInterest(IFToken fToken)`
type FTokenAdaptorV1_CallAddInterest struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the fToken to call `addInterest` on.
	FToken string `protobuf:"bytes,1,opt,name=f_token,json=fToken,proto3" json:"f_token,omitempty"`
}

func (x *FTokenAdaptorV1_CallAddInterest) Reset() {
	*x = FTokenAdaptorV1_CallAddInterest{}
	if protoimpl.UnsafeEnabled {
		mi := &file_f_token_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *FTokenAdaptorV1_CallAddInterest) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*FTokenAdaptorV1_CallAddInterest) ProtoMessage() {}

func (x *FTokenAdaptorV1_CallAddInterest) ProtoReflect() protoreflect.Message {
	mi := &file_f_token_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use FTokenAdaptorV1_CallAddInterest.ProtoReflect.Descriptor instead.
func (*FTokenAdaptorV1_CallAddInterest) Descriptor() ([]byte, []int) {
	return file_f_token_proto_rawDescGZIP(), []int{0, 3}
}

func (x *FTokenAdaptorV1_CallAddInterest) GetFToken() string {
	if x != nil {
		return x.FToken
	}
	return ""
}

var File_f_token_proto protoreflect.FileDescriptor

var file_f_token_proto_rawDesc = []byte{
	0x0a, 0x0d, 0x66, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
	0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xda, 0x05, 0x0a, 0x0f, 0x46, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72,
	0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c,
	0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76,
	0x61, 0x6c, 0x12, 0x43, 0x0a, 0x09, 0x6c, 0x65, 0x6e, 0x64, 0x5f, 0x66, 0x72, 0x61, 0x78, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72,
	0x56, 0x31, 0x2e, 0x4c, 0x65, 0x6e, 0x64, 0x46, 0x72, 0x61, 0x78, 0x48, 0x00, 0x52, 0x08, 0x6c,
	0x65, 0x6e, 0x64, 0x46, 0x72, 0x61, 0x78, 0x12, 0x59, 0x0a, 0x11, 0x72, 0x65, 0x64, 0x65, 0x65,
	0x6d, 0x5f, 0x66, 0x72, 0x61, 0x78, 0x5f, 0x73, 0x68, 0x61, 0x72, 0x65, 0x18, 0x03, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e,
	0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e,
	0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x46, 0x72, 0x61, 0x78, 0x53, 0x68, 0x61, 0x72, 0x65, 0x48,
	0x00, 0x52, 0x0f, 0x72, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x46, 0x72, 0x61, 0x78, 0x53, 0x68, 0x61,
	0x72, 0x65, 0x12, 0x4f, 0x0a, 0x0d, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66,
	0x72, 0x61, 0x78, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46,
	0x72, 0x61, 0x78, 0x48, 0x00, 0x52, 0x0c, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46,
	0x72, 0x61, 0x78, 0x12, 0x59, 0x0a, 0x11, 0x63, 0x61, 0x6c, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x5f,
	0x69, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b,
	0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x46, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x43, 0x61, 0x6c, 0x6c,
	0x41, 0x64, 0x64, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x0f, 0x63,
	0x61, 0x6c, 0x6c, 0x41, 0x64, 0x64, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x1a, 0x4f,
	0x0a, 0x08, 0x4c, 0x65, 0x6e, 0x64, 0x46, 0x72, 0x61, 0x78, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x5f,
	0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x12, 0x2a, 0x0a, 0x11, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f,
	0x5f, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f,
	0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x1a,
	0x54, 0x0a, 0x0f, 0x52, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x46, 0x72, 0x61, 0x78, 0x53, 0x68, 0x61,
	0x72, 0x65, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x28, 0x0a, 0x10, 0x61,
	0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x64, 0x65, 0x65, 0x6d, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x52,
	0x65, 0x64, 0x65, 0x65, 0x6d, 0x1a, 0x55, 0x0a, 0x0c, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61,
	0x77, 0x46, 0x72, 0x61, 0x78, 0x12, 0x17, 0x0a, 0x07, 0x66, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
	0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x66, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x2c,
	0x0a, 0x12, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x77, 0x69, 0x74, 0x68,
	0x64, 0x72, 0x61, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x61, 0x6d, 0x6f, 0x75,
	0x6e, 0x74, 0x54, 0x6f, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x1a, 0x2a, 0x0a, 0x0f,
	0x43, 0x61, 0x6c, 0x6c, 0x41, 0x64, 0x64, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x65, 0x73, 0x74, 0x12,
	0x17, 0x0a, 0x07, 0x66, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x06, 0x66, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63,
	0x74, 0x69, 0x6f, 0x6e, 0x22, 0x49, 0x0a, 0x14, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x31, 0x0a, 0x05,
	0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x73, 0x74,
	0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x46, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42,
	0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_f_token_proto_rawDescOnce sync.Once
	file_f_token_proto_rawDescData = file_f_token_proto_rawDesc
)

func file_f_token_proto_rawDescGZIP() []byte {
	file_f_token_proto_rawDescOnce.Do(func() {
		file_f_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_f_token_proto_rawDescData)
	})
	return file_f_token_proto_rawDescData
}

var file_f_token_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_f_token_proto_goTypes = []interface{}{
	(*FTokenAdaptorV1)(nil),                 // 0: steward.v4.FTokenAdaptorV1
	(*FTokenAdaptorV1Calls)(nil),            // 1: steward.v4.FTokenAdaptorV1Calls
	(*FTokenAdaptorV1_LendFrax)(nil),        // 2: steward.v4.FTokenAdaptorV1.LendFrax
	(*FTokenAdaptorV1_RedeemFraxShare)(nil), // 3: steward.v4.FTokenAdaptorV1.RedeemFraxShare
	(*FTokenAdaptorV1_WithdrawFrax)(nil),    // 4: steward.v4.FTokenAdaptorV1.WithdrawFrax
	(*FTokenAdaptorV1_CallAddInterest)(nil), // 5: steward.v4.FTokenAdaptorV1.CallAddInterest
	(*RevokeApproval)(nil),                  // 6: steward.v4.RevokeApproval
}
var file_f_token_proto_depIdxs = []int32{
	6, // 0: steward.v4.FTokenAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.FTokenAdaptorV1.lend_frax:type_name -> steward.v4.FTokenAdaptorV1.LendFrax
	3, // 2: steward.v4.FTokenAdaptorV1.redeem_frax_share:type_name -> steward.v4.FTokenAdaptorV1.RedeemFraxShare
	4, // 3: steward.v4.FTokenAdaptorV1.withdraw_frax:type_name -> steward.v4.FTokenAdaptorV1.WithdrawFrax
	5, // 4: steward.v4.FTokenAdaptorV1.call_add_interest:type_name -> steward.v4.FTokenAdaptorV1.CallAddInterest
	0, // 5: steward.v4.FTokenAdaptorV1Calls.calls:type_name -> steward.v4.FTokenAdaptorV1
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_f_token_proto_init() }
func file_f_token_proto_init() {
	if File_f_token_proto != nil {
		return
	}
	file_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_f_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1); i {
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
		file_f_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1Calls); i {
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
		file_f_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1_LendFrax); i {
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
		file_f_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1_RedeemFraxShare); i {
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
		file_f_token_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1_WithdrawFrax); i {
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
		file_f_token_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*FTokenAdaptorV1_CallAddInterest); i {
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
	file_f_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*FTokenAdaptorV1_RevokeApproval)(nil),
		(*FTokenAdaptorV1_LendFrax_)(nil),
		(*FTokenAdaptorV1_RedeemFraxShare_)(nil),
		(*FTokenAdaptorV1_WithdrawFrax_)(nil),
		(*FTokenAdaptorV1_CallAddInterest_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_f_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   6,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_f_token_proto_goTypes,
		DependencyIndexes: file_f_token_proto_depIdxs,
		MessageInfos:      file_f_token_proto_msgTypes,
	}.Build()
	File_f_token_proto = out.File
	file_f_token_proto_rawDesc = nil
	file_f_token_proto_goTypes = nil
	file_f_token_proto_depIdxs = nil
}
