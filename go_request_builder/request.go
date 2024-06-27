package builder

import (
	"fmt"

	"github.com/ethereum/go-ethereum/common"
	"github.com/peggyjv/steward/steward_proto_go/steward_proto"
)

type ScheduleRequestBuilder struct {
    cellarID string
    chainID int
    callData *steward_proto.CellarV2_5
    blockHeight int
    deadline int
}

func NewScheduleRequestBuilder() *ScheduleRequestBuilder {
    return &ScheduleRequestBuilder{}
}

func (b *ScheduleRequestBuilder) Build() (*steward_proto.ScheduleRequest, error) {
    if err := b.Validate(); err != nil {
        return nil, err
    }

    return &steward_proto.ScheduleRequest {
        CellarId: b.cellarID,
        ChainId: uint64(b.chainID),
        CallData: &steward_proto.ScheduleRequest_CellarV2_5 { 
            CellarV2_5: b.callData,
        },
        BlockHeight: uint64(b.blockHeight),
        Deadline: uint64(b.deadline),
    }, nil
}

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

    return nil 
}

func (b *ScheduleRequestBuilder) WithCellarID(cellarID common.Address) *ScheduleRequestBuilder {
    b.cellarID = cellarID.Hex()
    return b
}

func (b *ScheduleRequestBuilder) WithChainID(chainID int) *ScheduleRequestBuilder {
    b.chainID = chainID
    return b
}

func (b *ScheduleRequestBuilder) WithBlockHeight(blockHeight int) *ScheduleRequestBuilder {
    b.blockHeight = blockHeight
    return b
}

func (b *ScheduleRequestBuilder) WithCallData(callData *steward_proto.CellarV2_5) *ScheduleRequestBuilder {
    b.callData = callData
    return b
}

func (b *ScheduleRequestBuilder) WithDeadline(deadline int) *ScheduleRequestBuilder {
    b.deadline = deadline
    return b
}


