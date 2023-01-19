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

// Represents call data for the Compound C Token adaptor, managing lending positions on Compound.
type CompoundCTokenAdaptor struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//	*CompoundCTokenAdaptor_DepositToCompound_
	//	*CompoundCTokenAdaptor_WithdrawFromCompound_
	//	*CompoundCTokenAdaptor_ClaimComp_
	//	*CompoundCTokenAdaptor_ClaimCompAndSwap_
	Function isCompoundCTokenAdaptor_Function `protobuf_oneof:"function"`
}

func (x *CompoundCTokenAdaptor) Reset() {
	*x = CompoundCTokenAdaptor{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptor) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptor) ProtoMessage() {}

func (x *CompoundCTokenAdaptor) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use CompoundCTokenAdaptor.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptor) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0}
}

func (m *CompoundCTokenAdaptor) GetFunction() isCompoundCTokenAdaptor_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *CompoundCTokenAdaptor) GetDepositToCompound() *CompoundCTokenAdaptor_DepositToCompound {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptor_DepositToCompound_); ok {
		return x.DepositToCompound
	}
	return nil
}

func (x *CompoundCTokenAdaptor) GetWithdrawFromCompound() *CompoundCTokenAdaptor_WithdrawFromCompound {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptor_WithdrawFromCompound_); ok {
		return x.WithdrawFromCompound
	}
	return nil
}

func (x *CompoundCTokenAdaptor) GetClaimComp() *CompoundCTokenAdaptor_ClaimComp {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptor_ClaimComp_); ok {
		return x.ClaimComp
	}
	return nil
}

func (x *CompoundCTokenAdaptor) GetClaimCompAndSwap() *CompoundCTokenAdaptor_ClaimCompAndSwap {
	if x, ok := x.GetFunction().(*CompoundCTokenAdaptor_ClaimCompAndSwap_); ok {
		return x.ClaimCompAndSwap
	}
	return nil
}

type isCompoundCTokenAdaptor_Function interface {
	isCompoundCTokenAdaptor_Function()
}

type CompoundCTokenAdaptor_DepositToCompound_ struct {
	// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
	DepositToCompound *CompoundCTokenAdaptor_DepositToCompound `protobuf:"bytes,1,opt,name=deposit_to_compound,json=depositToCompound,proto3,oneof"`
}

type CompoundCTokenAdaptor_WithdrawFromCompound_ struct {
	// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
	WithdrawFromCompound *CompoundCTokenAdaptor_WithdrawFromCompound `protobuf:"bytes,2,opt,name=withdraw_from_compound,json=withdrawFromCompound,proto3,oneof"`
}

type CompoundCTokenAdaptor_ClaimComp_ struct {
	// Represents function `claimComp()`
	ClaimComp *CompoundCTokenAdaptor_ClaimComp `protobuf:"bytes,3,opt,name=claim_comp,json=claimComp,proto3,oneof"`
}

type CompoundCTokenAdaptor_ClaimCompAndSwap_ struct {
	// Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
	ClaimCompAndSwap *CompoundCTokenAdaptor_ClaimCompAndSwap `protobuf:"bytes,4,opt,name=claim_comp_and_swap,json=claimCompAndSwap,proto3,oneof"`
}

func (*CompoundCTokenAdaptor_DepositToCompound_) isCompoundCTokenAdaptor_Function() {}

func (*CompoundCTokenAdaptor_WithdrawFromCompound_) isCompoundCTokenAdaptor_Function() {}

func (*CompoundCTokenAdaptor_ClaimComp_) isCompoundCTokenAdaptor_Function() {}

func (*CompoundCTokenAdaptor_ClaimCompAndSwap_) isCompoundCTokenAdaptor_Function() {}

type CompoundCTokenAdaptorCalls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*CompoundCTokenAdaptor `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *CompoundCTokenAdaptorCalls) Reset() {
	*x = CompoundCTokenAdaptorCalls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptorCalls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptorCalls) ProtoMessage() {}

func (x *CompoundCTokenAdaptorCalls) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use CompoundCTokenAdaptorCalls.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptorCalls) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{1}
}

func (x *CompoundCTokenAdaptorCalls) GetCalls() []*CompoundCTokenAdaptor {
	if x != nil {
		return x.Calls
	}
	return nil
}

//
// Allows strategists to lend assets on Compound.
//
// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
type CompoundCTokenAdaptor_DepositToCompound struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Market          string `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	AmountToDeposit string `protobuf:"bytes,2,opt,name=amount_to_deposit,json=amountToDeposit,proto3" json:"amount_to_deposit,omitempty"`
}

