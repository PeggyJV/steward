// Code generated by protoc-gen-go. DO NOT EDIT.
// source: steward.proto

package integration_tests

import (
	fmt "fmt"
	proto "github.com/golang/protobuf/proto"
	math "math"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion3 // please upgrade the proto package

type SubmitRequest struct {
	CellarId string `protobuf:"bytes,1,opt,name=cellar_id,json=cellarId,proto3" json:"cellar_id,omitempty"`
	// Types that are valid to be assigned to CallData:
	//	*SubmitRequest_AaveV2Stablecoin
	CallData             isSubmitRequest_CallData `protobuf_oneof:"call_data"`
	XXX_NoUnkeyedLiteral struct{}                 `json:"-"`
	XXX_unrecognized     []byte                   `json:"-"`
	XXX_sizecache        int32                    `json:"-"`
}

func (m *SubmitRequest) Reset()         { *m = SubmitRequest{} }
func (m *SubmitRequest) String() string { return proto.CompactTextString(m) }
func (*SubmitRequest) ProtoMessage()    {}
func (*SubmitRequest) Descriptor() ([]byte, []int) {
	return fileDescriptor_f13fb30925f6e913, []int{0}
}

func (m *SubmitRequest) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_SubmitRequest.Unmarshal(m, b)
}
func (m *SubmitRequest) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_SubmitRequest.Marshal(b, m, deterministic)
}
func (m *SubmitRequest) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SubmitRequest.Merge(m, src)
}
func (m *SubmitRequest) XXX_Size() int {
	return xxx_messageInfo_SubmitRequest.Size(m)
}
func (m *SubmitRequest) XXX_DiscardUnknown() {
	xxx_messageInfo_SubmitRequest.DiscardUnknown(m)
}

var xxx_messageInfo_SubmitRequest proto.InternalMessageInfo

func (m *SubmitRequest) GetCellarId() string {
	if m != nil {
		return m.CellarId
	}
	return ""
}

type isSubmitRequest_CallData interface {
	isSubmitRequest_CallData()
}

type SubmitRequest_AaveV2Stablecoin struct {
	AaveV2Stablecoin *AaveV2Stablecoin `protobuf:"bytes,2,opt,name=aave_v2_stablecoin,json=aaveV2Stablecoin,proto3,oneof"`
}

func (*SubmitRequest_AaveV2Stablecoin) isSubmitRequest_CallData() {}

func (m *SubmitRequest) GetCallData() isSubmitRequest_CallData {
	if m != nil {
		return m.CallData
	}
	return nil
}

func (m *SubmitRequest) GetAaveV2Stablecoin() *AaveV2Stablecoin {
	if x, ok := m.GetCallData().(*SubmitRequest_AaveV2Stablecoin); ok {
		return x.AaveV2Stablecoin
	}
	return nil
}

// XXX_OneofWrappers is for the internal use of the proto package.
func (*SubmitRequest) XXX_OneofWrappers() []interface{} {
	return []interface{}{
		(*SubmitRequest_AaveV2Stablecoin)(nil),
	}
}

type SubmitResponse struct {
	XXX_NoUnkeyedLiteral struct{} `json:"-"`
	XXX_unrecognized     []byte   `json:"-"`
	XXX_sizecache        int32    `json:"-"`
}

func (m *SubmitResponse) Reset()         { *m = SubmitResponse{} }
func (m *SubmitResponse) String() string { return proto.CompactTextString(m) }
func (*SubmitResponse) ProtoMessage()    {}
func (*SubmitResponse) Descriptor() ([]byte, []int) {
	return fileDescriptor_f13fb30925f6e913, []int{1}
}

