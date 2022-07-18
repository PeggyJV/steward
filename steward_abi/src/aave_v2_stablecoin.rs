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
            serde_json :: from_str ("{\n  \"_format\": \"hh-sol-artifact-1\",\n  \"contractName\": \"AaveV2StablecoinCellar\",\n  \"sourceName\": \"src/AaveV2StablecoinCellar.sol\",\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ERC20[]\",\n          \"name\": \"_approvedPositions\",\n          \"type\": \"address[]\"\n        },\n        {\n          \"internalType\": \"contract ICurveSwaps\",\n          \"name\": \"_curveRegistryExchange\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ISushiSwapRouter\",\n          \"name\": \"_sushiswapRouter\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ILendingPool\",\n          \"name\": \"_lendingPool\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract IAaveIncentivesController\",\n          \"name\": \"_incentivesController\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract IGravity\",\n          \"name\": \"_gravityBridge\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract IStakedTokenV2\",\n          \"name\": \"_stkAAVE\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_AAVE\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_WETH\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"STATE_AccrualOngoing\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"STATE_ContractShutdown\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"maxDeposit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"USR_DepositRestricted\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"token\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"USR_ProtectedAsset\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"USR_SamePosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"newDecimals\",\n          \"type\": \"uint8\"\n        },\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"maxDecimals\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"name\": \"USR_TooManyDecimals\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"unsupportedPosition\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"USR_UnsupportedPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"USR_UntrustedPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"platformFees\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"performanceFees\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"yield\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Accrual\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"oldPeriod\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"newPeriod\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"AccrualPeriodChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Approval\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"rewards\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"ClaimAndUnstake\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Deposit\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"DepositIntoPosition\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"oldLimit\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newLimit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"DepositLimitChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"EnterPosition\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"ExitPosition\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes32\",\n          \"name\": \"oldFeesDistributor\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes32\",\n          \"name\": \"newFeesDistributor\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"FeesDistributorChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"oldLimit\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newLimit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"LiquidityLimitChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"previousOwner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"OwnershipTransferred\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"oldPerformanceFee\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"newPerformanceFee\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"PerformanceFeeChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"oldPlatformFee\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"newPlatformFee\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"PlatformFeeChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"oldAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Rebalance\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"rewards\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Reinvest\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"feesInSharesRedeemed\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"feesInAssetsSent\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"SendFees\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"emptyPositions\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"ShutdownInitiated\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [],\n      \"name\": \"ShutdownLifted\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"token\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Sweep\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"trusted\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"TrustChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Withdraw\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"WithdrawFromPosition\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"AAVE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DOMAIN_SEPARATOR\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"WETH\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"accrualPeriod\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"accrue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"allowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"approve\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"asset\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"assetAToken\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"assetDecimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"claimAndUnstake\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"rewards\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToShares\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"curveRegistryExchange\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ICurveSwaps\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"decimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"deposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"depositLimit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"enterPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"enterPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"exitPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"exitPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"feesDistributor\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"gravityBridge\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IGravity\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"incentivesController\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IAaveIncentivesController\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"emptyPosition\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"initiateShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"isShutdown\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"isTrusted\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"lastAccrual\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"lendingPool\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ILendingPool\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"liftShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"liquidityLimit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"maxLocked\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint160\",\n          \"name\": \"\",\n          \"type\": \"uint160\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"mint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"data\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"name\": \"multicall\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"results\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"stateMutability\": \"payable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"nonces\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"performanceFee\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"deadline\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"v\",\n          \"type\": \"uint8\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"r\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"s\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"permit\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"platformFee\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address[9]\",\n          \"name\": \"route\",\n          \"type\": \"address[9]\"\n        },\n        {\n          \"internalType\": \"uint256[3][4]\",\n          \"name\": \"swapParams\",\n          \"type\": \"uint256[3][4]\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"minAssetsOut\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"rebalance\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"redeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"minAssetsOut\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"reinvest\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"renounceOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"sendFees\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"newAccrualPeriod\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"setAccrualPeriod\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newLimit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setDepositLimit\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"newFeesDistributor\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"setFeesDistributor\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newLimit\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setLiquidityLimit\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"position\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"trust\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"setTrust\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"stkAAVE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract IStakedTokenV2\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"sushiswapRouter\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ISushiSwapRouter\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"token\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"sweep\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"symbol\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalBalance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint240\",\n          \"name\": \"\",\n          \"type\": \"uint240\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalHoldings\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalLocked\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalSupply\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transferFrom\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"transferOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"withdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": \"0x6101e0604052600a805463ffffffff191662093a8017905573b813554b423266bbd4c16c32fa383394868c1f55600b553480156200003c57600080fd5b5060405162005542380380620055428339810160408190526200005f916200070c565b896040518060600160405280602c815260200162005516602c913960408051808201909152600b81526a61617665322d434c522d5360a81b602082015260128282826000620000af848262000923565b506001620000be838262000923565b5060ff81166080524660a052620000d46200025c565b60c0525050600680546001600160a01b0319166001600160a01b039690961695909517909455506200011292503391506200010c9050565b620002f8565b6001600160a01b0380891660e05287811661010052868116610120528581166101405284811661016052838116610180528281166101a0528181166101c0528a166000908152600c60205260408120805460ff19166001179055620001778b6200034a565b905060006200018882600a62000b04565b90506200019981624c4b4062000b15565b600d55620001aa8161c35062000b15565b600e5560005b8b518110156200021c576001600c60008e8481518110620001d557620001d562000b37565b6020908102919091018101516001600160a01b03168252810191909152604001600020805460ff191691151591909117905580620002138162000b4d565b915050620001b0565b50600a8054600160201b600160601b0319164263ffffffff16640100000000021790556200024a866200057d565b50505050505050505050505062000d6f565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f600060405162000290919062000b69565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b600780546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b610120516040516335ea6a7560e01b81526001600160a01b03838116600483015260009283929116906335ea6a759060240161018060405180830381865afa1580156200039b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620003c1919062000c11565b50929a50506001600160a01b038a1698506200040997505050505050505057604051630a5c5e7d60e11b81526001600160a01b03841660048201526024015b60405180910390fd5b6000600860149054906101000a900460ff169050836001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa1580156200045c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000482919062000d08565b925060128360ff161115620004b757604051630651982f60e11b815260ff841660048201526012602482015260440162000400565b60ff811615801590620004d057508260ff168160ff1614155b156200053357600e54600d5460001982146200050657620005028386846200064e60201b62003057179092919060201c565b600e555b600019811462000530576200052c8386836200064e60201b62003057179092919060201c565b600d555b50505b50600680546001600160a01b03199081166001600160a01b0395861617909155600880546001600160a81b031916600160a01b60ff86160290921691909117919093161790915590565b6007546001600160a01b03163314620005d95760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640162000400565b6001600160a01b038116620006405760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b606482015260840162000400565b6200064b81620002f8565b50565b60008160ff168360ff160362000666575082620006c7565b8160ff168360ff161015620006a25762000681838362000d26565b6200068e90600a62000b04565b6200069a908562000b15565b9050620006c7565b620006ae828462000d26565b620006bb90600a62000b04565b6200069a908562000d4c565b9392505050565b6001600160a01b03811681146200064b57600080fd5b8051620006f181620006ce565b919050565b634e487b7160e01b600052604160045260246000fd5b6000806000806000806000806000806101408b8d0312156200072d57600080fd5b8a516200073a81620006ce565b60208c0151909a506001600160401b03808211156200075857600080fd5b818d0191508d601f8301126200076d57600080fd5b815181811115620007825762000782620006f6565b604051601f19603f8360051b011681018181108482111715620007a957620007a9620006f6565b6040528181526020808201935060059290921b8401909101908f821115620007d057600080fd5b6020840193505b81841015620007fd57620007eb84620006e4565b835260209384019390920191620007d7565b9b50620008119250505060408c01620006e4565b97506200082160608c01620006e4565b96506200083160808c01620006e4565b95506200084160a08c01620006e4565b94506200085160c08c01620006e4565b93506200086160e08c01620006e4565b9250620008726101008c01620006e4565b9150620008836101208c01620006e4565b90509295989b9194979a5092959850565b600181811c90821680620008a957607f821691505b602082108103620008ca57634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200091e57600081815260208120601f850160051c81016020861015620008f95750805b601f850160051c820191505b818110156200091a5782815560010162000905565b5050505b505050565b81516001600160401b038111156200093f576200093f620006f6565b620009578162000950845462000894565b84620008d0565b602080601f8311600181146200098f5760008415620009765750858301515b600019600386901b1c1916600185901b1785556200091a565b600085815260208120601f198616915b82811015620009c0578886015182559484019460019091019084016200099f565b5085821015620009df5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b634e487b7160e01b600052601160045260246000fd5b600181815b8085111562000a4657816000190482111562000a2a5762000a2a620009ef565b8085161562000a3857918102915b93841c939080029062000a0a565b509250929050565b60008262000a5f5750600162000afe565b8162000a6e5750600062000afe565b816001811462000a87576002811462000a925762000ab2565b600191505062000afe565b60ff84111562000aa65762000aa6620009ef565b50506001821b62000afe565b5060208310610133831016604e8410600b841016171562000ad7575081810a62000afe565b62000ae3838362000a05565b806000190482111562000afa5762000afa620009ef565b0290505b92915050565b6000620006c760ff84168362000a4e565b600081600019048311821515161562000b325762000b32620009ef565b500290565b634e487b7160e01b600052603260045260246000fd5b60006001820162000b625762000b62620009ef565b5060010190565b600080835462000b798162000894565b6001828116801562000b94576001811462000baa5762000bdb565b60ff198416875282151583028701945062000bdb565b8760005260208060002060005b8581101562000bd25781548a82015290840190820162000bb7565b50505082870194505b50929695505050505050565b80516001600160801b0381168114620006f157600080fd5b805160ff81168114620006f157600080fd5b6000806000806000806000806000806000806101808d8f03121562000c3557600080fd5b8c519b5062000c4760208e0162000be7565b9a5062000c5760408e0162000be7565b995062000c6760608e0162000be7565b985062000c7760808e0162000be7565b975062000c8760a08e0162000be7565b965060c08d015164ffffffffff8116811462000ca257600080fd5b955062000cb260e08e01620006e4565b945062000cc36101008e01620006e4565b935062000cd46101208e01620006e4565b925062000ce56101408e01620006e4565b915062000cf66101608e0162000bff565b90509295989b509295989b509295989b565b60006020828403121562000d1b57600080fd5b620006c78262000bff565b600060ff821660ff84168082101562000d435762000d43620009ef565b90039392505050565b60008262000d6a57634e487b7160e01b600052601260045260246000fd5b500490565b60805160a05160c05160e05161010051610120516101405161016051610180516101a0516101c0516146a262000e7460003960008181610a200152611b1e01526000818161067301528181611a1f01528181611aca0152611ba0015260008181610524015281816119a40152818161209301526122c101526000818161092f01528181612a630152612ab9015260008181610a8c015261200f0152600081816109630152818161313c015281816132970152818161349d01526134f5015260008181610cfe01528181611bc20152611bf30152600081816109b7015281816111d90152611218015260006114a001526000611470015260006105c301526146a26000f3fe60806040526004361061041b5760003560e01c8063997292161161021e578063c63d75b611610123578063e9240c2d116100ab578063ef7ac8831161007a578063ef7ac88314610d6b578063ef8b30f714610d8b578063f2fde38b14610dab578063f666415514610dcb578063f8ba4cff14610dfd57600080fd5b8063e9240c2d14610cec578063e9ec2e9914610d20578063ecf7085814610d35578063ef465d9214610d4b57600080fd5b8063d505accf116100f2578063d505accf14610c29578063d905777e14610c49578063dd62ed3e14610c7f578063df05a52a14610cb7578063dff90b5b14610cd757600080fd5b8063c63d75b614610ba9578063c6e6f59214610bc9578063cab5923814610be9578063ce96cb7714610c0957600080fd5b8063af1df255116101a6578063ba08765211610175578063ba08765214610b0e578063bdc8144b14610b2e578063bf86d69014610b4e578063c17f674014610b68578063c2d4160114610b8857600080fd5b8063af1df25514610a7a578063b3d7f6b914610aae578063b460af9414610ace578063b8dc491b14610aee57600080fd5b8063ac353510116101ed578063ac353510146109a5578063ac9650d8146109d9578063ad004e20146109f9578063ad5c464814610a0e578063ad7a672f14610a4257600080fd5b80639972921614610908578063a4da2d021461091d578063a59a997314610951578063a9059cbb1461098557600080fd5b80635e2c576e116103245780637ecebe00116102ac5780638e0bae7f1161027b5780638e0bae7f146108665780638fdc9dfa1461087c57806394bf804d146108a357806395d89b41146108c357806396d64879146108d857600080fd5b80637ecebe00146107df57806383b4918b1461080c578063877887821461082c5780638da5cb5b1461084857600080fd5b806370a08231116102f357806370a082311461073f578063715018a61461076c578063721637151461078157806378dc9059146107975780637b3baab4146107b757600080fd5b80635e2c576e146106ca5780636e08406b146106df5780636e553f65146106ff5780636e85f1831461071f57600080fd5b806326232a2e116103a75780633dc6eabf116103765780633dc6eabf1461062c578063402d267d1461064157806348ccda3c146106615780634cdad5061461069557806356891412146106b557600080fd5b806326232a2e1461057e578063313ce567146105b15780633644e515146105f757806338d52e0f1461060c57600080fd5b80630a28a477116103ee5780630a28a477146104ba57806315f4c611146104da57806318160ddd146104fc5780631fc29c011461051257806323b872dd1461055e57600080fd5b806301e1d1141461042057806306fdde031461044857806307a2d13a1461046a578063095ea7b31461048a575b600080fd5b34801561042c57600080fd5b50610435610e12565b6040519081526020015b60405180910390f35b34801561045457600080fd5b5061045d610e49565b60405161043f91906139e6565b34801561047657600080fd5b506104356104853660046139f9565b610ed7565b34801561049657600080fd5b506104aa6104a5366004613a27565b610f1d565b604051901515815260200161043f565b3480156104c657600080fd5b506104356104d53660046139f9565b610f8a565b3480156104e657600080fd5b506104fa6104f5366004613b06565b610fc4565b005b34801561050857600080fd5b5061043560025481565b34801561051e57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161043f565b34801561056a57600080fd5b506104aa610579366004613c10565b61138c565b34801561058a57600080fd5b506105996608e1bc9bf0400081565b6040516001600160401b03909116815260200161043f565b3480156105bd57600080fd5b506105e57f000000000000000000000000000000000000000000000000000000000000000081565b60405160ff909116815260200161043f565b34801561060357600080fd5b5061043561146c565b34801561061857600080fd5b50600654610546906001600160a01b031681565b34801561063857600080fd5b506104fa6114c2565b34801561064d57600080fd5b5061043561065c366004613c51565b6114cf565b34801561066d57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b3480156106a157600080fd5b506104356106b03660046139f9565b611547565b3480156106c157600080fd5b50610435611552565b3480156106d657600080fd5b506104fa6115cd565b3480156106eb57600080fd5b506104fa6106fa3660046139f9565b61162c565b34801561070b57600080fd5b5061043561071a366004613c6e565b61171c565b34801561072b57600080fd5b506104fa61073a3660046139f9565b6117e0565b34801561074b57600080fd5b5061043561075a366004613c51565b60036020526000908152604090205481565b34801561077857600080fd5b506104fa61184b565b34801561078d57600080fd5b50610435600d5481565b3480156107a357600080fd5b506104fa6107b23660046139f9565b61187f565b3480156107c357600080fd5b50600a546105999064010000000090046001600160401b031681565b3480156107eb57600080fd5b506104356107fa366004613c51565b60056020526000908152604090205481565b34801561081857600080fd5b506104fa6108273660046139f9565b61195d565b34801561083857600080fd5b5061059967016345785d8a000081565b34801561085457600080fd5b506007546001600160a01b0316610546565b34801561087257600080fd5b50610435600b5481565b34801561088857600080fd5b50600a5461054690600160601b90046001600160a01b031681565b3480156108af57600080fd5b506104356108be366004613c6e565b611d1f565b3480156108cf57600080fd5b5061045d611d9c565b3480156108e457600080fd5b506104aa6108f3366004613c51565b600c6020526000908152604090205460ff1681565b34801561091457600080fd5b506104fa611da9565b34801561092957600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b34801561095d57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b34801561099157600080fd5b506104aa6109a0366004613a27565b611dbe565b3480156109b157600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b6109ec6109e7366004613c9e565b611e24565b60405161043f9190613d12565b348015610a0557600080fd5b50610435611f7b565b348015610a1a57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610a4e57600080fd5b50600954610a62906001600160f01b031681565b6040516001600160f01b03909116815260200161043f565b348015610a8657600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610aba57600080fd5b50610435610ac93660046139f9565b612141565b348015610ada57600080fd5b50610435610ae9366004613d74565b612160565b348015610afa57600080fd5b506104fa610b09366004613db6565b612253565b348015610b1a57600080fd5b50610435610b29366004613d74565b6123f1565b348015610b3a57600080fd5b506104fa610b493660046139f9565b612522565b348015610b5a57600080fd5b50600f546104aa9060ff1681565b348015610b7457600080fd5b50600854610546906001600160a01b031681565b348015610b9457600080fd5b506008546105e590600160a01b900460ff1681565b348015610bb557600080fd5b50610435610bc4366004613c51565b61258d565b348015610bd557600080fd5b50610435610be43660046139f9565b6125f7565b348015610bf557600080fd5b506104fa610c04366004613df9565b612617565b348015610c1557600080fd5b50610435610c24366004613c51565b6126e4565b348015610c3557600080fd5b506104fa610c44366004613e3d565b612706565b348015610c5557600080fd5b50610435610c64366004613c51565b6001600160a01b031660009081526003602052604090205490565b348015610c8b57600080fd5b50610435610c9a366004613db6565b600460209081526000928352604080842090915290825290205481565b348015610cc357600080fd5b506104fa610cd23660046139f9565b61294a565b348015610ce357600080fd5b506104fa6129b5565b348015610cf857600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610d2c57600080fd5b50610435612b56565b348015610d4157600080fd5b50610435600e5481565b348015610d5757600080fd5b506104fa610d66366004613eae565b612bc3565b348015610d7757600080fd5b506104fa610d86366004613ec9565b612c78565b348015610d9757600080fd5b50610435610da63660046139f9565b612d2b565b348015610db757600080fd5b506104fa610dc6366004613c51565b612d36565b348015610dd757600080fd5b50600a54610de89063ffffffff1681565b60405163ffffffff909116815260200161043f565b348015610e0957600080fd5b506104fa612dd1565b6000610e1c611552565b610e24612b56565b600954610e3a91906001600160f01b0316613f05565b610e449190613f1d565b905090565b60008054610e5690613f34565b80601f0160208091040260200160405190810160405280929190818152602001828054610e8290613f34565b8015610ecf5780601f10610ea457610100808354040283529160200191610ecf565b820191906000526020600020905b815481529060010190602001808311610eb257829003601f168201915b505050505081565b6002546000908015610efb57610ef6610eee610e12565b8490836130c0565b610f16565b600854610f16908490601290600160a01b900460ff16613057565b9392505050565b3360008181526004602090815260408083206001600160a01b038716808552925280832085905551919290917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590610f789086815260200190565b60405180910390a35060015b92915050565b6002546000908015610faa57610ef681610fa2610e12565b8591906130df565b600854610f16908490600160a01b900460ff166012613057565b600f5460ff1615610fe857604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b0316331461101b5760405162461bcd60e51b815260040161101290613f6e565b60405180910390fd5b6000805b80600814806110575750600085611037836001613f05565b6009811061104757611047613fa3565b60200201516001600160a01b0316145b1561107a5784816009811061106e5761106e613fa3565b6020020151915061108c565b611085600282613f05565b905061101f565b506001600160a01b0381166000908152600c602052604090205460ff166110d1576040516386433f2b60e01b81526001600160a01b0382166004820152602401611012565b6006546001600160a01b0390811690821681900361110d57604051630613aecf60e11b81526001600160a01b0382166004820152602401611012565b6000611117612b56565b6009549091506000906111349083906001600160f01b0316613f05565b6008546040516370a0823160e01b815230600482015291925060009182916001600160a01b0316906370a0823190602401602060405180830381865afa158015611182573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111a69190613fb9565b116111b157826111c8565b826111be8560001961310d565b6111c89190613f05565b90506111fe6001600160a01b0385167f0000000000000000000000000000000000000000000000000000000000000000836131f6565b604051630d4f290960e21b81526000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063353ca42490611253908c908c9087908d90600401613fd2565b6020604051808303816000875af1158015611272573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112969190613fb9565b600854909150600160a01b900460ff1660006112b188613273565b90506112bd888461348e565b600a546112db90600160601b90046001600160a01b03168383613057565b600a80546001600160a01b0392909216600160601b026bffffffffffffffffffffffff909216919091179055600061131d611317878585613057565b8561358c565b600980546001600160f01b0319166001600160f01b0383161790556040518181529091506001600160a01b038a811691908a16907fb0850b8e0f9e8315dde3c9f9f31138283e6bbe16cd29e8552eb1dcdf9fac9e3b9060200160405180910390a3505050505050505050505050565b6001600160a01b038316600090815260046020908152604080832033845290915281205460001981146113e8576113c38382613f1d565b6001600160a01b03861660009081526004602090815260408083203384529091529020555b6001600160a01b03851660009081526003602052604081208054859290611410908490613f1d565b90915550506001600160a01b038085166000818152600360205260409081902080548701905551909187169060008051602061464d833981519152906114599087815260200190565b60405180910390a3506001949350505050565b60007f0000000000000000000000000000000000000000000000000000000000000000461461149d57610e446135a2565b507f000000000000000000000000000000000000000000000000000000000000000090565b6114cd6106fa612b56565b565b600f5460009060ff16156114e557506000919050565b600e54600d54600019821480156114fd575060001981145b1561150d57506000199392505050565b600061152261151b866126e4565b849061363c565b9050600061153161151b610e12565b905061153d828261358c565b9695505050505050565b6000610f8482610ed7565b600a546000906001600160401b036401000000008204169063ffffffff1661157a8183613f05565b42106115895760009250505090565b600a54600160601b90046001600160a01b0316816115a78442613f1d565b6115b1908361406e565b6115bb919061408d565b6115c59082613f1d565b935050505090565b6007546001600160a01b031633146115f75760405162461bcd60e51b815260040161101290613f6e565b600f805460ff191690556040517f09bec6199b5712abe9cbb71997b06f6149a453eca5abec15d528e14e65e1605e90600090a1565b600f5460ff161561165057604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b0316331461167a5760405162461bcd60e51b815260040161101290613f6e565b600654600980546001600160a01b03909216918391906000906116a79084906001600160f01b03166140af565b92506101000a8154816001600160f01b0302191690836001600160f01b031602179055506116d5818361348e565b806001600160a01b03167fb6f4b9255ee989b1844a8e6b7da8906b81200c38f7b3f4f1ac31e9a241c757508360405161171091815260200190565b60405180910390a25050565b600061172783612d2b565b9050806000036117675760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f53484152455360a81b6044820152606401611012565b611772838284613656565b60065461178a906001600160a01b03163330866136b2565b611794828261373c565b60408051848152602081018390526001600160a01b0384169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d791015b60405180910390a3610f84565b6007546001600160a01b0316331461180a5760405162461bcd60e51b815260040161101290613f6e565b600b5460408051918252602082018390527f513ac19cbbaaad4e450c732ed37635178b7d83bf8e84a940ffe7e052c9c7caa2910160405180910390a1600b55565b6007546001600160a01b031633146118755760405162461bcd60e51b815260040161101290613f6e565b6114cd6000613796565b600f5460ff16156118a357604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b031633146118cd5760405162461bcd60e51b815260040161101290613f6e565b6006546001600160a01b03166118e3818361310d565b600980546000906118fe9084906001600160f01b03166140da565b92506101000a8154816001600160f01b0302191690836001600160f01b03160217905550806001600160a01b03167fde4cc1d2dd41970a827a8df55efd18c527c17c26485847d680cc2b4c71e7a87c8360405161171091815260200190565b6007546001600160a01b031633146119875760405162461bcd60e51b815260040161101290613f6e565b6040516301e9a69560e41b815230600482015260001960248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690631e9a695090604401600060405180830381600087803b1580156119f057600080fd5b505af1158015611a04573d6000803e3d6000fd5b50506040516370a0823160e01b8152306004820152600092507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691506370a0823190602401602060405180830381865afa158015611a6f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a939190613fb9565b600654604080516003808252608082019092529293506001600160a01b0390911691600091602082016060803683370190505090507f000000000000000000000000000000000000000000000000000000000000000081600081518110611afc57611afc613fa3565b60200260200101906001600160a01b031690816001600160a01b0316815250507f000000000000000000000000000000000000000000000000000000000000000081600181518110611b5057611b50613fa3565b60200260200101906001600160a01b031690816001600160a01b0316815250508181600281518110611b8457611b84613fa3565b6001600160a01b039283166020918202929092010152611be7907f0000000000000000000000000000000000000000000000000000000000000000167f0000000000000000000000000000000000000000000000000000000000000000856131f6565b60006001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166338ed173985878530611c2742603c613f05565b6040518663ffffffff1660e01b8152600401611c47959493929190614146565b6000604051808303816000875af1158015611c66573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611c8e9190810190614182565b905060008160018351611ca19190613f1d565b81518110611cb157611cb1613fa3565b6020908102919091010151600f5490915060ff16611cd357611cd3848261348e565b60408051868152602081018390526001600160a01b038616917fc003f45bc224d116b6d079100d4ab57a5b9633244c47a5a92a176c5b79a85f28910160405180910390a2505050505050565b6000611d2a83612141565b9050611d37818484613656565b600654611d4f906001600160a01b03163330846136b2565b611d59828461373c565b60408051828152602081018590526001600160a01b0384169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d791016117d3565b60018054610e5690613f34565b6009546114cd906001600160f01b031661187f565b33600090815260036020526040812080548391908390611ddf908490613f1d565b90915550506001600160a01b0383166000818152600360205260409081902080548501905551339060008051602061464d83398151915290610f789086815260200190565b6060816001600160401b03811115611e3e57611e3e613a53565b604051908082528060200260200182016040528015611e7157816020015b6060815260200190600190039081611e5c5790505b50905060005b82811015611f745760008030868685818110611e9557611e95613fa3565b9050602002810190611ea79190614227565b604051611eb5929190614274565b600060405180830381855af49150503d8060008114611ef0576040519150601f19603f3d011682016040523d82523d6000602084013e611ef5565b606091505b509150915081611f4157604481511015611f0e57600080fd5b60048101905080806020019051810190611f289190614284565b60405162461bcd60e51b815260040161101291906139e6565b80848481518110611f5457611f54613fa3565b602002602001018190525050508080611f6c90614317565b915050611e77565b5092915050565b6007546000906001600160a01b03163314611fa85760405162461bcd60e51b815260040161101290613f6e565b60408051600180825281830190925260009160208083019080368337505060085482519293506001600160a01b031691839150600090611fea57611fea613fa3565b6001600160a01b039283166020918202929092010152604051633111e7b360e01b81527f000000000000000000000000000000000000000000000000000000000000000090911690633111e7b39061204c908490600019903090600401614330565b6020604051808303816000875af115801561206b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061208f9190613fb9565b91507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663787a08a66040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156120ec57600080fd5b505af1158015612100573d6000803e3d6000fd5b505050507f8ca0188d9770b383d1a7a2ddfe5e0c1f029084481a53697d6c51525c47a8d88e8260405161213591815260200190565b60405180910390a15090565b6002546000908015610efb57610ef6612158610e12565b8490836130df565b600061216b84610f8a565b9050336001600160a01b038316146121db576001600160a01b038216600090815260046020908152604080832033845290915290205460001981146121d9576121b48282613f1d565b6001600160a01b03841660009081526004602090815260408083203384529091529020555b505b6121e7848285856137e8565b6121f18282613863565b60408051858152602081018390526001600160a01b03808516929086169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a4600654610ef6906001600160a01b031684866138c5565b6007546001600160a01b0316331461227d5760405162461bcd60e51b815260040161101290613f6e565b6006546001600160a01b03838116911614806122a657506008546001600160a01b038381169116145b806122b957506001600160a01b03821630145b806122f557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316826001600160a01b0316145b1561231e576040516339b8549160e01b81526001600160a01b0383166004820152602401611012565b6040516370a0823160e01b81523060048201526000906001600160a01b038416906370a0823190602401602060405180830381865afa158015612365573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123899190613fb9565b905061239f6001600160a01b03841683836138c5565b816001600160a01b0316836001600160a01b03167fed679328aebf74ede77ae09efcf36e90244f83643dadac1c2d9f0b21a46f6ab7836040516123e491815260200190565b60405180910390a3505050565b6000336001600160a01b03831614612461576001600160a01b0382166000908152600460209081526040808320338452909152902054600019811461245f5761243a8582613f1d565b6001600160a01b03841660009081526004602090815260408083203384529091529020555b505b61246a84611547565b9050806000036124aa5760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f41535345545360a81b6044820152606401611012565b6124b6818585856137e8565b6124c08285613863565b60408051828152602081018690526001600160a01b03808516929086169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a4600654610ef6906001600160a01b031684836138c5565b6007546001600160a01b0316331461254c5760405162461bcd60e51b815260040161101290613f6e565b600e5460408051918252602082018390527fcfb5a454b8aa7dc04ecb5bc1410b2a57969ca1d67f66d565196f60c6f9975404910160405180910390a1600e55565b600f5460009060ff16156125a357506000919050565b600e54600d54600019821480156125bb575060001981145b156125cb57506000199392505050565b60006125d961151b866126e4565b905060006125e861151b610e12565b905061153d610be4838361358c565b6002546000908015610faa57610ef68161260f610e12565b8591906130c0565b6007546001600160a01b031633146126415760405162461bcd60e51b815260040161101290613f6e565b6001600160a01b038281166000908152600c60205260409020805460ff1916831580159182179092556006549092169161268c5750806001600160a01b0316836001600160a01b0316145b1561269a5761269a8161393d565b826001600160a01b03167fd600b9348603c6deff34b4e0b28b60e1c8036c806741b9e6d90032e7f37dd27f836040516126d7911515815260200190565b60405180910390a2505050565b6001600160a01b038116600090815260036020526040812054610f8490610ed7565b428410156127565760405162461bcd60e51b815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152606401611012565b6000600161276261146c565b6001600160a01b038a811660008181526005602090815260409182902080546001810190915582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98184015280840194909452938d166060840152608083018c905260a083019390935260c08083018b90528151808403909101815260e08301909152805192019190912061190160f01b6101008301526101028201929092526101228101919091526101420160408051601f198184030181528282528051602091820120600084529083018083525260ff871690820152606081018590526080810184905260a0016020604051602081039080840390855afa15801561286e573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116158015906128a45750876001600160a01b0316816001600160a01b0316145b6128e15760405162461bcd60e51b815260206004820152600e60248201526d24a72b20a624a22fa9a4a3a722a960911b6044820152606401611012565b6001600160a01b0390811660009081526004602090815260408083208a8516808552908352928190208990555188815291928a16917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a350505050505050565b6007546001600160a01b031633146129745760405162461bcd60e51b815260040161101290613f6e565b600d5460408051918252602082018390527f1f21432dd7b8ead64d2e7c06a74baf13783b2d2f7153f099e2c4cabc3c5dbec6910160405180910390a1600d55565b6007546001600160a01b031633146129df5760405162461bcd60e51b815260040161101290613f6e565b30600090815260036020526040812054906129f982611547565b905080600003612a395760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f41535345545360a81b6044820152606401611012565b612a478160008060006137e8565b612a513083613863565b6006546001600160a01b0316612a88817f0000000000000000000000000000000000000000000000000000000000000000846131f6565b600b54604051631ffbe7f960e01b81526001600160a01b0383811660048301526024820192909252604481018490527f000000000000000000000000000000000000000000000000000000000000000090911690631ffbe7f990606401600060405180830381600087803b158015612aff57600080fd5b505af1158015612b13573d6000803e3d6000fd5b505060408051868152602081018690527f15e3e2a76a6839c244c1ed0a821c233ce8af552dffcb856089eae6cbbbb71ea6935001905060405180910390a1505050565b6006546040516370a0823160e01b81523060048201526000916001600160a01b0316906370a0823190602401602060405180830381865afa158015612b9f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e449190613fb9565b600f5460ff1615612be757604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b03163314612c115760405162461bcd60e51b815260040161101290613f6e565b8015612c2c57600654612c2c906001600160a01b031661393d565b600f805460ff191660011790556040517f6e7cab6accf9b093a6b800ed920df610db4dbfd8807417f5f2c48dd66c03babb90612c6d90831515815260200190565b60405180910390a150565b6007546001600160a01b03163314612ca25760405162461bcd60e51b815260040161101290613f6e565b6000612cac611552565b1115612ccb57604051636b86639360e11b815260040160405180910390fd5b600a546040805163ffffffff928316815291831660208301527f3c392b44ad99b1fb7c87ae7b914cbd1de1aeed3e9369a20d3070cc771669898f910160405180910390a1600a805463ffffffff191663ffffffff92909216919091179055565b6000610f84826125f7565b6007546001600160a01b03163314612d605760405162461bcd60e51b815260040161101290613f6e565b6001600160a01b038116612dc55760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611012565b612dce81613796565b50565b6000612ddb611552565b9050612def6007546001600160a01b031690565b6001600160a01b0316336001600160a01b031614158015612e105750600081115b15612e2e57604051636b86639360e11b815260040160405180910390fd5b600854600090612e4990600160a01b900460ff16600a614447565b90506000612e56826125f7565b6009546008546040516370a0823160e01b81523060048201529293506001600160f01b03909116916000916001600160a01b0316906370a0823190602401602060405180830381865afa158015612eb1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ed59190613fb9565b600a54909150600090612ef99064010000000090046001600160401b031642613f1d565b905060006301e13380670de0b6b3a76400006608e1bc9bf04000612f1d858761406e565b612f27919061406e565b612f31919061408d565b612f3b919061408d565b90506000612f4a8287896130c0565b90506000612f58858761363c565b90506000612f6e8267016345785d8a0000613979565b90506000612f7d828a8c6130c0565b9050612f9230612f8d8387613f05565b61373c565b612f9f61151b8387613f05565b612fa9908c613f05565b600a805463ffffffff908116600160601b6001600160a01b0394909416939093026bffffffffffffffff000000001916929092174290921664010000000002919091179055600980546001600160f01b0319166001600160f01b03891617905560408051858152602081018390529081018490527ffd23cefb4992bc1b95df1f544efdb9908d901288354421270f7a8f8a0dfec20a9060600160405180910390a15050505050505050505050565b60008160ff168360ff160361306d575082610f16565b8160ff168360ff1610156130a1576130858383614456565b61309090600a614447565b61309a908561406e565b9050610f16565b6130ab8284614456565b6130b690600a614447565b61309a908561408d565b8282028115158415858304851417166130d857600080fd5b0492915050565b8282028115158415858304851417166130f757600080fd5b6001826001830304018115150290509392505050565b604051631a4ca37b60e21b81526001600160a01b038381166004830152602482018390523060448301526000917f0000000000000000000000000000000000000000000000000000000000000000909116906369328dec906064016020604051808303816000875af1158015613187573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131ab9190613fb9565b9050826001600160a01b03167f84343cc97621dbc51bce198a258218a2063c160e4d473ff51007c7a60eec5fa1826040516131e891815260200190565b60405180910390a292915050565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061326d5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b6044820152606401611012565b50505050565b6040516335ea6a7560e01b81526001600160a01b03828116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a759060240161018060405180830381865afa1580156132df573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061330391906144af565b50929a50506001600160a01b038a16985061334597505050505050505057604051630a5c5e7d60e11b81526001600160a01b0384166004820152602401611012565b6000600860149054906101000a900460ff169050836001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613397573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133bb9190614590565b925060128360ff1611156133ee57604051630651982f60e11b815260ff8416600482015260126024820152604401611012565b60ff81161580159061340657508260ff168160ff1614155b1561344457600e54600d54600019821461342957613425828487613057565b600e555b60001981146134415761343d818487613057565b600d555b50505b50600680546001600160a01b03199081166001600160a01b0395861617909155600880546001600160a81b031916600160a01b60ff86160290921691909117919093161790915590565b6134c26001600160a01b0383167f0000000000000000000000000000000000000000000000000000000000000000836131f6565b60405163e8eda9df60e01b81526001600160a01b03838116600483015260248201839052306044830152600060648301527f0000000000000000000000000000000000000000000000000000000000000000169063e8eda9df90608401600060405180830381600087803b15801561353957600080fd5b505af115801561354d573d6000803e3d6000fd5b50505050816001600160a01b03167ff099efd56d0c64f9a1aa1379a470d871392b67ea7678ed5659ad4bfe7dd765758260405161171091815260200190565b600081831061359b5781610f16565b5090919050565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60006040516135d491906145ad565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b600081831161364c576000610f16565b610f168284613f1d565b600f5460ff161561367a57604051632f22819760e11b815260040160405180910390fd5b6000613685826114cf565b90508084111561326d576040516323dc290560e21b81526004810185905260248101829052604401611012565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806137355760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401611012565b5050505050565b806002600082825461374e9190613f05565b90915550506001600160a01b03821660008181526003602090815260408083208054860190555184815260008051602061464d83398151915291015b60405180910390a35050565b600780546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6006546001600160a01b031660006137fe612b56565b90508086111561385b5761381b826138168389613f1d565b61310d565b600980546000906138369084906001600160f01b03166140da565b92506101000a8154816001600160f01b0302191690836001600160f01b031602179055505b505050505050565b6001600160a01b0382166000908152600360205260408120805483929061388b908490613f1d565b90915550506002805482900390556040518181526000906001600160a01b0384169060008051602061464d8339815191529060200161378a565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061326d5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401611012565b6009546001600160f01b0316801561397557613957612dd1565b6139638260001961310d565b50600980546001600160f01b03191690555b5050565b6000610f168383670de0b6b3a76400006130c0565b60005b838110156139a9578181015183820152602001613991565b8381111561326d5750506000910152565b600081518084526139d281602086016020860161398e565b601f01601f19169290920160200192915050565b602081526000610f1660208301846139ba565b600060208284031215613a0b57600080fd5b5035919050565b6001600160a01b0381168114612dce57600080fd5b60008060408385031215613a3a57600080fd5b8235613a4581613a12565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b60405161012081016001600160401b0381118282101715613a8c57613a8c613a53565b60405290565b604051608081016001600160401b0381118282101715613a8c57613a8c613a53565b604051606081016001600160401b0381118282101715613a8c57613a8c613a53565b604051601f8201601f191681016001600160401b0381118282101715613afe57613afe613a53565b604052919050565b60008060006102c08486031215613b1c57600080fd5b601f8581860112613b2c57600080fd5b613b34613a69565b80610120870188811115613b4757600080fd5b875b81811015613b6a578035613b5c81613a12565b845260209384019301613b49565b508196508861013f890112613b7e57600080fd5b613b86613a92565b92508291506102a0880189811115613b9d57600080fd5b80821015613c00578985830112613bb45760008081fd5b613bbc613ab4565b80606084018c811115613bcf5760008081fd5b845b81811015613be9578035845260209384019301613bd1565b505085525060209093019260609190910190613b9d565b9699919850509435955050505050565b600080600060608486031215613c2557600080fd5b8335613c3081613a12565b92506020840135613c4081613a12565b929592945050506040919091013590565b600060208284031215613c6357600080fd5b8135610f1681613a12565b60008060408385031215613c8157600080fd5b823591506020830135613c9381613a12565b809150509250929050565b60008060208385031215613cb157600080fd5b82356001600160401b0380821115613cc857600080fd5b818501915085601f830112613cdc57600080fd5b813581811115613ceb57600080fd5b8660208260051b8501011115613d0057600080fd5b60209290920196919550909350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015613d6757603f19888603018452613d558583516139ba565b94509285019290850190600101613d39565b5092979650505050505050565b600080600060608486031215613d8957600080fd5b833592506020840135613d9b81613a12565b91506040840135613dab81613a12565b809150509250925092565b60008060408385031215613dc957600080fd5b8235613dd481613a12565b91506020830135613c9381613a12565b80358015158114613df457600080fd5b919050565b60008060408385031215613e0c57600080fd5b8235613e1781613a12565b9150613e2560208401613de4565b90509250929050565b60ff81168114612dce57600080fd5b600080600080600080600060e0888a031215613e5857600080fd5b8735613e6381613a12565b96506020880135613e7381613a12565b955060408801359450606088013593506080880135613e9181613e2e565b9699959850939692959460a0840135945060c09093013592915050565b600060208284031215613ec057600080fd5b610f1682613de4565b600060208284031215613edb57600080fd5b813563ffffffff81168114610f1657600080fd5b634e487b7160e01b600052601160045260246000fd5b60008219821115613f1857613f18613eef565b500190565b600082821015613f2f57613f2f613eef565b500390565b600181811c90821680613f4857607f821691505b602082108103613f6857634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b634e487b7160e01b600052603260045260246000fd5b600060208284031215613fcb57600080fd5b5051919050565b6102e08101818660005b60098110156140045781516001600160a01b0316835260209283019290910190600101613fdc565b50505061012082018560005b60048110156140575781518360005b600381101561403e57825182526020928301929091019060010161401f565b5050506060929092019160209190910190600101614010565b5050506102a08201939093526102c0015292915050565b600081600019048311821515161561408857614088613eef565b500290565b6000826140aa57634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160f01b038281168482168083038211156140d1576140d1613eef565b01949350505050565b60006001600160f01b03838116908316818110156140fa576140fa613eef565b039392505050565b600081518084526020808501945080840160005b8381101561413b5781516001600160a01b031687529582019590820190600101614116565b509495945050505050565b85815284602082015260a06040820152600061416560a0830186614102565b6001600160a01b0394909416606083015250608001529392505050565b6000602080838503121561419557600080fd5b82516001600160401b03808211156141ac57600080fd5b818501915085601f8301126141c057600080fd5b8151818111156141d2576141d2613a53565b8060051b91506141e3848301613ad6565b81815291830184019184810190888411156141fd57600080fd5b938501935b8385101561421b57845182529385019390850190614202565b98975050505050505050565b6000808335601e1984360301811261423e57600080fd5b8301803591506001600160401b0382111561425857600080fd5b60200191503681900382131561426d57600080fd5b9250929050565b8183823760009101908152919050565b60006020828403121561429657600080fd5b81516001600160401b03808211156142ad57600080fd5b818401915084601f8301126142c157600080fd5b8151818111156142d3576142d3613a53565b6142e6601f8201601f1916602001613ad6565b91508082528560208285010111156142fd57600080fd5b61430e81602084016020860161398e565b50949350505050565b60006001820161432957614329613eef565b5060010190565b6060815260006143436060830186614102565b6020830194909452506001600160a01b0391909116604090910152919050565b600181815b8085111561439e57816000190482111561438457614384613eef565b8085161561439157918102915b93841c9390800290614368565b509250929050565b6000826143b557506001610f84565b816143c257506000610f84565b81600181146143d857600281146143e2576143fe565b6001915050610f84565b60ff8411156143f3576143f3613eef565b50506001821b610f84565b5060208310610133831016604e8410600b8410161715614421575081810a610f84565b61442b8383614363565b806000190482111561443f5761443f613eef565b029392505050565b6000610f1660ff8416836143a6565b600060ff821660ff84168082101561447057614470613eef565b90039392505050565b80516fffffffffffffffffffffffffffffffff81168114613df457600080fd5b8051613df481613a12565b8051613df481613e2e565b6000806000806000806000806000806000806101808d8f0312156144d257600080fd5b8c519b506144e260208e01614479565b9a506144f060408e01614479565b99506144fe60608e01614479565b985061450c60808e01614479565b975061451a60a08e01614479565b965060c08d015164ffffffffff8116811461453457600080fd5b955061454260e08e01614499565b94506145516101008e01614499565b93506145606101208e01614499565b925061456f6101408e01614499565b915061457e6101608e016144a4565b90509295989b509295989b509295989b565b6000602082840312156145a257600080fd5b8151610f1681613e2e565b600080835481600182811c9150808316806145c957607f831692505b602080841082036145e857634e487b7160e01b86526022600452602486fd5b8180156145fc57600181146146115761463e565b60ff198616895284151585028901965061463e565b60008a81526020902060005b868110156146365781548b82015290850190830161461d565b505084890196505b50949897505050505050505056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220ea6498ab16f3f2edc730577a9653186b5a6461655c1a554ed9f137096534cbd164736f6c634300080f0033536f6d6d656c696572204161766520563220537461626c65636f696e2043656c6c6172204c5020546f6b656e\",\n  \"deployedBytecode\": \"0x60806040526004361061041b5760003560e01c8063997292161161021e578063c63d75b611610123578063e9240c2d116100ab578063ef7ac8831161007a578063ef7ac88314610d6b578063ef8b30f714610d8b578063f2fde38b14610dab578063f666415514610dcb578063f8ba4cff14610dfd57600080fd5b8063e9240c2d14610cec578063e9ec2e9914610d20578063ecf7085814610d35578063ef465d9214610d4b57600080fd5b8063d505accf116100f2578063d505accf14610c29578063d905777e14610c49578063dd62ed3e14610c7f578063df05a52a14610cb7578063dff90b5b14610cd757600080fd5b8063c63d75b614610ba9578063c6e6f59214610bc9578063cab5923814610be9578063ce96cb7714610c0957600080fd5b8063af1df255116101a6578063ba08765211610175578063ba08765214610b0e578063bdc8144b14610b2e578063bf86d69014610b4e578063c17f674014610b68578063c2d4160114610b8857600080fd5b8063af1df25514610a7a578063b3d7f6b914610aae578063b460af9414610ace578063b8dc491b14610aee57600080fd5b8063ac353510116101ed578063ac353510146109a5578063ac9650d8146109d9578063ad004e20146109f9578063ad5c464814610a0e578063ad7a672f14610a4257600080fd5b80639972921614610908578063a4da2d021461091d578063a59a997314610951578063a9059cbb1461098557600080fd5b80635e2c576e116103245780637ecebe00116102ac5780638e0bae7f1161027b5780638e0bae7f146108665780638fdc9dfa1461087c57806394bf804d146108a357806395d89b41146108c357806396d64879146108d857600080fd5b80637ecebe00146107df57806383b4918b1461080c578063877887821461082c5780638da5cb5b1461084857600080fd5b806370a08231116102f357806370a082311461073f578063715018a61461076c578063721637151461078157806378dc9059146107975780637b3baab4146107b757600080fd5b80635e2c576e146106ca5780636e08406b146106df5780636e553f65146106ff5780636e85f1831461071f57600080fd5b806326232a2e116103a75780633dc6eabf116103765780633dc6eabf1461062c578063402d267d1461064157806348ccda3c146106615780634cdad5061461069557806356891412146106b557600080fd5b806326232a2e1461057e578063313ce567146105b15780633644e515146105f757806338d52e0f1461060c57600080fd5b80630a28a477116103ee5780630a28a477146104ba57806315f4c611146104da57806318160ddd146104fc5780631fc29c011461051257806323b872dd1461055e57600080fd5b806301e1d1141461042057806306fdde031461044857806307a2d13a1461046a578063095ea7b31461048a575b600080fd5b34801561042c57600080fd5b50610435610e12565b6040519081526020015b60405180910390f35b34801561045457600080fd5b5061045d610e49565b60405161043f91906139e6565b34801561047657600080fd5b506104356104853660046139f9565b610ed7565b34801561049657600080fd5b506104aa6104a5366004613a27565b610f1d565b604051901515815260200161043f565b3480156104c657600080fd5b506104356104d53660046139f9565b610f8a565b3480156104e657600080fd5b506104fa6104f5366004613b06565b610fc4565b005b34801561050857600080fd5b5061043560025481565b34801561051e57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b03909116815260200161043f565b34801561056a57600080fd5b506104aa610579366004613c10565b61138c565b34801561058a57600080fd5b506105996608e1bc9bf0400081565b6040516001600160401b03909116815260200161043f565b3480156105bd57600080fd5b506105e57f000000000000000000000000000000000000000000000000000000000000000081565b60405160ff909116815260200161043f565b34801561060357600080fd5b5061043561146c565b34801561061857600080fd5b50600654610546906001600160a01b031681565b34801561063857600080fd5b506104fa6114c2565b34801561064d57600080fd5b5061043561065c366004613c51565b6114cf565b34801561066d57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b3480156106a157600080fd5b506104356106b03660046139f9565b611547565b3480156106c157600080fd5b50610435611552565b3480156106d657600080fd5b506104fa6115cd565b3480156106eb57600080fd5b506104fa6106fa3660046139f9565b61162c565b34801561070b57600080fd5b5061043561071a366004613c6e565b61171c565b34801561072b57600080fd5b506104fa61073a3660046139f9565b6117e0565b34801561074b57600080fd5b5061043561075a366004613c51565b60036020526000908152604090205481565b34801561077857600080fd5b506104fa61184b565b34801561078d57600080fd5b50610435600d5481565b3480156107a357600080fd5b506104fa6107b23660046139f9565b61187f565b3480156107c357600080fd5b50600a546105999064010000000090046001600160401b031681565b3480156107eb57600080fd5b506104356107fa366004613c51565b60056020526000908152604090205481565b34801561081857600080fd5b506104fa6108273660046139f9565b61195d565b34801561083857600080fd5b5061059967016345785d8a000081565b34801561085457600080fd5b506007546001600160a01b0316610546565b34801561087257600080fd5b50610435600b5481565b34801561088857600080fd5b50600a5461054690600160601b90046001600160a01b031681565b3480156108af57600080fd5b506104356108be366004613c6e565b611d1f565b3480156108cf57600080fd5b5061045d611d9c565b3480156108e457600080fd5b506104aa6108f3366004613c51565b600c6020526000908152604090205460ff1681565b34801561091457600080fd5b506104fa611da9565b34801561092957600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b34801561095d57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b34801561099157600080fd5b506104aa6109a0366004613a27565b611dbe565b3480156109b157600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b6109ec6109e7366004613c9e565b611e24565b60405161043f9190613d12565b348015610a0557600080fd5b50610435611f7b565b348015610a1a57600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610a4e57600080fd5b50600954610a62906001600160f01b031681565b6040516001600160f01b03909116815260200161043f565b348015610a8657600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610aba57600080fd5b50610435610ac93660046139f9565b612141565b348015610ada57600080fd5b50610435610ae9366004613d74565b612160565b348015610afa57600080fd5b506104fa610b09366004613db6565b612253565b348015610b1a57600080fd5b50610435610b29366004613d74565b6123f1565b348015610b3a57600080fd5b506104fa610b493660046139f9565b612522565b348015610b5a57600080fd5b50600f546104aa9060ff1681565b348015610b7457600080fd5b50600854610546906001600160a01b031681565b348015610b9457600080fd5b506008546105e590600160a01b900460ff1681565b348015610bb557600080fd5b50610435610bc4366004613c51565b61258d565b348015610bd557600080fd5b50610435610be43660046139f9565b6125f7565b348015610bf557600080fd5b506104fa610c04366004613df9565b612617565b348015610c1557600080fd5b50610435610c24366004613c51565b6126e4565b348015610c3557600080fd5b506104fa610c44366004613e3d565b612706565b348015610c5557600080fd5b50610435610c64366004613c51565b6001600160a01b031660009081526003602052604090205490565b348015610c8b57600080fd5b50610435610c9a366004613db6565b600460209081526000928352604080842090915290825290205481565b348015610cc357600080fd5b506104fa610cd23660046139f9565b61294a565b348015610ce357600080fd5b506104fa6129b5565b348015610cf857600080fd5b506105467f000000000000000000000000000000000000000000000000000000000000000081565b348015610d2c57600080fd5b50610435612b56565b348015610d4157600080fd5b50610435600e5481565b348015610d5757600080fd5b506104fa610d66366004613eae565b612bc3565b348015610d7757600080fd5b506104fa610d86366004613ec9565b612c78565b348015610d9757600080fd5b50610435610da63660046139f9565b612d2b565b348015610db757600080fd5b506104fa610dc6366004613c51565b612d36565b348015610dd757600080fd5b50600a54610de89063ffffffff1681565b60405163ffffffff909116815260200161043f565b348015610e0957600080fd5b506104fa612dd1565b6000610e1c611552565b610e24612b56565b600954610e3a91906001600160f01b0316613f05565b610e449190613f1d565b905090565b60008054610e5690613f34565b80601f0160208091040260200160405190810160405280929190818152602001828054610e8290613f34565b8015610ecf5780601f10610ea457610100808354040283529160200191610ecf565b820191906000526020600020905b815481529060010190602001808311610eb257829003601f168201915b505050505081565b6002546000908015610efb57610ef6610eee610e12565b8490836130c0565b610f16565b600854610f16908490601290600160a01b900460ff16613057565b9392505050565b3360008181526004602090815260408083206001600160a01b038716808552925280832085905551919290917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590610f789086815260200190565b60405180910390a35060015b92915050565b6002546000908015610faa57610ef681610fa2610e12565b8591906130df565b600854610f16908490600160a01b900460ff166012613057565b600f5460ff1615610fe857604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b0316331461101b5760405162461bcd60e51b815260040161101290613f6e565b60405180910390fd5b6000805b80600814806110575750600085611037836001613f05565b6009811061104757611047613fa3565b60200201516001600160a01b0316145b1561107a5784816009811061106e5761106e613fa3565b6020020151915061108c565b611085600282613f05565b905061101f565b506001600160a01b0381166000908152600c602052604090205460ff166110d1576040516386433f2b60e01b81526001600160a01b0382166004820152602401611012565b6006546001600160a01b0390811690821681900361110d57604051630613aecf60e11b81526001600160a01b0382166004820152602401611012565b6000611117612b56565b6009549091506000906111349083906001600160f01b0316613f05565b6008546040516370a0823160e01b815230600482015291925060009182916001600160a01b0316906370a0823190602401602060405180830381865afa158015611182573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906111a69190613fb9565b116111b157826111c8565b826111be8560001961310d565b6111c89190613f05565b90506111fe6001600160a01b0385167f0000000000000000000000000000000000000000000000000000000000000000836131f6565b604051630d4f290960e21b81526000906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063353ca42490611253908c908c9087908d90600401613fd2565b6020604051808303816000875af1158015611272573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112969190613fb9565b600854909150600160a01b900460ff1660006112b188613273565b90506112bd888461348e565b600a546112db90600160601b90046001600160a01b03168383613057565b600a80546001600160a01b0392909216600160601b026bffffffffffffffffffffffff909216919091179055600061131d611317878585613057565b8561358c565b600980546001600160f01b0319166001600160f01b0383161790556040518181529091506001600160a01b038a811691908a16907fb0850b8e0f9e8315dde3c9f9f31138283e6bbe16cd29e8552eb1dcdf9fac9e3b9060200160405180910390a3505050505050505050505050565b6001600160a01b038316600090815260046020908152604080832033845290915281205460001981146113e8576113c38382613f1d565b6001600160a01b03861660009081526004602090815260408083203384529091529020555b6001600160a01b03851660009081526003602052604081208054859290611410908490613f1d565b90915550506001600160a01b038085166000818152600360205260409081902080548701905551909187169060008051602061464d833981519152906114599087815260200190565b60405180910390a3506001949350505050565b60007f0000000000000000000000000000000000000000000000000000000000000000461461149d57610e446135a2565b507f000000000000000000000000000000000000000000000000000000000000000090565b6114cd6106fa612b56565b565b600f5460009060ff16156114e557506000919050565b600e54600d54600019821480156114fd575060001981145b1561150d57506000199392505050565b600061152261151b866126e4565b849061363c565b9050600061153161151b610e12565b905061153d828261358c565b9695505050505050565b6000610f8482610ed7565b600a546000906001600160401b036401000000008204169063ffffffff1661157a8183613f05565b42106115895760009250505090565b600a54600160601b90046001600160a01b0316816115a78442613f1d565b6115b1908361406e565b6115bb919061408d565b6115c59082613f1d565b935050505090565b6007546001600160a01b031633146115f75760405162461bcd60e51b815260040161101290613f6e565b600f805460ff191690556040517f09bec6199b5712abe9cbb71997b06f6149a453eca5abec15d528e14e65e1605e90600090a1565b600f5460ff161561165057604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b0316331461167a5760405162461bcd60e51b815260040161101290613f6e565b600654600980546001600160a01b03909216918391906000906116a79084906001600160f01b03166140af565b92506101000a8154816001600160f01b0302191690836001600160f01b031602179055506116d5818361348e565b806001600160a01b03167fb6f4b9255ee989b1844a8e6b7da8906b81200c38f7b3f4f1ac31e9a241c757508360405161171091815260200190565b60405180910390a25050565b600061172783612d2b565b9050806000036117675760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f53484152455360a81b6044820152606401611012565b611772838284613656565b60065461178a906001600160a01b03163330866136b2565b611794828261373c565b60408051848152602081018390526001600160a01b0384169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d791015b60405180910390a3610f84565b6007546001600160a01b0316331461180a5760405162461bcd60e51b815260040161101290613f6e565b600b5460408051918252602082018390527f513ac19cbbaaad4e450c732ed37635178b7d83bf8e84a940ffe7e052c9c7caa2910160405180910390a1600b55565b6007546001600160a01b031633146118755760405162461bcd60e51b815260040161101290613f6e565b6114cd6000613796565b600f5460ff16156118a357604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b031633146118cd5760405162461bcd60e51b815260040161101290613f6e565b6006546001600160a01b03166118e3818361310d565b600980546000906118fe9084906001600160f01b03166140da565b92506101000a8154816001600160f01b0302191690836001600160f01b03160217905550806001600160a01b03167fde4cc1d2dd41970a827a8df55efd18c527c17c26485847d680cc2b4c71e7a87c8360405161171091815260200190565b6007546001600160a01b031633146119875760405162461bcd60e51b815260040161101290613f6e565b6040516301e9a69560e41b815230600482015260001960248201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690631e9a695090604401600060405180830381600087803b1580156119f057600080fd5b505af1158015611a04573d6000803e3d6000fd5b50506040516370a0823160e01b8152306004820152600092507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691506370a0823190602401602060405180830381865afa158015611a6f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a939190613fb9565b600654604080516003808252608082019092529293506001600160a01b0390911691600091602082016060803683370190505090507f000000000000000000000000000000000000000000000000000000000000000081600081518110611afc57611afc613fa3565b60200260200101906001600160a01b031690816001600160a01b0316815250507f000000000000000000000000000000000000000000000000000000000000000081600181518110611b5057611b50613fa3565b60200260200101906001600160a01b031690816001600160a01b0316815250508181600281518110611b8457611b84613fa3565b6001600160a01b039283166020918202929092010152611be7907f0000000000000000000000000000000000000000000000000000000000000000167f0000000000000000000000000000000000000000000000000000000000000000856131f6565b60006001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166338ed173985878530611c2742603c613f05565b6040518663ffffffff1660e01b8152600401611c47959493929190614146565b6000604051808303816000875af1158015611c66573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611c8e9190810190614182565b905060008160018351611ca19190613f1d565b81518110611cb157611cb1613fa3565b6020908102919091010151600f5490915060ff16611cd357611cd3848261348e565b60408051868152602081018390526001600160a01b038616917fc003f45bc224d116b6d079100d4ab57a5b9633244c47a5a92a176c5b79a85f28910160405180910390a2505050505050565b6000611d2a83612141565b9050611d37818484613656565b600654611d4f906001600160a01b03163330846136b2565b611d59828461373c565b60408051828152602081018590526001600160a01b0384169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d791016117d3565b60018054610e5690613f34565b6009546114cd906001600160f01b031661187f565b33600090815260036020526040812080548391908390611ddf908490613f1d565b90915550506001600160a01b0383166000818152600360205260409081902080548501905551339060008051602061464d83398151915290610f789086815260200190565b6060816001600160401b03811115611e3e57611e3e613a53565b604051908082528060200260200182016040528015611e7157816020015b6060815260200190600190039081611e5c5790505b50905060005b82811015611f745760008030868685818110611e9557611e95613fa3565b9050602002810190611ea79190614227565b604051611eb5929190614274565b600060405180830381855af49150503d8060008114611ef0576040519150601f19603f3d011682016040523d82523d6000602084013e611ef5565b606091505b509150915081611f4157604481511015611f0e57600080fd5b60048101905080806020019051810190611f289190614284565b60405162461bcd60e51b815260040161101291906139e6565b80848481518110611f5457611f54613fa3565b602002602001018190525050508080611f6c90614317565b915050611e77565b5092915050565b6007546000906001600160a01b03163314611fa85760405162461bcd60e51b815260040161101290613f6e565b60408051600180825281830190925260009160208083019080368337505060085482519293506001600160a01b031691839150600090611fea57611fea613fa3565b6001600160a01b039283166020918202929092010152604051633111e7b360e01b81527f000000000000000000000000000000000000000000000000000000000000000090911690633111e7b39061204c908490600019903090600401614330565b6020604051808303816000875af115801561206b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061208f9190613fb9565b91507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663787a08a66040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156120ec57600080fd5b505af1158015612100573d6000803e3d6000fd5b505050507f8ca0188d9770b383d1a7a2ddfe5e0c1f029084481a53697d6c51525c47a8d88e8260405161213591815260200190565b60405180910390a15090565b6002546000908015610efb57610ef6612158610e12565b8490836130df565b600061216b84610f8a565b9050336001600160a01b038316146121db576001600160a01b038216600090815260046020908152604080832033845290915290205460001981146121d9576121b48282613f1d565b6001600160a01b03841660009081526004602090815260408083203384529091529020555b505b6121e7848285856137e8565b6121f18282613863565b60408051858152602081018390526001600160a01b03808516929086169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a4600654610ef6906001600160a01b031684866138c5565b6007546001600160a01b0316331461227d5760405162461bcd60e51b815260040161101290613f6e565b6006546001600160a01b03838116911614806122a657506008546001600160a01b038381169116145b806122b957506001600160a01b03821630145b806122f557507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316826001600160a01b0316145b1561231e576040516339b8549160e01b81526001600160a01b0383166004820152602401611012565b6040516370a0823160e01b81523060048201526000906001600160a01b038416906370a0823190602401602060405180830381865afa158015612365573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123899190613fb9565b905061239f6001600160a01b03841683836138c5565b816001600160a01b0316836001600160a01b03167fed679328aebf74ede77ae09efcf36e90244f83643dadac1c2d9f0b21a46f6ab7836040516123e491815260200190565b60405180910390a3505050565b6000336001600160a01b03831614612461576001600160a01b0382166000908152600460209081526040808320338452909152902054600019811461245f5761243a8582613f1d565b6001600160a01b03841660009081526004602090815260408083203384529091529020555b505b61246a84611547565b9050806000036124aa5760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f41535345545360a81b6044820152606401611012565b6124b6818585856137e8565b6124c08285613863565b60408051828152602081018690526001600160a01b03808516929086169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a4600654610ef6906001600160a01b031684836138c5565b6007546001600160a01b0316331461254c5760405162461bcd60e51b815260040161101290613f6e565b600e5460408051918252602082018390527fcfb5a454b8aa7dc04ecb5bc1410b2a57969ca1d67f66d565196f60c6f9975404910160405180910390a1600e55565b600f5460009060ff16156125a357506000919050565b600e54600d54600019821480156125bb575060001981145b156125cb57506000199392505050565b60006125d961151b866126e4565b905060006125e861151b610e12565b905061153d610be4838361358c565b6002546000908015610faa57610ef68161260f610e12565b8591906130c0565b6007546001600160a01b031633146126415760405162461bcd60e51b815260040161101290613f6e565b6001600160a01b038281166000908152600c60205260409020805460ff1916831580159182179092556006549092169161268c5750806001600160a01b0316836001600160a01b0316145b1561269a5761269a8161393d565b826001600160a01b03167fd600b9348603c6deff34b4e0b28b60e1c8036c806741b9e6d90032e7f37dd27f836040516126d7911515815260200190565b60405180910390a2505050565b6001600160a01b038116600090815260036020526040812054610f8490610ed7565b428410156127565760405162461bcd60e51b815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152606401611012565b6000600161276261146c565b6001600160a01b038a811660008181526005602090815260409182902080546001810190915582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98184015280840194909452938d166060840152608083018c905260a083019390935260c08083018b90528151808403909101815260e08301909152805192019190912061190160f01b6101008301526101028201929092526101228101919091526101420160408051601f198184030181528282528051602091820120600084529083018083525260ff871690820152606081018590526080810184905260a0016020604051602081039080840390855afa15801561286e573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b038116158015906128a45750876001600160a01b0316816001600160a01b0316145b6128e15760405162461bcd60e51b815260206004820152600e60248201526d24a72b20a624a22fa9a4a3a722a960911b6044820152606401611012565b6001600160a01b0390811660009081526004602090815260408083208a8516808552908352928190208990555188815291928a16917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a350505050505050565b6007546001600160a01b031633146129745760405162461bcd60e51b815260040161101290613f6e565b600d5460408051918252602082018390527f1f21432dd7b8ead64d2e7c06a74baf13783b2d2f7153f099e2c4cabc3c5dbec6910160405180910390a1600d55565b6007546001600160a01b031633146129df5760405162461bcd60e51b815260040161101290613f6e565b30600090815260036020526040812054906129f982611547565b905080600003612a395760405162461bcd60e51b815260206004820152600b60248201526a5a45524f5f41535345545360a81b6044820152606401611012565b612a478160008060006137e8565b612a513083613863565b6006546001600160a01b0316612a88817f0000000000000000000000000000000000000000000000000000000000000000846131f6565b600b54604051631ffbe7f960e01b81526001600160a01b0383811660048301526024820192909252604481018490527f000000000000000000000000000000000000000000000000000000000000000090911690631ffbe7f990606401600060405180830381600087803b158015612aff57600080fd5b505af1158015612b13573d6000803e3d6000fd5b505060408051868152602081018690527f15e3e2a76a6839c244c1ed0a821c233ce8af552dffcb856089eae6cbbbb71ea6935001905060405180910390a1505050565b6006546040516370a0823160e01b81523060048201526000916001600160a01b0316906370a0823190602401602060405180830381865afa158015612b9f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610e449190613fb9565b600f5460ff1615612be757604051632f22819760e11b815260040160405180910390fd5b6007546001600160a01b03163314612c115760405162461bcd60e51b815260040161101290613f6e565b8015612c2c57600654612c2c906001600160a01b031661393d565b600f805460ff191660011790556040517f6e7cab6accf9b093a6b800ed920df610db4dbfd8807417f5f2c48dd66c03babb90612c6d90831515815260200190565b60405180910390a150565b6007546001600160a01b03163314612ca25760405162461bcd60e51b815260040161101290613f6e565b6000612cac611552565b1115612ccb57604051636b86639360e11b815260040160405180910390fd5b600a546040805163ffffffff928316815291831660208301527f3c392b44ad99b1fb7c87ae7b914cbd1de1aeed3e9369a20d3070cc771669898f910160405180910390a1600a805463ffffffff191663ffffffff92909216919091179055565b6000610f84826125f7565b6007546001600160a01b03163314612d605760405162461bcd60e51b815260040161101290613f6e565b6001600160a01b038116612dc55760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611012565b612dce81613796565b50565b6000612ddb611552565b9050612def6007546001600160a01b031690565b6001600160a01b0316336001600160a01b031614158015612e105750600081115b15612e2e57604051636b86639360e11b815260040160405180910390fd5b600854600090612e4990600160a01b900460ff16600a614447565b90506000612e56826125f7565b6009546008546040516370a0823160e01b81523060048201529293506001600160f01b03909116916000916001600160a01b0316906370a0823190602401602060405180830381865afa158015612eb1573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ed59190613fb9565b600a54909150600090612ef99064010000000090046001600160401b031642613f1d565b905060006301e13380670de0b6b3a76400006608e1bc9bf04000612f1d858761406e565b612f27919061406e565b612f31919061408d565b612f3b919061408d565b90506000612f4a8287896130c0565b90506000612f58858761363c565b90506000612f6e8267016345785d8a0000613979565b90506000612f7d828a8c6130c0565b9050612f9230612f8d8387613f05565b61373c565b612f9f61151b8387613f05565b612fa9908c613f05565b600a805463ffffffff908116600160601b6001600160a01b0394909416939093026bffffffffffffffff000000001916929092174290921664010000000002919091179055600980546001600160f01b0319166001600160f01b03891617905560408051858152602081018390529081018490527ffd23cefb4992bc1b95df1f544efdb9908d901288354421270f7a8f8a0dfec20a9060600160405180910390a15050505050505050505050565b60008160ff168360ff160361306d575082610f16565b8160ff168360ff1610156130a1576130858383614456565b61309090600a614447565b61309a908561406e565b9050610f16565b6130ab8284614456565b6130b690600a614447565b61309a908561408d565b8282028115158415858304851417166130d857600080fd5b0492915050565b8282028115158415858304851417166130f757600080fd5b6001826001830304018115150290509392505050565b604051631a4ca37b60e21b81526001600160a01b038381166004830152602482018390523060448301526000917f0000000000000000000000000000000000000000000000000000000000000000909116906369328dec906064016020604051808303816000875af1158015613187573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906131ab9190613fb9565b9050826001600160a01b03167f84343cc97621dbc51bce198a258218a2063c160e4d473ff51007c7a60eec5fa1826040516131e891815260200190565b60405180910390a292915050565b600060405163095ea7b360e01b8152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061326d5760405162461bcd60e51b815260206004820152600e60248201526d1054141493d59157d1905253115160921b6044820152606401611012565b50505050565b6040516335ea6a7560e01b81526001600160a01b03828116600483015260009182917f000000000000000000000000000000000000000000000000000000000000000016906335ea6a759060240161018060405180830381865afa1580156132df573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061330391906144af565b50929a50506001600160a01b038a16985061334597505050505050505057604051630a5c5e7d60e11b81526001600160a01b0384166004820152602401611012565b6000600860149054906101000a900460ff169050836001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015613397573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906133bb9190614590565b925060128360ff1611156133ee57604051630651982f60e11b815260ff8416600482015260126024820152604401611012565b60ff81161580159061340657508260ff168160ff1614155b1561344457600e54600d54600019821461342957613425828487613057565b600e555b60001981146134415761343d818487613057565b600d555b50505b50600680546001600160a01b03199081166001600160a01b0395861617909155600880546001600160a81b031916600160a01b60ff86160290921691909117919093161790915590565b6134c26001600160a01b0383167f0000000000000000000000000000000000000000000000000000000000000000836131f6565b60405163e8eda9df60e01b81526001600160a01b03838116600483015260248201839052306044830152600060648301527f0000000000000000000000000000000000000000000000000000000000000000169063e8eda9df90608401600060405180830381600087803b15801561353957600080fd5b505af115801561354d573d6000803e3d6000fd5b50505050816001600160a01b03167ff099efd56d0c64f9a1aa1379a470d871392b67ea7678ed5659ad4bfe7dd765758260405161171091815260200190565b600081831061359b5781610f16565b5090919050565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60006040516135d491906145ad565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b600081831161364c576000610f16565b610f168284613f1d565b600f5460ff161561367a57604051632f22819760e11b815260040160405180910390fd5b6000613685826114cf565b90508084111561326d576040516323dc290560e21b81526004810185905260248101829052604401611012565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806137355760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401611012565b5050505050565b806002600082825461374e9190613f05565b90915550506001600160a01b03821660008181526003602090815260408083208054860190555184815260008051602061464d83398151915291015b60405180910390a35050565b600780546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6006546001600160a01b031660006137fe612b56565b90508086111561385b5761381b826138168389613f1d565b61310d565b600980546000906138369084906001600160f01b03166140da565b92506101000a8154816001600160f01b0302191690836001600160f01b031602179055505b505050505050565b6001600160a01b0382166000908152600360205260408120805483929061388b908490613f1d565b90915550506002805482900390556040518181526000906001600160a01b0384169060008051602061464d8339815191529060200161378a565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061326d5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401611012565b6009546001600160f01b0316801561397557613957612dd1565b6139638260001961310d565b50600980546001600160f01b03191690555b5050565b6000610f168383670de0b6b3a76400006130c0565b60005b838110156139a9578181015183820152602001613991565b8381111561326d5750506000910152565b600081518084526139d281602086016020860161398e565b601f01601f19169290920160200192915050565b602081526000610f1660208301846139ba565b600060208284031215613a0b57600080fd5b5035919050565b6001600160a01b0381168114612dce57600080fd5b60008060408385031215613a3a57600080fd5b8235613a4581613a12565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b60405161012081016001600160401b0381118282101715613a8c57613a8c613a53565b60405290565b604051608081016001600160401b0381118282101715613a8c57613a8c613a53565b604051606081016001600160401b0381118282101715613a8c57613a8c613a53565b604051601f8201601f191681016001600160401b0381118282101715613afe57613afe613a53565b604052919050565b60008060006102c08486031215613b1c57600080fd5b601f8581860112613b2c57600080fd5b613b34613a69565b80610120870188811115613b4757600080fd5b875b81811015613b6a578035613b5c81613a12565b845260209384019301613b49565b508196508861013f890112613b7e57600080fd5b613b86613a92565b92508291506102a0880189811115613b9d57600080fd5b80821015613c00578985830112613bb45760008081fd5b613bbc613ab4565b80606084018c811115613bcf5760008081fd5b845b81811015613be9578035845260209384019301613bd1565b505085525060209093019260609190910190613b9d565b9699919850509435955050505050565b600080600060608486031215613c2557600080fd5b8335613c3081613a12565b92506020840135613c4081613a12565b929592945050506040919091013590565b600060208284031215613c6357600080fd5b8135610f1681613a12565b60008060408385031215613c8157600080fd5b823591506020830135613c9381613a12565b809150509250929050565b60008060208385031215613cb157600080fd5b82356001600160401b0380821115613cc857600080fd5b818501915085601f830112613cdc57600080fd5b813581811115613ceb57600080fd5b8660208260051b8501011115613d0057600080fd5b60209290920196919550909350505050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015613d6757603f19888603018452613d558583516139ba565b94509285019290850190600101613d39565b5092979650505050505050565b600080600060608486031215613d8957600080fd5b833592506020840135613d9b81613a12565b91506040840135613dab81613a12565b809150509250925092565b60008060408385031215613dc957600080fd5b8235613dd481613a12565b91506020830135613c9381613a12565b80358015158114613df457600080fd5b919050565b60008060408385031215613e0c57600080fd5b8235613e1781613a12565b9150613e2560208401613de4565b90509250929050565b60ff81168114612dce57600080fd5b600080600080600080600060e0888a031215613e5857600080fd5b8735613e6381613a12565b96506020880135613e7381613a12565b955060408801359450606088013593506080880135613e9181613e2e565b9699959850939692959460a0840135945060c09093013592915050565b600060208284031215613ec057600080fd5b610f1682613de4565b600060208284031215613edb57600080fd5b813563ffffffff81168114610f1657600080fd5b634e487b7160e01b600052601160045260246000fd5b60008219821115613f1857613f18613eef565b500190565b600082821015613f2f57613f2f613eef565b500390565b600181811c90821680613f4857607f821691505b602082108103613f6857634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252818101527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604082015260600190565b634e487b7160e01b600052603260045260246000fd5b600060208284031215613fcb57600080fd5b5051919050565b6102e08101818660005b60098110156140045781516001600160a01b0316835260209283019290910190600101613fdc565b50505061012082018560005b60048110156140575781518360005b600381101561403e57825182526020928301929091019060010161401f565b5050506060929092019160209190910190600101614010565b5050506102a08201939093526102c0015292915050565b600081600019048311821515161561408857614088613eef565b500290565b6000826140aa57634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160f01b038281168482168083038211156140d1576140d1613eef565b01949350505050565b60006001600160f01b03838116908316818110156140fa576140fa613eef565b039392505050565b600081518084526020808501945080840160005b8381101561413b5781516001600160a01b031687529582019590820190600101614116565b509495945050505050565b85815284602082015260a06040820152600061416560a0830186614102565b6001600160a01b0394909416606083015250608001529392505050565b6000602080838503121561419557600080fd5b82516001600160401b03808211156141ac57600080fd5b818501915085601f8301126141c057600080fd5b8151818111156141d2576141d2613a53565b8060051b91506141e3848301613ad6565b81815291830184019184810190888411156141fd57600080fd5b938501935b8385101561421b57845182529385019390850190614202565b98975050505050505050565b6000808335601e1984360301811261423e57600080fd5b8301803591506001600160401b0382111561425857600080fd5b60200191503681900382131561426d57600080fd5b9250929050565b8183823760009101908152919050565b60006020828403121561429657600080fd5b81516001600160401b03808211156142ad57600080fd5b818401915084601f8301126142c157600080fd5b8151818111156142d3576142d3613a53565b6142e6601f8201601f1916602001613ad6565b91508082528560208285010111156142fd57600080fd5b61430e81602084016020860161398e565b50949350505050565b60006001820161432957614329613eef565b5060010190565b6060815260006143436060830186614102565b6020830194909452506001600160a01b0391909116604090910152919050565b600181815b8085111561439e57816000190482111561438457614384613eef565b8085161561439157918102915b93841c9390800290614368565b509250929050565b6000826143b557506001610f84565b816143c257506000610f84565b81600181146143d857600281146143e2576143fe565b6001915050610f84565b60ff8411156143f3576143f3613eef565b50506001821b610f84565b5060208310610133831016604e8410600b8410161715614421575081810a610f84565b61442b8383614363565b806000190482111561443f5761443f613eef565b029392505050565b6000610f1660ff8416836143a6565b600060ff821660ff84168082101561447057614470613eef565b90039392505050565b80516fffffffffffffffffffffffffffffffff81168114613df457600080fd5b8051613df481613a12565b8051613df481613e2e565b6000806000806000806000806000806000806101808d8f0312156144d257600080fd5b8c519b506144e260208e01614479565b9a506144f060408e01614479565b99506144fe60608e01614479565b985061450c60808e01614479565b975061451a60a08e01614479565b965060c08d015164ffffffffff8116811461453457600080fd5b955061454260e08e01614499565b94506145516101008e01614499565b93506145606101208e01614499565b925061456f6101408e01614499565b915061457e6101608e016144a4565b90509295989b509295989b509295989b565b6000602082840312156145a257600080fd5b8151610f1681613e2e565b600080835481600182811c9150808316806145c957607f831692505b602080841082036145e857634e487b7160e01b86526022600452602486fd5b8180156145fc57600181146146115761463e565b60ff198616895284151585028901965061463e565b60008a81526020902060005b868110156146365781548b82015290850190830161461d565b505084890196505b50949897505050505050505056feddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa2646970667358221220ea6498ab16f3f2edc730577a9653186b5a6461655c1a554ed9f137096534cbd164736f6c634300080f0033\",\n  \"linkReferences\": {},\n  \"deployedLinkReferences\": {}\n}\n") . expect ("invalid abi")
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
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
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
        #[doc = "Calls the contract's `accrualPeriod` (0xf6664155) function"]
        pub fn accrual_period(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([246, 102, 65, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `accrue` (0xf8ba4cff) function"]
        pub fn accrue(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 186, 76, 255], ())
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
        #[doc = "Calls the contract's `asset` (0x38d52e0f) function"]
        pub fn asset(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetAToken` (0xc17f6740) function"]
        pub fn asset_a_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([193, 127, 103, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetDecimals` (0xc2d41601) function"]
        pub fn asset_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
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
        #[doc = "Calls the contract's `curveRegistryExchange` (0xac353510) function"]
        pub fn curve_registry_exchange(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([172, 53, 53, 16], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x6e553f65) function"]
        pub fn deposit(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositLimit` (0xecf70858) function"]
        pub fn deposit_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([236, 247, 8, 88], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enterPosition` (0x3dc6eabf) function"]
        pub fn enter_position(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 198, 234, 191], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `enterPosition` (0x6e08406b) function"]
        pub fn enter_position_with_assets(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 8, 64, 107], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitPosition` (0x78dc9059) function"]
        pub fn exit_position_with_assets(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 220, 144, 89], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exitPosition` (0x99729216) function"]
        pub fn exit_position(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 114, 146, 22], ())
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
        #[doc = "Calls the contract's `incentivesController` (0xaf1df255) function"]
        pub fn incentives_controller(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([175, 29, 242, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateShutdown` (0xef465d92) function"]
        pub fn initiate_shutdown(
            &self,
            empty_position: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 70, 93, 146], empty_position)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isShutdown` (0xbf86d690) function"]
        pub fn is_shutdown(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isTrusted` (0x96d64879) function"]
        pub fn is_trusted(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([150, 214, 72, 121], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastAccrual` (0x7b3baab4) function"]
        pub fn last_accrual(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([123, 59, 170, 180], ())
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
        #[doc = "Calls the contract's `liftShutdown` (0x5e2c576e) function"]
        pub fn lift_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 44, 87, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidityLimit` (0x72163715) function"]
        pub fn liquidity_limit(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([114, 22, 55, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxDeposit` (0x402d267d) function"]
        pub fn max_deposit(
            &self,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], receiver)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxLocked` (0x8fdc9dfa) function"]
        pub fn max_locked(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([143, 220, 157, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxMint` (0xc63d75b6) function"]
        pub fn max_mint(
            &self,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], receiver)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxRedeem` (0xd905777e) function"]
        pub fn max_redeem(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([217, 5, 119, 126], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxWithdraw` (0xce96cb77) function"]
        pub fn max_withdraw(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([206, 150, 203, 119], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x94bf804d) function"]
        pub fn mint(
            &self,
            shares: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([148, 191, 128, 77], (shares, receiver))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::Bytes>>
        {
            self.0
                .method_hash([172, 150, 80, 216], data)
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
        #[doc = "Calls the contract's `performanceFee` (0x87788782) function"]
        pub fn performance_fee(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([135, 120, 135, 130], ())
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
        #[doc = "Calls the contract's `platformFee` (0x26232a2e) function"]
        pub fn platform_fee(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([38, 35, 42, 46], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewDeposit` (0xef8b30f7) function"]
        pub fn preview_deposit(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewMint` (0xb3d7f6b9) function"]
        pub fn preview_mint(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([179, 215, 246, 185], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewRedeem` (0x4cdad506) function"]
        pub fn preview_redeem(
            &self,
            shares: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([76, 218, 213, 6], shares)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `previewWithdraw` (0x0a28a477) function"]
        pub fn preview_withdraw(
            &self,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([10, 40, 164, 119], assets)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalance` (0x15f4c611) function"]
        pub fn rebalance(
            &self,
            route: [ethers::core::types::Address; 9usize],
            swap_params: [[ethers::core::types::U256; 3usize]; 4usize],
            min_assets_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 244, 198, 17], (route, swap_params, min_assets_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeem` (0xba087652) function"]
        pub fn redeem(
            &self,
            shares: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
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
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendFees` (0xdff90b5b) function"]
        pub fn send_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAccrualPeriod` (0xef7ac883) function"]
        pub fn set_accrual_period(
            &self,
            new_accrual_period: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 122, 200, 131], new_accrual_period)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDepositLimit` (0xbdc8144b) function"]
        pub fn set_deposit_limit(
            &self,
            new_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 200, 20, 75], new_limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFeesDistributor` (0x6e85f183) function"]
        pub fn set_fees_distributor(
            &self,
            new_fees_distributor: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 133, 241, 131], new_fees_distributor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLiquidityLimit` (0xdf05a52a) function"]
        pub fn set_liquidity_limit(
            &self,
            new_limit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 5, 165, 42], new_limit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrust` (0xcab59238) function"]
        pub fn set_trust(
            &self,
            position: ethers::core::types::Address,
            trust: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 181, 146, 56], (position, trust))
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
        #[doc = "Calls the contract's `sushiswapRouter` (0xe9240c2d) function"]
        pub fn sushiswap_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([233, 36, 12, 45], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweep` (0xb8dc491b) function"]
        pub fn sweep(
            &self,
            token: ethers::core::types::Address,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 220, 73, 27], (token, to))
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
        #[doc = "Calls the contract's `totalBalance` (0xad7a672f) function"]
        pub fn total_balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([173, 122, 103, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalHoldings` (0xe9ec2e99) function"]
        pub fn total_holdings(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([233, 236, 46, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalLocked` (0x56891412) function"]
        pub fn total_locked(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([86, 137, 20, 18], ())
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
        #[doc = "Calls the contract's `withdraw` (0xb460af94) function"]
        pub fn withdraw(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Accrual` event"]
        pub fn accrual_filter(&self) -> ethers::contract::builders::Event<M, AccrualFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AccrualPeriodChanged` event"]
        pub fn accrual_period_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AccrualPeriodChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ClaimAndUnstake` event"]
        pub fn claim_and_unstake_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ClaimAndUnstakeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositIntoPosition` event"]
        pub fn deposit_into_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositIntoPositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DepositLimitChanged` event"]
        pub fn deposit_limit_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DepositLimitChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EnterPosition` event"]
        pub fn enter_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EnterPositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExitPosition` event"]
        pub fn exit_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExitPositionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FeesDistributorChanged` event"]
        pub fn fees_distributor_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, FeesDistributorChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidityLimitChanged` event"]
        pub fn liquidity_limit_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidityLimitChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PerformanceFeeChanged` event"]
        pub fn performance_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PerformanceFeeChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlatformFeeChanged` event"]
        pub fn platform_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PlatformFeeChangedFilter> {
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
        #[doc = "Gets the contract's `SendFees` event"]
        pub fn send_fees_filter(&self) -> ethers::contract::builders::Event<M, SendFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShutdownInitiated` event"]
        pub fn shutdown_initiated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShutdownInitiatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShutdownLifted` event"]
        pub fn shutdown_lifted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShutdownLiftedFilter> {
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
        #[doc = "Gets the contract's `TrustChanged` event"]
        pub fn trust_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TrustChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `WithdrawFromPosition` event"]
        pub fn withdraw_from_position_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, WithdrawFromPositionFilter> {
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
    #[ethevent(name = "Accrual", abi = "Accrual(uint256,uint256,uint256)")]
    pub struct AccrualFilter {
        pub platform_fees: ethers::core::types::U256,
        pub performance_fees: ethers::core::types::U256,
        pub yield_: ethers::core::types::U256,
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
        name = "AccrualPeriodChanged",
        abi = "AccrualPeriodChanged(uint32,uint32)"
    )]
    pub struct AccrualPeriodChangedFilter {
        pub old_period: u32,
        pub new_period: u32,
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
    #[ethevent(name = "ClaimAndUnstake", abi = "ClaimAndUnstake(uint256)")]
    pub struct ClaimAndUnstakeFilter {
        pub rewards: ethers::core::types::U256,
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
    #[ethevent(
        name = "DepositIntoPosition",
        abi = "DepositIntoPosition(address,uint256)"
    )]
    pub struct DepositIntoPositionFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
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
        name = "DepositLimitChanged",
        abi = "DepositLimitChanged(uint256,uint256)"
    )]
    pub struct DepositLimitChangedFilter {
        pub old_limit: ethers::core::types::U256,
        pub new_limit: ethers::core::types::U256,
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
    #[ethevent(name = "EnterPosition", abi = "EnterPosition(address,uint256)")]
    pub struct EnterPositionFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
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
    #[ethevent(name = "ExitPosition", abi = "ExitPosition(address,uint256)")]
    pub struct ExitPositionFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
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
        name = "FeesDistributorChanged",
        abi = "FeesDistributorChanged(bytes32,bytes32)"
    )]
    pub struct FeesDistributorChangedFilter {
        pub old_fees_distributor: [u8; 32],
        pub new_fees_distributor: [u8; 32],
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
        name = "LiquidityLimitChanged",
        abi = "LiquidityLimitChanged(uint256,uint256)"
    )]
    pub struct LiquidityLimitChangedFilter {
        pub old_limit: ethers::core::types::U256,
        pub new_limit: ethers::core::types::U256,
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
    #[ethevent(
        name = "PerformanceFeeChanged",
        abi = "PerformanceFeeChanged(uint64,uint64)"
    )]
    pub struct PerformanceFeeChangedFilter {
        pub old_performance_fee: u64,
        pub new_performance_fee: u64,
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
    #[ethevent(name = "PlatformFeeChanged", abi = "PlatformFeeChanged(uint64,uint64)")]
    pub struct PlatformFeeChangedFilter {
        pub old_platform_fee: u64,
        pub new_platform_fee: u64,
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
    #[ethevent(name = "Rebalance", abi = "Rebalance(address,address,uint256)")]
    pub struct RebalanceFilter {
        #[ethevent(indexed)]
        pub old_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_asset: ethers::core::types::Address,
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
    #[ethevent(name = "Reinvest", abi = "Reinvest(address,uint256,uint256)")]
    pub struct ReinvestFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
        pub rewards: ethers::core::types::U256,
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
    #[ethevent(name = "SendFees", abi = "SendFees(uint256,uint256)")]
    pub struct SendFeesFilter {
        pub fees_in_shares_redeemed: ethers::core::types::U256,
        pub fees_in_assets_sent: ethers::core::types::U256,
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
    #[ethevent(name = "ShutdownInitiated", abi = "ShutdownInitiated(bool)")]
    pub struct ShutdownInitiatedFilter {
        pub empty_positions: bool,
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
    #[ethevent(name = "ShutdownLifted", abi = "ShutdownLifted()")]
    pub struct ShutdownLiftedFilter();
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
    #[ethevent(name = "Sweep", abi = "Sweep(address,address,uint256)")]
    pub struct SweepFilter {
        #[ethevent(indexed)]
        pub token: ethers::core::types::Address,
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
    #[ethevent(name = "TrustChanged", abi = "TrustChanged(address,bool)")]
    pub struct TrustChangedFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub trusted: bool,
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
        name = "WithdrawFromPosition",
        abi = "WithdrawFromPosition(address,uint256)"
    )]
    pub struct WithdrawFromPositionFilter {
        #[ethevent(indexed)]
        pub position: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveV2StablecoinCellarEvents {
        AccrualFilter(AccrualFilter),
        AccrualPeriodChangedFilter(AccrualPeriodChangedFilter),
        ApprovalFilter(ApprovalFilter),
        ClaimAndUnstakeFilter(ClaimAndUnstakeFilter),
        DepositFilter(DepositFilter),
        DepositIntoPositionFilter(DepositIntoPositionFilter),
        DepositLimitChangedFilter(DepositLimitChangedFilter),
        EnterPositionFilter(EnterPositionFilter),
        ExitPositionFilter(ExitPositionFilter),
        FeesDistributorChangedFilter(FeesDistributorChangedFilter),
        LiquidityLimitChangedFilter(LiquidityLimitChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PerformanceFeeChangedFilter(PerformanceFeeChangedFilter),
        PlatformFeeChangedFilter(PlatformFeeChangedFilter),
        RebalanceFilter(RebalanceFilter),
        ReinvestFilter(ReinvestFilter),
        SendFeesFilter(SendFeesFilter),
        ShutdownInitiatedFilter(ShutdownInitiatedFilter),
        ShutdownLiftedFilter(ShutdownLiftedFilter),
        SweepFilter(SweepFilter),
        TransferFilter(TransferFilter),
        TrustChangedFilter(TrustChangedFilter),
        WithdrawFilter(WithdrawFilter),
        WithdrawFromPositionFilter(WithdrawFromPositionFilter),
    }
    impl ethers::contract::EthLogDecode for AaveV2StablecoinCellarEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AccrualFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::AccrualFilter(decoded));
            }
            if let Ok(decoded) = AccrualPeriodChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::AccrualPeriodChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ClaimAndUnstakeFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ClaimAndUnstakeFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositIntoPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositIntoPositionFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DepositLimitChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositLimitChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EnterPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::EnterPositionFilter(decoded));
            }
            if let Ok(decoded) = ExitPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ExitPositionFilter(decoded));
            }
            if let Ok(decoded) = FeesDistributorChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::FeesDistributorChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = LiquidityLimitChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::LiquidityLimitChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PerformanceFeeChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::PerformanceFeeChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PlatformFeeChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::PlatformFeeChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = ReinvestFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ReinvestFilter(decoded));
            }
            if let Ok(decoded) = SendFeesFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SendFeesFilter(decoded));
            }
            if let Ok(decoded) = ShutdownInitiatedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ShutdownInitiatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ShutdownLiftedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ShutdownLiftedFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SweepFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TrustChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TrustChangedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFromPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::WithdrawFromPositionFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AaveV2StablecoinCellarEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveV2StablecoinCellarEvents::AccrualFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::AccrualPeriodChangedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ApprovalFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ClaimAndUnstakeFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::DepositFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::DepositIntoPositionFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::DepositLimitChangedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::EnterPositionFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ExitPositionFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::FeesDistributorChangedFilter(element) => {
                    element.fmt(f)
                }
                AaveV2StablecoinCellarEvents::LiquidityLimitChangedFilter(element) => {
                    element.fmt(f)
                }
                AaveV2StablecoinCellarEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::PerformanceFeeChangedFilter(element) => {
                    element.fmt(f)
                }
                AaveV2StablecoinCellarEvents::PlatformFeeChangedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::RebalanceFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ReinvestFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::SendFeesFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ShutdownInitiatedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::ShutdownLiftedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::SweepFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::TransferFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::TrustChangedFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::WithdrawFilter(element) => element.fmt(f),
                AaveV2StablecoinCellarEvents::WithdrawFromPositionFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `accrualPeriod`function with signature `accrualPeriod()` and selector `[246, 102, 65, 85]`"]
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
    #[ethcall(name = "accrualPeriod", abi = "accrualPeriod()")]
    pub struct AccrualPeriodCall;
    #[doc = "Container type for all input parameters for the `accrue`function with signature `accrue()` and selector `[248, 186, 76, 255]`"]
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
    #[ethcall(name = "accrue", abi = "accrue()")]
    pub struct AccrueCall;
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
    #[doc = "Container type for all input parameters for the `asset`function with signature `asset()` and selector `[56, 213, 46, 15]`"]
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    #[doc = "Container type for all input parameters for the `assetAToken`function with signature `assetAToken()` and selector `[193, 127, 103, 64]`"]
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
    #[ethcall(name = "assetAToken", abi = "assetAToken()")]
    pub struct AssetATokenCall;
    #[doc = "Container type for all input parameters for the `assetDecimals`function with signature `assetDecimals()` and selector `[194, 212, 22, 1]`"]
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
    #[ethcall(name = "assetDecimals", abi = "assetDecimals()")]
    pub struct AssetDecimalsCall;
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
    #[doc = "Container type for all input parameters for the `curveRegistryExchange`function with signature `curveRegistryExchange()` and selector `[172, 53, 53, 16]`"]
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
    #[ethcall(name = "curveRegistryExchange", abi = "curveRegistryExchange()")]
    pub struct CurveRegistryExchangeCall;
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
    pub struct DepositCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `depositLimit`function with signature `depositLimit()` and selector `[236, 247, 8, 88]`"]
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
    #[ethcall(name = "depositLimit", abi = "depositLimit()")]
    pub struct DepositLimitCall;
    #[doc = "Container type for all input parameters for the `enterPosition`function with signature `enterPosition()` and selector `[61, 198, 234, 191]`"]
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
    #[ethcall(name = "enterPosition", abi = "enterPosition()")]
    pub struct EnterPositionCall;
    #[doc = "Container type for all input parameters for the `enterPosition`function with signature `enterPosition(uint256)` and selector `[110, 8, 64, 107]`"]
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
    #[ethcall(name = "enterPosition", abi = "enterPosition(uint256)")]
    pub struct EnterPositionWithAssetsCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exitPosition`function with signature `exitPosition(uint256)` and selector `[120, 220, 144, 89]`"]
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
    #[ethcall(name = "exitPosition", abi = "exitPosition(uint256)")]
    pub struct ExitPositionWithAssetsCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `exitPosition`function with signature `exitPosition()` and selector `[153, 114, 146, 22]`"]
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
    #[ethcall(name = "exitPosition", abi = "exitPosition()")]
    pub struct ExitPositionCall;
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
    #[doc = "Container type for all input parameters for the `initiateShutdown`function with signature `initiateShutdown(bool)` and selector `[239, 70, 93, 146]`"]
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
    #[ethcall(name = "initiateShutdown", abi = "initiateShutdown(bool)")]
    pub struct InitiateShutdownCall {
        pub empty_position: bool,
    }
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
    #[doc = "Container type for all input parameters for the `isTrusted`function with signature `isTrusted(address)` and selector `[150, 214, 72, 121]`"]
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
    #[ethcall(name = "isTrusted", abi = "isTrusted(address)")]
    pub struct IsTrustedCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `lastAccrual`function with signature `lastAccrual()` and selector `[123, 59, 170, 180]`"]
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
    #[ethcall(name = "lastAccrual", abi = "lastAccrual()")]
    pub struct LastAccrualCall;
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
    #[doc = "Container type for all input parameters for the `liftShutdown`function with signature `liftShutdown()` and selector `[94, 44, 87, 110]`"]
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
    #[ethcall(name = "liftShutdown", abi = "liftShutdown()")]
    pub struct LiftShutdownCall;
    #[doc = "Container type for all input parameters for the `liquidityLimit`function with signature `liquidityLimit()` and selector `[114, 22, 55, 21]`"]
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
    #[ethcall(name = "liquidityLimit", abi = "liquidityLimit()")]
    pub struct LiquidityLimitCall;
    #[doc = "Container type for all input parameters for the `maxDeposit`function with signature `maxDeposit(address)` and selector `[64, 45, 38, 125]`"]
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit(address)")]
    pub struct MaxDepositCall {
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxLocked`function with signature `maxLocked()` and selector `[143, 220, 157, 250]`"]
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
    #[ethcall(name = "maxLocked", abi = "maxLocked()")]
    pub struct MaxLockedCall;
    #[doc = "Container type for all input parameters for the `maxMint`function with signature `maxMint(address)` and selector `[198, 61, 117, 182]`"]
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
    #[ethcall(name = "maxMint", abi = "maxMint(address)")]
    pub struct MaxMintCall {
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxRedeem`function with signature `maxRedeem(address)` and selector `[217, 5, 119, 126]`"]
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
    #[ethcall(name = "maxRedeem", abi = "maxRedeem(address)")]
    pub struct MaxRedeemCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `maxWithdraw`function with signature `maxWithdraw(address)` and selector `[206, 150, 203, 119]`"]
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
    #[ethcall(name = "maxWithdraw", abi = "maxWithdraw(address)")]
    pub struct MaxWithdrawCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256,address)` and selector `[148, 191, 128, 77]`"]
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
    #[ethcall(name = "mint", abi = "mint(uint256,address)")]
    pub struct MintCall {
        pub shares: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `multicall`function with signature `multicall(bytes[])` and selector `[172, 150, 80, 216]`"]
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<ethers::core::types::Bytes>,
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
    #[doc = "Container type for all input parameters for the `platformFee`function with signature `platformFee()` and selector `[38, 35, 42, 46]`"]
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
    #[ethcall(name = "platformFee", abi = "platformFee()")]
    pub struct PlatformFeeCall;
    #[doc = "Container type for all input parameters for the `previewDeposit`function with signature `previewDeposit(uint256)` and selector `[239, 139, 48, 247]`"]
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewMint`function with signature `previewMint(uint256)` and selector `[179, 215, 246, 185]`"]
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
    #[ethcall(name = "previewMint", abi = "previewMint(uint256)")]
    pub struct PreviewMintCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewRedeem`function with signature `previewRedeem(uint256)` and selector `[76, 218, 213, 6]`"]
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
    #[ethcall(name = "previewRedeem", abi = "previewRedeem(uint256)")]
    pub struct PreviewRedeemCall {
        pub shares: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `previewWithdraw`function with signature `previewWithdraw(uint256)` and selector `[10, 40, 164, 119]`"]
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
    #[ethcall(name = "previewWithdraw", abi = "previewWithdraw(uint256)")]
    pub struct PreviewWithdrawCall {
        pub assets: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `rebalance`function with signature `rebalance(address[9],uint256[3][4],uint256)` and selector `[21, 244, 198, 17]`"]
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
        abi = "rebalance(address[9],uint256[3][4],uint256)"
    )]
    pub struct RebalanceCall {
        pub route: [ethers::core::types::Address; 9usize],
        pub swap_params: [[ethers::core::types::U256; 3usize]; 4usize],
        pub min_assets_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `redeem`function with signature `redeem(uint256,address,address)` and selector `[186, 8, 118, 82]`"]
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `sendFees`function with signature `sendFees()` and selector `[223, 249, 11, 91]`"]
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
    #[ethcall(name = "sendFees", abi = "sendFees()")]
    pub struct SendFeesCall;
    #[doc = "Container type for all input parameters for the `setAccrualPeriod`function with signature `setAccrualPeriod(uint32)` and selector `[239, 122, 200, 131]`"]
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
    #[ethcall(name = "setAccrualPeriod", abi = "setAccrualPeriod(uint32)")]
    pub struct SetAccrualPeriodCall {
        pub new_accrual_period: u32,
    }
    #[doc = "Container type for all input parameters for the `setDepositLimit`function with signature `setDepositLimit(uint256)` and selector `[189, 200, 20, 75]`"]
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
    #[ethcall(name = "setDepositLimit", abi = "setDepositLimit(uint256)")]
    pub struct SetDepositLimitCall {
        pub new_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setFeesDistributor`function with signature `setFeesDistributor(bytes32)` and selector `[110, 133, 241, 131]`"]
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
    #[ethcall(name = "setFeesDistributor", abi = "setFeesDistributor(bytes32)")]
    pub struct SetFeesDistributorCall {
        pub new_fees_distributor: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setLiquidityLimit`function with signature `setLiquidityLimit(uint256)` and selector `[223, 5, 165, 42]`"]
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
    #[ethcall(name = "setLiquidityLimit", abi = "setLiquidityLimit(uint256)")]
    pub struct SetLiquidityLimitCall {
        pub new_limit: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setTrust`function with signature `setTrust(address,bool)` and selector `[202, 181, 146, 56]`"]
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
    #[ethcall(name = "setTrust", abi = "setTrust(address,bool)")]
    pub struct SetTrustCall {
        pub position: ethers::core::types::Address,
        pub trust: bool,
    }
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
    #[doc = "Container type for all input parameters for the `sushiswapRouter`function with signature `sushiswapRouter()` and selector `[233, 36, 12, 45]`"]
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
    #[ethcall(name = "sushiswapRouter", abi = "sushiswapRouter()")]
    pub struct SushiswapRouterCall;
    #[doc = "Container type for all input parameters for the `sweep`function with signature `sweep(address,address)` and selector `[184, 220, 73, 27]`"]
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
    #[ethcall(name = "sweep", abi = "sweep(address,address)")]
    pub struct SweepCall {
        pub token: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `totalBalance`function with signature `totalBalance()` and selector `[173, 122, 103, 47]`"]
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
    #[ethcall(name = "totalBalance", abi = "totalBalance()")]
    pub struct TotalBalanceCall;
    #[doc = "Container type for all input parameters for the `totalHoldings`function with signature `totalHoldings()` and selector `[233, 236, 46, 153]`"]
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
    #[ethcall(name = "totalHoldings", abi = "totalHoldings()")]
    pub struct TotalHoldingsCall;
    #[doc = "Container type for all input parameters for the `totalLocked`function with signature `totalLocked()` and selector `[86, 137, 20, 18]`"]
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
    #[ethcall(name = "totalLocked", abi = "totalLocked()")]
    pub struct TotalLockedCall;
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
    pub struct WithdrawCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveV2StablecoinCellarCalls {
        Aave(AaveCall),
        DomainSeparator(DomainSeparatorCall),
        Weth(WethCall),
        AccrualPeriod(AccrualPeriodCall),
        Accrue(AccrueCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        AssetAToken(AssetATokenCall),
        AssetDecimals(AssetDecimalsCall),
        BalanceOf(BalanceOfCall),
        ClaimAndUnstake(ClaimAndUnstakeCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CurveRegistryExchange(CurveRegistryExchangeCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        DepositLimit(DepositLimitCall),
        EnterPosition(EnterPositionCall),
        EnterPositionWithAssets(EnterPositionWithAssetsCall),
        ExitPositionWithAssets(ExitPositionWithAssetsCall),
        ExitPosition(ExitPositionCall),
        FeesDistributor(FeesDistributorCall),
        GravityBridge(GravityBridgeCall),
        IncentivesController(IncentivesControllerCall),
        InitiateShutdown(InitiateShutdownCall),
        IsShutdown(IsShutdownCall),
        IsTrusted(IsTrustedCall),
        LastAccrual(LastAccrualCall),
        LendingPool(LendingPoolCall),
        LiftShutdown(LiftShutdownCall),
        LiquidityLimit(LiquidityLimitCall),
        MaxDeposit(MaxDepositCall),
        MaxLocked(MaxLockedCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        PerformanceFee(PerformanceFeeCall),
        Permit(PermitCall),
        PlatformFee(PlatformFeeCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        Rebalance(RebalanceCall),
        Redeem(RedeemCall),
        Reinvest(ReinvestCall),
        RenounceOwnership(RenounceOwnershipCall),
        SendFees(SendFeesCall),
        SetAccrualPeriod(SetAccrualPeriodCall),
        SetDepositLimit(SetDepositLimitCall),
        SetFeesDistributor(SetFeesDistributorCall),
        SetLiquidityLimit(SetLiquidityLimitCall),
        SetTrust(SetTrustCall),
        StkAAVE(StkAAVECall),
        SushiswapRouter(SushiswapRouterCall),
        Sweep(SweepCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalBalance(TotalBalanceCall),
        TotalHoldings(TotalHoldingsCall),
        TotalLocked(TotalLockedCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for AaveV2StablecoinCellarCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AaveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Aave(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <AccrualPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AccrualPeriod(decoded));
            }
            if let Ok(decoded) = <AccrueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Accrue(decoded));
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
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Asset(decoded));
            }
            if let Ok(decoded) =
                <AssetATokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AssetAToken(decoded));
            }
            if let Ok(decoded) =
                <AssetDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::AssetDecimals(decoded));
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
                <CurveRegistryExchangeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::CurveRegistryExchange(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::DepositLimit(decoded));
            }
            if let Ok(decoded) =
                <EnterPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::EnterPosition(decoded));
            }
            if let Ok(decoded) =
                <EnterPositionWithAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::EnterPositionWithAssets(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ExitPositionWithAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ExitPositionWithAssets(decoded));
            }
            if let Ok(decoded) =
                <ExitPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::ExitPosition(decoded));
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
                <IncentivesControllerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IncentivesController(decoded));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::InitiateShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsTrustedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::IsTrusted(decoded));
            }
            if let Ok(decoded) =
                <LastAccrualCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LastAccrual(decoded));
            }
            if let Ok(decoded) =
                <LendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LendingPool(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LiftShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::LiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxLockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxLocked(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveV2StablecoinCellarCalls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Multicall(decoded));
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
            if let Ok(decoded) =
                <PerformanceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PerformanceFee(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PlatformFee(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <RebalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Rebalance(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <ReinvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Reinvest(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SendFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SendFees(decoded));
            }
            if let Ok(decoded) =
                <SetAccrualPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetAccrualPeriod(decoded));
            }
            if let Ok(decoded) =
                <SetDepositLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetDepositLimit(decoded));
            }
            if let Ok(decoded) =
                <SetFeesDistributorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetFeesDistributor(decoded));
            }
            if let Ok(decoded) =
                <SetLiquidityLimitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetLiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <SetTrustCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SetTrust(decoded));
            }
            if let Ok(decoded) =
                <StkAAVECall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::StkAAVE(decoded));
            }
            if let Ok(decoded) =
                <SushiswapRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::SushiswapRouter(decoded));
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
                <TotalBalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TotalBalance(decoded));
            }
            if let Ok(decoded) =
                <TotalHoldingsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TotalHoldings(decoded));
            }
            if let Ok(decoded) =
                <TotalLockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::TotalLocked(decoded));
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
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveV2StablecoinCellarCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveV2StablecoinCellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveV2StablecoinCellarCalls::Aave(element) => element.encode(),
                AaveV2StablecoinCellarCalls::DomainSeparator(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Weth(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AccrualPeriod(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Accrue(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Allowance(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Approve(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Asset(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AssetAToken(element) => element.encode(),
                AaveV2StablecoinCellarCalls::AssetDecimals(element) => element.encode(),
                AaveV2StablecoinCellarCalls::BalanceOf(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ClaimAndUnstake(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ConvertToAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ConvertToShares(element) => element.encode(),
                AaveV2StablecoinCellarCalls::CurveRegistryExchange(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Decimals(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Deposit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::DepositLimit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::EnterPosition(element) => element.encode(),
                AaveV2StablecoinCellarCalls::EnterPositionWithAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ExitPositionWithAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::ExitPosition(element) => element.encode(),
                AaveV2StablecoinCellarCalls::FeesDistributor(element) => element.encode(),
                AaveV2StablecoinCellarCalls::GravityBridge(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IncentivesController(element) => element.encode(),
                AaveV2StablecoinCellarCalls::InitiateShutdown(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IsShutdown(element) => element.encode(),
                AaveV2StablecoinCellarCalls::IsTrusted(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LastAccrual(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LendingPool(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LiftShutdown(element) => element.encode(),
                AaveV2StablecoinCellarCalls::LiquidityLimit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxDeposit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxLocked(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxMint(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxRedeem(element) => element.encode(),
                AaveV2StablecoinCellarCalls::MaxWithdraw(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Mint(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Multicall(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Name(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Nonces(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Owner(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PerformanceFee(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Permit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PlatformFee(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PreviewDeposit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PreviewMint(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PreviewRedeem(element) => element.encode(),
                AaveV2StablecoinCellarCalls::PreviewWithdraw(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Rebalance(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Redeem(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Reinvest(element) => element.encode(),
                AaveV2StablecoinCellarCalls::RenounceOwnership(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SendFees(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetAccrualPeriod(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetDepositLimit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetFeesDistributor(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetLiquidityLimit(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SetTrust(element) => element.encode(),
                AaveV2StablecoinCellarCalls::StkAAVE(element) => element.encode(),
                AaveV2StablecoinCellarCalls::SushiswapRouter(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Sweep(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Symbol(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalAssets(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalBalance(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalHoldings(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalLocked(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TotalSupply(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Transfer(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TransferFrom(element) => element.encode(),
                AaveV2StablecoinCellarCalls::TransferOwnership(element) => element.encode(),
                AaveV2StablecoinCellarCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveV2StablecoinCellarCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveV2StablecoinCellarCalls::Aave(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::DomainSeparator(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Weth(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AccrualPeriod(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Accrue(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Allowance(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Approve(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Asset(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AssetAToken(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::AssetDecimals(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::BalanceOf(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ClaimAndUnstake(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ConvertToAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ConvertToShares(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::CurveRegistryExchange(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Decimals(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Deposit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::DepositLimit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::EnterPosition(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::EnterPositionWithAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ExitPositionWithAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::ExitPosition(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::FeesDistributor(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::GravityBridge(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IncentivesController(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::InitiateShutdown(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IsShutdown(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::IsTrusted(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LastAccrual(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LendingPool(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LiftShutdown(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::LiquidityLimit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxDeposit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxLocked(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxMint(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxRedeem(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::MaxWithdraw(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Mint(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Multicall(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Name(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Nonces(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Owner(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PerformanceFee(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Permit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PlatformFee(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PreviewDeposit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PreviewMint(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PreviewRedeem(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::PreviewWithdraw(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Rebalance(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Redeem(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Reinvest(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::RenounceOwnership(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SendFees(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetAccrualPeriod(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetDepositLimit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetFeesDistributor(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetLiquidityLimit(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SetTrust(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::StkAAVE(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::SushiswapRouter(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Sweep(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Symbol(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalAssets(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalBalance(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalHoldings(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalLocked(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TotalSupply(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Transfer(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TransferFrom(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::TransferOwnership(element) => element.fmt(f),
                AaveV2StablecoinCellarCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AaveCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AaveCall) -> Self {
            AaveV2StablecoinCellarCalls::Aave(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DomainSeparatorCall) -> Self {
            AaveV2StablecoinCellarCalls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<WethCall> for AaveV2StablecoinCellarCalls {
        fn from(var: WethCall) -> Self {
            AaveV2StablecoinCellarCalls::Weth(var)
        }
    }
    impl ::std::convert::From<AccrualPeriodCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AccrualPeriodCall) -> Self {
            AaveV2StablecoinCellarCalls::AccrualPeriod(var)
        }
    }
    impl ::std::convert::From<AccrueCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AccrueCall) -> Self {
            AaveV2StablecoinCellarCalls::Accrue(var)
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
    impl ::std::convert::From<AssetCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AssetCall) -> Self {
            AaveV2StablecoinCellarCalls::Asset(var)
        }
    }
    impl ::std::convert::From<AssetATokenCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AssetATokenCall) -> Self {
            AaveV2StablecoinCellarCalls::AssetAToken(var)
        }
    }
    impl ::std::convert::From<AssetDecimalsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: AssetDecimalsCall) -> Self {
            AaveV2StablecoinCellarCalls::AssetDecimals(var)
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
    impl ::std::convert::From<CurveRegistryExchangeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: CurveRegistryExchangeCall) -> Self {
            AaveV2StablecoinCellarCalls::CurveRegistryExchange(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DecimalsCall) -> Self {
            AaveV2StablecoinCellarCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DepositCall) -> Self {
            AaveV2StablecoinCellarCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: DepositLimitCall) -> Self {
            AaveV2StablecoinCellarCalls::DepositLimit(var)
        }
    }
    impl ::std::convert::From<EnterPositionCall> for AaveV2StablecoinCellarCalls {
        fn from(var: EnterPositionCall) -> Self {
            AaveV2StablecoinCellarCalls::EnterPosition(var)
        }
    }
    impl ::std::convert::From<EnterPositionWithAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: EnterPositionWithAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::EnterPositionWithAssets(var)
        }
    }
    impl ::std::convert::From<ExitPositionWithAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ExitPositionWithAssetsCall) -> Self {
            AaveV2StablecoinCellarCalls::ExitPositionWithAssets(var)
        }
    }
    impl ::std::convert::From<ExitPositionCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ExitPositionCall) -> Self {
            AaveV2StablecoinCellarCalls::ExitPosition(var)
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
    impl ::std::convert::From<IncentivesControllerCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IncentivesControllerCall) -> Self {
            AaveV2StablecoinCellarCalls::IncentivesController(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(var: InitiateShutdownCall) -> Self {
            AaveV2StablecoinCellarCalls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IsShutdownCall) -> Self {
            AaveV2StablecoinCellarCalls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<IsTrustedCall> for AaveV2StablecoinCellarCalls {
        fn from(var: IsTrustedCall) -> Self {
            AaveV2StablecoinCellarCalls::IsTrusted(var)
        }
    }
    impl ::std::convert::From<LastAccrualCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LastAccrualCall) -> Self {
            AaveV2StablecoinCellarCalls::LastAccrual(var)
        }
    }
    impl ::std::convert::From<LendingPoolCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LendingPoolCall) -> Self {
            AaveV2StablecoinCellarCalls::LendingPool(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LiftShutdownCall) -> Self {
            AaveV2StablecoinCellarCalls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LiquidityLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: LiquidityLimitCall) -> Self {
            AaveV2StablecoinCellarCalls::LiquidityLimit(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxDepositCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxLockedCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxLockedCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxLocked(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxMintCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxRedeemCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MaxWithdrawCall) -> Self {
            AaveV2StablecoinCellarCalls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MintCall) -> Self {
            AaveV2StablecoinCellarCalls::Mint(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for AaveV2StablecoinCellarCalls {
        fn from(var: MulticallCall) -> Self {
            AaveV2StablecoinCellarCalls::Multicall(var)
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
    impl ::std::convert::From<PerformanceFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PerformanceFeeCall) -> Self {
            AaveV2StablecoinCellarCalls::PerformanceFee(var)
        }
    }
    impl ::std::convert::From<PermitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PermitCall) -> Self {
            AaveV2StablecoinCellarCalls::Permit(var)
        }
    }
    impl ::std::convert::From<PlatformFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PlatformFeeCall) -> Self {
            AaveV2StablecoinCellarCalls::PlatformFee(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PreviewDepositCall) -> Self {
            AaveV2StablecoinCellarCalls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PreviewMintCall) -> Self {
            AaveV2StablecoinCellarCalls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PreviewRedeemCall) -> Self {
            AaveV2StablecoinCellarCalls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(var: PreviewWithdrawCall) -> Self {
            AaveV2StablecoinCellarCalls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<RebalanceCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RebalanceCall) -> Self {
            AaveV2StablecoinCellarCalls::Rebalance(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RedeemCall) -> Self {
            AaveV2StablecoinCellarCalls::Redeem(var)
        }
    }
    impl ::std::convert::From<ReinvestCall> for AaveV2StablecoinCellarCalls {
        fn from(var: ReinvestCall) -> Self {
            AaveV2StablecoinCellarCalls::Reinvest(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for AaveV2StablecoinCellarCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            AaveV2StablecoinCellarCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SendFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SendFeesCall) -> Self {
            AaveV2StablecoinCellarCalls::SendFees(var)
        }
    }
    impl ::std::convert::From<SetAccrualPeriodCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetAccrualPeriodCall) -> Self {
            AaveV2StablecoinCellarCalls::SetAccrualPeriod(var)
        }
    }
    impl ::std::convert::From<SetDepositLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetDepositLimitCall) -> Self {
            AaveV2StablecoinCellarCalls::SetDepositLimit(var)
        }
    }
    impl ::std::convert::From<SetFeesDistributorCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetFeesDistributorCall) -> Self {
            AaveV2StablecoinCellarCalls::SetFeesDistributor(var)
        }
    }
    impl ::std::convert::From<SetLiquidityLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetLiquidityLimitCall) -> Self {
            AaveV2StablecoinCellarCalls::SetLiquidityLimit(var)
        }
    }
    impl ::std::convert::From<SetTrustCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SetTrustCall) -> Self {
            AaveV2StablecoinCellarCalls::SetTrust(var)
        }
    }
    impl ::std::convert::From<StkAAVECall> for AaveV2StablecoinCellarCalls {
        fn from(var: StkAAVECall) -> Self {
            AaveV2StablecoinCellarCalls::StkAAVE(var)
        }
    }
    impl ::std::convert::From<SushiswapRouterCall> for AaveV2StablecoinCellarCalls {
        fn from(var: SushiswapRouterCall) -> Self {
            AaveV2StablecoinCellarCalls::SushiswapRouter(var)
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
    impl ::std::convert::From<TotalBalanceCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TotalBalanceCall) -> Self {
            AaveV2StablecoinCellarCalls::TotalBalance(var)
        }
    }
    impl ::std::convert::From<TotalHoldingsCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TotalHoldingsCall) -> Self {
            AaveV2StablecoinCellarCalls::TotalHoldings(var)
        }
    }
    impl ::std::convert::From<TotalLockedCall> for AaveV2StablecoinCellarCalls {
        fn from(var: TotalLockedCall) -> Self {
            AaveV2StablecoinCellarCalls::TotalLocked(var)
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
    impl ::std::convert::From<WithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(var: WithdrawCall) -> Self {
            AaveV2StablecoinCellarCalls::Withdraw(var)
        }
    }
}
