package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the AaveV3DebtTokenAdaptorCallBuilder constructor
func TestNewAaveV3DebtTokenAdaptorCall(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3DebtTokenAdaptorV1CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))
}

// Test RepayDebt function
func TestRepayDebt(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3DebtTokenAdaptorV1CallBuilder(adaptor)

	asset := common.HexToAddress("0x0000000000000000000000000000000000000000")
	builder.RepayAaveDebt(asset, big.NewInt(100))

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveV3DebtTokenAdaptorV1_RepayAaveDebt_{}, builder.calls[0].Function)
}
