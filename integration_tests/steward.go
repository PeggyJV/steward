package integration_tests

import (
	"fmt"

	"github.com/cosmos/cosmos-sdk/crypto/keyring"
)

type steward struct {
	index    int
	mnemonic string
	keyInfo  keyring.Info
	keyring  *keyring.Keyring
}

func (o *steward) instanceName() string {
	return fmt.Sprintf("orchestrator%d", o.index)
}
