// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v4.25.3
// source: steward.proto

package steward_proto

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// ContractCallServiceClient is the client API for ContractCallService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type ContractCallServiceClient interface {
	// Handles scheduled contract call submission
	Schedule(ctx context.Context, in *ScheduleRequest, opts ...grpc.CallOption) (*ScheduleResponse, error)
}

type contractCallServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewContractCallServiceClient(cc grpc.ClientConnInterface) ContractCallServiceClient {
	return &contractCallServiceClient{cc}
}

func (c *contractCallServiceClient) Schedule(ctx context.Context, in *ScheduleRequest, opts ...grpc.CallOption) (*ScheduleResponse, error) {
	out := new(ScheduleResponse)
	err := c.cc.Invoke(ctx, "/steward.v4.ContractCallService/Schedule", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// ContractCallServiceServer is the server API for ContractCallService service.
// All implementations must embed UnimplementedContractCallServiceServer
// for forward compatibility
type ContractCallServiceServer interface {
	// Handles scheduled contract call submission
	Schedule(context.Context, *ScheduleRequest) (*ScheduleResponse, error)
	mustEmbedUnimplementedContractCallServiceServer()
}

// UnimplementedContractCallServiceServer must be embedded to have forward compatible implementations.
type UnimplementedContractCallServiceServer struct {
}

func (UnimplementedContractCallServiceServer) Schedule(context.Context, *ScheduleRequest) (*ScheduleResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Schedule not implemented")
}
func (UnimplementedContractCallServiceServer) mustEmbedUnimplementedContractCallServiceServer() {}

// UnsafeContractCallServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to ContractCallServiceServer will
// result in compilation errors.
type UnsafeContractCallServiceServer interface {
	mustEmbedUnimplementedContractCallServiceServer()
}

func RegisterContractCallServiceServer(s grpc.ServiceRegistrar, srv ContractCallServiceServer) {
	s.RegisterService(&ContractCallService_ServiceDesc, srv)
}

func _ContractCallService_Schedule_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ScheduleRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(ContractCallServiceServer).Schedule(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v4.ContractCallService/Schedule",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(ContractCallServiceServer).Schedule(ctx, req.(*ScheduleRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// ContractCallService_ServiceDesc is the grpc.ServiceDesc for ContractCallService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var ContractCallService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v4.ContractCallService",
	HandlerType: (*ContractCallServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Schedule",
			Handler:    _ContractCallService_Schedule_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}

// EncodingServiceClient is the client API for EncodingService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type EncodingServiceClient interface {
	// Handles contract call encoding
	Encode(ctx context.Context, in *EncodeRequest, opts ...grpc.CallOption) (*EncodeResponse, error)
}

type encodingServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewEncodingServiceClient(cc grpc.ClientConnInterface) EncodingServiceClient {
	return &encodingServiceClient{cc}
}

func (c *encodingServiceClient) Encode(ctx context.Context, in *EncodeRequest, opts ...grpc.CallOption) (*EncodeResponse, error) {
	out := new(EncodeResponse)
	err := c.cc.Invoke(ctx, "/steward.v4.EncodingService/Encode", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// EncodingServiceServer is the server API for EncodingService service.
// All implementations must embed UnimplementedEncodingServiceServer
// for forward compatibility
type EncodingServiceServer interface {
	// Handles contract call encoding
	Encode(context.Context, *EncodeRequest) (*EncodeResponse, error)
	mustEmbedUnimplementedEncodingServiceServer()
}

// UnimplementedEncodingServiceServer must be embedded to have forward compatible implementations.
type UnimplementedEncodingServiceServer struct {
}

func (UnimplementedEncodingServiceServer) Encode(context.Context, *EncodeRequest) (*EncodeResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Encode not implemented")
}
func (UnimplementedEncodingServiceServer) mustEmbedUnimplementedEncodingServiceServer() {}

// UnsafeEncodingServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to EncodingServiceServer will
// result in compilation errors.
type UnsafeEncodingServiceServer interface {
	mustEmbedUnimplementedEncodingServiceServer()
}

func RegisterEncodingServiceServer(s grpc.ServiceRegistrar, srv EncodingServiceServer) {
	s.RegisterService(&EncodingService_ServiceDesc, srv)
}

func _EncodingService_Encode_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(EncodeRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(EncodingServiceServer).Encode(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v4.EncodingService/Encode",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(EncodingServiceServer).Encode(ctx, req.(*EncodeRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// EncodingService_ServiceDesc is the grpc.ServiceDesc for EncodingService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var EncodingService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v4.EncodingService",
	HandlerType: (*EncodingServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Encode",
			Handler:    _EncodingService_Encode_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}

// SimulateContractCallServiceClient is the client API for SimulateContractCallService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type SimulateContractCallServiceClient interface {
	// Handles simulated contract call submission
	Simulate(ctx context.Context, in *SimulateRequest, opts ...grpc.CallOption) (*SimulateResponse, error)
}

type simulateContractCallServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewSimulateContractCallServiceClient(cc grpc.ClientConnInterface) SimulateContractCallServiceClient {
	return &simulateContractCallServiceClient{cc}
}

func (c *simulateContractCallServiceClient) Simulate(ctx context.Context, in *SimulateRequest, opts ...grpc.CallOption) (*SimulateResponse, error) {
	out := new(SimulateResponse)
	err := c.cc.Invoke(ctx, "/steward.v4.SimulateContractCallService/Simulate", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// SimulateContractCallServiceServer is the server API for SimulateContractCallService service.
// All implementations must embed UnimplementedSimulateContractCallServiceServer
// for forward compatibility
type SimulateContractCallServiceServer interface {
	// Handles simulated contract call submission
	Simulate(context.Context, *SimulateRequest) (*SimulateResponse, error)
	mustEmbedUnimplementedSimulateContractCallServiceServer()
}

// UnimplementedSimulateContractCallServiceServer must be embedded to have forward compatible implementations.
type UnimplementedSimulateContractCallServiceServer struct {
}

func (UnimplementedSimulateContractCallServiceServer) Simulate(context.Context, *SimulateRequest) (*SimulateResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Simulate not implemented")
}
func (UnimplementedSimulateContractCallServiceServer) mustEmbedUnimplementedSimulateContractCallServiceServer() {
}

// UnsafeSimulateContractCallServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to SimulateContractCallServiceServer will
// result in compilation errors.
type UnsafeSimulateContractCallServiceServer interface {
	mustEmbedUnimplementedSimulateContractCallServiceServer()
}

func RegisterSimulateContractCallServiceServer(s grpc.ServiceRegistrar, srv SimulateContractCallServiceServer) {
	s.RegisterService(&SimulateContractCallService_ServiceDesc, srv)
}

func _SimulateContractCallService_Simulate_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SimulateRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(SimulateContractCallServiceServer).Simulate(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v4.SimulateContractCallService/Simulate",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(SimulateContractCallServiceServer).Simulate(ctx, req.(*SimulateRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// SimulateContractCallService_ServiceDesc is the grpc.ServiceDesc for SimulateContractCallService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var SimulateContractCallService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v4.SimulateContractCallService",
	HandlerType: (*SimulateContractCallServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Simulate",
			Handler:    _SimulateContractCallService_Simulate_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}

// StatusServiceClient is the client API for StatusService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type StatusServiceClient interface {
	Version(ctx context.Context, in *VersionRequest, opts ...grpc.CallOption) (*VersionResponse, error)
}

type statusServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewStatusServiceClient(cc grpc.ClientConnInterface) StatusServiceClient {
	return &statusServiceClient{cc}
}

func (c *statusServiceClient) Version(ctx context.Context, in *VersionRequest, opts ...grpc.CallOption) (*VersionResponse, error) {
	out := new(VersionResponse)
	err := c.cc.Invoke(ctx, "/steward.v4.StatusService/Version", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// StatusServiceServer is the server API for StatusService service.
// All implementations must embed UnimplementedStatusServiceServer
// for forward compatibility
type StatusServiceServer interface {
	Version(context.Context, *VersionRequest) (*VersionResponse, error)
	mustEmbedUnimplementedStatusServiceServer()
}

// UnimplementedStatusServiceServer must be embedded to have forward compatible implementations.
type UnimplementedStatusServiceServer struct {
}

func (UnimplementedStatusServiceServer) Version(context.Context, *VersionRequest) (*VersionResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Version not implemented")
}
func (UnimplementedStatusServiceServer) mustEmbedUnimplementedStatusServiceServer() {}

// UnsafeStatusServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to StatusServiceServer will
// result in compilation errors.
type UnsafeStatusServiceServer interface {
	mustEmbedUnimplementedStatusServiceServer()
}

func RegisterStatusServiceServer(s grpc.ServiceRegistrar, srv StatusServiceServer) {
	s.RegisterService(&StatusService_ServiceDesc, srv)
}

func _StatusService_Version_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(VersionRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StatusServiceServer).Version(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/steward.v4.StatusService/Version",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(StatusServiceServer).Version(ctx, req.(*VersionRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// StatusService_ServiceDesc is the grpc.ServiceDesc for StatusService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var StatusService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "steward.v4.StatusService",
	HandlerType: (*StatusServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Version",
			Handler:    _StatusService_Version_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "steward.proto",
}
