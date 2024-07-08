package adaptors

import (
	"testing"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
	"github.com/stretchr/testify/assert"
)

// Test enabling an asset as collateral
func TestEnableAssetAsCollateral(t *testing.T) {
	adaptor := common.HexToAddress("0x1234567890123456789012345678901234567890")
	builder := NewAaveV2EnableAssetAsCollateralAdaptorV1CallBuilder(adaptor)

	// Test enabling
	asset := common.HexToAddress("0x00000000000000000000000000000000000000000")
	builder.SetUserUseReserveAsCollateral(asset, true)

	// Check the builder
	assert.Equal(t, 1, len(builder.calls))
	assert.IsType(t, &steward_proto.AaveV2EnableAssetAsCollateralAdaptorV1_SetUserUseReserveAsCollateral_{}, builder.calls[0].Function)
}
