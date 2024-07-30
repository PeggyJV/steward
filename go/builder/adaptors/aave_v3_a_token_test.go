package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the AaveV3ATokenAdaptorCallBuilder constructor
func TestNewAaveV3ATokenAdaptorCall(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3ATokenAdaptorV1CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))
}

// Test DepositToAave function
func TestDepositToAaveV3(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3ATokenAdaptorV1CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.DepositToAave(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveV3ATokenAdaptorV1_DepositToAave_{}, builder.calls[0].Function)
}

// Test WithdrawFromAave function
func TestWithdrawFromAaveV3(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3ATokenAdaptorV1CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.WithdrawFromAave(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveV3ATokenAdaptorV1_WithdrawFromAave_{}, builder.calls[0].Function)
}
