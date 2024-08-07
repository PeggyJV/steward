//
// Protos for function calls to the Aave Debt Token adaptor.

// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.33.0
// 	protoc        v4.25.3
// source: debt_token.proto

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

// Represents call data for the Aave Debt Token adaptor V1, used for borrowing and repaying debt on Aave.
type AaveDebtTokenAdaptorV1 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*AaveDebtTokenAdaptorV1_Swap
	//	*AaveDebtTokenAdaptorV1_OracleSwap
	//	*AaveDebtTokenAdaptorV1_RevokeApproval
	//	*AaveDebtTokenAdaptorV1_BorrowFromAave_
	//	*AaveDebtTokenAdaptorV1_RepayAaveDebt_
	//	*AaveDebtTokenAdaptorV1_SwapAndRepay_
	Function isAaveDebtTokenAdaptorV1_Function `protobuf_oneof:"function"`
}

func (x *AaveDebtTokenAdaptorV1) Reset() {
	*x = AaveDebtTokenAdaptorV1{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV1) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV1) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV1) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV1.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV1) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{0}
}

func (m *AaveDebtTokenAdaptorV1) GetFunction() isAaveDebtTokenAdaptorV1_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetSwap() *Swap {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_Swap); ok {
		return x.Swap
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetOracleSwap() *OracleSwap {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_OracleSwap); ok {
		return x.OracleSwap
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetBorrowFromAave() *AaveDebtTokenAdaptorV1_BorrowFromAave {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_BorrowFromAave_); ok {
		return x.BorrowFromAave
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetRepayAaveDebt() *AaveDebtTokenAdaptorV1_RepayAaveDebt {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_RepayAaveDebt_); ok {
		return x.RepayAaveDebt
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV1) GetSwapAndRepay() *AaveDebtTokenAdaptorV1_SwapAndRepay {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV1_SwapAndRepay_); ok {
		return x.SwapAndRepay
	}
	return nil
}

type isAaveDebtTokenAdaptorV1_Function interface {
	isAaveDebtTokenAdaptorV1_Function()
}

type AaveDebtTokenAdaptorV1_Swap struct {
	// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
	Swap *Swap `protobuf:"bytes,1,opt,name=swap,proto3,oneof"`
}

type AaveDebtTokenAdaptorV1_OracleSwap struct {
	// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
	OracleSwap *OracleSwap `protobuf:"bytes,2,opt,name=oracle_swap,json=oracleSwap,proto3,oneof"`
}

type AaveDebtTokenAdaptorV1_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,3,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type AaveDebtTokenAdaptorV1_BorrowFromAave_ struct {
	// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
	BorrowFromAave *AaveDebtTokenAdaptorV1_BorrowFromAave `protobuf:"bytes,4,opt,name=borrow_from_aave,json=borrowFromAave,proto3,oneof"`
}

type AaveDebtTokenAdaptorV1_RepayAaveDebt_ struct {
	// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
	RepayAaveDebt *AaveDebtTokenAdaptorV1_RepayAaveDebt `protobuf:"bytes,5,opt,name=repay_aave_debt,json=repayAaveDebt,proto3,oneof"`
}

type AaveDebtTokenAdaptorV1_SwapAndRepay_ struct {
	// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
	SwapAndRepay *AaveDebtTokenAdaptorV1_SwapAndRepay `protobuf:"bytes,6,opt,name=swap_and_repay,json=swapAndRepay,proto3,oneof"`
}

func (*AaveDebtTokenAdaptorV1_Swap) isAaveDebtTokenAdaptorV1_Function() {}

func (*AaveDebtTokenAdaptorV1_OracleSwap) isAaveDebtTokenAdaptorV1_Function() {}

func (*AaveDebtTokenAdaptorV1_RevokeApproval) isAaveDebtTokenAdaptorV1_Function() {}

func (*AaveDebtTokenAdaptorV1_BorrowFromAave_) isAaveDebtTokenAdaptorV1_Function() {}

func (*AaveDebtTokenAdaptorV1_RepayAaveDebt_) isAaveDebtTokenAdaptorV1_Function() {}

func (*AaveDebtTokenAdaptorV1_SwapAndRepay_) isAaveDebtTokenAdaptorV1_Function() {}

// Represents call data for the Aave Debt Token adaptor V2, used for borrowing and repaying debt on Aave.
type AaveDebtTokenAdaptorV2 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Function:
	//
	//	*AaveDebtTokenAdaptorV2_RevokeApproval
	//	*AaveDebtTokenAdaptorV2_BorrowFromAave_
	//	*AaveDebtTokenAdaptorV2_RepayAaveDebt_
	Function isAaveDebtTokenAdaptorV2_Function `protobuf_oneof:"function"`
}

func (x *AaveDebtTokenAdaptorV2) Reset() {
	*x = AaveDebtTokenAdaptorV2{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV2) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV2) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV2) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV2.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV2) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{1}
}

