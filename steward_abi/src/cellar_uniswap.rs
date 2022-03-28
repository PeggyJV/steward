pub use uniswapv3cellar_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv3cellar_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "UniswapV3Cellar was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV3CELLAR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"CellarPoolShare\",\n  \"sourceName\": \"contracts/CellarPoolShare.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"name_\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"symbol_\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_token0\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_token1\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint24\",\n          \"name\": \"_feeLevel\",\n          \"type\": \"uint24\"\n        },\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint184\",\n              \"name\": \"tokenId\",\n              \"type\": \"uint184\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickUpper\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickLower\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"uint24\",\n              \"name\": \"weight\",\n              \"type\": \"uint24\"\n            }\n          ],\n          \"internalType\": \"struct ICellarPoolShare.CellarTickInfo[]\",\n          \"name\": \"_cellarTickInfo\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token0\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token1\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint128\",\n          \"name\": \"liquidity\",\n          \"type\": \"uint128\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount1\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"AddedLiquidity\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Approval\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"fees0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"fees1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"managementFee0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"managementFee1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"performanceFee0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"performanceFee1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount1\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Rebalance\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"fees0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"fees1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"managementFee0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"managementFee1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"performanceFee0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"performanceFee1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount1\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Reinvest\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token0\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token1\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint128\",\n          \"name\": \"liquidity\",\n          \"type\": \"uint128\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount0\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount1\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"RemovedLiquidity\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"adjuster\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"value\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"SetAdjuster\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newFee\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"SetManagementFee\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newFee\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"SetPerformanceFee\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"validator\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"value\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"SetValidator\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"TransferOwnership\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount0Desired\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount1Desired\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount0Min\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount1Min\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"recipient\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"deadline\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct ICellarPoolShare.CellarAddParams\",\n          \"name\": \"cellarParams\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"addLiquidityForUniV3\",\n      \"outputs\": [],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"adjuster\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner_\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"allowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"approve\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"account\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"cellarTickInfo\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint184\",\n          \"name\": \"tokenId\",\n          \"type\": \"uint184\"\n        },\n        {\n          \"internalType\": \"int24\",\n          \"name\": \"tickUpper\",\n          \"type\": \"int24\"\n        },\n        {\n          \"internalType\": \"int24\",\n          \"name\": \"tickLower\",\n          \"type\": \"int24\"\n        },\n        {\n          \"internalType\": \"uint24\",\n          \"name\": \"weight\",\n          \"type\": \"uint24\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"decimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"pure\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"feeLevel\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint24\",\n          \"name\": \"\",\n          \"type\": \"uint24\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCellarTickInfo\",\n      \"outputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint184\",\n              \"name\": \"tokenId\",\n              \"type\": \"uint184\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickUpper\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickLower\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"uint24\",\n              \"name\": \"weight\",\n              \"type\": \"uint24\"\n            }\n          ],\n          \"internalType\": \"struct ICellarPoolShare.CellarTickInfo[]\",\n          \"name\": \"\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"lastLockedBlock\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"managementFee\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"performanceFee\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint184\",\n              \"name\": \"tokenId\",\n              \"type\": \"uint184\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickUpper\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"int24\",\n              \"name\": \"tickLower\",\n              \"type\": \"int24\"\n            },\n            {\n              \"internalType\": \"uint24\",\n              \"name\": \"weight\",\n              \"type\": \"uint24\"\n            }\n          ],\n          \"internalType\": \"struct ICellarPoolShare.CellarTickInfo[]\",\n          \"name\": \"_cellarTickInfo\",\n          \"type\": \"tuple[]\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"currentPriceX96\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"rebalance\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"currentPriceX96\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"reinvest\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"tokenAmount\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount0Min\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount1Min\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"recipient\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"deadline\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"internalType\": \"struct ICellarPoolShare.CellarRemoveParams\",\n          \"name\": \"cellarParams\",\n          \"type\": \"tuple\"\n        }\n      ],\n      \"name\": \"removeLiquidityFromUniV3\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_adjuster\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"value\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"setAdjuster\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newFee\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setManagementFee\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newFee\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setPerformanceFee\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_validator\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"value\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"setValidator\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"symbol\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"token0\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"token1\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalSupply\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"recipient\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"sender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"recipient\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transferFrom\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"transferOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"validator\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"stateMutability\": \"payable\",\n      \"type\": \"receive\"\n    }\n  ],\n  \"bytecode\": \"0x60e06040526107d0600b5560c8600c553480156200001c57600080fd5b5060405162006518380380620065188339810160408190526200003f9162000504565b8551620000549060079060208901906200038b565b5084516200006a9060089060208801906200038b565b50826001600160a01b0316846001600160a01b031610620000a85760405162461bcd60e51b81526004016200009f9062000704565b60405180910390fd5b6001600160601b0319606085811b821660805284901b1660a0526001600160e81b031960e883901b1660c05260005b815181101562000336576000828281518110620000f057fe5b60200260200101516060015162ffffff1611620001215760405162461bcd60e51b81526004016200009f90620006ce565b8181815181106200012e57fe5b6020026020010151600001516001600160b81b0316600014620001655760405162461bcd60e51b81526004016200009f90620006e9565b8181815181106200017257fe5b60200260200101516040015160020b8282815181106200018e57fe5b60200260200101516020015160020b13620001bd5760405162461bcd60e51b81526004016200009f90620006b3565b80156200022057816001820381518110620001d457fe5b60200260200101516040015160020b828281518110620001f057fe5b60200260200101516020015160020b1315620002205760405162461bcd60e51b81526004016200009f90620006b3565b6009604051806080016040528060006001600160b81b031681526020018484815181106200024a57fe5b60200260200101516020015160020b81526020018484815181106200026b57fe5b60200260200101516040015160020b81526020018484815181106200028c57fe5b60209081029190910181015160609081015162ffffff90811690935284546001808201875560009687529583902085519101805493860151604087015196909301516001600160b81b03199094166001600160b81b039092169190911762ffffff60b81b1916600160b81b600293840b8616021762ffffff60d01b1916600160d01b9590920b841694909402176001600160e81b0316600160e81b919092160217905501620000d7565b5050600680546001600160a01b031916339081179091556000908152600360209081526040808320805460ff199081166001908117909255600490935292208054909116909117905550620007439350505050565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282620003c357600085556200040e565b82601f10620003de57805160ff19168380011785556200040e565b828001600101855582156200040e579182015b828111156200040e578251825591602001919060010190620003f1565b506200041c92915062000420565b5090565b5b808211156200041c576000815560010162000421565b80516001600160a01b03811681146200044f57600080fd5b919050565b8051600281900b81146200044f57600080fd5b600082601f83011262000478578081fd5b81516001600160401b038111156200048c57fe5b6020620004a2601f8301601f191682016200071f565b8281528582848701011115620004b6578384fd5b835b83811015620004d5578581018301518282018401528201620004b8565b83811115620004e657848385840101525b5095945050505050565b805162ffffff811681146200044f57600080fd5b60008060008060008060c087890312156200051d578182fd5b86516001600160401b038082111562000534578384fd5b620005428a838b0162000467565b9750602089015191508082111562000558578384fd5b620005668a838b0162000467565b96506200057660408a0162000437565b95506200058660608a0162000437565b94506200059660808a01620004f0565b935060a0890151915080821115620005ac578283fd5b818901915089601f830112620005c0578283fd5b815181811115620005cd57fe5b620005dd6020808302016200071f565b80828252602082019150602085018d602060808602880101111562000600578687fd5b8695505b83861015620006a0576080818f0312156200061d578687fd5b6040516080810181811087821117156200063357fe5b60405281516001600160b81b03811681146200064d578889fd5b81526200065d6020830162000454565b6020820152620006706040830162000454565b60408201526200068360608301620004f0565b606082015283526001959095019460209092019160800162000604565b5080955050505050509295509295509295565b6020808252600190820152604360f81b604082015260600190565b6020808252600190820152604160f81b604082015260600190565b6020808252600190820152602160f91b604082015260600190565b6020808252600190820152603960f81b604082015260600190565b6040518181016001600160401b03811182821017156200073b57fe5b604052919050565b60805160601c60a05160601c60c05160e81c615c9662000882600039806113c552806114e9528061224a5280612b0752806135cb52806138335280613d6a528061435d5250806108ed528061091b5280610a365280610ca75280610cda5280610e9852806110ba528061119a52806111c352806114c752806118a25280611a84528061222852806129785280612ad85280612fb1528061336652806135a4528061379752806137e752806139135280613a055280613d48528061433b52508061062952806108125280610a0d5280610a605280610b695280610e2c5280610e625280610fab528061108b52806111ed52806114a55280611862528061220652806129275280612aa95280612f6852806132db528061352f528061357f52806136ab528061380c52806139665280613d2652806143195250615c966000f3fe6080604052600436106101c65760003560e01c806383b4918b116100f7578063a6f7f5d611610095578063dd62ed3e11610064578063dd62ed3e146104fa578063e1b5c3121461051a578063f2fde38b1461053a578063fe56e2321461055a576101e7565b8063a6f7f5d614610490578063a9059cbb146104a5578063d21220a7146104c5578063db5d820e146104da576101e7565b80638da5cb5b116100d15780638da5cb5b1461042457806395d89b411461043957806396d01a7d1461044e5780639f3e8b3414610470576101e7565b806383b4918b146103cf57806387788782146103ef5780638c231af914610404576101e7565b8063276cd920116101645780634623c91d1161013e5780634623c91d1461034d57806370897b231461036d57806370a082311461038d5780637cf134cb146103ad576101e7565b8063276cd920146102f8578063313ce5671461031857806333bc230a1461033a576101e7565b806315723866116101a0578063157238661461026657806318160ddd14610296578063223b3b7a146102b857806323b872dd146102d8576101e7565b806306fdde03146101ec578063095ea7b3146102175780630dfe168114610244576101e7565b366101e75733600080516020615c41833981519152146101e557600080fd5b005b600080fd5b3480156101f857600080fd5b5061020161057a565b60405161020e91906155ff565b60405180910390f35b34801561022357600080fd5b50610237610232366004614fb4565b610610565b60405161020e91906155be565b34801561025057600080fd5b50610259610627565b60405161020e9190615497565b34801561027257600080fd5b506102866102813660046152f1565b61064b565b60405161020e9493929190615b57565b3480156102a257600080fd5b506102ab61069a565b60405161020e9190615b96565b3480156102c457600080fd5b506102376102d3366004614ed7565b6106a0565b3480156102e457600080fd5b506102376102f3366004614f47565b6106b5565b34801561030457600080fd5b506101e56103133660046151a5565b610707565b34801561032457600080fd5b5061032d610ad7565b60405161020e9190615b9f565b6101e5610348366004615113565b610adc565b34801561035957600080fd5b506101e5610368366004614f87565b611267565b34801561037957600080fd5b506101e56103883660046152f1565b611319565b34801561039957600080fd5b506102ab6103a8366004614ed7565b6113a4565b3480156103b957600080fd5b506103c26113c3565b60405161020e9190615b86565b3480156103db57600080fd5b506101e56103ea3660046152f1565b6113e7565b3480156103fb57600080fd5b506102ab611931565b34801561041057600080fd5b5061023761041f366004614ed7565b611937565b34801561043057600080fd5b5061025961194c565b34801561044557600080fd5b5061020161195b565b34801561045a57600080fd5b506104636119bc565b60405161020e9190615545565b34801561047c57600080fd5b506102ab61048b366004614ed7565b611a5d565b34801561049c57600080fd5b506102ab611a6f565b3480156104b157600080fd5b506102376104c0366004614fb4565b611a75565b3480156104d157600080fd5b50610259611a82565b3480156104e657600080fd5b506101e56104f5366004614f87565b611aa6565b34801561050657600080fd5b506102ab610515366004614f0f565b611b4c565b34801561052657600080fd5b506101e5610535366004614fdf565b611b77565b34801561054657600080fd5b506101e5610555366004614ed7565b611e3a565b34801561056657600080fd5b506101e56105753660046152f1565b611ed5565b60078054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156106065780601f106105db57610100808354040283529160200191610606565b820191906000526020600020905b8154815290600101906020018083116105e957829003601f168201915b5050505050905090565b600061061d338484611f55565b5060015b92915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6009818154811061065b57600080fd5b6000918252602090912001546001600160b81b0381169150600160b81b8104600290810b91600160d01b810490910b90600160e81b900462ffffff1684565b60055490565b60036020526000908152604090205460ff1681565b60006106c2848484612009565b6001600160a01b0384166000908152600260209081526040808320338085529252909120546106fc9186916106f790866120fa565b611f55565b5060015b9392505050565b600654600160a01b900460ff161561073a5760405162461bcd60e51b815260040161073190615725565b60405180910390fd5b6006805460ff60a01b1916600160a01b1790553360008181526020819052604090205443101561077c5760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208190526040812043600101905580806107b56107ae368790038701876151b6565b6000612122565b50919450925090506107c833863561273b565b84602001358310156107ec5760405162461bcd60e51b815260040161073190615668565b84604001358210156108105760405162461bcd60e51b8152600401610731906156d4565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561091957604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90610880908690600401615b96565b600060405180830381600087803b15801561089a57600080fd5b505af11580156108ae573d6000803e3d6000fd5b505060405133925085156108fc02915085906000818181858888f193505050501580156108df573d6000803e3d6000fd5b506109146001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633846127f2565b610a34565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c418339815191521461096f5760405162461bcd60e51b8152600401610731906157e2565b604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d906109a0908590600401615b96565b600060405180830381600087803b1580156109ba57600080fd5b505af11580156109ce573d6000803e3d6000fd5b505060405133925084156108fc02915084906000818181858888f193505050501580156109ff573d6000803e3d6000fd5b50610a346001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633856127f2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167ff56fda5b6bb8f75f807e6a868835d0a42a72be51a6f15fd874acdd3cbe3087df838686604051610abb93929190615b22565b60405180910390a350506006805460ff60a01b19169055505050565b601290565b600654600160a01b900460ff1615610b065760405162461bcd60e51b815260040161073190615725565b6006805460ff60a01b1916600160a01b17905533600081815260208190526040902054431015610b485760405162461bcd60e51b81526004016107319061590b565b6001600160a01b0380821660009081526020819052604090204360010190557f000000000000000000000000000000000000000000000000000000000000000016600080516020615c418339815191521415610cd85781353410610c4b578135341115610be05760405133908335340380156108fc02916000818181858888f19350505050158015610bde573d6000803e3d6000fd5b505b600080516020615c418339815191526001600160a01b031663d0e30db083600001356040518263ffffffff1660e01b81526004016000604051808303818588803b158015610c2d57600080fd5b505af1158015610c41573d6000803e3d6000fd5b5050505050610c9a565b610c66600080516020615c418339815191523330853561284d565b3415610c9a5760405133903480156108fc02916000818181858888f19350505050158015610c98573d6000803e3d6000fd5b505b610cd36001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330602086013561284d565b610ec4565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c418339815191521415610e555781602001353410610dcd578160200135341115610d625760405133906020840135340380156108fc02916000818181858888f19350505050158015610d60573d6000803e3d6000fd5b505b600080516020615c418339815191526001600160a01b031663d0e30db083602001356040518263ffffffff1660e01b81526004016000604051808303818588803b158015610daf57600080fd5b505af1158015610dc3573d6000803e3d6000fd5b5050505050610e1f565b610deb600080516020615c418339815191523330602086013561284d565b3415610e1f5760405133903480156108fc02916000818181858888f19350505050158015610e1d573d6000803e3d6000fd5b505b610cd36001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330853561284d565b610e8b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330853561284d565b610ec46001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330602086013561284d565b6000808080610ee0610edb3688900388018861512a565b612874565b9350935093509350816001600160801b031660001415610f1257610f0d33826001600160801b0316612ff9565b610f3a565b610f3a33610f35836001600160801b0316600554866001600160801b0316613080565b612ff9565b8560400135841015610f5e5760405162461bcd60e51b815260040161073190615668565b8560600135831015610f825760405162461bcd60e51b8152600401610731906156d4565b6000610f8f8735866120fa565b90506000610fa16020890135866120fa565b905081156110b2577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561107e57604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90611019908590600401615b96565b600060405180830381600087803b15801561103357600080fd5b505af1158015611047573d6000803e3d6000fd5b505060405133925084156108fc02915084906000818181858888f19350505050158015611078573d6000803e3d6000fd5b506110b2565b6110b26001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633846127f2565b80156111c1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561118d57604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90611128908490600401615b96565b600060405180830381600087803b15801561114257600080fd5b505af1158015611156573d6000803e3d6000fd5b505060405133925083156108fc02915083906000818181858888f19350505050158015611187573d6000803e3d6000fd5b506111c1565b6111c16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633836127f2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167ff1cc7b6dd2da0338fc327ee7647aeb5e744f007a5bbf0628a0033c6b5ffaf0be85898960405161124893929190615b22565b60405180910390a350506006805460ff60a01b19169055505050505050565b6006546001600160a01b031633146112915760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b0382166112b75760405162461bcd60e51b81526004016107319061570a565b6001600160a01b03821660009081526003602052604090819020805460ff1916831515179055517ff777a2dd5104082173537070fa5bc14b35ab543cc11a90836e264f3cef55c0649061130d9084908490615511565b60405180910390a15050565b6006546001600160a01b031633146113435760405162461bcd60e51b8152600401610731906157ac565b61271081106113645760405162461bcd60e51b81526004016107319061570a565b600b8190556040517f8b940a95968ad5b511f89b01075446a4fe9f614f2dc5fbb9e9a6b227d6d4fd7090611399908390615b96565b60405180910390a150565b6001600160a01b0381166000908152600160205260409020545b919050565b7f000000000000000000000000000000000000000000000000000000000000000081565b3360009081526003602052604090205460ff166114165760405162461bcd60e51b81526004016107319061575b565b336000818152602081905260409020544310156114455760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208190526040808220436001019055600a549051630b4c774160e11b815282918291829182914291909103908290731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290611511907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561152957600080fd5b505afa15801561153d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115619190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b15801561159957600080fd5b505afa1580156115ad573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115d1919061525b565b505050505050905061271060328a02816115e757fe5b0489826001600160a01b0316031080611611575061271060328a0204816001600160a01b03168a03105b61162d5760405162461bcd60e51b81526004016107319061584e565b60005b6009548110156117e1576009818154811061164757fe5b6000918252602090912001546001600160b81b03166116785760405162461bcd60e51b8152600401610731906157fd565b6009818154811061168557fe5b90600052602060002001600001601d9054906101000a900462ffffff1662ffffff168801975060008073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b031663fc6f78656040518060800160405280600987815481106116e957fe5b600091825260209182902001546001600160b81b0316825230908201526001600160801b036040808301829052606090920152516001600160e01b031960e084901b16815261173b9190600401615941565b6040805180830381600087803b15801561175457600080fd5b505af1158015611768573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061178c919061535c565b91509150818901985080880197506000806117c9600986815481106117ad57fe5b6000918252602090912001546001600160b81b0316878961312f565b99019897909701965050600190920191506116309050565b50600b54612710878202819004808701969288029190910480860195919089881115611824578997508984101561181c57838a039150611824565b899350600091505b8887111561184757889650888310156118405750818803611847565b5087915060005b42600a55871561188b5760065461188b906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811691168a6127f2565b86156118cb576006546118cb906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169116896127f2565b6000806118d78761327f565b915091507f8e9f83b74c783b864e2e6c8c8faff32b21e62ed8ceac8ae83d1da7e0ca1ba8fd8c8c8888888888886040516119189897969594939291906155c9565b60405180910390a1505050505050505050505050505050565b600b5481565b60046020526000908152604090205460ff1681565b6006546001600160a01b031690565b60088054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156106065780601f106105db57610100808354040283529160200191610606565b60606009805480602002602001604051908101604052809291908181526020016000905b82821015611a5457600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff1660608201528252600190920191016119e0565b50505050905090565b60006020819052908152604090205481565b600c5481565b600061061d338484612009565b7f000000000000000000000000000000000000000000000000000000000000000081565b6006546001600160a01b03163314611ad05760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b038216611af65760405162461bcd60e51b81526004016107319061570a565b6001600160a01b03821660009081526004602052604090819020805460ff1916831515179055517f4d5db8249eb7b157e4deefe2c0c726e88aeee890e3dd5a95c68dfe435bf652ff9061130d9084908490615511565b6001600160a01b03918216600090815260026020908152604080832093909416825291909152205490565b33600081815260208190526040902054431015611ba65760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208181526040808320436001019055338352600490915290205460ff16611bef5760405162461bcd60e51b815260040161073190615683565b611bfb60096000614dbc565b60005b8351811015611deb57838181518110611c1357fe5b60200260200101516040015160020b848281518110611c2e57fe5b60200260200101516020015160020b13611c5a5760405162461bcd60e51b815260040161073190615632565b8015611cb757836001820381518110611c6f57fe5b60200260200101516040015160020b848281518110611c8a57fe5b60200260200101516020015160020b1315611cb75760405162461bcd60e51b815260040161073190615632565b6000848281518110611cc557fe5b60200260200101516060015162ffffff1611611cf35760405162461bcd60e51b81526004016107319061564d565b838181518110611cff57fe5b6020026020010151600001516001600160b81b0316600014611d335760405162461bcd60e51b8152600401610731906156b9565b6009848281518110611d4157fe5b60209081029190910181015182546001818101855560009485529383902082519101805493830151604084015160609094015162ffffff908116600160e81b026001600160e81b03600296870b8316600160d01b0262ffffff60d01b199490970b909216600160b81b0262ffffff60b81b196001600160b81b039096166001600160b81b031990981697909717949094169590951716929092179290921691909117905501611bfe565b507fc1c463f316d85c725592f65d9d6f5b014967c835e0b8de363fbc10dcb752dd81600080600080600080600080604051611e2d9897969594939291906155c9565b60405180910390a1505050565b6006546001600160a01b03163314611e645760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b038116611e8a5760405162461bcd60e51b81526004016107319061570a565b600680546001600160a01b0319166001600160a01b0383161790556040517fcfaaa26691e16e66e73290fc725eee1a6b4e0e693a1640484937aac25ffb55a490611399908390615497565b6006546001600160a01b03163314611eff5760405162461bcd60e51b8152600401610731906157ac565b6127108110611f205760405162461bcd60e51b81526004016107319061570a565b600c8190556040517fd87632b1c6ebfa21acbca0e3279b3cf6385a377cb8fda51e5b866baa6e6012ab90611399908390615b96565b6001600160a01b038316611f7b5760405162461bcd60e51b8152600401610731906157c7565b6001600160a01b038216611fa15760405162461bcd60e51b8152600401610731906156ef565b6001600160a01b0380841660008181526002602090815260408083209487168084529490915290819020849055517f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590611ffc908590615b96565b60405180910390a3505050565b6001600160a01b03831661202f5760405162461bcd60e51b815260040161073190615791565b6001600160a01b0382166120555760405162461bcd60e51b815260040161073190615869565b6001600160a01b0383166000908152600160205260409020548181101561208e5760405162461bcd60e51b815260040161073190615776565b6001600160a01b0380851660008181526001602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906120ec908690615b96565b60405180910390a350505050565b60008282111561211c5760405162461bcd60e51b815260040161073190615740565b50900390565b600080600061212f614ddd565b60006009805480602002602001604051908101604052809291908181526020016000905b828210156121c757600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff166060820152825260019092019101612153565b5050600a54604051630b4c774160e11b815293945042039260009250731f98431c8ad98523631ae4a59f267346ea31f9849150631698ee8290612272907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561228a57600080fd5b505afa15801561229e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122c29190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b1580156122fa57600080fd5b505afa15801561230e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612332919061525b565b505050505050905060005b835181101561270457600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab8886848151811061237757fe5b6020026020010151600001516040518263ffffffff1660e01b815260040161239f9190615b43565b6101806040518083038186803b1580156123b857600080fd5b505afa1580156123cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123f0919061537f565b505050509750505050505050506000612419826001600160801b03168d60000151600554613080565b905060006040518060a0016040528088868151811061243457fe5b6020026020010151600001516001600160b81b03168152602001836001600160801b0316815260200160008152602001600081526020018e60800151815250905061247d614e13565b604051630624e65f60e11b815273c36442b4a4522e871399cd717abdd847ab11fe8890630c49ccbe906124b4908590600401615984565b6040805180830381600087803b1580156124cd57600080fd5b505af11580156124e1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612505919061535c565b60208301528152612514614e13565b73c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b031663fc6f786560405180608001604052808c8a8151811061254e57fe5b602090810291909101810151516001600160b81b0316825230908201526001600160801b036040808301829052606090920152516001600160e01b031960e084901b1681526125a09190600401615941565b6040805180830381600087803b1580156125b957600080fd5b505af11580156125cd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f1919061535c565b602083015281528151612605908e90613b04565b9c5061261e82602001518d613b0490919063ffffffff16565b9b50838b019a50836001600160801b03168b6001600160801b031610156126575760405162461bcd60e51b81526004016107319061589f565b8d156126f357815181518b5161266f92909103613b04565b8a5260208083015182820151918c015161268c9290919003613b04565b60208b015288516126bf908a90889081106126a357fe5b6020026020010151600001516001600160b81b0316888a61312f565b602084015280835260408b01516126d591613b04565b60408b0152602082015160608b01516126ed91613b04565b60608b01525b50506001909301925061233d915050565b50871561272f57600b5484516127109102046080850152600b54602085015161271091020460a08501525b50505092959194509250565b6001600160a01b0382166127615760405162461bcd60e51b8152600401610731906158f0565b6001600160a01b0382166000908152600160205260409020548181101561279a5760405162461bcd60e51b815260040161073190615833565b6001600160a01b0383166000818152600160205260408082208585039055600580548690039055519091907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90611ffc908690615b96565b6128488363a9059cbb60e01b848460405160240161281192919061552c565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152613b29565b505050565b61286e846323b872dd60e01b858585604051602401612811939291906154ed565b50505050565b60008060008060006009805480602002602001604051908101604052809291908181526020016000905b8282101561291257600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff16606082015282526001909201910161289e565b50508851929350612965926001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925073c36442b4a4522e871399cd717abdd847ab11fe889150613b9c565b60208601516129b4906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169073c36442b4a4522e871399cd717abdd847ab11fe8890613b9c565b6000806000835167ffffffffffffffff811180156129d157600080fd5b506040519080825280602002602001820160405280156129fb578160200160208202803683370190505b5090506000845167ffffffffffffffff81118015612a1857600080fd5b50604051908082528060200260200182016040528015612a42578160200160208202803683370190505b509050612a4e85613c5f565b919a5092965090945090925090508315801590612a6b5750600083115b15612a8d57612a87858b600001518c60200151878787876142c8565b90945092505b60005b8551811015612f5a5760006040518061016001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f000000000000000000000000000000000000000000000000000000000000000062ffffff168152602001888481518110612b3c57fe5b60200260200101516040015160020b8152602001888481518110612b5c57fe5b60200260200101516020015160020b815260200160008152602001600081526020016000815260200160008152602001306001600160a01b031681526020018d60a00151815250905060006040518060c00160405280898581518110612bbe57fe5b6020026020010151600001516001600160b81b03168152602001600081526020016000815260200160008152602001600081526020018e60a0015181525090506000871115612c5a57612c298d60000151868581518110612c1b57fe5b602002602001015189613080565b60a08301819052602082015260408d01518551612c4d9190879086908110612c1b57fe5b60e0830181905260608201525b8515612cb457612c828d60200151858581518110612c7457fe5b602002602001015188613080565b60c08301819052604082015260608d01518451612ca69190869086908110612c7457fe5b610100830181905260808201525b60008260a001511180612ccb575060008260c00151115b15612f5057612cd8614e2d565b888481518110612ce457fe5b6020026020010151600001516001600160b81b031660001415612e4857604051634418b22b60e11b815273c36442b4a4522e871399cd717abdd847ab11fe8890638831645690612d38908690600401615a74565b608060405180830381600087803b158015612d5257600080fd5b505af1925050508015612d82575060408051601f3d908101601f19168201909252612d7f91810190615321565b60015b612d8b57612da9565b9284526001600160801b039091166020840152604083015260608201525b80516009805486908110612db957fe5b600091825260209091200180546001600160b81b0319166001600160b81b03929092169190911790556040810151612df2908e90613b04565b9c50612e0b81606001518d613b0490919063ffffffff16565b6020820151909c50998a01996001600160801b03908116908b161015612e435760405162461bcd60e51b81526004016107319061589f565b612f4e565b60405163219f5d1760e01b815273c36442b4a4522e871399cd717abdd847ab11fe889063219f5d1790612e7f908590600401615a30565b606060405180830381600087803b158015612e9957600080fd5b505af1925050508015612ec9575060408051601f3d908101601f19168201909252612ec691810190615227565b60015b612ed257612eed565b6001600160801b039092166020840152604083015260608201525b6040810151612efd908e90613b04565b9c50612f1681606001518d613b0490919063ffffffff16565b6020820151909c50998a01996001600160801b03908116908b161015612f4e5760405162461bcd60e51b81526004016107319061589f565b505b5050600101612a90565b50612fa46001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673c36442b4a4522e871399cd717abdd847ab11fe886000613b9c565b612fed6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673c36442b4a4522e871399cd717abdd847ab11fe886000613b9c565b50505050509193509193565b6001600160a01b03821661301f5760405162461bcd60e51b815260040161073190615926565b60058054820190556001600160a01b038216600081815260016020526040808220805485019055517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90613074908590615b96565b60405180910390a35050565b60008080600019858709868602925082811090839003039050806130b657600084116130ab57600080fd5b508290049050610700565b8084116130c257600080fd5b6000848688096000868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b600080600080600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab88896040518263ffffffff1660e01b81526004016131779190615b96565b6101806040518083038186803b15801561319057600080fd5b505afa1580156131a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131c8919061537f565b50505050975097509750505050505060006131e284614691565b905060006131ef84614691565b90506000806132008b8585886149aa565b915091506127106301e1855861322b8c613225600c5487614a4690919063ffffffff16565b90614a46565b8161323257fe5b048161323a57fe5b0498506127106301e1855861325e8c613225600c5486614a4690919063ffffffff16565b8161326557fe5b048161326d57fe5b04975050505050505050935093915050565b6006546000908190600160a01b900460ff16156132ae5760405162461bcd60e51b815260040161073190615725565b6006805460ff60a01b1916600160a01b1790556040516370a0823160e01b81526000906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190613310903090600401615497565b60206040518083038186803b15801561332857600080fd5b505afa15801561333c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133609190615309565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166370a08231306040518263ffffffff1660e01b81526004016133b09190615497565b60206040518083038186803b1580156133c857600080fd5b505afa1580156133dc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906134009190615309565b90506000806134456040518060c001604052808681526020018581526020016000815260200160008152602001306001600160a01b0316815260200142815250612874565b509193509150613457905084836120fa565b935061346383826120fa565b925061346f8683613b04565b955061347b8582613b04565b945060006134898484614a46565b6134938684614a46565b11806134b15750821580156134a6575081155b80156134b157508385115b156136e757821580156134c2575081155b156134d1575060028404613522565b816135016134ed858b6001600160a01b0316600160601b613080565b8a6001600160a01b0316600160601b613080565b0161350c8585614a46565b6135168785614a46565b038161351e57fe5b0490505b61356a6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c0586156483613b9c565b60408051610100810182526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811682527f00000000000000000000000000000000000000000000000000000000000000001660208201527f000000000000000000000000000000000000000000000000000000000000000062ffffff168183015230606082015242608082015260a08101839052600060c0820181905260e0820152905163414bf38960e01b815273e592427a0aece92de3edee1f18e0157c058615649163414bf3899161364991906004016159c7565b602060405180830381600087803b15801561366357600080fd5b505af1925050508015613693575060408051601f3d908101601f1916820190925261369091810190615309565b60015b61369c5761369e565b505b6136e76001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c058615646000613b9c565b6136f18484614a46565b6136fb8684614a46565b108061371957508215801561370e575081155b801561371957508385105b1561394f578215801561372a575081155b1561373957506002830461378a565b8261376961375584600160601b8c6001600160a01b0316613080565b600160601b8b6001600160a01b0316613080565b016137748684614a46565b61377e8686614a46565b038161378657fe5b0490505b6137d26001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c0586156483613b9c565b60408051610100810182526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811682527f00000000000000000000000000000000000000000000000000000000000000001660208201527f000000000000000000000000000000000000000000000000000000000000000062ffffff168183015230606082015242608082015260a08101839052600060c0820181905260e0820152905163414bf38960e01b815273e592427a0aece92de3edee1f18e0157c058615649163414bf389916138b191906004016159c7565b602060405180830381600087803b1580156138cb57600080fd5b505af19250505080156138fb575060408051601f3d908101601f191682019092526138f891810190615309565b60015b61390457613906565b505b61394f6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c058615646000613b9c565b6040516370a0823160e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a082319061399b903090600401615497565b60206040518083038186803b1580156139b357600080fd5b505afa1580156139c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906139eb9190615309565b6040516370a0823160e01b81529095506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190613a3a903090600401615497565b60206040518083038186803b158015613a5257600080fd5b505afa158015613a66573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613a8a9190615309565b9350613acc6040518060c001604052808781526020018681526020016000815260200160008152602001306001600160a01b0316815260200142815250612874565b509194509250613ade90508784613b04565b9650613aea8683613b04565b6006805460ff60a01b191690559698969750505050505050565b6000828201838110156107005760405162461bcd60e51b81526004016107319061589f565b6000613b6282604051806040016040528060018152602001603760f81b815250856001600160a01b0316614a809092919063ffffffff16565b8051909150156128485780806020019051810190613b8091906150f7565b6128485760405162461bcd60e51b8152600401610731906158d5565b801580613c245750604051636eb1769f60e11b81526001600160a01b0384169063dd62ed3e90613bd290309086906004016154ab565b60206040518083038186803b158015613bea57600080fd5b505afa158015613bfe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613c229190615309565b155b613c405760405162461bcd60e51b8152600401610731906158ba565b6128488363095ea7b360e01b848460405160240161281192919061552c565b6000806000606080855167ffffffffffffffff81118015613c7f57600080fd5b50604051908082528060200260200182016040528015613ca9578160200160208202803683370190505b509150855167ffffffffffffffff81118015613cc457600080fd5b50604051908082528060200260200182016040528015613cee578160200160208202803683370190505b50604051630b4c774160e11b81529091506000908190731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290613d92907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b158015613daa57600080fd5b505afa158015613dbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613de29190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b158015613e1a57600080fd5b505afa158015613e2e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613e52919061525b565b505050505091509150613e63614e13565b600080613e878b600081518110613e7657fe5b602002602001015160400151614691565b6001600160a01b031683528a51613eb4908c90600090613ea357fe5b602002602001015160200151614691565b6001600160a01b031660208401528a518b90600090613ecf57fe5b60200260200101516060015162ffffff1691508a60018c510381518110613ef257fe5b60200260200101516060015162ffffff16905060005b8b518110156142b95760008c8281518110613f1f57fe5b6020026020010151600001516001600160b81b03161115613ff857600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab888e8481518110613f6b57fe5b6020026020010151600001516040518263ffffffff1660e01b8152600401613f939190615b43565b6101806040518083038186803b158015613fac57600080fd5b505afa158015613fc0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613fe4919061537f565b50505050975050505050505050808a019950505b614000614e13565b61400f8d8381518110613e7657fe5b6001600160a01b031681528c5161402c908e9084908110613ea357fe5b6001600160a01b031660208201528c518d908390811061404857fe5b60200260200101516040015160020b8660020b136140fc57838d838151811061406d57fe5b60200260200101516060015162ffffff166140be6140b16140a189600001518a602001518b600001518c6020015103613080565b8551602087015190810390613080565b8451600160601b90613080565b02816140c657fe5b048983815181106140d357fe5b6020026020010181815250508882815181106140eb57fe5b60200260200101518c019b506142b0565b8c828151811061410857fe5b60200260200101516020015160020b8660020b1261419f57828d838151811061412d57fe5b60200260200101516060015162ffffff166141618360000151846020015103600160601b89600001518a6020015103613080565b028161416957fe5b0488838151811061417657fe5b60200260200101818152505087828151811061418e57fe5b60200260200101518b019a506142b0565b838d83815181106141ac57fe5b60200260200101516060015162ffffff166141f76137556141e089600001518a602001518b600001518c6020015103613080565b60208601516001600160a01b038d16810390613080565b02816141ff57fe5b0489838151811061420c57fe5b602002602001018181525050828d838151811061422557fe5b60200260200101516060015162ffffff1661425e83600001518a6001600160a01b031603600160601b89600001518a6020015103613080565b028161426657fe5b0488838151811061427357fe5b60200260200101818152505088828151811061428b57fe5b60200260200101518c019b508782815181106142a357fe5b60200260200101518b019a505b50600101613f08565b50505050505091939590929450565b6000808851600114156142df575084905083614685565b6142e7614e13565b604051630b4c774160e11b8152600090731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290614385907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561439d57600080fd5b505afa1580156143b1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906143d59190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b15801561440d57600080fd5b505afa158015614421573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190614445919061525b565b50505050505090506144a6816144618d600081518110613e7657fe5b6144718e600081518110613ea357fe5b6144908e8b60008151811061448257fe5b60200260200101518e613080565b6144a18e8b60008151811061448257fe5b614a97565b6001600160801b031682528a5160001981019061450c9083906144cf908f9085908110613e7657fe5b6144de8f8581518110613ea357fe5b6144fc8f8c87815181106144ee57fe5b60200260200101518f613080565b6144a18f8c88815181106144ee57fe5b6001600160801b031660208401528b518c9060009061452757fe5b60200260200101516060015162ffffff168360200151028c828151811061454a57fe5b60200260200101516060015162ffffff1684600001510211156145f75760008c60008151811061457657fe5b60200260200101516060015162ffffff1684602001510211156145ea576145e3898d83815181106145a357fe5b60200260200101516060015162ffffff168560000151028e6000815181106145c757fe5b60200260200101516060015162ffffff16866020015102613080565b94506145ef565b600094505b879350614681565b88945060008c828151811061460857fe5b60200260200101516060015162ffffff16846000015102111561467c57614675888d60008151811061463657fe5b60200260200101516060015162ffffff168560200151028e848151811061465957fe5b60200260200101516060015162ffffff16866000015102613080565b9350614681565b600093505b5050505b97509795505050505050565b60008060008360020b126146a8578260020b6146b0565b8260020b6000035b9050620d89e88111156146d55760405162461bcd60e51b8152600401610731906157c7565b6000600182166146e957600160801b6146fb565b6ffffcb933bd6fad37aa2d162d1a5940015b70ffffffffffffffffffffffffffffffffff169050600282161561472f576ffff97272373d413259a46990580e213a0260801c5b600482161561474e576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b600882161561476d576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b601082161561478c576fffcb9843d60f6159c9db58835c9266440260801c5b60208216156147ab576fff973b41fa98c081472e6896dfb254c00260801c5b60408216156147ca576fff2ea16466c96a3843ec78b326b528610260801c5b60808216156147e9576ffe5dee046a99a2a811c461f1969c30530260801c5b610100821615614809576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b610200821615614829576ff987a7253ac413176f2b074cf7815e540260801c5b610400821615614849576ff3392b0822b70005940c7a398e4b70f30260801c5b610800821615614869576fe7159475a2c29b7443b29c7fa6e889d90260801c5b611000821615614889576fd097f3bdfd2022b8845ad8f792aa58250260801c5b6120008216156148a9576fa9f746462d870fdf8a65dc1f90e061e50260801c5b6140008216156148c9576f70d869a156d2a1b890bb3df62baf32f70260801c5b6180008216156148e9576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561490a576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b6202000082161561492a576e5d6af8dedb81196699c329225ee6040260801c5b62040000821615614949576d2216e584f5fa1ea926041bedfe980260801c5b62080000821615614966576b048a170391f7dc42444e8fa20260801c5b60008460020b131561498157806000198161497d57fe5b0490505b640100000000810615614995576001614998565b60005b60ff16602082901c0192505050919050565b600080836001600160a01b0316856001600160a01b031611156149cb579293925b846001600160a01b0316866001600160a01b0316116149f6576149ef858585614b5b565b9150614a3d565b836001600160a01b0316866001600160a01b03161015614a2f57614a1b868585614b5b565b9150614a28858785614bc4565b9050614a3d565b614a3a858585614bc4565b90505b94509492505050565b600082614a5557506000610621565b82820282848281614a6257fe5b04146107005760405162461bcd60e51b815260040161073190615818565b6060614a8f8484600085614c07565b949350505050565b6000836001600160a01b0316856001600160a01b03161115614ab7579293925b846001600160a01b0316866001600160a01b031611614ae257614adb858585614cc7565b9050614b52565b836001600160a01b0316866001600160a01b03161015614b44576000614b09878686614cc7565b90506000614b18878986614d2a565b9050806001600160801b0316826001600160801b031610614b395780614b3b565b815b92505050614b52565b614b4f858584614d2a565b90505b95945050505050565b6000826001600160a01b0316846001600160a01b03161115614b7b579192915b836001600160a01b0316614bb4606060ff16846001600160801b0316901b8686036001600160a01b0316866001600160a01b0316613080565b81614bbb57fe5b04949350505050565b6000826001600160a01b0316846001600160a01b03161115614be4579192915b614a8f826001600160801b03168585036001600160a01b0316600160601b613080565b606082471015614c295760405162461bcd60e51b81526004016107319061569e565b614c3285614d67565b614c4e5760405162461bcd60e51b815260040161073190615884565b600080866001600160a01b03168587604051614c6a919061547b565b60006040518083038185875af1925050503d8060008114614ca7576040519150601f19603f3d011682016040523d82523d6000602084013e614cac565b606091505b5091509150614cbc828286614d6d565b979650505050505050565b6000826001600160a01b0316846001600160a01b03161115614ce7579192915b6000614d0a856001600160a01b0316856001600160a01b0316600160601b613080565b9050614b52614d2584838888036001600160a01b0316613080565b614da6565b6000826001600160a01b0316846001600160a01b03161115614d4a579192915b614a8f614d2583600160601b8787036001600160a01b0316613080565b3b151590565b60608315614d7c575081610700565b825115614d8c5782518084602001fd5b8160405162461bcd60e51b815260040161073191906155ff565b806001600160801b03811681146113be57600080fd5b5080546000825590600052602060002090810190614dda9190614e5e565b50565b6040518060c001604052806000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604051806040016040528060008152602001600081525090565b60405180608001604052806000815260200160006001600160801b0316815260200160008152602001600081525090565b5b80821115614e735760008155600101614e5f565b5090565b80516113be81615bfd565b80356113be81615c20565b80516113be81615c20565b80516001600160801b03811681146113be57600080fd5b805161ffff811681146113be57600080fd5b80356113be81615c2f565b80516113be81615c2f565b600060208284031215614ee8578081fd5b813561070081615bfd565b600060208284031215614f04578081fd5b815161070081615bfd565b60008060408385031215614f21578081fd5b8235614f2c81615bfd565b91506020830135614f3c81615bfd565b809150509250929050565b600080600060608486031215614f5b578081fd5b8335614f6681615bfd565b92506020840135614f7681615bfd565b929592945050506040919091013590565b60008060408385031215614f99578182fd5b8235614fa481615bfd565b91506020830135614f3c81615c12565b60008060408385031215614fc6578182fd5b8235614fd181615bfd565b946020939093013593505050565b6000806040808486031215614ff2578283fd5b833567ffffffffffffffff80821115615009578485fd5b818601915086601f83011261501c578485fd5b813560208282111561502a57fe5b6150378182840201615bad565b828152818101908583016080808602880185018d1015615055578a8bfd5b8a97505b858810156150e35780828e03121561506f578a8bfd5b8851818101818110898211171561508257fe5b8a5282356001600160b81b038116811461509a578c8dfd5b8152828601356150a981615c20565b818701526150b8838b01614e82565b8a82015260606150c9818501614ec1565b908201528452600197909701969284019290810190615059565b50909b999092013599505050505050505050565b600060208284031215615108578081fd5b815161070081615c12565b600060c08284031215615124578081fd5b50919050565b600060c0828403121561513b578081fd5b60405160c0810181811067ffffffffffffffff8211171561515857fe5b806040525082358152602083013560208201526040830135604082015260608301356060820152608083013561518d81615bfd565b608082015260a0928301359281019290925250919050565b600060a08284031215615124578081fd5b600060a082840312156151c7578081fd5b60405160a0810181811067ffffffffffffffff821117156151e457fe5b8060405250823581526020830135602082015260408301356040820152606083013561520f81615bfd565b60608201526080928301359281019290925250919050565b60008060006060848603121561523b578081fd5b61524484614e98565b925060208401519150604084015190509250925092565b600080600080600080600060e0888a031215615275578485fd5b875161528081615bfd565b602089015190975061529181615c20565b955061529f60408901614eaf565b94506152ad60608901614eaf565b93506152bb60808901614eaf565b925060a088015160ff811681146152d0578283fd5b60c08901519092506152e181615c12565b8091505092959891949750929550565b600060208284031215615302578081fd5b5035919050565b60006020828403121561531a578081fd5b5051919050565b60008060008060808587031215615336578182fd5b8451935061534660208601614e98565b6040860151606090960151949790965092505050565b6000806040838503121561536e578182fd5b505080516020909101519092909150565b6000806000806000806000806000806000806101808d8f0312156153a1578586fd5b8c516bffffffffffffffffffffffff811681146153bc578687fd5b9b506153ca60208e01614e77565b9a506153d860408e01614e77565b99506153e660608e01614e77565b98506153f460808e01614ecc565b975061540260a08e01614e8d565b965061541060c08e01614e8d565b955061541e60e08e01614e98565b94506101008d015193506101208d0151925061543d6101408e01614e98565b915061544c6101608e01614e98565b90509295989b509295989b509295989b565b6001600160a01b03169052565b60020b9052565b62ffffff169052565b6000825161548d818460208701615bd1565b9190910192915050565b6001600160a01b0391909116815260200190565b6001600160a01b0392831681529116602082015260400190565b6001600160a01b03938416815291909216602082015262ffffff909116604082015260600190565b6001600160a01b039384168152919092166020820152604081019190915260600190565b6001600160a01b039290921682521515602082015260400190565b6001600160a01b03929092168252602082015260400190565b602080825282518282018190526000919060409081850190868401855b828110156155b157815180516001600160b81b0316855286810151600290810b8887015286820151900b8686015260609081015162ffffff169085015260809093019290850190600101615562565b5091979650505050505050565b901515815260200190565b978852602088019690965260408701949094526060860192909252608085015260a084015260c083015260e08201526101000190565b600060208252825180602084015261561e816040850160208701615bd1565b601f01601f19169190910160400192915050565b6020808252600190820152604360f81b604082015260600190565b6020808252600190820152604160f81b604082015260600190565b6020808252600190820152604760f81b604082015260600190565b6020808252600190820152606360f81b604082015260600190565b6020808252600190820152600d60fa1b604082015260600190565b6020808252600190820152602160f91b604082015260600190565b6020808252600190820152600960fb1b604082015260600190565b6020808252600190820152605560f81b604082015260600190565b6020808252600190820152606160f81b604082015260600190565b6020808252600190820152604560f81b604082015260600190565b6020808252600190820152600b60fb1b604082015260600190565b6020808252600190820152601160fa1b604082015260600190565b6020808252600190820152600560fc1b604082015260600190565b6020808252600190820152602760f91b604082015260600190565b6020808252600190820152604d60f81b604082015260600190565b6020808252600190820152601560fa1b604082015260600190565b6020808252600190820152602560f91b604082015260600190565b6020808252600190820152604b60f81b604082015260600190565b6020808252600190820152605960f81b604082015260600190565b6020808252600190820152605360f81b604082015260600190565b6020808252600190820152603160f91b604082015260600190565b6020808252600190820152604f60f81b604082015260600190565b6020808252600190820152603560f81b604082015260600190565b6020808252600190820152605760f81b604082015260600190565b6020808252600190820152601b60f91b604082015260600190565b6020808252600190820152600760fb1b604082015260600190565b6020808252600190820152602960f91b604082015260600190565b6020808252600190820152602b60f91b604082015260600190565b6020808252600190820152605160f81b604082015260600190565b815181526020808301516001600160a01b0316908201526040808301516001600160801b0390811691830191909152606092830151169181019190915260800190565b600060a082019050825182526001600160801b03602084015116602083015260408301516040830152606083015160608301526080830151608083015292915050565b81516001600160a01b03908116825260208084015182169083015260408084015162ffffff16908301526060808401518216908301526080808401519083015260a0838101519083015260c0808401519083015260e09283015116918101919091526101000190565b600060c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b600061016082019050615a8882845161545e565b6020830151615a9a602084018261545e565b506040830151615aad6040840182615472565b506060830151615ac0606084018261546b565b506080830151615ad3608084018261546b565b5060a083015160a083015260c083015160c083015260e083015160e083015261010080840151818401525061012080840151615b118285018261545e565b505061014092830151919092015290565b6001600160801b039390931683526020830191909152604082015260600190565b6001600160b81b0391909116815260200190565b6001600160b81b03949094168452600292830b6020850152910b604083015262ffffff16606082015260800190565b62ffffff91909116815260200190565b90815260200190565b60ff91909116815260200190565b60405181810167ffffffffffffffff81118282101715615bc957fe5b604052919050565b60005b83811015615bec578181015183820152602001615bd4565b8381111561286e5750506000910152565b6001600160a01b0381168114614dda57600080fd5b8015158114614dda57600080fd5b8060020b8114614dda57600080fd5b62ffffff81168114614dda57600080fdfe000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2a26469706673582212206c2235d8d0c2c276add461f3a1d1360311c2fb3f3f8371fb8849df89ae84094a64736f6c63430007060033\",\n  \"deployedBytecode\": \"0x6080604052600436106101c65760003560e01c806383b4918b116100f7578063a6f7f5d611610095578063dd62ed3e11610064578063dd62ed3e146104fa578063e1b5c3121461051a578063f2fde38b1461053a578063fe56e2321461055a576101e7565b8063a6f7f5d614610490578063a9059cbb146104a5578063d21220a7146104c5578063db5d820e146104da576101e7565b80638da5cb5b116100d15780638da5cb5b1461042457806395d89b411461043957806396d01a7d1461044e5780639f3e8b3414610470576101e7565b806383b4918b146103cf57806387788782146103ef5780638c231af914610404576101e7565b8063276cd920116101645780634623c91d1161013e5780634623c91d1461034d57806370897b231461036d57806370a082311461038d5780637cf134cb146103ad576101e7565b8063276cd920146102f8578063313ce5671461031857806333bc230a1461033a576101e7565b806315723866116101a0578063157238661461026657806318160ddd14610296578063223b3b7a146102b857806323b872dd146102d8576101e7565b806306fdde03146101ec578063095ea7b3146102175780630dfe168114610244576101e7565b366101e75733600080516020615c41833981519152146101e557600080fd5b005b600080fd5b3480156101f857600080fd5b5061020161057a565b60405161020e91906155ff565b60405180910390f35b34801561022357600080fd5b50610237610232366004614fb4565b610610565b60405161020e91906155be565b34801561025057600080fd5b50610259610627565b60405161020e9190615497565b34801561027257600080fd5b506102866102813660046152f1565b61064b565b60405161020e9493929190615b57565b3480156102a257600080fd5b506102ab61069a565b60405161020e9190615b96565b3480156102c457600080fd5b506102376102d3366004614ed7565b6106a0565b3480156102e457600080fd5b506102376102f3366004614f47565b6106b5565b34801561030457600080fd5b506101e56103133660046151a5565b610707565b34801561032457600080fd5b5061032d610ad7565b60405161020e9190615b9f565b6101e5610348366004615113565b610adc565b34801561035957600080fd5b506101e5610368366004614f87565b611267565b34801561037957600080fd5b506101e56103883660046152f1565b611319565b34801561039957600080fd5b506102ab6103a8366004614ed7565b6113a4565b3480156103b957600080fd5b506103c26113c3565b60405161020e9190615b86565b3480156103db57600080fd5b506101e56103ea3660046152f1565b6113e7565b3480156103fb57600080fd5b506102ab611931565b34801561041057600080fd5b5061023761041f366004614ed7565b611937565b34801561043057600080fd5b5061025961194c565b34801561044557600080fd5b5061020161195b565b34801561045a57600080fd5b506104636119bc565b60405161020e9190615545565b34801561047c57600080fd5b506102ab61048b366004614ed7565b611a5d565b34801561049c57600080fd5b506102ab611a6f565b3480156104b157600080fd5b506102376104c0366004614fb4565b611a75565b3480156104d157600080fd5b50610259611a82565b3480156104e657600080fd5b506101e56104f5366004614f87565b611aa6565b34801561050657600080fd5b506102ab610515366004614f0f565b611b4c565b34801561052657600080fd5b506101e5610535366004614fdf565b611b77565b34801561054657600080fd5b506101e5610555366004614ed7565b611e3a565b34801561056657600080fd5b506101e56105753660046152f1565b611ed5565b60078054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156106065780601f106105db57610100808354040283529160200191610606565b820191906000526020600020905b8154815290600101906020018083116105e957829003601f168201915b5050505050905090565b600061061d338484611f55565b5060015b92915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b6009818154811061065b57600080fd5b6000918252602090912001546001600160b81b0381169150600160b81b8104600290810b91600160d01b810490910b90600160e81b900462ffffff1684565b60055490565b60036020526000908152604090205460ff1681565b60006106c2848484612009565b6001600160a01b0384166000908152600260209081526040808320338085529252909120546106fc9186916106f790866120fa565b611f55565b5060015b9392505050565b600654600160a01b900460ff161561073a5760405162461bcd60e51b815260040161073190615725565b60405180910390fd5b6006805460ff60a01b1916600160a01b1790553360008181526020819052604090205443101561077c5760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208190526040812043600101905580806107b56107ae368790038701876151b6565b6000612122565b50919450925090506107c833863561273b565b84602001358310156107ec5760405162461bcd60e51b815260040161073190615668565b84604001358210156108105760405162461bcd60e51b8152600401610731906156d4565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561091957604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90610880908690600401615b96565b600060405180830381600087803b15801561089a57600080fd5b505af11580156108ae573d6000803e3d6000fd5b505060405133925085156108fc02915085906000818181858888f193505050501580156108df573d6000803e3d6000fd5b506109146001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633846127f2565b610a34565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c418339815191521461096f5760405162461bcd60e51b8152600401610731906157e2565b604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d906109a0908590600401615b96565b600060405180830381600087803b1580156109ba57600080fd5b505af11580156109ce573d6000803e3d6000fd5b505060405133925084156108fc02915084906000818181858888f193505050501580156109ff573d6000803e3d6000fd5b50610a346001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633856127f2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167ff56fda5b6bb8f75f807e6a868835d0a42a72be51a6f15fd874acdd3cbe3087df838686604051610abb93929190615b22565b60405180910390a350506006805460ff60a01b19169055505050565b601290565b600654600160a01b900460ff1615610b065760405162461bcd60e51b815260040161073190615725565b6006805460ff60a01b1916600160a01b17905533600081815260208190526040902054431015610b485760405162461bcd60e51b81526004016107319061590b565b6001600160a01b0380821660009081526020819052604090204360010190557f000000000000000000000000000000000000000000000000000000000000000016600080516020615c418339815191521415610cd85781353410610c4b578135341115610be05760405133908335340380156108fc02916000818181858888f19350505050158015610bde573d6000803e3d6000fd5b505b600080516020615c418339815191526001600160a01b031663d0e30db083600001356040518263ffffffff1660e01b81526004016000604051808303818588803b158015610c2d57600080fd5b505af1158015610c41573d6000803e3d6000fd5b5050505050610c9a565b610c66600080516020615c418339815191523330853561284d565b3415610c9a5760405133903480156108fc02916000818181858888f19350505050158015610c98573d6000803e3d6000fd5b505b610cd36001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330602086013561284d565b610ec4565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c418339815191521415610e555781602001353410610dcd578160200135341115610d625760405133906020840135340380156108fc02916000818181858888f19350505050158015610d60573d6000803e3d6000fd5b505b600080516020615c418339815191526001600160a01b031663d0e30db083602001356040518263ffffffff1660e01b81526004016000604051808303818588803b158015610daf57600080fd5b505af1158015610dc3573d6000803e3d6000fd5b5050505050610e1f565b610deb600080516020615c418339815191523330602086013561284d565b3415610e1f5760405133903480156108fc02916000818181858888f19350505050158015610e1d573d6000803e3d6000fd5b505b610cd36001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330853561284d565b610e8b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330853561284d565b610ec46001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000163330602086013561284d565b6000808080610ee0610edb3688900388018861512a565b612874565b9350935093509350816001600160801b031660001415610f1257610f0d33826001600160801b0316612ff9565b610f3a565b610f3a33610f35836001600160801b0316600554866001600160801b0316613080565b612ff9565b8560400135841015610f5e5760405162461bcd60e51b815260040161073190615668565b8560600135831015610f825760405162461bcd60e51b8152600401610731906156d4565b6000610f8f8735866120fa565b90506000610fa16020890135866120fa565b905081156110b2577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561107e57604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90611019908590600401615b96565b600060405180830381600087803b15801561103357600080fd5b505af1158015611047573d6000803e3d6000fd5b505060405133925084156108fc02915084906000818181858888f19350505050158015611078573d6000803e3d6000fd5b506110b2565b6110b26001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633846127f2565b80156111c1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316600080516020615c41833981519152141561118d57604051632e1a7d4d60e01b8152600080516020615c4183398151915290632e1a7d4d90611128908490600401615b96565b600060405180830381600087803b15801561114257600080fd5b505af1158015611156573d6000803e3d6000fd5b505060405133925083156108fc02915083906000818181858888f19350505050158015611187573d6000803e3d6000fd5b506111c1565b6111c16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001633836127f2565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03167ff1cc7b6dd2da0338fc327ee7647aeb5e744f007a5bbf0628a0033c6b5ffaf0be85898960405161124893929190615b22565b60405180910390a350506006805460ff60a01b19169055505050505050565b6006546001600160a01b031633146112915760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b0382166112b75760405162461bcd60e51b81526004016107319061570a565b6001600160a01b03821660009081526003602052604090819020805460ff1916831515179055517ff777a2dd5104082173537070fa5bc14b35ab543cc11a90836e264f3cef55c0649061130d9084908490615511565b60405180910390a15050565b6006546001600160a01b031633146113435760405162461bcd60e51b8152600401610731906157ac565b61271081106113645760405162461bcd60e51b81526004016107319061570a565b600b8190556040517f8b940a95968ad5b511f89b01075446a4fe9f614f2dc5fbb9e9a6b227d6d4fd7090611399908390615b96565b60405180910390a150565b6001600160a01b0381166000908152600160205260409020545b919050565b7f000000000000000000000000000000000000000000000000000000000000000081565b3360009081526003602052604090205460ff166114165760405162461bcd60e51b81526004016107319061575b565b336000818152602081905260409020544310156114455760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208190526040808220436001019055600a549051630b4c774160e11b815282918291829182914291909103908290731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290611511907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561152957600080fd5b505afa15801561153d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115619190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b15801561159957600080fd5b505afa1580156115ad573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115d1919061525b565b505050505050905061271060328a02816115e757fe5b0489826001600160a01b0316031080611611575061271060328a0204816001600160a01b03168a03105b61162d5760405162461bcd60e51b81526004016107319061584e565b60005b6009548110156117e1576009818154811061164757fe5b6000918252602090912001546001600160b81b03166116785760405162461bcd60e51b8152600401610731906157fd565b6009818154811061168557fe5b90600052602060002001600001601d9054906101000a900462ffffff1662ffffff168801975060008073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b031663fc6f78656040518060800160405280600987815481106116e957fe5b600091825260209182902001546001600160b81b0316825230908201526001600160801b036040808301829052606090920152516001600160e01b031960e084901b16815261173b9190600401615941565b6040805180830381600087803b15801561175457600080fd5b505af1158015611768573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061178c919061535c565b91509150818901985080880197506000806117c9600986815481106117ad57fe5b6000918252602090912001546001600160b81b0316878961312f565b99019897909701965050600190920191506116309050565b50600b54612710878202819004808701969288029190910480860195919089881115611824578997508984101561181c57838a039150611824565b899350600091505b8887111561184757889650888310156118405750818803611847565b5087915060005b42600a55871561188b5760065461188b906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811691168a6127f2565b86156118cb576006546118cb906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169116896127f2565b6000806118d78761327f565b915091507f8e9f83b74c783b864e2e6c8c8faff32b21e62ed8ceac8ae83d1da7e0ca1ba8fd8c8c8888888888886040516119189897969594939291906155c9565b60405180910390a1505050505050505050505050505050565b600b5481565b60046020526000908152604090205460ff1681565b6006546001600160a01b031690565b60088054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156106065780601f106105db57610100808354040283529160200191610606565b60606009805480602002602001604051908101604052809291908181526020016000905b82821015611a5457600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff1660608201528252600190920191016119e0565b50505050905090565b60006020819052908152604090205481565b600c5481565b600061061d338484612009565b7f000000000000000000000000000000000000000000000000000000000000000081565b6006546001600160a01b03163314611ad05760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b038216611af65760405162461bcd60e51b81526004016107319061570a565b6001600160a01b03821660009081526004602052604090819020805460ff1916831515179055517f4d5db8249eb7b157e4deefe2c0c726e88aeee890e3dd5a95c68dfe435bf652ff9061130d9084908490615511565b6001600160a01b03918216600090815260026020908152604080832093909416825291909152205490565b33600081815260208190526040902054431015611ba65760405162461bcd60e51b81526004016107319061590b565b6001600160a01b038116600090815260208181526040808320436001019055338352600490915290205460ff16611bef5760405162461bcd60e51b815260040161073190615683565b611bfb60096000614dbc565b60005b8351811015611deb57838181518110611c1357fe5b60200260200101516040015160020b848281518110611c2e57fe5b60200260200101516020015160020b13611c5a5760405162461bcd60e51b815260040161073190615632565b8015611cb757836001820381518110611c6f57fe5b60200260200101516040015160020b848281518110611c8a57fe5b60200260200101516020015160020b1315611cb75760405162461bcd60e51b815260040161073190615632565b6000848281518110611cc557fe5b60200260200101516060015162ffffff1611611cf35760405162461bcd60e51b81526004016107319061564d565b838181518110611cff57fe5b6020026020010151600001516001600160b81b0316600014611d335760405162461bcd60e51b8152600401610731906156b9565b6009848281518110611d4157fe5b60209081029190910181015182546001818101855560009485529383902082519101805493830151604084015160609094015162ffffff908116600160e81b026001600160e81b03600296870b8316600160d01b0262ffffff60d01b199490970b909216600160b81b0262ffffff60b81b196001600160b81b039096166001600160b81b031990981697909717949094169590951716929092179290921691909117905501611bfe565b507fc1c463f316d85c725592f65d9d6f5b014967c835e0b8de363fbc10dcb752dd81600080600080600080600080604051611e2d9897969594939291906155c9565b60405180910390a1505050565b6006546001600160a01b03163314611e645760405162461bcd60e51b8152600401610731906157ac565b6001600160a01b038116611e8a5760405162461bcd60e51b81526004016107319061570a565b600680546001600160a01b0319166001600160a01b0383161790556040517fcfaaa26691e16e66e73290fc725eee1a6b4e0e693a1640484937aac25ffb55a490611399908390615497565b6006546001600160a01b03163314611eff5760405162461bcd60e51b8152600401610731906157ac565b6127108110611f205760405162461bcd60e51b81526004016107319061570a565b600c8190556040517fd87632b1c6ebfa21acbca0e3279b3cf6385a377cb8fda51e5b866baa6e6012ab90611399908390615b96565b6001600160a01b038316611f7b5760405162461bcd60e51b8152600401610731906157c7565b6001600160a01b038216611fa15760405162461bcd60e51b8152600401610731906156ef565b6001600160a01b0380841660008181526002602090815260408083209487168084529490915290819020849055517f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590611ffc908590615b96565b60405180910390a3505050565b6001600160a01b03831661202f5760405162461bcd60e51b815260040161073190615791565b6001600160a01b0382166120555760405162461bcd60e51b815260040161073190615869565b6001600160a01b0383166000908152600160205260409020548181101561208e5760405162461bcd60e51b815260040161073190615776565b6001600160a01b0380851660008181526001602052604080822086860390559286168082529083902080548601905591517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef906120ec908690615b96565b60405180910390a350505050565b60008282111561211c5760405162461bcd60e51b815260040161073190615740565b50900390565b600080600061212f614ddd565b60006009805480602002602001604051908101604052809291908181526020016000905b828210156121c757600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff166060820152825260019092019101612153565b5050600a54604051630b4c774160e11b815293945042039260009250731f98431c8ad98523631ae4a59f267346ea31f9849150631698ee8290612272907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561228a57600080fd5b505afa15801561229e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906122c29190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b1580156122fa57600080fd5b505afa15801561230e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612332919061525b565b505050505050905060005b835181101561270457600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab8886848151811061237757fe5b6020026020010151600001516040518263ffffffff1660e01b815260040161239f9190615b43565b6101806040518083038186803b1580156123b857600080fd5b505afa1580156123cc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123f0919061537f565b505050509750505050505050506000612419826001600160801b03168d60000151600554613080565b905060006040518060a0016040528088868151811061243457fe5b6020026020010151600001516001600160b81b03168152602001836001600160801b0316815260200160008152602001600081526020018e60800151815250905061247d614e13565b604051630624e65f60e11b815273c36442b4a4522e871399cd717abdd847ab11fe8890630c49ccbe906124b4908590600401615984565b6040805180830381600087803b1580156124cd57600080fd5b505af11580156124e1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612505919061535c565b60208301528152612514614e13565b73c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b031663fc6f786560405180608001604052808c8a8151811061254e57fe5b602090810291909101810151516001600160b81b0316825230908201526001600160801b036040808301829052606090920152516001600160e01b031960e084901b1681526125a09190600401615941565b6040805180830381600087803b1580156125b957600080fd5b505af11580156125cd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f1919061535c565b602083015281528151612605908e90613b04565b9c5061261e82602001518d613b0490919063ffffffff16565b9b50838b019a50836001600160801b03168b6001600160801b031610156126575760405162461bcd60e51b81526004016107319061589f565b8d156126f357815181518b5161266f92909103613b04565b8a5260208083015182820151918c015161268c9290919003613b04565b60208b015288516126bf908a90889081106126a357fe5b6020026020010151600001516001600160b81b0316888a61312f565b602084015280835260408b01516126d591613b04565b60408b0152602082015160608b01516126ed91613b04565b60608b01525b50506001909301925061233d915050565b50871561272f57600b5484516127109102046080850152600b54602085015161271091020460a08501525b50505092959194509250565b6001600160a01b0382166127615760405162461bcd60e51b8152600401610731906158f0565b6001600160a01b0382166000908152600160205260409020548181101561279a5760405162461bcd60e51b815260040161073190615833565b6001600160a01b0383166000818152600160205260408082208585039055600580548690039055519091907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90611ffc908690615b96565b6128488363a9059cbb60e01b848460405160240161281192919061552c565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152613b29565b505050565b61286e846323b872dd60e01b858585604051602401612811939291906154ed565b50505050565b60008060008060006009805480602002602001604051908101604052809291908181526020016000905b8282101561291257600084815260209081902060408051608081018252918501546001600160b81b0381168352600160b81b8104600290810b810b810b84860152600160d01b8204810b810b900b91830191909152600160e81b900462ffffff16606082015282526001909201910161289e565b50508851929350612965926001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016925073c36442b4a4522e871399cd717abdd847ab11fe889150613b9c565b60208601516129b4906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169073c36442b4a4522e871399cd717abdd847ab11fe8890613b9c565b6000806000835167ffffffffffffffff811180156129d157600080fd5b506040519080825280602002602001820160405280156129fb578160200160208202803683370190505b5090506000845167ffffffffffffffff81118015612a1857600080fd5b50604051908082528060200260200182016040528015612a42578160200160208202803683370190505b509050612a4e85613c5f565b919a5092965090945090925090508315801590612a6b5750600083115b15612a8d57612a87858b600001518c60200151878787876142c8565b90945092505b60005b8551811015612f5a5760006040518061016001604052807f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031681526020017f000000000000000000000000000000000000000000000000000000000000000062ffffff168152602001888481518110612b3c57fe5b60200260200101516040015160020b8152602001888481518110612b5c57fe5b60200260200101516020015160020b815260200160008152602001600081526020016000815260200160008152602001306001600160a01b031681526020018d60a00151815250905060006040518060c00160405280898581518110612bbe57fe5b6020026020010151600001516001600160b81b03168152602001600081526020016000815260200160008152602001600081526020018e60a0015181525090506000871115612c5a57612c298d60000151868581518110612c1b57fe5b602002602001015189613080565b60a08301819052602082015260408d01518551612c4d9190879086908110612c1b57fe5b60e0830181905260608201525b8515612cb457612c828d60200151858581518110612c7457fe5b602002602001015188613080565b60c08301819052604082015260608d01518451612ca69190869086908110612c7457fe5b610100830181905260808201525b60008260a001511180612ccb575060008260c00151115b15612f5057612cd8614e2d565b888481518110612ce457fe5b6020026020010151600001516001600160b81b031660001415612e4857604051634418b22b60e11b815273c36442b4a4522e871399cd717abdd847ab11fe8890638831645690612d38908690600401615a74565b608060405180830381600087803b158015612d5257600080fd5b505af1925050508015612d82575060408051601f3d908101601f19168201909252612d7f91810190615321565b60015b612d8b57612da9565b9284526001600160801b039091166020840152604083015260608201525b80516009805486908110612db957fe5b600091825260209091200180546001600160b81b0319166001600160b81b03929092169190911790556040810151612df2908e90613b04565b9c50612e0b81606001518d613b0490919063ffffffff16565b6020820151909c50998a01996001600160801b03908116908b161015612e435760405162461bcd60e51b81526004016107319061589f565b612f4e565b60405163219f5d1760e01b815273c36442b4a4522e871399cd717abdd847ab11fe889063219f5d1790612e7f908590600401615a30565b606060405180830381600087803b158015612e9957600080fd5b505af1925050508015612ec9575060408051601f3d908101601f19168201909252612ec691810190615227565b60015b612ed257612eed565b6001600160801b039092166020840152604083015260608201525b6040810151612efd908e90613b04565b9c50612f1681606001518d613b0490919063ffffffff16565b6020820151909c50998a01996001600160801b03908116908b161015612f4e5760405162461bcd60e51b81526004016107319061589f565b505b5050600101612a90565b50612fa46001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673c36442b4a4522e871399cd717abdd847ab11fe886000613b9c565b612fed6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673c36442b4a4522e871399cd717abdd847ab11fe886000613b9c565b50505050509193509193565b6001600160a01b03821661301f5760405162461bcd60e51b815260040161073190615926565b60058054820190556001600160a01b038216600081815260016020526040808220805485019055517fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef90613074908590615b96565b60405180910390a35050565b60008080600019858709868602925082811090839003039050806130b657600084116130ab57600080fd5b508290049050610700565b8084116130c257600080fd5b6000848688096000868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b600080600080600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab88896040518263ffffffff1660e01b81526004016131779190615b96565b6101806040518083038186803b15801561319057600080fd5b505afa1580156131a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131c8919061537f565b50505050975097509750505050505060006131e284614691565b905060006131ef84614691565b90506000806132008b8585886149aa565b915091506127106301e1855861322b8c613225600c5487614a4690919063ffffffff16565b90614a46565b8161323257fe5b048161323a57fe5b0498506127106301e1855861325e8c613225600c5486614a4690919063ffffffff16565b8161326557fe5b048161326d57fe5b04975050505050505050935093915050565b6006546000908190600160a01b900460ff16156132ae5760405162461bcd60e51b815260040161073190615725565b6006805460ff60a01b1916600160a01b1790556040516370a0823160e01b81526000906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190613310903090600401615497565b60206040518083038186803b15801561332857600080fd5b505afa15801561333c573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133609190615309565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166370a08231306040518263ffffffff1660e01b81526004016133b09190615497565b60206040518083038186803b1580156133c857600080fd5b505afa1580156133dc573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906134009190615309565b90506000806134456040518060c001604052808681526020018581526020016000815260200160008152602001306001600160a01b0316815260200142815250612874565b509193509150613457905084836120fa565b935061346383826120fa565b925061346f8683613b04565b955061347b8582613b04565b945060006134898484614a46565b6134938684614a46565b11806134b15750821580156134a6575081155b80156134b157508385115b156136e757821580156134c2575081155b156134d1575060028404613522565b816135016134ed858b6001600160a01b0316600160601b613080565b8a6001600160a01b0316600160601b613080565b0161350c8585614a46565b6135168785614a46565b038161351e57fe5b0490505b61356a6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c0586156483613b9c565b60408051610100810182526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811682527f00000000000000000000000000000000000000000000000000000000000000001660208201527f000000000000000000000000000000000000000000000000000000000000000062ffffff168183015230606082015242608082015260a08101839052600060c0820181905260e0820152905163414bf38960e01b815273e592427a0aece92de3edee1f18e0157c058615649163414bf3899161364991906004016159c7565b602060405180830381600087803b15801561366357600080fd5b505af1925050508015613693575060408051601f3d908101601f1916820190925261369091810190615309565b60015b61369c5761369e565b505b6136e76001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c058615646000613b9c565b6136f18484614a46565b6136fb8684614a46565b108061371957508215801561370e575081155b801561371957508385105b1561394f578215801561372a575081155b1561373957506002830461378a565b8261376961375584600160601b8c6001600160a01b0316613080565b600160601b8b6001600160a01b0316613080565b016137748684614a46565b61377e8686614a46565b038161378657fe5b0490505b6137d26001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c0586156483613b9c565b60408051610100810182526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811682527f00000000000000000000000000000000000000000000000000000000000000001660208201527f000000000000000000000000000000000000000000000000000000000000000062ffffff168183015230606082015242608082015260a08101839052600060c0820181905260e0820152905163414bf38960e01b815273e592427a0aece92de3edee1f18e0157c058615649163414bf389916138b191906004016159c7565b602060405180830381600087803b1580156138cb57600080fd5b505af19250505080156138fb575060408051601f3d908101601f191682019092526138f891810190615309565b60015b61390457613906565b505b61394f6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001673e592427a0aece92de3edee1f18e0157c058615646000613b9c565b6040516370a0823160e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a082319061399b903090600401615497565b60206040518083038186803b1580156139b357600080fd5b505afa1580156139c7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906139eb9190615309565b6040516370a0823160e01b81529095506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a0823190613a3a903090600401615497565b60206040518083038186803b158015613a5257600080fd5b505afa158015613a66573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613a8a9190615309565b9350613acc6040518060c001604052808781526020018681526020016000815260200160008152602001306001600160a01b0316815260200142815250612874565b509194509250613ade90508784613b04565b9650613aea8683613b04565b6006805460ff60a01b191690559698969750505050505050565b6000828201838110156107005760405162461bcd60e51b81526004016107319061589f565b6000613b6282604051806040016040528060018152602001603760f81b815250856001600160a01b0316614a809092919063ffffffff16565b8051909150156128485780806020019051810190613b8091906150f7565b6128485760405162461bcd60e51b8152600401610731906158d5565b801580613c245750604051636eb1769f60e11b81526001600160a01b0384169063dd62ed3e90613bd290309086906004016154ab565b60206040518083038186803b158015613bea57600080fd5b505afa158015613bfe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613c229190615309565b155b613c405760405162461bcd60e51b8152600401610731906158ba565b6128488363095ea7b360e01b848460405160240161281192919061552c565b6000806000606080855167ffffffffffffffff81118015613c7f57600080fd5b50604051908082528060200260200182016040528015613ca9578160200160208202803683370190505b509150855167ffffffffffffffff81118015613cc457600080fd5b50604051908082528060200260200182016040528015613cee578160200160208202803683370190505b50604051630b4c774160e11b81529091506000908190731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290613d92907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b158015613daa57600080fd5b505afa158015613dbe573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613de29190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b158015613e1a57600080fd5b505afa158015613e2e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613e52919061525b565b505050505091509150613e63614e13565b600080613e878b600081518110613e7657fe5b602002602001015160400151614691565b6001600160a01b031683528a51613eb4908c90600090613ea357fe5b602002602001015160200151614691565b6001600160a01b031660208401528a518b90600090613ecf57fe5b60200260200101516060015162ffffff1691508a60018c510381518110613ef257fe5b60200260200101516060015162ffffff16905060005b8b518110156142b95760008c8281518110613f1f57fe5b6020026020010151600001516001600160b81b03161115613ff857600073c36442b4a4522e871399cd717abdd847ab11fe886001600160a01b03166399fbab888e8481518110613f6b57fe5b6020026020010151600001516040518263ffffffff1660e01b8152600401613f939190615b43565b6101806040518083038186803b158015613fac57600080fd5b505afa158015613fc0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613fe4919061537f565b50505050975050505050505050808a019950505b614000614e13565b61400f8d8381518110613e7657fe5b6001600160a01b031681528c5161402c908e9084908110613ea357fe5b6001600160a01b031660208201528c518d908390811061404857fe5b60200260200101516040015160020b8660020b136140fc57838d838151811061406d57fe5b60200260200101516060015162ffffff166140be6140b16140a189600001518a602001518b600001518c6020015103613080565b8551602087015190810390613080565b8451600160601b90613080565b02816140c657fe5b048983815181106140d357fe5b6020026020010181815250508882815181106140eb57fe5b60200260200101518c019b506142b0565b8c828151811061410857fe5b60200260200101516020015160020b8660020b1261419f57828d838151811061412d57fe5b60200260200101516060015162ffffff166141618360000151846020015103600160601b89600001518a6020015103613080565b028161416957fe5b0488838151811061417657fe5b60200260200101818152505087828151811061418e57fe5b60200260200101518b019a506142b0565b838d83815181106141ac57fe5b60200260200101516060015162ffffff166141f76137556141e089600001518a602001518b600001518c6020015103613080565b60208601516001600160a01b038d16810390613080565b02816141ff57fe5b0489838151811061420c57fe5b602002602001018181525050828d838151811061422557fe5b60200260200101516060015162ffffff1661425e83600001518a6001600160a01b031603600160601b89600001518a6020015103613080565b028161426657fe5b0488838151811061427357fe5b60200260200101818152505088828151811061428b57fe5b60200260200101518c019b508782815181106142a357fe5b60200260200101518b019a505b50600101613f08565b50505050505091939590929450565b6000808851600114156142df575084905083614685565b6142e7614e13565b604051630b4c774160e11b8152600090731f98431c8ad98523631ae4a59f267346ea31f98490631698ee8290614385907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000907f0000000000000000000000000000000000000000000000000000000000000000906004016154c5565b60206040518083038186803b15801561439d57600080fd5b505afa1580156143b1573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906143d59190614ef3565b6001600160a01b0316633850c7bd6040518163ffffffff1660e01b815260040160e06040518083038186803b15801561440d57600080fd5b505afa158015614421573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190614445919061525b565b50505050505090506144a6816144618d600081518110613e7657fe5b6144718e600081518110613ea357fe5b6144908e8b60008151811061448257fe5b60200260200101518e613080565b6144a18e8b60008151811061448257fe5b614a97565b6001600160801b031682528a5160001981019061450c9083906144cf908f9085908110613e7657fe5b6144de8f8581518110613ea357fe5b6144fc8f8c87815181106144ee57fe5b60200260200101518f613080565b6144a18f8c88815181106144ee57fe5b6001600160801b031660208401528b518c9060009061452757fe5b60200260200101516060015162ffffff168360200151028c828151811061454a57fe5b60200260200101516060015162ffffff1684600001510211156145f75760008c60008151811061457657fe5b60200260200101516060015162ffffff1684602001510211156145ea576145e3898d83815181106145a357fe5b60200260200101516060015162ffffff168560000151028e6000815181106145c757fe5b60200260200101516060015162ffffff16866020015102613080565b94506145ef565b600094505b879350614681565b88945060008c828151811061460857fe5b60200260200101516060015162ffffff16846000015102111561467c57614675888d60008151811061463657fe5b60200260200101516060015162ffffff168560200151028e848151811061465957fe5b60200260200101516060015162ffffff16866000015102613080565b9350614681565b600093505b5050505b97509795505050505050565b60008060008360020b126146a8578260020b6146b0565b8260020b6000035b9050620d89e88111156146d55760405162461bcd60e51b8152600401610731906157c7565b6000600182166146e957600160801b6146fb565b6ffffcb933bd6fad37aa2d162d1a5940015b70ffffffffffffffffffffffffffffffffff169050600282161561472f576ffff97272373d413259a46990580e213a0260801c5b600482161561474e576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b600882161561476d576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b601082161561478c576fffcb9843d60f6159c9db58835c9266440260801c5b60208216156147ab576fff973b41fa98c081472e6896dfb254c00260801c5b60408216156147ca576fff2ea16466c96a3843ec78b326b528610260801c5b60808216156147e9576ffe5dee046a99a2a811c461f1969c30530260801c5b610100821615614809576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b610200821615614829576ff987a7253ac413176f2b074cf7815e540260801c5b610400821615614849576ff3392b0822b70005940c7a398e4b70f30260801c5b610800821615614869576fe7159475a2c29b7443b29c7fa6e889d90260801c5b611000821615614889576fd097f3bdfd2022b8845ad8f792aa58250260801c5b6120008216156148a9576fa9f746462d870fdf8a65dc1f90e061e50260801c5b6140008216156148c9576f70d869a156d2a1b890bb3df62baf32f70260801c5b6180008216156148e9576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561490a576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b6202000082161561492a576e5d6af8dedb81196699c329225ee6040260801c5b62040000821615614949576d2216e584f5fa1ea926041bedfe980260801c5b62080000821615614966576b048a170391f7dc42444e8fa20260801c5b60008460020b131561498157806000198161497d57fe5b0490505b640100000000810615614995576001614998565b60005b60ff16602082901c0192505050919050565b600080836001600160a01b0316856001600160a01b031611156149cb579293925b846001600160a01b0316866001600160a01b0316116149f6576149ef858585614b5b565b9150614a3d565b836001600160a01b0316866001600160a01b03161015614a2f57614a1b868585614b5b565b9150614a28858785614bc4565b9050614a3d565b614a3a858585614bc4565b90505b94509492505050565b600082614a5557506000610621565b82820282848281614a6257fe5b04146107005760405162461bcd60e51b815260040161073190615818565b6060614a8f8484600085614c07565b949350505050565b6000836001600160a01b0316856001600160a01b03161115614ab7579293925b846001600160a01b0316866001600160a01b031611614ae257614adb858585614cc7565b9050614b52565b836001600160a01b0316866001600160a01b03161015614b44576000614b09878686614cc7565b90506000614b18878986614d2a565b9050806001600160801b0316826001600160801b031610614b395780614b3b565b815b92505050614b52565b614b4f858584614d2a565b90505b95945050505050565b6000826001600160a01b0316846001600160a01b03161115614b7b579192915b836001600160a01b0316614bb4606060ff16846001600160801b0316901b8686036001600160a01b0316866001600160a01b0316613080565b81614bbb57fe5b04949350505050565b6000826001600160a01b0316846001600160a01b03161115614be4579192915b614a8f826001600160801b03168585036001600160a01b0316600160601b613080565b606082471015614c295760405162461bcd60e51b81526004016107319061569e565b614c3285614d67565b614c4e5760405162461bcd60e51b815260040161073190615884565b600080866001600160a01b03168587604051614c6a919061547b565b60006040518083038185875af1925050503d8060008114614ca7576040519150601f19603f3d011682016040523d82523d6000602084013e614cac565b606091505b5091509150614cbc828286614d6d565b979650505050505050565b6000826001600160a01b0316846001600160a01b03161115614ce7579192915b6000614d0a856001600160a01b0316856001600160a01b0316600160601b613080565b9050614b52614d2584838888036001600160a01b0316613080565b614da6565b6000826001600160a01b0316846001600160a01b03161115614d4a579192915b614a8f614d2583600160601b8787036001600160a01b0316613080565b3b151590565b60608315614d7c575081610700565b825115614d8c5782518084602001fd5b8160405162461bcd60e51b815260040161073191906155ff565b806001600160801b03811681146113be57600080fd5b5080546000825590600052602060002090810190614dda9190614e5e565b50565b6040518060c001604052806000815260200160008152602001600081526020016000815260200160008152602001600081525090565b604051806040016040528060008152602001600081525090565b60405180608001604052806000815260200160006001600160801b0316815260200160008152602001600081525090565b5b80821115614e735760008155600101614e5f565b5090565b80516113be81615bfd565b80356113be81615c20565b80516113be81615c20565b80516001600160801b03811681146113be57600080fd5b805161ffff811681146113be57600080fd5b80356113be81615c2f565b80516113be81615c2f565b600060208284031215614ee8578081fd5b813561070081615bfd565b600060208284031215614f04578081fd5b815161070081615bfd565b60008060408385031215614f21578081fd5b8235614f2c81615bfd565b91506020830135614f3c81615bfd565b809150509250929050565b600080600060608486031215614f5b578081fd5b8335614f6681615bfd565b92506020840135614f7681615bfd565b929592945050506040919091013590565b60008060408385031215614f99578182fd5b8235614fa481615bfd565b91506020830135614f3c81615c12565b60008060408385031215614fc6578182fd5b8235614fd181615bfd565b946020939093013593505050565b6000806040808486031215614ff2578283fd5b833567ffffffffffffffff80821115615009578485fd5b818601915086601f83011261501c578485fd5b813560208282111561502a57fe5b6150378182840201615bad565b828152818101908583016080808602880185018d1015615055578a8bfd5b8a97505b858810156150e35780828e03121561506f578a8bfd5b8851818101818110898211171561508257fe5b8a5282356001600160b81b038116811461509a578c8dfd5b8152828601356150a981615c20565b818701526150b8838b01614e82565b8a82015260606150c9818501614ec1565b908201528452600197909701969284019290810190615059565b50909b999092013599505050505050505050565b600060208284031215615108578081fd5b815161070081615c12565b600060c08284031215615124578081fd5b50919050565b600060c0828403121561513b578081fd5b60405160c0810181811067ffffffffffffffff8211171561515857fe5b806040525082358152602083013560208201526040830135604082015260608301356060820152608083013561518d81615bfd565b608082015260a0928301359281019290925250919050565b600060a08284031215615124578081fd5b600060a082840312156151c7578081fd5b60405160a0810181811067ffffffffffffffff821117156151e457fe5b8060405250823581526020830135602082015260408301356040820152606083013561520f81615bfd565b60608201526080928301359281019290925250919050565b60008060006060848603121561523b578081fd5b61524484614e98565b925060208401519150604084015190509250925092565b600080600080600080600060e0888a031215615275578485fd5b875161528081615bfd565b602089015190975061529181615c20565b955061529f60408901614eaf565b94506152ad60608901614eaf565b93506152bb60808901614eaf565b925060a088015160ff811681146152d0578283fd5b60c08901519092506152e181615c12565b8091505092959891949750929550565b600060208284031215615302578081fd5b5035919050565b60006020828403121561531a578081fd5b5051919050565b60008060008060808587031215615336578182fd5b8451935061534660208601614e98565b6040860151606090960151949790965092505050565b6000806040838503121561536e578182fd5b505080516020909101519092909150565b6000806000806000806000806000806000806101808d8f0312156153a1578586fd5b8c516bffffffffffffffffffffffff811681146153bc578687fd5b9b506153ca60208e01614e77565b9a506153d860408e01614e77565b99506153e660608e01614e77565b98506153f460808e01614ecc565b975061540260a08e01614e8d565b965061541060c08e01614e8d565b955061541e60e08e01614e98565b94506101008d015193506101208d0151925061543d6101408e01614e98565b915061544c6101608e01614e98565b90509295989b509295989b509295989b565b6001600160a01b03169052565b60020b9052565b62ffffff169052565b6000825161548d818460208701615bd1565b9190910192915050565b6001600160a01b0391909116815260200190565b6001600160a01b0392831681529116602082015260400190565b6001600160a01b03938416815291909216602082015262ffffff909116604082015260600190565b6001600160a01b039384168152919092166020820152604081019190915260600190565b6001600160a01b039290921682521515602082015260400190565b6001600160a01b03929092168252602082015260400190565b602080825282518282018190526000919060409081850190868401855b828110156155b157815180516001600160b81b0316855286810151600290810b8887015286820151900b8686015260609081015162ffffff169085015260809093019290850190600101615562565b5091979650505050505050565b901515815260200190565b978852602088019690965260408701949094526060860192909252608085015260a084015260c083015260e08201526101000190565b600060208252825180602084015261561e816040850160208701615bd1565b601f01601f19169190910160400192915050565b6020808252600190820152604360f81b604082015260600190565b6020808252600190820152604160f81b604082015260600190565b6020808252600190820152604760f81b604082015260600190565b6020808252600190820152606360f81b604082015260600190565b6020808252600190820152600d60fa1b604082015260600190565b6020808252600190820152602160f91b604082015260600190565b6020808252600190820152600960fb1b604082015260600190565b6020808252600190820152605560f81b604082015260600190565b6020808252600190820152606160f81b604082015260600190565b6020808252600190820152604560f81b604082015260600190565b6020808252600190820152600b60fb1b604082015260600190565b6020808252600190820152601160fa1b604082015260600190565b6020808252600190820152600560fc1b604082015260600190565b6020808252600190820152602760f91b604082015260600190565b6020808252600190820152604d60f81b604082015260600190565b6020808252600190820152601560fa1b604082015260600190565b6020808252600190820152602560f91b604082015260600190565b6020808252600190820152604b60f81b604082015260600190565b6020808252600190820152605960f81b604082015260600190565b6020808252600190820152605360f81b604082015260600190565b6020808252600190820152603160f91b604082015260600190565b6020808252600190820152604f60f81b604082015260600190565b6020808252600190820152603560f81b604082015260600190565b6020808252600190820152605760f81b604082015260600190565b6020808252600190820152601b60f91b604082015260600190565b6020808252600190820152600760fb1b604082015260600190565b6020808252600190820152602960f91b604082015260600190565b6020808252600190820152602b60f91b604082015260600190565b6020808252600190820152605160f81b604082015260600190565b815181526020808301516001600160a01b0316908201526040808301516001600160801b0390811691830191909152606092830151169181019190915260800190565b600060a082019050825182526001600160801b03602084015116602083015260408301516040830152606083015160608301526080830151608083015292915050565b81516001600160a01b03908116825260208084015182169083015260408084015162ffffff16908301526060808401518216908301526080808401519083015260a0838101519083015260c0808401519083015260e09283015116918101919091526101000190565b600060c082019050825182526020830151602083015260408301516040830152606083015160608301526080830151608083015260a083015160a083015292915050565b600061016082019050615a8882845161545e565b6020830151615a9a602084018261545e565b506040830151615aad6040840182615472565b506060830151615ac0606084018261546b565b506080830151615ad3608084018261546b565b5060a083015160a083015260c083015160c083015260e083015160e083015261010080840151818401525061012080840151615b118285018261545e565b505061014092830151919092015290565b6001600160801b039390931683526020830191909152604082015260600190565b6001600160b81b0391909116815260200190565b6001600160b81b03949094168452600292830b6020850152910b604083015262ffffff16606082015260800190565b62ffffff91909116815260200190565b90815260200190565b60ff91909116815260200190565b60405181810167ffffffffffffffff81118282101715615bc957fe5b604052919050565b60005b83811015615bec578181015183820152602001615bd4565b8381111561286e5750506000910152565b6001600160a01b0381168114614dda57600080fd5b8015158114614dda57600080fd5b8060020b8114614dda57600080fd5b62ffffff81168114614dda57600080fdfe000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2a26469706673582212206c2235d8d0c2c276add461f3a1d1360311c2fb3f3f8371fb8849df89ae84094a64736f6c63430007060033\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV3Cellar<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV3Cellar<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV3Cellar<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3Cellar))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV3Cellar<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                UNISWAPV3CELLAR_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `addLiquidityForUniV3` (0x33bc230a) function"]
        pub fn add_liquidity_for_uni_v3(
            &self,
            cellar_params: CellarAddParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 188, 35, 10], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `adjuster` (0x8c231af9) function"]
        pub fn adjuster(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([140, 35, 26, 249], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cellarTickInfo` (0x15723866) function"]
        pub fn cellar_tick_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, i32, i32, u32)>
        {
            self.0
                .method_hash([21, 114, 56, 102], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeLevel` (0x7cf134cb) function"]
        pub fn fee_level(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([124, 241, 52, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCellarTickInfo` (0x96d01a7d) function"]
        pub fn get_cellar_tick_info(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<CellarTickInfo>> {
            self.0
                .method_hash([150, 208, 26, 125], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastLockedBlock` (0x9f3e8b34) function"]
        pub fn last_locked_block(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 62, 139, 52], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `managementFee` (0xa6f7f5d6) function"]
        pub fn management_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([166, 247, 245, 214], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `performanceFee` (0x87788782) function"]
        pub fn performance_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([135, 120, 135, 130], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalance` (0xe1b5c312) function"]
        pub fn rebalance(
            &self,
            cellar_tick_info: ::std::vec::Vec<CellarTickInfo>,
            current_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 181, 195, 18], (cellar_tick_info, current_price_x96))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reinvest` (0x83b4918b) function"]
        pub fn reinvest(
            &self,
            current_price_x96: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 180, 145, 139], current_price_x96)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityFromUniV3` (0x276cd920) function"]
        pub fn remove_liquidity_from_uni_v3(
            &self,
            cellar_params: CellarRemoveParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 108, 217, 32], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAdjuster` (0xdb5d820e) function"]
        pub fn set_adjuster(
            &self,
            adjuster: ethers::core::types::Address,
            value: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 93, 130, 14], (adjuster, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setManagementFee` (0xfe56e232) function"]
        pub fn set_management_fee(
            &self,
            new_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 86, 226, 50], new_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPerformanceFee` (0x70897b23) function"]
        pub fn set_performance_fee(
            &self,
            new_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 137, 123, 35], new_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setValidator` (0x4623c91d) function"]
        pub fn set_validator(
            &self,
            validator: ethers::core::types::Address,
            value: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 35, 201, 29], (validator, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(
            &self,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validator` (0x223b3b7a) function"]
        pub fn validator(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 59, 59, 122], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddedLiquidity` event"]
        pub fn added_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddedLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Rebalance` event"]
        pub fn rebalance_filter(&self) -> ethers::contract::builders::Event<M, RebalanceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Reinvest` event"]
        pub fn reinvest_filter(&self) -> ethers::contract::builders::Event<M, ReinvestFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemovedLiquidity` event"]
        pub fn removed_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemovedLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetAdjuster` event"]
        pub fn set_adjuster_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetAdjusterFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetManagementFee` event"]
        pub fn set_management_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetManagementFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetPerformanceFee` event"]
        pub fn set_performance_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetPerformanceFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetValidator` event"]
        pub fn set_validator_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetValidatorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferOwnership` event"]
        pub fn transfer_ownership_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferOwnershipFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UniswapV3CellarEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "AddedLiquidity",
        abi = "AddedLiquidity(address,address,uint128,uint256,uint256)"
    )]
    pub struct AddedLiquidityFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Rebalance",
        abi = "Rebalance(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct RebalanceFilter {
        pub fees_0: ethers::core::types::U256,
        pub fees_1: ethers::core::types::U256,
        pub management_fee_0: ethers::core::types::U256,
        pub management_fee_1: ethers::core::types::U256,
        pub performance_fee_0: ethers::core::types::U256,
        pub performance_fee_1: ethers::core::types::U256,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "Reinvest",
        abi = "Reinvest(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ReinvestFilter {
        pub fees_0: ethers::core::types::U256,
        pub fees_1: ethers::core::types::U256,
        pub management_fee_0: ethers::core::types::U256,
        pub management_fee_1: ethers::core::types::U256,
        pub performance_fee_0: ethers::core::types::U256,
        pub performance_fee_1: ethers::core::types::U256,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(
        name = "RemovedLiquidity",
        abi = "RemovedLiquidity(address,address,uint128,uint256,uint256)"
    )]
    pub struct RemovedLiquidityFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "SetAdjuster", abi = "SetAdjuster(address,bool)")]
    pub struct SetAdjusterFilter {
        pub adjuster: ethers::core::types::Address,
        pub value: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "SetManagementFee", abi = "SetManagementFee(uint256)")]
    pub struct SetManagementFeeFilter {
        pub new_fee: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "SetPerformanceFee", abi = "SetPerformanceFee(uint256)")]
    pub struct SetPerformanceFeeFilter {
        pub new_fee: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "SetValidator", abi = "SetValidator(address,bool)")]
    pub struct SetValidatorFilter {
        pub validator: ethers::core::types::Address,
        pub value: bool,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethevent(name = "TransferOwnership", abi = "TransferOwnership(address)")]
    pub struct TransferOwnershipFilter {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3CellarEvents {
        AddedLiquidityFilter(AddedLiquidityFilter),
        ApprovalFilter(ApprovalFilter),
        RebalanceFilter(RebalanceFilter),
        ReinvestFilter(ReinvestFilter),
        RemovedLiquidityFilter(RemovedLiquidityFilter),
        SetAdjusterFilter(SetAdjusterFilter),
        SetManagementFeeFilter(SetManagementFeeFilter),
        SetPerformanceFeeFilter(SetPerformanceFeeFilter),
        SetValidatorFilter(SetValidatorFilter),
        TransferFilter(TransferFilter),
        TransferOwnershipFilter(TransferOwnershipFilter),
    }
    impl ethers::contract::EthLogDecode for UniswapV3CellarEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddedLiquidityFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::AddedLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = ReinvestFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::ReinvestFilter(decoded));
            }
            if let Ok(decoded) = RemovedLiquidityFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::RemovedLiquidityFilter(decoded));
            }
            if let Ok(decoded) = SetAdjusterFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::SetAdjusterFilter(decoded));
            }
            if let Ok(decoded) = SetManagementFeeFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::SetManagementFeeFilter(decoded));
            }
            if let Ok(decoded) = SetPerformanceFeeFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::SetPerformanceFeeFilter(decoded));
            }
            if let Ok(decoded) = SetValidatorFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::SetValidatorFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferOwnershipFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::TransferOwnershipFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniswapV3CellarEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3CellarEvents::AddedLiquidityFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::ApprovalFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::RebalanceFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::ReinvestFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::RemovedLiquidityFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::SetAdjusterFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::SetManagementFeeFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::SetPerformanceFeeFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::SetValidatorFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::TransferFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::TransferOwnershipFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `addLiquidityForUniV3`function with signature `addLiquidityForUniV3((uint256,uint256,uint256,uint256,address,uint256))` and selector `[51, 188, 35, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "addLiquidityForUniV3",
        abi = "addLiquidityForUniV3((uint256,uint256,uint256,uint256,address,uint256))"
    )]
    pub struct AddLiquidityForUniV3Call {
        pub cellar_params: CellarAddParams,
    }
    #[doc = "Container type for all input parameters for the `adjuster`function with signature `adjuster(address)` and selector `[140, 35, 26, 249]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "adjuster", abi = "adjuster(address)")]
    pub struct AdjusterCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `allowance`function with signature `allowance(address,address)` and selector `[221, 98, 237, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `approve`function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cellarTickInfo`function with signature `cellarTickInfo(uint256)` and selector `[21, 114, 56, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "cellarTickInfo", abi = "cellarTickInfo(uint256)")]
    pub struct CellarTickInfoCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `decimals`function with signature `decimals()` and selector `[49, 60, 229, 103]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    #[doc = "Container type for all input parameters for the `feeLevel`function with signature `feeLevel()` and selector `[124, 241, 52, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "feeLevel", abi = "feeLevel()")]
    pub struct FeeLevelCall;
    #[doc = "Container type for all input parameters for the `getCellarTickInfo`function with signature `getCellarTickInfo()` and selector `[150, 208, 26, 125]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getCellarTickInfo", abi = "getCellarTickInfo()")]
    pub struct GetCellarTickInfoCall;
    #[doc = "Container type for all input parameters for the `lastLockedBlock`function with signature `lastLockedBlock(address)` and selector `[159, 62, 139, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "lastLockedBlock", abi = "lastLockedBlock(address)")]
    pub struct LastLockedBlockCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `managementFee`function with signature `managementFee()` and selector `[166, 247, 245, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "managementFee", abi = "managementFee()")]
    pub struct ManagementFeeCall;
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `owner`function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `performanceFee`function with signature `performanceFee()` and selector `[135, 120, 135, 130]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "performanceFee", abi = "performanceFee()")]
    pub struct PerformanceFeeCall;
    #[doc = "Container type for all input parameters for the `rebalance`function with signature `rebalance((uint184,int24,int24,uint24)[],uint256)` and selector `[225, 181, 195, 18]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "rebalance",
        abi = "rebalance((uint184,int24,int24,uint24)[],uint256)"
    )]
    pub struct RebalanceCall {
        pub cellar_tick_info: ::std::vec::Vec<CellarTickInfo>,
        pub current_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `reinvest`function with signature `reinvest(uint256)` and selector `[131, 180, 145, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "reinvest", abi = "reinvest(uint256)")]
    pub struct ReinvestCall {
        pub current_price_x96: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityFromUniV3`function with signature `removeLiquidityFromUniV3((uint256,uint256,uint256,address,uint256))` and selector `[39, 108, 217, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(
        name = "removeLiquidityFromUniV3",
        abi = "removeLiquidityFromUniV3((uint256,uint256,uint256,address,uint256))"
    )]
    pub struct RemoveLiquidityFromUniV3Call {
        pub cellar_params: CellarRemoveParams,
    }
    #[doc = "Container type for all input parameters for the `setAdjuster`function with signature `setAdjuster(address,bool)` and selector `[219, 93, 130, 14]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setAdjuster", abi = "setAdjuster(address,bool)")]
    pub struct SetAdjusterCall {
        pub adjuster: ethers::core::types::Address,
        pub value: bool,
    }
    #[doc = "Container type for all input parameters for the `setManagementFee`function with signature `setManagementFee(uint256)` and selector `[254, 86, 226, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setManagementFee", abi = "setManagementFee(uint256)")]
    pub struct SetManagementFeeCall {
        pub new_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setPerformanceFee`function with signature `setPerformanceFee(uint256)` and selector `[112, 137, 123, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setPerformanceFee", abi = "setPerformanceFee(uint256)")]
    pub struct SetPerformanceFeeCall {
        pub new_fee: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setValidator`function with signature `setValidator(address,bool)` and selector `[70, 35, 201, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setValidator", abi = "setValidator(address,bool)")]
    pub struct SetValidatorCall {
        pub validator: ethers::core::types::Address,
        pub value: bool,
    }
    #[doc = "Container type for all input parameters for the `symbol`function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
    #[doc = "Container type for all input parameters for the `totalSupply`function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transfer`function with signature `transfer(address,uint256)` and selector `[169, 5, 156, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFrom`function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `validator`function with signature `validator(address)` and selector `[34, 59, 59, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "validator", abi = "validator(address)")]
    pub struct ValidatorCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3CellarCalls {
        AddLiquidityForUniV3(AddLiquidityForUniV3Call),
        Adjuster(AdjusterCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CellarTickInfo(CellarTickInfoCall),
        Decimals(DecimalsCall),
        FeeLevel(FeeLevelCall),
        GetCellarTickInfo(GetCellarTickInfoCall),
        LastLockedBlock(LastLockedBlockCall),
        ManagementFee(ManagementFeeCall),
        Name(NameCall),
        Owner(OwnerCall),
        PerformanceFee(PerformanceFeeCall),
        Rebalance(RebalanceCall),
        Reinvest(ReinvestCall),
        RemoveLiquidityFromUniV3(RemoveLiquidityFromUniV3Call),
        SetAdjuster(SetAdjusterCall),
        SetManagementFee(SetManagementFeeCall),
        SetPerformanceFee(SetPerformanceFeeCall),
        SetValidator(SetValidatorCall),
        Symbol(SymbolCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Validator(ValidatorCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV3CellarCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityForUniV3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::AddLiquidityForUniV3(decoded));
            }
            if let Ok(decoded) =
                <AdjusterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Adjuster(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CellarTickInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::CellarTickInfo(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <FeeLevelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::FeeLevel(decoded));
            }
            if let Ok(decoded) =
                <GetCellarTickInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::GetCellarTickInfo(decoded));
            }
            if let Ok(decoded) =
                <LastLockedBlockCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::LastLockedBlock(decoded));
            }
            if let Ok(decoded) =
                <ManagementFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::ManagementFee(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3CellarCalls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::PerformanceFee(decoded));
            }
            if let Ok(decoded) =
                <RebalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Rebalance(decoded));
            }
            if let Ok(decoded) =
                <ReinvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Reinvest(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityFromUniV3Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3CellarCalls::RemoveLiquidityFromUniV3(decoded));
            }
            if let Ok(decoded) =
                <SetAdjusterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetAdjuster(decoded));
            }
            if let Ok(decoded) =
                <SetManagementFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetManagementFee(decoded));
            }
            if let Ok(decoded) =
                <SetPerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetPerformanceFee(decoded));
            }
            if let Ok(decoded) =
                <SetValidatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetValidator(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Token1(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ValidatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Validator(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV3CellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3CellarCalls::AddLiquidityForUniV3(element) => element.encode(),
                UniswapV3CellarCalls::Adjuster(element) => element.encode(),
                UniswapV3CellarCalls::Allowance(element) => element.encode(),
                UniswapV3CellarCalls::Approve(element) => element.encode(),
                UniswapV3CellarCalls::BalanceOf(element) => element.encode(),
                UniswapV3CellarCalls::CellarTickInfo(element) => element.encode(),
                UniswapV3CellarCalls::Decimals(element) => element.encode(),
                UniswapV3CellarCalls::FeeLevel(element) => element.encode(),
                UniswapV3CellarCalls::GetCellarTickInfo(element) => element.encode(),
                UniswapV3CellarCalls::LastLockedBlock(element) => element.encode(),
                UniswapV3CellarCalls::ManagementFee(element) => element.encode(),
                UniswapV3CellarCalls::Name(element) => element.encode(),
                UniswapV3CellarCalls::Owner(element) => element.encode(),
                UniswapV3CellarCalls::PerformanceFee(element) => element.encode(),
                UniswapV3CellarCalls::Rebalance(element) => element.encode(),
                UniswapV3CellarCalls::Reinvest(element) => element.encode(),
                UniswapV3CellarCalls::RemoveLiquidityFromUniV3(element) => element.encode(),
                UniswapV3CellarCalls::SetAdjuster(element) => element.encode(),
                UniswapV3CellarCalls::SetManagementFee(element) => element.encode(),
                UniswapV3CellarCalls::SetPerformanceFee(element) => element.encode(),
                UniswapV3CellarCalls::SetValidator(element) => element.encode(),
                UniswapV3CellarCalls::Symbol(element) => element.encode(),
                UniswapV3CellarCalls::Token0(element) => element.encode(),
                UniswapV3CellarCalls::Token1(element) => element.encode(),
                UniswapV3CellarCalls::TotalSupply(element) => element.encode(),
                UniswapV3CellarCalls::Transfer(element) => element.encode(),
                UniswapV3CellarCalls::TransferFrom(element) => element.encode(),
                UniswapV3CellarCalls::TransferOwnership(element) => element.encode(),
                UniswapV3CellarCalls::Validator(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3CellarCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3CellarCalls::AddLiquidityForUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::Adjuster(element) => element.fmt(f),
                UniswapV3CellarCalls::Allowance(element) => element.fmt(f),
                UniswapV3CellarCalls::Approve(element) => element.fmt(f),
                UniswapV3CellarCalls::BalanceOf(element) => element.fmt(f),
                UniswapV3CellarCalls::CellarTickInfo(element) => element.fmt(f),
                UniswapV3CellarCalls::Decimals(element) => element.fmt(f),
                UniswapV3CellarCalls::FeeLevel(element) => element.fmt(f),
                UniswapV3CellarCalls::GetCellarTickInfo(element) => element.fmt(f),
                UniswapV3CellarCalls::LastLockedBlock(element) => element.fmt(f),
                UniswapV3CellarCalls::ManagementFee(element) => element.fmt(f),
                UniswapV3CellarCalls::Name(element) => element.fmt(f),
                UniswapV3CellarCalls::Owner(element) => element.fmt(f),
                UniswapV3CellarCalls::PerformanceFee(element) => element.fmt(f),
                UniswapV3CellarCalls::Rebalance(element) => element.fmt(f),
                UniswapV3CellarCalls::Reinvest(element) => element.fmt(f),
                UniswapV3CellarCalls::RemoveLiquidityFromUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::SetAdjuster(element) => element.fmt(f),
                UniswapV3CellarCalls::SetManagementFee(element) => element.fmt(f),
                UniswapV3CellarCalls::SetPerformanceFee(element) => element.fmt(f),
                UniswapV3CellarCalls::SetValidator(element) => element.fmt(f),
                UniswapV3CellarCalls::Symbol(element) => element.fmt(f),
                UniswapV3CellarCalls::Token0(element) => element.fmt(f),
                UniswapV3CellarCalls::Token1(element) => element.fmt(f),
                UniswapV3CellarCalls::TotalSupply(element) => element.fmt(f),
                UniswapV3CellarCalls::Transfer(element) => element.fmt(f),
                UniswapV3CellarCalls::TransferFrom(element) => element.fmt(f),
                UniswapV3CellarCalls::TransferOwnership(element) => element.fmt(f),
                UniswapV3CellarCalls::Validator(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityForUniV3Call> for UniswapV3CellarCalls {
        fn from(var: AddLiquidityForUniV3Call) -> Self {
            UniswapV3CellarCalls::AddLiquidityForUniV3(var)
        }
    }
    impl ::std::convert::From<AdjusterCall> for UniswapV3CellarCalls {
        fn from(var: AdjusterCall) -> Self {
            UniswapV3CellarCalls::Adjuster(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for UniswapV3CellarCalls {
        fn from(var: AllowanceCall) -> Self {
            UniswapV3CellarCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for UniswapV3CellarCalls {
        fn from(var: ApproveCall) -> Self {
            UniswapV3CellarCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for UniswapV3CellarCalls {
        fn from(var: BalanceOfCall) -> Self {
            UniswapV3CellarCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CellarTickInfoCall> for UniswapV3CellarCalls {
        fn from(var: CellarTickInfoCall) -> Self {
            UniswapV3CellarCalls::CellarTickInfo(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for UniswapV3CellarCalls {
        fn from(var: DecimalsCall) -> Self {
            UniswapV3CellarCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FeeLevelCall> for UniswapV3CellarCalls {
        fn from(var: FeeLevelCall) -> Self {
            UniswapV3CellarCalls::FeeLevel(var)
        }
    }
    impl ::std::convert::From<GetCellarTickInfoCall> for UniswapV3CellarCalls {
        fn from(var: GetCellarTickInfoCall) -> Self {
            UniswapV3CellarCalls::GetCellarTickInfo(var)
        }
    }
    impl ::std::convert::From<LastLockedBlockCall> for UniswapV3CellarCalls {
        fn from(var: LastLockedBlockCall) -> Self {
            UniswapV3CellarCalls::LastLockedBlock(var)
        }
    }
    impl ::std::convert::From<ManagementFeeCall> for UniswapV3CellarCalls {
        fn from(var: ManagementFeeCall) -> Self {
            UniswapV3CellarCalls::ManagementFee(var)
        }
    }
    impl ::std::convert::From<NameCall> for UniswapV3CellarCalls {
        fn from(var: NameCall) -> Self {
            UniswapV3CellarCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for UniswapV3CellarCalls {
        fn from(var: OwnerCall) -> Self {
            UniswapV3CellarCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PerformanceFeeCall> for UniswapV3CellarCalls {
        fn from(var: PerformanceFeeCall) -> Self {
            UniswapV3CellarCalls::PerformanceFee(var)
        }
    }
    impl ::std::convert::From<RebalanceCall> for UniswapV3CellarCalls {
        fn from(var: RebalanceCall) -> Self {
            UniswapV3CellarCalls::Rebalance(var)
        }
    }
    impl ::std::convert::From<ReinvestCall> for UniswapV3CellarCalls {
        fn from(var: ReinvestCall) -> Self {
            UniswapV3CellarCalls::Reinvest(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityFromUniV3Call> for UniswapV3CellarCalls {
        fn from(var: RemoveLiquidityFromUniV3Call) -> Self {
            UniswapV3CellarCalls::RemoveLiquidityFromUniV3(var)
        }
    }
    impl ::std::convert::From<SetAdjusterCall> for UniswapV3CellarCalls {
        fn from(var: SetAdjusterCall) -> Self {
            UniswapV3CellarCalls::SetAdjuster(var)
        }
    }
    impl ::std::convert::From<SetManagementFeeCall> for UniswapV3CellarCalls {
        fn from(var: SetManagementFeeCall) -> Self {
            UniswapV3CellarCalls::SetManagementFee(var)
        }
    }
    impl ::std::convert::From<SetPerformanceFeeCall> for UniswapV3CellarCalls {
        fn from(var: SetPerformanceFeeCall) -> Self {
            UniswapV3CellarCalls::SetPerformanceFee(var)
        }
    }
    impl ::std::convert::From<SetValidatorCall> for UniswapV3CellarCalls {
        fn from(var: SetValidatorCall) -> Self {
            UniswapV3CellarCalls::SetValidator(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for UniswapV3CellarCalls {
        fn from(var: SymbolCall) -> Self {
            UniswapV3CellarCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<Token0Call> for UniswapV3CellarCalls {
        fn from(var: Token0Call) -> Self {
            UniswapV3CellarCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for UniswapV3CellarCalls {
        fn from(var: Token1Call) -> Self {
            UniswapV3CellarCalls::Token1(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for UniswapV3CellarCalls {
        fn from(var: TotalSupplyCall) -> Self {
            UniswapV3CellarCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for UniswapV3CellarCalls {
        fn from(var: TransferCall) -> Self {
            UniswapV3CellarCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for UniswapV3CellarCalls {
        fn from(var: TransferFromCall) -> Self {
            UniswapV3CellarCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for UniswapV3CellarCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            UniswapV3CellarCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<ValidatorCall> for UniswapV3CellarCalls {
        fn from(var: ValidatorCall) -> Self {
            UniswapV3CellarCalls::Validator(var)
        }
    }
    #[doc = "`CellarAddParams(uint256,uint256,uint256,uint256,address,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct CellarAddParams {
        pub amount_0_desired: ethers::core::types::U256,
        pub amount_1_desired: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "`CellarRemoveParams(uint256,uint256,uint256,address,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct CellarRemoveParams {
        pub token_amount: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "`CellarTickInfo(uint184,int24,int24,uint24)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    pub struct CellarTickInfo {
        pub token_id: ethers::core::types::U256,
        pub tick_upper: i32,
        pub tick_lower: i32,
        pub weight: u32,
    }
}
