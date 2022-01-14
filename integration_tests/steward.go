package integration_tests

import (
	"fmt"
)

type steward struct {
	chain        *chain
	index        int
	keystorePath string
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