func (m *SubmitResponse) XXX_Unmarshal(b []byte) error {
	return xxx_messageInfo_SubmitResponse.Unmarshal(m, b)
}
func (m *SubmitResponse) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	return xxx_messageInfo_SubmitResponse.Marshal(b, m, deterministic)
}
func (m *SubmitResponse) XXX_Merge(src proto.Message) {
	xxx_messageInfo_SubmitResponse.Merge(m, src)
}
func (m *SubmitResponse) XXX_Size() int {
	return xxx_messageInfo_SubmitResponse.Size(m)
}
func (m *SubmitResponse) XXX_DiscardUnknown() {
	xxx_messageInfo_SubmitResponse.DiscardUnknown(m)
}

var xxx_messageInfo_SubmitResponse proto.InternalMessageInfo

func init() {
	proto.RegisterType((*SubmitRequest)(nil), "steward.v1.SubmitRequest")
	proto.RegisterType((*SubmitResponse)(nil), "steward.v1.SubmitResponse")
}

func init() { proto.RegisterFile("steward.proto", fileDescriptor_f13fb30925f6e913) }

var fileDescriptor_f13fb30925f6e913 = []byte{
	// 221 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xe2, 0xe2, 0x2d, 0x2e, 0x49, 0x2d,
	0x4f, 0x2c, 0x4a, 0xd1, 0x2b, 0x28, 0xca, 0x2f, 0xc9, 0x17, 0xe2, 0x82, 0x71, 0xcb, 0x0c, 0xa5,
	0x24, 0x12, 0x13, 0xcb, 0x52, 0xe3, 0xcb, 0x8c, 0xe2, 0x8b, 0x4b, 0x12, 0x93, 0x72, 0x52, 0x93,
	0xf3, 0x33, 0xf3, 0x20, 0xaa, 0x94, 0xda, 0x19, 0xb9, 0x78, 0x83, 0x4b, 0x93, 0x72, 0x33, 0x4b,
	0x82, 0x52, 0x0b, 0x4b, 0x53, 0x8b, 0x4b, 0x84, 0xa4, 0xb9, 0x38, 0x93, 0x53, 0x73, 0x72, 0x12,
	0x8b, 0xe2, 0x33, 0x53, 0x24, 0x18, 0x15, 0x18, 0x35, 0x38, 0x83, 0x38, 0x20, 0x02, 0x9e, 0x29,
	0x42, 0x3e, 0x5c, 0x42, 0x98, 0x46, 0x49, 0x30, 0x29, 0x30, 0x6a, 0x70, 0x1b, 0xc9, 0xe8, 0x21,
	0x6c, 0xd4, 0x73, 0x4c, 0x2c, 0x4b, 0x0d, 0x33, 0x0a, 0x86, 0xab, 0xf1, 0x60, 0x08, 0x12, 0x48,
	0x44, 0x13, 0x73, 0xe2, 0xe6, 0xe2, 0x4c, 0x4e, 0xcc, 0xc9, 0x89, 0x4f, 0x49, 0x2c, 0x49, 0x54,
	0x12, 0xe0, 0xe2, 0x83, 0x39, 0xa4, 0xb8, 0x20, 0x3f, 0xaf, 0x38, 0xd5, 0x28, 0x90, 0x8b, 0xc7,
	0x39, 0x3f, 0xaf, 0xa4, 0x28, 0x31, 0xb9, 0xc4, 0x39, 0x31, 0x27, 0x47, 0xc8, 0x91, 0x8b, 0x0d,
	0xa2, 0x42, 0x48, 0x12, 0xd9, 0x2a, 0x14, 0xe7, 0x4b, 0x49, 0x61, 0x93, 0x82, 0x18, 0xa8, 0xc4,
	0xe0, 0x24, 0x10, 0xc5, 0xa7, 0x0f, 0x95, 0x8f, 0x07, 0x07, 0x40, 0x12, 0x1b, 0x98, 0x32, 0x06,
	0x04, 0x00, 0x00, 0xff, 0xff, 0xcd, 0x39, 0xb1, 0xfe, 0x3e, 0x01, 0x00, 0x00,
}