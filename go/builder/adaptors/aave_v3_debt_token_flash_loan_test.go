package adaptors

import (
	"math/big"
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test the AaveV3DebtTokenFlashLoanAdaptorCallBuilder constructor
func TestNewAaveV3DebtTokenFlashLoanAdaptorCall(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3DebtTokenFlashLoanAdaptorV1CallBuilder(adaptor)

	// Check the builder
	assert.Equal(t, adaptor.Hex(), builder.adaptor.Hex())
	assert.Equal(t, 0, len(builder.calls))
}

// Test RequestFlashLoan function
func TestRequestFlashLoan(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV3DebtTokenFlashLoanAdaptorV1CallBuilder(adaptor)

	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")

	// AaveV2 adaptor call builder
	call := NewAaveV2ATokenAdaptorV2CallBuilder(common.HexToAddress("0x1")).DepositToAave(common.HexToAddress("0x2"), big.NewInt(100)).Build()

	builder.FlashLoan([]common.Address{asset}, []*big.Int{big.NewInt(100)}, []*steward_proto.AdaptorCall{call})

	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveV3DebtTokenAdaptorV1FlashLoan_FlashLoan{}, builder.calls[0].FlashLoan)
}
