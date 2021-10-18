import "@nomiclabs/hardhat-waffle";

// This is a sample Hardhat task. To learn how to create your own go to
// https://hardhat.org/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, hre) => {
    const accounts = await hre.ethers.getSigners();for (const account of accounts) {
      console.log(account.address);
    }
  });// You need to export an object to set up your config
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
  