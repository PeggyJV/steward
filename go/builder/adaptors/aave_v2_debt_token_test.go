package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the AaveV2DebtTokenAdaptorCallBuilder constructor
func TestNewAaveDebtTokenAdaptorCall(t *testing.T) {
	// Create a new AaveV2DebtTokenAdaptorCallBuilder
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	spender := common.HexToAddress("0x11111111111111111111111111111111111111111")
	builder.RevokeApproval(asset, spender)
	builder.BorrowFromAave(asset, big.NewInt(100))
	builder.RepayAaveDebt(asset, big.NewInt(100))

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 3, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_RevokeApproval{}, builder.calls[0].Function)
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_BorrowFromAave_{}, builder.calls[1].Function)
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_RepayAaveDebt_{}, builder.calls[2].Function)

	result := builder.Build()
	assert.Equal(t, adaptor.Hex(), result.Adaptor)
	assert.IsType(t, &steward_proto.AdaptorCall_AaveDebtTokenV2Calls{}, result.CallData)
}

func TestEmptyAaveV2DebtTokenAdaptorBuilder(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor)

	assert.Equal(t, 0, len(builder.calls))

	result := builder.Build()
	assert.Equal(t, adaptor.Hex(), result.Adaptor)
	assert.IsType(t, &steward_proto.AdaptorCall_AaveDebtTokenV2Calls{}, result.CallData)
}

func TestAaveV2DebtTokenAdaptorRevokeApproval(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	spender := common.HexToAddress("0x11111111111111111111111111111111111111111")
	builder.RevokeApproval(asset, spender)

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_RevokeApproval{}, builder.calls[0].Function)
}

func TestBorrowFromAave(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.BorrowFromAave(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_BorrowFromAave_{}, builder.calls[0].Function)
}

func TestRepayAaveDebt(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2DebtTokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.RepayAaveDebt(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveDebtTokenAdaptorV2_RepayAaveDebt_{}, builder.calls[0].Function)
}
