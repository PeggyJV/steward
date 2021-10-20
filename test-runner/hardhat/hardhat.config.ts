import "@nomiclabs/hardhat-waffle";
import { task } from 'hardhat/config';

// Simple hardhat task for Cellars Rebalancer test

task("integration_test_setup", 'Sets up contracts for the integration test', async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  const ADDRESSES = {
    CELLAR_OWNER: '0xB6C951cf962977f123bF37de42945f7ca1cd2A52',
    CELLAR: '0x6ea5992aB4A78D5720bD12A089D13c073d04B55d',
  };

  console.log('retrieving cellar contract');
  const cellarSigner = await hre.ethers.getSigner(ADDRESSES.CELLAR_OWNER)

  // get Cellars contract from forked mainnet
  const Cellar = hre.ethers.getContractAt(
    'CellarPoolShare',
    ADDRESSES.CELLAR,
    cellarSigner,
  );
  const cellar = await Cellar;
  console.log(cellar)

});


// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

module.exports = {
  networks: {
    hardhat: {
      forking: {
        url: "https://mainnet.infura.io/v3/d6f22be0f7fd447186086d2495779003",
        blockNumber: 13103326
      }
    },
  },
  solidity: {
    version: "0.7.3",
    settings: {
      optimizer: {
        enabled: true
      }
    }
  },
  // TODO: add forking configuration
  typechain: {
    outDir: "typechain",
    target: "ethers-v5",
    runOnCompile: true
  },
  gasReporter: {
    enabled: true
  },
};
