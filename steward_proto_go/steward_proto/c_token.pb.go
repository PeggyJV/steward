//
// Protos for function calls to the Compound CToken adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.26.0
// 	protoc        v3.21.12
// source: adaptors/compound/c_token.proto

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

// Represents call data for the Compound C Token adaptor V2, managing lending positions on Compound.
type CompoundCTokenAdaptorV2 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*CompoundCTokenAdaptorV2_RevokeApproval
	//	*CompoundCTokenAdaptorV2_DepositToCompound_
	//	*CompoundCTokenAdaptorV2_WithdrawFromCompound_
	//	*CompoundCTokenAdaptorV2_ClaimComp_
	Function isCompoundCTokenAdaptorV2_Function `protobuf_oneof:"function"`
}

func (x *CompoundCTokenAdaptorV2) Reset() {
	*x = CompoundCTokenAdaptorV2{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorV2) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorV2) ProtoMessage() {}

func (x *CompoundCTokenAdaptorV2) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptorV2.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorV2) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0}
}

func (m *CompoundCTokenAdaptorV2) GetFunction() isCompoundCTokenAdaptorV2_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *CompoundCTokenAdaptorV2) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptorV2_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *CompoundCTokenAdaptorV2) GetDepositToCompound() *CompoundCTokenAdaptorV2_DepositToCompound {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptorV2_DepositToCompound_); ok {
		return x.DepositToCompound
	}
	return nil
}

func (x *CompoundCTokenAdaptorV2) GetWithdrawFromCompound() *CompoundCTokenAdaptorV2_WithdrawFromCompound {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptorV2_WithdrawFromCompound_); ok {
		return x.WithdrawFromCompound
	}
	return nil
}

func (x *CompoundCTokenAdaptorV2) GetClaimComp() *CompoundCTokenAdaptorV2_ClaimComp {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptorV2_ClaimComp_); ok {
		return x.ClaimComp
	}
	return nil
}

type isCompoundCTokenAdaptorV2_Function interface {
	isCompoundCTokenAdaptorV2_Function()
}

type CompoundCTokenAdaptorV2_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type CompoundCTokenAdaptorV2_DepositToCompound_ struct {
	// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
	DepositToCompound *CompoundCTokenAdaptorV2_DepositToCompound `protobuf:"bytes,2,opt,name=deposit_to_compound,json=depositToCompound,proto3,oneof"`
}

type CompoundCTokenAdaptorV2_WithdrawFromCompound_ struct {
	// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
	WithdrawFromCompound *CompoundCTokenAdaptorV2_WithdrawFromCompound `protobuf:"bytes,3,opt,name=withdraw_from_compound,json=withdrawFromCompound,proto3,oneof"`
}

type CompoundCTokenAdaptorV2_ClaimComp_ struct {
	// Represents function `claimComp()`
	ClaimComp *CompoundCTokenAdaptorV2_ClaimComp `protobuf:"bytes,4,opt,name=claim_comp,json=claimComp,proto3,oneof"`
}

func (*CompoundCTokenAdaptorV2_RevokeApproval) isCompoundCTokenAdaptorV2_Function() {}

func (*CompoundCTokenAdaptorV2_DepositToCompound_) isCompoundCTokenAdaptorV2_Function() {}

func (*CompoundCTokenAdaptorV2_WithdrawFromCompound_) isCompoundCTokenAdaptorV2_Function() {}

func (*CompoundCTokenAdaptorV2_ClaimComp_) isCompoundCTokenAdaptorV2_Function() {}

type CompoundCTokenAdaptorV2Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*CompoundCTokenAdaptorV2 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *CompoundCTokenAdaptorV2Calls) Reset() {
	*x = CompoundCTokenAdaptorV2Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorV2Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorV2Calls) ProtoMessage() {}

func (x *CompoundCTokenAdaptorV2Calls) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptorV2Calls.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorV2Calls) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{1}
}