func (m *AaveDebtTokenAdaptorV2) GetFunction() isAaveDebtTokenAdaptorV2_Function {
	if m != nil {
		return m.Function
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV2) GetRevokeApproval() *RevokeApproval {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV2_RevokeApproval); ok {
		return x.RevokeApproval
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV2) GetBorrowFromAave() *AaveDebtTokenAdaptorV2_BorrowFromAave {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV2_BorrowFromAave_); ok {
		return x.BorrowFromAave
	}
	return nil
}

func (x *AaveDebtTokenAdaptorV2) GetRepayAaveDebt() *AaveDebtTokenAdaptorV2_RepayAaveDebt {
	if x, ok := x.GetFunction().(*AaveDebtTokenAdaptorV2_RepayAaveDebt_); ok {
		return x.RepayAaveDebt
	}
	return nil
}

type isAaveDebtTokenAdaptorV2_Function interface {
	isAaveDebtTokenAdaptorV2_Function()
}

type AaveDebtTokenAdaptorV2_RevokeApproval struct {
	// Represents function `revokeApproval(ERC20 asset, address spender)`
	RevokeApproval *RevokeApproval `protobuf:"bytes,1,opt,name=revoke_approval,json=revokeApproval,proto3,oneof"`
}

type AaveDebtTokenAdaptorV2_BorrowFromAave_ struct {
	// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
	BorrowFromAave *AaveDebtTokenAdaptorV2_BorrowFromAave `protobuf:"bytes,2,opt,name=borrow_from_aave,json=borrowFromAave,proto3,oneof"`
}

type AaveDebtTokenAdaptorV2_RepayAaveDebt_ struct {
	// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
	RepayAaveDebt *AaveDebtTokenAdaptorV2_RepayAaveDebt `protobuf:"bytes,3,opt,name=repay_aave_debt,json=repayAaveDebt,proto3,oneof"`
}

func (*AaveDebtTokenAdaptorV2_RevokeApproval) isAaveDebtTokenAdaptorV2_Function() {}

func (*AaveDebtTokenAdaptorV2_BorrowFromAave_) isAaveDebtTokenAdaptorV2_Function() {}

func (*AaveDebtTokenAdaptorV2_RepayAaveDebt_) isAaveDebtTokenAdaptorV2_Function() {}

type AaveDebtTokenAdaptorV1Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*AaveDebtTokenAdaptorV1 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *AaveDebtTokenAdaptorV1Calls) Reset() {
	*x = AaveDebtTokenAdaptorV1Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV1Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV1Calls) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV1Calls) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV1Calls.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV1Calls) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{2}
}

func (x *AaveDebtTokenAdaptorV1Calls) GetCalls() []*AaveDebtTokenAdaptorV1 {
	if x != nil {
		return x.Calls
	}
	return nil
}

