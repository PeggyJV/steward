package builder

import (
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

// ScheduleRequestBuilder is the top-level builder for sending requests to Steward. It is used to build a ScheduleRequest.
type ScheduleRequestBuilder struct {
	cellarID    string
	chainID     int
	callData    *steward_proto.CellarV2_5
	blockHeight int
	deadline    int
}

// NewScheduleRequestBuilder creates a new ScheduleRequestBuilder.
func NewScheduleRequestBuilder() *ScheduleRequestBuilder {
	return &ScheduleRequestBuilder{
		callData: &steward_proto.CellarV2_5{},
	}
}

// Build tries to build the ScheduleRequest from the builder. It will return an error if basic validation fails.
func (b *ScheduleRequestBuilder) Build() (*steward_proto.ScheduleRequest, error) {
	if err := b.Validate(); err != nil {
		return nil, err
	}

	return &steward_proto.ScheduleRequest{
		CellarId: b.cellarID,
		ChainId:  uint64(b.chainID),
		CallData: &steward_proto.ScheduleRequest_CellarV2_5{
			CellarV2_5: b.callData,
		},
		BlockHeight: uint64(b.blockHeight),
		Deadline:    uint64(b.deadline),
	}, nil
}

// Validate checks if the builder has all the required fields set with legal values.
func (b *ScheduleRequestBuilder) Validate() error {
	if b.chainID == 0 {
		return fmt.Errorf("chain ID cannot be zero")
	}
	if b.blockHeight == 0 || b.blockHeight < 0 {
		return fmt.Errorf("block height must be greater than zero")
	}
	if b.deadline < 0 {
		return fmt.Errorf("deadline must be zero or positive")
	}

	return validateCallData(b.callData)
}

// Sanity check for call data builder
func validateCallData(callData *steward_proto.CellarV2_5) error {
	if callData == nil {
		return fmt.Errorf("call data cannot be nil")
	}

	switch callData.GetCallType().(type) {
	case *steward_proto.CellarV2_5_FunctionCall_:
		if callData.GetFunctionCall() == nil {
			return fmt.Errorf("function call cannot be nil")
		}
	case *steward_proto.CellarV2_5_Multicall_:
		if callData.GetMulticall() == nil {
			return fmt.Errorf("multi call cannot be nil")
		}

		if len(callData.GetMulticall().FunctionCalls) == 0 {
			return fmt.Errorf("multi call cannot be empty")
		}
	}

	return nil
}

// WithCellarID sets the cellar ID for the request.
func (b *ScheduleRequestBuilder) WithCellarID(cellarID common.Address) *ScheduleRequestBuilder {
	b.cellarID = cellarID.Hex()
	return b
}

// WithChainID sets the chain ID for the request.
func (b *ScheduleRequestBuilder) WithChainID(chainID int) *ScheduleRequestBuilder {
	b.chainID = chainID
	return b
}

// WithBlockHeight sets the Sommelier block height for the request at which the chain will tally votes for the cellar call.
func (b *ScheduleRequestBuilder) WithBlockHeight(blockHeight int) *ScheduleRequestBuilder {
	b.blockHeight = blockHeight
	return b
}

// WithCallData sets the call data for the request.
func (b *ScheduleRequestBuilder) WithCallData(callData *steward_proto.CellarV2_5) *ScheduleRequestBuilder {
	b.callData = callData
	return b
}

// WithDeadline sets the deadline for the request. Only applies to non-Ethereum chains.
func (b *ScheduleRequestBuilder) WithDeadline(deadline int) *ScheduleRequestBuilder {
	b.deadline = deadline
	return b
}
