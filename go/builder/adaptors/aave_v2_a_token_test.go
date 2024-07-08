package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the AaveV2ATokenAdaptorCallBuilder constructor
func TestNewAaveATokenAdaptorCall(t *testing.T) {
	// Create a new AaveV2ATokenAdaptorCallBuilder
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2ATokenAdaptorV2CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	spender := common.HexToAddress("0x11111111111111111111111111111111111111111")
	builder.RevokeApproval(asset, spender)
	builder.DepositToAave(asset, big.NewInt(100))
	builder.WithdrawFromAave(asset, big.NewInt(100))

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 3, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_RevokeApproval{}, builder.calls[0].Function)
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_DepositToAave_{}, builder.calls[1].Function)
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_WithdrawFromAave_{}, builder.calls[2].Function)
}

func TestEmptyBuilder(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2ATokenAdaptorV2CallBuilder(adaptor)

	assert.Equal(t, 0, len(builder.calls))
}

func TestRevokeApproval(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2ATokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	spender := common.HexToAddress("0x11111111111111111111111111111111111111111")
	builder.RevokeApproval(asset, spender)

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_RevokeApproval{}, builder.calls[0].Function)
}

func TestDepositToAave(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2ATokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.DepositToAave(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_DepositToAave_{}, builder.calls[0].Function)
}

func TestWithdrawFromAave(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2ATokenAdaptorV2CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.WithdrawFromAave(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveATokenAdaptorV2_WithdrawFromAave_{}, builder.calls[0].Function)
}
