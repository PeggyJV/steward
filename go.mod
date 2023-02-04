module github.com/peggyjv/steward

go 1.15

require (
	github.com/cosmos/cosmos-sdk v0.44.5
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.16
	github.com/miguelmota/go-ethereum-hdwallet v0.1.1
	github.com/ory/dockertest/v3 v3.8.1
	github.com/peggyjv/gravity-bridge/module/v2 v2.0.2
	github.com/peggyjv/sommelier/v4 v4.0.2-0.20230124233240-d214e8c0681b
	github.com/peggyjv/steward/steward_proto_go/steward_proto v0.0.0-00010101000000-000000000000
	github.com/spf13/viper v1.10.1
	github.com/stretchr/testify v1.8.0
	github.com/tendermint/tendermint v0.34.14
	google.golang.org/grpc v1.45.0
)

replace google.golang.org/grpc => google.golang.org/grpc v1.33.2

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1

replace github.com/peggyjv/steward/steward_proto_go/steward_proto => ./steward_proto_go/steward_proto

replace github.com/confio/ics23/go => github.com/cosmos/cosmos-sdk/ics23/go v0.8.0
