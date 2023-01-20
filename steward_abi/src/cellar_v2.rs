pub use cellarv2_mod::*;
#[allow(clippy::too_many_arguments)]
mod cellarv2_mod {
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
    #[doc = "CellarV2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CELLARV2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract Registry\",\n                \"name\": \"_registry\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__AdaptorNotSetUp\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"expectedAsset\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__AssetMismatch\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__CallerNotAavePool\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ContractNotShutdown\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ContractShutdown\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__DebtMismatch\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"maxDeposit\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__DepositRestricted\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ExternalInitiator\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"illiquidPosition\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__IlliquidWithdraw\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assetsOwed\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__IncompleteWithdraw\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidFee\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidFeeCut\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__InvalidHoldingPosition\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"requested\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"max\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__InvalidRebalanceDeviation\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidShareLockPeriod\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"depositor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__NotApprovedToDepositOnBehalf\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__PayoutNotSet\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__PositionAlreadyUsed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"maxPositions\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__PositionArrayFull\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"sharesRemaining\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__PositionNotEmpty\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__PositionNotUsed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__RemovingHoldingPosition\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"timeSharesAreUnlocked\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"currentBlock\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__SharesAreLocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"max\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__TotalAssetDeviatedOutsideRange\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"current\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"expected\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__TotalSharesMustRemainConstant\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ZeroAssets\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ZeroShares\",\n        \"type\": \"error\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Approval\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"caller\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Deposit\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint8\",\n                \"name\": \"version\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"name\": \"Initialized\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"user\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"OwnershipTransferred\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"oldPlatformFee\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"newPlatformFee\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"PlatformFeeChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionAdded\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionRemoved\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"newPosition1\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"newPosition2\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index2\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionSwapped\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"oldDeviation\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"newDeviation\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"RebalanceDeviationChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"feesInSharesRedeemed\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"feesInAssetsSent\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"SendFees\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"oldPeriod\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"newPeriod\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"ShareLockingPeriodChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isShutdown\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"ShutdownChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"oldPayoutAddress\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"newPayoutAddress\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"StrategistPayoutAddressChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"oldPlatformCut\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"newPlatformCut\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"StrategistPlatformCutChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Transfer\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"caller\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Withdraw\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"DOMAIN_SEPARATOR\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"GRAVITY_BRIDGE_REGISTRY_SLOT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_FEE_CUT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_PLATFORM_FEE\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_POSITIONS\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_REBALANCE_DEVIATION\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MINIMUM_SHARE_LOCK_PERIOD\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"PRICE_ROUTER_REGISTRY_SLOT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"aavePool\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"addPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"allowance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"allowedRebalanceDeviation\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"asset\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"assetRiskTolerance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"blockExternalReceiver\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"adaptor\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"bytes[]\",\n                        \"name\": \"callData\",\n                        \"type\": \"bytes[]\"\n                    }\n                ],\n                \"internalType\": \"struct Cellar.AdaptorCall[]\",\n                \"name\": \"data\",\n                \"type\": \"tuple[]\"\n            }\n        ],\n        \"name\": \"callOnAdaptor\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"convertToAssets\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"convertToShares\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"creditPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"debtPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"decimals\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"premiums\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"initiator\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"executeOperation\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeData\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"strategistPlatformCut\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"platformFee\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"lastAccrual\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"strategistPayoutAddress\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getCreditPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32[]\",\n                \"name\": \"\",\n                \"type\": \"uint32[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getDebtPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32[]\",\n                \"name\": \"\",\n                \"type\": \"uint32[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getPositionAssets\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"getPositionData\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"isDebt\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"holdingPosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"initialize\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"initiateShutdown\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"isAdaptorSetup\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"isPositionUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isShutdown\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"liftShutdown\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"locked\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxDeposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxMint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxRedeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxWithdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"mint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"name\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"nonces\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"onERC721Received\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"owner\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"permit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewDeposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewMint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewRedeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewWithdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"protocolRiskTolerance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"redeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"registry\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract Registry\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"removePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"sendFees\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"setHoldingPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"newPlatformFee\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"setPlatformFee\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"newDeviation\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setRebalanceDeviation\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"newLock\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setShareLockPeriod\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"payout\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setStrategistPayoutAddress\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"cut\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"setStrategistPlatformCut\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_adaptor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setupAdaptor\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"shareLockPeriod\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index1\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index2\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"swapPositions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"symbol\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalAssets\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalAssetsWithdrawable\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalSupply\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transferFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"transferOwnership\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"userShareLockStartTime\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CellarV2<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CellarV2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CellarV2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CellarV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CellarV2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CELLARV2_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `GRAVITY_BRIDGE_REGISTRY_SLOT` (0xcd82f8b1) function"]
        pub fn gravity_bridge_registry_slot(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([205, 130, 248, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAXIMUM_SHARE_LOCK_PERIOD` (0x0402ab63) function"]
        pub fn maximum_share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([4, 2, 171, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_FEE_CUT` (0xeef33eca) function"]
        pub fn max_fee_cut(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([238, 243, 62, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_PLATFORM_FEE` (0x3998a681) function"]
        pub fn max_platform_fee(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([57, 152, 166, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_POSITIONS` (0xf7b24e08) function"]
        pub fn max_positions(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([247, 178, 78, 8], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_REBALANCE_DEVIATION` (0x6ff1c02a) function"]
        pub fn max_rebalance_deviation(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([111, 241, 192, 42], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MINIMUM_SHARE_LOCK_PERIOD` (0x0051a3b7) function"]
        pub fn minimum_share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([0, 81, 163, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PRICE_ROUTER_REGISTRY_SLOT` (0x5a400d25) function"]
        pub fn price_router_registry_slot(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([90, 64, 13, 37], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aavePool` (0xa03e4bc3) function"]
        pub fn aave_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([160, 62, 75, 195], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addPosition` (0x9955a9d4) function"]
        pub fn add_position(
            &self,
            index: u32,
            position_id: u32,
            configuration_data: ethers::core::types::Bytes,
            in_debt_array: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 85, 169, 212],
                    (index, position_id, configuration_data, in_debt_array),
                )
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
        #[doc = "Calls the contract's `allowedRebalanceDeviation` (0xc244245a) function"]
        pub fn allowed_rebalance_deviation(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([194, 68, 36, 90], ())
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
        #[doc = "Calls the contract's `assetRiskTolerance` (0x9babf9fd) function"]
        pub fn asset_risk_tolerance(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([155, 171, 249, 253], ())
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
        #[doc = "Calls the contract's `blockExternalReceiver` (0x4c4602da) function"]
        pub fn block_external_receiver(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 70, 2, 218], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `callOnAdaptor` (0x4e84befe) function"]
        pub fn call_on_adaptor(
            &self,
            data: ::std::vec::Vec<AdaptorCall>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 132, 190, 254], data)
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
        #[doc = "Calls the contract's `creditPositions` (0x59d20b4e) function"]
        pub fn credit_positions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([89, 210, 11, 78], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debtPositions` (0xe1b1acb7) function"]
        pub fn debt_positions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([225, 177, 172, 183], p0)
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
        #[doc = "Calls the contract's `executeOperation` (0x920f5c84) function"]
        pub fn execute_operation(
            &self,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            premiums: ::std::vec::Vec<ethers::core::types::U256>,
            initiator: ethers::core::types::Address,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [146, 15, 92, 132],
                    (assets, amounts, premiums, initiator, params),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeData` (0xe753e600) function"]
        pub fn fee_data(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (u64, u64, u64, ethers::core::types::Address),
        > {
            self.0
                .method_hash([231, 83, 230, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCreditPositions` (0x71e99dc2) function"]
        pub fn get_credit_positions(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([113, 233, 157, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDebtPositions` (0x3e3382ba) function"]
        pub fn get_debt_positions(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([62, 51, 130, 186], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionAssets` (0x087ed837) function"]
        pub fn get_position_assets(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([8, 126, 216, 55], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPositionData` (0x7384504f) function"]
        pub fn get_position_data(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                bool,
                ethers::core::types::Bytes,
                ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([115, 132, 80, 79], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `holdingPosition` (0x9c5f00c2) function"]
        pub fn holding_position(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([156, 95, 0, 194], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x439fab91) function"]
        pub fn initialize(
            &self,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 159, 171, 145], params)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateShutdown` (0x0a680e18) function"]
        pub fn initiate_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 104, 14, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isAdaptorSetup` (0x709ffa18) function"]
        pub fn is_adaptor_setup(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([112, 159, 250, 24], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPositionUsed` (0x93bbeac0) function"]
        pub fn is_position_used(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([147, 187, 234, 192], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isShutdown` (0xbf86d690) function"]
        pub fn is_shutdown(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liftShutdown` (0x5e2c576e) function"]
        pub fn lift_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 44, 87, 110], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `locked` (0xcf309012) function"]
        pub fn locked(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([207, 48, 144, 18], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxDeposit` (0x402d267d) function"]
        pub fn max_deposit(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maxMint` (0xc63d75b6) function"]
        pub fn max_mint(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], p0)
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
        #[doc = "Calls the contract's `onERC721Received` (0x150b7a02) function"]
        pub fn on_erc721_received(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::U256,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
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
        #[doc = "Calls the contract's `protocolRiskTolerance` (0xb2c69eed) function"]
        pub fn protocol_risk_tolerance(&self) -> ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([178, 198, 158, 237], ())
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
        #[doc = "Calls the contract's `registry` (0x7b103999) function"]
        pub fn registry(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removePosition` (0x33e15be2) function"]
        pub fn remove_position(
            &self,
            index: u32,
            in_debt_array: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 225, 91, 226], (index, in_debt_array))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sendFees` (0xdff90b5b) function"]
        pub fn send_fees(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setHoldingPosition` (0x0780fd3a) function"]
        pub fn set_holding_position(
            &self,
            position_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 128, 253, 58], position_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPlatformFee` (0x70af7df6) function"]
        pub fn set_platform_fee(
            &self,
            new_platform_fee: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 175, 125, 246], new_platform_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setRebalanceDeviation` (0x530a3714) function"]
        pub fn set_rebalance_deviation(
            &self,
            new_deviation: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 10, 55, 20], new_deviation)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setShareLockPeriod` (0x9c552ca8) function"]
        pub fn set_share_lock_period(
            &self,
            new_lock: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 85, 44, 168], new_lock)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategistPayoutAddress` (0xb0a75d36) function"]
        pub fn set_strategist_payout_address(
            &self,
            payout: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 167, 93, 54], payout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setStrategistPlatformCut` (0xb5292a99) function"]
        pub fn set_strategist_platform_cut(
            &self,
            cut: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 41, 42, 153], cut)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setupAdaptor` (0x23689f05) function"]
        pub fn setup_adaptor(
            &self,
            adaptor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 104, 159, 5], adaptor)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shareLockPeriod` (0x9fdb11b6) function"]
        pub fn share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 219, 17, 182], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapPositions` (0x379e0b13) function"]
        pub fn swap_positions(
            &self,
            index_1: u32,
            index_2: u32,
            in_debt_array: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 158, 11, 19], (index_1, index_2, in_debt_array))
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
        #[doc = "Calls the contract's `totalAssetsWithdrawable` (0xa8144e48) function"]
        pub fn total_assets_withdrawable(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([168, 20, 78, 72], ())
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
        #[doc = "Calls the contract's `userShareLockStartTime` (0x687c2b50) function"]
        pub fn user_share_lock_start_time(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([104, 124, 43, 80], p0)
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
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PlatformFeeChanged` event"]
        pub fn platform_fee_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PlatformFeeChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionAdded` event"]
        pub fn position_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionRemoved` event"]
        pub fn position_removed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionRemovedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionSwapped` event"]
        pub fn position_swapped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionSwappedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RebalanceDeviationChanged` event"]
        pub fn rebalance_deviation_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RebalanceDeviationChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SendFees` event"]
        pub fn send_fees_filter(&self) -> ethers::contract::builders::Event<M, SendFeesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShareLockingPeriodChanged` event"]
        pub fn share_locking_period_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShareLockingPeriodChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ShutdownChanged` event"]
        pub fn shutdown_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ShutdownChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StrategistPayoutAddressChanged` event"]
        pub fn strategist_payout_address_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StrategistPayoutAddressChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `StrategistPlatformCutChanged` event"]
        pub fn strategist_platform_cut_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, StrategistPlatformCutChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, CellarV2Events> {
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
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
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
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "PositionAdded", abi = "PositionAdded(uint32,uint256)")]
    pub struct PositionAddedFilter {
        pub position: u32,
        pub index: ethers::core::types::U256,
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
    #[ethevent(name = "PositionRemoved", abi = "PositionRemoved(uint32,uint256)")]
    pub struct PositionRemovedFilter {
        pub position: u32,
        pub index: ethers::core::types::U256,
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
        name = "PositionSwapped",
        abi = "PositionSwapped(uint32,uint32,uint256,uint256)"
    )]
    pub struct PositionSwappedFilter {
        pub new_position_1: u32,
        pub new_position_2: u32,
        pub index_1: ethers::core::types::U256,
        pub index_2: ethers::core::types::U256,
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
        name = "RebalanceDeviationChanged",
        abi = "RebalanceDeviationChanged(uint256,uint256)"
    )]
    pub struct RebalanceDeviationChangedFilter {
        pub old_deviation: ethers::core::types::U256,
        pub new_deviation: ethers::core::types::U256,
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
    #[ethevent(
        name = "ShareLockingPeriodChanged",
        abi = "ShareLockingPeriodChanged(uint256,uint256)"
    )]
    pub struct ShareLockingPeriodChangedFilter {
        pub old_period: ethers::core::types::U256,
        pub new_period: ethers::core::types::U256,
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
    #[ethevent(name = "ShutdownChanged", abi = "ShutdownChanged(bool)")]
    pub struct ShutdownChangedFilter {
        pub is_shutdown: bool,
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
        name = "StrategistPayoutAddressChanged",
        abi = "StrategistPayoutAddressChanged(address,address)"
    )]
    pub struct StrategistPayoutAddressChangedFilter {
        pub old_payout_address: ethers::core::types::Address,
        pub new_payout_address: ethers::core::types::Address,
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
        name = "StrategistPlatformCutChanged",
        abi = "StrategistPlatformCutChanged(uint64,uint64)"
    )]
    pub struct StrategistPlatformCutChangedFilter {
        pub old_platform_cut: u64,
        pub new_platform_cut: u64,
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
    pub enum CellarV2Events {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PlatformFeeChangedFilter(PlatformFeeChangedFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        SendFeesFilter(SendFeesFilter),
        ShareLockingPeriodChangedFilter(ShareLockingPeriodChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for CellarV2Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarV2Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarV2Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(CellarV2Events::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CellarV2Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PlatformFeeChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::PlatformFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarV2Events::PositionAddedFilter(decoded));
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarV2Events::PositionRemovedFilter(decoded));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarV2Events::PositionSwappedFilter(decoded));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::RebalanceDeviationChangedFilter(decoded));
            }
            if let Ok(decoded) = SendFeesFilter::decode_log(log) {
                return Ok(CellarV2Events::SendFeesFilter(decoded));
            }
            if let Ok(decoded) = ShareLockingPeriodChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::ShareLockingPeriodChangedFilter(decoded));
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::ShutdownChangedFilter(decoded));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::StrategistPayoutAddressChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(CellarV2Events::StrategistPlatformCutChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarV2Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarV2Events::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CellarV2Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV2Events::ApprovalFilter(element) => element.fmt(f),
                CellarV2Events::DepositFilter(element) => element.fmt(f),
                CellarV2Events::InitializedFilter(element) => element.fmt(f),
                CellarV2Events::OwnershipTransferredFilter(element) => element.fmt(f),
                CellarV2Events::PlatformFeeChangedFilter(element) => element.fmt(f),
                CellarV2Events::PositionAddedFilter(element) => element.fmt(f),
                CellarV2Events::PositionRemovedFilter(element) => element.fmt(f),
                CellarV2Events::PositionSwappedFilter(element) => element.fmt(f),
                CellarV2Events::RebalanceDeviationChangedFilter(element) => element.fmt(f),
                CellarV2Events::SendFeesFilter(element) => element.fmt(f),
                CellarV2Events::ShareLockingPeriodChangedFilter(element) => element.fmt(f),
                CellarV2Events::ShutdownChangedFilter(element) => element.fmt(f),
                CellarV2Events::StrategistPayoutAddressChangedFilter(element) => element.fmt(f),
                CellarV2Events::StrategistPlatformCutChangedFilter(element) => element.fmt(f),
                CellarV2Events::TransferFilter(element) => element.fmt(f),
                CellarV2Events::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
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
    #[doc = "Container type for all input parameters for the `GRAVITY_BRIDGE_REGISTRY_SLOT`function with signature `GRAVITY_BRIDGE_REGISTRY_SLOT()` and selector `[205, 130, 248, 177]`"]
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
        name = "GRAVITY_BRIDGE_REGISTRY_SLOT",
        abi = "GRAVITY_BRIDGE_REGISTRY_SLOT()"
    )]
    pub struct GravityBridgeRegistrySlotCall;
    #[doc = "Container type for all input parameters for the `MAXIMUM_SHARE_LOCK_PERIOD`function with signature `MAXIMUM_SHARE_LOCK_PERIOD()` and selector `[4, 2, 171, 99]`"]
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
        name = "MAXIMUM_SHARE_LOCK_PERIOD",
        abi = "MAXIMUM_SHARE_LOCK_PERIOD()"
    )]
    pub struct MaximumShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `MAX_FEE_CUT`function with signature `MAX_FEE_CUT()` and selector `[238, 243, 62, 202]`"]
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
    #[ethcall(name = "MAX_FEE_CUT", abi = "MAX_FEE_CUT()")]
    pub struct MaxFeeCutCall;
    #[doc = "Container type for all input parameters for the `MAX_PLATFORM_FEE`function with signature `MAX_PLATFORM_FEE()` and selector `[57, 152, 166, 129]`"]
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
    #[ethcall(name = "MAX_PLATFORM_FEE", abi = "MAX_PLATFORM_FEE()")]
    pub struct MaxPlatformFeeCall;
    #[doc = "Container type for all input parameters for the `MAX_POSITIONS`function with signature `MAX_POSITIONS()` and selector `[247, 178, 78, 8]`"]
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
    #[ethcall(name = "MAX_POSITIONS", abi = "MAX_POSITIONS()")]
    pub struct MaxPositionsCall;
    #[doc = "Container type for all input parameters for the `MAX_REBALANCE_DEVIATION`function with signature `MAX_REBALANCE_DEVIATION()` and selector `[111, 241, 192, 42]`"]
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
    #[ethcall(name = "MAX_REBALANCE_DEVIATION", abi = "MAX_REBALANCE_DEVIATION()")]
    pub struct MaxRebalanceDeviationCall;
    #[doc = "Container type for all input parameters for the `MINIMUM_SHARE_LOCK_PERIOD`function with signature `MINIMUM_SHARE_LOCK_PERIOD()` and selector `[0, 81, 163, 183]`"]
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
        name = "MINIMUM_SHARE_LOCK_PERIOD",
        abi = "MINIMUM_SHARE_LOCK_PERIOD()"
    )]
    pub struct MinimumShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `PRICE_ROUTER_REGISTRY_SLOT`function with signature `PRICE_ROUTER_REGISTRY_SLOT()` and selector `[90, 64, 13, 37]`"]
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
        name = "PRICE_ROUTER_REGISTRY_SLOT",
        abi = "PRICE_ROUTER_REGISTRY_SLOT()"
    )]
    pub struct PriceRouterRegistrySlotCall;
    #[doc = "Container type for all input parameters for the `aavePool`function with signature `aavePool()` and selector `[160, 62, 75, 195]`"]
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
    #[ethcall(name = "aavePool", abi = "aavePool()")]
    pub struct AavePoolCall;
    #[doc = "Container type for all input parameters for the `addPosition`function with signature `addPosition(uint32,uint32,bytes,bool)` and selector `[153, 85, 169, 212]`"]
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
    #[ethcall(name = "addPosition", abi = "addPosition(uint32,uint32,bytes,bool)")]
    pub struct AddPositionCall {
        pub index: u32,
        pub position_id: u32,
        pub configuration_data: ethers::core::types::Bytes,
        pub in_debt_array: bool,
    }
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
    #[doc = "Container type for all input parameters for the `allowedRebalanceDeviation`function with signature `allowedRebalanceDeviation()` and selector `[194, 68, 36, 90]`"]
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
        name = "allowedRebalanceDeviation",
        abi = "allowedRebalanceDeviation()"
    )]
    pub struct AllowedRebalanceDeviationCall;
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
    #[doc = "Container type for all input parameters for the `assetRiskTolerance`function with signature `assetRiskTolerance()` and selector `[155, 171, 249, 253]`"]
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
    #[ethcall(name = "assetRiskTolerance", abi = "assetRiskTolerance()")]
    pub struct AssetRiskToleranceCall;
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
    #[doc = "Container type for all input parameters for the `blockExternalReceiver`function with signature `blockExternalReceiver()` and selector `[76, 70, 2, 218]`"]
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
    #[ethcall(name = "blockExternalReceiver", abi = "blockExternalReceiver()")]
    pub struct BlockExternalReceiverCall;
    #[doc = "Container type for all input parameters for the `callOnAdaptor`function with signature `callOnAdaptor((address,bytes[])[])` and selector `[78, 132, 190, 254]`"]
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
    #[ethcall(name = "callOnAdaptor", abi = "callOnAdaptor((address,bytes[])[])")]
    pub struct CallOnAdaptorCall {
        pub data: ::std::vec::Vec<AdaptorCall>,
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
    #[doc = "Container type for all input parameters for the `creditPositions`function with signature `creditPositions(uint256)` and selector `[89, 210, 11, 78]`"]
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
    #[ethcall(name = "creditPositions", abi = "creditPositions(uint256)")]
    pub struct CreditPositionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `debtPositions`function with signature `debtPositions(uint256)` and selector `[225, 177, 172, 183]`"]
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
    #[ethcall(name = "debtPositions", abi = "debtPositions(uint256)")]
    pub struct DebtPositionsCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `executeOperation`function with signature `executeOperation(address[],uint256[],uint256[],address,bytes)` and selector `[146, 15, 92, 132]`"]
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
        name = "executeOperation",
        abi = "executeOperation(address[],uint256[],uint256[],address,bytes)"
    )]
    pub struct ExecuteOperationCall {
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub premiums: ::std::vec::Vec<ethers::core::types::U256>,
        pub initiator: ethers::core::types::Address,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `feeData`function with signature `feeData()` and selector `[231, 83, 230, 0]`"]
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
    #[ethcall(name = "feeData", abi = "feeData()")]
    pub struct FeeDataCall;
    #[doc = "Container type for all input parameters for the `getCreditPositions`function with signature `getCreditPositions()` and selector `[113, 233, 157, 194]`"]
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
    #[ethcall(name = "getCreditPositions", abi = "getCreditPositions()")]
    pub struct GetCreditPositionsCall;
    #[doc = "Container type for all input parameters for the `getDebtPositions`function with signature `getDebtPositions()` and selector `[62, 51, 130, 186]`"]
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
    #[ethcall(name = "getDebtPositions", abi = "getDebtPositions()")]
    pub struct GetDebtPositionsCall;
    #[doc = "Container type for all input parameters for the `getPositionAssets`function with signature `getPositionAssets()` and selector `[8, 126, 216, 55]`"]
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
    #[ethcall(name = "getPositionAssets", abi = "getPositionAssets()")]
    pub struct GetPositionAssetsCall;
    #[doc = "Container type for all input parameters for the `getPositionData`function with signature `getPositionData(uint32)` and selector `[115, 132, 80, 79]`"]
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
    #[ethcall(name = "getPositionData", abi = "getPositionData(uint32)")]
    pub struct GetPositionDataCall(pub u32);
    #[doc = "Container type for all input parameters for the `holdingPosition`function with signature `holdingPosition()` and selector `[156, 95, 0, 194]`"]
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
    #[ethcall(name = "holdingPosition", abi = "holdingPosition()")]
    pub struct HoldingPositionCall;
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(bytes)` and selector `[67, 159, 171, 145]`"]
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
    #[ethcall(name = "initialize", abi = "initialize(bytes)")]
    pub struct InitializeCall {
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `initiateShutdown`function with signature `initiateShutdown()` and selector `[10, 104, 14, 24]`"]
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
    #[ethcall(name = "initiateShutdown", abi = "initiateShutdown()")]
    pub struct InitiateShutdownCall;
    #[doc = "Container type for all input parameters for the `isAdaptorSetup`function with signature `isAdaptorSetup(address)` and selector `[112, 159, 250, 24]`"]
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
    #[ethcall(name = "isAdaptorSetup", abi = "isAdaptorSetup(address)")]
    pub struct IsAdaptorSetupCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `isPositionUsed`function with signature `isPositionUsed(uint256)` and selector `[147, 187, 234, 192]`"]
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
    #[ethcall(name = "isPositionUsed", abi = "isPositionUsed(uint256)")]
    pub struct IsPositionUsedCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `locked`function with signature `locked()` and selector `[207, 48, 144, 18]`"]
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
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
    pub struct MaxDepositCall(pub ethers::core::types::Address);
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
    pub struct MaxMintCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `onERC721Received`function with signature `onERC721Received(address,address,uint256,bytes)` and selector `[21, 11, 122, 2]`"]
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
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
    #[doc = "Container type for all input parameters for the `protocolRiskTolerance`function with signature `protocolRiskTolerance()` and selector `[178, 198, 158, 237]`"]
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
    #[ethcall(name = "protocolRiskTolerance", abi = "protocolRiskTolerance()")]
    pub struct ProtocolRiskToleranceCall;
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
    #[doc = "Container type for all input parameters for the `registry`function with signature `registry()` and selector `[123, 16, 57, 153]`"]
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
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    #[doc = "Container type for all input parameters for the `removePosition`function with signature `removePosition(uint32,bool)` and selector `[51, 225, 91, 226]`"]
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
    #[ethcall(name = "removePosition", abi = "removePosition(uint32,bool)")]
    pub struct RemovePositionCall {
        pub index: u32,
        pub in_debt_array: bool,
    }
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
    #[doc = "Container type for all input parameters for the `setHoldingPosition`function with signature `setHoldingPosition(uint32)` and selector `[7, 128, 253, 58]`"]
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
    #[ethcall(name = "setHoldingPosition", abi = "setHoldingPosition(uint32)")]
    pub struct SetHoldingPositionCall {
        pub position_id: u32,
    }
    #[doc = "Container type for all input parameters for the `setPlatformFee`function with signature `setPlatformFee(uint64)` and selector `[112, 175, 125, 246]`"]
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
    #[ethcall(name = "setPlatformFee", abi = "setPlatformFee(uint64)")]
    pub struct SetPlatformFeeCall {
        pub new_platform_fee: u64,
    }
    #[doc = "Container type for all input parameters for the `setRebalanceDeviation`function with signature `setRebalanceDeviation(uint256)` and selector `[83, 10, 55, 20]`"]
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
    #[ethcall(name = "setRebalanceDeviation", abi = "setRebalanceDeviation(uint256)")]
    pub struct SetRebalanceDeviationCall {
        pub new_deviation: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setShareLockPeriod`function with signature `setShareLockPeriod(uint256)` and selector `[156, 85, 44, 168]`"]
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
    #[ethcall(name = "setShareLockPeriod", abi = "setShareLockPeriod(uint256)")]
    pub struct SetShareLockPeriodCall {
        pub new_lock: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setStrategistPayoutAddress`function with signature `setStrategistPayoutAddress(address)` and selector `[176, 167, 93, 54]`"]
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
        name = "setStrategistPayoutAddress",
        abi = "setStrategistPayoutAddress(address)"
    )]
    pub struct SetStrategistPayoutAddressCall {
        pub payout: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setStrategistPlatformCut`function with signature `setStrategistPlatformCut(uint64)` and selector `[181, 41, 42, 153]`"]
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
        name = "setStrategistPlatformCut",
        abi = "setStrategistPlatformCut(uint64)"
    )]
    pub struct SetStrategistPlatformCutCall {
        pub cut: u64,
    }
    #[doc = "Container type for all input parameters for the `setupAdaptor`function with signature `setupAdaptor(address)` and selector `[35, 104, 159, 5]`"]
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
    #[ethcall(name = "setupAdaptor", abi = "setupAdaptor(address)")]
    pub struct SetupAdaptorCall {
        pub adaptor: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `shareLockPeriod`function with signature `shareLockPeriod()` and selector `[159, 219, 17, 182]`"]
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
    #[ethcall(name = "shareLockPeriod", abi = "shareLockPeriod()")]
    pub struct ShareLockPeriodCall;
    #[doc = "Container type for all input parameters for the `swapPositions`function with signature `swapPositions(uint32,uint32,bool)` and selector `[55, 158, 11, 19]`"]
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
    #[ethcall(name = "swapPositions", abi = "swapPositions(uint32,uint32,bool)")]
    pub struct SwapPositionsCall {
        pub index_1: u32,
        pub index_2: u32,
        pub in_debt_array: bool,
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
    #[doc = "Container type for all input parameters for the `totalAssetsWithdrawable`function with signature `totalAssetsWithdrawable()` and selector `[168, 20, 78, 72]`"]
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
    #[ethcall(name = "totalAssetsWithdrawable", abi = "totalAssetsWithdrawable()")]
    pub struct TotalAssetsWithdrawableCall;
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
    #[doc = "Container type for all input parameters for the `userShareLockStartTime`function with signature `userShareLockStartTime(address)` and selector `[104, 124, 43, 80]`"]
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
        name = "userShareLockStartTime",
        abi = "userShareLockStartTime(address)"
    )]
    pub struct UserShareLockStartTimeCall(pub ethers::core::types::Address);
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
    pub enum CellarV2Calls {
        DomainSeparator(DomainSeparatorCall),
        GravityBridgeRegistrySlot(GravityBridgeRegistrySlotCall),
        MaximumShareLockPeriod(MaximumShareLockPeriodCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        MinimumShareLockPeriod(MinimumShareLockPeriodCall),
        PriceRouterRegistrySlot(PriceRouterRegistrySlotCall),
        AavePool(AavePoolCall),
        AddPosition(AddPositionCall),
        Allowance(AllowanceCall),
        AllowedRebalanceDeviation(AllowedRebalanceDeviationCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        AssetRiskTolerance(AssetRiskToleranceCall),
        BalanceOf(BalanceOfCall),
        BlockExternalReceiver(BlockExternalReceiverCall),
        CallOnAdaptor(CallOnAdaptorCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CreditPositions(CreditPositionsCall),
        DebtPositions(DebtPositionsCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        ExecuteOperation(ExecuteOperationCall),
        FeeData(FeeDataCall),
        GetCreditPositions(GetCreditPositionsCall),
        GetDebtPositions(GetDebtPositionsCall),
        GetPositionAssets(GetPositionAssetsCall),
        GetPositionData(GetPositionDataCall),
        HoldingPosition(HoldingPositionCall),
        Initialize(InitializeCall),
        InitiateShutdown(InitiateShutdownCall),
        IsAdaptorSetup(IsAdaptorSetupCall),
        IsPositionUsed(IsPositionUsedCall),
        IsShutdown(IsShutdownCall),
        LiftShutdown(LiftShutdownCall),
        Locked(LockedCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        ProtocolRiskTolerance(ProtocolRiskToleranceCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemovePosition(RemovePositionCall),
        SendFees(SendFeesCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetPlatformFee(SetPlatformFeeCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetShareLockPeriod(SetShareLockPeriodCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        SetupAdaptor(SetupAdaptorCall),
        ShareLockPeriod(ShareLockPeriodCall),
        SwapPositions(SwapPositionsCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithdrawable(TotalAssetsWithdrawableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        UserShareLockStartTime(UserShareLockStartTimeCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for CellarV2Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <GravityBridgeRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2Calls::GravityBridgeRegistrySlot(decoded));
            }
            if let Ok(decoded) =
                <MaximumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaximumShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxFeeCut(decoded));
            }
            if let Ok(decoded) =
                <MaxPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <MaxPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxPositions(decoded));
            }
            if let Ok(decoded) =
                <MaxRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <MinimumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MinimumShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::PriceRouterRegistrySlot(decoded));
            }
            if let Ok(decoded) =
                <AavePoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::AavePool(decoded));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AllowedRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2Calls::AllowedRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Asset(decoded));
            }
            if let Ok(decoded) =
                <AssetRiskToleranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::AssetRiskTolerance(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BlockExternalReceiverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::BlockExternalReceiver(decoded));
            }
            if let Ok(decoded) =
                <CallOnAdaptorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::CallOnAdaptor(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <CreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::CreditPositions(decoded));
            }
            if let Ok(decoded) =
                <DebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::DebtPositions(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <ExecuteOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::ExecuteOperation(decoded));
            }
            if let Ok(decoded) =
                <FeeDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::FeeData(decoded));
            }
            if let Ok(decoded) =
                <GetCreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::GetCreditPositions(decoded));
            }
            if let Ok(decoded) =
                <GetDebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::GetDebtPositions(decoded));
            }
            if let Ok(decoded) =
                <GetPositionAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::GetPositionAssets(decoded));
            }
            if let Ok(decoded) =
                <GetPositionDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::GetPositionData(decoded));
            }
            if let Ok(decoded) =
                <HoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::HoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::InitiateShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsAdaptorSetupCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::IsAdaptorSetup(decoded));
            }
            if let Ok(decoded) =
                <IsPositionUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::IsPositionUsed(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Locked(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV2Calls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV2Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <ProtocolRiskToleranceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::ProtocolRiskTolerance(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::RemovePosition(decoded));
            }
            if let Ok(decoded) =
                <SendFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SendFees(decoded));
            }
            if let Ok(decoded) =
                <SetHoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SetHoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <SetPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SetPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <SetRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SetRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <SetShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SetShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPayoutAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2Calls::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPlatformCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2Calls::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) =
                <SetupAdaptorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SetupAdaptor(decoded));
            }
            if let Ok(decoded) =
                <ShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::ShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <SwapPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::TotalAssetsWithdrawable(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UserShareLockStartTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::UserShareLockStartTime(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CellarV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CellarV2Calls::DomainSeparator(element) => element.encode(),
                CellarV2Calls::GravityBridgeRegistrySlot(element) => element.encode(),
                CellarV2Calls::MaximumShareLockPeriod(element) => element.encode(),
                CellarV2Calls::MaxFeeCut(element) => element.encode(),
                CellarV2Calls::MaxPlatformFee(element) => element.encode(),
                CellarV2Calls::MaxPositions(element) => element.encode(),
                CellarV2Calls::MaxRebalanceDeviation(element) => element.encode(),
                CellarV2Calls::MinimumShareLockPeriod(element) => element.encode(),
                CellarV2Calls::PriceRouterRegistrySlot(element) => element.encode(),
                CellarV2Calls::AavePool(element) => element.encode(),
                CellarV2Calls::AddPosition(element) => element.encode(),
                CellarV2Calls::Allowance(element) => element.encode(),
                CellarV2Calls::AllowedRebalanceDeviation(element) => element.encode(),
                CellarV2Calls::Approve(element) => element.encode(),
                CellarV2Calls::Asset(element) => element.encode(),
                CellarV2Calls::AssetRiskTolerance(element) => element.encode(),
                CellarV2Calls::BalanceOf(element) => element.encode(),
                CellarV2Calls::BlockExternalReceiver(element) => element.encode(),
                CellarV2Calls::CallOnAdaptor(element) => element.encode(),
                CellarV2Calls::ConvertToAssets(element) => element.encode(),
                CellarV2Calls::ConvertToShares(element) => element.encode(),
                CellarV2Calls::CreditPositions(element) => element.encode(),
                CellarV2Calls::DebtPositions(element) => element.encode(),
                CellarV2Calls::Decimals(element) => element.encode(),
                CellarV2Calls::Deposit(element) => element.encode(),
                CellarV2Calls::ExecuteOperation(element) => element.encode(),
                CellarV2Calls::FeeData(element) => element.encode(),
                CellarV2Calls::GetCreditPositions(element) => element.encode(),
                CellarV2Calls::GetDebtPositions(element) => element.encode(),
                CellarV2Calls::GetPositionAssets(element) => element.encode(),
                CellarV2Calls::GetPositionData(element) => element.encode(),
                CellarV2Calls::HoldingPosition(element) => element.encode(),
                CellarV2Calls::Initialize(element) => element.encode(),
                CellarV2Calls::InitiateShutdown(element) => element.encode(),
                CellarV2Calls::IsAdaptorSetup(element) => element.encode(),
                CellarV2Calls::IsPositionUsed(element) => element.encode(),
                CellarV2Calls::IsShutdown(element) => element.encode(),
                CellarV2Calls::LiftShutdown(element) => element.encode(),
                CellarV2Calls::Locked(element) => element.encode(),
                CellarV2Calls::MaxDeposit(element) => element.encode(),
                CellarV2Calls::MaxMint(element) => element.encode(),
                CellarV2Calls::MaxRedeem(element) => element.encode(),
                CellarV2Calls::MaxWithdraw(element) => element.encode(),
                CellarV2Calls::Mint(element) => element.encode(),
                CellarV2Calls::Name(element) => element.encode(),
                CellarV2Calls::Nonces(element) => element.encode(),
                CellarV2Calls::OnERC721Received(element) => element.encode(),
                CellarV2Calls::Owner(element) => element.encode(),
                CellarV2Calls::Permit(element) => element.encode(),
                CellarV2Calls::PreviewDeposit(element) => element.encode(),
                CellarV2Calls::PreviewMint(element) => element.encode(),
                CellarV2Calls::PreviewRedeem(element) => element.encode(),
                CellarV2Calls::PreviewWithdraw(element) => element.encode(),
                CellarV2Calls::ProtocolRiskTolerance(element) => element.encode(),
                CellarV2Calls::Redeem(element) => element.encode(),
                CellarV2Calls::Registry(element) => element.encode(),
                CellarV2Calls::RemovePosition(element) => element.encode(),
                CellarV2Calls::SendFees(element) => element.encode(),
                CellarV2Calls::SetHoldingPosition(element) => element.encode(),
                CellarV2Calls::SetPlatformFee(element) => element.encode(),
                CellarV2Calls::SetRebalanceDeviation(element) => element.encode(),
                CellarV2Calls::SetShareLockPeriod(element) => element.encode(),
                CellarV2Calls::SetStrategistPayoutAddress(element) => element.encode(),
                CellarV2Calls::SetStrategistPlatformCut(element) => element.encode(),
                CellarV2Calls::SetupAdaptor(element) => element.encode(),
                CellarV2Calls::ShareLockPeriod(element) => element.encode(),
                CellarV2Calls::SwapPositions(element) => element.encode(),
                CellarV2Calls::Symbol(element) => element.encode(),
                CellarV2Calls::TotalAssets(element) => element.encode(),
                CellarV2Calls::TotalAssetsWithdrawable(element) => element.encode(),
                CellarV2Calls::TotalSupply(element) => element.encode(),
                CellarV2Calls::Transfer(element) => element.encode(),
                CellarV2Calls::TransferFrom(element) => element.encode(),
                CellarV2Calls::TransferOwnership(element) => element.encode(),
                CellarV2Calls::UserShareLockStartTime(element) => element.encode(),
                CellarV2Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CellarV2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV2Calls::DomainSeparator(element) => element.fmt(f),
                CellarV2Calls::GravityBridgeRegistrySlot(element) => element.fmt(f),
                CellarV2Calls::MaximumShareLockPeriod(element) => element.fmt(f),
                CellarV2Calls::MaxFeeCut(element) => element.fmt(f),
                CellarV2Calls::MaxPlatformFee(element) => element.fmt(f),
                CellarV2Calls::MaxPositions(element) => element.fmt(f),
                CellarV2Calls::MaxRebalanceDeviation(element) => element.fmt(f),
                CellarV2Calls::MinimumShareLockPeriod(element) => element.fmt(f),
                CellarV2Calls::PriceRouterRegistrySlot(element) => element.fmt(f),
                CellarV2Calls::AavePool(element) => element.fmt(f),
                CellarV2Calls::AddPosition(element) => element.fmt(f),
                CellarV2Calls::Allowance(element) => element.fmt(f),
                CellarV2Calls::AllowedRebalanceDeviation(element) => element.fmt(f),
                CellarV2Calls::Approve(element) => element.fmt(f),
                CellarV2Calls::Asset(element) => element.fmt(f),
                CellarV2Calls::AssetRiskTolerance(element) => element.fmt(f),
                CellarV2Calls::BalanceOf(element) => element.fmt(f),
                CellarV2Calls::BlockExternalReceiver(element) => element.fmt(f),
                CellarV2Calls::CallOnAdaptor(element) => element.fmt(f),
                CellarV2Calls::ConvertToAssets(element) => element.fmt(f),
                CellarV2Calls::ConvertToShares(element) => element.fmt(f),
                CellarV2Calls::CreditPositions(element) => element.fmt(f),
                CellarV2Calls::DebtPositions(element) => element.fmt(f),
                CellarV2Calls::Decimals(element) => element.fmt(f),
                CellarV2Calls::Deposit(element) => element.fmt(f),
                CellarV2Calls::ExecuteOperation(element) => element.fmt(f),
                CellarV2Calls::FeeData(element) => element.fmt(f),
                CellarV2Calls::GetCreditPositions(element) => element.fmt(f),
                CellarV2Calls::GetDebtPositions(element) => element.fmt(f),
                CellarV2Calls::GetPositionAssets(element) => element.fmt(f),
                CellarV2Calls::GetPositionData(element) => element.fmt(f),
                CellarV2Calls::HoldingPosition(element) => element.fmt(f),
                CellarV2Calls::Initialize(element) => element.fmt(f),
                CellarV2Calls::InitiateShutdown(element) => element.fmt(f),
                CellarV2Calls::IsAdaptorSetup(element) => element.fmt(f),
                CellarV2Calls::IsPositionUsed(element) => element.fmt(f),
                CellarV2Calls::IsShutdown(element) => element.fmt(f),
                CellarV2Calls::LiftShutdown(element) => element.fmt(f),
                CellarV2Calls::Locked(element) => element.fmt(f),
                CellarV2Calls::MaxDeposit(element) => element.fmt(f),
                CellarV2Calls::MaxMint(element) => element.fmt(f),
                CellarV2Calls::MaxRedeem(element) => element.fmt(f),
                CellarV2Calls::MaxWithdraw(element) => element.fmt(f),
                CellarV2Calls::Mint(element) => element.fmt(f),
                CellarV2Calls::Name(element) => element.fmt(f),
                CellarV2Calls::Nonces(element) => element.fmt(f),
                CellarV2Calls::OnERC721Received(element) => element.fmt(f),
                CellarV2Calls::Owner(element) => element.fmt(f),
                CellarV2Calls::Permit(element) => element.fmt(f),
                CellarV2Calls::PreviewDeposit(element) => element.fmt(f),
                CellarV2Calls::PreviewMint(element) => element.fmt(f),
                CellarV2Calls::PreviewRedeem(element) => element.fmt(f),
                CellarV2Calls::PreviewWithdraw(element) => element.fmt(f),
                CellarV2Calls::ProtocolRiskTolerance(element) => element.fmt(f),
                CellarV2Calls::Redeem(element) => element.fmt(f),
                CellarV2Calls::Registry(element) => element.fmt(f),
                CellarV2Calls::RemovePosition(element) => element.fmt(f),
                CellarV2Calls::SendFees(element) => element.fmt(f),
                CellarV2Calls::SetHoldingPosition(element) => element.fmt(f),
                CellarV2Calls::SetPlatformFee(element) => element.fmt(f),
                CellarV2Calls::SetRebalanceDeviation(element) => element.fmt(f),
                CellarV2Calls::SetShareLockPeriod(element) => element.fmt(f),
                CellarV2Calls::SetStrategistPayoutAddress(element) => element.fmt(f),
                CellarV2Calls::SetStrategistPlatformCut(element) => element.fmt(f),
                CellarV2Calls::SetupAdaptor(element) => element.fmt(f),
                CellarV2Calls::ShareLockPeriod(element) => element.fmt(f),
                CellarV2Calls::SwapPositions(element) => element.fmt(f),
                CellarV2Calls::Symbol(element) => element.fmt(f),
                CellarV2Calls::TotalAssets(element) => element.fmt(f),
                CellarV2Calls::TotalAssetsWithdrawable(element) => element.fmt(f),
                CellarV2Calls::TotalSupply(element) => element.fmt(f),
                CellarV2Calls::Transfer(element) => element.fmt(f),
                CellarV2Calls::TransferFrom(element) => element.fmt(f),
                CellarV2Calls::TransferOwnership(element) => element.fmt(f),
                CellarV2Calls::UserShareLockStartTime(element) => element.fmt(f),
                CellarV2Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for CellarV2Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            CellarV2Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<GravityBridgeRegistrySlotCall> for CellarV2Calls {
        fn from(var: GravityBridgeRegistrySlotCall) -> Self {
            CellarV2Calls::GravityBridgeRegistrySlot(var)
        }
    }
    impl ::std::convert::From<MaximumShareLockPeriodCall> for CellarV2Calls {
        fn from(var: MaximumShareLockPeriodCall) -> Self {
            CellarV2Calls::MaximumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<MaxFeeCutCall> for CellarV2Calls {
        fn from(var: MaxFeeCutCall) -> Self {
            CellarV2Calls::MaxFeeCut(var)
        }
    }
    impl ::std::convert::From<MaxPlatformFeeCall> for CellarV2Calls {
        fn from(var: MaxPlatformFeeCall) -> Self {
            CellarV2Calls::MaxPlatformFee(var)
        }
    }
    impl ::std::convert::From<MaxPositionsCall> for CellarV2Calls {
        fn from(var: MaxPositionsCall) -> Self {
            CellarV2Calls::MaxPositions(var)
        }
    }
    impl ::std::convert::From<MaxRebalanceDeviationCall> for CellarV2Calls {
        fn from(var: MaxRebalanceDeviationCall) -> Self {
            CellarV2Calls::MaxRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<MinimumShareLockPeriodCall> for CellarV2Calls {
        fn from(var: MinimumShareLockPeriodCall) -> Self {
            CellarV2Calls::MinimumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<PriceRouterRegistrySlotCall> for CellarV2Calls {
        fn from(var: PriceRouterRegistrySlotCall) -> Self {
            CellarV2Calls::PriceRouterRegistrySlot(var)
        }
    }
    impl ::std::convert::From<AavePoolCall> for CellarV2Calls {
        fn from(var: AavePoolCall) -> Self {
            CellarV2Calls::AavePool(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for CellarV2Calls {
        fn from(var: AddPositionCall) -> Self {
            CellarV2Calls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CellarV2Calls {
        fn from(var: AllowanceCall) -> Self {
            CellarV2Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<AllowedRebalanceDeviationCall> for CellarV2Calls {
        fn from(var: AllowedRebalanceDeviationCall) -> Self {
            CellarV2Calls::AllowedRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CellarV2Calls {
        fn from(var: ApproveCall) -> Self {
            CellarV2Calls::Approve(var)
        }
    }
    impl ::std::convert::From<AssetCall> for CellarV2Calls {
        fn from(var: AssetCall) -> Self {
            CellarV2Calls::Asset(var)
        }
    }
    impl ::std::convert::From<AssetRiskToleranceCall> for CellarV2Calls {
        fn from(var: AssetRiskToleranceCall) -> Self {
            CellarV2Calls::AssetRiskTolerance(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CellarV2Calls {
        fn from(var: BalanceOfCall) -> Self {
            CellarV2Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BlockExternalReceiverCall> for CellarV2Calls {
        fn from(var: BlockExternalReceiverCall) -> Self {
            CellarV2Calls::BlockExternalReceiver(var)
        }
    }
    impl ::std::convert::From<CallOnAdaptorCall> for CellarV2Calls {
        fn from(var: CallOnAdaptorCall) -> Self {
            CellarV2Calls::CallOnAdaptor(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for CellarV2Calls {
        fn from(var: ConvertToAssetsCall) -> Self {
            CellarV2Calls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for CellarV2Calls {
        fn from(var: ConvertToSharesCall) -> Self {
            CellarV2Calls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<CreditPositionsCall> for CellarV2Calls {
        fn from(var: CreditPositionsCall) -> Self {
            CellarV2Calls::CreditPositions(var)
        }
    }
    impl ::std::convert::From<DebtPositionsCall> for CellarV2Calls {
        fn from(var: DebtPositionsCall) -> Self {
            CellarV2Calls::DebtPositions(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CellarV2Calls {
        fn from(var: DecimalsCall) -> Self {
            CellarV2Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CellarV2Calls {
        fn from(var: DepositCall) -> Self {
            CellarV2Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<ExecuteOperationCall> for CellarV2Calls {
        fn from(var: ExecuteOperationCall) -> Self {
            CellarV2Calls::ExecuteOperation(var)
        }
    }
    impl ::std::convert::From<FeeDataCall> for CellarV2Calls {
        fn from(var: FeeDataCall) -> Self {
            CellarV2Calls::FeeData(var)
        }
    }
    impl ::std::convert::From<GetCreditPositionsCall> for CellarV2Calls {
        fn from(var: GetCreditPositionsCall) -> Self {
            CellarV2Calls::GetCreditPositions(var)
        }
    }
    impl ::std::convert::From<GetDebtPositionsCall> for CellarV2Calls {
        fn from(var: GetDebtPositionsCall) -> Self {
            CellarV2Calls::GetDebtPositions(var)
        }
    }
    impl ::std::convert::From<GetPositionAssetsCall> for CellarV2Calls {
        fn from(var: GetPositionAssetsCall) -> Self {
            CellarV2Calls::GetPositionAssets(var)
        }
    }
    impl ::std::convert::From<GetPositionDataCall> for CellarV2Calls {
        fn from(var: GetPositionDataCall) -> Self {
            CellarV2Calls::GetPositionData(var)
        }
    }
    impl ::std::convert::From<HoldingPositionCall> for CellarV2Calls {
        fn from(var: HoldingPositionCall) -> Self {
            CellarV2Calls::HoldingPosition(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for CellarV2Calls {
        fn from(var: InitializeCall) -> Self {
            CellarV2Calls::Initialize(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for CellarV2Calls {
        fn from(var: InitiateShutdownCall) -> Self {
            CellarV2Calls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsAdaptorSetupCall> for CellarV2Calls {
        fn from(var: IsAdaptorSetupCall) -> Self {
            CellarV2Calls::IsAdaptorSetup(var)
        }
    }
    impl ::std::convert::From<IsPositionUsedCall> for CellarV2Calls {
        fn from(var: IsPositionUsedCall) -> Self {
            CellarV2Calls::IsPositionUsed(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for CellarV2Calls {
        fn from(var: IsShutdownCall) -> Self {
            CellarV2Calls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for CellarV2Calls {
        fn from(var: LiftShutdownCall) -> Self {
            CellarV2Calls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LockedCall> for CellarV2Calls {
        fn from(var: LockedCall) -> Self {
            CellarV2Calls::Locked(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for CellarV2Calls {
        fn from(var: MaxDepositCall) -> Self {
            CellarV2Calls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for CellarV2Calls {
        fn from(var: MaxMintCall) -> Self {
            CellarV2Calls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for CellarV2Calls {
        fn from(var: MaxRedeemCall) -> Self {
            CellarV2Calls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for CellarV2Calls {
        fn from(var: MaxWithdrawCall) -> Self {
            CellarV2Calls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for CellarV2Calls {
        fn from(var: MintCall) -> Self {
            CellarV2Calls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for CellarV2Calls {
        fn from(var: NameCall) -> Self {
            CellarV2Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CellarV2Calls {
        fn from(var: NoncesCall) -> Self {
            CellarV2Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for CellarV2Calls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            CellarV2Calls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CellarV2Calls {
        fn from(var: OwnerCall) -> Self {
            CellarV2Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for CellarV2Calls {
        fn from(var: PermitCall) -> Self {
            CellarV2Calls::Permit(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for CellarV2Calls {
        fn from(var: PreviewDepositCall) -> Self {
            CellarV2Calls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for CellarV2Calls {
        fn from(var: PreviewMintCall) -> Self {
            CellarV2Calls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for CellarV2Calls {
        fn from(var: PreviewRedeemCall) -> Self {
            CellarV2Calls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for CellarV2Calls {
        fn from(var: PreviewWithdrawCall) -> Self {
            CellarV2Calls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<ProtocolRiskToleranceCall> for CellarV2Calls {
        fn from(var: ProtocolRiskToleranceCall) -> Self {
            CellarV2Calls::ProtocolRiskTolerance(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CellarV2Calls {
        fn from(var: RedeemCall) -> Self {
            CellarV2Calls::Redeem(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for CellarV2Calls {
        fn from(var: RegistryCall) -> Self {
            CellarV2Calls::Registry(var)
        }
    }
    impl ::std::convert::From<RemovePositionCall> for CellarV2Calls {
        fn from(var: RemovePositionCall) -> Self {
            CellarV2Calls::RemovePosition(var)
        }
    }
    impl ::std::convert::From<SendFeesCall> for CellarV2Calls {
        fn from(var: SendFeesCall) -> Self {
            CellarV2Calls::SendFees(var)
        }
    }
    impl ::std::convert::From<SetHoldingPositionCall> for CellarV2Calls {
        fn from(var: SetHoldingPositionCall) -> Self {
            CellarV2Calls::SetHoldingPosition(var)
        }
    }
    impl ::std::convert::From<SetPlatformFeeCall> for CellarV2Calls {
        fn from(var: SetPlatformFeeCall) -> Self {
            CellarV2Calls::SetPlatformFee(var)
        }
    }
    impl ::std::convert::From<SetRebalanceDeviationCall> for CellarV2Calls {
        fn from(var: SetRebalanceDeviationCall) -> Self {
            CellarV2Calls::SetRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<SetShareLockPeriodCall> for CellarV2Calls {
        fn from(var: SetShareLockPeriodCall) -> Self {
            CellarV2Calls::SetShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<SetStrategistPayoutAddressCall> for CellarV2Calls {
        fn from(var: SetStrategistPayoutAddressCall) -> Self {
            CellarV2Calls::SetStrategistPayoutAddress(var)
        }
    }
    impl ::std::convert::From<SetStrategistPlatformCutCall> for CellarV2Calls {
        fn from(var: SetStrategistPlatformCutCall) -> Self {
            CellarV2Calls::SetStrategistPlatformCut(var)
        }
    }
    impl ::std::convert::From<SetupAdaptorCall> for CellarV2Calls {
        fn from(var: SetupAdaptorCall) -> Self {
            CellarV2Calls::SetupAdaptor(var)
        }
    }
    impl ::std::convert::From<ShareLockPeriodCall> for CellarV2Calls {
        fn from(var: ShareLockPeriodCall) -> Self {
            CellarV2Calls::ShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<SwapPositionsCall> for CellarV2Calls {
        fn from(var: SwapPositionsCall) -> Self {
            CellarV2Calls::SwapPositions(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CellarV2Calls {
        fn from(var: SymbolCall) -> Self {
            CellarV2Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for CellarV2Calls {
        fn from(var: TotalAssetsCall) -> Self {
            CellarV2Calls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalAssetsWithdrawableCall> for CellarV2Calls {
        fn from(var: TotalAssetsWithdrawableCall) -> Self {
            CellarV2Calls::TotalAssetsWithdrawable(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CellarV2Calls {
        fn from(var: TotalSupplyCall) -> Self {
            CellarV2Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CellarV2Calls {
        fn from(var: TransferCall) -> Self {
            CellarV2Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CellarV2Calls {
        fn from(var: TransferFromCall) -> Self {
            CellarV2Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CellarV2Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CellarV2Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UserShareLockStartTimeCall> for CellarV2Calls {
        fn from(var: UserShareLockStartTimeCall) -> Self {
            CellarV2Calls::UserShareLockStartTime(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CellarV2Calls {
        fn from(var: WithdrawCall) -> Self {
            CellarV2Calls::Withdraw(var)
        }
    }
    #[doc = "`AdaptorCall(address,bytes[])`"]
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
    pub struct AdaptorCall {
        pub adaptor: ethers::core::types::Address,
        pub call_data: Vec<ethers::core::types::Bytes>,
    }
}