func (x *CompoundCTokenAdaptor_DepositToCompound) Reset() {
	*x = CompoundCTokenAdaptor_DepositToCompound{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptor_DepositToCompound) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptor_DepositToCompound) ProtoMessage() {}

func (x *CompoundCTokenAdaptor_DepositToCompound) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use CompoundCTokenAdaptor_DepositToCompound.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptor_DepositToCompound) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *CompoundCTokenAdaptor_DepositToCompound) GetMarket() string {
	if x != nil {
		return x.Market
	}
	return ""
}

func (x *CompoundCTokenAdaptor_DepositToCompound) GetAmountToDeposit() string {
	if x != nil {
		return x.AmountToDeposit
	}
	return ""
}

//
// Allows strategists to withdraw assets from Compound.
//
// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
type CompoundCTokenAdaptor_WithdrawFromCompound struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Market           string `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	AmountToWithdraw string `protobuf:"bytes,2,opt,name=amount_to_withdraw,json=amountToWithdraw,proto3" json:"amount_to_withdraw,omitempty"`
}

func (x *CompoundCTokenAdaptor_WithdrawFromCompound) Reset() {
	*x = CompoundCTokenAdaptor_WithdrawFromCompound{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptor_WithdrawFromCompound) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptor_WithdrawFromCompound) ProtoMessage() {}

func (x *CompoundCTokenAdaptor_WithdrawFromCompound) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use CompoundCTokenAdaptor_WithdrawFromCompound.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptor_WithdrawFromCompound) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *CompoundCTokenAdaptor_WithdrawFromCompound) GetMarket() string {
	if x != nil {
		return x.Market
	}
	return ""
}

func (x *CompoundCTokenAdaptor_WithdrawFromCompound) GetAmountToWithdraw() string {
	if x != nil {
		return x.AmountToWithdraw
	}
	return ""
}

//
// Allows strategists to claim COMP rewards.
//
// Represents function `claimComp()`
type CompoundCTokenAdaptor_ClaimComp struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields
}

func (x *CompoundCTokenAdaptor_ClaimComp) Reset() {
	*x = CompoundCTokenAdaptor_ClaimComp{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptor_ClaimComp) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptor_ClaimComp) ProtoMessage() {}

func (x *CompoundCTokenAdaptor_ClaimComp) ProtoReflect() protoreflect.Message {
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

// Deprecated: Use CompoundCTokenAdaptor_ClaimComp.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptor_ClaimComp) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 2}
}

//
// Allows strategists to claim COMP and immediately swap it using oracleSwap.
//
// Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
type CompoundCTokenAdaptor_ClaimCompAndSwap struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	AssetOut string            `protobuf:"bytes,1,opt,name=asset_out,json=assetOut,proto3" json:"asset_out,omitempty"`
	Exchange Exchange          `protobuf:"varint,2,opt,name=exchange,proto3,enum=steward.v2.Exchange" json:"exchange,omitempty"`
	Params   *OracleSwapParams `protobuf:"bytes,3,opt,name=params,proto3" json:"params,omitempty"`
	Slippage uint64            `protobuf:"varint,4,opt,name=slippage,proto3" json:"slippage,omitempty"`
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) Reset() {
	*x = CompoundCTokenAdaptor_ClaimCompAndSwap{}
	if protoimpl.UnsafeEnabled {
		mi := &file_adaptors_compound_c_token_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CompoundCTokenAdaptor_ClaimCompAndSwap) ProtoMessage() {}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) ProtoReflect() protoreflect.Message {
	mi := &file_adaptors_compound_c_token_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use CompoundCTokenAdaptor_ClaimCompAndSwap.ProtoReflect.Descriptor instead.
func (*CompoundCTokenAdaptor_ClaimCompAndSwap) Descriptor() ([]byte, []int) {
	return file_adaptors_compound_c_token_proto_rawDescGZIP(), []int{0, 3}
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) GetAssetOut() string {
	if x != nil {
		return x.AssetOut
	}
	return ""
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) GetExchange() Exchange {
	if x != nil {
		return x.Exchange
	}
	return Exchange_EXCHANGE_UNSPECIFIED
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) GetParams() *OracleSwapParams {
	if x != nil {
		return x.Params
	}
	return nil
}

func (x *CompoundCTokenAdaptor_ClaimCompAndSwap) GetSlippage() uint64 {
	if x != nil {
		return x.Slippage
	}
	return 0
}

var File_adaptors_compound_c_token_proto protoreflect.FileDescriptor

var file_adaptors_compound_c_token_proto_rawDesc = []byte{
	0x0a, 0x1f, 0x61, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
	0x75, 0x6e, 0x64, 0x2f, 0x63, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x32, 0x1a, 0x13, 0x61,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x22, 0xa7, 0x06, 0x0a, 0x15, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x65, 0x0a, 0x13, 0x64, 0x65,
	0x70, 0x6f, 0x73, 0x69, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e,
	0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x33, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x44, 0x65, 0x70, 0x6f, 0x73,
	0x69, 0x74, 0x54, 0x6f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x48, 0x00, 0x52, 0x11,
	0x64, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54, 0x6f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e,
	0x64, 0x12, 0x6e, 0x0a, 0x16, 0x77, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x5f, 0x66, 0x72,
	0x6f, 0x6d, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x36, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x32, 0x2e, 0x43,
	0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x2e, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f,
	0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x48, 0x00, 0x52, 0x14, 0x77, 0x69, 0x74,
	0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e,
	0x64, 0x12, 0x4c, 0x0a, 0x0a, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65,
	0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x43, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f,
	0x6d, 0x70, 0x48, 0x00, 0x52, 0x09, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x12,
	0x63, 0x0a, 0x13, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x5f, 0x63, 0x6f, 0x6d, 0x70, 0x5f, 0x61, 0x6e,
	0x64, 0x5f, 0x73, 0x77, 0x61, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75,
	0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x2e,
	0x43, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x41, 0x6e, 0x64, 0x53, 0x77, 0x61, 0x70,
	0x48, 0x00, 0x52, 0x10, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x41, 0x6e, 0x64,
	0x53, 0x77, 0x61, 0x70, 0x1a, 0x57, 0x0a, 0x11, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x54,
	0x6f, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72,
	0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65,
	0x74, 0x12, 0x2a, 0x0a, 0x11, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x64,
	0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0f, 0x61, 0x6d,
	0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x1a, 0x5c, 0x0a,
	0x14, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x43, 0x6f, 0x6d,
	0x70, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x16, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x2c, 0x0a,
	0x12, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x5f, 0x77, 0x69, 0x74, 0x68, 0x64,
	0x72, 0x61, 0x77, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x61, 0x6d, 0x6f, 0x75, 0x6e,
	0x74, 0x54, 0x6f, 0x57, 0x69, 0x74, 0x68, 0x64, 0x72, 0x61, 0x77, 0x1a, 0x0b, 0x0a, 0x09, 0x43,
	0x6c, 0x61, 0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x1a, 0xb3, 0x01, 0x0a, 0x10, 0x43, 0x6c, 0x61,
	0x69, 0x6d, 0x43, 0x6f, 0x6d, 0x70, 0x41, 0x6e, 0x64, 0x53, 0x77, 0x61, 0x70, 0x12, 0x1b, 0x0a,
	0x09, 0x61, 0x73, 0x73, 0x65, 0x74, 0x5f, 0x6f, 0x75, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x08, 0x61, 0x73, 0x73, 0x65, 0x74, 0x4f, 0x75, 0x74, 0x12, 0x30, 0x0a, 0x08, 0x65, 0x78,
	0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x32, 0x2e, 0x45, 0x78, 0x63, 0x68, 0x61, 0x6e,
	0x67, 0x65, 0x52, 0x08, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x34, 0x0a, 0x06,
	0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x32, 0x2e, 0x4f, 0x72, 0x61, 0x63, 0x6c, 0x65,
	0x53, 0x77, 0x61, 0x70, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61,
	0x6d, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x73, 0x6c, 0x69, 0x70, 0x70, 0x61, 0x67, 0x65, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x73, 0x6c, 0x69, 0x70, 0x70, 0x61, 0x67, 0x65, 0x42, 0x0a,
	0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x55, 0x0a, 0x1a, 0x43, 0x6f,
	0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x37, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c,
	0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x32, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x6f, 0x75, 0x6e, 0x64, 0x43, 0x54, 0x6f,
	0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c,
	0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x5f, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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

var file_adaptors_compound_c_token_proto_msgTypes = make([]protoimpl.MessageInfo, 6)
var file_adaptors_compound_c_token_proto_goTypes = []interface{}{
	(*CompoundCTokenAdaptor)(nil),                      // 0: steward.v2.CompoundCTokenAdaptor
	(*CompoundCTokenAdaptorCalls)(nil),                 // 1: steward.v2.CompoundCTokenAdaptorCalls
	(*CompoundCTokenAdaptor_DepositToCompound)(nil),    // 2: steward.v2.CompoundCTokenAdaptor.DepositToCompound
	(*CompoundCTokenAdaptor_WithdrawFromCompound)(nil), // 3: steward.v2.CompoundCTokenAdaptor.WithdrawFromCompound
	(*CompoundCTokenAdaptor_ClaimComp)(nil),            // 4: steward.v2.CompoundCTokenAdaptor.ClaimComp
	(*CompoundCTokenAdaptor_ClaimCompAndSwap)(nil),     // 5: steward.v2.CompoundCTokenAdaptor.ClaimCompAndSwap
	(Exchange)(0),            // 6: steward.v2.Exchange
	(*OracleSwapParams)(nil), // 7: steward.v2.OracleSwapParams
}
var file_adaptors_compound_c_token_proto_depIdxs = []int32{
	2, // 0: steward.v2.CompoundCTokenAdaptor.deposit_to_compound:type_name -> steward.v2.CompoundCTokenAdaptor.DepositToCompound
	3, // 1: steward.v2.CompoundCTokenAdaptor.withdraw_from_compound:type_name -> steward.v2.CompoundCTokenAdaptor.WithdrawFromCompound
	4, // 2: steward.v2.CompoundCTokenAdaptor.claim_comp:type_name -> steward.v2.CompoundCTokenAdaptor.ClaimComp
	5, // 3: steward.v2.CompoundCTokenAdaptor.claim_comp_and_swap:type_name -> steward.v2.CompoundCTokenAdaptor.ClaimCompAndSwap
	0, // 4: steward.v2.CompoundCTokenAdaptorCalls.calls:type_name -> steward.v2.CompoundCTokenAdaptor
	6, // 5: steward.v2.CompoundCTokenAdaptor.ClaimCompAndSwap.exchange:type_name -> steward.v2.Exchange
	7, // 6: steward.v2.CompoundCTokenAdaptor.ClaimCompAndSwap.params:type_name -> steward.v2.OracleSwapParams
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_adaptors_compound_c_token_proto_init() }
func file_adaptors_compound_c_token_proto_init() {
	if File_adaptors_compound_c_token_proto != nil {
		return
	}
	file_adaptors_base_proto_init()
	file_common_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_adaptors_compound_c_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptor); i {
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
			switch v := v.(*CompoundCTokenAdaptorCalls); i {
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
			switch v := v.(*CompoundCTokenAdaptor_DepositToCompound); i {
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
			switch v := v.(*CompoundCTokenAdaptor_WithdrawFromCompound); i {
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
			switch v := v.(*CompoundCTokenAdaptor_ClaimComp); i {
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
		file_adaptors_compound_c_token_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*CompoundCTokenAdaptor_ClaimCompAndSwap); i {
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
		(*CompoundCTokenAdaptor_DepositToCompound_)(nil),
		(*CompoundCTokenAdaptor_WithdrawFromCompound_)(nil),
		(*CompoundCTokenAdaptor_ClaimComp_)(nil),
		(*CompoundCTokenAdaptor_ClaimCompAndSwap_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_adaptors_compound_c_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   6,
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
