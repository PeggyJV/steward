pub use aavev2stablecoincellar_mod::*;
#[allow(clippy::too_many_arguments)]
mod aavev2stablecoincellar_mod {
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
    #[doc = "AaveV2StablecoinCellar was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVEV2STABLECOINCELLAR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"contract ISwapRouter\",\r\n                \"name\": \"_uniswapRouter\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"contract ISushiSwapRouter\",\r\n                \"name\": \"_sushiSwapRouter\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"contract ILendingPool\",\r\n                \"name\": \"_lendingPool\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"contract IAaveIncentivesController\",\r\n                \"name\": \"_incentivesController\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"contract Gravity\",\r\n                \"name\": \"_gravityBridge\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"contract IStakedTokenV2\",\r\n                \"name\": \"_stkAAVE\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"_AAVE\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"_WETH\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"_USDC\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"_currentLendingToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"constructor\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"AlreadyShutdown\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"ContractPaused\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"ContractShutdown\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"currentDeposit\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"maxDeposit\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"DepositRestricted\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"currentLiquidity\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"maxLiquidity\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"LiquidityRestricted\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"protectedToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"ProtectedToken\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"lendingToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"SameLendingToken\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"unsupportedToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"TokenIsNotSupportedByAave\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"unapprovedToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"UnapprovedToken\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"ZeroAssets\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"ZeroShares\",\r\n        \"type\": \"error\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"owner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Approval\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"caller\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"owner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Deposit\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"DepositToAave\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [],\r\n        \"name\": \"LiquidityRestrictionRemoved\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"previousOwner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"newOwner\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"OwnershipTransferred\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"caller\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"isPaused\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"Pause\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Rebalance\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"RedeemFromAave\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"isApproved\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"SetInputToken\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"caller\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"Shutdown\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"tokenIn\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountIn\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"tokenOut\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountOut\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Swapped\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Sweep\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"from\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Transfer\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"feeInShares\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"feeInAssets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"TransferFees\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"caller\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"receiver\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"owner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Withdraw\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"AAVE\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"DENOMINATOR\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"DOMAIN_SEPARATOR\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"\",\r\n                \"type\": \"bytes32\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"PERFORMANCE_FEE\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"PERMIT_TYPEHASH\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"\",\r\n                \"type\": \"bytes32\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"PLATFORM_FEE\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"POOL_FEE\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint24\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint24\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"SECS_PER_YEAR\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"USDC\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"WETH\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"accruePlatformFees\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"accruedPerformanceFees\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"accruedPlatformFees\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"activeAssets\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"allowance\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"approve\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"balanceOf\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"claimAndUnstake\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"claimAndUnstake\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"claimed\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"convertToAssets\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"convertToShares\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"currentAToken\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"currentLendingToken\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"decimals\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint8\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint8\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"receiver\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"deposit\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"minAssetsIn\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"receiver\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"deposit\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"deposit\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"enterStrategy\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"feesDistributor\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"\",\r\n                \"type\": \"bytes32\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"gravityBridge\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract Gravity\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"inactiveAssets\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"incentivesController\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract IAaveIncentivesController\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"inputTokens\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"isPaused\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"isShutdown\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"lastTimeAccruedPlatformFees\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"lastTimeEnteredStrategy\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"lendingPool\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract ILendingPool\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"maxDeposit\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"maxLiquidity\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address[]\",\r\n                \"name\": \"path\",\r\n                \"type\": \"address[]\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountIn\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountOutMinimum\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"multihopSwap\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"name\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"string\",\r\n                \"name\": \"\",\r\n                \"type\": \"string\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"nonces\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"owner\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"owner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"value\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"deadline\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint8\",\r\n                \"name\": \"v\",\r\n                \"type\": \"uint8\"\r\n            },\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"r\",\r\n                \"type\": \"bytes32\"\r\n            },\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"s\",\r\n                \"type\": \"bytes32\"\r\n            }\r\n        ],\r\n        \"name\": \"permit\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"newLendingToken\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"minNewLendingTokenAmount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"rebalance\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"minAssetsOut\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"reinvest\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"minAssetsOut\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"reinvest\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"removeLiquidityRestriction\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"renounceOwnership\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"bytes32\",\r\n                \"name\": \"_newFeeDistributor\",\r\n                \"type\": \"bytes32\"\r\n            }\r\n        ],\r\n        \"name\": \"setFeeDistributor\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"isApproved\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"setInputToken\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"_isPaused\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"setPause\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"shutdown\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"stkAAVE\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract IStakedTokenV2\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"sushiSwapRouter\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract ISushiSwapRouter\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address[]\",\r\n                \"name\": \"path\",\r\n                \"type\": \"address[]\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountIn\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountOutMinimum\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"sushiswap\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"tokenIn\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"tokenOut\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountIn\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountOutMinimum\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"swap\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amountOut\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"token\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"sweep\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"symbol\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"string\",\r\n                \"name\": \"\",\r\n                \"type\": \"string\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"totalAssets\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"totalSupply\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"transfer\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"transferFees\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"from\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"transferFrom\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"newOwner\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"transferOwnership\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"uniswapRouter\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract ISwapRouter\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"userDeposits\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"timeDeposited\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"withdraw\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"receiver\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"owner\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"withdraw\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"shares\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    }\r\n]\r\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AaveV2StablecoinCellar<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AaveV2StablecoinCellar<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveV2StablecoinCellar<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveV2StablecoinCellar))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AaveV2StablecoinCellar<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                AAVEV2STABLECOINCELLAR_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `AAVE` (0x48ccda3c) function"]
        pub fn aave(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([72, 204, 218, 60], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DENOMINATOR` (0x918f8674) function"]
        pub fn denominator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([145, 143, 134, 116], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERFORMANCE_FEE` (0xe93f5665) function"]
        pub fn performance_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([233, 63, 86, 101], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PLATFORM_FEE` (0x34fbc9a1) function"]
        pub fn platform_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([52, 251, 201, 161], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `POOL_FEE` (0xdd1b9c4a) function"]
        pub fn pool_fee(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([221, 27, 156, 74], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SECS_PER_YEAR` (0x28998af0) function"]
        pub fn secs_per_year(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([40, 153, 138, 240], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `USDC` (0x89a30271) function"]
        pub fn usdc(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([137, 163, 2, 113], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accruePlatformFees` (0x5fbe7a24) function"]
        pub fn accrue_platform_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 190, 122, 36], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accruedPerformanceFees` (0xdbbae3a7) function"]
        pub fn accrued_performance_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 186, 227, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accruedPlatformFees` (0x3fa0635a) function"]
        pub fn accrued_platform_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([63, 160, 99, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `activeAssets` (0x1c17b946) function"]
        pub fn active_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 23, 185, 70], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
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
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimAndUnstake` (0xad004e20) function"]
        pub fn claim_and_unstake(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 0, 78, 32], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimAndUnstake` (0xd102072f) function"]
        pub fn claim_and_unstake_with_amount(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([209, 2, 7, 47], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToAssets` (0x07a2d13a) function"]
        pub fn convert_to_assets(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `convertToShares` (0xc6e6f592) function"]
        pub fn convert_to_shares(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentAToken` (0x38420c09) function"]
        pub fn current_a_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([56, 66, 12, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `currentLendingToken` (0x2d1ff9c9) function"]
        pub fn current_lending_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([45, 31, 249, 201], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x6e553f65) function"]
        pub fn deposit_with_receiver(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x8b6099db) function"]
        pub fn deposit_with_token_and_min_assets_in_and_receiver(
            &self,
            token: ethers::core::types::Address,
            assets: ethers::core::types::U256,
            min_assets_in: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [139, 96, 153, 219],
                    (token, assets, min_assets_in, receiver),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 181, 95, 37], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enterStrategy` (0x59272e2a) function"]
        pub fn enter_strategy(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 39, 46, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feesDistributor` (0x8e0bae7f) function"]
        pub fn fees_distributor(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 11, 174, 127], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gravityBridge` (0xa4da2d02) function"]
        pub fn gravity_bridge(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([164, 218, 45, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `inactiveAssets` (0x14834938) function"]
        pub fn inactive_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([20, 131, 73, 56], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incentivesController` (0xaf1df255) function"]
        pub fn incentives_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([175, 29, 242, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `inputTokens` (0x4999eb8f) function"]
        pub fn input_tokens(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([73, 153, 235, 143], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPaused` (0xb187bd26) function"]
        pub fn is_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isShutdown` (0xbf86d690) function"]
        pub fn is_shutdown(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastTimeAccruedPlatformFees` (0xa85fb17e) function"]
        pub fn last_time_accrued_platform_fees(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 95, 177, 126], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastTimeEnteredStrategy` (0x4304421c) function"]
        pub fn last_time_entered_strategy(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([67, 4, 66, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lendingPool` (0xa59a9973) function"]
        pub fn lending_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([165, 154, 153, 115], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxDeposit` (0x6083e59a) function"]
        pub fn max_deposit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([96, 131, 229, 154], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLiquidity` (0x70c0345c) function"]
        pub fn max_liquidity(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 192, 52, 92], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multihopSwap` (0x85d2892e) function"]
        pub fn multihop_swap(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            amount_in: ethers::core::types::U256,
            amount_out_minimum: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([133, 210, 137, 46], (path, amount_in, amount_out_minimum))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
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
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalance` (0x3da9b9d0) function"]
        pub fn rebalance(
            &self,
            new_lending_token: ethers::core::types::Address,
            min_new_lending_token_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [61, 169, 185, 208],
                    (new_lending_token, min_new_lending_token_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reinvest` (0x4458bbc0) function"]
        pub fn reinvest_with_amount(
            &self,
            amount: ethers::core::types::U256,
            min_assets_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 88, 187, 192], (amount, min_assets_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reinvest` (0x83b4918b) function"]
        pub fn reinvest(
            &self,
            min_assets_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 180, 145, 139], min_assets_out)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityRestriction` (0x89895950) function"]
        pub fn remove_liquidity_restriction(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 137, 89, 80], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeeDistributor` (0xd6f3df35) function"]
        pub fn set_fee_distributor(
            &self,
            new_fee_distributor: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 243, 223, 53], new_fee_distributor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setInputToken` (0xf731f473) function"]
        pub fn set_input_token(
            &self,
            token: ethers::core::types::Address,
            is_approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 49, 244, 115], (token, is_approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPause` (0xbedb86fb) function"]
        pub fn set_pause(
            &self,
            is_paused: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 219, 134, 251], is_paused)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shutdown` (0xfc0e74d1) function"]
        pub fn shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `stkAAVE` (0x1fc29c01) function"]
        pub fn stk_aave(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([31, 194, 156, 1], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sushiSwapRouter` (0xfd829a23) function"]
        pub fn sushi_swap_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([253, 130, 154, 35], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sushiswap` (0x21a59c57) function"]
        pub fn sushiswap(
            &self,
            path: ::std::vec::Vec<ethers::core::types::Address>,
            amount_in: ethers::core::types::U256,
            amount_out_minimum: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([33, 165, 156, 87], (path, amount_in, amount_out_minimum))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swap` (0xfe029156) function"]
        pub fn swap(
            &self,
            token_in: ethers::core::types::Address,
            token_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            amount_out_minimum: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [254, 2, 145, 86],
                    (token_in, token_out, amount_in, amount_out_minimum),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweep` (0x01681a62) function"]
        pub fn sweep(
            &self,
            token: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 104, 26, 98], token)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalAssets` (0x01e1d114) function"]
        pub fn total_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([1, 225, 209, 20], ())
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
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFees` (0xc2fbe7bc) function"]
        pub fn transfer_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 251, 231, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
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
        #[doc = "Calls the contract's `uniswapRouter` (0x735de9f7) function"]
        pub fn uniswap_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([115, 93, 233, 247], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `userDeposits` (0x08f43333) function"]
        pub fn user_deposits(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([8, 244, 51, 51], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([46, 26, 125, 77], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xb460af94) function"]
        pub fn withdraw_with_receiver_and_owner(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositToAave` event"]
        pub fn deposit_to_aave_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositToAaveFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidityRestrictionRemoved` event"]
        pub fn liquidity_restriction_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidityRestrictionRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Pause` event"]
        pub fn pause_filter(&self) -> ethers::contract::builders::Event<M, PauseFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Rebalance` event"]
        pub fn rebalance_filter(&self) -> ethers::contract::builders::Event<M, RebalanceFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RedeemFromAave` event"]
        pub fn redeem_from_aave_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RedeemFromAaveFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetInputToken` event"]
        pub fn set_input_token_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetInputTokenFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Shutdown` event"]
        pub fn shutdown_filter(&self) -> ethers::contract::builders::Event<M, ShutdownFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swapped` event"]
        pub fn swapped_filter(&self) -> ethers::contract::builders::Event<M, SwappedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Sweep` event"]
        pub fn sweep_filter(&self) -> ethers::contract::builders::Event<M, SweepFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TransferFees` event"]
        pub fn transfer_fees_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TransferFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AaveV2StablecoinCellarEvents> {
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
        pub shares: ethers::core::types::U256,
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
    #[ethevent(name = "DepositToAave", abi = "DepositToAave(address,uint256)")]
    pub struct DepositToAaveFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
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
        name = "LiquidityRestrictionRemoved",
        abi = "LiquidityRestrictionRemoved()"
    )]
    pub struct LiquidityRestrictionRemovedFilter();
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "Pause", abi = "Pause(address,bool)")]
    pub struct PauseFilter {
        pub caller: ethers::core::types::Address,
        pub is_paused: bool,
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
    #[ethevent(name = "Rebalance", abi = "Rebalance(address,uint256)")]
    pub struct RebalanceFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
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
    #[ethevent(name = "RedeemFromAave", abi = "RedeemFromAave(address,uint256)")]
    pub struct RedeemFromAaveFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
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
    #[ethevent(name = "SetInputToken", abi = "SetInputToken(address,bool)")]
    pub struct SetInputTokenFilter {
        pub token: ethers::core::types::Address,
        pub is_approved: bool,
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
    #[ethevent(name = "Shutdown", abi = "Shutdown(address)")]
    pub struct ShutdownFilter {
        pub caller: ethers::core::types::Address,
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
    #[ethevent(name = "Swapped", abi = "Swapped(address,uint256,address,uint256)")]
    pub struct SwappedFilter {
        #[ethevent(indexed)]
        pub token_in: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub token_out: ethers::core::types::Address,
        pub amount_out: ethers::core::types::U256,
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
    #[ethevent(name = "Sweep", abi = "Sweep(address,uint256)")]
    pub struct SweepFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
        pub amount: ethers::core::types::U256,
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
    #[ethevent(name = "TransferFees", abi = "TransferFees(uint256,uint256)")]
    pub struct TransferFeesFilter {
        pub fee_in_shares: ethers::core::types::U256,
        pub fee_in_assets: ethers::core::types::U256,
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
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
        pub shares: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveV2StablecoinCellarEvents {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        DepositToAaveFilter(DepositToAaveFilter),
        LiquidityRestrictionRemovedFilter(LiquidityRestrictionRemovedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PauseFilter(PauseFilter),
        RebalanceFilter(RebalanceFilter),
        RedeemFromAaveFilter(RedeemFromAaveFilter),
        SetInputTokenFilter(SetInputTokenFilter),
        ShutdownFilter(ShutdownFilter),
        SwappedFilter(SwappedFilter),
        SweepFilter(SweepFilter),
        TransferFilter(TransferFilter),
        TransferFeesFilter(TransferFeesFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for AaveV2StablecoinCellarEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositToAaveFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositToAaveFilter(decoded));
            }
            if let Ok(decoded) = LiquidityRestrictionRemovedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::LiquidityRestrictionRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PauseFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::PauseFilter(decoded));
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = RedeemFromAaveFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::RedeemFromAaveFilter(decoded));
            }
            if let Ok(decoded) = SetInputTokenFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SetInputTokenFilter(decoded));
            }
            if let Ok(decoded) = ShutdownFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ShutdownFilter(decoded));
            }
            if let Ok(decoded) = SwappedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SwappedFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SweepFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferFeesFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TransferFeesFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AaveV2StablecoinCellarEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveV2StablecoinCellarEvents::ApprovalFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::DepositFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::DepositToAaveFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::LiquidityRestrictionRemovedFilter(element) => {
                    element.fmt(f)
                }
                AaveV2StablecoinCellarEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::PauseFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::RebalanceFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::RedeemFromAaveFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::SetInputTokenFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ShutdownFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::SwappedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::SweepFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::TransferFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::TransferFeesFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `AAVE`function with signature `AAVE()` and selector `[72, 204, 218, 60]`"]
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
    #[ethcall(name = "AAVE", abi = "AAVE()")]
    pub struct AaveCall;
    #[doc = "Container type for all input parameters for the `DENOMINATOR`function with signature `DENOMINATOR()` and selector `[145, 143, 134, 116]`"]
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
    #[ethcall(name = "DENOMINATOR", abi = "DENOMINATOR()")]
    pub struct DenominatorCall;
    #[doc = "Container type for all input parameters for the `DOMAIN_SEPARATOR`function with signature `DOMAIN_SEPARATOR()` and selector `[54, 68, 229, 21]`"]
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `PERFORMANCE_FEE`function with signature `PERFORMANCE_FEE()` and selector `[233, 63, 86, 101]`"]
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
    #[ethcall(name = "PERFORMANCE_FEE", abi = "PERFORMANCE_FEE()")]
    pub struct PerformanceFeeCall;
    #[doc = "Container type for all input parameters for the `PERMIT_TYPEHASH`function with signature `PERMIT_TYPEHASH()` and selector `[48, 173, 248, 31]`"]
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
    #[ethcall(name = "PERMIT_TYPEHASH", abi = "PERMIT_TYPEHASH()")]
    pub struct PermitTypehashCall;
    #[doc = "Container type for all input parameters for the `PLATFORM_FEE`function with signature `PLATFORM_FEE()` and selector `[52, 251, 201, 161]`"]
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
    #[ethcall(name = "PLATFORM_FEE", abi = "PLATFORM_FEE()")]
    pub struct PlatformFeeCall;
    #[doc = "Container type for all input parameters for the `POOL_FEE`function with signature `POOL_FEE()` and selector `[221, 27, 156, 74]`"]
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
    #[ethcall(name = "POOL_FEE", abi = "POOL_FEE()")]
    pub struct PoolFeeCall;
    #[doc = "Container type for all input parameters for the `SECS_PER_YEAR`function with signature `SECS_PER_YEAR()` and selector `[40, 153, 138, 240]`"]
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
    #[ethcall(name = "SECS_PER_YEAR", abi = "SECS_PER_YEAR()")]
    pub struct SecsPerYearCall;
    #[doc = "Container type for all input parameters for the `USDC`function with signature `USDC()` and selector `[137, 163, 2, 113]`"]
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
    #[ethcall(name = "USDC", abi = "USDC()")]
    pub struct UsdcCall;
    #[doc = "Container type for all input parameters for the `WETH`function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
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
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `accruePlatformFees`function with signature `accruePlatformFees()` and selector `[95, 190, 122, 36]`"]
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
    #[ethcall(name = "accruePlatformFees", abi = "accruePlatformFees()")]
    pub struct AccruePlatformFeesCall;
    #[doc = "Container type for all input parameters for the `accruedPerformanceFees`function with signature `accruedPerformanceFees()` and selector `[219, 186, 227, 167]`"]
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
    #[ethcall(name = "accruedPerformanceFees", abi = "accruedPerformanceFees()")]
    pub struct AccruedPerformanceFeesCall;
    #[doc = "Container type for all input parameters for the `accruedPlatformFees`function with signature `accruedPlatformFees()` and selector `[63, 160, 99, 90]`"]
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
    #[ethcall(name = "accruedPlatformFees", abi = "accruedPlatformFees()")]
    pub struct AccruedPlatformFeesCall;
    #[doc = "Container type for all input parameters for the `activeAssets`function with signature `activeAssets()` and selector `[28, 23, 185, 70]`"]
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
    #[ethcall(name = "activeAssets", abi = "activeAssets()")]
    pub struct ActiveAssetsCall;
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
    pub struct AllowanceCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
    );
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
    pub struct BalanceOfCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `claimAndUnstake`function with signature `claimAndUnstake()` and selector `[173, 0, 78, 32]`"]
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
    #[ethcall(name = "claimAndUnstake", abi = "claimAndUnstake()")]
    pub struct ClaimAndUnstakeCall;
    #[doc = "Container type for all input parameters for the `claimAndUnstake`function with signature `claimAndUnstake(uint256)` and selector `[209, 2, 7, 47]`"]
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
    #[ethcall(name = "claimAndUnstake", abi = "claimAndUnstake(uint256)")]
    pub struct ClaimAndUnstakeWithAmountCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `convertToAssets`function with signature `convertToAssets(uint256)` and selector `[7, 162, 209, 58]`"]
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `convertToShares`function with signature `convertToShares(uint256)` and selector `[198, 230, 245, 146]`"]
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `currentAToken`function with signature `currentAToken()` and selector `[56, 66, 12, 9]`"]
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
    #[ethcall(name = "currentAToken", abi = "currentAToken()")]
    pub struct CurrentATokenCall;
    #[doc = "Container type for all input parameters for the `currentLendingToken`function with signature `currentLendingToken()` and selector `[45, 31, 249, 201]`"]
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
    #[ethcall(name = "currentLendingToken", abi = "currentLendingToken()")]
    pub struct CurrentLendingTokenCall;
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
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,address)` and selector `[110, 85, 63, 101]`"]
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositWithReceiverCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(address,uint256,uint256,address)` and selector `[139, 96, 153, 219]`"]
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
    #[ethcall(name = "deposit", abi = "deposit(address,uint256,uint256,address)")]
    pub struct DepositWithTokenAndMinAssetsInAndReceiverCall {
        pub token: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
        pub min_assets_in: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256)` and selector `[182, 181, 95, 37]`"]
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
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `enterStrategy`function with signature `enterStrategy()` and selector `[89, 39, 46, 42]`"]
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
    #[ethcall(name = "enterStrategy", abi = "enterStrategy()")]
    pub struct EnterStrategyCall;
    #[doc = "Container type for all input parameters for the `feesDistributor`function with signature `feesDistributor()` and selector `[142, 11, 174, 127]`"]
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
    #[ethcall(name = "feesDistributor", abi = "feesDistributor()")]
    pub struct FeesDistributorCall;
    #[doc = "Container type for all input parameters for the `gravityBridge`function with signature `gravityBridge()` and selector `[164, 218, 45, 2]`"]
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
    #[ethcall(name = "gravityBridge", abi = "gravityBridge()")]
    pub struct GravityBridgeCall;
    #[doc = "Container type for all input parameters for the `inactiveAssets`function with signature `inactiveAssets()` and selector `[20, 131, 73, 56]`"]
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
    #[ethcall(name = "inactiveAssets", abi = "inactiveAssets()")]
    pub struct InactiveAssetsCall;
    #[doc = "Container type for all input parameters for the `incentivesController`function with signature `incentivesController()` and selector `[175, 29, 242, 85]`"]
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
    #[ethcall(name = "incentivesController", abi = "incentivesController()")]
    pub struct IncentivesControllerCall;
    #[doc = "Container type for all input parameters for the `inputTokens`function with signature `inputTokens(address)` and selector `[73, 153, 235, 143]`"]
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
    #[ethcall(name = "inputTokens", abi = "inputTokens(address)")]
    pub struct InputTokensCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isPaused`function with signature `isPaused()` and selector `[177, 135, 189, 38]`"]
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
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    #[doc = "Container type for all input parameters for the `isShutdown`function with signature `isShutdown()` and selector `[191, 134, 214, 144]`"]
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
    #[ethcall(name = "isShutdown", abi = "isShutdown()")]
    pub struct IsShutdownCall;
    #[doc = "Container type for all input parameters for the `lastTimeAccruedPlatformFees`function with signature `lastTimeAccruedPlatformFees()` and selector `[168, 95, 177, 126]`"]
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
        name = "lastTimeAccruedPlatformFees",
        abi = "lastTimeAccruedPlatformFees()"
    )]
    pub struct LastTimeAccruedPlatformFeesCall;
    #[doc = "Container type for all input parameters for the `lastTimeEnteredStrategy`function with signature `lastTimeEnteredStrategy()` and selector `[67, 4, 66, 28]`"]
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
    #[ethcall(name = "lastTimeEnteredStrategy", abi = "lastTimeEnteredStrategy()")]
    pub struct LastTimeEnteredStrategyCall;
    #[doc = "Container type for all input parameters for the `lendingPool`function with signature `lendingPool()` and selector `[165, 154, 153, 115]`"]
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
    #[ethcall(name = "lendingPool", abi = "lendingPool()")]
    pub struct LendingPoolCall;
    #[doc = "Container type for all input parameters for the `maxDeposit`function with signature `maxDeposit()` and selector `[96, 131, 229, 154]`"]
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit()")]
    pub struct MaxDepositCall;
    #[doc = "Container type for all input parameters for the `maxLiquidity`function with signature `maxLiquidity()` and selector `[112, 192, 52, 92]`"]
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
    #[ethcall(name = "maxLiquidity", abi = "maxLiquidity()")]
    pub struct MaxLiquidityCall;
    #[doc = "Container type for all input parameters for the `multihopSwap`function with signature `multihopSwap(address[],uint256,uint256)` and selector `[133, 210, 137, 46]`"]
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
    #[ethcall(name = "multihopSwap", abi = "multihopSwap(address[],uint256,uint256)")]
    pub struct MultihopSwapCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_minimum: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `nonces`function with signature `nonces(address)` and selector `[126, 206, 190, 0]`"]
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `permit`function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[213, 5, 172, 207]`"]
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `rebalance`function with signature `rebalance(address,uint256)` and selector `[61, 169, 185, 208]`"]
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
    #[ethcall(name = "rebalance", abi = "rebalance(address,uint256)")]
    pub struct RebalanceCall {
        pub new_lending_token: ethers::core::types::Address,
        pub min_new_lending_token_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `reinvest`function with signature `reinvest(uint256,uint256)` and selector `[68, 88, 187, 192]`"]
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
    #[ethcall(name = "reinvest", abi = "reinvest(uint256,uint256)")]
    pub struct ReinvestWithAmountCall {
        pub amount: ethers::core::types::U256,
        pub min_assets_out: ethers::core::types::U256,
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
        pub min_assets_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityRestriction`function with signature `removeLiquidityRestriction()` and selector `[137, 137, 89, 80]`"]
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
        name = "removeLiquidityRestriction",
        abi = "removeLiquidityRestriction()"
    )]
    pub struct RemoveLiquidityRestrictionCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership`function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `setFeeDistributor`function with signature `setFeeDistributor(bytes32)` and selector `[214, 243, 223, 53]`"]
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
    #[ethcall(name = "setFeeDistributor", abi = "setFeeDistributor(bytes32)")]
    pub struct SetFeeDistributorCall {
        pub new_fee_distributor: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setInputToken`function with signature `setInputToken(address,bool)` and selector `[247, 49, 244, 115]`"]
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
    #[ethcall(name = "setInputToken", abi = "setInputToken(address,bool)")]
    pub struct SetInputTokenCall {
        pub token: ethers::core::types::Address,
        pub is_approved: bool,
    }
    #[doc = "Container type for all input parameters for the `setPause`function with signature `setPause(bool)` and selector `[190, 219, 134, 251]`"]
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
    #[ethcall(name = "setPause", abi = "setPause(bool)")]
    pub struct SetPauseCall {
        pub is_paused: bool,
    }
    #[doc = "Container type for all input parameters for the `shutdown`function with signature `shutdown()` and selector `[252, 14, 116, 209]`"]
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
    #[ethcall(name = "shutdown", abi = "shutdown()")]
    pub struct ShutdownCall;
    #[doc = "Container type for all input parameters for the `stkAAVE`function with signature `stkAAVE()` and selector `[31, 194, 156, 1]`"]
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
    #[ethcall(name = "stkAAVE", abi = "stkAAVE()")]
    pub struct StkAAVECall;
    #[doc = "Container type for all input parameters for the `sushiSwapRouter`function with signature `sushiSwapRouter()` and selector `[253, 130, 154, 35]`"]
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
    #[ethcall(name = "sushiSwapRouter", abi = "sushiSwapRouter()")]
    pub struct SushiSwapRouterCall;
    #[doc = "Container type for all input parameters for the `sushiswap`function with signature `sushiswap(address[],uint256,uint256)` and selector `[33, 165, 156, 87]`"]
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
    #[ethcall(name = "sushiswap", abi = "sushiswap(address[],uint256,uint256)")]
    pub struct SushiswapCall {
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_minimum: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(address,address,uint256,uint256)` and selector `[254, 2, 145, 86]`"]
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
    #[ethcall(name = "swap", abi = "swap(address,address,uint256,uint256)")]
    pub struct SwapCall {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_minimum: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `sweep`function with signature `sweep(address)` and selector `[1, 104, 26, 98]`"]
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
    #[ethcall(name = "sweep", abi = "sweep(address)")]
    pub struct SweepCall {
        pub token: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `totalAssets`function with signature `totalAssets()` and selector `[1, 225, 209, 20]`"]
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
    #[ethcall(name = "totalAssets", abi = "totalAssets()")]
    pub struct TotalAssetsCall;
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
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `transferFees`function with signature `transferFees()` and selector `[194, 251, 231, 188]`"]
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
    #[ethcall(name = "transferFees", abi = "transferFees()")]
    pub struct TransferFeesCall;
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
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `uniswapRouter`function with signature `uniswapRouter()` and selector `[115, 93, 233, 247]`"]
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
    #[ethcall(name = "uniswapRouter", abi = "uniswapRouter()")]
    pub struct UniswapRouterCall;
    #[doc = "Container type for all input parameters for the `userDeposits`function with signature `userDeposits(address,uint256)` and selector `[8, 244, 51, 51]`"]
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
    #[ethcall(name = "userDeposits", abi = "userDeposits(address,uint256)")]
    pub struct UserDepositsCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address,address)` and selector `[180, 96, 175, 148]`"]
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address)")]
    pub struct WithdrawWithReceiverAndOwnerCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveV2StablecoinCellarCalls {
        Aave(AaveCall),
        Denominator(DenominatorCall),
        DomainSeparator(DomainSeparatorCall),
        PerformanceFee(PerformanceFeeCall),
        PermitTypehash(PermitTypehashCall),
        PlatformFee(PlatformFeeCall),
        PoolFee(PoolFeeCall),
        SecsPerYear(SecsPerYearCall),
        Usdc(UsdcCall),
        Weth(WethCall),
        AccruePlatformFees(AccruePlatformFeesCall),
        AccruedPerformanceFees(AccruedPerformanceFeesCall),
        AccruedPlatformFees(AccruedPlatformFeesCall),
        ActiveAssets(ActiveAssetsCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        ClaimAndUnstake(ClaimAndUnstakeCall),
        ClaimAndUnstakeWithAmount(ClaimAndUnstakeWithAmountCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CurrentAToken(CurrentATokenCall),
        CurrentLendingToken(CurrentLendingTokenCall),
        Decimals(DecimalsCall),
        DepositWithReceiver(DepositWithReceiverCall),
        DepositWithTokenAndMinAssetsInAndReceiver(DepositWithTokenAndMinAssetsInAndReceiverCall),
        Deposit(DepositCall),
        EnterStrategy(EnterStrategyCall),
        FeesDistributor(FeesDistributorCall),
        GravityBridge(GravityBridgeCall),
        InactiveAssets(InactiveAssetsCall),
        IncentivesController(IncentivesControllerCall),
        InputTokens(InputTokensCall),
        IsPaused(IsPausedCall),
        IsShutdown(IsShutdownCall),
        LastTimeAccruedPlatformFees(LastTimeAccruedPlatformFeesCall),
        LastTimeEnteredStrategy(LastTimeEnteredStrategyCall),
        LendingPool(LendingPoolCall),
        MaxDeposit(MaxDepositCall),
        MaxLiquidity(MaxLiquidityCall),
        MultihopSwap(MultihopSwapCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        Rebalance(RebalanceCall),
        ReinvestWithAmount(ReinvestWithAmountCall),
        Reinvest(ReinvestCall),
        RemoveLiquidityRestriction(RemoveLiquidityRestrictionCall),
        RenounceOwnership(RenounceOwnershipCall),
        SetFeeDistributor(SetFeeDistributorCall),
        SetInputToken(SetInputTokenCall),
        SetPause(SetPauseCall),
        Shutdown(ShutdownCall),
        StkAAVE(StkAAVECall),
        SushiSwapRouter(SushiSwapRouterCall),
        Sushiswap(SushiswapCall),
        Swap(SwapCall),
        Sweep(SweepCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFees(TransferFeesCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        UniswapRouter(UniswapRouterCall),
        UserDeposits(UserDepositsCall),
        Withdraw(WithdrawCall),
        WithdrawWithReceiverAndOwner(WithdrawWithReceiverAndOwnerCall),
    }
    impl ethers::core::abi::AbiDecode for AaveV2StablecoinCellarCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AaveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Aave(decoded));
            }
            if let Ok(decoded) =
                <DenominatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Denominator(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <PerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PerformanceFee(decoded));
            }
            if let Ok(decoded) =
                <PermitTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PermitTypehash(decoded));
            }
            if let Ok(decoded) =
                <PlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PlatformFee(decoded));
            }
            if let Ok(decoded) =
                <PoolFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PoolFee(decoded));
            }
            if let Ok(decoded) =
                <SecsPerYearCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SecsPerYear(decoded));
            }
            if let Ok(decoded) = <UsdcCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Usdc(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <AccruePlatformFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AccruePlatformFees(decoded));
            }
            if let Ok(decoded) =
                <AccruedPerformanceFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AccruedPerformanceFees(decoded));
            }
            if let Ok(decoded) =
                <AccruedPlatformFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AccruedPlatformFees(decoded));
            }
            if let Ok(decoded) =
                <ActiveAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ActiveAssets(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ClaimAndUnstakeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ClaimAndUnstake(decoded));
            }
            if let Ok(decoded) =
                <ClaimAndUnstakeWithAmountCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <CurrentATokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::CurrentAToken(decoded));
            }
            if let Ok(decoded) =
                <CurrentLendingTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::CurrentLendingToken(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DepositWithReceiverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::DepositWithReceiver(decoded));
            }
            if let Ok (decoded) = < DepositWithTokenAndMinAssetsInAndReceiverCall as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (AaveV2StablecoinCellarCalls :: DepositWithTokenAndMinAssetsInAndReceiver (decoded)) }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <EnterStrategyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::EnterStrategy(decoded));
            }
            if let Ok(decoded) =
                <FeesDistributorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::FeesDistributor(decoded));
            }
            if let Ok(decoded) =
                <GravityBridgeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::GravityBridge(decoded));
            }
            if let Ok(decoded) =
                <InactiveAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::InactiveAssets(decoded));
            }
            if let Ok(decoded) =
                <IncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IncentivesController(decoded));
            }
            if let Ok(decoded) =
                <InputTokensCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::InputTokens(decoded));
            }
            if let Ok(decoded) =
                <IsPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IsPaused(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <LastTimeAccruedPlatformFeesCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveV2StablecoinCellarCalls::LastTimeAccruedPlatformFees(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LastTimeEnteredStrategyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LastTimeEnteredStrategy(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <LendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LendingPool(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxLiquidity(decoded));
            }
            if let Ok(decoded) =
                <MultihopSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MultihopSwap(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <RebalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Rebalance(decoded));
            }
            if let Ok(decoded) =
                <ReinvestWithAmountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ReinvestWithAmount(decoded));
            }
            if let Ok(decoded) =
                <ReinvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Reinvest(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityRestrictionCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SetFeeDistributorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetFeeDistributor(decoded));
            }
            if let Ok(decoded) =
                <SetInputTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetInputToken(decoded));
            }
            if let Ok(decoded) =
                <SetPauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetPause(decoded));
            }
            if let Ok(decoded) =
                <ShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Shutdown(decoded));
            }
            if let Ok(decoded) =
                <StkAAVECall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::StkAAVE(decoded));
            }
            if let Ok(decoded) =
                <SushiSwapRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SushiSwapRouter(decoded));
            }
            if let Ok(decoded) =
                <SushiswapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Sushiswap(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Swap(decoded));
            }
            if let Ok(decoded) = <SweepCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Sweep(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TransferFees(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UniswapRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::UniswapRouter(decoded));
            }
            if let Ok(decoded) =
                <UserDepositsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::UserDeposits(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawWithReceiverAndOwnerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveV2StablecoinCellarCalls::WithdrawWithReceiverAndOwner(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveV2StablecoinCellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveV2StablecoinCellarCalls::Aave(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Denominator(element) => element.encode(),
                AaveV2StablecoinCellarCalls::DomainSeparator(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PerformanceFee(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PermitTypehash(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PlatformFee(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PoolFee(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SecsPerYear(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Usdc(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Weth(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AccruePlatformFees(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AccruedPerformanceFees(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AccruedPlatformFees(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ActiveAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Allowance(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Approve(element) => element.encode(),
                AaveV2StablecoinCellarCalls::BalanceOf(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ClaimAndUnstake(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ConvertToAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ConvertToShares(element) => element.encode(),
                AaveV2StablecoinCellarCalls::CurrentAToken(element) => element.encode(),
                AaveV2StablecoinCellarCalls::CurrentLendingToken(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Decimals(element) => element.encode(),
                AaveV2StablecoinCellarCalls::DepositWithReceiver(element) => element.encode(),
                AaveV2StablecoinCellarCalls::DepositWithTokenAndMinAssetsInAndReceiver(element) => {
                    element.encode()
                }
                AaveV2StablecoinCellarCalls::Deposit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::EnterStrategy(element) => element.encode(),
                AaveV2StablecoinCellarCalls::FeesDistributor(element) => element.encode(),
                AaveV2StablecoinCellarCalls::GravityBridge(element) => element.encode(),
                AaveV2StablecoinCellarCalls::InactiveAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IncentivesController(element) => element.encode(),
                AaveV2StablecoinCellarCalls::InputTokens(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IsPaused(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IsShutdown(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LastTimeAccruedPlatformFees(element) => {
                    element.encode()
                }
                AaveV2StablecoinCellarCalls::LastTimeEnteredStrategy(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LendingPool(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxDeposit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxLiquidity(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MultihopSwap(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Name(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Nonces(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Owner(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Permit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Rebalance(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ReinvestWithAmount(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Reinvest(element) => element.encode(),
                AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(element) => {
                    element.encode()
                }
                AaveV2StablecoinCellarCalls::RenounceOwnership(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetFeeDistributor(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetInputToken(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetPause(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Shutdown(element) => element.encode(),
                AaveV2StablecoinCellarCalls::StkAAVE(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SushiSwapRouter(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Sushiswap(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Swap(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Sweep(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Symbol(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalSupply(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Transfer(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TransferFees(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TransferFrom(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TransferOwnership(element) => element.encode(),
                AaveV2StablecoinCellarCalls::UniswapRouter(element) => element.encode(),
                AaveV2StablecoinCellarCalls::UserDeposits(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Withdraw(element) => element.encode(),
                AaveV2StablecoinCellarCalls::WithdrawWithReceiverAndOwner(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for AaveV2StablecoinCellarCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveV2StablecoinCellarCalls::Aave(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Denominator(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::DomainSeparator(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PerformanceFee(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PermitTypehash(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PlatformFee(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PoolFee(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SecsPerYear(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Usdc(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Weth(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AccruePlatformFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AccruedPerformanceFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AccruedPlatformFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ActiveAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Allowance(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Approve(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::BalanceOf(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ClaimAndUnstake(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ConvertToAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ConvertToShares(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::CurrentAToken(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::CurrentLendingToken(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Decimals(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::DepositWithReceiver(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::DepositWithTokenAndMinAssetsInAndReceiver(element) => {
                    element.fmt(f)
                }
                AaveV2StablecoinCellarCalls::Deposit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::EnterStrategy(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::FeesDistributor(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::GravityBridge(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::InactiveAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IncentivesController(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::InputTokens(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IsPaused(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IsShutdown(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LastTimeAccruedPlatformFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LastTimeEnteredStrategy(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LendingPool(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxDeposit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxLiquidity(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MultihopSwap(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Name(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Nonces(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Owner(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Permit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Rebalance(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ReinvestWithAmount(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Reinvest(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::RenounceOwnership(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetFeeDistributor(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetInputToken(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetPause(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Shutdown(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::StkAAVE(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SushiSwapRouter(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Sushiswap(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Swap(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Sweep(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Symbol(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalSupply(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Transfer(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TransferFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TransferFrom(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TransferOwnership(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::UniswapRouter(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::UserDeposits(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Withdraw(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::WithdrawWithReceiverAndOwner(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<AaveCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AaveCall) -> Self {
            AaveV2StablecoinCellarCalls::Aave(var)
        }
    }
    impl ::std::convert::From<DenominatorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DenominatorCall) -> Self {
            AaveV2StablecoinCellarCalls::Denominator(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            AaveV2StablecoinCellarCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<PerformanceFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PerformanceFeeCall) -> Self {
            AaveV2StablecoinCellarCalls::PerformanceFee(var)
        }
    }
    impl ::std::convert::From<PermitTypehashCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PermitTypehashCall) -> Self {
            AaveV2StablecoinCellarCalls::PermitTypehash(var)
        }
    }
    impl ::std::convert::From<PlatformFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PlatformFeeCall) -> Self {
            AaveV2StablecoinCellarCalls::PlatformFee(var)
        }
    }
    impl ::std::convert::From<PoolFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PoolFeeCall) -> Self {
            AaveV2StablecoinCellarCalls::PoolFee(var)
        }
    }
    impl ::std::convert::From<SecsPerYearCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SecsPerYearCall) -> Self {
            AaveV2StablecoinCellarCalls::SecsPerYear(var)
        }
    }
    impl ::std::convert::From<UsdcCall> for AaveV2StablecoinCellarCalls {
        fn from(var: UsdcCall) -> Self {
            AaveV2StablecoinCellarCalls::Usdc(var)
        }
    }
    impl ::std::convert::From<WethCall> for AaveV2StablecoinCellarCalls {
        fn from(var: WethCall) -> Self {
            AaveV2StablecoinCellarCalls::Weth(var)
        }
    }
    impl ::std::convert::From<AccruePlatformFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AccruePlatformFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::AccruePlatformFees(var)
        }
    }
    impl ::std::convert::From<AccruedPerformanceFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AccruedPerformanceFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::AccruedPerformanceFees(var)
        }
    }
    impl ::std::convert::From<AccruedPlatformFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AccruedPlatformFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::AccruedPlatformFees(var)
        }
    }
    impl ::std::convert::From<ActiveAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ActiveAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::ActiveAssets(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AllowanceCall) -> Self {
            AaveV2StablecoinCellarCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ApproveCall) -> Self {
            AaveV2StablecoinCellarCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for AaveV2StablecoinCellarCalls {
        fn from(var: BalanceOfCall) -> Self {
            AaveV2StablecoinCellarCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ClaimAndUnstakeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ClaimAndUnstakeCall) -> Self {
            AaveV2StablecoinCellarCalls::ClaimAndUnstake(var)
        }
    }
    impl ::std::convert::From<ClaimAndUnstakeWithAmountCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ClaimAndUnstakeWithAmountCall) -> Self {
            AaveV2StablecoinCellarCalls::ClaimAndUnstakeWithAmount(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ConvertToAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ConvertToSharesCall) -> Self {
            AaveV2StablecoinCellarCalls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<CurrentATokenCall> for AaveV2StablecoinCellarCalls {
        fn from(var: CurrentATokenCall) -> Self {
            AaveV2StablecoinCellarCalls::CurrentAToken(var)
        }
    }
    impl ::std::convert::From<CurrentLendingTokenCall> for AaveV2StablecoinCellarCalls {
        fn from(var: CurrentLendingTokenCall) -> Self {
            AaveV2StablecoinCellarCalls::CurrentLendingToken(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DecimalsCall) -> Self {
            AaveV2StablecoinCellarCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DepositWithReceiverCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DepositWithReceiverCall) -> Self {
            AaveV2StablecoinCellarCalls::DepositWithReceiver(var)
        }
    }
    impl ::std::convert::From<DepositWithTokenAndMinAssetsInAndReceiverCall>
        for AaveV2StablecoinCellarCalls
    {
        fn from(var: DepositWithTokenAndMinAssetsInAndReceiverCall) -> Self {
            AaveV2StablecoinCellarCalls::DepositWithTokenAndMinAssetsInAndReceiver(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DepositCall) -> Self {
            AaveV2StablecoinCellarCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<EnterStrategyCall> for AaveV2StablecoinCellarCalls {
        fn from(var: EnterStrategyCall) -> Self {
            AaveV2StablecoinCellarCalls::EnterStrategy(var)
        }
    }
    impl ::std::convert::From<FeesDistributorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: FeesDistributorCall) -> Self {
            AaveV2StablecoinCellarCalls::FeesDistributor(var)
        }
    }
    impl ::std::convert::From<GravityBridgeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: GravityBridgeCall) -> Self {
            AaveV2StablecoinCellarCalls::GravityBridge(var)
        }
    }
    impl ::std::convert::From<InactiveAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: InactiveAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::InactiveAssets(var)
        }
    }
    impl ::std::convert::From<IncentivesControllerCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IncentivesControllerCall) -> Self {
            AaveV2StablecoinCellarCalls::IncentivesController(var)
        }
    }
    impl ::std::convert::From<InputTokensCall> for AaveV2StablecoinCellarCalls {
        fn from(var: InputTokensCall) -> Self {
            AaveV2StablecoinCellarCalls::InputTokens(var)
        }
    }
    impl ::std::convert::From<IsPausedCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IsPausedCall) -> Self {
            AaveV2StablecoinCellarCalls::IsPaused(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IsShutdownCall) -> Self {
            AaveV2StablecoinCellarCalls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<LastTimeAccruedPlatformFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LastTimeAccruedPlatformFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::LastTimeAccruedPlatformFees(var)
        }
    }
    impl ::std::convert::From<LastTimeEnteredStrategyCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LastTimeEnteredStrategyCall) -> Self {
            AaveV2StablecoinCellarCalls::LastTimeEnteredStrategy(var)
        }
    }
    impl ::std::convert::From<LendingPoolCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LendingPoolCall) -> Self {
            AaveV2StablecoinCellarCalls::LendingPool(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxDepositCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxLiquidityCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxLiquidityCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxLiquidity(var)
        }
    }
    impl ::std::convert::From<MultihopSwapCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MultihopSwapCall) -> Self {
            AaveV2StablecoinCellarCalls::MultihopSwap(var)
        }
    }
    impl ::std::convert::From<NameCall> for AaveV2StablecoinCellarCalls {
        fn from(var: NameCall) -> Self {
            AaveV2StablecoinCellarCalls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: NoncesCall) -> Self {
            AaveV2StablecoinCellarCalls::Nonces(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for AaveV2StablecoinCellarCalls {
        fn from(var: OwnerCall) -> Self {
            AaveV2StablecoinCellarCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PermitCall) -> Self {
            AaveV2StablecoinCellarCalls::Permit(var)
        }
    }
    impl ::std::convert::From<RebalanceCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RebalanceCall) -> Self {
            AaveV2StablecoinCellarCalls::Rebalance(var)
        }
    }
    impl ::std::convert::From<ReinvestWithAmountCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ReinvestWithAmountCall) -> Self {
            AaveV2StablecoinCellarCalls::ReinvestWithAmount(var)
        }
    }
    impl ::std::convert::From<ReinvestCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ReinvestCall) -> Self {
            AaveV2StablecoinCellarCalls::Reinvest(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityRestrictionCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RemoveLiquidityRestrictionCall) -> Self {
            AaveV2StablecoinCellarCalls::RemoveLiquidityRestriction(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            AaveV2StablecoinCellarCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SetFeeDistributorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetFeeDistributorCall) -> Self {
            AaveV2StablecoinCellarCalls::SetFeeDistributor(var)
        }
    }
    impl ::std::convert::From<SetInputTokenCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetInputTokenCall) -> Self {
            AaveV2StablecoinCellarCalls::SetInputToken(var)
        }
    }
    impl ::std::convert::From<SetPauseCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetPauseCall) -> Self {
            AaveV2StablecoinCellarCalls::SetPause(var)
        }
    }
    impl ::std::convert::From<ShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ShutdownCall) -> Self {
            AaveV2StablecoinCellarCalls::Shutdown(var)
        }
    }
    impl ::std::convert::From<StkAAVECall> for AaveV2StablecoinCellarCalls {
        fn from(var: StkAAVECall) -> Self {
            AaveV2StablecoinCellarCalls::StkAAVE(var)
        }
    }
    impl ::std::convert::From<SushiSwapRouterCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SushiSwapRouterCall) -> Self {
            AaveV2StablecoinCellarCalls::SushiSwapRouter(var)
        }
    }
    impl ::std::convert::From<SushiswapCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SushiswapCall) -> Self {
            AaveV2StablecoinCellarCalls::Sushiswap(var)
        }
    }
    impl ::std::convert::From<SwapCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SwapCall) -> Self {
            AaveV2StablecoinCellarCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SweepCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SweepCall) -> Self {
            AaveV2StablecoinCellarCalls::Sweep(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SymbolCall) -> Self {
            AaveV2StablecoinCellarCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TotalAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TotalSupplyCall) -> Self {
            AaveV2StablecoinCellarCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TransferCall) -> Self {
            AaveV2StablecoinCellarCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TransferFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::TransferFees(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TransferFromCall) -> Self {
            AaveV2StablecoinCellarCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            AaveV2StablecoinCellarCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UniswapRouterCall> for AaveV2StablecoinCellarCalls {
        fn from(var: UniswapRouterCall) -> Self {
            AaveV2StablecoinCellarCalls::UniswapRouter(var)
        }
    }
    impl ::std::convert::From<UserDepositsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: UserDepositsCall) -> Self {
            AaveV2StablecoinCellarCalls::UserDeposits(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(var: WithdrawCall) -> Self {
            AaveV2StablecoinCellarCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawWithReceiverAndOwnerCall> for AaveV2StablecoinCellarCalls {
        fn from(var: WithdrawWithReceiverAndOwnerCall) -> Self {
            AaveV2StablecoinCellarCalls::WithdrawWithReceiverAndOwner(var)
        }
    }
}