func (x *CompoundCTokenAdaptorV2Calls) GetCalls() []*CompoundCTokenAdaptorV2 {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to lend assets on Compound.
//
// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
type CompoundCTokenAdaptorV2_DepositToCompound struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Market          string `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	AmountToDeposit string `protobuf:"bytes,2,opt,name=amount_to_deposit,json=amountToDeposit,proto3" json:"amount_to_deposit,omitempty"`
}

func (x *CompoundCTokenAdaptorV2_DepositToCompound) Reset() {
	*x = CompoundCTokenAdaptorV2_DepositToCompound{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorV2_DepositToCompound) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorV2_DepositToCompound) ProtoMessage() {}

func (x *CompoundCTokenAdaptorV2_DepositToCompound) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptorV2_DepositToCompound.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorV2_DepositToCompound) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *CompoundCTokenAdaptorV2_DepositToCompound) GetMarket() string {
	if x != nil {
		return x.Market
	}
	return ""
}

func (x *CompoundCTokenAdaptorV2_DepositToCompound) GetAmountToDeposit() string {
	if x != nil {
		return x.AmountToDeposit
	}
	return ""
}

//
// Allows strategists to withdraw assets from Compound.
//
// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
type CompoundCTokenAdaptorV2_WithdrawFromCompound struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Market           string `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	AmountToWithdraw string `protobuf:"bytes,2,opt,name=amount_to_withdraw,json=amountToWithdraw,proto3" json:"amount_to_withdraw,omitempty"`
}

func (x *CompoundCTokenAdaptorV2_WithdrawFromCompound) Reset() {
	*x = CompoundCTokenAdaptorV2_WithdrawFromCompound{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorV2_WithdrawFromCompound) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorV2_WithdrawFromCompound) ProtoMessage() {}

func (x *CompoundCTokenAdaptorV2_WithdrawFromCompound) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptorV2_WithdrawFromCompound.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorV2_WithdrawFromCompound) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *CompoundCTokenAdaptorV2_WithdrawFromCompound) GetMarket() string {
	if x != nil {
		return x.Market
	}
	return ""
}

func (x *CompoundCTokenAdaptorV2_WithdrawFromCompound) GetAmountToWithdraw() string {
	if x != nil {
		return x.AmountToWithdraw
	}
	return ""
}

//
// Allows strategists to claim COMP rewards.
//
// Represents function `claimComp()`
type CompoundCTokenAdaptorV2_ClaimComp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *CompoundCTokenAdaptorV2_ClaimComp) Reset() {
	*x = CompoundCTokenAdaptorV2_ClaimComp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorV2_ClaimComp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorV2_ClaimComp) ProtoMessage() {}

func (x *CompoundCTokenAdaptorV2_ClaimComp) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptorV2_ClaimComp.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorV2_ClaimComp) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 2}
}

var File_adaptors_compound_c_token_proto protoreflect.FileDescriptor

var file_adaptors_compound_c_token_proto_rawDesc = []byte{
	0x0a, 0x1f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
	0x75, 0x6e, 0x64, 0x2f, 0x63, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x13, 0x61,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x22, 0xdb, 0x04, 0x0a, 0x17, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43,
	0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x12, 0x45,
	0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61,
	0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f,
	0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70,
	0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x67, 0x0a, 0x13, 0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74,
	0x5f, 0x74, 0x6f, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x35, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e,
	0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x2e, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54,
	0x6f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x48, 0x00, 0x52, 0x11, 0x64, 0x65, 0x70,
	0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x70,
	0x0a, 0x16, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f,
	0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x38,
	0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x43, 0x6f, 0x6d, 0x70,
	0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f,
	0x72, 0x56, 0x32, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d,
	0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x48, 0x00, 0x52, 0x14, 0x77, 0x69, 0x74, 0x68,
	0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64,
	0x12, 0x4e, 0x0a, 0x0a, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76,
	0x34, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x2e, 0x43, 0x6c, 0x61, 0x69, 0x6d, 0x43,
	0x6f, 0x6d, 0x70, 0x48, 0x00, 0x52, 0x09, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70,
	0x1a, 0x57, 0x0a, 0x11, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x43, 0x6f, 0x6d,
	0x70, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x2a, 0x0a,
	0x11, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x1a, 0x5c, 0x0a, 0x14, 0x57, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e,
	0x64, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x2c, 0x0a, 0x12, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x57,
	0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x1a, 0x0b, 0x0a, 0x09, 0x43, 0x6c, 0x61, 0x69, 0x6d,
	0x43, 0x6f, 0x6d, 0x70, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x22, 0x59, 0x0a, 0x1c, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x43, 0x61, 0x6c, 0x6c, 0x73,
	0x12, 0x39, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
	0x23, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x43, 0x6f, 0x6d,
	0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74,
	0x6f, 0x72, 0x56, 0x32, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_adaptors_compound_c_token_proto_rawDescOnce sync.Once
	file_adaptors_compound_c_token_proto_rawDescData = file_adaptors_compound_c_token_proto_rawDesc
)