type AaveDebtTokenAdaptorV2Calls struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Calls []*AaveDebtTokenAdaptorV2 `protobuf:"bytes,1,rep,name=calls,proto3" json:"calls,omitempty"`
}

func (x *AaveDebtTokenAdaptorV2Calls) Reset() {
	*x = AaveDebtTokenAdaptorV2Calls{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV2Calls) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV2Calls) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV2Calls) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV2Calls.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV2Calls) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{3}
}

func (x *AaveDebtTokenAdaptorV2Calls) GetCalls() []*AaveDebtTokenAdaptorV2 {
	if x != nil {
		return x.Calls
	}
	return nil
}

// Allows strategists to borrow assets from Aave.
//
// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
type AaveDebtTokenAdaptorV1_BorrowFromAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to borrow
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to borrow
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveDebtTokenAdaptorV1_BorrowFromAave) Reset() {
	*x = AaveDebtTokenAdaptorV1_BorrowFromAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[4]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV1_BorrowFromAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV1_BorrowFromAave) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV1_BorrowFromAave) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[4]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV1_BorrowFromAave.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV1_BorrowFromAave) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{0, 0}
}

func (x *AaveDebtTokenAdaptorV1_BorrowFromAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV1_BorrowFromAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

// Allows strategists to repay loan debt on Aave.
//
// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
type AaveDebtTokenAdaptorV1_RepayAaveDebt struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to repay
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to repay
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveDebtTokenAdaptorV1_RepayAaveDebt) Reset() {
	*x = AaveDebtTokenAdaptorV1_RepayAaveDebt{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[5]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV1_RepayAaveDebt) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV1_RepayAaveDebt) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV1_RepayAaveDebt) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[5]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV1_RepayAaveDebt.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV1_RepayAaveDebt) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{0, 1}
}

func (x *AaveDebtTokenAdaptorV1_RepayAaveDebt) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV1_RepayAaveDebt) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

// Allows strategists to swap assets and repay loans in one call.
//
// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
type AaveDebtTokenAdaptorV1_SwapAndRepay struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the token to swap from
	TokenIn string `protobuf:"bytes,1,opt,name=token_in,json=tokenIn,proto3" json:"token_in,omitempty"`
	// The address of the token to swap to and repay with
	TokenToRepay string `protobuf:"bytes,2,opt,name=token_to_repay,json=tokenToRepay,proto3" json:"token_to_repay,omitempty"`
	// The amount to swap
	AmountIn string `protobuf:"bytes,3,opt,name=amount_in,json=amountIn,proto3" json:"amount_in,omitempty"`
	// The exchange to make the swap on
	Exchange Exchange `protobuf:"varint,4,opt,name=exchange,proto3,enum=steward.v4.Exchange" json:"exchange,omitempty"`
	// The parameters for the swap
	Params *SwapParams `protobuf:"bytes,5,opt,name=params,proto3" json:"params,omitempty"`
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) Reset() {
	*x = AaveDebtTokenAdaptorV1_SwapAndRepay{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[6]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV1_SwapAndRepay) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[6]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV1_SwapAndRepay.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV1_SwapAndRepay) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{0, 2}
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) GetTokenIn() string {
	if x != nil {
		return x.TokenIn
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) GetTokenToRepay() string {
	if x != nil {
		return x.TokenToRepay
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) GetAmountIn() string {
	if x != nil {
		return x.AmountIn
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) GetExchange() Exchange {
	if x != nil {
		return x.Exchange
	}
	return Exchange_EXCHANGE_UNSPECIFIED
}

func (x *AaveDebtTokenAdaptorV1_SwapAndRepay) GetParams() *SwapParams {
	if x != nil {
		return x.Params
	}
	return nil
}

// Allows strategists to borrow assets from Aave.
//
// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
type AaveDebtTokenAdaptorV2_BorrowFromAave struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to borrow
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to borrow
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveDebtTokenAdaptorV2_BorrowFromAave) Reset() {
	*x = AaveDebtTokenAdaptorV2_BorrowFromAave{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[7]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV2_BorrowFromAave) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV2_BorrowFromAave) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV2_BorrowFromAave) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[7]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV2_BorrowFromAave.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV2_BorrowFromAave) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{1, 0}
}

