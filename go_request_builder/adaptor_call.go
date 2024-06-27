package builder

import "github.com/peggyjv/steward/steward_proto_go/steward_proto"

type AdaptorCallBuilder struct {
    callData *steward_proto.AdaptorCall,
}

func NewAdaptorCallBuilder(adaptor common.Address) *AdaptorCallBuilder {
    return &AdaptorCallBuilder{
        callData: &steward_proto.AdaptorCall{
            Adaptor: adaptor.Hex(), 
            CallData: &steward_proto.AdaptorCall_CellarV2_5{},
        },
    }
}


