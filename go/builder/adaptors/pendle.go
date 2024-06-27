package adaptors

import (
	"github.com/ethereum/go-ethereum/common"
)

// Builder
type PendleAdaptorCallBuilder struct {
	adaptor common.Address
}

// Constructor
func NewPendleAdaptorCall(adaptor common.Address) *PendleAdaptorCallBuilder {
	return &PendleAdaptorCallBuilder{
		adaptor: adaptor,
	}
}
