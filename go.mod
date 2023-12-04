module github.com/peggyjv/steward

go 1.15

require (
	github.com/cosmos/cosmos-sdk v0.45.10
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.22
	github.com/miguelmota/go-ethereum-hdwallet v0.1.1
	github.com/ory/dockertest/v3 v3.10.0
	github.com/peggyjv/gravity-bridge/module/v3 v3.0.0-20230818150036-143c0cb21c3b
	github.com/peggyjv/sommelier/v7 v7.0.0-20231130195331-471e3832c4a1
	github.com/spf13/viper v1.14.0
	github.com/stretchr/testify v1.8.2
	github.com/tendermint/tendermint v0.34.22
	google.golang.org/grpc v1.54.0
	google.golang.org/protobuf v1.30.0
)

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1

replace github.com/99designs/keyring => github.com/cosmos/keyring v1.1.7-0.20210622111912-ef00f8ac3d76

replace github.com/peggyjv/steward/steward_proto_go/steward_proto => ./steward_proto_go/steward_proto

replace github.com/confio/ics23/go => github.com/cosmos/cosmos-sdk/ics23/go v0.8.0
