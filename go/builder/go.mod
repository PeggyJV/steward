module github.com/peggyjv/steward/go/builder

go 1.22

require (
	github.com/ethereum/go-ethereum v1.14.5
	github.com/peggyjv/steward/steward_proto_go/steward_proto v0.1.0
	github.com/stretchr/testify v1.8.4
)

require (
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/holiman/uint256 v1.2.4 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	golang.org/x/crypto v0.22.0 // indirect
	golang.org/x/net v0.24.0 // indirect
	golang.org/x/sys v0.20.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20240318140521-94a12d6c2237 // indirect
	google.golang.org/grpc v1.64.0 // indirect
	google.golang.org/protobuf v1.34.2 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

replace github.com/peggyjv/steward/steward_proto_go/steward_proto => ../../steward_proto_go/steward_proto
