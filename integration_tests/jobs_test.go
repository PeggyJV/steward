package integration_tests

import (
	"time"

	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/ethclient"
)

func (s *IntegrationTestSuite) TestUpdateStrategistPlatformCutJob() {
	s.Run("Update strategist platform cut", func() {
		s.checkCellarExists(v2_2Cellar)
		s.checkCellarExists(v2_5Cellar)

		// Make eth client and query cut value from cellars
		ethClient, err := ethclient.Dial("http://localhost:8545")
		s.Require().NoError(err)

		v2_2CellarInstance, err := NewCellarV22(v2_2Cellar, ethClient)
		s.Require().NoError(err)

		initialV2_2FeeData, err := v2_2CellarInstance.FeeData(&bind.CallOpts{})
		s.Require().NoError(err)
		s.Require().Equal(750000000000000000, int(initialV2_2FeeData.StrategistPlatformCut))

		v2_5CellarInstance, err := NewCellarV25(v2_5Cellar, ethClient)
		s.Require().NoError(err)

		initialV2_5FeeData, err := v2_5CellarInstance.FeeData(&bind.CallOpts{})
		s.Require().NoError(err)
		s.Require().Equal(750000000000000000, int(initialV2_5FeeData.StrategistPlatformCut))

		s.T().Logf("initial v2.2 fee cut: %v", initialV2_2FeeData.StrategistPlatformCut)
		s.T().Logf("initial v2.5 fee cut: %v", initialV2_5FeeData.StrategistPlatformCut)

		// Wait for them to get updated
		s.T().Logf("Waiting for v2.2 and v2.5 fee data to get updated...")
		updated := false
		s.Require().Eventually(func() bool {
			v2_2FeeData, err := v2_2CellarInstance.FeeData(&bind.CallOpts{})
			if err != nil {
				s.T().Logf("Error getting v2.2 fee data: %v", err)
				return false
			}
			v2_5FeeData, err := v2_5CellarInstance.FeeData(&bind.CallOpts{})
			if err != nil {
				s.T().Logf("Error getting v2.5 fee data: %v", err)
				return false
			}

			updated = v2_2FeeData.StrategistPlatformCut == 500000000000000000 && v2_5FeeData.StrategistPlatformCut == 500000000000000000
			if updated {
				s.T().Logf("v2.2 and v2.5 fee data updated")
			}

			return updated
		}, 180*time.Second, 5*time.Second, "v2.2 and v2.5 fee data not updated")

		s.T().Logf("Done!")
	})
}
