package integration_tests

import (
	"fmt"
)

const STEWARD_PORT = 5734

type steward struct {
	chain    *chain
	index    int
	mnemonic string
}

func (s *steward) configBind() string {
	return fmt.Sprintf("%s:%s", s.configDir(), "/home/steward")
}

func (s *steward) configDir() string {
	return fmt.Sprintf("%s/%s", s.chain.configDir(), s.instanceName())
}

func (s *steward) instanceName() string {
	return fmt.Sprintf("steward%d", s.index)
}
