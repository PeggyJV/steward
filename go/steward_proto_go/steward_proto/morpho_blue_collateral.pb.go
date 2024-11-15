//
// Protos for function calls to the Morpho Blue Collateral adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.33.0
// 	protoc        v5.27.1
// source: morpho_blue_collateral.proto

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

// Represents call data for the Morpho Blue Collateral adaptor.
type MorphoBlueCollateralAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*MorphoBlueCollateralAdaptorV1_RevokeApproval
	//	*MorphoBlueCollateralAdaptorV1_AddCollateral_
	//	*MorphoBlueCollateralAdaptorV1_RemoveCollateral_
	Function isMorphoBlueCollateralAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *MorphoBlueCollateralAdaptorV1) Reset() {
	*x = MorphoBlueCollateralAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_collateral_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueCollateralAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueCollateralAdaptorV1) ProtoMessage() {}

func (x *MorphoBlueCollateralAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_collateral_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueCollateralAdaptorV1.ProtoReflect.Descriptor instead.
func (*MorphoBlueCollateralAdaptorV1) Descriptor() ([]byte, []int) {
	return file_morpho_blue_collateral_proto_rawDescGZIP(), []int{0}
}

func (m *MorphoBlueCollateralAdaptorV1) GetFunction() isMorphoBlueCollateralAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *MorphoBlueCollateralAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*MorphoBlueCollateralAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *MorphoBlueCollateralAdaptorV1) GetAddCollateral() *MorphoBlueCollateralAdaptorV1_AddCollateral {
	if x, ok := x.GetFunction().(*MorphoBlueCollateralAdaptorV1_AddCollateral_); ok {
		return x.AddCollateral
	}
	return nil
}

func (x *MorphoBlueCollateralAdaptorV1) GetRemoveCollateral() *MorphoBlueCollateralAdaptorV1_RemoveCollateral {
	if x, ok := x.GetFunction().(*MorphoBlueCollateralAdaptorV1_RemoveCollateral_); ok {
		return x.RemoveCollateral
	}
	return nil
}

type isMorphoBlueCollateralAdaptorV1_Function interface {
	isMorphoBlueCollateralAdaptorV1_Function()
}

type MorphoBlueCollateralAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type MorphoBlueCollateralAdaptorV1_AddCollateral_ struct {
	// Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)`
	AddCollateral *MorphoBlueCollateralAdaptorV1_AddCollateral `protobuf:"bytes,2,opt,name=add_collateral,json=addCollateral,proto3,oneof"`
}

type MorphoBlueCollateralAdaptorV1_RemoveCollateral_ struct {
	// Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)`
	RemoveCollateral *MorphoBlueCollateralAdaptorV1_RemoveCollateral `protobuf:"bytes,3,opt,name=remove_collateral,json=removeCollateral,proto3,oneof"`
}

func (*MorphoBlueCollateralAdaptorV1_RevokeApproval) isMorphoBlueCollateralAdaptorV1_Function() {}

func (*MorphoBlueCollateralAdaptorV1_AddCollateral_) isMorphoBlueCollateralAdaptorV1_Function() {}

func (*MorphoBlueCollateralAdaptorV1_RemoveCollateral_) isMorphoBlueCollateralAdaptorV1_Function() {}

type MorphoBlueCollateralAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*MorphoBlueCollateralAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *MorphoBlueCollateralAdaptorV1Calls) Reset() {
	*x = MorphoBlueCollateralAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_collateral_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueCollateralAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueCollateralAdaptorV1Calls) ProtoMessage() {}

func (x *MorphoBlueCollateralAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_collateral_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueCollateralAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*MorphoBlueCollateralAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_morpho_blue_collateral_proto_rawDescGZIP(), []int{1}
}

