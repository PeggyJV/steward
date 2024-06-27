module github.com/peggyjv/steward/client_example

go 1.22.1

replace github.com/peggyjv/steward/steward_proto_go/steward_proto => ../steward_proto_go/steward_proto

require (
	github.com/peggyjv/steward/steward_proto_go/steward_proto v0.0.0-20230323171404-00b2e7c3ae9d
	google.golang.org/grpc v1.64.0
)

require (
	github.com/ethereum/go-ethereum v1.14.5 // indirect
	github.com/holiman/uint256 v1.2.4 // indirect
	golang.org/x/crypto v0.22.0 // indirect
	golang.org/x/net v0.24.0 // indirect
	golang.org/x/sys v0.20.0 // indirect
	golang.org/x/text v0.14.0 // indirect
	google.golang.org/genproto/googleapis/rpc v0.0.0-20240318140521-94a12d6c2237 // indirect
	google.golang.org/protobuf v1.33.0 // indirect
)