func file_adaptors_compound_c_token_proto_rawDescGZIP() []byte {
	file_adaptors_compound_c_token_proto_rawDescOnce.Do(func() {
		file_adaptors_compound_c_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_adaptors_compound_c_token_proto_rawDescData)
	})
	return file_adaptors_compound_c_token_proto_rawDescData
}

var file_adaptors_compound_c_token_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_adaptors_compound_c_token_proto_goTypes = []interface{}{
	(*CompoundCTokenAdaptorV2)(nil),                      // 0: steward.v4.CompoundCTokenAdaptorV2
	(*CompoundCTokenAdaptorV2Calls)(nil),                 // 1: steward.v4.CompoundCTokenAdaptorV2Calls
	(*CompoundCTokenAdaptorV2_DepositToCompound)(nil),    // 2: steward.v4.CompoundCTokenAdaptorV2.DepositToCompound
	(*CompoundCTokenAdaptorV2_WithdrawFromCompound)(nil), // 3: steward.v4.CompoundCTokenAdaptorV2.WithdrawFromCompound
	(*CompoundCTokenAdaptorV2_ClaimComp)(nil),            // 4: steward.v4.CompoundCTokenAdaptorV2.ClaimComp
	(*RevokeApproval)(nil),                               // 5: steward.v4.RevokeApproval
}
var file_adaptors_compound_c_token_proto_depIdxs = []int32{
	5, // 0: steward.v4.CompoundCTokenAdaptorV2.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.CompoundCTokenAdaptorV2.deposit_to_compound:type_name -> steward.v4.CompoundCTokenAdaptorV2.DepositToCompound
	3, // 2: steward.v4.CompoundCTokenAdaptorV2.withdraw_from_compound:type_name -> steward.v4.CompoundCTokenAdaptorV2.WithdrawFromCompound
	4, // 3: steward.v4.CompoundCTokenAdaptorV2.claim_comp:type_name -> steward.v4.CompoundCTokenAdaptorV2.ClaimComp
	0, // 4: steward.v4.CompoundCTokenAdaptorV2Calls.calls:type_name -> steward.v4.CompoundCTokenAdaptorV2
	5, // [5:5] is the sub-list for method output_type
	5, // [5:5] is the sub-list for method input_type
	5, // [5:5] is the sub-list for extension type_name
	5, // [5:5] is the sub-list for extension extendee
	0, // [0:5] is the sub-list for field type_name
}

func init() { file_adaptors_compound_c_token_proto_init() }
func file_adaptors_compound_c_token_proto_init() {
	if File_adaptors_compound_c_token_proto != nil {
		return
	}
	file_adaptors_base_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_adaptors_compound_c_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptorV2); i {
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
		file_adaptors_compound_c_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptorV2Calls); i {
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
		file_adaptors_compound_c_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptorV2_DepositToCompound); i {
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
		file_adaptors_compound_c_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptorV2_WithdrawFromCompound); i {
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
		file_adaptors_compound_c_token_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptorV2_ClaimComp); i {
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
	file_adaptors_compound_c_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*CompoundCTokenAdaptorV2_RevokeApproval)(nil),
		(*CompoundCTokenAdaptorV2_DepositToCompound_)(nil),
		(*CompoundCTokenAdaptorV2_WithdrawFromCompound_)(nil),
		(*CompoundCTokenAdaptorV2_ClaimComp_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_adaptors_compound_c_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_adaptors_compound_c_token_proto_goTypes,
		DependencyIndexes: file_adaptors_compound_c_token_proto_depIdxs,
		MessageInfos:      file_adaptors_compound_c_token_proto_msgTypes,
	}.Build()
	File_adaptors_compound_c_token_proto = out.File
	file_adaptors_compound_c_token_proto_rawDesc = nil
	file_adaptors_compound_c_token_proto_goTypes = nil
	file_adaptors_compound_c_token_proto_depIdxs = nil
}