func (x *AaveDebtTokenAdaptorV2_BorrowFromAave) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV2_BorrowFromAave) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

// Allows strategists to repay loan debt on Aave.
//
// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
type AaveDebtTokenAdaptorV2_RepayAaveDebt struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// The address of the ERC20 token to repay
	Token string `protobuf:"bytes,1,opt,name=token,proto3" json:"token,omitempty"`
	// The amount to repay
	Amount string `protobuf:"bytes,2,opt,name=amount,proto3" json:"amount,omitempty"`
}

func (x *AaveDebtTokenAdaptorV2_RepayAaveDebt) Reset() {
	*x = AaveDebtTokenAdaptorV2_RepayAaveDebt{}
	if protoimpl.UnsafeEnabled {
		mi := &file_debt_token_proto_msgTypes[8]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *AaveDebtTokenAdaptorV2_RepayAaveDebt) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*AaveDebtTokenAdaptorV2_RepayAaveDebt) ProtoMessage() {}

func (x *AaveDebtTokenAdaptorV2_RepayAaveDebt) ProtoReflect() protoreflect.Message {
	mi := &file_debt_token_proto_msgTypes[8]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use AaveDebtTokenAdaptorV2_RepayAaveDebt.ProtoReflect.Descriptor instead.
func (*AaveDebtTokenAdaptorV2_RepayAaveDebt) Descriptor() ([]byte, []int) {
	return file_debt_token_proto_rawDescGZIP(), []int{1, 1}
}

func (x *AaveDebtTokenAdaptorV2_RepayAaveDebt) GetToken() string {
	if x != nil {
		return x.Token
	}
	return ""
}

func (x *AaveDebtTokenAdaptorV2_RepayAaveDebt) GetAmount() string {
	if x != nil {
		return x.Amount
	}
	return ""
}

var File_debt_token_proto protoreflect.FileDescriptor

var file_debt_token_proto_rawDesc = []byte{
	0x0a, 0x10, 0x64, 0x65, 0x62, 0x74, 0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x0a, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x1a, 0x0a,
	0x62, 0x61, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d,
	0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb2, 0x06, 0x0a, 0x16, 0x41, 0x61, 0x76,
	0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f,
	0x72, 0x56, 0x31, 0x12, 0x26, 0x0a, 0x04, 0x73, 0x77, 0x61, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x10, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x53,
	0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x04, 0x73, 0x77, 0x61, 0x70, 0x12, 0x39, 0x0a, 0x0b, 0x6f,
	0x72, 0x61, 0x63, 0x6c, 0x65, 0x5f, 0x73, 0x77, 0x61, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x4f, 0x72,
	0x61, 0x63, 0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x48, 0x00, 0x52, 0x0a, 0x6f, 0x72, 0x61, 0x63,
	0x6c, 0x65, 0x53, 0x77, 0x61, 0x70, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65,
	0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52, 0x65, 0x76,
	0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52, 0x0e, 0x72,
	0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12, 0x5d, 0x0a,
	0x10, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x61, 0x76,
	0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x42, 0x6f, 0x72, 0x72,
	0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x0e, 0x62, 0x6f,
	0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x12, 0x5a, 0x0a, 0x0f,
	0x72, 0x65, 0x70, 0x61, 0x79, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x64, 0x65, 0x62, 0x74, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e,
	0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e,
	0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x2e, 0x52, 0x65, 0x70, 0x61, 0x79, 0x41,
	0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x72, 0x65, 0x70, 0x61, 0x79,
	0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x12, 0x57, 0x0a, 0x0e, 0x73, 0x77, 0x61, 0x70,
	0x5f, 0x61, 0x6e, 0x64, 0x5f, 0x72, 0x65, 0x70, 0x61, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x2f, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61,
	0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74,
	0x6f, 0x72, 0x56, 0x31, 0x2e, 0x53, 0x77, 0x61, 0x70, 0x41, 0x6e, 0x64, 0x52, 0x65, 0x70, 0x61,
	0x79, 0x48, 0x00, 0x52, 0x0c, 0x73, 0x77, 0x61, 0x70, 0x41, 0x6e, 0x64, 0x52, 0x65, 0x70, 0x61,
	0x79, 0x1a, 0x3e, 0x0a, 0x0e, 0x42, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41,
	0x61, 0x76, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f,
	0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e,
	0x74, 0x1a, 0x3d, 0x0a, 0x0d, 0x52, 0x65, 0x70, 0x61, 0x79, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65,
	0x62, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75,
	0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
	0x1a, 0xce, 0x01, 0x0a, 0x0c, 0x53, 0x77, 0x61, 0x70, 0x41, 0x6e, 0x64, 0x52, 0x65, 0x70, 0x61,
	0x79, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x69, 0x6e, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x07, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x49, 0x6e, 0x12, 0x24, 0x0a, 0x0e,
	0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x5f, 0x74, 0x6f, 0x5f, 0x72, 0x65, 0x70, 0x61, 0x79, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x54, 0x6f, 0x52, 0x65, 0x70,
	0x61, 0x79, 0x12, 0x1b, 0x0a, 0x09, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x69, 0x6e, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x49, 0x6e, 0x12,
	0x30, 0x0a, 0x08, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x0e, 0x32, 0x14, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x45,
	0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x52, 0x08, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67,
	0x65, 0x12, 0x2e, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x16, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x53,
	0x77, 0x61, 0x70, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d,
	0x73, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa5, 0x03,
	0x0a, 0x16, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x12, 0x45, 0x0a, 0x0f, 0x72, 0x65, 0x76, 0x6f,
	0x6b, 0x65, 0x5f, 0x61, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x1a, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x52,
	0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x48, 0x00, 0x52,
	0x0e, 0x72, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x41, 0x70, 0x70, 0x72, 0x6f, 0x76, 0x61, 0x6c, 0x12,
	0x5d, 0x0a, 0x10, 0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61,
	0x61, 0x76, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54,
	0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x2e, 0x42, 0x6f,
	0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x0e,
	0x62, 0x6f, 0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x12, 0x5a,
	0x0a, 0x0f, 0x72, 0x65, 0x70, 0x61, 0x79, 0x5f, 0x61, 0x61, 0x76, 0x65, 0x5f, 0x64, 0x65, 0x62,
	0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72,
	0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b,
	0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x2e, 0x52, 0x65, 0x70, 0x61,
	0x79, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x72, 0x65, 0x70,
	0x61, 0x79, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x1a, 0x3e, 0x0a, 0x0e, 0x42, 0x6f,
	0x72, 0x72, 0x6f, 0x77, 0x46, 0x72, 0x6f, 0x6d, 0x41, 0x61, 0x76, 0x65, 0x12, 0x14, 0x0a, 0x05,
	0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b,
	0x65, 0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x1a, 0x3d, 0x0a, 0x0d, 0x52, 0x65,
	0x70, 0x61, 0x79, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x74,
	0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x6f, 0x6b, 0x65,
	0x6e, 0x12, 0x16, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x42, 0x0a, 0x0a, 0x08, 0x66, 0x75, 0x6e,
	0x63, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x57, 0x0a, 0x1b, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62,
	0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x43,
	0x61, 0x6c, 0x6c, 0x73, 0x12, 0x38, 0x0a, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20,
	0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73, 0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34,
	0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64,
	0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x31, 0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x22, 0x57,
	0x0a, 0x1b, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65, 0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41,
	0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32, 0x43, 0x61, 0x6c, 0x6c, 0x73, 0x12, 0x38, 0x0a,
	0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x22, 0x2e, 0x73,
	0x74, 0x65, 0x77, 0x61, 0x72, 0x64, 0x2e, 0x76, 0x34, 0x2e, 0x41, 0x61, 0x76, 0x65, 0x44, 0x65,
	0x62, 0x74, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x41, 0x64, 0x61, 0x70, 0x74, 0x6f, 0x72, 0x56, 0x32,
	0x52, 0x05, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x42, 0x10, 0x5a, 0x0e, 0x2f, 0x73, 0x74, 0x65, 0x77,
	0x61, 0x72, 0x64, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_debt_token_proto_rawDescOnce sync.Once
	file_debt_token_proto_rawDescData = file_debt_token_proto_rawDesc
)

func file_debt_token_proto_rawDescGZIP() []byte {
	file_debt_token_proto_rawDescOnce.Do(func() {
		file_debt_token_proto_rawDescData = protoimpl.X.CompressGZIP(file_debt_token_proto_rawDescData)
	})
	return file_debt_token_proto_rawDescData
}

var file_debt_token_proto_msgTypes = make([]protoimpl.MessageInfo, 9)
var file_debt_token_proto_goTypes = []interface{}{
	(*AaveDebtTokenAdaptorV1)(nil),                // 0: steward.v4.AaveDebtTokenAdaptorV1
	(*AaveDebtTokenAdaptorV2)(nil),                // 1: steward.v4.AaveDebtTokenAdaptorV2
	(*AaveDebtTokenAdaptorV1Calls)(nil),           // 2: steward.v4.AaveDebtTokenAdaptorV1Calls
	(*AaveDebtTokenAdaptorV2Calls)(nil),           // 3: steward.v4.AaveDebtTokenAdaptorV2Calls
	(*AaveDebtTokenAdaptorV1_BorrowFromAave)(nil), // 4: steward.v4.AaveDebtTokenAdaptorV1.BorrowFromAave
	(*AaveDebtTokenAdaptorV1_RepayAaveDebt)(nil),  // 5: steward.v4.AaveDebtTokenAdaptorV1.RepayAaveDebt
	(*AaveDebtTokenAdaptorV1_SwapAndRepay)(nil),   // 6: steward.v4.AaveDebtTokenAdaptorV1.SwapAndRepay
	(*AaveDebtTokenAdaptorV2_BorrowFromAave)(nil), // 7: steward.v4.AaveDebtTokenAdaptorV2.BorrowFromAave
	(*AaveDebtTokenAdaptorV2_RepayAaveDebt)(nil),  // 8: steward.v4.AaveDebtTokenAdaptorV2.RepayAaveDebt
	(*Swap)(nil),           // 9: steward.v4.Swap
	(*OracleSwap)(nil),     // 10: steward.v4.OracleSwap
	(*RevokeApproval)(nil), // 11: steward.v4.RevokeApproval
	(Exchange)(0),          // 12: steward.v4.Exchange
	(*SwapParams)(nil),     // 13: steward.v4.SwapParams
}
var file_debt_token_proto_depIdxs = []int32{
	9,  // 0: steward.v4.AaveDebtTokenAdaptorV1.swap:type_name -> steward.v4.Swap
	10, // 1: steward.v4.AaveDebtTokenAdaptorV1.oracle_swap:type_name -> steward.v4.OracleSwap
	11, // 2: steward.v4.AaveDebtTokenAdaptorV1.revoke_approval:type_name -> steward.v4.RevokeApproval
	4,  // 3: steward.v4.AaveDebtTokenAdaptorV1.borrow_from_aave:type_name -> steward.v4.AaveDebtTokenAdaptorV1.BorrowFromAave
	5,  // 4: steward.v4.AaveDebtTokenAdaptorV1.repay_aave_debt:type_name -> steward.v4.AaveDebtTokenAdaptorV1.RepayAaveDebt
	6,  // 5: steward.v4.AaveDebtTokenAdaptorV1.swap_and_repay:type_name -> steward.v4.AaveDebtTokenAdaptorV1.SwapAndRepay
	11, // 6: steward.v4.AaveDebtTokenAdaptorV2.revoke_approval:type_name -> steward.v4.RevokeApproval
	7,  // 7: steward.v4.AaveDebtTokenAdaptorV2.borrow_from_aave:type_name -> steward.v4.AaveDebtTokenAdaptorV2.BorrowFromAave
	8,  // 8: steward.v4.AaveDebtTokenAdaptorV2.repay_aave_debt:type_name -> steward.v4.AaveDebtTokenAdaptorV2.RepayAaveDebt
	0,  // 9: steward.v4.AaveDebtTokenAdaptorV1Calls.calls:type_name -> steward.v4.AaveDebtTokenAdaptorV1
	1,  // 10: steward.v4.AaveDebtTokenAdaptorV2Calls.calls:type_name -> steward.v4.AaveDebtTokenAdaptorV2
	12, // 11: steward.v4.AaveDebtTokenAdaptorV1.SwapAndRepay.exchange:type_name -> steward.v4.Exchange
	13, // 12: steward.v4.AaveDebtTokenAdaptorV1.SwapAndRepay.params:type_name -> steward.v4.SwapParams
	13, // [13:13] is the sub-list for method output_type
	13, // [13:13] is the sub-list for method input_type
	13, // [13:13] is the sub-list for extension type_name
	13, // [13:13] is the sub-list for extension extendee
	0,  // [0:13] is the sub-list for field type_name
}

func init() { file_debt_token_proto_init() }
func file_debt_token_proto_init() {
	if File_debt_token_proto != nil {
		return
	}
	file_base_proto_init()
	file_common_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_debt_token_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV1); i {
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
		file_debt_token_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV2); i {
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
		file_debt_token_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV1Calls); i {
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
		file_debt_token_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV2Calls); i {
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
		file_debt_token_proto_msgTypes[4].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV1_BorrowFromAave); i {
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
		file_debt_token_proto_msgTypes[5].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV1_RepayAaveDebt); i {
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
		file_debt_token_proto_msgTypes[6].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV1_SwapAndRepay); i {
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
		file_debt_token_proto_msgTypes[7].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV2_BorrowFromAave); i {
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
		file_debt_token_proto_msgTypes[8].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*AaveDebtTokenAdaptorV2_RepayAaveDebt); i {
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
	file_debt_token_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*AaveDebtTokenAdaptorV1_Swap)(nil),
		(*AaveDebtTokenAdaptorV1_OracleSwap)(nil),
		(*AaveDebtTokenAdaptorV1_RevokeApproval)(nil),
		(*AaveDebtTokenAdaptorV1_BorrowFromAave_)(nil),
		(*AaveDebtTokenAdaptorV1_RepayAaveDebt_)(nil),
		(*AaveDebtTokenAdaptorV1_SwapAndRepay_)(nil),
	}
	file_debt_token_proto_msgTypes[1].OneofWrappers = []interface{}{
		(*AaveDebtTokenAdaptorV2_RevokeApproval)(nil),
		(*AaveDebtTokenAdaptorV2_BorrowFromAave_)(nil),
		(*AaveDebtTokenAdaptorV2_RepayAaveDebt_)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_debt_token_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   9,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_debt_token_proto_goTypes,
		DependencyIndexes: file_debt_token_proto_depIdxs,
		MessageInfos:      file_debt_token_proto_msgTypes,
	}.Build()
	File_debt_token_proto = out.File
	file_debt_token_proto_rawDesc = nil
	file_debt_token_proto_goTypes = nil
	file_debt_token_proto_depIdxs = nil
}
