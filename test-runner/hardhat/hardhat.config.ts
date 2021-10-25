import "@nomiclabs/hardhat-waffle";
import { BigNumber } from "ethers";
import { task } from 'hardhat/config';

// Simple hardhat task for Cellars Rebalancer test

task("integration_test_setup", 'Sets up contracts for the integration test', async (taskArgs, hre) => {
  const accounts = await hre.ethers.getSigners();

  const ADDRESSES = {
    CELLAR_OWNER: '0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266',
    CELLAR: '0x08c0a0B8D2eDB1d040d4f2C00A1d2f9d9b9F2677',
    WETH9: '0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2',
    WETH9_OWNER: '0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266'
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

  console.log('retrieving weth contract');

  const wethSigner = await hre.ethers.getSigner(ADDRESSES.WETH9_OWNER)

  const Weth = hre.ethers.getContractAt(
    'WETH9',
    ADDRESSES.WETH9,
    wethSigner,
  );

  const weth = await Weth;

  const send: BigNumber = weth.deposit({value: 50000000000000000000n})

  await hre.run('node');

});


// You need to export an object to set up your config
// Go to https://hardhat.org/config/ to learn more

module.exports = {
  networks: {
    hardhat: {
      chainId: 1337,
      paths: {
        sources: "./contracts/weth9.sol",
     },
      forking: {
        url: "https://mainnet.infura.io/v3/d6f22be0f7fd447186086d2495779003",
        blockNumber: 13103326
      }
    },
  },
  solidity: {
    version: "0.7.6",
    settings: {
      optimizer: {
        enabled: true
      }
    }
  },
  // TODO: add forking configuration
  typechain: {
    outDir: "typechain",
    target: "es2020",
    lib: ["es2020"],
  },
  runOnCompile: true,
  gasReporter: {
    enabled: true
  },
};