func (x *MorphoBlueCollateralAdaptorV1Calls) GetCalls() []*MorphoBlueCollateralAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists to add collateral to the respective cellar position on specified MB Market, enabling borrowing.
//
// Represents function `addCollateral(MarketParams memory _market, uint256 _collateralToDeposit)`
type MorphoBlueCollateralAdaptorV1_AddCollateral struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Identifier of a Morpho Blue Market
	Market *MarketParams `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	// The amount of collateral to add
	CollateralToDeposit string `protobuf:"bytes,2,opt,name=collateral_to_deposit,json=collateralToDeposit,proto3" json:"collateral_to_deposit,omitempty"`
}

func (x *MorphoBlueCollateralAdaptorV1_AddCollateral) Reset() {
	*x = MorphoBlueCollateralAdaptorV1_AddCollateral{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_collateral_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueCollateralAdaptorV1_AddCollateral) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueCollateralAdaptorV1_AddCollateral) ProtoMessage() {}

func (x *MorphoBlueCollateralAdaptorV1_AddCollateral) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_collateral_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueCollateralAdaptorV1_AddCollateral.ProtoReflect.Descriptor instead.
func (*MorphoBlueCollateralAdaptorV1_AddCollateral) Descriptor() ([]byte, []int) {
	return file_morpho_blue_collateral_proto_rawDescGZIP(), []int{0, 0}
}

func (x *MorphoBlueCollateralAdaptorV1_AddCollateral) GetMarket() *MarketParams {
	if x != nil {
		return x.Market
	}
	return nil
}

func (x *MorphoBlueCollateralAdaptorV1_AddCollateral) GetCollateralToDeposit() string {
	if x != nil {
		return x.CollateralToDeposit
	}
	return ""
}

// Allows strategists to remove collateral from the respective cellar position on specified MB Market.
//
// Represents function `removeCollateral(MarketParams memory _market, uint256 _collateralAmount)`
type MorphoBlueCollateralAdaptorV1_RemoveCollateral struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Identifier of a Morpho Blue Market
	Market *MarketParams `protobuf:"bytes,1,opt,name=market,proto3" json:"market,omitempty"`
	// The amount of collateral to remove
	CollateralAmount string `protobuf:"bytes,2,opt,name=collateral_amount,json=collateralAmount,proto3" json:"collateral_amount,omitempty"`
}

func (x *MorphoBlueCollateralAdaptorV1_RemoveCollateral) Reset() {
	*x = MorphoBlueCollateralAdaptorV1_RemoveCollateral{}
	if protoimpl.UnsafeEnabled {
		mi := &file_morpho_blue_collateral_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *MorphoBlueCollateralAdaptorV1_RemoveCollateral) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*MorphoBlueCollateralAdaptorV1_RemoveCollateral) ProtoMessage() {}

func (x *MorphoBlueCollateralAdaptorV1_RemoveCollateral) ProtoReflect() protoreflect.Message {
	mi := &file_morpho_blue_collateral_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use MorphoBlueCollateralAdaptorV1_RemoveCollateral.ProtoReflect.Descriptor instead.
func (*MorphoBlueCollateralAdaptorV1_RemoveCollateral) Descriptor() ([]byte, []int) {
	return file_morpho_blue_collateral_proto_rawDescGZIP(), []int{0, 1}
}

func (x *MorphoBlueCollateralAdaptorV1_RemoveCollateral) GetMarket() *MarketParams {
	if x != nil {
		return x.Market
	}
	return nil
}

func (x *MorphoBlueCollateralAdaptorV1_RemoveCollateral) GetCollateralAmount() string {
	if x != nil {
		return x.CollateralAmount
	}
	return ""
}

var File_morpho_blue_collateral_proto protoreflect.FileDescriptor

var file_morpho_blue_collateral_proto_rawDesc = []byte{
	0x0a, 0x1c, 0x6d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x5f, 0x62, 0x6c, 0x75, 0x65, 0x5f, 0x63, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a,
	0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a, 0x62, 0x61, 0x73, 0x65,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa9, 0x04, 0x0a, 0x1d, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42,
	0x6c, 0x75, 0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61,
	0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65,
	0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76,
	0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72,
	0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x60, 0x0a,
	0x0e, 0x61, 0x64, 0x64, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x43, 0x6f, 0x6c,
	0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31,
	0x2e, 0x41, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00,
	0x52, 0x0d, 0x61, 0x64, 0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x12,
	0x69, 0x0a, 0x11, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x5f, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74,
	0x65, 0x72, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x3a, 0x2e, 0x73, 0x74, 0x65,
	0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c,
	0x75, 0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70,
	0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x43, 0x6f, 0x6c, 0x6c,
	0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x10, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65,
	0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x1a, 0x75, 0x0a, 0x0d, 0x41, 0x64,
	0x64, 0x43, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x30, 0x0a, 0x06, 0x6d,
	0x61, 0x72, 0x6b, 0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x73, 0x74,
	0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x50,
	0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x32, 0x0a,
	0x15, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f, 0x74, 0x6f, 0x5f, 0x64,
	0x65, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x13, 0x63, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x54, 0x6f, 0x44, 0x65, 0x70, 0x6f, 0x73, 0x69,
	0x74, 0x1a, 0x71, 0x0a, 0x10, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x43, 0x6f, 0x6c, 0x6c, 0x61,
	0x74, 0x65, 0x72, 0x61, 0x6c, 0x12, 0x30, 0x0a, 0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x4d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52,
	0x06, 0x6d, 0x61, 0x72, 0x6b, 0x65, 0x74, 0x12, 0x2b, 0x0a, 0x11, 0x63, 0x6f, 0x6c, 0x6c, 0x61,
	0x74, 0x65, 0x72, 0x61, 0x6c, 0x5f, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x10, 0x63, 0x6f, 0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x6d,
	0x6f, 0x75, 0x6e, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e,
	0x22, 0x65, 0x0a, 0x22, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x43, 0x6f,
	0x6c, 0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56,
	0x31, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x3f, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18,
	0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x4d, 0x6f, 0x72, 0x70, 0x68, 0x6f, 0x42, 0x6c, 0x75, 0x65, 0x43, 0x6f, 0x6c,
	0x6c, 0x61, 0x74, 0x65, 0x72, 0x61, 0x6c, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31,
	0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_morpho_blue_collateral_proto_rawDescOnce sync.Once
	file_morpho_blue_collateral_proto_rawDescData = file_morpho_blue_collateral_proto_rawDesc
)

func file_morpho_blue_collateral_proto_rawDescGZIP() []byte {
	file_morpho_blue_collateral_proto_rawDescOnce.Do(func() {
		file_morpho_blue_collateral_proto_rawDescData = protoimpl.X.CompressGZIP(file_morpho_blue_collateral_proto_rawDescData)
	})
	return file_morpho_blue_collateral_proto_rawDescData
}

var file_morpho_blue_collateral_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_morpho_blue_collateral_proto_goTypes = []interface{}{
	(*MorphoBlueCollateralAdaptorV1)(nil),                  // 0: steward.v4.MorphoBlueCollateralAdaptorV1
	(*MorphoBlueCollateralAdaptorV1Calls)(nil),             // 1: steward.v4.MorphoBlueCollateralAdaptorV1Calls
	(*MorphoBlueCollateralAdaptorV1_AddCollateral)(nil),    // 2: steward.v4.MorphoBlueCollateralAdaptorV1.AddCollateral
	(*MorphoBlueCollateralAdaptorV1_RemoveCollateral)(nil), // 3: steward.v4.MorphoBlueCollateralAdaptorV1.RemoveCollateral
	(*RevokeApproval)(nil),                                 // 4: steward.v4.RevokeApproval
	(*MarketParams)(nil),                                   // 5: steward.v4.MarketParams
}
var file_morpho_blue_collateral_proto_depIdxs = []int32{
	4, // 0: steward.v4.MorphoBlueCollateralAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	2, // 1: steward.v4.MorphoBlueCollateralAdaptorV1.add_collateral:type_name -> steward.v4.MorphoBlueCollateralAdaptorV1.AddCollateral
	3, // 2: steward.v4.MorphoBlueCollateralAdaptorV1.remove_collateral:type_name -> steward.v4.MorphoBlueCollateralAdaptorV1.RemoveCollateral
	0, // 3: steward.v4.MorphoBlueCollateralAdaptorV1Calls.calls:type_name -> steward.v4.MorphoBlueCollateralAdaptorV1
	5, // 4: steward.v4.MorphoBlueCollateralAdaptorV1.AddCollateral.market:type_name -> steward.v4.MarketParams
	5, // 5: steward.v4.MorphoBlueCollateralAdaptorV1.RemoveCollateral.market:type_name -> steward.v4.MarketParams
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_morpho_blue_collateral_proto_init() }
func file_morpho_blue_collateral_proto_init() {
	if File_morpho_blue_collateral_proto != nil {
		return
	}
	file_base_proto_init()
	file_common_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_morpho_blue_collateral_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueCollateralAdaptorV1); i {
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
		file_morpho_blue_collateral_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueCollateralAdaptorV1Calls); i {
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
		file_morpho_blue_collateral_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueCollateralAdaptorV1_AddCollateral); i {
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
		file_morpho_blue_collateral_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*MorphoBlueCollateralAdaptorV1_RemoveCollateral); i {
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
	file_morpho_blue_collateral_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*MorphoBlueCollateralAdaptorV1_RevokeApproval)(nil),
		(*MorphoBlueCollateralAdaptorV1_AddCollateral_)(nil),
		(*MorphoBlueCollateralAdaptorV1_RemoveCollateral_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_morpho_blue_collateral_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_morpho_blue_collateral_proto_goTypes,
		DependencyIndexes: file_morpho_blue_collateral_proto_depIdxs,
		MessageInfos:      file_morpho_blue_collateral_proto_msgTypes,
	}.Build()
	File_morpho_blue_collateral_proto = out.File
	file_morpho_blue_collateral_proto_rawDesc = nil
	file_morpho_blue_collateral_proto_goTypes = nil
	file_morpho_blue_collateral_proto_depIdxs = nil
}
