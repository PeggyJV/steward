module github.com/peggyjv/steward

go 1.15

require (
	github.com/cosmos/cosmos-sdk v0.44.5
	github.com/cosmos/go-bip39 v1.0.0
	github.com/ethereum/go-ethereum v1.10.11
	github.com/miguelmota/go-ethereum-hdwallet v0.1.1
	github.com/ory/dockertest/v3 v3.8.1
	github.com/peggyjv/gravity-bridge/module v0.3.9
	github.com/peggyjv/sommelier/v3 v3.1.2-0.20220224184033-89d9ddf9254a
	github.com/spf13/viper v1.8.1
	github.com/stretchr/testify v1.7.0
	github.com/tendermint/tendermint v0.34.14
	google.golang.org/grpc v1.42.0
	google.golang.org/protobuf v1.27.1
)

replace google.golang.org/grpc => google.golang.org/grpc v1.33.2

replace github.com/gogo/protobuf => github.com/regen-network/protobuf v1.3.3-alpha.regen.1
