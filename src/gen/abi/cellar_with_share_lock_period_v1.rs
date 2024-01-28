pub use cellarwithsharelockperiodv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod cellarwithsharelockperiodv1_mod {
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
    #[doc = "CellarWithShareLockPeriodV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CELLARWITHSHARELOCKPERIODV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("{\n  \"abi\": [\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract Registry\",\n          \"name\": \"_registry\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"_name\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"_symbol\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"_holdingPosition\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_holdingPositionConfig\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"_initialDeposit\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"_strategistPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_shareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"expectedAsset\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__AssetMismatch\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__CallToAdaptorNotAllowed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__CallerNotApprovedToRebalance\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ContractNotShutdown\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ContractShutdown\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__DebtMismatch\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ExpectedAddressDoesNotMatchActual\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__FailedToForceOutPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"illiquidPosition\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__IlliquidWithdraw\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assetsOwed\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__IncompleteWithdraw\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidFee\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidFeeCut\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__InvalidHoldingPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"requested\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"max\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__InvalidRebalanceDeviation\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidShareLockPeriod\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidShareSupplyCap\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__MinimumConstructorMintNotMet\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"depositor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__NotApprovedToDepositOnBehalf\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__Paused\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionAlreadyUsed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"maxPositions\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__PositionArrayFull\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"sharesRemaining\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotEmpty\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotInCatalogue\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotUsed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__RemovingHoldingPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__SettingValueToRegistryIdZeroIsProhibited\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ShareSupplyCapExceeded\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"timeSharesAreUnlocked\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"currentBlock\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__SharesAreLocked\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"min\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"max\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__TotalAssetDeviatedOutsideRange\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"current\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"expected\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__TotalSharesMustRemainConstant\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ZeroAssets\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ZeroShares\",\n      \"type\": \"error\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"AdaptorCalled\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"inCatalogue\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"AdaptorCatalogueAltered\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Approval\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newAutomationActions\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__AutomationActionsUpdated\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Deposit\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"OwnershipTransferred\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionAdded\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"inCatalogue\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"PositionCatalogueAltered\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionRemoved\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"newPosition1\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"newPosition2\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index2\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionSwapped\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"oldDeviation\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newDeviation\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"RebalanceDeviationChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"oldPeriod\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newPeriod\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"ShareLockingPeriodChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"isShutdown\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"ShutdownChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"oldPayoutAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newPayoutAddress\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"StrategistPayoutAddressChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"oldPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"newPlatformCut\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"StrategistPlatformCutChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Withdraw\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DOMAIN_SEPARATOR\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"GRAVITY_BRIDGE_REGISTRY_SLOT\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MAX_FEE_CUT\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MAX_PLATFORM_FEE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MAX_POSITIONS\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MAX_REBALANCE_DEVIATION\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"MINIMUM_SHARE_LOCK_PERIOD\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"PRICE_ROUTER_REGISTRY_SLOT\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"adaptorCatalogue\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"addAdaptorToCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"configurationData\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"addPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"addPositionToCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"allowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"allowedRebalanceDeviation\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"approve\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"asset\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"automationActions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"blockExternalReceiver\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"checkTotalAssets\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"uint16\",\n          \"name\": \"allowableRange\",\n          \"type\": \"uint16\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"expectedPriceRouter\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"cachePriceRouter\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes[]\",\n              \"name\": \"callData\",\n              \"type\": \"bytes[]\"\n            }\n          ],\n          \"internalType\": \"struct Cellar.AdaptorCall[]\",\n          \"name\": \"data\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"callOnAdaptor\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToShares\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"creditPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"debtPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"decimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_newShareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"decreaseShareSupplyCap\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"deposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"feeData\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"strategistPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"platformFee\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"lastAccrual\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"strategistPayoutAddress\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"forcePositionOut\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCreditPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32[]\",\n          \"name\": \"\",\n          \"type\": \"uint32[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getDebtPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32[]\",\n          \"name\": \"\",\n          \"type\": \"uint32[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"getPositionData\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"isDebt\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"adaptorData\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"configurationData\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"holdingPosition\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"ignorePause\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_newShareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"increaseShareSupplyCap\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"initiateShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"isPaused\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"isPositionUsed\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"isShutdown\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"liftShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"locked\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"mint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"data\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"name\": \"multicall\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"nonces\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC721Received\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"deadline\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"v\",\n          \"type\": \"uint8\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"r\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"s\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"permit\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"positionCatalogue\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"priceRouter\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract PriceRouter\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"redeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"registry\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract Registry\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"removeAdaptorFromCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"removePosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"removePositionFromCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"_registryId\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_expectedAutomationActions\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"setAutomationActions\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"setHoldingPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newDeviation\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setRebalanceDeviation\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newLock\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setShareLockPeriod\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"payout\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"setStrategistPayoutAddress\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"cut\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"setStrategistPlatformCut\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"shareLockPeriod\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"shareSupplyCap\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index1\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index2\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"swapPositions\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"symbol\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"toggleIgnorePause\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalAssetsWithdrawable\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalSupply\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transferFrom\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"transferOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"userShareLockStartTime\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"viewPositionBalances\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20[]\",\n          \"name\": \"assets\",\n          \"type\": \"address[]\"\n        },\n        {\n          \"internalType\": \"uint256[]\",\n          \"name\": \"balances\",\n          \"type\": \"uint256[]\"\n        },\n        {\n          \"internalType\": \"bool[]\",\n          \"name\": \"isDebt\",\n          \"type\": \"bool[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"withdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n  ],\n  \"bytecode\": {\n    \"object\": \"0x6101a0604052670a688906bd8b000061012052662386f26fc1000061014052600061016081905261018052600f80546001600160c01b0319166e2386f26fc100000a688906bd8b0000179055601080546001600160a01b0319169055660110d9316ec0006012556202a3006013553480156200007a57600080fd5b50604051620072d0380380620072d08339810160408190526200009d916200105d565b89898989898989898989338888888181846001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015620000ec573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906200011291906200116b565b60006200012084826200121f565b5060016200012f83826200121f565b5060ff81166080524660a05262000145620002f0565b60c052505050506001600160a01b0391821660e05250600680546001600160a01b031916918316918217905560405160009060008051602062007289833981519152908290a3506001600160a01b038916610100819052604051635c9fcd8560e11b81526002600482015263b93f9b0a90602401602060405180830381865afa158015620001d7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001fd9190620012eb565b600880546001600160a01b0319166001600160a01b039290921691909117905562000228856200038c565b6200023760008686826200049d565b6200024285620007ac565b600780546001600160c01b0319166001600160c01b0383161790556127108310156200028157604051632d0d251960e11b815260040160405180910390fd5b620002986001600160a01b0389168b308662000906565b620002a433846200099b565b620002b0858462000a08565b600f80546001600160401b0319166001600160401b038416179055620002d68a62000a9f565b505050505050505050505050505050505050505062001586565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60006040516200032491906200130b565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b6006546001600160a01b03163314620003db5760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b60448201526064015b60405180910390fd5b61010051604051635159d87f60e11b815263ffffffff831660048201526001600160a01b039091169063a2b3b0fe9060240160006040518083038186803b1580156200042657600080fd5b505afa1580156200043b573d6000803e3d6000fd5b5050505063ffffffff81166000818152600d6020908152604091829020805460ff191660019081179091558251938452908301527fea052d1fb1ecba6aaf6bd32e92f20e7b6a094eaa478248322cc8ff024a90978f910160405180910390a150565b6006546001600160a01b03163314620004e85760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b6044820152606401620003d2565b620004f262000b25565b63ffffffff83166000908152600b602052604090205460ff1615620005335760405163335894fb60e11b815263ffffffff84166004820152602401620003d2565b63ffffffff83166000908152600d602052604090205460ff166200057357604051631f9db01d60e31b815263ffffffff84166004820152602401620003d2565b610100516040516385ae5d5760e01b815263ffffffff85166004820152600091829182916001600160a01b0316906385ae5d5790602401600060405180830381865afa158015620005c8573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052620005f2919081019062001389565b925092509250831515821515146200062657604051632b1d0bd360e11b815263ffffffff87166004820152602401620003d2565b604080516080810182526001600160a01b0380861682528415156020808401918252838501868152606085018b905263ffffffff8c166000908152600c9092529490208351815492511515600160a01b026001600160a81b031990931693169290921717815591519091906001820190620006a290826200121f565b5060608201516002820190620006b990826200121f565b5090505081156200070057600a54602011620006ec5760405163f025236d60e01b815260206004820152602401620003d2565b620006fa600a888862000b53565b62000736565b600954602011620007285760405163f025236d60e01b815260206004820152602401620003d2565b620007366009888862000b53565b63ffffffff86166000908152600b602052604090819020805460ff19166001179055517fc4f8cb57c016f0b294fff2666f86fa6cfee9b03aed19f816ae4bf44b7e837bbb906200079b9088908a9063ffffffff92831681529116602082015260400190565b60405180910390a150505050505050565b6006546001600160a01b03163314620007f75760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b6044820152606401620003d2565b63ffffffff81166000908152600b602052604090205460ff1662000837576040516370abe85960e01b815263ffffffff82166004820152602401620003d2565b60e0516001600160a01b03166200084e8262000d30565b6001600160a01b031614620008985760e0516200086b8262000d30565b60405163298473c760e11b81526001600160a01b03928316600482015291166024820152604401620003d2565b63ffffffff81166000908152600c6020526040902054600160a01b900460ff1615620008e057604051630a42c0f960e41b815263ffffffff82166004820152602401620003d2565b6006805463ffffffff909216600160c01b0263ffffffff60c01b19909216919091179055565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080620009945760405162461bcd60e51b815260206004820152601460248201527f5452414e534645525f46524f4d5f4641494c45440000000000000000000000006044820152606401620003d2565b5050505050565b8060026000828254620009af91906200140e565b90915550506001600160a01b0382166000818152600360209081526040808320805486019055518481527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef910160405180910390a35050565b63ffffffff82166000908152600c602052604090819020805491516001600160a01b039092169162000a99916369445c3160e01b9162000a56918691600182019160020190602401620014a9565b60408051808303601f190181529190526020810180516001600160e01b0319939093166001600160e01b039384161790526001600160a01b0384169162000dc516565b50505050565b6006546001600160a01b0316331462000aea5760405162461bcd60e51b815260206004820152600c60248201526b15539055551213d49256915160a21b6044820152606401620003d2565b600680546001600160a01b0319166001600160a01b03831690811790915560405133906000805160206200728983398151915290600090a350565b600654600160a81b900460ff161562000b51576040516337a5332d60e11b815260040160405180910390fd5b565b8254801562000cef57838062000b6b600184620014d8565b8154811062000b7e5762000b7e620014ee565b60009182526020808320600880840490910154855460018082018855968652928520918304909101805463ffffffff60046007958616810261010090810a83810219909416969097160290950a909204909316021790559062000be29083620014d8565b90505b8363ffffffff1681111562000c99578462000c02600183620014d8565b8154811062000c155762000c15620014ee565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff1685828154811062000c505762000c50620014ee565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff160217905550808062000c909062001504565b91505062000be5565b5081848463ffffffff168154811062000cb65762000cb6620014ee565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff16021790555062000a99565b508254600181018455600093845260209093206008840401805460079094166004026101000a63ffffffff8181021990951692909416939093021790915550565b63ffffffff81166000908152600c60205260408082208054915163e170a9bf60e01b81526001600160a01b0390921691829163e170a9bf9162000d7a91600101906004016200151e565b602060405180830381865afa15801562000d98573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019062000dbe9190620012eb565b9392505050565b606062000ded8383604051806060016040528060278152602001620072a96027913962000df6565b90505b92915050565b6060600080856001600160a01b03168560405162000e15919062001533565b600060405180830381855af49150503d806000811462000e52576040519150601f19603f3d011682016040523d82523d6000602084013e62000e57565b606091505b50909250905062000e6b8683838762000e75565b9695505050505050565b6060831562000ee957825160000362000ee1576001600160a01b0385163b62000ee15760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401620003d2565b508162000ef5565b62000ef5838362000efd565b949350505050565b81511562000f0e5781518083602001fd5b8060405162461bcd60e51b8152600401620003d2919062001551565b6001600160a01b038116811462000f4057600080fd5b50565b805162000f508162000f2a565b919050565b634e487b7160e01b600052604160045260246000fd5b60005b8381101562000f8857818101518382015260200162000f6e565b50506000910152565b600082601f83011262000fa357600080fd5b81516001600160401b038082111562000fc05762000fc062000f55565b604051601f8301601f19908116603f0116810190828211818310171562000feb5762000feb62000f55565b816040528381528660208588010111156200100557600080fd5b62000e6b84602083016020890162000f6b565b805163ffffffff8116811462000f5057600080fd5b80516001600160401b038116811462000f5057600080fd5b80516001600160c01b038116811462000f5057600080fd5b6000806000806000806000806000806101408b8d0312156200107e57600080fd5b620010898b62000f43565b99506200109960208c0162000f43565b9850620010a960408c0162000f43565b60608c01519098506001600160401b0380821115620010c757600080fd5b620010d58e838f0162000f91565b985060808d0151915080821115620010ec57600080fd5b620010fa8e838f0162000f91565b97506200110a60a08e0162001018565b965060c08d01519150808211156200112157600080fd5b50620011308d828e0162000f91565b94505060e08b01519250620011496101008c016200102d565b91506200115a6101208c0162001045565b90509295989b9194979a5092959850565b6000602082840312156200117e57600080fd5b815160ff8116811462000dbe57600080fd5b600181811c90821680620011a557607f821691505b602082108103620011c657634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200121a57600081815260208120601f850160051c81016020861015620011f55750805b601f850160051c820191505b81811015620012165782815560010162001201565b5050505b505050565b81516001600160401b038111156200123b576200123b62000f55565b62001253816200124c845462001190565b84620011cc565b602080601f8311600181146200128b5760008415620012725750858301515b600019600386901b1c1916600185901b17855562001216565b600085815260208120601f198616915b82811015620012bc578886015182559484019460019091019084016200129b565b5085821015620012db5787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b600060208284031215620012fe57600080fd5b815162000dbe8162000f2a565b60008083546200131b8162001190565b600182811680156200133657600181146200134c576200137d565b60ff19841687528215158302870194506200137d565b8760005260208060002060005b85811015620013745781548a82015290840190820162001359565b50505082870194505b50929695505050505050565b6000806000606084860312156200139f57600080fd5b8351620013ac8162000f2a565b60208501519093508015158114620013c357600080fd5b60408501519092506001600160401b03811115620013e057600080fd5b620013ee8682870162000f91565b9150509250925092565b634e487b7160e01b600052601160045260246000fd5b8082018082111562000df05762000df0620013f8565b60008154620014338162001190565b8085526020600183811680156200145357600181146200146e576200149e565b60ff1985168884015283151560051b8801830195506200149e565b866000528260002060005b85811015620014965781548a820186015290830190840162001479565b890184019650505b505050505092915050565b838152606060208201526000620014c4606083018562001424565b828103604084015262000e6b818562001424565b8181038181111562000df05762000df0620013f8565b634e487b7160e01b600052603260045260246000fd5b600081620015165762001516620013f8565b506000190190565b60208152600062000ded602083018462001424565b600082516200154781846020870162000f6b565b9190910192915050565b60208152600082518060208401526200157281604085016020870162000f6b565b601f01601f19169190910160400192915050565b60805160a05160c05160e05161010051615c5562001634600039600081816108360152818161140a015281816117d601528181612213015281816125ab015281816128a1015281816130ea01528181614051015261439001526000818161064601528181610d4c01528181610d8d01528181613309015281816135ca01528181613c4c0152614656015260006110c7015260006110970152600081816105df01526146d40152615c556000f3fe608060405234801561001057600080fd5b50600436106104b65760003560e01c80638da5cb5b11610278578063bf86d6901161015c578063d446bbcc116100ce578063e1b1acb711610092578063e1b1acb714610b56578063e753e60014610b69578063eef33eca14610bde578063ef8b30f714610a3c578063f2fde38b14610bed578063f7b24e0814610c0057600080fd5b8063d446bbcc14610ac7578063d505accf14610af2578063d7d4bf4514610b05578063d905777e14610b18578063dd62ed3e14610b2b57600080fd5b8063c8e8195011610120578063c8e8195014610a4f578063cbdf33d014610a62578063cd82f8b114610a85578063ce96cb7714610a8d578063cf30901214610aa0578063d1e8840414610ab457600080fd5b8063bf86d690146109f9578063c244245a14610a0d578063c588d8d614610a16578063c63d75b614610a29578063c6e6f59214610a3c57600080fd5b8063a373e3ff116101f5578063b0a75d36116101b9578063b0a75d3614610992578063b187bd26146109a5578063b3d7f6b9146109ad578063b460af94146109c0578063b5292a99146109d3578063ba087652146109e657600080fd5b8063a373e3ff14610949578063a8144e4814610951578063a9059cbb14610959578063ac9650d81461096c578063b0646e271461097f57600080fd5b80639959af941161023c5780639959af94146108ef5780639c552ca8146109035780639c5f00c2146109165780639fdb11b61461092d578063a07bee0b1461093657600080fd5b80638da5cb5b1461088b57806393bbeac01461089e57806394bf804d146108c157806395d89b41146108d45780639955a9d4146108dc57600080fd5b8063402d267d1161039f5780635f6b88a01161031c57806371e99dc2116102e057806371e99dc2146107ef5780637384504f146107f757806378e0233e1461081a5780637b103999146108315780637ecebe001461085857806388c4caba1461087857600080fd5b80635f6b88a01461077a578063687c2b501461078d5780636e553f65146107ad5780636ff1c02a146107c057806370a08231146107cf57600080fd5b8063530a371411610363578063530a37141461071c578063575bbce61461072f57806359d20b4e146107425780635a400d251461076a5780635e2c576e1461077257600080fd5b8063402d267d146106cf5780634c4602da146106e25780634cdad506146105135780634e84befe146106f6578063501eb4fe1461070957600080fd5b806318160ddd116104385780633644e515116103fc5780633644e51514610626578063379e0b131461062e57806338d52e0f146106415780633998a681146106805780633d8ab1e5146106a75780633e3382ba146106ba57600080fd5b806318160ddd1461059b57806318d4c143146105a457806323b872dd146105c7578063313ce567146105da57806333e15be21461061357600080fd5b806307a2d13a1161047f57806307a2d13a14610513578063095ea7b3146105265780630a28a477146105495780630a680e181461055c578063150b7a021461056457600080fd5b806251a3b7146104bb57806301e1d114146104d75780630402ab63146104df57806306fdde03146104e95780630780fd3a146104fe575b600080fd5b6104c461012c81565b6040519081526020015b60405180910390f35b6104c4610c08565b6104c46202a30081565b6104f1610c54565b6040516104ce9190614c54565b61051161050c366004614c80565b610ce2565b005b6104c4610521366004614c9b565b610e4d565b610539610534366004614cc9565b610e73565b60405190151581526020016104ce565b6104c4610557366004614c9b565b610ee0565b610511610efe565b610582610572366004614de0565b630a85bd0160e11b949350505050565b6040516001600160e01b031990911681526020016104ce565b6104c460025481565b6105396105b2366004614e4b565b600e6020526000908152604090205460ff1681565b6105396105d5366004614e68565b610f7a565b6106017f000000000000000000000000000000000000000000000000000000000000000081565b60405160ff90911681526020016104ce565b610511610621366004614eb7565b610f90565b6104c4611093565b61051161063c366004614eee565b6110e9565b6106687f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016104ce565b61068f6702c68af0bb14000081565b6040516001600160401b0390911681526020016104ce565b6105116106b5366004614e4b565b6113c1565b6106c26114c6565b6040516104ce9190614f35565b6104c46106dd366004614e4b565b61154a565b60065461053990600160b81b900460ff1681565b610511610704366004614fca565b6115ec565b610511610717366004614c80565b611791565b61051161072a366004614c9b565b611890565b61051161073d36600461500b565b61193a565b610755610750366004614c9b565b6119b6565b60405163ffffffff90911681526020016104ce565b6104c4600281565b6105116119f0565b610511610788366004614e4b565b611a82565b6104c461079b366004614e4b565b60146020526000908152604090205481565b6104c46107bb366004615034565b611b03565b61068f67016345785d8a000081565b6104c46107dd366004614e4b565b60036020526000908152604090205481565b6106c2611bd7565b61080a610805366004614c80565b611c34565b6040516104ce9493929190615059565b610822611d7d565b6040516104ce93929190615116565b6106687f000000000000000000000000000000000000000000000000000000000000000081565b6104c4610866366004614e4b565b60056020526000908152604090205481565b601154610668906001600160a01b031681565b600654610668906001600160a01b031681565b6105396108ac366004614c9b565b600b6020526000908152604090205460ff1681565b6104c46108cf366004615034565b61206e565b6104f161212d565b6105116108ea366004615181565b61213a565b60065461053990600160b01b900460ff1681565b610511610911366004614c9b565b61242b565b60065461075590600160c01b900463ffffffff1681565b6104c460135481565b610511610944366004614eee565b6124c3565b610511612647565b6104c46126aa565b610539610967366004614cc9565b6126e8565b61051161097a366004614fca565b612704565b61051161098d36600461500b565b61278b565b6105116109a0366004614e4b565b6127e5565b610539612878565b6104c46109bb366004614c9b565b61291a565b6104c46109ce3660046151f1565b612938565b6105116109e1366004615228565b6129b7565b6104c46109f43660046151f1565b612a7d565b60065461053990600160a81b900460ff1681565b6104c460125481565b610511610a24366004615251565b612b07565b6104c4610a37366004614e4b565b612bf4565b6104c4610a4a366004614c9b565b612c6c565b610511610a5d366004615034565b612c8a565b610539610a70366004614c80565b600d6020526000908152604090205460ff1681565b6104c4600081565b6104c4610a9b366004614e4b565b612d0c565b60065461053990600160a01b900460ff1681565b610511610ac2366004614c80565b612d44565b600754610ada906001600160c01b031681565b6040516001600160c01b0390911681526020016104ce565b610511610b00366004615297565b612dc2565b600854610668906001600160a01b031681565b6104c4610b26366004614e4b565b613006565b6104c4610b39366004615308565b600460209081526000928352604080842090915290825290205481565b610755610b64366004614c9b565b61303e565b600f54601054610ba5916001600160401b0380821692680100000000000000008304821692600160801b9004909116906001600160a01b031684565b604080516001600160401b039586168152938516602085015291909316908201526001600160a01b0390911660608201526080016104ce565b61068f670de0b6b3a764000081565b610511610bfb366004614e4b565b61304e565b6104c4602081565b6000610c126130c4565b600654600160a01b900460ff1615610c455760405162461bcd60e51b8152600401610c3c90615336565b60405180910390fd5b610c4f600061317d565b905090565b60008054610c619061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8d9061535a565b8015610cda5780601f10610caf57610100808354040283529160200191610cda565b820191906000526020600020905b815481529060010190602001808311610cbd57829003601f168201915b505050505081565b6006546001600160a01b03163314610d0c5760405162461bcd60e51b8152600401610c3c90615394565b63ffffffff81166000908152600b602052604090205460ff16610d4a576040516370abe85960e01b815263ffffffff82166004820152602401610c3c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610d7d8261363f565b6001600160a01b031614610de1577f0000000000000000000000000000000000000000000000000000000000000000610db58261363f565b60405163298473c760e11b81526001600160a01b03928316600482015291166024820152604401610c3c565b63ffffffff81166000908152600c6020526040902054600160a01b900460ff1615610e2757604051630a42c0f960e41b815263ffffffff82166004820152602401610c3c565b6006805463ffffffff909216600160c01b0263ffffffff60c01b19909216919091179055565b6000806000610e5c60006136c8565b91509150610e6b8483836136e1565b949350505050565b3360008181526004602090815260408083206001600160a01b038716808552925280832085905551919290917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590610ece9086815260200190565b60405180910390a35060015b92915050565b6000806000610eef60006136c8565b91509150610e6b8483836136ee565b6006546001600160a01b03163314610f285760405162461bcd60e51b8152600401610c3c90615394565b610f306136fb565b6006805460ff60a81b1916600160a81b179055604051600181527fb8527b93c36dabdfe078af41be789ba946a4adcfeafcf9d8de21d51629859e3c906020015b60405180910390a1565b6000610f8584613726565b610e6b848484613785565b6006546001600160a01b03163314610fba5760405162461bcd60e51b8152600401610c3c90615394565b6000816110055760098363ffffffff1681548110610fda57610fda6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16611045565b600a8363ffffffff168154811061101e5761101e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff165b9050600061105282613865565b9050801561108257604051631c7b946d60e31b815263ffffffff8316600482015260248101829052604401610c3c565b61108d8483856138ee565b50505050565b60007f000000000000000000000000000000000000000000000000000000000000000046146110c457610c4f6139da565b507f000000000000000000000000000000000000000000000000000000000000000090565b6006546001600160a01b031633146111135760405162461bcd60e51b8152600401610c3c90615394565b600080821561124557600a8463ffffffff1681548110611135576111356153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff169150600a8563ffffffff1681548110611176576111766153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff1690508181600a8763ffffffff16815481106111b9576111b96153ba565b9060005260206000209060089182820401919006600402600a8863ffffffff16815481106111e9576111e96153ba565b90600052602060002090600891828204019190066004028491906101000a81548163ffffffff021916908363ffffffff1602179055508391906101000a81548163ffffffff021916908363ffffffff160217905550505061136a565b60098463ffffffff168154811061125e5761125e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16915060098563ffffffff168154811061129f5761129f6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff169050818160098763ffffffff16815481106112e2576112e26153ba565b906000526020600020906008918282040191900660040260098863ffffffff1681548110611312576113126153ba565b90600052602060002090600891828204019190066004028491906101000a81548163ffffffff021916908363ffffffff1602179055508391906101000a81548163ffffffff021916908363ffffffff16021790555050505b6040805163ffffffff84811682528381166020830152878116828401528616606082015290517fb7c5df04749a3a06a9a7bf1a8142ccf2a4ee6cbf4709489e876a6e4eb3301e8a9181900360800190a15050505050565b6006546001600160a01b031633146113eb5760405162461bcd60e51b8152600401610c3c90615394565b604051636777140560e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063ceee280a9060240160006040518083038186803b15801561144c57600080fd5b505afa158015611460573d6000803e3d6000fd5b505050506001600160a01b0381166000818152600e6020908152604091829020805460ff191660019081179091558251938452908301527f572570e8a43782d3698a3fed258c72f9c201c19be1e4764e359d1adc8f00af7a91015b60405180910390a150565b6060600a80548060200260200160405190810160405280929190818152602001828054801561154057602002820191906000526020600020906000905b82829054906101000a900463ffffffff1663ffffffff16815260200190600401906020826003010492830192600103820291508084116115035790505b5050505050905090565b600654600090600160a81b900460ff161561156757506000919050565b6007546001600160c01b03166002600160c01b0319810161158c575060001992915050565b60008061159960016136c8565b91509150826001600160c01b031681106115b857506000949350505050565b60006115cd826001600160c01b0386166153e6565b90506115da8184846136e1565b9695505050505050565b505050919050565b600654600160a01b900460ff16156116165760405162461bcd60e51b8152600401610c3c90615336565b60068054600160a01b60ff60a01b198216179091556001600160a01b0316331480159061164e57506011546001600160a01b03163314155b1561166c57604051633fd2923560e01b815260040160405180910390fd5b6116746136fb565b61167c6130c4565b6006805460ff60b81b1916600160b81b179055600080808061169d8161317d565b90506116c8601254670de0b6b3a76400006116b891906153e6565b8290670de0b6b3a7640000613a74565b93506116e3601254670de0b6b3a76400006116b891906153f9565b60025490935091506116ff90506116fa858761542f565b613aa2565b600061170b600061317d565b90508381108061171a57508281115b156117495760405163628cc47560e11b8152600481018290526024810185905260448101849052606401610c3c565b600254821461177957600254604051632b40145960e21b8152600481019190915260248101839052604401610c3c565b50506006805463ff0000ff60a01b1916905550505050565b6006546001600160a01b031633146117bb5760405162461bcd60e51b8152600401610c3c90615394565b604051635159d87f60e11b815263ffffffff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a2b3b0fe9060240160006040518083038186803b15801561182057600080fd5b505afa158015611834573d6000803e3d6000fd5b5050505063ffffffff81166000818152600d6020908152604091829020805460ff191660019081179091558251938452908301527fea052d1fb1ecba6aaf6bd32e92f20e7b6a094eaa478248322cc8ff024a90978f91016114bb565b6006546001600160a01b031633146118ba5760405162461bcd60e51b8152600401610c3c90615394565b67016345785d8a00008111156118f4576040516302d2a90f60e51b81526004810182905267016345785d8a00006024820152604401610c3c565b601280549082905560408051828152602081018490527fdf4be33b2e9e3dd4d9e0e85645aea428494a0644a72c51d6a15aedae6b66a3ff91015b60405180910390a15050565b6006546001600160a01b031633146119645760405162461bcd60e51b8152600401610c3c90615394565b6007546001600160c01b039081169082161115611994576040516334f1ec1b60e01b815260040160405180910390fd5b600780546001600160c01b0319166001600160c01b0392909216919091179055565b600981815481106119c657600080fd5b9060005260206000209060089182820401919006600402915054906101000a900463ffffffff1681565b6006546001600160a01b03163314611a1a5760405162461bcd60e51b8152600401610c3c90615394565b600654600160a81b900460ff16611a445760405163ec7165bf60e01b815260040160405180910390fd5b6006805460ff60a81b19169055604051600081527fb8527b93c36dabdfe078af41be789ba946a4adcfeafcf9d8de21d51629859e3c90602001610f70565b6006546001600160a01b03163314611aac5760405162461bcd60e51b8152600401610c3c90615394565b6001600160a01b0381166000818152600e60209081526040808320805460ff191690558051938452908301919091527f572570e8a43782d3698a3fed258c72f9c201c19be1e4764e359d1adc8f00af7a91016114bb565b600654600090600160a01b900460ff1615611b305760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080611b5060016136c8565b91509150611b5f858383613c27565b925082600003611b825760405163426f153760e11b815260040160405180910390fd5b6007546001600160c01b0316611b9884836153f9565b1115611bb75760405163adea3dfd60e01b815260040160405180910390fd5b611bc2858486613c34565b50506006805460ff60a01b1916905592915050565b60606009805480602002602001604051908101604052809291908181526020018280548015611540576000918252602091829020805463ffffffff1684529082028301929091600491018084116115035790505050505050905090565b600c60205260009081526040902080546001820180546001600160a01b03831693600160a01b90930460ff16929190611c6c9061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054611c989061535a565b8015611ce55780601f10611cba57610100808354040283529160200191611ce5565b820191906000526020600020905b815481529060010190602001808311611cc857829003601f168201915b505050505090806002018054611cfa9061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054611d269061535a565b8015611d735780601f10611d4857610100808354040283529160200191611d73565b820191906000526020600020905b815481529060010190602001808311611d5657829003601f168201915b5050505050905084565b600954600a546060918291829190611d9581836153f9565b6001600160401b03811115611dac57611dac614cf5565b604051908082528060200260200182016040528015611dd5578160200160208202803683370190505b509450611de281836153f9565b6001600160401b03811115611df957611df9614cf5565b604051908082528060200260200182016040528015611e22578160200160208202803683370190505b509350611e2f81836153f9565b6001600160401b03811115611e4657611e46614cf5565b604051908082528060200260200182016040528015611e6f578160200160208202803683370190505b50925060005b82811015611f8357611ebe60098281548110611e9357611e936153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff1661363f565b868281518110611ed057611ed06153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050611f3160098281548110611f0657611f066153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16613865565b858281518110611f4357611f436153ba565b6020026020010181815250506000848281518110611f6357611f636153ba565b91151560209283029190910190910152611f7c81615555565b9050611e75565b5060005b8181101561206657611fa5600a8281548110611e9357611e936153ba565b6009548790611fb490846153f9565b81518110611fc457611fc46153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050611ffa600a8281548110611f0657611f066153ba565b600954869061200990846153f9565b81518110612019576120196153ba565b6020908102919091010152600954600190859061203690846153f9565b81518110612046576120466153ba565b9115156020928302919091019091015261205f81615555565b9050611f87565b505050909192565b600654600090600160a01b900460ff161561209b5760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b1790556000806120bb60016136c8565b915091506120ca858383613ccf565b9250826000036120ed57604051639768300560e01b815260040160405180910390fd5b6007546001600160c01b031661210386836153f9565b11156121225760405163adea3dfd60e01b815260040160405180910390fd5b611bc2838686613c34565b60018054610c619061535a565b6006546001600160a01b031633146121645760405162461bcd60e51b8152600401610c3c90615394565b61216c6136fb565b63ffffffff83166000908152600b602052604090205460ff16156121ab5760405163335894fb60e11b815263ffffffff84166004820152602401610c3c565b63ffffffff83166000908152600d602052604090205460ff166121e957604051631f9db01d60e31b815263ffffffff84166004820152602401610c3c565b6040516385ae5d5760e01b815263ffffffff84166004820152600090819081906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906385ae5d5790602401600060405180830381865afa15801561225a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612282919081019061556e565b925092509250831515821515146122b457604051632b1d0bd360e11b815263ffffffff87166004820152602401610c3c565b604080516080810182526001600160a01b0380861682528415156020808401918252838501868152606085018b905263ffffffff8c166000908152600c9092529490208351815492511515600160a01b026001600160a81b03199093169316929092171781559151909190600182019061232e9082615653565b50606082015160028201906123439082615653565b50905050811561238457600a546020116123735760405163f025236d60e01b815260206004820152602401610c3c565b61237f600a8888613cdc565b6123b6565b6009546020116123aa5760405163f025236d60e01b815260206004820152602401610c3c565b6123b660098888613cdc565b63ffffffff86166000908152600b602052604090819020805460ff19166001179055517fc4f8cb57c016f0b294fff2666f86fa6cfee9b03aed19f816ae4bf44b7e837bbb9061241a9088908a9063ffffffff92831681529116602082015260400190565b60405180910390a150505050505050565b6006546001600160a01b031633146124555760405162461bcd60e51b8152600401610c3c90615394565b61012c81108061246757506202a30081115b1561248557604051633a60233f60e21b815260040160405180910390fd5b601380549082905560408051828152602081018490527f227ff5c6b5ffb395236b09fd1b472bb128b36eaa17556633feefe28e94411f24910161192e565b6006546001600160a01b031633146124ed5760405162461bcd60e51b8152600401610c3c90615394565b6000816125385760098463ffffffff168154811061250d5761250d6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16612578565b600a8463ffffffff1681548110612551576125516153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff165b90508063ffffffff168363ffffffff1614158061261e57506040516321a0f75360e01b815263ffffffff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906321a0f75390602401602060405180830381865afa1580156125fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061261e9190615712565b1561263c5760405163d4db0b7960e01b815260040160405180910390fd5b61108d8484846138ee565b6006546001600160a01b031633146126715760405162461bcd60e51b8152600401610c3c90615394565b600654600160b01b900460ff1661268957600161268c565b60005b60068054911515600160b01b0260ff60b01b19909216919091179055565b60006126b46130c4565b600654600160a01b900460ff16156126de5760405162461bcd60e51b8152600401610c3c90615336565b610c4f600161317d565b60006126f333613726565b6126fd8383613ea1565b9392505050565b60005b8181101561278657612773838383818110612724576127246153ba565b9050602002810190612736919061572f565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152503093925050613f079050565b508061277e81615555565b915050612707565b505050565b6006546001600160a01b031633146127b55760405162461bcd60e51b8152600401610c3c90615394565b6007546001600160c01b039081169082161015611994576040516334f1ec1b60e01b815260040160405180910390fd5b6006546001600160a01b0316331461280f5760405162461bcd60e51b8152600401610c3c90615394565b601054604080516001600160a01b03928316815291831660208301527f51dbb5a65bb22737861a63ec12ba6ce78a98631e9404b0567a2eaf7a06fc544d910160405180910390a1601080546001600160a01b0319166001600160a01b0392909216919091179055565b600654600090600160b01b900460ff1661291457604051630ad85dff60e41b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ad85dff090602401602060405180830381865afa1580156128f0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c4f9190615712565b50600090565b600080600061292960016136c8565b91509150610e6b848383613ccf565b600654600090600160a01b900460ff16156129655760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080612984816136c8565b915091506129938683836136ee565b92506129a186848787613f2c565b50506006805460ff60a01b191690559392505050565b6006546001600160a01b031633146129e15760405162461bcd60e51b8152600401610c3c90615394565b670de0b6b3a76400006001600160401b0382161115612a1357604051633d0203e560e01b815260040160405180910390fd5b600f54604080516001600160401b03928316815291831660208301527fb5cc994a260a85a42d6588668221571ae0a14f0a28f9e4817a5195262102c868910160405180910390a1600f805467ffffffffffffffff19166001600160401b0392909216919091179055565b600654600090600160a01b900460ff1615612aaa5760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080612ac9816136c8565b91509150612ad88683836136e1565b925082600003612afb57604051639768300560e01b815260040160405180910390fd5b6129a183878787613f2c565b6006546001600160a01b03163314612b315760405162461bcd60e51b8152600401610c3c90615394565b6000808415612b78576000612b44610c08565b9050612b63612b5586612710615775565b829061ffff16612710614005565b9250612b74612b5586612710615790565b9150505b612b83600284614024565b600880546001600160a01b0319166001600160a01b0385161790556000612ba8610c08565b90508515612bec5782811080612bbd57508181115b15612bec5760405163628cc47560e11b8152600481018290526024810184905260448101839052606401610c3c565b505050505050565b600654600090600160a81b900460ff1615612c1157506000919050565b6007546001600160c01b03166002600160c01b03198101612c36575060001992915050565b6002546001600160c01b038216811015612c6257612c5d816001600160c01b0384166153e6565b610e6b565b6000949350505050565b6000806000612c7b60016136c8565b91509150610e6b848383613c27565b6006546001600160a01b03163314612cb45760405162461bcd60e51b8152600401610c3c90615394565b612cbe8282614024565b601180546001600160a01b0319166001600160a01b0383169081179091556040519081527f3ced9f0d0ac37f3370e1e00515182a375773698b50f5a46523e2cb37360155839060200161192e565b600654600090600160a01b900460ff1615612d395760405162461bcd60e51b8152600401610c3c90615336565b610eda826000614105565b6006546001600160a01b03163314612d6e5760405162461bcd60e51b8152600401610c3c90615394565b63ffffffff81166000818152600d60209081526040808320805460ff191690558051938452908301919091527fea052d1fb1ecba6aaf6bd32e92f20e7b6a094eaa478248322cc8ff024a90978f91016114bb565b42841015612e125760405162461bcd60e51b815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152606401610c3c565b60006001612e1e611093565b6001600160a01b038a811660008181526005602090815260409182902080546001810190915582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98184015280840194909452938d166060840152608083018c905260a083019390935260c08083018b90528151808403909101815260e08301909152805192019190912061190160f01b6101008301526101028201929092526101228101919091526101420160408051601f198184030181528282528051602091820120600084529083018083525260ff871690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015612f2a573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811615801590612f605750876001600160a01b0316816001600160a01b0316145b612f9d5760405162461bcd60e51b815260206004820152600e60248201526d24a72b20a624a22fa9a4a3a722a960911b6044820152606401610c3c565b6001600160a01b0390811660009081526004602090815260408083208a8516808552908352928190208990555188815291928a16917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a350505050505050565b600654600090600160a01b900460ff16156130335760405162461bcd60e51b8152600401610c3c90615336565b610eda826001614105565b600a81815481106119c657600080fd5b6006546001600160a01b031633146130785760405162461bcd60e51b8152600401610c3c90615394565b600680546001600160a01b0319166001600160a01b03831690811790915560405133907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a350565b600654600160b01b900460ff1661317b57604051630ad85dff60e41b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ad85dff090602401602060405180830381865afa158015613139573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061315d9190615712565b1561317b57604051630f301f8f60e41b815260040160405180910390fd5b565b60095460009081816001600160401b0381111561319c5761319c614cf5565b6040519080825280602002602001820160405280156131c5578160200160208202803683370190505b5090506000826001600160401b038111156131e2576131e2614cf5565b60405190808252806020026020018201604052801561320b578160200160208202803683370190505b50905084156133795760005b838110156132dc57600060098281548110613234576132346153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061326581614161565b838381518110613277576132776153ba565b602002602001018181525060000361328f57506132cc565b6132988161363f565b8483815181106132aa576132aa6153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b6132d581615555565b9050613217565b5060085460405163b333a17560e01b81526001600160a01b039091169063b333a1759061333190859085907f0000000000000000000000000000000000000000000000000000000000000000906004016157ab565b602060405180830381865afa15801561334e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061337291906157e9565b93506115e4565b600a546000816001600160401b0381111561339657613396614cf5565b6040519080825280602002602001820160405280156133bf578160200160208202803683370190505b5090506000826001600160401b038111156133dc576133dc614cf5565b604051908082528060200260200182016040528015613405578160200160208202803683370190505b50905060005b868110156134d057600060098281548110613428576134286153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061345981613865565b86838151811061346b5761346b6153ba565b602002602001018181525060000361348357506134c0565b61348c8161363f565b87838151811061349e5761349e6153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b6134c981615555565b905061340b565b5060005b83811015613599576000600a82815481106134f1576134f16153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061352281613865565b838381518110613534576135346153ba565b602002602001018181525060000361354c5750613589565b6135558161363f565b848381518110613567576135676153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b61359281615555565b90506134d4565b50600854604051637563738b60e11b81526001600160a01b039091169063eac6e716906135f29088908890879087907f000000000000000000000000000000000000000000000000000000000000000090600401615802565b602060405180830381865afa15801561360f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061363391906157e9565b98975050505050505050565b63ffffffff81166000908152600c60205260408082208054915163e170a9bf60e01b81526001600160a01b0390921691829163e170a9bf9161368791600101906004016158e7565b602060405180830381865afa1580156136a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126fd91906158fa565b6000806136d5600061317d565b91506002549050915091565b6000610e6b848484614005565b6000610e6b848385613a74565b600654600160a81b900460ff161561317b576040516337a5332d60e11b815260040160405180910390fd5b6001600160a01b03811660009081526014602052604090205480156137815760006013548261375591906153f9565b905042811115612786576040516306f8ee3f60e21b815260048101829052426024820152604401610c3c565b5050565b6001600160a01b038316600090815260046020908152604080832033845290915281205460001981146137e1576137bc83826153e6565b6001600160a01b03861660009081526004602090815260408083203384529091529020555b6001600160a01b038516600090815260036020526040812080548592906138099084906153e6565b90915550506001600160a01b0380851660008181526003602052604090819020805487019055519091871690600080516020615c00833981519152906138529087815260200190565b60405180910390a3506001949350505050565b63ffffffff81166000908152600c602052604080822080549151637841536560e01b81526001600160a01b039092169182916378415365916138ad91600101906004016158e7565b602060405180830381865afa1580156138ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126fd91906157e9565b60065463ffffffff600160c01b909104811690831603613921576040516319ded73160e21b815260040160405180910390fd5b801561393757613932600a8461421b565b613942565b61394260098461421b565b63ffffffff82166000908152600b60209081526040808320805460ff19169055600c909152812080546001600160a81b0319168155906139856001830182614bae565b613993600283016000614bae565b50506040805163ffffffff8085168252851660208201527fa5cd0099b78b279c04987aa80ffffaf8fc8c8af4e7c7bce2686e8d01e2e1bd51910160405180910390a1505050565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6000604051613a0c9190615917565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b828202811515841585830485141716613a8c57600080fd5b6001826001830304018115150290509392505050565b60005b8151811015613781576000828281518110613ac257613ac26153ba565b602090810291909101810151516001600160a01b0381166000908152600e90925260409091205490915060ff16613b1757604051635df6b61760e11b81526001600160a01b0382166004820152602401610c3c565b60005b838381518110613b2c57613b2c6153ba565b60200260200101516020015151811015613c1457613b92848481518110613b5557613b556153ba565b6020026020010151602001518281518110613b7257613b726153ba565b6020026020010151836001600160a01b0316613f0790919063ffffffff16565b507f7445c6598e1b553f076d507692eab3dceef0d608757141b53e9e56aa8bbaf48382858581518110613bc757613bc76153ba565b6020026020010151602001518381518110613be457613be46153ba565b6020026020010151604051613bfa92919061598d565b60405180910390a180613c0c81615555565b915050613b1a565b505080613c2090615555565b9050613aa5565b6000610e6b848385614005565b613c3f838383614360565b613c746001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333086614422565b613c7e81836144ac565b60408051848152602081018490526001600160a01b0383169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d7910160405180910390a3612786838383614506565b6000610e6b848484613a74565b82548015613e60578380613cf16001846153e6565b81548110613d0157613d016153ba565b60009182526020808320600880840490910154855460018082018855968652928520918304909101805463ffffffff60046007958616810261010090810a83810219909416969097160290950a9092049093160217905590613d6390836153e6565b90505b8363ffffffff16811115613e0e5784613d806001836153e6565b81548110613d9057613d906153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16858281548110613dc857613dc86153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff1602179055508080613e06906159b1565b915050613d66565b5081848463ffffffff1681548110613e2857613e286153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff16021790555061108d565b508254600181018455600093845260209093206008840401805460079094166004026101000a63ffffffff8181021990951692909416939093021790915550565b33600090815260036020526040812080548391908390613ec29084906153e6565b90915550506001600160a01b03831660008181526003602052604090819020805485019055513390600080516020615c0083398151915290610ece9086815260200190565b60606126fd8383604051806060016040528060278152602001615bd96027913961452c565b613f388484848461459a565b336001600160a01b03821614613fa6576001600160a01b03811660009081526004602090815260408083203384529091529020546000198114613fa457613f7f84826153e6565b6001600160a01b03831660009081526004602090815260408083203384529091529020555b505b613fb081846145af565b60408051858152602081018590526001600160a01b03808416929085169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a461108d8483614611565b82820281151584158583048514171661401d57600080fd5b0492915050565b8160000361404557604051632db38d0560e01b815260040160405180910390fd5b806001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b93f9b0a846040518263ffffffff1660e01b815260040161409d91815260200190565b602060405180830381865afa1580156140ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906140de91906158fa565b6001600160a01b03161461378157604051634ee204d760e01b815260040160405180910390fd5b6000614111838361497b565b6001600160a01b038416600090815260146020526040902054909150801561415a5760006013548261414391906153f9565b90504281111561415857600092505050610eda565b505b5092915050565b63ffffffff81166000908152600c6020526040812054600160a01b900460ff161561418e57506000919050565b63ffffffff82166000908152600c60205260409081902080549151637d2872e960e11b81526001600160a01b039092169163fa50e5d2916141da916001820191600201906004016159c8565b602060405180830381865afa1580156141f7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610eda91906157e9565b815463ffffffff821681116142685760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610c3c565b63ffffffff82165b61427b6001836153e6565b81101561431c578361428e8260016153f9565b8154811061429e5761429e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff168482815481106142d6576142d66153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff160217905550808061431490615555565b915050614270565b508280548061432d5761432d6159f6565b600082815260209020600860001990920191820401805463ffffffff600460078516026101000a02191690559055505050565b61436b8383836149fd565b336001600160a01b0382161461278657604051635551e1b560e01b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635551e1b590602401602060405180830381865afa1580156143df573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906144039190615712565b612786576040516334871f2560e21b8152336004820152602401610c3c565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806144a55760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610c3c565b5050505050565b80600260008282546144be91906153f9565b90915550506001600160a01b038216600081815260036020908152604080832080548601905551848152600080516020615c0083398151915291015b60405180910390a35050565b6001600160a01b0381166000908152601460205260409020429055612786838383614a0d565b6060600080856001600160a01b0316856040516145499190615a0c565b600060405180830381855af49150503d8060008114614584576040519150601f19603f3d011682016040523d82523d6000602084013e614589565b606091505b50915091506115da86838387614a27565b6145a381613726565b61108d84848484614aa0565b6001600160a01b038216600090815260036020526040812080548392906145d79084906153e6565b90915550506002805482900390556040518181526000906001600160a01b03841690600080516020615c00833981519152906020016144fa565b61463c6040518060800160405280600081526020016000815260200160008152602001600081525090565b600854604051630226614760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116600483015290911690630226614790602401602060405180830381865afa1580156146a6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906146ca91906157e9565b60408201526146fa7f0000000000000000000000000000000000000000000000000000000000000000600a615b0c565b606082015260095460005b8181101561495857600060098281548110614722576147226153ba565b60009182526020822060088204015460079091166004026101000a900463ffffffff16915061475082614161565b905080600003614761575050614948565b600061476c8361363f565b600854604051630226614760e01b81526001600160a01b038084166004830152929350911690630226614790602401602060405180830381865afa1580156147b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906147dc91906157e9565b866000018181525050806001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015614823573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906148479190615b1b565b61485290600a615b0c565b602087018190528651600091829161487d9161487687670de0b6b3a7640000615b38565b9190614005565b905061489c88606001518960400151836140059092919063ffffffff16565b91506148b0670de0b6b3a764000083615b4f565b9150506000898211156149165760006148e089604001518a606001518d670de0b6b3a76400006148769190615b38565b60208a01518a519192506148f691839190614005565b915061490a670de0b6b3a764000083615b4f565b915060009a5050614925565b5082614922828b6153e6565b99505b61493085828b614aa8565b89600003614942575050505050614958565b50505050505b61495181615555565b9050614705565b50831561108d5760405163cc5ea39b60e01b815260048101859052602401610c3c565b60006149856130c4565b60008061499260006136c8565b6001600160a01b038716600090815260036020526040812054929450909250906149bd9084846136e1565b905060006149cb600161317d565b9050808211156149db57806149dd565b815b945085156149f3576149f0858585613c27565b94505b5050505092915050565b614a056136fb565b6127866130c4565b60065461278690600160c01b900463ffffffff1684614b39565b60608315614a96578251600003614a8f576001600160a01b0385163b614a8f5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610c3c565b5081610e6b565b610e6b8383614b84565b61108d6130c4565b63ffffffff83166000908152600c602052604090819020805491516001600160a01b03909216916144a59163c9111bd760e01b91614af791879187916001810191600290910190602401615b71565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526001600160a01b03831690613f07565b63ffffffff82166000908152600c602052604090819020805491516001600160a01b039092169161108d916369445c3160e01b91614af7918691600182019160020190602401615bad565b815115614b945781518083602001fd5b8060405162461bcd60e51b8152600401610c3c9190614c54565b508054614bba9061535a565b6000825580601f10614bca575050565b601f016020900490600052602060002090810190614be89190614beb565b50565b5b80821115614c005760008155600101614bec565b5090565b60005b83811015614c1f578181015183820152602001614c07565b50506000910152565b60008151808452614c40816020860160208601614c04565b601f01601f19169290920160200192915050565b6020815260006126fd6020830184614c28565b803563ffffffff81168114614c7b57600080fd5b919050565b600060208284031215614c9257600080fd5b6126fd82614c67565b600060208284031215614cad57600080fd5b5035919050565b6001600160a01b0381168114614be857600080fd5b60008060408385031215614cdc57600080fd5b8235614ce781614cb4565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715614d2d57614d2d614cf5565b60405290565b604051601f8201601f191681016001600160401b0381118282101715614d5b57614d5b614cf5565b604052919050565b60006001600160401b03821115614d7c57614d7c614cf5565b50601f01601f191660200190565b600082601f830112614d9b57600080fd5b8135614dae614da982614d63565b614d33565b818152846020838601011115614dc357600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060808587031215614df657600080fd5b8435614e0181614cb4565b93506020850135614e1181614cb4565b92506040850135915060608501356001600160401b03811115614e3357600080fd5b614e3f87828801614d8a565b91505092959194509250565b600060208284031215614e5d57600080fd5b81356126fd81614cb4565b600080600060608486031215614e7d57600080fd5b8335614e8881614cb4565b92506020840135614e9881614cb4565b929592945050506040919091013590565b8015158114614be857600080fd5b60008060408385031215614eca57600080fd5b614ed383614c67565b91506020830135614ee381614ea9565b809150509250929050565b600080600060608486031215614f0357600080fd5b614f0c84614c67565b9250614f1a60208501614c67565b91506040840135614f2a81614ea9565b809150509250925092565b6020808252825182820181905260009190848201906040850190845b81811015614f7357835163ffffffff1683529284019291840191600101614f51565b50909695505050505050565b60008083601f840112614f9157600080fd5b5081356001600160401b03811115614fa857600080fd5b6020830191508360208260051b8501011115614fc357600080fd5b9250929050565b60008060208385031215614fdd57600080fd5b82356001600160401b03811115614ff357600080fd5b614fff85828601614f7f565b90969095509350505050565b60006020828403121561501d57600080fd5b81356001600160c01b03811681146126fd57600080fd5b6000806040838503121561504757600080fd5b823591506020830135614ee381614cb4565b6001600160a01b0385168152831515602082015260806040820181905260009061508590830185614c28565b82810360608401526150978185614c28565b979650505050505050565b600081518084526020808501945080840160005b838110156150db5781516001600160a01b0316875295820195908201906001016150b6565b509495945050505050565b600081518084526020808501945080840160005b838110156150db578151875295820195908201906001016150fa565b60608152600061512960608301866150a2565b60208382038185015261513c82876150e6565b8481036040860152855180825282870193509082019060005b81811015615173578451151583529383019391830191600101615155565b509098975050505050505050565b6000806000806080858703121561519757600080fd5b6151a085614c67565b93506151ae60208601614c67565b925060408501356001600160401b038111156151c957600080fd5b6151d587828801614d8a565b92505060608501356151e681614ea9565b939692955090935050565b60008060006060848603121561520657600080fd5b83359250602084013561521881614cb4565b91506040840135614f2a81614cb4565b60006020828403121561523a57600080fd5b81356001600160401b03811681146126fd57600080fd5b60008060006060848603121561526657600080fd5b833561527181614ea9565b9250602084013561ffff8116811461521857600080fd5b60ff81168114614be857600080fd5b600080600080600080600060e0888a0312156152b257600080fd5b87356152bd81614cb4565b965060208801356152cd81614cb4565b9550604088013594506060880135935060808801356152eb81615288565b9699959850939692959460a0840135945060c09093013592915050565b6000806040838503121561531b57600080fd5b823561532681614cb4565b91506020830135614ee381614cb4565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b600181811c9082168061536e57607f821691505b60208210810361538e57634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b81810381811115610eda57610eda6153d0565b80820180821115610eda57610eda6153d0565b60006001600160401b0382111561542557615425614cf5565b5060051b60200190565b600061543d614da98461540c565b83815260208082019190600586811b86013681111561545b57600080fd5b865b818110156155485780356001600160401b038082111561547d5760008081fd5b818a019150604082360312156154935760008081fd5b61549b614d0b565b82356154a681614cb4565b815282870135828111156154ba5760008081fd5b929092019136601f8401126154cf5760008081fd5b82356154dd614da98261540c565b81815290871b840188019088810190368311156154fa5760008081fd5b8986015b83811015615532578035868111156155165760008081fd5b615524368d838b0101614d8a565b845250918a01918a016154fe565b50838a015250508852505094830194830161545d565b5092979650505050505050565b600060018201615567576155676153d0565b5060010190565b60008060006060848603121561558357600080fd5b835161558e81614cb4565b602085015190935061559f81614ea9565b60408501519092506001600160401b038111156155bb57600080fd5b8401601f810186136155cc57600080fd5b80516155da614da982614d63565b8181528760208385010111156155ef57600080fd5b615600826020830160208601614c04565b8093505050509250925092565b601f82111561278657600081815260208120601f850160051c810160208610156156345750805b601f850160051c820191505b81811015612bec57828155600101615640565b81516001600160401b0381111561566c5761566c614cf5565b6156808161567a845461535a565b8461560d565b602080601f8311600181146156b5576000841561569d5750858301515b600019600386901b1c1916600185901b178555612bec565b600085815260208120601f198616915b828110156156e4578886015182559484019460019091019084016156c5565b50858210156157025787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b60006020828403121561572457600080fd5b81516126fd81614ea9565b6000808335601e1984360301811261574657600080fd5b8301803591506001600160401b0382111561576057600080fd5b602001915036819003821315614fc357600080fd5b61ffff82811682821603908082111561415a5761415a6153d0565b61ffff81811683821601908082111561415a5761415a6153d0565b6060815260006157be60608301866150a2565b82810360208401526157d081866150e6565b91505060018060a01b0383166040830152949350505050565b6000602082840312156157fb57600080fd5b5051919050565b60a08152600061581560a08301886150a2565b828103602084015261582781886150e6565b9050828103604084015261583b81876150a2565b9050828103606084015261584f81866150e6565b91505060018060a01b03831660808301529695505050505050565b600081546158778161535a565b80855260206001838116801561589457600181146158ae576158dc565b60ff1985168884015283151560051b8801830195506158dc565b866000528260002060005b858110156158d45781548a82018601529083019084016158b9565b890184019650505b505050505092915050565b6020815260006126fd602083018461586a565b60006020828403121561590c57600080fd5b81516126fd81614cb4565b60008083546159258161535a565b6001828116801561593d576001811461595257615981565b60ff1984168752821515830287019450615981565b8760005260208060002060005b858110156159785781548a82015290840190820161595f565b50505082870194505b50929695505050505050565b6001600160a01b0383168152604060208201819052600090610e6b90830184614c28565b6000816159c0576159c06153d0565b506000190190565b6040815260006159db604083018561586a565b82810360208401526159ed818561586a565b95945050505050565b634e487b7160e01b600052603160045260246000fd5b60008251615a1e818460208701614c04565b9190910192915050565b600181815b80851115615a63578160001904821115615a4957615a496153d0565b80851615615a5657918102915b93841c9390800290615a2d565b509250929050565b600082615a7a57506001610eda565b81615a8757506000610eda565b8160018114615a9d5760028114615aa757615ac3565b6001915050610eda565b60ff841115615ab857615ab86153d0565b50506001821b610eda565b5060208310610133831016604e8410600b8410161715615ae6575081810a610eda565b615af08383615a28565b8060001904821115615b0457615b046153d0565b029392505050565b60006126fd60ff841683615a6b565b600060208284031215615b2d57600080fd5b81516126fd81615288565b8082028115828204841417610eda57610eda6153d0565b600082615b6c57634e487b7160e01b600052601260045260246000fd5b500490565b8481526001600160a01b0384166020820152608060408201819052600090615b9b9083018561586a565b8281036060840152615097818561586a565b838152606060208201526000615bc6606083018561586a565b82810360408401526115da818561586a56fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa264697066735822122033bcfe5c8070dd09d9233cec0bd62bca9ecdff131eeb7dbe2a4960a32bfce8f064736f6c634300081500338be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564\",\n    \"sourceMap\": \"19032:174:92:-:0;134:5676:99;19032:174:92;19077:7;134:5676:99;19032:174:92;19111:7;19032:174;;-1:-1:-1;19032:174:92;;;;;;18999:207;;;-1:-1:-1;;;;;;18999:207:92;;;;;;;;-1:-1:-1;;;;;;18999:207:92;;;49997:9;49954:52;;1984:6:99;2138:58;;214:629;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;577:6;597:9;620:6;640:5;659:7;680:16;710:22;746:15;775:22;811:15;24807:10:92;24777:6;24785:5;24792:7;1290:5:59;1297:7;1306:6;-1:-1:-1;;;;;1306:15:59;;:17;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;2094:4:60;:12;2101:5;2094:4;:12;:::i;:::-;-1:-1:-1;2116:6:60;:16;2125:7;2116:6;:16;:::i;:::-;-1:-1:-1;2142:20:60;;;;;2192:13;2173:32;;2242:24;:22;:24::i;:::-;2215:51;;-1:-1:-1;;;;;;;;;1335:14:59;;::::1;;::::0;-1:-1:-1;1045:5:58;:14;;-1:-1:-1;;;;;;1045:14:58;;;;;;;;;1075:40;;-1:-1:-1;;;;;;;;;;;;;1075:40:58;-1:-1:-1;;1075:40:58;-1:-1:-1;;;;;;24829:20:92;::::2;;::::0;;;24885:48:::2;::::0;-1:-1:-1;;;24885:48:92;;22957:1:::2;24885:48;::::0;::::2;6308:25:238::0;24885:20:92::2;::::0;6281:18:238;;24885:48:92::2;;;;;;;;;;;;;;;;;::::0;::::2;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;24859:11;:75:::0;;-1:-1:-1;;;;;;24859:75:92::2;-1:-1:-1::0;;;;;24859:75:92;;;::::2;::::0;;;::::2;::::0;;24985:40:::2;25008:16:::0;24985:22:::2;:40::i;:::-;25035:63;25047:1;25050:16:::0;25068:22;25047:1;25035:11:::2;:63::i;:::-;25108:36;25127:16:::0;25108:18:::2;:36::i;:::-;25191:14;:32:::0;;-1:-1:-1;;;;;;25191:32:92::2;-1:-1:-1::0;;;;;25191:32:92;::::2;;::::0;;23110:3:::2;25238:42:::0;::::2;25234:93;;;25289:38;;-1:-1:-1::0;;;25289:38:92::2;;;;;;;;;;;25234:93;25407:63;-1:-1:-1::0;;;;;25407:23:92;::::2;25431:6:::0;25447:4:::2;25454:15:::0;25407:23:::2;:63::i;:::-;25541:34;25547:10;25559:15:::0;25541:5:::2;:34::i;:::-;25643:45;25654:16:::0;25672:15;25643:10:::2;:45::i;:::-;25699:7;:54:::0;;-1:-1:-1;;;;;;25699:54:92::2;-1:-1:-1::0;;;;;25699:54:92;::::2;;::::0;;25763:25:::2;25781:6:::0;25763:17:::2;:25::i;:::-;24434:1361:::0;;;;;;;;;;214:629:99;;;;;;;;;;134:5676;;5510:446:60;5575:7;5672:95;5805:4;5789:22;;;;;;:::i;:::-;;;;;;;;;;5640:295;;;7708:25:238;;;;7749:18;;7742:34;;;;5833:14:60;7792:18:238;;;7785:34;5869:13:60;7835:18:238;;;7828:34;5912:4:60;7878:19:238;;;7871:61;7680:19;;5640:295:60;;;;;;;;;;;;5613:336;;;;;;5594:355;;5510:446;:::o;10633:297:92:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;8145:2:238;756:44:58;;;8127:21:238;8184:2;8164:18;;;8157:30;-1:-1:-1;;;8203:18:238;;;8196:42;8255:18;;756:44:58;;;;;;;;;10771:8:92::1;::::0;:49:::1;::::0;-1:-1:-1;;;10771:49:92;;::::1;8446:23:238::0;;10771:49:92::1;::::0;::::1;8428:42:238::0;-1:-1:-1;;;;;10771:37:92;;::::1;::::0;::::1;::::0;8401:18:238;;10771:49:92::1;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;::::0;::::1;;;;;-1:-1:-1::0;;;;10830:29:92::1;::::0;::::1;;::::0;;;:17:::1;:29;::::0;;;;;;;;:36;;-1:-1:-1;;10830:36:92::1;10862:4;10830:36:::0;;::::1;::::0;;;10881:42;;8647::238;;;8705:18;;;8698:50;10881:42:92::1;::::0;8620:18:238;10881:42:92::1;;;;;;;10633:297:::0;:::o;12378:1626::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;8145:2:238;756:44:58;;;8127:21:238;8184:2;8164:18;;;8157:30;-1:-1:-1;;;8203:18:238;;;8196:42;8255:18;;756:44:58;7943:336:238;756:44:58;12547:18:92::1;:16;:18::i;:::-;12632:26;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;::::1;;12628:78;;;12667:39;::::0;-1:-1:-1;;;12667:39:92;;8458:10:238;8446:23;;12667:39:92::1;::::0;::::1;8428:42:238::0;8401:18;;12667:39:92::1;8284:192:238::0;12628:78:92::1;12781:29;::::0;::::1;;::::0;;;:17:::1;:29;::::0;;;;;::::1;;12776:85;;12819:42;::::0;-1:-1:-1;;;12819:42:92;;8458:10:238;8446:23;;12819:42:92::1;::::0;::::1;8428::238::0;8401:18;;12819:42:92::1;8284:192:238::0;12776:85:92::1;13045:8;::::0;:40:::1;::::0;-1:-1:-1;;;13045:40:92;;::::1;8446:23:238::0;;13045:40:92::1;::::0;::::1;8428:42:238::0;12987:15:92::1;::::0;;;;;-1:-1:-1;;;;;13045:28:92::1;::::0;::::1;::::0;8401:18:238;;13045:40:92::1;;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;;;;;;::::0;;::::1;-1:-1:-1::0;;13045:40:92::1;::::0;::::1;;::::0;::::1;::::0;;;::::1;::::0;::::1;:::i;:::-;12986:99;;;;;;13110:11;13100:21;;:6;:21;;;13096:66;;13130:32;::::0;-1:-1:-1;;;13130:32:92;;8458:10:238;8446:23;;13130:32:92::1;::::0;::::1;8428:42:238::0;8401:18;;13130:32:92::1;8284:192:238::0;13096:66:92::1;13256:179;::::0;;::::1;::::0;::::1;::::0;;-1:-1:-1;;;;;13256:179:92;;::::1;::::0;;;::::1;;;::::0;;::::1;::::0;;;;;;;;;;;;;;;13226:27:::1;::::0;::::1;-1:-1:-1::0;13226:27:92;;;:15:::1;:27:::0;;;;;;:209;;;;;;::::1;;-1:-1:-1::0;;;13226:209:92::1;-1:-1:-1::0;;;;;;13226:209:92;;;;::::1;::::0;;;;::::1;::::0;;;;13256:179;;13226:27;:209;;::::1;::::0;::::1;::::0;;::::1;:::i;:::-;-1:-1:-1::0;13226:209:92::1;::::0;::::1;::::0;::::1;::::0;::::1;::::0;::::1;::::0;;::::1;:::i;:::-;;;;;13450:6;13446:460;;;13476:13;:20:::0;9610:2:::1;-1:-1:-1::0;13472:90:92::1;;13522:40;::::0;-1:-1:-1;;;13522:40:92;;9610:2:::1;13522:40;::::0;::::1;6308:25:238::0;6281:18;;13522:40:92::1;6162:177:238::0;13472:90:92::1;13630:36;:13;13648:5:::0;13655:10;13630:17:::1;:36::i;:::-;13446:460;;;13701:15;:22:::0;9610:2:::1;-1:-1:-1::0;13697:92:92::1;;13749:40;::::0;-1:-1:-1;;;13749:40:92;;9610:2:::1;13749:40;::::0;::::1;6308:25:238::0;6281:18;;13749:40:92::1;6162:177:238::0;13697:92:92::1;13857:38;:15;13877:5:::0;13884:10;13857:19:::1;:38::i;:::-;13916:26;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;;:33;;-1:-1:-1;;13916:33:92::1;13945:4;13916:33;::::0;;13965:32;::::1;::::0;::::1;::::0;13931:10;;13991:5;;10926:10:238;10963:15;;;10945:34;;11015:15;;11010:2;10995:18;;10988:43;10904:2;10889:18;;10745:292;13965:32:92::1;;;;;;;;12537:1467;;;12378:1626:::0;;;;:::o;9741:413::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;8145:2:238;756:44:58;;;8127:21:238;8184:2;8164:18;;;8157:30;-1:-1:-1;;;8203:18:238;;;8196:42;8255:18;;756:44:58;7943:336:238;756:44:58;9820:26:92::1;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;::::1;;9815:75;;9855:35;::::0;-1:-1:-1;;;9855:35:92;;8458:10:238;8446:23;;9855:35:92::1;::::0;::::1;8428:42:238::0;8401:18;;9855:35:92::1;8284:192:238::0;9815:75:92::1;9928:5;::::0;-1:-1:-1;;;;;9904:29:92::1;:20;9913:10:::0;9904:8:::1;:20::i;:::-;-1:-1:-1::0;;;;;9904:29:92::1;;9900:110;;9972:5;::::0;9988:20:::1;9997:10:::0;9988:8:::1;:20::i;:::-;9942:68;::::0;-1:-1:-1;;;9942:68:92;;-1:-1:-1;;;;;11272:15:238;;;9942:68:92::1;::::0;::::1;11254:34:238::0;11324:15;;11304:18;;;11297:43;11189:18;;9942:68:92::1;11042:304:238::0;9900:110:92::1;10024:27;::::0;::::1;;::::0;;;:15:::1;:27;::::0;;;;:34;-1:-1:-1;;;10024:34:92;::::1;;;10020:89;;;10067:42;::::0;-1:-1:-1;;;10067:42:92;;8458:10:238;8446:23;;10067:42:92::1;::::0;::::1;8428::238::0;8401:18;;10067:42:92::1;8284:192:238::0;10020:89:92::1;10119:15;:28:::0;;::::1;::::0;;::::1;-1:-1:-1::0;;;10119:28:92::1;-1:-1:-1::0;;;;10119:28:92;;::::1;::::0;;;::::1;::::0;;9741:413::o;1328:1616:65:-;1466:12;1636:4;1630:11;-1:-1:-1;;;1759:17:65;1752:93;1892:4;1888:1;1869:17;1865:25;1858:39;1976:2;1971;1952:17;1948:26;1941:38;2056:6;2051:2;2032:17;2028:26;2021:42;2860:2;2857:1;2852:3;2833:17;2830:1;2823:5;2816;2811:52;2379:16;2372:24;2366:2;2348:16;2345:24;2341:1;2337;2331:8;2328:15;2324:46;2321:76;2121:756;2110:767;;;2905:7;2897:40;;;;-1:-1:-1;;;2897:40:65;;11553:2:238;2897:40:65;;;11535:21:238;11592:2;11572:18;;;11565:30;11631:22;11611:18;;;11604:50;11671:18;;2897:40:65;11351:344:238;2897:40:65;1456:1488;1328:1616;;;;:::o;6150:325:60:-;6235:6;6220:11;;:21;;;;;;;:::i;:::-;;;;-1:-1:-1;;;;;;;6387:13:60;;;;;;:9;:13;;;;;;;;:23;;;;;;6436:32;6308:25:238;;;6436:32:60;;6281:18:238;6436:32:60;;;;;;;6150:325;;:::o;56565:414:92:-;56655:25;;;56637:15;56655:25;;;:15;:25;;;;;;;:33;;56740:222;;-1:-1:-1;;;;;56655:33:92;;;;56698:274;;-1:-1:-1;;;56780:28:92;56740:222;;56826:6;;56655:33;56850:37;;;56905:43;;;56740:222;;;:::i;:::-;;;;;;;-1:-1:-1;;56740:222:92;;;;;;;;;;;-1:-1:-1;;;;;;56740:222:92;;;;-1:-1:-1;;;;;56740:222:92;;;;;;-1:-1:-1;;;;;56698:28:92;;;;:274;:::i;:::-;;56627:352;56565:414;;:::o;1312:161:58:-;778:5;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;8145:2:238;756:44:58;;;8127:21:238;8184:2;8164:18;;;8157:30;-1:-1:-1;;;8203:18:238;;;8196:42;8255:18;;756:44:58;7943:336:238;756:44:58;1392:5:::1;:16:::0;;-1:-1:-1;;;;;;1392:16:58::1;-1:-1:-1::0;;;;;1392:16:58;::::1;::::0;;::::1;::::0;;;1424:42:::1;::::0;1445:10:::1;::::0;-1:-1:-1;;;;;;;;;;;1424:42:58;-1:-1:-1;;1424:42:58::1;1312:161:::0;:::o;21872:108:92:-;21928:10;;-1:-1:-1;;;21928:10:92;;;;21924:49;;;21947:26;;-1:-1:-1;;;21947:26:92;;;;;;;;;;;21924:49;21872:108::o;493:354:206:-;591:12;;618:7;;614:227;;641:5;;658:7;664:1;658:3;:7;:::i;:::-;652:14;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;641:26;;;;;;;;;;;;;;;;;;;;;;652:14;;;641:26;;;;;652:14;641:26;;;;;;;;;;652:14;;;;;;;;;;;;;;641:26;;;;652:14;699:7;;:3;:7;:::i;:::-;687:19;;682:65;712:5;708:9;;:1;:9;682:65;;;735:5;741;745:1;741;:5;:::i;:::-;735:12;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;724:5;730:1;724:8;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;:23;;;;;;;;;;;;;;;;;;719:3;;;;;:::i;:::-;;;;682:65;;;;777:5;762;768;762:12;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;:20;;;;;;;;;;;;;;;;;;614:227;;;-1:-1:-1;813:17:206;;;;;;;-1:-1:-1;813:17:206;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;493:354:206:o;59041:217:92:-;59134:25;;;59099:5;59134:25;;;:15;:25;;;;;;:33;;59184:67;;-1:-1:-1;;;59184:67:92;;-1:-1:-1;;;;;59134:33:92;;;;;;59184:28;;:67;;59134:33;59213:37;;59184:67;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;59177:74;59041:217;-1:-1:-1;;;59041:217:92:o;6468:198:50:-;6551:12;6582:77;6603:6;6611:4;6582:77;;;;;;;;;;;;;;;;;:20;:77::i;:::-;6575:84;;6468:198;;;;;:::o;6852:325::-;6993:12;7018;7032:23;7059:6;-1:-1:-1;;;;;7059:19:50;7079:4;7059:25;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;7017:67:50;;-1:-1:-1;7017:67:50;-1:-1:-1;7101:69:50;7128:6;7017:67;;7157:12;7101:26;:69::i;:::-;7094:76;6852:325;-1:-1:-1;;;;;;6852:325:50:o;7465:628::-;7645:12;7673:7;7669:418;;;7700:10;:17;7721:1;7700:22;7696:286;;-1:-1:-1;;;;;1465:19:50;;;7907:60;;;;-1:-1:-1;;;7907:60:50;;14599:2:238;7907:60:50;;;14581:21:238;14638:2;14618:18;;;14611:30;14677:31;14657:18;;;14650:59;14726:18;;7907:60:50;14397:353:238;7907:60:50;-1:-1:-1;8002:10:50;7995:17;;7669:418;8043:33;8051:10;8063:12;8043:7;:33::i;:::-;7465:628;;;;;;:::o;8615:540::-;8774:17;;:21;8770:379;;9002:10;8996:17;9058:15;9045:10;9041:2;9037:19;9030:44;8770:379;9125:12;9118:20;;-1:-1:-1;;;9118:20:50;;;;;;;;:::i;14:131:238:-;-1:-1:-1;;;;;89:31:238;;79:42;;69:70;;135:1;132;125:12;69:70;14:131;:::o;150:138::-;229:13;;251:31;229:13;251:31;:::i;:::-;150:138;;;:::o;293:127::-;354:10;349:3;345:20;342:1;335:31;385:4;382:1;375:15;409:4;406:1;399:15;425:250;510:1;520:113;534:6;531:1;528:13;520:113;;;610:11;;;604:18;591:11;;;584:39;556:2;549:10;520:113;;;-1:-1:-1;;667:1:238;649:16;;642:27;425:250::o;680:699::-;734:5;787:3;780:4;772:6;768:17;764:27;754:55;;805:1;802;795:12;754:55;828:13;;-1:-1:-1;;;;;890:10:238;;;887:36;;;903:18;;:::i;:::-;978:2;972:9;946:2;1032:13;;-1:-1:-1;;1028:22:238;;;1052:2;1024:31;1020:40;1008:53;;;1076:18;;;1096:22;;;1073:46;1070:72;;;1122:18;;:::i;:::-;1162:10;1158:2;1151:22;1197:2;1189:6;1182:18;1243:3;1236:4;1231:2;1223:6;1219:15;1215:26;1212:35;1209:55;;;1260:1;1257;1250:12;1209:55;1273:76;1346:2;1339:4;1331:6;1327:17;1320:4;1312:6;1308:17;1273:76;:::i;1384:167::-;1462:13;;1515:10;1504:22;;1494:33;;1484:61;;1541:1;1538;1531:12;1556:175;1634:13;;-1:-1:-1;;;;;1676:30:238;;1666:41;;1656:69;;1721:1;1718;1711:12;1736:177;1815:13;;-1:-1:-1;;;;;1857:31:238;;1847:42;;1837:70;;1903:1;1900;1893:12;1918:1372;2129:6;2137;2145;2153;2161;2169;2177;2185;2193;2201;2254:3;2242:9;2233:7;2229:23;2225:33;2222:53;;;2271:1;2268;2261:12;2222:53;2294:40;2324:9;2294:40;:::i;:::-;2284:50;;2353:49;2398:2;2387:9;2383:18;2353:49;:::i;:::-;2343:59;;2421:49;2466:2;2455:9;2451:18;2421:49;:::i;:::-;2514:2;2499:18;;2493:25;2411:59;;-1:-1:-1;;;;;;2567:14:238;;;2564:34;;;2594:1;2591;2584:12;2564:34;2617:61;2670:7;2661:6;2650:9;2646:22;2617:61;:::i;:::-;2607:71;;2724:3;2713:9;2709:19;2703:26;2687:42;;2754:2;2744:8;2741:16;2738:36;;;2770:1;2767;2760:12;2738:36;2793:63;2848:7;2837:8;2826:9;2822:24;2793:63;:::i;:::-;2783:73;;2875:49;2919:3;2908:9;2904:19;2875:49;:::i;:::-;2865:59;;2970:3;2959:9;2955:19;2949:26;2933:42;;3000:2;2990:8;2987:16;2984:36;;;3016:1;3013;3006:12;2984:36;;3039:63;3094:7;3083:8;3072:9;3068:24;3039:63;:::i;:::-;3029:73;;;3142:3;3131:9;3127:19;3121:26;3111:36;;3166:49;3210:3;3199:9;3195:19;3166:49;:::i;:::-;3156:59;;3234:50;3279:3;3268:9;3264:19;3234:50;:::i;:::-;3224:60;;1918:1372;;;;;;;;;;;;;:::o;3295:273::-;3363:6;3416:2;3404:9;3395:7;3391:23;3387:32;3384:52;;;3432:1;3429;3422:12;3384:52;3464:9;3458:16;3514:4;3507:5;3503:16;3496:5;3493:27;3483:55;;3534:1;3531;3524:12;3573:380;3652:1;3648:12;;;;3695;;;3716:61;;3770:4;3762:6;3758:17;3748:27;;3716:61;3823:2;3815:6;3812:14;3792:18;3789:38;3786:161;;3869:10;3864:3;3860:20;3857:1;3850:31;3904:4;3901:1;3894:15;3932:4;3929:1;3922:15;3786:161;;3573:380;;;:::o;4084:545::-;4186:2;4181:3;4178:11;4175:448;;;4222:1;4247:5;4243:2;4236:17;4292:4;4288:2;4278:19;4362:2;4350:10;4346:19;4343:1;4339:27;4333:4;4329:38;4398:4;4386:10;4383:20;4380:47;;;-1:-1:-1;4421:4:238;4380:47;4476:2;4471:3;4467:12;4464:1;4460:20;4454:4;4450:31;4440:41;;4531:82;4549:2;4542:5;4539:13;4531:82;;;4594:17;;;4575:1;4564:13;4531:82;;;4535:3;;;4175:448;4084:545;;;:::o;4805:1352::-;4925:10;;-1:-1:-1;;;;;4947:30:238;;4944:56;;;4980:18;;:::i;:::-;5009:97;5099:6;5059:38;5091:4;5085:11;5059:38;:::i;:::-;5053:4;5009:97;:::i;:::-;5161:4;;5225:2;5214:14;;5242:1;5237:663;;;;5944:1;5961:6;5958:89;;;-1:-1:-1;6013:19:238;;;6007:26;5958:89;-1:-1:-1;;4762:1:238;4758:11;;;4754:24;4750:29;4740:40;4786:1;4782:11;;;4737:57;6060:81;;5207:944;;5237:663;4031:1;4024:14;;;4068:4;4055:18;;-1:-1:-1;;5273:20:238;;;5391:236;5405:7;5402:1;5399:14;5391:236;;;5494:19;;;5488:26;5473:42;;5586:27;;;;5554:1;5542:14;;;;5421:19;;5391:236;;;5395:3;5655:6;5646:7;5643:19;5640:201;;;5716:19;;;5710:26;-1:-1:-1;;5799:1:238;5795:14;;;5811:3;5791:24;5787:37;5783:42;5768:58;5753:74;;5640:201;-1:-1:-1;;;;;5887:1:238;5871:14;;;5867:22;5854:36;;-1:-1:-1;4805:1352:238:o;6344:251::-;6414:6;6467:2;6455:9;6446:7;6442:23;6438:32;6435:52;;;6483:1;6480;6473:12;6435:52;6515:9;6509:16;6534:31;6559:5;6534:31;:::i;6600:844::-;6730:3;6759:1;6792:6;6786:13;6822:36;6848:9;6822:36;:::i;:::-;6877:1;6894:18;;;6921:133;;;;7068:1;7063:356;;;;6887:532;;6921:133;-1:-1:-1;;6954:24:238;;6942:37;;7027:14;;7020:22;7008:35;;6999:45;;;-1:-1:-1;6921:133:238;;7063:356;7094:6;7091:1;7084:17;7124:4;7169:2;7166:1;7156:16;7194:1;7208:165;7222:6;7219:1;7216:13;7208:165;;;7300:14;;7287:11;;;7280:35;7343:16;;;;7237:10;;7208:165;;;7212:3;;;7402:6;7397:3;7393:16;7386:23;;6887:532;-1:-1:-1;7435:3:238;;6600:844;-1:-1:-1;;;;;;6600:844:238:o;8759:626::-;8853:6;8861;8869;8922:2;8910:9;8901:7;8897:23;8893:32;8890:52;;;8938:1;8935;8928:12;8890:52;8970:9;8964:16;8989:31;9014:5;8989:31;:::i;:::-;9089:2;9074:18;;9068:25;9039:5;;-1:-1:-1;9131:15:238;;9124:23;9112:36;;9102:64;;9162:1;9159;9152:12;9102:64;9236:2;9221:18;;9215:25;9185:7;;-1:-1:-1;;;;;;9252:30:238;;9249:50;;;9295:1;9292;9285:12;9249:50;9318:61;9371:7;9362:6;9351:9;9347:22;9318:61;:::i;:::-;9308:71;;;8759:626;;;;;:::o;11700:127::-;11761:10;11756:3;11752:20;11749:1;11742:31;11792:4;11789:1;11782:15;11816:4;11813:1;11806:15;11832:125;11897:9;;;11918:10;;;11915:36;;;11931:18;;:::i;11962:771::-;12011:3;12052:5;12046:12;12081:36;12107:9;12081:36;:::i;:::-;12126:19;;;12164:4;12187:1;12204:18;;;12231:146;;;;12391:1;12386:341;;;;12197:530;;12231:146;-1:-1:-1;;12273:24:238;;12259:12;;;12252:46;12345:14;;12338:22;12335:1;12331:30;12322:40;;12318:49;;;-1:-1:-1;12231:146:238;;12386:341;12417:5;12414:1;12407:16;12464:2;12461:1;12451:16;12489:1;12503:174;12517:6;12514:1;12511:13;12503:174;;;12604:14;;12586:11;;;12582:20;;12575:44;12647:16;;;;12532:10;;12503:174;;;12701:11;;12697:20;;;-1:-1:-1;;12197:530:238;;;;;;11962:771;;;;:::o;12738:458::-;12953:6;12942:9;12935:25;12996:2;12991;12980:9;12976:18;12969:30;12916:4;13022:52;13070:2;13059:9;13055:18;13047:6;13022:52;:::i;:::-;13122:9;13114:6;13110:22;13105:2;13094:9;13090:18;13083:50;13150:40;13183:6;13175;13150:40;:::i;13201:128::-;13268:9;;;13289:11;;;13286:37;;;13303:18;;:::i;13334:127::-;13395:10;13390:3;13386:20;13383:1;13376:31;13426:4;13423:1;13416:15;13450:4;13447:1;13440:15;13466:136;13505:3;13533:5;13523:39;;13542:18;;:::i;:::-;-1:-1:-1;;;13578:18:238;;13466:136::o;13607:222::-;13751:2;13740:9;13733:21;13714:4;13771:52;13819:2;13808:9;13804:18;13796:6;13771:52;:::i;14105:287::-;14234:3;14272:6;14266:13;14288:66;14347:6;14342:3;14335:4;14327:6;14323:17;14288:66;:::i;:::-;14370:16;;;;;14105:287;-1:-1:-1;;14105:287:238:o;14755:396::-;14904:2;14893:9;14886:21;14867:4;14936:6;14930:13;14979:6;14974:2;14963:9;14959:18;14952:34;14995:79;15067:6;15062:2;15051:9;15047:18;15042:2;15034:6;15030:15;14995:79;:::i;:::-;15135:2;15114:15;-1:-1:-1;;15110:29:238;15095:45;;;;15142:2;15091:54;;14755:396;-1:-1:-1;;14755:396:238:o;:::-;134:5676:99;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;\",\n    \"linkReferences\": {}\n  },\n  \"deployedBytecode\": {\n    \"object\": \"0x608060405234801561001057600080fd5b50600436106104b65760003560e01c80638da5cb5b11610278578063bf86d6901161015c578063d446bbcc116100ce578063e1b1acb711610092578063e1b1acb714610b56578063e753e60014610b69578063eef33eca14610bde578063ef8b30f714610a3c578063f2fde38b14610bed578063f7b24e0814610c0057600080fd5b8063d446bbcc14610ac7578063d505accf14610af2578063d7d4bf4514610b05578063d905777e14610b18578063dd62ed3e14610b2b57600080fd5b8063c8e8195011610120578063c8e8195014610a4f578063cbdf33d014610a62578063cd82f8b114610a85578063ce96cb7714610a8d578063cf30901214610aa0578063d1e8840414610ab457600080fd5b8063bf86d690146109f9578063c244245a14610a0d578063c588d8d614610a16578063c63d75b614610a29578063c6e6f59214610a3c57600080fd5b8063a373e3ff116101f5578063b0a75d36116101b9578063b0a75d3614610992578063b187bd26146109a5578063b3d7f6b9146109ad578063b460af94146109c0578063b5292a99146109d3578063ba087652146109e657600080fd5b8063a373e3ff14610949578063a8144e4814610951578063a9059cbb14610959578063ac9650d81461096c578063b0646e271461097f57600080fd5b80639959af941161023c5780639959af94146108ef5780639c552ca8146109035780639c5f00c2146109165780639fdb11b61461092d578063a07bee0b1461093657600080fd5b80638da5cb5b1461088b57806393bbeac01461089e57806394bf804d146108c157806395d89b41146108d45780639955a9d4146108dc57600080fd5b8063402d267d1161039f5780635f6b88a01161031c57806371e99dc2116102e057806371e99dc2146107ef5780637384504f146107f757806378e0233e1461081a5780637b103999146108315780637ecebe001461085857806388c4caba1461087857600080fd5b80635f6b88a01461077a578063687c2b501461078d5780636e553f65146107ad5780636ff1c02a146107c057806370a08231146107cf57600080fd5b8063530a371411610363578063530a37141461071c578063575bbce61461072f57806359d20b4e146107425780635a400d251461076a5780635e2c576e1461077257600080fd5b8063402d267d146106cf5780634c4602da146106e25780634cdad506146105135780634e84befe146106f6578063501eb4fe1461070957600080fd5b806318160ddd116104385780633644e515116103fc5780633644e51514610626578063379e0b131461062e57806338d52e0f146106415780633998a681146106805780633d8ab1e5146106a75780633e3382ba146106ba57600080fd5b806318160ddd1461059b57806318d4c143146105a457806323b872dd146105c7578063313ce567146105da57806333e15be21461061357600080fd5b806307a2d13a1161047f57806307a2d13a14610513578063095ea7b3146105265780630a28a477146105495780630a680e181461055c578063150b7a021461056457600080fd5b806251a3b7146104bb57806301e1d114146104d75780630402ab63146104df57806306fdde03146104e95780630780fd3a146104fe575b600080fd5b6104c461012c81565b6040519081526020015b60405180910390f35b6104c4610c08565b6104c46202a30081565b6104f1610c54565b6040516104ce9190614c54565b61051161050c366004614c80565b610ce2565b005b6104c4610521366004614c9b565b610e4d565b610539610534366004614cc9565b610e73565b60405190151581526020016104ce565b6104c4610557366004614c9b565b610ee0565b610511610efe565b610582610572366004614de0565b630a85bd0160e11b949350505050565b6040516001600160e01b031990911681526020016104ce565b6104c460025481565b6105396105b2366004614e4b565b600e6020526000908152604090205460ff1681565b6105396105d5366004614e68565b610f7a565b6106017f000000000000000000000000000000000000000000000000000000000000000081565b60405160ff90911681526020016104ce565b610511610621366004614eb7565b610f90565b6104c4611093565b61051161063c366004614eee565b6110e9565b6106687f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016104ce565b61068f6702c68af0bb14000081565b6040516001600160401b0390911681526020016104ce565b6105116106b5366004614e4b565b6113c1565b6106c26114c6565b6040516104ce9190614f35565b6104c46106dd366004614e4b565b61154a565b60065461053990600160b81b900460ff1681565b610511610704366004614fca565b6115ec565b610511610717366004614c80565b611791565b61051161072a366004614c9b565b611890565b61051161073d36600461500b565b61193a565b610755610750366004614c9b565b6119b6565b60405163ffffffff90911681526020016104ce565b6104c4600281565b6105116119f0565b610511610788366004614e4b565b611a82565b6104c461079b366004614e4b565b60146020526000908152604090205481565b6104c46107bb366004615034565b611b03565b61068f67016345785d8a000081565b6104c46107dd366004614e4b565b60036020526000908152604090205481565b6106c2611bd7565b61080a610805366004614c80565b611c34565b6040516104ce9493929190615059565b610822611d7d565b6040516104ce93929190615116565b6106687f000000000000000000000000000000000000000000000000000000000000000081565b6104c4610866366004614e4b565b60056020526000908152604090205481565b601154610668906001600160a01b031681565b600654610668906001600160a01b031681565b6105396108ac366004614c9b565b600b6020526000908152604090205460ff1681565b6104c46108cf366004615034565b61206e565b6104f161212d565b6105116108ea366004615181565b61213a565b60065461053990600160b01b900460ff1681565b610511610911366004614c9b565b61242b565b60065461075590600160c01b900463ffffffff1681565b6104c460135481565b610511610944366004614eee565b6124c3565b610511612647565b6104c46126aa565b610539610967366004614cc9565b6126e8565b61051161097a366004614fca565b612704565b61051161098d36600461500b565b61278b565b6105116109a0366004614e4b565b6127e5565b610539612878565b6104c46109bb366004614c9b565b61291a565b6104c46109ce3660046151f1565b612938565b6105116109e1366004615228565b6129b7565b6104c46109f43660046151f1565b612a7d565b60065461053990600160a81b900460ff1681565b6104c460125481565b610511610a24366004615251565b612b07565b6104c4610a37366004614e4b565b612bf4565b6104c4610a4a366004614c9b565b612c6c565b610511610a5d366004615034565b612c8a565b610539610a70366004614c80565b600d6020526000908152604090205460ff1681565b6104c4600081565b6104c4610a9b366004614e4b565b612d0c565b60065461053990600160a01b900460ff1681565b610511610ac2366004614c80565b612d44565b600754610ada906001600160c01b031681565b6040516001600160c01b0390911681526020016104ce565b610511610b00366004615297565b612dc2565b600854610668906001600160a01b031681565b6104c4610b26366004614e4b565b613006565b6104c4610b39366004615308565b600460209081526000928352604080842090915290825290205481565b610755610b64366004614c9b565b61303e565b600f54601054610ba5916001600160401b0380821692680100000000000000008304821692600160801b9004909116906001600160a01b031684565b604080516001600160401b039586168152938516602085015291909316908201526001600160a01b0390911660608201526080016104ce565b61068f670de0b6b3a764000081565b610511610bfb366004614e4b565b61304e565b6104c4602081565b6000610c126130c4565b600654600160a01b900460ff1615610c455760405162461bcd60e51b8152600401610c3c90615336565b60405180910390fd5b610c4f600061317d565b905090565b60008054610c619061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054610c8d9061535a565b8015610cda5780601f10610caf57610100808354040283529160200191610cda565b820191906000526020600020905b815481529060010190602001808311610cbd57829003601f168201915b505050505081565b6006546001600160a01b03163314610d0c5760405162461bcd60e51b8152600401610c3c90615394565b63ffffffff81166000908152600b602052604090205460ff16610d4a576040516370abe85960e01b815263ffffffff82166004820152602401610c3c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610d7d8261363f565b6001600160a01b031614610de1577f0000000000000000000000000000000000000000000000000000000000000000610db58261363f565b60405163298473c760e11b81526001600160a01b03928316600482015291166024820152604401610c3c565b63ffffffff81166000908152600c6020526040902054600160a01b900460ff1615610e2757604051630a42c0f960e41b815263ffffffff82166004820152602401610c3c565b6006805463ffffffff909216600160c01b0263ffffffff60c01b19909216919091179055565b6000806000610e5c60006136c8565b91509150610e6b8483836136e1565b949350505050565b3360008181526004602090815260408083206001600160a01b038716808552925280832085905551919290917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92590610ece9086815260200190565b60405180910390a35060015b92915050565b6000806000610eef60006136c8565b91509150610e6b8483836136ee565b6006546001600160a01b03163314610f285760405162461bcd60e51b8152600401610c3c90615394565b610f306136fb565b6006805460ff60a81b1916600160a81b179055604051600181527fb8527b93c36dabdfe078af41be789ba946a4adcfeafcf9d8de21d51629859e3c906020015b60405180910390a1565b6000610f8584613726565b610e6b848484613785565b6006546001600160a01b03163314610fba5760405162461bcd60e51b8152600401610c3c90615394565b6000816110055760098363ffffffff1681548110610fda57610fda6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16611045565b600a8363ffffffff168154811061101e5761101e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff165b9050600061105282613865565b9050801561108257604051631c7b946d60e31b815263ffffffff8316600482015260248101829052604401610c3c565b61108d8483856138ee565b50505050565b60007f000000000000000000000000000000000000000000000000000000000000000046146110c457610c4f6139da565b507f000000000000000000000000000000000000000000000000000000000000000090565b6006546001600160a01b031633146111135760405162461bcd60e51b8152600401610c3c90615394565b600080821561124557600a8463ffffffff1681548110611135576111356153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff169150600a8563ffffffff1681548110611176576111766153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff1690508181600a8763ffffffff16815481106111b9576111b96153ba565b9060005260206000209060089182820401919006600402600a8863ffffffff16815481106111e9576111e96153ba565b90600052602060002090600891828204019190066004028491906101000a81548163ffffffff021916908363ffffffff1602179055508391906101000a81548163ffffffff021916908363ffffffff160217905550505061136a565b60098463ffffffff168154811061125e5761125e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16915060098563ffffffff168154811061129f5761129f6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff169050818160098763ffffffff16815481106112e2576112e26153ba565b906000526020600020906008918282040191900660040260098863ffffffff1681548110611312576113126153ba565b90600052602060002090600891828204019190066004028491906101000a81548163ffffffff021916908363ffffffff1602179055508391906101000a81548163ffffffff021916908363ffffffff16021790555050505b6040805163ffffffff84811682528381166020830152878116828401528616606082015290517fb7c5df04749a3a06a9a7bf1a8142ccf2a4ee6cbf4709489e876a6e4eb3301e8a9181900360800190a15050505050565b6006546001600160a01b031633146113eb5760405162461bcd60e51b8152600401610c3c90615394565b604051636777140560e11b81526001600160a01b0382811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063ceee280a9060240160006040518083038186803b15801561144c57600080fd5b505afa158015611460573d6000803e3d6000fd5b505050506001600160a01b0381166000818152600e6020908152604091829020805460ff191660019081179091558251938452908301527f572570e8a43782d3698a3fed258c72f9c201c19be1e4764e359d1adc8f00af7a91015b60405180910390a150565b6060600a80548060200260200160405190810160405280929190818152602001828054801561154057602002820191906000526020600020906000905b82829054906101000a900463ffffffff1663ffffffff16815260200190600401906020826003010492830192600103820291508084116115035790505b5050505050905090565b600654600090600160a81b900460ff161561156757506000919050565b6007546001600160c01b03166002600160c01b0319810161158c575060001992915050565b60008061159960016136c8565b91509150826001600160c01b031681106115b857506000949350505050565b60006115cd826001600160c01b0386166153e6565b90506115da8184846136e1565b9695505050505050565b505050919050565b600654600160a01b900460ff16156116165760405162461bcd60e51b8152600401610c3c90615336565b60068054600160a01b60ff60a01b198216179091556001600160a01b0316331480159061164e57506011546001600160a01b03163314155b1561166c57604051633fd2923560e01b815260040160405180910390fd5b6116746136fb565b61167c6130c4565b6006805460ff60b81b1916600160b81b179055600080808061169d8161317d565b90506116c8601254670de0b6b3a76400006116b891906153e6565b8290670de0b6b3a7640000613a74565b93506116e3601254670de0b6b3a76400006116b891906153f9565b60025490935091506116ff90506116fa858761542f565b613aa2565b600061170b600061317d565b90508381108061171a57508281115b156117495760405163628cc47560e11b8152600481018290526024810185905260448101849052606401610c3c565b600254821461177957600254604051632b40145960e21b8152600481019190915260248101839052604401610c3c565b50506006805463ff0000ff60a01b1916905550505050565b6006546001600160a01b031633146117bb5760405162461bcd60e51b8152600401610c3c90615394565b604051635159d87f60e11b815263ffffffff821660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063a2b3b0fe9060240160006040518083038186803b15801561182057600080fd5b505afa158015611834573d6000803e3d6000fd5b5050505063ffffffff81166000818152600d6020908152604091829020805460ff191660019081179091558251938452908301527fea052d1fb1ecba6aaf6bd32e92f20e7b6a094eaa478248322cc8ff024a90978f91016114bb565b6006546001600160a01b031633146118ba5760405162461bcd60e51b8152600401610c3c90615394565b67016345785d8a00008111156118f4576040516302d2a90f60e51b81526004810182905267016345785d8a00006024820152604401610c3c565b601280549082905560408051828152602081018490527fdf4be33b2e9e3dd4d9e0e85645aea428494a0644a72c51d6a15aedae6b66a3ff91015b60405180910390a15050565b6006546001600160a01b031633146119645760405162461bcd60e51b8152600401610c3c90615394565b6007546001600160c01b039081169082161115611994576040516334f1ec1b60e01b815260040160405180910390fd5b600780546001600160c01b0319166001600160c01b0392909216919091179055565b600981815481106119c657600080fd5b9060005260206000209060089182820401919006600402915054906101000a900463ffffffff1681565b6006546001600160a01b03163314611a1a5760405162461bcd60e51b8152600401610c3c90615394565b600654600160a81b900460ff16611a445760405163ec7165bf60e01b815260040160405180910390fd5b6006805460ff60a81b19169055604051600081527fb8527b93c36dabdfe078af41be789ba946a4adcfeafcf9d8de21d51629859e3c90602001610f70565b6006546001600160a01b03163314611aac5760405162461bcd60e51b8152600401610c3c90615394565b6001600160a01b0381166000818152600e60209081526040808320805460ff191690558051938452908301919091527f572570e8a43782d3698a3fed258c72f9c201c19be1e4764e359d1adc8f00af7a91016114bb565b600654600090600160a01b900460ff1615611b305760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080611b5060016136c8565b91509150611b5f858383613c27565b925082600003611b825760405163426f153760e11b815260040160405180910390fd5b6007546001600160c01b0316611b9884836153f9565b1115611bb75760405163adea3dfd60e01b815260040160405180910390fd5b611bc2858486613c34565b50506006805460ff60a01b1916905592915050565b60606009805480602002602001604051908101604052809291908181526020018280548015611540576000918252602091829020805463ffffffff1684529082028301929091600491018084116115035790505050505050905090565b600c60205260009081526040902080546001820180546001600160a01b03831693600160a01b90930460ff16929190611c6c9061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054611c989061535a565b8015611ce55780601f10611cba57610100808354040283529160200191611ce5565b820191906000526020600020905b815481529060010190602001808311611cc857829003601f168201915b505050505090806002018054611cfa9061535a565b80601f0160208091040260200160405190810160405280929190818152602001828054611d269061535a565b8015611d735780601f10611d4857610100808354040283529160200191611d73565b820191906000526020600020905b815481529060010190602001808311611d5657829003601f168201915b5050505050905084565b600954600a546060918291829190611d9581836153f9565b6001600160401b03811115611dac57611dac614cf5565b604051908082528060200260200182016040528015611dd5578160200160208202803683370190505b509450611de281836153f9565b6001600160401b03811115611df957611df9614cf5565b604051908082528060200260200182016040528015611e22578160200160208202803683370190505b509350611e2f81836153f9565b6001600160401b03811115611e4657611e46614cf5565b604051908082528060200260200182016040528015611e6f578160200160208202803683370190505b50925060005b82811015611f8357611ebe60098281548110611e9357611e936153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff1661363f565b868281518110611ed057611ed06153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050611f3160098281548110611f0657611f066153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16613865565b858281518110611f4357611f436153ba565b6020026020010181815250506000848281518110611f6357611f636153ba565b91151560209283029190910190910152611f7c81615555565b9050611e75565b5060005b8181101561206657611fa5600a8281548110611e9357611e936153ba565b6009548790611fb490846153f9565b81518110611fc457611fc46153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050611ffa600a8281548110611f0657611f066153ba565b600954869061200990846153f9565b81518110612019576120196153ba565b6020908102919091010152600954600190859061203690846153f9565b81518110612046576120466153ba565b9115156020928302919091019091015261205f81615555565b9050611f87565b505050909192565b600654600090600160a01b900460ff161561209b5760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b1790556000806120bb60016136c8565b915091506120ca858383613ccf565b9250826000036120ed57604051639768300560e01b815260040160405180910390fd5b6007546001600160c01b031661210386836153f9565b11156121225760405163adea3dfd60e01b815260040160405180910390fd5b611bc2838686613c34565b60018054610c619061535a565b6006546001600160a01b031633146121645760405162461bcd60e51b8152600401610c3c90615394565b61216c6136fb565b63ffffffff83166000908152600b602052604090205460ff16156121ab5760405163335894fb60e11b815263ffffffff84166004820152602401610c3c565b63ffffffff83166000908152600d602052604090205460ff166121e957604051631f9db01d60e31b815263ffffffff84166004820152602401610c3c565b6040516385ae5d5760e01b815263ffffffff84166004820152600090819081906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906385ae5d5790602401600060405180830381865afa15801561225a573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612282919081019061556e565b925092509250831515821515146122b457604051632b1d0bd360e11b815263ffffffff87166004820152602401610c3c565b604080516080810182526001600160a01b0380861682528415156020808401918252838501868152606085018b905263ffffffff8c166000908152600c9092529490208351815492511515600160a01b026001600160a81b03199093169316929092171781559151909190600182019061232e9082615653565b50606082015160028201906123439082615653565b50905050811561238457600a546020116123735760405163f025236d60e01b815260206004820152602401610c3c565b61237f600a8888613cdc565b6123b6565b6009546020116123aa5760405163f025236d60e01b815260206004820152602401610c3c565b6123b660098888613cdc565b63ffffffff86166000908152600b602052604090819020805460ff19166001179055517fc4f8cb57c016f0b294fff2666f86fa6cfee9b03aed19f816ae4bf44b7e837bbb9061241a9088908a9063ffffffff92831681529116602082015260400190565b60405180910390a150505050505050565b6006546001600160a01b031633146124555760405162461bcd60e51b8152600401610c3c90615394565b61012c81108061246757506202a30081115b1561248557604051633a60233f60e21b815260040160405180910390fd5b601380549082905560408051828152602081018490527f227ff5c6b5ffb395236b09fd1b472bb128b36eaa17556633feefe28e94411f24910161192e565b6006546001600160a01b031633146124ed5760405162461bcd60e51b8152600401610c3c90615394565b6000816125385760098463ffffffff168154811061250d5761250d6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16612578565b600a8463ffffffff1681548110612551576125516153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff165b90508063ffffffff168363ffffffff1614158061261e57506040516321a0f75360e01b815263ffffffff841660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906321a0f75390602401602060405180830381865afa1580156125fa573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061261e9190615712565b1561263c5760405163d4db0b7960e01b815260040160405180910390fd5b61108d8484846138ee565b6006546001600160a01b031633146126715760405162461bcd60e51b8152600401610c3c90615394565b600654600160b01b900460ff1661268957600161268c565b60005b60068054911515600160b01b0260ff60b01b19909216919091179055565b60006126b46130c4565b600654600160a01b900460ff16156126de5760405162461bcd60e51b8152600401610c3c90615336565b610c4f600161317d565b60006126f333613726565b6126fd8383613ea1565b9392505050565b60005b8181101561278657612773838383818110612724576127246153ba565b9050602002810190612736919061572f565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152503093925050613f079050565b508061277e81615555565b915050612707565b505050565b6006546001600160a01b031633146127b55760405162461bcd60e51b8152600401610c3c90615394565b6007546001600160c01b039081169082161015611994576040516334f1ec1b60e01b815260040160405180910390fd5b6006546001600160a01b0316331461280f5760405162461bcd60e51b8152600401610c3c90615394565b601054604080516001600160a01b03928316815291831660208301527f51dbb5a65bb22737861a63ec12ba6ce78a98631e9404b0567a2eaf7a06fc544d910160405180910390a1601080546001600160a01b0319166001600160a01b0392909216919091179055565b600654600090600160b01b900460ff1661291457604051630ad85dff60e41b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ad85dff090602401602060405180830381865afa1580156128f0573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c4f9190615712565b50600090565b600080600061292960016136c8565b91509150610e6b848383613ccf565b600654600090600160a01b900460ff16156129655760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080612984816136c8565b915091506129938683836136ee565b92506129a186848787613f2c565b50506006805460ff60a01b191690559392505050565b6006546001600160a01b031633146129e15760405162461bcd60e51b8152600401610c3c90615394565b670de0b6b3a76400006001600160401b0382161115612a1357604051633d0203e560e01b815260040160405180910390fd5b600f54604080516001600160401b03928316815291831660208301527fb5cc994a260a85a42d6588668221571ae0a14f0a28f9e4817a5195262102c868910160405180910390a1600f805467ffffffffffffffff19166001600160401b0392909216919091179055565b600654600090600160a01b900460ff1615612aaa5760405162461bcd60e51b8152600401610c3c90615336565b6006805460ff60a01b1916600160a01b179055600080612ac9816136c8565b91509150612ad88683836136e1565b925082600003612afb57604051639768300560e01b815260040160405180910390fd5b6129a183878787613f2c565b6006546001600160a01b03163314612b315760405162461bcd60e51b8152600401610c3c90615394565b6000808415612b78576000612b44610c08565b9050612b63612b5586612710615775565b829061ffff16612710614005565b9250612b74612b5586612710615790565b9150505b612b83600284614024565b600880546001600160a01b0319166001600160a01b0385161790556000612ba8610c08565b90508515612bec5782811080612bbd57508181115b15612bec5760405163628cc47560e11b8152600481018290526024810184905260448101839052606401610c3c565b505050505050565b600654600090600160a81b900460ff1615612c1157506000919050565b6007546001600160c01b03166002600160c01b03198101612c36575060001992915050565b6002546001600160c01b038216811015612c6257612c5d816001600160c01b0384166153e6565b610e6b565b6000949350505050565b6000806000612c7b60016136c8565b91509150610e6b848383613c27565b6006546001600160a01b03163314612cb45760405162461bcd60e51b8152600401610c3c90615394565b612cbe8282614024565b601180546001600160a01b0319166001600160a01b0383169081179091556040519081527f3ced9f0d0ac37f3370e1e00515182a375773698b50f5a46523e2cb37360155839060200161192e565b600654600090600160a01b900460ff1615612d395760405162461bcd60e51b8152600401610c3c90615336565b610eda826000614105565b6006546001600160a01b03163314612d6e5760405162461bcd60e51b8152600401610c3c90615394565b63ffffffff81166000818152600d60209081526040808320805460ff191690558051938452908301919091527fea052d1fb1ecba6aaf6bd32e92f20e7b6a094eaa478248322cc8ff024a90978f91016114bb565b42841015612e125760405162461bcd60e51b815260206004820152601760248201527f5045524d49545f444541444c494e455f455850495245440000000000000000006044820152606401610c3c565b60006001612e1e611093565b6001600160a01b038a811660008181526005602090815260409182902080546001810190915582517f6e71edae12b1b97f4d1f60370fef10105fa2faae0126114a169c64845d6126c98184015280840194909452938d166060840152608083018c905260a083019390935260c08083018b90528151808403909101815260e08301909152805192019190912061190160f01b6101008301526101028201929092526101228101919091526101420160408051601f198184030181528282528051602091820120600084529083018083525260ff871690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015612f2a573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811615801590612f605750876001600160a01b0316816001600160a01b0316145b612f9d5760405162461bcd60e51b815260206004820152600e60248201526d24a72b20a624a22fa9a4a3a722a960911b6044820152606401610c3c565b6001600160a01b0390811660009081526004602090815260408083208a8516808552908352928190208990555188815291928a16917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925910160405180910390a350505050505050565b600654600090600160a01b900460ff16156130335760405162461bcd60e51b8152600401610c3c90615336565b610eda826001614105565b600a81815481106119c657600080fd5b6006546001600160a01b031633146130785760405162461bcd60e51b8152600401610c3c90615394565b600680546001600160a01b0319166001600160a01b03831690811790915560405133907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a350565b600654600160b01b900460ff1661317b57604051630ad85dff60e41b81523060048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063ad85dff090602401602060405180830381865afa158015613139573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061315d9190615712565b1561317b57604051630f301f8f60e41b815260040160405180910390fd5b565b60095460009081816001600160401b0381111561319c5761319c614cf5565b6040519080825280602002602001820160405280156131c5578160200160208202803683370190505b5090506000826001600160401b038111156131e2576131e2614cf5565b60405190808252806020026020018201604052801561320b578160200160208202803683370190505b50905084156133795760005b838110156132dc57600060098281548110613234576132346153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061326581614161565b838381518110613277576132776153ba565b602002602001018181525060000361328f57506132cc565b6132988161363f565b8483815181106132aa576132aa6153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b6132d581615555565b9050613217565b5060085460405163b333a17560e01b81526001600160a01b039091169063b333a1759061333190859085907f0000000000000000000000000000000000000000000000000000000000000000906004016157ab565b602060405180830381865afa15801561334e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061337291906157e9565b93506115e4565b600a546000816001600160401b0381111561339657613396614cf5565b6040519080825280602002602001820160405280156133bf578160200160208202803683370190505b5090506000826001600160401b038111156133dc576133dc614cf5565b604051908082528060200260200182016040528015613405578160200160208202803683370190505b50905060005b868110156134d057600060098281548110613428576134286153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061345981613865565b86838151811061346b5761346b6153ba565b602002602001018181525060000361348357506134c0565b61348c8161363f565b87838151811061349e5761349e6153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b6134c981615555565b905061340b565b5060005b83811015613599576000600a82815481106134f1576134f16153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16905061352281613865565b838381518110613534576135346153ba565b602002602001018181525060000361354c5750613589565b6135558161363f565b848381518110613567576135676153ba565b60200260200101906001600160a01b031690816001600160a01b031681525050505b61359281615555565b90506134d4565b50600854604051637563738b60e11b81526001600160a01b039091169063eac6e716906135f29088908890879087907f000000000000000000000000000000000000000000000000000000000000000090600401615802565b602060405180830381865afa15801561360f573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061363391906157e9565b98975050505050505050565b63ffffffff81166000908152600c60205260408082208054915163e170a9bf60e01b81526001600160a01b0390921691829163e170a9bf9161368791600101906004016158e7565b602060405180830381865afa1580156136a4573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126fd91906158fa565b6000806136d5600061317d565b91506002549050915091565b6000610e6b848484614005565b6000610e6b848385613a74565b600654600160a81b900460ff161561317b576040516337a5332d60e11b815260040160405180910390fd5b6001600160a01b03811660009081526014602052604090205480156137815760006013548261375591906153f9565b905042811115612786576040516306f8ee3f60e21b815260048101829052426024820152604401610c3c565b5050565b6001600160a01b038316600090815260046020908152604080832033845290915281205460001981146137e1576137bc83826153e6565b6001600160a01b03861660009081526004602090815260408083203384529091529020555b6001600160a01b038516600090815260036020526040812080548592906138099084906153e6565b90915550506001600160a01b0380851660008181526003602052604090819020805487019055519091871690600080516020615c00833981519152906138529087815260200190565b60405180910390a3506001949350505050565b63ffffffff81166000908152600c602052604080822080549151637841536560e01b81526001600160a01b039092169182916378415365916138ad91600101906004016158e7565b602060405180830381865afa1580156138ca573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906126fd91906157e9565b60065463ffffffff600160c01b909104811690831603613921576040516319ded73160e21b815260040160405180910390fd5b801561393757613932600a8461421b565b613942565b61394260098461421b565b63ffffffff82166000908152600b60209081526040808320805460ff19169055600c909152812080546001600160a81b0319168155906139856001830182614bae565b613993600283016000614bae565b50506040805163ffffffff8085168252851660208201527fa5cd0099b78b279c04987aa80ffffaf8fc8c8af4e7c7bce2686e8d01e2e1bd51910160405180910390a1505050565b60007f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6000604051613a0c9190615917565b6040805191829003822060208301939093528101919091527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b828202811515841585830485141716613a8c57600080fd5b6001826001830304018115150290509392505050565b60005b8151811015613781576000828281518110613ac257613ac26153ba565b602090810291909101810151516001600160a01b0381166000908152600e90925260409091205490915060ff16613b1757604051635df6b61760e11b81526001600160a01b0382166004820152602401610c3c565b60005b838381518110613b2c57613b2c6153ba565b60200260200101516020015151811015613c1457613b92848481518110613b5557613b556153ba565b6020026020010151602001518281518110613b7257613b726153ba565b6020026020010151836001600160a01b0316613f0790919063ffffffff16565b507f7445c6598e1b553f076d507692eab3dceef0d608757141b53e9e56aa8bbaf48382858581518110613bc757613bc76153ba565b6020026020010151602001518381518110613be457613be46153ba565b6020026020010151604051613bfa92919061598d565b60405180910390a180613c0c81615555565b915050613b1a565b505080613c2090615555565b9050613aa5565b6000610e6b848385614005565b613c3f838383614360565b613c746001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016333086614422565b613c7e81836144ac565b60408051848152602081018490526001600160a01b0383169133917fdcbc1c05240f31ff3ad067ef1ee35ce4997762752e3a095284754544f4c709d7910160405180910390a3612786838383614506565b6000610e6b848484613a74565b82548015613e60578380613cf16001846153e6565b81548110613d0157613d016153ba565b60009182526020808320600880840490910154855460018082018855968652928520918304909101805463ffffffff60046007958616810261010090810a83810219909416969097160290950a9092049093160217905590613d6390836153e6565b90505b8363ffffffff16811115613e0e5784613d806001836153e6565b81548110613d9057613d906153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff16858281548110613dc857613dc86153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff1602179055508080613e06906159b1565b915050613d66565b5081848463ffffffff1681548110613e2857613e286153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff16021790555061108d565b508254600181018455600093845260209093206008840401805460079094166004026101000a63ffffffff8181021990951692909416939093021790915550565b33600090815260036020526040812080548391908390613ec29084906153e6565b90915550506001600160a01b03831660008181526003602052604090819020805485019055513390600080516020615c0083398151915290610ece9086815260200190565b60606126fd8383604051806060016040528060278152602001615bd96027913961452c565b613f388484848461459a565b336001600160a01b03821614613fa6576001600160a01b03811660009081526004602090815260408083203384529091529020546000198114613fa457613f7f84826153e6565b6001600160a01b03831660009081526004602090815260408083203384529091529020555b505b613fb081846145af565b60408051858152602081018590526001600160a01b03808416929085169133917ffbde797d201c681b91056529119e0b02407c7bb96a4a2c75c01fc9667232c8db910160405180910390a461108d8483614611565b82820281151584158583048514171661401d57600080fd5b0492915050565b8160000361404557604051632db38d0560e01b815260040160405180910390fd5b806001600160a01b03167f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b93f9b0a846040518263ffffffff1660e01b815260040161409d91815260200190565b602060405180830381865afa1580156140ba573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906140de91906158fa565b6001600160a01b03161461378157604051634ee204d760e01b815260040160405180910390fd5b6000614111838361497b565b6001600160a01b038416600090815260146020526040902054909150801561415a5760006013548261414391906153f9565b90504281111561415857600092505050610eda565b505b5092915050565b63ffffffff81166000908152600c6020526040812054600160a01b900460ff161561418e57506000919050565b63ffffffff82166000908152600c60205260409081902080549151637d2872e960e11b81526001600160a01b039092169163fa50e5d2916141da916001820191600201906004016159c8565b602060405180830381865afa1580156141f7573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610eda91906157e9565b815463ffffffff821681116142685760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610c3c565b63ffffffff82165b61427b6001836153e6565b81101561431c578361428e8260016153f9565b8154811061429e5761429e6153ba565b90600052602060002090600891828204019190066004029054906101000a900463ffffffff168482815481106142d6576142d66153ba565b90600052602060002090600891828204019190066004026101000a81548163ffffffff021916908363ffffffff160217905550808061431490615555565b915050614270565b508280548061432d5761432d6159f6565b600082815260209020600860001990920191820401805463ffffffff600460078516026101000a02191690559055505050565b61436b8383836149fd565b336001600160a01b0382161461278657604051635551e1b560e01b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635551e1b590602401602060405180830381865afa1580156143df573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906144039190615712565b612786576040516334871f2560e21b8152336004820152602401610c3c565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806144a55760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610c3c565b5050505050565b80600260008282546144be91906153f9565b90915550506001600160a01b038216600081815260036020908152604080832080548601905551848152600080516020615c0083398151915291015b60405180910390a35050565b6001600160a01b0381166000908152601460205260409020429055612786838383614a0d565b6060600080856001600160a01b0316856040516145499190615a0c565b600060405180830381855af49150503d8060008114614584576040519150601f19603f3d011682016040523d82523d6000602084013e614589565b606091505b50915091506115da86838387614a27565b6145a381613726565b61108d84848484614aa0565b6001600160a01b038216600090815260036020526040812080548392906145d79084906153e6565b90915550506002805482900390556040518181526000906001600160a01b03841690600080516020615c00833981519152906020016144fa565b61463c6040518060800160405280600081526020016000815260200160008152602001600081525090565b600854604051630226614760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116600483015290911690630226614790602401602060405180830381865afa1580156146a6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906146ca91906157e9565b60408201526146fa7f0000000000000000000000000000000000000000000000000000000000000000600a615b0c565b606082015260095460005b8181101561495857600060098281548110614722576147226153ba565b60009182526020822060088204015460079091166004026101000a900463ffffffff16915061475082614161565b905080600003614761575050614948565b600061476c8361363f565b600854604051630226614760e01b81526001600160a01b038084166004830152929350911690630226614790602401602060405180830381865afa1580156147b8573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906147dc91906157e9565b866000018181525050806001600160a01b031663313ce5676040518163ffffffff1660e01b8152600401602060405180830381865afa158015614823573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906148479190615b1b565b61485290600a615b0c565b602087018190528651600091829161487d9161487687670de0b6b3a7640000615b38565b9190614005565b905061489c88606001518960400151836140059092919063ffffffff16565b91506148b0670de0b6b3a764000083615b4f565b9150506000898211156149165760006148e089604001518a606001518d670de0b6b3a76400006148769190615b38565b60208a01518a519192506148f691839190614005565b915061490a670de0b6b3a764000083615b4f565b915060009a5050614925565b5082614922828b6153e6565b99505b61493085828b614aa8565b89600003614942575050505050614958565b50505050505b61495181615555565b9050614705565b50831561108d5760405163cc5ea39b60e01b815260048101859052602401610c3c565b60006149856130c4565b60008061499260006136c8565b6001600160a01b038716600090815260036020526040812054929450909250906149bd9084846136e1565b905060006149cb600161317d565b9050808211156149db57806149dd565b815b945085156149f3576149f0858585613c27565b94505b5050505092915050565b614a056136fb565b6127866130c4565b60065461278690600160c01b900463ffffffff1684614b39565b60608315614a96578251600003614a8f576001600160a01b0385163b614a8f5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610c3c565b5081610e6b565b610e6b8383614b84565b61108d6130c4565b63ffffffff83166000908152600c602052604090819020805491516001600160a01b03909216916144a59163c9111bd760e01b91614af791879187916001810191600290910190602401615b71565b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b0319909316929092179091526001600160a01b03831690613f07565b63ffffffff82166000908152600c602052604090819020805491516001600160a01b039092169161108d916369445c3160e01b91614af7918691600182019160020190602401615bad565b815115614b945781518083602001fd5b8060405162461bcd60e51b8152600401610c3c9190614c54565b508054614bba9061535a565b6000825580601f10614bca575050565b601f016020900490600052602060002090810190614be89190614beb565b50565b5b80821115614c005760008155600101614bec565b5090565b60005b83811015614c1f578181015183820152602001614c07565b50506000910152565b60008151808452614c40816020860160208601614c04565b601f01601f19169290920160200192915050565b6020815260006126fd6020830184614c28565b803563ffffffff81168114614c7b57600080fd5b919050565b600060208284031215614c9257600080fd5b6126fd82614c67565b600060208284031215614cad57600080fd5b5035919050565b6001600160a01b0381168114614be857600080fd5b60008060408385031215614cdc57600080fd5b8235614ce781614cb4565b946020939093013593505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715614d2d57614d2d614cf5565b60405290565b604051601f8201601f191681016001600160401b0381118282101715614d5b57614d5b614cf5565b604052919050565b60006001600160401b03821115614d7c57614d7c614cf5565b50601f01601f191660200190565b600082601f830112614d9b57600080fd5b8135614dae614da982614d63565b614d33565b818152846020838601011115614dc357600080fd5b816020850160208301376000918101602001919091529392505050565b60008060008060808587031215614df657600080fd5b8435614e0181614cb4565b93506020850135614e1181614cb4565b92506040850135915060608501356001600160401b03811115614e3357600080fd5b614e3f87828801614d8a565b91505092959194509250565b600060208284031215614e5d57600080fd5b81356126fd81614cb4565b600080600060608486031215614e7d57600080fd5b8335614e8881614cb4565b92506020840135614e9881614cb4565b929592945050506040919091013590565b8015158114614be857600080fd5b60008060408385031215614eca57600080fd5b614ed383614c67565b91506020830135614ee381614ea9565b809150509250929050565b600080600060608486031215614f0357600080fd5b614f0c84614c67565b9250614f1a60208501614c67565b91506040840135614f2a81614ea9565b809150509250925092565b6020808252825182820181905260009190848201906040850190845b81811015614f7357835163ffffffff1683529284019291840191600101614f51565b50909695505050505050565b60008083601f840112614f9157600080fd5b5081356001600160401b03811115614fa857600080fd5b6020830191508360208260051b8501011115614fc357600080fd5b9250929050565b60008060208385031215614fdd57600080fd5b82356001600160401b03811115614ff357600080fd5b614fff85828601614f7f565b90969095509350505050565b60006020828403121561501d57600080fd5b81356001600160c01b03811681146126fd57600080fd5b6000806040838503121561504757600080fd5b823591506020830135614ee381614cb4565b6001600160a01b0385168152831515602082015260806040820181905260009061508590830185614c28565b82810360608401526150978185614c28565b979650505050505050565b600081518084526020808501945080840160005b838110156150db5781516001600160a01b0316875295820195908201906001016150b6565b509495945050505050565b600081518084526020808501945080840160005b838110156150db578151875295820195908201906001016150fa565b60608152600061512960608301866150a2565b60208382038185015261513c82876150e6565b8481036040860152855180825282870193509082019060005b81811015615173578451151583529383019391830191600101615155565b509098975050505050505050565b6000806000806080858703121561519757600080fd5b6151a085614c67565b93506151ae60208601614c67565b925060408501356001600160401b038111156151c957600080fd5b6151d587828801614d8a565b92505060608501356151e681614ea9565b939692955090935050565b60008060006060848603121561520657600080fd5b83359250602084013561521881614cb4565b91506040840135614f2a81614cb4565b60006020828403121561523a57600080fd5b81356001600160401b03811681146126fd57600080fd5b60008060006060848603121561526657600080fd5b833561527181614ea9565b9250602084013561ffff8116811461521857600080fd5b60ff81168114614be857600080fd5b600080600080600080600060e0888a0312156152b257600080fd5b87356152bd81614cb4565b965060208801356152cd81614cb4565b9550604088013594506060880135935060808801356152eb81615288565b9699959850939692959460a0840135945060c09093013592915050565b6000806040838503121561531b57600080fd5b823561532681614cb4565b91506020830135614ee381614cb4565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b600181811c9082168061536e57607f821691505b60208210810361538e57634e487b7160e01b600052602260045260246000fd5b50919050565b6020808252600c908201526b15539055551213d49256915160a21b604082015260600190565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b81810381811115610eda57610eda6153d0565b80820180821115610eda57610eda6153d0565b60006001600160401b0382111561542557615425614cf5565b5060051b60200190565b600061543d614da98461540c565b83815260208082019190600586811b86013681111561545b57600080fd5b865b818110156155485780356001600160401b038082111561547d5760008081fd5b818a019150604082360312156154935760008081fd5b61549b614d0b565b82356154a681614cb4565b815282870135828111156154ba5760008081fd5b929092019136601f8401126154cf5760008081fd5b82356154dd614da98261540c565b81815290871b840188019088810190368311156154fa5760008081fd5b8986015b83811015615532578035868111156155165760008081fd5b615524368d838b0101614d8a565b845250918a01918a016154fe565b50838a015250508852505094830194830161545d565b5092979650505050505050565b600060018201615567576155676153d0565b5060010190565b60008060006060848603121561558357600080fd5b835161558e81614cb4565b602085015190935061559f81614ea9565b60408501519092506001600160401b038111156155bb57600080fd5b8401601f810186136155cc57600080fd5b80516155da614da982614d63565b8181528760208385010111156155ef57600080fd5b615600826020830160208601614c04565b8093505050509250925092565b601f82111561278657600081815260208120601f850160051c810160208610156156345750805b601f850160051c820191505b81811015612bec57828155600101615640565b81516001600160401b0381111561566c5761566c614cf5565b6156808161567a845461535a565b8461560d565b602080601f8311600181146156b5576000841561569d5750858301515b600019600386901b1c1916600185901b178555612bec565b600085815260208120601f198616915b828110156156e4578886015182559484019460019091019084016156c5565b50858210156157025787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b60006020828403121561572457600080fd5b81516126fd81614ea9565b6000808335601e1984360301811261574657600080fd5b8301803591506001600160401b0382111561576057600080fd5b602001915036819003821315614fc357600080fd5b61ffff82811682821603908082111561415a5761415a6153d0565b61ffff81811683821601908082111561415a5761415a6153d0565b6060815260006157be60608301866150a2565b82810360208401526157d081866150e6565b91505060018060a01b0383166040830152949350505050565b6000602082840312156157fb57600080fd5b5051919050565b60a08152600061581560a08301886150a2565b828103602084015261582781886150e6565b9050828103604084015261583b81876150a2565b9050828103606084015261584f81866150e6565b91505060018060a01b03831660808301529695505050505050565b600081546158778161535a565b80855260206001838116801561589457600181146158ae576158dc565b60ff1985168884015283151560051b8801830195506158dc565b866000528260002060005b858110156158d45781548a82018601529083019084016158b9565b890184019650505b505050505092915050565b6020815260006126fd602083018461586a565b60006020828403121561590c57600080fd5b81516126fd81614cb4565b60008083546159258161535a565b6001828116801561593d576001811461595257615981565b60ff1984168752821515830287019450615981565b8760005260208060002060005b858110156159785781548a82015290840190820161595f565b50505082870194505b50929695505050505050565b6001600160a01b0383168152604060208201819052600090610e6b90830184614c28565b6000816159c0576159c06153d0565b506000190190565b6040815260006159db604083018561586a565b82810360208401526159ed818561586a565b95945050505050565b634e487b7160e01b600052603160045260246000fd5b60008251615a1e818460208701614c04565b9190910192915050565b600181815b80851115615a63578160001904821115615a4957615a496153d0565b80851615615a5657918102915b93841c9390800290615a2d565b509250929050565b600082615a7a57506001610eda565b81615a8757506000610eda565b8160018114615a9d5760028114615aa757615ac3565b6001915050610eda565b60ff841115615ab857615ab86153d0565b50506001821b610eda565b5060208310610133831016604e8410600b8410161715615ae6575081810a610eda565b615af08383615a28565b8060001904821115615b0457615b046153d0565b029392505050565b60006126fd60ff841683615a6b565b600060208284031215615b2d57600080fd5b81516126fd81615288565b8082028115828204841417610eda57610eda6153d0565b600082615b6c57634e487b7160e01b600052601260045260246000fd5b500490565b8481526001600160a01b0384166020820152608060408201819052600090615b9b9083018561586a565b8281036060840152615097818561586a565b838152606060208201526000615bc6606083018561586a565b82810360408401526115da818561586a56fe416464726573733a206c6f772d6c6576656c2064656c65676174652063616c6c206661696c6564ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa264697066735822122033bcfe5c8070dd09d9233cec0bd62bca9ecdff131eeb7dbe2a4960a32bfce8f064736f6c63430008150033\",\n    \"sourceMap\": \"134:5676:99:-:0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;1781:58;;1833:6;1781:58;;;;;160:25:238;;;148:2;133:18;1781:58:99;;;;;;;;39746:214:92;;;:::i;1932:58:99:-;;1984:6;1932:58;;1031:18:60;;;:::i;:::-;;;;;;;:::i;9741:413:92:-;;;;;;:::i;:::-;;:::i;:::-;;40709:258;;;;;;:::i;:::-;;:::i;2461:211:60:-;;;;;;:::i;:::-;;:::i;:::-;;;2115:14:238;;2108:22;2090:41;;2078:2;2063:18;2461:211:60;1950:187:238;42249:258:92;;;;;;:::i;:::-;;:::i;22143:146::-;;;:::i;588:200:49:-;;;;;;:::i;:::-;-1:-1:-1;;;588:200:49;;;;;;;;;;-1:-1:-1;;;;;;4306:33:238;;;4288:52;;4276:2;4261:18;588:200:49;4144:202:238;1304:26:60;;;;;;10438:48:92;;;;;;:::i;:::-;;;;;;;;;;;;;;;;3767:191:99;;;;;;:::i;:::-;;:::i;1083:31:60:-;;;;;;;;5236:4:238;5224:17;;;5206:36;;5194:2;5179:18;1083:31:60;5064:184:238;14263:509:92;;;;;;:::i;:::-;;:::i;5327:177:60:-;;;:::i;16403:798:92:-;;;;;;:::i;:::-;;:::i;1149:28:59:-;;;;;;;;-1:-1:-1;;;;;6445:32:238;;;6427:51;;6415:2;6400:18;1149:28:59;6266:218:238;19299:48:92;;19341:6;19299:48;;;;;-1:-1:-1;;;;;6651:31:238;;;6633:50;;6621:2;6606:18;19299:48:92;6489:200:238;11413:283:92;;;;;;:::i;:::-;;:::i;9354:105::-;;;:::i;:::-;;;;;;;:::i;55210:526::-;;;;;;:::i;:::-;;:::i;1930:33::-;;;;;-1:-1:-1;;;1930:33:92;;;;;;52505:1506;;;;;;:::i;:::-;;:::i;10633:297::-;;;;;;:::i;:::-;;:::i;50214:402::-;;;;;;:::i;:::-;;:::i;54828:219::-;;;;;;:::i;:::-;;:::i;8621:31::-;;;;;;:::i;:::-;;:::i;:::-;;;8697:10:238;8685:23;;;8667:42;;8655:2;8640:18;8621:31:92;8523:192:238;22904:54:92;;22957:1;22904:54;;22392:179;;;:::i;11846:176::-;;;;;;:::i;:::-;;:::i;2302:57:99:-;;;;;;:::i;:::-;;;;;;;;;;;;;;27922:669:92;;;;;;:::i;:::-;;:::i;49766:55::-;;49815:6;49766:55;;1337:44:60;;;;;;:::i;:::-;;;;;;;;;;;;;;9144:109:92;;;:::i;8977:63::-;;;;;;:::i;:::-;;:::i;:::-;;;;;;;;;;:::i;60065:893::-;;;:::i;:::-;;;;;;;;;:::i;23381:34::-;;;;;1751:41:60;;;;;;:::i;:::-;;;;;;;;;;;;;;47434:32:92;;;;;-1:-1:-1;;;;;47434:32:92;;;690:20:58;;;;;-1:-1:-1;;;;;690:20:58;;;8856:46:92;;;;;;:::i;:::-;;;;;;;;;;;;;;;;28867:554;;;;;;:::i;:::-;;:::i;1056:20:60:-;;;:::i;12378:1626:92:-;;;;;;:::i;:::-;;:::i;1776:23::-;;;;;-1:-1:-1;;;1776:23:92;;;;;;2525:366:99;;;;;;:::i;:::-;;:::i;2078:29:92:-;;;;;-1:-1:-1;;;2078:29:92;;;;;;2138:58:99;;;;;;14968:485:92;;;;;;:::i;:::-;;:::i;21674:105::-;;;:::i;40172:216::-;;;:::i;3512:169:99:-;;;;;;:::i;:::-;;:::i;2586:153:92:-;;;;;;:::i;:::-;;:::i;54494:219::-;;;;;;:::i;:::-;;:::i;20087:215::-;;;;;;:::i;:::-;;:::i;21094:175::-;;;:::i;41769:249::-;;;;;;:::i;:::-;;:::i;31050:449::-;;;;;;:::i;:::-;;:::i;19653:260::-;;;;;;:::i;:::-;;:::i;32377:484::-;;;;;;:::i;:::-;;:::i;1660:22::-;;;;;-1:-1:-1;;;1660:22:92;;;;;;49954:52;;;;;;4093:954;;;;;;:::i;:::-;;:::i;55893:322::-;;;;;;:::i;:::-;;:::i;41288:257::-;;;;;;:::i;:::-;;:::i;47775:331::-;;;;;;:::i;:::-;;:::i;10272:48::-;;;;;;:::i;:::-;;;;;;;;;;;;;;;;22764:56;;22819:1;22764:56;;44805:161;;;;;;:::i;:::-;;:::i;1539:18::-;;;;;-1:-1:-1;;;1539:18:92;;;;;;11081:187;;;;;;:::i;:::-;;:::i;2304:29::-;;;;;-1:-1:-1;;;;;2304:29:92;;;;;;-1:-1:-1;;;;;14537:32:238;;;14519:51;;14507:2;14492:18;2304:29:92;14373:203:238;3838:1483:60;;;;;;:::i;:::-;;:::i;3233:30:92:-;;;;;-1:-1:-1;;;;;3233:30:92;;;45321:158;;;;;;:::i;:::-;;:::i;1388:64:60:-;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;8746:29:92;;;;;;:::i;:::-;;:::i;18999:207::-;;;;;;;-1:-1:-1;;;;;18999:207:92;;;;;;;;;;-1:-1:-1;;;18999:207:92;;;;;;-1:-1:-1;;;;;18999:207:92;;;;;;;-1:-1:-1;;;;;16408:15:238;;;16390:34;;16460:15;;;16455:2;16440:18;;16433:43;16512:15;;;;16492:18;;;16485:43;-1:-1:-1;;;;;16564:32:238;;;16559:2;16544:18;;16537:60;16340:3;16325:19;18999:207:92;16128:475:238;19432:41:92;;19469:4;19432:41;;1312:161:58;;;;;;:::i;:::-;;:::i;9570:42:92:-;;9610:2;9570:42;;39746:214;39799:14;39825:16;:14;:16::i;:::-;39860:6;;-1:-1:-1;;;39860:6:92;;;;39859:7;39851:30;;;;-1:-1:-1;;;39851:30:92;;;;;;;:::i;:::-;;;;;;;;;39900:53;39947:5;39900:46;:53::i;:::-;39891:62;;39746:214;:::o;1031:18:60:-;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::o;9741:413:92:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;9820:26:92::1;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;::::1;;9815:75;;9855:35;::::0;-1:-1:-1;;;9855:35:92;;8697:10:238;8685:23;;9855:35:92::1;::::0;::::1;8667:42:238::0;8640:18;;9855:35:92::1;8523:192:238::0;9815:75:92::1;9928:5;-1:-1:-1::0;;;;;9904:29:92::1;:20;9913:10;9904:8;:20::i;:::-;-1:-1:-1::0;;;;;9904:29:92::1;;9900:110;;9972:5;9988:20;9997:10;9988:8;:20::i;:::-;9942:68;::::0;-1:-1:-1;;;9942:68:92;;-1:-1:-1;;;;;17903:15:238;;;9942:68:92::1;::::0;::::1;17885:34:238::0;17955:15;;17935:18;;;17928:43;17820:18;;9942:68:92::1;17673:304:238::0;9900:110:92::1;10024:27;::::0;::::1;;::::0;;;:15:::1;:27;::::0;;;;:34;-1:-1:-1;;;10024:34:92;::::1;;;10020:89;;;10067:42;::::0;-1:-1:-1;;;10067:42:92;;8697:10:238;8685:23;;10067:42:92::1;::::0;::::1;8667::238::0;8640:18;;10067:42:92::1;8523:192:238::0;10020:89:92::1;10119:15;:28:::0;;::::1;::::0;;::::1;-1:-1:-1::0;;;10119:28:92::1;-1:-1:-1::0;;;;10119:28:92;;::::1;::::0;;;::::1;::::0;;9741:413::o;40709:258::-;40780:14;40807:20;40829;40853:36;40883:5;40853:29;:36::i;:::-;40806:83;;;;40908:52;40925:6;40933:12;40947;40908:16;:52::i;:::-;40899:61;40709:258;-1:-1:-1;;;;40709:258:92:o;2461:211:60:-;2561:10;2535:4;2551:21;;;:9;:21;;;;;;;;-1:-1:-1;;;;;2551:30:60;;;;;;;;;;:39;;;2606:37;2535:4;;2551:30;;2606:37;;;;2584:6;160:25:238;;148:2;133:18;;14:177;2606:37:60;;;;;;;;-1:-1:-1;2661:4:60;2461:211;;;;;:::o;42249:258:92:-;42320:14;42347:20;42369;42393:36;42423:5;42393:29;:36::i;:::-;42346:83;;;;42448:52;42465:6;42473:12;42487;42448:16;:52::i;22143:146::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;22200:18:92::1;:16;:18::i;:::-;22228:10;:17:::0;;-1:-1:-1;;;;22228:17:92::1;-1:-1:-1::0;;;22228:17:92::1;::::0;;22261:21:::1;::::0;-1:-1:-1;2090:41:238;;22261:21:92::1;::::0;2078:2:238;2063:18;22261:21:92::1;;;;;;;;22143:146::o:0;3767:191:99:-;3856:4;3872:26;3893:4;3872:20;:26::i;:::-;3915:36;3934:4;3940:2;3944:6;3915:18;:36::i;14263:509:92:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;14387:17:92::1;14407:11;:59;;14444:15;14460:5;14444:22;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;14407:59;;;14421:13;14435:5;14421:20;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;14407:59;14387:79;;14564:23;14590:22;14601:10;14590;:22::i;:::-;14564:48:::0;-1:-1:-1;14626:19:92;;14622:85:::1;;14654:53;::::0;-1:-1:-1;;;14654:53:92;;18316:10:238;18304:23;;14654:53:92::1;::::0;::::1;18286:42:238::0;18344:18;;;18337:34;;;18259:18;;14654:53:92::1;18114:263:238::0;14622:85:92::1;14718:47;14734:5;14741:10;14753:11;14718:15;:47::i;:::-;14338:434;;14263:509:::0;;:::o;5327:177:60:-;5384:7;5427:16;5410:13;:33;:87;;5473:24;:22;:24::i;5410:87::-;-1:-1:-1;5446:24:60;;5327:177::o;16403:798:92:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;16564:19:92::1;16593::::0;16627:11:::1;16623:497;;;16669:13;16683:6;16669:21;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;16654:36;;16719:13;16733:6;16719:21;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;16704:36;;16835:12;16849;16786:13;16800:6;16786:21;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;16809:13;16823:6;16809:21;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;16785:77;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;16623:497;;;16908:15;16924:6;16908:23;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;16893:38;;16960:15;16976:6;16960:23;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;16945:38;;17082:12;17096;17029:15;17045:6;17029:23;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;17054:15;17070:6;17054:23;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;17028:81;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;16623:497;17135:59;::::0;;18617:10:238;18654:15;;;18636:34;;18706:15;;;18701:2;18686:18;;18679:43;18758:15;;;18738:18;;;18731:43;18810:15;;18805:2;18790:18;;18783:43;17135:59:92;;::::1;::::0;;;;18594:3:238;17135:59:92;;::::1;16493:708;;16403:798:::0;;;:::o;11413:283::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;11549:45:92::1;::::0;-1:-1:-1;;;11549:45:92;;-1:-1:-1;;;;;6445:32:238;;;11549:45:92::1;::::0;::::1;6427:51:238::0;11549:8:92::1;:36;::::0;::::1;::::0;6400:18:238;;11549:45:92::1;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;::::0;::::1;;;;;-1:-1:-1::0;;;;;;;;;11604:25:92;::::1;;::::0;;;:16:::1;:25;::::0;;;;;;;;:32;;-1:-1:-1;;11604:32:92::1;11632:4;11604:32:::0;;::::1;::::0;;;11651:38;;19005:51:238;;;19072:18;;;19065:50;11651:38:92::1;::::0;18978:18:238;11651:38:92::1;;;;;;;;11413:283:::0;:::o;9354:105::-;9405:15;9439:13;9432:20;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;9354:105;:::o;55210:526::-;55292:10;;55269:7;;-1:-1:-1;;;55292:10:92;;;;55288:24;;;-1:-1:-1;55311:1:92;;55210:526;-1:-1:-1;55210:526:92:o;55288:24::-;55338:14;;-1:-1:-1;;;;;55338:14:92;-1:-1:-1;;;;;;55366:44:92;;55362:74;;-1:-1:-1;;;55419:17:92;55210:526;-1:-1:-1;;55210:526:92:o;55362:74::-;55448:20;55470;55494:35;55524:4;55494:29;:35::i;:::-;55447:82;;;;55559:4;-1:-1:-1;;;;;55543:20:92;:12;:20;55539:191;;-1:-1:-1;55572:1:92;;55210:526;-1:-1:-1;;;;55210:526:92:o;55539:191::-;55602:18;55623:19;55630:12;-1:-1:-1;;;;;55623:19:92;;;:::i;:::-;55602:40;;55663:56;55680:10;55692:12;55706;55663:16;:56::i;:::-;55656:63;55210:526;-1:-1:-1;;;;;;55210:526:92:o;55539:191::-;55278:458;;;55210:526;;;:::o;52505:1506::-;2897:6;;-1:-1:-1;;;2897:6:92;;;;2896:7;2888:30;;;;-1:-1:-1;;;2888:30:92;;;;;;;:::i;:::-;2929:6;:13;;-1:-1:-1;;;;;;;2929:13:92;;;;;;-1:-1:-1;;;;;52615:5:92;52601:10:::1;:19;::::0;::::1;::::0;:54:::1;;-1:-1:-1::0;52638:17:92::1;::::0;-1:-1:-1;;;;;52638:17:92::1;52624:10;:31;;52601:54;52597:105;;;52664:38;;-1:-1:-1::0;;;52664:38:92::1;;;;;;;;;;;52597:105;52712:18;:16;:18::i;:::-;52740:16;:14;:16::i;:::-;52766:21;:28:::0;;-1:-1:-1;;;;52766:28:92::1;-1:-1:-1::0;;;52766:28:92::1;::::0;;;;;;53042:53:::1;52766:28:::0;53042:46:::1;:53::i;:::-;53008:87;;53132:74;53173:25;;53166:4;:32;;;;:::i;:::-;53132:23:::0;;53201:4:::1;53132:32;:74::i;:::-;53109:97;;53243:74;53284:25;;53277:4;:32;;;;:::i;53243:74::-;53345:11;::::0;53220:97;;-1:-1:-1;53345:11:92;-1:-1:-1;53411:23:92::1;::::0;-1:-1:-1;53411:23:92::1;53429:4:::0;;53411:23:::1;:::i;:::-;:17;:23::i;:::-;53583:14;53600:53;53647:5;53600:46;:53::i;:::-;53583:70;;53676:20;53667:6;:29;:62;;;;53709:20;53700:6;:29;53667:62;53663:190;;;53752:90;::::0;-1:-1:-1;;;53752:90:92;;::::1;::::0;::::1;22514:25:238::0;;;22555:18;;;22548:34;;;22598:18;;;22591:34;;;22487:18;;53752:90:92::1;22312:319:238::0;53663:190:92::1;53881:11;;53866;:26;53862:102;;53939:11;::::0;53901:63:::1;::::0;-1:-1:-1;;;53901:63:92;;::::1;::::0;::::1;22810:25:238::0;;;;22851:18;;;22844:34;;;22783:18;;53901:63:92::1;22636:248:238::0;53862:102:92::1;-1:-1:-1::0;;53975:21:92::1;:29:::0;;-1:-1:-1;;;;2965:14:92;;;-1:-1:-1;;;;52505:1506:92:o;10633:297::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;10771:49:92::1;::::0;-1:-1:-1;;;10771:49:92;;8697:10:238;8685:23;;10771:49:92::1;::::0;::::1;8667:42:238::0;10771:8:92::1;-1:-1:-1::0;;;;;10771:37:92::1;::::0;::::1;::::0;8640:18:238;;10771:49:92::1;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;::::0;::::1;;;;;-1:-1:-1::0;;;;10830:29:92::1;::::0;::::1;;::::0;;;:17:::1;:29;::::0;;;;;;;;:36;;-1:-1:-1;;10830:36:92::1;10862:4;10830:36:::0;;::::1;::::0;;;10881:42;;23055::238;;;23113:18;;;23106:50;10881:42:92::1;::::0;23028:18:238;10881:42:92::1;22889:273:238::0;50214:402:92;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;49815:6:92::1;50300:38:::0;::::1;50296:135;;;50359:72;::::0;-1:-1:-1;;;50359:72:92;;::::1;::::0;::::1;23340:25:238::0;;;49815:6:92::1;23381:18:238::0;;;23374:59;23313:18;;50359:72:92::1;23167:272:238::0;50296:135:92::1;50465:25;::::0;;50500:40;;;;50556:53:::1;::::0;;22810:25:238;;;22866:2;22851:18;;22844:34;;;50556:53:92::1;::::0;22783:18:238;50556:53:92::1;;;;;;;;50286:330;50214:402:::0;:::o;54828:219::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;54940:14:92::1;::::0;-1:-1:-1;;;;;54940:14:92;;::::1;54919:35:::0;;::::1;;54915:79;;;54963:31;;-1:-1:-1::0;;;54963:31:92::1;;;;;;;;;;;54915:79;55005:14;:35:::0;;-1:-1:-1;;;;;;55005:35:92::1;-1:-1:-1::0;;;;;55005:35:92;;;::::1;::::0;;;::::1;::::0;;54828:219::o;8621:31::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::o;22392:179::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;22450:10:92::1;::::0;-1:-1:-1;;;22450:10:92;::::1;;;22445:53;;22469:29;;-1:-1:-1::0;;;22469:29:92::1;;;;;;;;;;;22445:53;22508:10;:18:::0;;-1:-1:-1;;;;22508:18:92::1;::::0;;22542:22:::1;::::0;-1:-1:-1;2090:41:238;;22542:22:92::1;::::0;2078:2:238;2063:18;22542:22:92::1;1950:187:238::0;11846:176:92;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;-1:-1:-1;;;;;11928:25:92;::::1;11956:5;11928:25:::0;;;:16:::1;:25;::::0;;;;;;;:33;;-1:-1:-1;;11928:33:92::1;::::0;;11976:39;;19005:51:238;;;19072:18;;;19065:50;;;;11976:39:92::1;::::0;18978:18:238;11976:39:92::1;18837:284:238::0;27922:669:92;2897:6;;28011:14;;-1:-1:-1;;;2897:6:92;;;;2896:7;2888:30;;;;-1:-1:-1;;;2888:30:92;;;;;;;:::i;:::-;2929:6;:13;;-1:-1:-1;;;;2929:13:92;-1:-1:-1;;;2929:13:92;;;;;28223:35:::1;2938:4:::0;28223:29:::1;:35::i;:::-;28176:82;;;;28358:52;28375:6;28383:12;28397;28358:16;:52::i;:::-;28349:61;;;28415:1;28348:68:::0;28344:101:::1;;28425:20;;-1:-1:-1::0;;;28425:20:92::1;;;;;;;;;;;28344:101;28486:14;::::0;-1:-1:-1;;;;;28486:14:92::1;28461:21;28476:6:::0;28461:12;:21:::1;:::i;:::-;28460:40;28456:85;;;28509:32;;-1:-1:-1::0;;;28509:32:92::1;;;;;;;;;;;28456:85;28552:32;28559:6;28567;28575:8;28552:6;:32::i;:::-;-1:-1:-1::0;;2965:6:92;:14;;-1:-1:-1;;;;2965:14:92;;;27922:669;;-1:-1:-1;;27922:669:92:o;9144:109::-;9197:15;9231;9224:22;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;9144:109;:::o;8977:63::-;;;;;;;;;;;;;;;;;;-1:-1:-1;;;;;8977:63:92;;;-1:-1:-1;;;8977:63:92;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::o;60065:893::-;60250:15;:22;60300:13;:20;60144:21;;;;;;60250:22;60351:19;60300:20;60250:22;60351:19;:::i;:::-;-1:-1:-1;;;;;60339:32:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;60339:32:92;-1:-1:-1;60330:41:92;-1:-1:-1;60406:19:92;60418:7;60406:9;:19;:::i;:::-;-1:-1:-1;;;;;60392:34:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;60392:34:92;-1:-1:-1;60381:45:92;-1:-1:-1;60456:19:92;60468:7;60456:9;:19;:::i;:::-;-1:-1:-1;;;;;60445:31:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;60445:31:92;;60436:40;;60491:9;60486:194;60510:9;60506:1;:13;60486:194;;;60552:28;60561:15;60577:1;60561:18;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;60552:8;:28::i;:::-;60540:6;60547:1;60540:9;;;;;;;;:::i;:::-;;;;;;:40;-1:-1:-1;;;;;60540:40:92;;;-1:-1:-1;;;;;60540:40:92;;;;;60608:30;60619:15;60635:1;60619:18;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;60608:10;:30::i;:::-;60594:8;60603:1;60594:11;;;;;;;;:::i;:::-;;;;;;:44;;;;;60664:5;60652:6;60659:1;60652:9;;;;;;;;:::i;:::-;:17;;;:9;;;;;;;;;;;:17;60521:3;;;:::i;:::-;;;60486:194;;;;60695:9;60690:262;60714:7;60710:1;:11;60690:262;;;60779:26;60788:13;60802:1;60788:16;;;;;;;;:::i;60779:26::-;60753:15;:22;60742:6;;60749:26;;:1;:26;:::i;:::-;60742:34;;;;;;;;:::i;:::-;;;;;;:63;-1:-1:-1;;;;;60742:63:92;;;-1:-1:-1;;;;;60742:63:92;;;;;60858:28;60869:13;60883:1;60869:16;;;;;;;;:::i;60858:28::-;60832:15;:22;60819:8;;60828:26;;:1;:26;:::i;:::-;60819:36;;;;;;;;:::i;:::-;;;;;;;;;;:67;60911:15;:22;60937:4;;60900:6;;60907:26;;:1;:26;:::i;:::-;60900:34;;;;;;;;:::i;:::-;:41;;;:34;;;;;;;;;;;:41;60723:3;;;:::i;:::-;;;60690:262;;;;60220:738;;60065:893;;;:::o;28867:554::-;2897:6;;28953:14;;-1:-1:-1;;;2897:6:92;;;;2896:7;2888:30;;;;-1:-1:-1;;;2888:30:92;;;;;;;:::i;:::-;2929:6;:13;;-1:-1:-1;;;;2929:13:92;-1:-1:-1;;;2929:13:92;;;;;29026:35:::1;2938:4:::0;29026:29:::1;:35::i;:::-;28979:82;;;;29192:48;29205:6;29213:12;29227;29192;:48::i;:::-;29183:57;;;29245:1;29182:64:::0;29178:97:::1;;29255:20;;-1:-1:-1::0;;;29255:20:92::1;;;;;;;;;;;29178:97;29316:14;::::0;-1:-1:-1;;;;;29316:14:92::1;29291:21;29306:6:::0;29291:12;:21:::1;:::i;:::-;29290:40;29286:85;;;29339:32;;-1:-1:-1::0;;;29339:32:92::1;;;;;;;;;;;29286:85;29382:32;29389:6;29397;29405:8;29382:6;:32::i;1056:20:60:-:0;;;;;;;:::i;12378:1626:92:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;12547:18:92::1;:16;:18::i;:::-;12632:26;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;::::1;;12628:78;;;12667:39;::::0;-1:-1:-1;;;12667:39:92;;8697:10:238;8685:23;;12667:39:92::1;::::0;::::1;8667:42:238::0;8640:18;;12667:39:92::1;8523:192:238::0;12628:78:92::1;12781:29;::::0;::::1;;::::0;;;:17:::1;:29;::::0;;;;;::::1;;12776:85;;12819:42;::::0;-1:-1:-1;;;12819:42:92;;8697:10:238;8685:23;;12819:42:92::1;::::0;::::1;8667::238::0;8640:18;;12819:42:92::1;8523:192:238::0;12776:85:92::1;13045:40;::::0;-1:-1:-1;;;13045:40:92;;::::1;8685:23:238::0;;13045:40:92::1;::::0;::::1;8667:42:238::0;12987:15:92::1;::::0;;;;;-1:-1:-1;;;;;13045:8:92::1;:28;::::0;::::1;::::0;8640:18:238;;13045:40:92::1;;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;;;;;;::::0;;::::1;-1:-1:-1::0;;13045:40:92::1;::::0;::::1;;::::0;::::1;::::0;;;::::1;::::0;::::1;:::i;:::-;12986:99;;;;;;13110:11;13100:21;;:6;:21;;;13096:66;;13130:32;::::0;-1:-1:-1;;;13130:32:92;;8697:10:238;8685:23;;13130:32:92::1;::::0;::::1;8667:42:238::0;8640:18;;13130:32:92::1;8523:192:238::0;13096:66:92::1;13256:179;::::0;;::::1;::::0;::::1;::::0;;-1:-1:-1;;;;;13256:179:92;;::::1;::::0;;;::::1;;;::::0;;::::1;::::0;;;;;;;;;;;;;;;13226:27:::1;::::0;::::1;-1:-1:-1::0;13226:27:92;;;:15:::1;:27:::0;;;;;;:209;;;;;;::::1;;-1:-1:-1::0;;;13226:209:92::1;-1:-1:-1::0;;;;;;13226:209:92;;;;::::1;::::0;;;;::::1;::::0;;;;13256:179;;13226:27;:209;;::::1;::::0;::::1;::::0;;::::1;:::i;:::-;-1:-1:-1::0;13226:209:92::1;::::0;::::1;::::0;::::1;::::0;::::1;::::0;::::1;::::0;;::::1;:::i;:::-;;;;;13450:6;13446:460;;;13476:13;:20:::0;9610:2:::1;-1:-1:-1::0;13472:90:92::1;;13522:40;::::0;-1:-1:-1;;;13522:40:92;;9610:2:::1;13522:40;::::0;::::1;160:25:238::0;133:18;;13522:40:92::1;14:177:238::0;13472:90:92::1;13630:36;:13;13648:5:::0;13655:10;13630:17:::1;:36::i;:::-;13446:460;;;13701:15;:22:::0;9610:2:::1;-1:-1:-1::0;13697:92:92::1;;13749:40;::::0;-1:-1:-1;;;13749:40:92;;9610:2:::1;13749:40;::::0;::::1;160:25:238::0;133:18;;13749:40:92::1;14:177:238::0;13697:92:92::1;13857:38;:15;13877:5:::0;13884:10;13857:19:::1;:38::i;:::-;13916:26;::::0;::::1;;::::0;;;:14:::1;:26;::::0;;;;;;:33;;-1:-1:-1;;13916:33:92::1;13945:4;13916:33;::::0;;13965:32;::::1;::::0;::::1;::::0;13931:10;;13991:5;;26871:10:238;26908:15;;;26890:34;;26960:15;;26955:2;26940:18;;26933:43;26849:2;26834:18;;26690:292;13965:32:92::1;;;;;;;;12537:1467;;;12378:1626:::0;;;;:::o;2525:366:99:-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;1833:6:99::1;2603:7;:35;:74;;;;1984:6;2642:7;:35;2603:74;2599:131;;;2698:32;;-1:-1:-1::0;;;2698:32:99::1;;;;;;;;;;;2599:131;2767:15;::::0;;2792:25;;;;2832:52:::1;::::0;;22810:25:238;;;22866:2;22851:18;;22844:34;;;2832:52:99::1;::::0;22783:18:238;2832:52:99::1;22636:248:238::0;14968:485:92;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;15113:18:92::1;15134:11;:59;;15171:15;15187:5;15171:22;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;15134:59;;;15148:13;15162:5;15148:20;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;15134:59;15113:80;;15280:11;15266:25;;:10;:25;;;;:67;;;-1:-1:-1::0;15295:38:92::1;::::0;-1:-1:-1;;;15295:38:92;;8697:10:238;8685:23;;15295:38:92::1;::::0;::::1;8667:42:238::0;15295:8:92::1;-1:-1:-1::0;;;;;15295:26:92::1;::::0;::::1;::::0;8640:18:238;;15295:38:92::1;;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;15262:126;;;15354:34;;-1:-1:-1::0;;;15354:34:92::1;;;;;;;;;;;15262:126;15399:47;15415:5;15422:10;15434:11;15399:15;:47::i;21674:105::-:0;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;21746:11:92::1;::::0;-1:-1:-1;;;21746:11:92;::::1;;;:26;;21768:4;21746:26;;;21760:5;21746:26;21732:11;:40:::0;;;::::1;;-1:-1:-1::0;;;21732:40:92::1;-1:-1:-1::0;;;;21732:40:92;;::::1;::::0;;;::::1;::::0;;21674:105::o;40172:216::-;40228:14;40254:16;:14;:16::i;:::-;40289:6;;-1:-1:-1;;;40289:6:92;;;;40288:7;40280:30;;;;-1:-1:-1;;;40280:30:92;;;;;;;:::i;:::-;40329:52;40376:4;40329:46;:52::i;3512:169:99:-;3583:4;3599:32;3620:10;3599:20;:32::i;:::-;3648:26;3663:2;3667:6;3648:14;:26::i;:::-;3641:33;3512:169;-1:-1:-1;;;3512:169:99:o;2586:153:92:-;2652:9;2647:85;2667:15;;;2647:85;;;2689:43;2724:4;;2729:1;2724:7;;;;;;;:::i;:::-;;;;;;;;;;;;:::i;:::-;2689:43;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;2697:4:92;;2689:43;-1:-1:-1;;2689:34:92;:43;-1:-1:-1;2689:43:92:i;:::-;-1:-1:-1;2684:3:92;;;;:::i;:::-;;;;2647:85;;;;2586:153;;:::o;54494:219::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;54606:14:92::1;::::0;-1:-1:-1;;;;;54606:14:92;;::::1;54585:35:::0;;::::1;;54581:79;;;54629:31;;-1:-1:-1::0;;;54629:31:92::1;;;;;;;;;;;20087:215:::0;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;20204:31:92;;20173:71:::1;::::0;;-1:-1:-1;;;;;20204:31:92;;::::1;17885:34:238::0;;17955:15;;;17950:2;17935:18;;17928:43;20173:71:92::1;::::0;17820:18:238;20173:71:92::1;;;;;;;20255:31:::0;:40;;-1:-1:-1;;;;;;20255:40:92::1;-1:-1:-1::0;;;;;20255:40:92;;;::::1;::::0;;;::::1;::::0;;20087:215::o;21094:175::-;21158:11;;21137:4;;-1:-1:-1;;;21158:11:92;;;;21153:88;;21192:38;;-1:-1:-1;;;21192:38:92;;21224:4;21192:38;;;6427:51:238;21192:8:92;-1:-1:-1;;;;;21192:23:92;;;;6400:18:238;;21192:38:92;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;21153:88::-;-1:-1:-1;21257:5:92;;21094:175::o;41769:249::-;41836:14;41863:20;41885;41909:35;41939:4;41909:29;:35::i;:::-;41862:82;;;;41963:48;41976:6;41984:12;41998;41963;:48::i;31050:449::-;2897:6;;31185:14;;-1:-1:-1;;;2897:6:92;;;;2896:7;2888:30;;;;-1:-1:-1;;;2888:30:92;;;;;;;:::i;:::-;2929:6;:13;;-1:-1:-1;;;;2929:13:92;-1:-1:-1;;;2929:13:92;;;;;31258:36:::1;2929:13:::0;31258:29:::1;:36::i;:::-;31211:83;;;;31391:52;31408:6;31416:12;31430;31391:16;:52::i;:::-;31382:61;;31454:38;31460:6;31468;31476:8;31486:5;31454;:38::i;:::-;-1:-1:-1::0;;2965:6:92;:14;;-1:-1:-1;;;;2965:14:92;;;31050:449;;-1:-1:-1;;;31050:449:92:o;19653:260::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;19469:4:92::1;-1:-1:-1::0;;;;;19732:17:92;::::1;;19728:53;;;19758:23;;-1:-1:-1::0;;;19758:23:92::1;;;;;;;;;;;19728:53;19825:7;:29:::0;19796:64:::1;::::0;;-1:-1:-1;;;;;19825:29:92;;::::1;27970:34:238::0;;28040:15;;;28035:2;28020:18;;28013:43;19796:64:92::1;::::0;27906:18:238;19796:64:92::1;;;;;;;19871:7;:35:::0;;-1:-1:-1;;19871:35:92::1;-1:-1:-1::0;;;;;19871:35:92;;;::::1;::::0;;;::::1;::::0;;19653:260::o;32377:484::-;2897:6;;32510:14;;-1:-1:-1;;;2897:6:92;;;;2896:7;2888:30;;;;-1:-1:-1;;;2888:30:92;;;;;;;:::i;:::-;2929:6;:13;;-1:-1:-1;;;;2929:13:92;-1:-1:-1;;;2929:13:92;;;;;32583:36:::1;2929:13:::0;32583:29:::1;:36::i;:::-;32536:83;;;;32718:52;32735:6;32743:12;32757;32718:16;:52::i;:::-;32709:61;;;32775:1;32708:68:::0;32704:101:::1;;32785:20;;-1:-1:-1::0;;;32785:20:92::1;;;;;;;;;;;32704:101;32816:38;32822:6;32830;32838:8;32848:5;32816;:38::i;4093:954::-:0;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;4253:17:92::1;4280::::0;4312:16:::1;4308:235;;;4344:20;4367:13;:11;:13::i;:::-;4344:36:::0;-1:-1:-1;4406:50:92::1;4430:20;4436:14:::0;4430:3:::1;:20;:::i;:::-;4406:12:::0;;:50:::1;;4452:3;4406:23;:50::i;:::-;4394:62:::0;-1:-1:-1;4482:50:92::1;4506:20;4512:14:::0;4506:3:::1;:20;:::i;4482:50::-;4470:62;;4330:213;4308:235;4644:85;22957:1;4709:19;4644:36;:85::i;:::-;4740:11;:46:::0;;-1:-1:-1;;;;;;4740:46:92::1;-1:-1:-1::0;;;;;4740:46:92;::::1;;::::0;;-1:-1:-1;4818:13:92::1;:11;:13::i;:::-;4796:35;;4846:16;4842:199;;;4896:9;4882:11;:23;:50;;;;4923:9;4909:11;:23;4882:50;4878:152;;;4957:73;::::0;-1:-1:-1;;;4957:73:92;;::::1;::::0;::::1;22514:25:238::0;;;22555:18;;;22548:34;;;22598:18;;;22591:34;;;22487:18;;4957:73:92::1;22312:319:238::0;4878:152:92::1;4243:804;;;4093:954:::0;;;:::o;55893:322::-;55972:10;;55949:7;;-1:-1:-1;;;55972:10:92;;;;55968:24;;;-1:-1:-1;55991:1:92;;55893:322;-1:-1:-1;55893:322:92:o;55968:24::-;56037:14;;-1:-1:-1;;;;;56037:14:92;-1:-1:-1;;;;;;56029:44:92;;56025:74;;-1:-1:-1;;;56082:17:92;55893:322;-1:-1:-1;;55893:322:92:o;56025:74::-;56133:11;;-1:-1:-1;;;;;56162:20:92;;;;;:46;;56189:19;56196:12;-1:-1:-1;;;;;56189:19:92;;;:::i;:::-;56162:46;;;56185:1;56155:53;55893:322;-1:-1:-1;;;;55893:322:92:o;41288:257::-;41359:14;41386:20;41408;41432:35;41462:4;41432:29;:35::i;:::-;41385:82;;;;41486:52;41503:6;41511:12;41525;41486:16;:52::i;47775:331::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;47891:77:92::1;47928:11;47941:26;47891:36;:77::i;:::-;47978:17;:46:::0;;-1:-1:-1;;;;;;47978:46:92::1;-1:-1:-1::0;;;;;47978:46:92;::::1;::::0;;::::1;::::0;;;48039:60:::1;::::0;6427:51:238;;;48039:60:92::1;::::0;6415:2:238;6400:18;48039:60:92::1;6266:218:238::0;44805:161:92;44899:6;;44871:7;;-1:-1:-1;;;44899:6:92;;;;44898:7;44890:30;;;;-1:-1:-1;;;44890:30:92;;;;;;;:::i;:::-;44937:22;44946:5;44953;44937:8;:22::i;11081:187::-;778:5:58;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;11166:29:92::1;::::0;::::1;11198:5;11166:29:::0;;;:17:::1;:29;::::0;;;;;;;:37;;-1:-1:-1;;11166:37:92::1;::::0;;11218:43;;23055:42:238;;;23113:18;;;23106:50;;;;11218:43:92::1;::::0;23028:18:238;11218:43:92::1;22889:273:238::0;3838:1483:60;4057:15;4045:8;:27;;4037:63;;;;-1:-1:-1;;;4037:63:60;;28618:2:238;4037:63:60;;;28600:21:238;28657:2;28637:18;;;28630:30;28696:25;28676:18;;;28669:53;28739:18;;4037:63:60;28416:347:238;4037:63:60;4265:24;4292:805;4428:18;:16;:18::i;:::-;-1:-1:-1;;;;;4873:13:60;;;;;;;:6;:13;;;;;;;;;:15;;;;;;;;4511:449;;4555:165;4511:449;;;29055:25:238;29134:18;;;29127:43;;;;29206:15;;;29186:18;;;29179:43;29238:18;;;29231:34;;;29281:19;;;29274:35;;;;29325:19;;;;29318:35;;;4511:449:60;;;;;;;;;;29027:19:238;;;4511:449:60;;;4472:514;;;;;;;;-1:-1:-1;;;4350:658:60;;;29622:27:238;29665:11;;;29658:27;;;;29701:12;;;29694:28;;;;29738:12;;4350:658:60;;;-1:-1:-1;;4350:658:60;;;;;;;;;4319:707;;4350:658;4319:707;;;;4292:805;;;;;;;;;29988:25:238;30061:4;30049:17;;30029:18;;;30022:45;30083:18;;;30076:34;;;30126:18;;;30119:34;;;29960:19;;4292:805:60;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;;4292:805:60;;-1:-1:-1;;4292:805:60;;;-1:-1:-1;;;;;;;5120:30:60;;;;;;:59;;;5174:5;-1:-1:-1;;;;;5154:25:60;:16;-1:-1:-1;;;;;5154:25:60;;5120:59;5112:86;;;;-1:-1:-1;;;5112:86:60;;30366:2:238;5112:86:60;;;30348:21:238;30405:2;30385:18;;;30378:30;-1:-1:-1;;;30424:18:238;;;30417:44;30478:18;;5112:86:60;30164:338:238;5112:86:60;-1:-1:-1;;;;;5213:27:60;;;;;;;:9;:27;;;;;;;;:36;;;;;;;;;;;;;:44;;;5283:31;160:25:238;;;5213:36:60;;5283:31;;;;;133:18:238;5283:31:60;;;;;;;3838:1483;;;;;;;:::o;45321:158:92:-;45413:6;;45385:7;;-1:-1:-1;;;45413:6:92;;;;45412:7;45404:30;;;;-1:-1:-1;;;45404:30:92;;;;;;;:::i;:::-;45451:21;45460:5;45467:4;45451:8;:21::i;8746:29::-;;;;;;;;;;;;1312:161:58;778:5;;-1:-1:-1;;;;;778:5:58;764:10;:19;756:44;;;;-1:-1:-1;;;756:44:58;;;;;;;:::i;:::-;1392:5:::1;:16:::0;;-1:-1:-1;;;;;;1392:16:58::1;-1:-1:-1::0;;;;;1392:16:58;::::1;::::0;;::::1;::::0;;;1424:42:::1;::::0;1445:10:::1;::::0;1424:42:::1;::::0;-1:-1:-1;;1424:42:58::1;1312:161:::0;:::o;21362:166:92:-;21417:11;;-1:-1:-1;;;21417:11:92;;;;21412:110;;21448:38;;-1:-1:-1;;;21448:38:92;;21480:4;21448:38;;;6427:51:238;21448:8:92;-1:-1:-1;;;;;21448:23:92;;;;6400:18:238;;21448:38:92;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;21444:67;;;21495:16;;-1:-1:-1;;;21495:16:92;;;;;;;;;;;21444:67;21362:166::o;37117:2047::-;37292:15;:22;37235:14;;;37292:22;-1:-1:-1;;;;;37354:33:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;37354:33:92;;37324:63;;37397:31;37445:20;-1:-1:-1;;;;;37431:35:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;37431:35:92;;37397:69;;37556:18;37552:1606;;;37595:9;37590:385;37610:20;37606:1;:24;37590:385;;;37655:15;37673;37689:1;37673:18;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;37655:36;;37863:27;37881:8;37863:17;:27::i;:::-;37843:14;37858:1;37843:17;;;;;;;;:::i;:::-;;;;;;:47;;;;37895:1;37842:54;37838:68;;37898:8;;;37838:68;37942:18;37951:8;37942;:18::i;:::-;37924:12;37937:1;37924:15;;;;;;;;:::i;:::-;;;;;;:36;-1:-1:-1;;;;;37924:36:92;;;-1:-1:-1;;;;;37924:36:92;;;;;37637:338;37590:385;37632:3;;;:::i;:::-;;;37590:385;;;-1:-1:-1;37997:11:92;;:58;;-1:-1:-1;;;37997:58:92;;-1:-1:-1;;;;;37997:11:92;;;;:21;;:58;;38019:12;;38033:14;;38049:5;;37997:58;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;37988:67;;37552:1606;;;38115:13;:20;38086:26;38115:20;-1:-1:-1;;;;;38177:31:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;38177:31:92;;38149:59;;38222:29;38268:18;-1:-1:-1;;;;;38254:33:92;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;38254:33:92;;38222:65;;38306:9;38301:365;38321:20;38317:1;:24;38301:365;;;38366:15;38384;38400:1;38384:18;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;38366:36;;38561:20;38572:8;38561:10;:20::i;:::-;38541:14;38556:1;38541:17;;;;;;;;:::i;:::-;;;;;;:40;;;;38586:1;38540:47;38536:61;;38589:8;;;38536:61;38633:18;38642:8;38633;:18::i;:::-;38615:12;38628:1;38615:15;;;;;;;;:::i;:::-;;;;;;:36;-1:-1:-1;;;;;38615:36:92;;;-1:-1:-1;;;;;38615:36:92;;;;;38348:318;38301:365;38343:3;;;:::i;:::-;;;38301:365;;;;38684:9;38679:357;38699:18;38695:1;:22;38679:357;;;38742:15;38760:13;38774:1;38760:16;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;38742:34;;38933:20;38944:8;38933:10;:20::i;:::-;38915:12;38928:1;38915:15;;;;;;;;:::i;:::-;;;;;;:38;;;;38958:1;38914:45;38910:59;;38961:8;;;38910:59;39003:18;39012:8;39003;:18::i;:::-;38987:10;38998:1;38987:13;;;;;;;;:::i;:::-;;;;;;:34;-1:-1:-1;;;;;38987:34:92;;;-1:-1:-1;;;;;38987:34:92;;;;;38724:312;38679:357;38719:3;;;:::i;:::-;;;38679:357;;;-1:-1:-1;39058:11:92;;:89;;-1:-1:-1;;;39058:89:92;;-1:-1:-1;;;;;39058:11:92;;;;:26;;:89;;39085:12;;39099:14;;39115:10;;39127:12;;39141:5;;39058:89;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;39049:98;37117:2047;-1:-1:-1;;;;;;;;37117:2047:92:o;59041:217::-;59134:25;;;59099:5;59134:25;;;:15;:25;;;;;;:33;;59184:67;;-1:-1:-1;;;59184:67:92;;-1:-1:-1;;;;;59134:33:92;;;;;;59184:28;;:67;;59134:33;59213:37;;59184:67;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;36569:255::-;36659:20;36681;36728:53;36775:5;36728:46;:53::i;:::-;36713:68;;36806:11;;36791:26;;36569:255;;;:::o;45609:226::-;45748:14;45783:45;:6;45801:12;45815;45783:17;:45::i;46653:224::-;46792:14;46827:43;:6;46843:12;46857;46827:15;:43::i;21872:108::-;21928:10;;-1:-1:-1;;;21928:10:92;;;;21924:49;;;21947:26;;-1:-1:-1;;;21947:26:92;;;;;;;;;;;3050:380:99;-1:-1:-1;;;;;3138:29:99;;3119:16;3138:29;;;:22;:29;;;;;;3181:13;;3177:247;;3210:29;3253:15;;3242:8;:26;;;;:::i;:::-;3210:58;;3310:15;3286:21;:39;3282:131;;;3350:63;;-1:-1:-1;;;3350:63:99;;;;;22810:25:238;;;3397:15:99;22851:18:238;;;22844:34;22783:18;;3350:63:99;22636:248:238;3177:247:99;3109:321;3050:380;:::o;3057:592:60:-;-1:-1:-1;;;;;3209:15:60;;3175:4;3209:15;;;:9;:15;;;;;;;;3225:10;3209:27;;;;;;;;-1:-1:-1;;3287:28:60;;3283:80;;3347:16;3357:6;3347:7;:16;:::i;:::-;-1:-1:-1;;;;;3317:15:60;;;;;;:9;:15;;;;;;;;3333:10;3317:27;;;;;;;:46;3283:80;-1:-1:-1;;;;;3374:15:60;;;;;;:9;:15;;;;;:25;;3393:6;;3374:15;:25;;3393:6;;3374:25;:::i;:::-;;;;-1:-1:-1;;;;;;;3545:13:60;;;;;;;:9;:13;;;;;;;:23;;;;;;3594:26;3545:13;;3594:26;;;-1:-1:-1;;;;;;;;;;;3594:26:60;;;3562:6;160:25:238;;148:2;133:18;;14:177;3594:26:60;;;;;;;;-1:-1:-1;3638:4:60;;3057:592;-1:-1:-1;;;;3057:592:60:o;58681:223:92:-;58778:25;;;58741:7;58778:25;;;:15;:25;;;;;;:33;;58828:69;;-1:-1:-1;;;58828:69:92;;-1:-1:-1;;;;;58778:33:92;;;;;;58828:30;;:69;;58778:33;58859:37;;58828:69;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;15564:506::-;15677:15;;;-1:-1:-1;;;15677:15:92;;;;;15663:29;;;;15659:75;;15701:33;;-1:-1:-1;;;15701:33:92;;;;;;;;;;;15659:75;15749:11;15745:180;;;15827:27;:13;15848:5;15827:20;:27::i;:::-;15745:180;;;15885:29;:15;15908:5;15885:22;:29::i;:::-;15935:26;;;15964:5;15935:26;;;:14;:26;;;;;;;;:34;;-1:-1:-1;;15935:34:92;;;15986:15;:27;;;;;15979:34;;-1:-1:-1;;;;;;15979:34:92;;;15986:27;15979:34;15935;15979;;15964:5;15979:34;:::i;:::-;;;;;;;:::i;:::-;-1:-1:-1;;16029:34:92;;;26871:10:238;26908:15;;;26890:34;;26960:15;;26955:2;26940:18;;26933:43;16029:34:92;;26834:18:238;16029:34:92;;;;;;;15564:506;;;:::o;5510:446:60:-;5575:7;5672:95;5805:4;5789:22;;;;;;:::i;:::-;;;;;;;;;;5640:295;;;34719:25:238;;;;34760:18;;34753:34;;;;5833:14:60;34803:18:238;;;34796:34;5869:13:60;34846:18:238;;;34839:34;5912:4:60;34889:19:238;;;34882:61;34691:19;;5640:295:60;;;;;;;;;;;;5613:336;;;;;;5594:355;;5510:446;:::o;1842:722:204:-;2016:9;;;2147:19;;2140:27;2172:9;;2186;;;2183:16;;2169:31;2136:65;2126:121;;2231:1;2228;2221:12;2126:121;2545:1;2531:11;2527:1;2524;2520:9;2516:27;2512:35;2507:1;2500:9;2493:17;2489:59;2484:64;;1842:722;;;;;:::o;51159:560:92:-;51237:9;51232:481;51256:4;:11;51252:1;:15;51232:481;;;51288:15;51306:4;51311:1;51306:7;;;;;;;;:::i;:::-;;;;;;;;;;;;:15;-1:-1:-1;;;;;51413:25:92;;51306:15;51413:25;;;:16;:25;;;;;;;;51306:15;;-1:-1:-1;51413:25:92;;51408:79;;51447:40;;-1:-1:-1;;;51447:40:92;;-1:-1:-1;;;;;6445:32:238;;51447:40:92;;;6427:51:238;6400:18;;51447:40:92;6266:218:238;51408:79:92;51506:9;51501:202;51525:4;51530:1;51525:7;;;;;;;;:::i;:::-;;;;;;;:16;;;:23;51521:1;:27;51501:202;;;51573:49;51602:4;51607:1;51602:7;;;;;;;;:::i;:::-;;;;;;;:16;;;51619:1;51602:19;;;;;;;;:::i;:::-;;;;;;;51573:7;-1:-1:-1;;;;;51573:28:92;;;:49;;;;:::i;:::-;;51645:43;51659:7;51668:4;51673:1;51668:7;;;;;;;;:::i;:::-;;;;;;;:16;;;51685:1;51668:19;;;;;;;;:::i;:::-;;;;;;;51645:43;;;;;;;:::i;:::-;;;;;;;;51550:3;;;;:::i;:::-;;;;51501:202;;;;51274:439;51269:3;;;;:::i;:::-;;;51232:481;;45965:226;46104:14;46139:45;:6;46157:12;46171;46139:17;:45::i;27239:412::-;27324:39;27338:6;27346;27354:8;27324:13;:39::i;:::-;27443:57;-1:-1:-1;;;;;27443:5:92;:22;27466:10;27486:4;27493:6;27443:22;:57::i;:::-;27511:23;27517:8;27527:6;27511:5;:23::i;:::-;27550:45;;;22810:25:238;;;22866:2;22851:18;;22844:34;;;-1:-1:-1;;;;;27550:45:92;;;27558:10;;27550:45;;22783:18:238;27550:45:92;;;;;;;27606:38;27619:6;27627;27635:8;27606:12;:38::i;46310:220::-;46445:14;46480:43;:6;46496:12;46510;46480:15;:43::i;493:354:206:-;591:12;;618:7;;614:227;;641:5;;658:7;664:1;658:3;:7;:::i;:::-;652:14;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;641:26;;;;;;;;;;;;;;;;;;;;;;652:14;;;641:26;;;;;652:14;641:26;;;;;;;;;;652:14;;;;;;;;;;;;;;641:26;;;;652:14;699:7;;:3;:7;:::i;:::-;687:19;;682:65;712:5;708:9;;:1;:9;682:65;;;735:5;741;745:1;741;:5;:::i;:::-;735:12;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;724:5;730:1;724:8;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;:23;;;;;;;;;;;;;;;;;;719:3;;;;;:::i;:::-;;;;682:65;;;;777:5;762;768;762:12;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;:20;;;;;;;;;;;;;;;;;;614:227;;;-1:-1:-1;813:17:206;;;;;;;-1:-1:-1;813:17:206;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;493:354:206:o;2678:373:60:-;2774:10;2748:4;2764:21;;;:9;:21;;;;;:31;;2789:6;;2764:21;2748:4;;2764:31;;2789:6;;2764:31;:::i;:::-;;;;-1:-1:-1;;;;;;;2941:13:60;;;;;;:9;:13;;;;;;;:23;;;;;;2990:32;2999:10;;-1:-1:-1;;;;;;;;;;;2990:32:60;;;2958:6;160:25:238;;148:2;133:18;;14:177;6468:198:50;6551:12;6582:77;6603:6;6611:4;6582:77;;;;;;;;;;;;;;;;;:20;:77::i;29516:654:92:-;29615:47;29630:6;29638;29646:8;29656:5;29615:14;:47::i;:::-;29677:10;-1:-1:-1;;;;;29677:19:92;;;29673:228;;-1:-1:-1;;;;;29730:16:92;;29712:15;29730:16;;;:9;:16;;;;;;;;29747:10;29730:28;;;;;;;;-1:-1:-1;;29813:28:92;;29809:81;;29874:16;29884:6;29874:7;:16;:::i;:::-;-1:-1:-1;;;;;29843:16:92;;;;;;:9;:16;;;;;;;;29860:10;29843:28;;;;;;;:47;29809:81;29698:203;29673:228;29911:20;29917:5;29924:6;29911:5;:20::i;:::-;29947:53;;;22810:25:238;;;22866:2;22851:18;;22844:34;;;-1:-1:-1;;;;;29947:53:92;;;;;;;;29956:10;;29947:53;;22783:18:238;29947:53:92;;;;;;;30010:34;30027:6;30035:8;30010:16;:34::i;1331:505:204:-;1507:9;;;1638:19;;1631:27;1663:9;;1677;;;1674:16;;1660:31;1627:65;1617:121;;1722:1;1719;1712:12;1617:121;1801:19;;1331:505;-1:-1:-1;;1331:505:204:o;59670:307:92:-;59784:11;59799:1;59784:16;59780:79;;59809:50;;-1:-1:-1;;;59809:50:92;;;;;;;;;;;59780:79;59909:9;-1:-1:-1;;;;;59873:45:92;:8;-1:-1:-1;;;;;59873:19:92;;59893:11;59873:32;;;;;;;;;;;;;160:25:238;;148:2;133:18;;14:177;59873:32:92;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;-1:-1:-1;;;;;59873:45:92;;59869:101;;59927:43;;-1:-1:-1;;;59927:43:92;;;;;;;;;;;5419:389:99;5499:14;5534:31;5549:5;5556:8;5534:14;:31::i;:::-;-1:-1:-1;;;;;5594:29:99;;5575:16;5594:29;;;:22;:29;;;;;;5525:40;;-1:-1:-1;5637:13:99;;5633:169;;5666:29;5709:15;;5698:8;:26;;;;:::i;:::-;5666:58;;5766:15;5742:21;:39;5738:53;;;5790:1;5783:8;;;;;;5738:53;5652:150;5633:169;5515:293;5419:389;;;;:::o;57922:427:92:-;58078:25;;;57989:7;58078:25;;;:15;:25;;;;;:32;-1:-1:-1;;;58078:32:92;;;;58074:46;;;-1:-1:-1;58119:1:92;;57922:427;-1:-1:-1;57922:427:92:o;58074:46::-;58161:25;;;;;;;:15;:25;;;;;;;:33;;58149:193;;-1:-1:-1;;;58149:193:92;;-1:-1:-1;;;;;58161:33:92;;;;58149:63;;:193;;58161:33;58230:37;;;58285:43;;;58149:193;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;1042:258:206:-;1129:12;;1160:11;;;;-1:-1:-1;1152:43:206;;;;-1:-1:-1;;;1152:43:206;;36265:2:238;1152:43:206;;;36247:21:238;36304:2;36284:18;;;36277:30;-1:-1:-1;;;36323:18:238;;;36316:49;36382:18;;1152:43:206;36063:343:238;1152:43:206;1211:17;;;1206:65;1234:7;1240:1;1234:3;:7;:::i;:::-;1230:1;:11;1206:65;;;1259:5;1265;:1;1269;1265:5;:::i;:::-;1259:12;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;1248:5;1254:1;1248:8;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;:23;;;;;;;;;;;;;;;;;;1243:3;;;;;:::i;:::-;;;;1206:65;;;;1282:5;:11;;;;;;;:::i;:::-;;;;;;;;;-1:-1:-1;;1282:11:206;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;;;1042:258:206:o;4141:345:99:-;4247:45;4267:6;4275;4283:8;4247:19;:45::i;:::-;4306:10;-1:-1:-1;;;;;4306:22:99;;;4302:178;;4349:47;;-1:-1:-1;;;4349:47:99;;4385:10;4349:47;;;6427:51:238;4349:8:99;-1:-1:-1;;;;;4349:35:99;;;;6400:18:238;;4349:47:99;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;4344:125;;4421:48;;-1:-1:-1;;;4421:48:99;;4458:10;4421:48;;;6427:51:238;6400:18;;4421:48:99;6266:218:238;1328:1616:65;1466:12;1636:4;1630:11;-1:-1:-1;;;1759:17:65;1752:93;1892:4;1888:1;1869:17;1865:25;1858:39;1976:2;1971;1952:17;1948:26;1941:38;2056:6;2051:2;2032:17;2028:26;2021:42;2860:2;2857:1;2852:3;2833:17;2830:1;2823:5;2816;2811:52;2379:16;2372:24;2366:2;2348:16;2345:24;2341:1;2337;2331:8;2328:15;2324:46;2321:76;2121:756;2110:767;;;2905:7;2897:40;;;;-1:-1:-1;;;2897:40:65;;36745:2:238;2897:40:65;;;36727:21:238;36784:2;36764:18;;;36757:30;-1:-1:-1;;;36803:18:238;;;36796:50;36863:18;;2897:40:65;36543:344:238;2897:40:65;1456:1488;1328:1616;;;;:::o;6150:325:60:-;6235:6;6220:11;;:21;;;;;;;:::i;:::-;;;;-1:-1:-1;;;;;;;6387:13:60;;;;;;:9;:13;;;;;;;;:23;;;;;;6436:32;160:25:238;;;-1:-1:-1;;;;;;;;;;;6436:32:60;133:18:238;6436:32:60;;;;;;;;6150:325;;:::o;4610:211:99:-;-1:-1:-1;;;;;4710:32:99;;;;;;:22;:32;;;;;4745:15;4710:50;;4770:44;4789:6;4797;4733:8;4770:18;:44::i;6852:325:50:-;6993:12;7018;7032:23;7059:6;-1:-1:-1;;;;;7059:19:50;7079:4;7059:25;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;7017:67;;;;7101:69;7128:6;7136:7;7145:10;7157:12;7101:26;:69::i;4895:219:99:-;5017:27;5038:5;5017:20;:27::i;:::-;5054:53;5075:6;5083;5091:8;5101:5;5054:20;:53::i;6481:328:60:-;-1:-1:-1;;;;;6553:15:60;;;;;;:9;:15;;;;;:25;;6572:6;;6553:15;:25;;6572:6;;6553:25;:::i;:::-;;;;-1:-1:-1;;6721:11:60;:21;;;;;;;6768:34;;160:25:238;;;-1:-1:-1;;;;;;;6768:34:60;;;-1:-1:-1;;;;;;;;;;;6768:34:60;148:2:238;133:18;6768:34:60;14:177:238;33644:2532:92;33798:34;-1:-1:-1;;;;;;;;;;;;;;;;;;;;;;;;;;;;;33798:34:92;33870:11;;:32;;-1:-1:-1;;;33870:32:92;;-1:-1:-1;;;;;33896:5:92;6445:32:238;;33870::92;;;6427:51:238;33870:11:92;;;;:25;;6400:18:238;;33870:32:92;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;33842:25;;;:60;33935:14;33941:8;33935:2;:14;:::i;:::-;33912:20;;;:37;33982:15;:22;33959:20;34014:2026;34034:12;34030:1;:16;34014:2026;;;34067:15;34085;34101:1;34085:18;;;;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;;;-1:-1:-1;34147:27:92;34085:18;34147:17;:27::i;:::-;34117:57;;34254:19;34277:1;34254:24;34250:38;;34280:8;;;;34250:38;34302:19;34324:18;34333:8;34324;:18::i;:::-;34384:11;;:40;;-1:-1:-1;;;34384:40:92;;-1:-1:-1;;;;;6445:32:238;;;34384:40:92;;;6427:51:238;34302:40:92;;-1:-1:-1;34384:11:92;;;:25;;6400:18:238;;34384:40:92;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;34357:11;:24;;:67;;;;;34466:13;-1:-1:-1;;;;;34466:22:92;;:24;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;:::i;:::-;34460:30;;:2;:30;:::i;:::-;34438:19;;;:52;;;34688:24;;34504:40;;;;34611:160;;34612:42;34635:19;33329:4;34612:42;:::i;:::-;34611:55;:160;:55;:160::i;:::-;34576:195;;34824:142;34881:11;:20;;;34923:11;:25;;;34824:24;:35;;:142;;;;;:::i;:::-;34789:177;-1:-1:-1;35019:55:92;33329:4;34789:177;35019:55;:::i;:::-;34984:90;;34558:531;35197:14;35265:6;35230:32;:41;35226:621;;;35346:19;35368:149;35432:11;:25;;;35479:11;:20;;;35392:6;33329:4;35369:29;;;;:::i;35368:149::-;35567:19;;;;35588:24;;35346:171;;-1:-1:-1;35544:69:92;;35346:171;;35567:19;35544:22;:69::i;:::-;35535:78;-1:-1:-1;35640:29:92;33329:4;35535:78;35640:29;:::i;:::-;35631:38;;35696:1;35687:10;;35273:439;35226:621;;;-1:-1:-1;35745:19:92;35791:41;35800:32;35791:6;:41;:::i;:::-;35782:50;;35226:621;35900:41;35914:8;35924:6;35932:8;35900:13;:41::i;:::-;36011:6;36021:1;36011:11;36007:22;;36024:5;;;;;;;36007:22;34053:1987;;;;;34014:2026;34048:3;;;:::i;:::-;;;34014:2026;;;-1:-1:-1;36116:10:92;;36112:57;;36135:34;;-1:-1:-1;;;36135:34:92;;;;;160:25:238;;;133:18;;36135:34:92;14:177:238;43778:641:92;43857:14;43883:16;:14;:16::i;:::-;43955:20;43977;44001:36;44031:5;44001:29;:36::i;:::-;-1:-1:-1;;;;;44081:16:92;;44047:14;44081:16;;;:9;:16;;;;;;43954:83;;-1:-1:-1;43954:83:92;;-1:-1:-1;44047:14:92;44064:62;;43954:83;;44064:16;:62::i;:::-;44047:79;;44137:20;44160:52;44207:4;44160:46;:52::i;:::-;44137:75;;44241:12;44231:6;:22;;:46;;44265:12;44231:46;;;44256:6;44231:46;44222:55;;44292:8;44288:75;;;44311:52;44328:6;44336:12;44350;44311:16;:52::i;:::-;44302:61;;44288:75;43873:546;;;;43778:641;;;;:::o;26575:133::-;26657:18;:16;:18::i;:::-;26685:16;:14;:16::i;26832:125::-;26926:15;;26915:35;;-1:-1:-1;;;26926:15:92;;;;26943:6;26915:10;:35::i;7465:628:50:-;7645:12;7673:7;7669:418;;;7700:10;:17;7721:1;7700:22;7696:286;;-1:-1:-1;;;;;1465:19:50;;;7907:60;;;;-1:-1:-1;;;7907:60:50;;39416:2:238;7907:60:50;;;39398:21:238;39455:2;39435:18;;;39428:30;39494:31;39474:18;;;39467:59;39543:18;;7907:60:50;39214:353:238;7907:60:50;-1:-1:-1;8002:10:50;7995:17;;7669:418;8043:33;8051:10;8063:12;8043:7;:33::i;27031:115:92:-;27123:16;:14;:16::i;57284:462::-;57395:25;;;57377:15;57395:25;;;:15;:25;;;;;;;:33;;57480:249;;-1:-1:-1;;;;;57395:33:92;;;;57438:301;;-1:-1:-1;;;57520:29:92;57480:249;;57567:6;;57591:8;;57395:33;57617:37;;;57672:43;;;;;57480:249;;;:::i;:::-;;;;-1:-1:-1;;57480:249:92;;;;;;;;;;;;;;-1:-1:-1;;;;;57480:249:92;-1:-1:-1;;;;;;57480:249:92;;;;;;;;;;-1:-1:-1;;;;;57438:28:92;;;;:301::i;56565:414::-;56655:25;;;56637:15;56655:25;;;:15;:25;;;;;;;:33;;56740:222;;-1:-1:-1;;;;;56655:33:92;;;;56698:274;;-1:-1:-1;;;56780:28:92;56740:222;;56826:6;;56655:33;56850:37;;;56905:43;;;56740:222;;;:::i;8615:540:50:-;8774:17;;:21;8770:379;;9002:10;8996:17;9058:15;9045:10;9041:2;9037:19;9030:44;8770:379;9125:12;9118:20;;-1:-1:-1;;;9118:20:50;;;;;;;;:::i;-1:-1:-1:-;;;;;;;:::i;:::-;;;;;;;;;;;:::o;:::-;;;;;;;;;;;;;;;;;;;;:::i;:::-;;:::o;:::-;;;;;;;;;;;;;;;;;;:::o;196:250:238:-;281:1;291:113;305:6;302:1;299:13;291:113;;;381:11;;;375:18;362:11;;;355:39;327:2;320:10;291:113;;;-1:-1:-1;;438:1:238;420:16;;413:27;196:250::o;451:271::-;493:3;531:5;525:12;558:6;553:3;546:19;574:76;643:6;636:4;631:3;627:14;620:4;613:5;609:16;574:76;:::i;:::-;704:2;683:15;-1:-1:-1;;679:29:238;670:39;;;;711:4;666:50;;451:271;-1:-1:-1;;451:271:238:o;727:220::-;876:2;865:9;858:21;839:4;896:45;937:2;926:9;922:18;914:6;896:45;:::i;952:163::-;1019:20;;1079:10;1068:22;;1058:33;;1048:61;;1105:1;1102;1095:12;1048:61;952:163;;;:::o;1120:184::-;1178:6;1231:2;1219:9;1210:7;1206:23;1202:32;1199:52;;;1247:1;1244;1237:12;1199:52;1270:28;1288:9;1270:28;:::i;1309:180::-;1368:6;1421:2;1409:9;1400:7;1396:23;1392:32;1389:52;;;1437:1;1434;1427:12;1389:52;-1:-1:-1;1460:23:238;;1309:180;-1:-1:-1;1309:180:238:o;1494:131::-;-1:-1:-1;;;;;1569:31:238;;1559:42;;1549:70;;1615:1;1612;1605:12;1630:315;1698:6;1706;1759:2;1747:9;1738:7;1734:23;1730:32;1727:52;;;1775:1;1772;1765:12;1727:52;1814:9;1801:23;1833:31;1858:5;1833:31;:::i;:::-;1883:5;1935:2;1920:18;;;;1907:32;;-1:-1:-1;;;1630:315:238:o;2142:127::-;2203:10;2198:3;2194:20;2191:1;2184:31;2234:4;2231:1;2224:15;2258:4;2255:1;2248:15;2274:257;2346:4;2340:11;;;2378:17;;-1:-1:-1;;;;;2410:34:238;;2446:22;;;2407:62;2404:88;;;2472:18;;:::i;:::-;2508:4;2501:24;2274:257;:::o;2536:275::-;2607:2;2601:9;2672:2;2653:13;;-1:-1:-1;;2649:27:238;2637:40;;-1:-1:-1;;;;;2692:34:238;;2728:22;;;2689:62;2686:88;;;2754:18;;:::i;:::-;2790:2;2783:22;2536:275;;-1:-1:-1;2536:275:238:o;2816:186::-;2864:4;-1:-1:-1;;;;;2889:6:238;2886:30;2883:56;;;2919:18;;:::i;:::-;-1:-1:-1;2985:2:238;2964:15;-1:-1:-1;;2960:29:238;2991:4;2956:40;;2816:186::o;3007:462::-;3049:5;3102:3;3095:4;3087:6;3083:17;3079:27;3069:55;;3120:1;3117;3110:12;3069:55;3156:6;3143:20;3187:48;3203:31;3231:2;3203:31;:::i;:::-;3187:48;:::i;:::-;3260:2;3251:7;3244:19;3306:3;3299:4;3294:2;3286:6;3282:15;3278:26;3275:35;3272:55;;;3323:1;3320;3313:12;3272:55;3388:2;3381:4;3373:6;3369:17;3362:4;3353:7;3349:18;3336:55;3436:1;3411:16;;;3429:4;3407:27;3400:38;;;;3415:7;3007:462;-1:-1:-1;;;3007:462:238:o;3474:665::-;3569:6;3577;3585;3593;3646:3;3634:9;3625:7;3621:23;3617:33;3614:53;;;3663:1;3660;3653:12;3614:53;3702:9;3689:23;3721:31;3746:5;3721:31;:::i;:::-;3771:5;-1:-1:-1;3828:2:238;3813:18;;3800:32;3841:33;3800:32;3841:33;:::i;:::-;3893:7;-1:-1:-1;3947:2:238;3932:18;;3919:32;;-1:-1:-1;4002:2:238;3987:18;;3974:32;-1:-1:-1;;;;;4018:30:238;;4015:50;;;4061:1;4058;4051:12;4015:50;4084:49;4125:7;4116:6;4105:9;4101:22;4084:49;:::i;:::-;4074:59;;;3474:665;;;;;;;:::o;4351:247::-;4410:6;4463:2;4451:9;4442:7;4438:23;4434:32;4431:52;;;4479:1;4476;4469:12;4431:52;4518:9;4505:23;4537:31;4562:5;4537:31;:::i;4603:456::-;4680:6;4688;4696;4749:2;4737:9;4728:7;4724:23;4720:32;4717:52;;;4765:1;4762;4755:12;4717:52;4804:9;4791:23;4823:31;4848:5;4823:31;:::i;:::-;4873:5;-1:-1:-1;4930:2:238;4915:18;;4902:32;4943:33;4902:32;4943:33;:::i;:::-;4603:456;;4995:7;;-1:-1:-1;;;5049:2:238;5034:18;;;;5021:32;;4603:456::o;5253:118::-;5339:5;5332:13;5325:21;5318:5;5315:32;5305:60;;5361:1;5358;5351:12;5376:313;5440:6;5448;5501:2;5489:9;5480:7;5476:23;5472:32;5469:52;;;5517:1;5514;5507:12;5469:52;5540:28;5558:9;5540:28;:::i;:::-;5530:38;;5618:2;5607:9;5603:18;5590:32;5631:28;5653:5;5631:28;:::i;:::-;5678:5;5668:15;;;5376:313;;;;;:::o;5876:385::-;5948:6;5956;5964;6017:2;6005:9;5996:7;5992:23;5988:32;5985:52;;;6033:1;6030;6023:12;5985:52;6056:28;6074:9;6056:28;:::i;:::-;6046:38;;6103:37;6136:2;6125:9;6121:18;6103:37;:::i;:::-;6093:47;;6190:2;6179:9;6175:18;6162:32;6203:28;6225:5;6203:28;:::i;:::-;6250:5;6240:15;;;5876:385;;;;;:::o;6694:647::-;6863:2;6915:21;;;6985:13;;6888:18;;;7007:22;;;6834:4;;6863:2;7086:15;;;;7060:2;7045:18;;;6834:4;7129:186;7143:6;7140:1;7137:13;7129:186;;;7208:13;;7223:10;7204:30;7192:43;;7290:15;;;;7255:12;;;;7165:1;7158:9;7129:186;;;-1:-1:-1;7332:3:238;;6694:647;-1:-1:-1;;;;;;6694:647:238:o;7346:387::-;7429:8;7439:6;7493:3;7486:4;7478:6;7474:17;7470:27;7460:55;;7511:1;7508;7501:12;7460:55;-1:-1:-1;7534:20:238;;-1:-1:-1;;;;;7566:30:238;;7563:50;;;7609:1;7606;7599:12;7563:50;7646:4;7638:6;7634:17;7622:29;;7706:3;7699:4;7689:6;7686:1;7682:14;7674:6;7670:27;7666:38;7663:47;7660:67;;;7723:1;7720;7713:12;7660:67;7346:387;;;;;:::o;7738:489::-;7856:6;7864;7917:2;7905:9;7896:7;7892:23;7888:32;7885:52;;;7933:1;7930;7923:12;7885:52;7973:9;7960:23;-1:-1:-1;;;;;7998:6:238;7995:30;7992:50;;;8038:1;8035;8028:12;7992:50;8077:90;8159:7;8150:6;8139:9;8135:22;8077:90;:::i;:::-;8186:8;;8051:116;;-1:-1:-1;7738:489:238;-1:-1:-1;;;;7738:489:238:o;8232:286::-;8291:6;8344:2;8332:9;8323:7;8319:23;8315:32;8312:52;;;8360:1;8357;8350:12;8312:52;8386:23;;-1:-1:-1;;;;;8438:31:238;;8428:42;;8418:70;;8484:1;8481;8474:12;8720:315;8788:6;8796;8849:2;8837:9;8828:7;8824:23;8820:32;8817:52;;;8865:1;8862;8855:12;8817:52;8901:9;8888:23;8878:33;;8961:2;8950:9;8946:18;8933:32;8974:31;8999:5;8974:31;:::i;9040:559::-;-1:-1:-1;;;;;9283:32:238;;9265:51;;9359:14;;9352:22;9347:2;9332:18;;9325:50;9411:3;9406:2;9391:18;;9384:31;;;-1:-1:-1;;9438:46:238;;9464:19;;9456:6;9438:46;:::i;:::-;9532:9;9524:6;9520:22;9515:2;9504:9;9500:18;9493:50;9560:33;9586:6;9578;9560:33;:::i;:::-;9552:41;9040:559;-1:-1:-1;;;;;;;9040:559:238:o;9604:468::-;9664:3;9702:5;9696:12;9729:6;9724:3;9717:19;9755:4;9784:2;9779:3;9775:12;9768:19;;9821:2;9814:5;9810:14;9842:1;9852:195;9866:6;9863:1;9860:13;9852:195;;;9931:13;;-1:-1:-1;;;;;9927:39:238;9915:52;;9987:12;;;;10022:15;;;;9963:1;9881:9;9852:195;;;-1:-1:-1;10063:3:238;;9604:468;-1:-1:-1;;;;;9604:468:238:o;10077:435::-;10130:3;10168:5;10162:12;10195:6;10190:3;10183:19;10221:4;10250:2;10245:3;10241:12;10234:19;;10287:2;10280:5;10276:14;10308:1;10318:169;10332:6;10329:1;10326:13;10318:169;;;10393:13;;10381:26;;10427:12;;;;10462:15;;;;10354:1;10347:9;10318:169;;10517:1040;10861:2;10850:9;10843:21;10824:4;10887:63;10946:2;10935:9;10931:18;10923:6;10887:63;:::i;:::-;10969:2;11019:9;11011:6;11007:22;11002:2;10991:9;10987:18;10980:50;11053:44;11090:6;11082;11053:44;:::i;:::-;11133:22;;;11128:2;11113:18;;11106:50;11205:13;;11227:22;;;11303:15;;;;-1:-1:-1;11265:15:238;;;;11336:1;11346:185;11360:6;11357:1;11354:13;11346:185;;;11435:13;;11428:21;11421:29;11409:42;;11506:15;;;;11471:12;;;;11382:1;11375:9;11346:185;;;-1:-1:-1;11548:3:238;;10517:1040;-1:-1:-1;;;;;;;;10517:1040:238:o;11996:594::-;12086:6;12094;12102;12110;12163:3;12151:9;12142:7;12138:23;12134:33;12131:53;;;12180:1;12177;12170:12;12131:53;12203:28;12221:9;12203:28;:::i;:::-;12193:38;;12250:37;12283:2;12272:9;12268:18;12250:37;:::i;:::-;12240:47;;12338:2;12327:9;12323:18;12310:32;-1:-1:-1;;;;;12357:6:238;12354:30;12351:50;;;12397:1;12394;12387:12;12351:50;12420:49;12461:7;12452:6;12441:9;12437:22;12420:49;:::i;:::-;12410:59;;;12519:2;12508:9;12504:18;12491:32;12532:28;12554:5;12532:28;:::i;:::-;11996:594;;;;-1:-1:-1;11996:594:238;;-1:-1:-1;;11996:594:238:o;13068:456::-;13145:6;13153;13161;13214:2;13202:9;13193:7;13189:23;13185:32;13182:52;;;13230:1;13227;13220:12;13182:52;13266:9;13253:23;13243:33;;13326:2;13315:9;13311:18;13298:32;13339:31;13364:5;13339:31;:::i;:::-;13389:5;-1:-1:-1;13446:2:238;13431:18;;13418:32;13459:33;13418:32;13459:33;:::i;13529:284::-;13587:6;13640:2;13628:9;13619:7;13615:23;13611:32;13608:52;;;13656:1;13653;13646:12;13608:52;13695:9;13682:23;-1:-1:-1;;;;;13738:5:238;13734:30;13727:5;13724:41;13714:69;;13779:1;13776;13769:12;13818:550;13891:6;13899;13907;13960:2;13948:9;13939:7;13935:23;13931:32;13928:52;;;13976:1;13973;13966:12;13928:52;14015:9;14002:23;14034:28;14056:5;14034:28;:::i;:::-;14081:5;-1:-1:-1;14138:2:238;14123:18;;14110:32;14186:6;14173:20;;14161:33;;14151:61;;14208:1;14205;14198:12;14581:114;14665:4;14658:5;14654:16;14647:5;14644:27;14634:55;;14685:1;14682;14675:12;14700:801;14811:6;14819;14827;14835;14843;14851;14859;14912:3;14900:9;14891:7;14887:23;14883:33;14880:53;;;14929:1;14926;14919:12;14880:53;14968:9;14955:23;14987:31;15012:5;14987:31;:::i;:::-;15037:5;-1:-1:-1;15094:2:238;15079:18;;15066:32;15107:33;15066:32;15107:33;:::i;:::-;15159:7;-1:-1:-1;15213:2:238;15198:18;;15185:32;;-1:-1:-1;15264:2:238;15249:18;;15236:32;;-1:-1:-1;15320:3:238;15305:19;;15292:33;15334:31;15292:33;15334:31;:::i;:::-;14700:801;;;;-1:-1:-1;14700:801:238;;;;15384:7;15438:3;15423:19;;15410:33;;-1:-1:-1;15490:3:238;15475:19;;;15462:33;;14700:801;-1:-1:-1;;14700:801:238:o;15735:388::-;15803:6;15811;15864:2;15852:9;15843:7;15839:23;15835:32;15832:52;;;15880:1;15877;15870:12;15832:52;15919:9;15906:23;15938:31;15963:5;15938:31;:::i;:::-;15988:5;-1:-1:-1;16045:2:238;16030:18;;16017:32;16058:33;16017:32;16058:33;:::i;16608:334::-;16810:2;16792:21;;;16849:2;16829:18;;;16822:30;-1:-1:-1;;;16883:2:238;16868:18;;16861:40;16933:2;16918:18;;16608:334::o;16947:380::-;17026:1;17022:12;;;;17069;;;17090:61;;17144:4;17136:6;17132:17;17122:27;;17090:61;17197:2;17189:6;17186:14;17166:18;17163:38;17160:161;;17243:10;17238:3;17234:20;17231:1;17224:31;17278:4;17275:1;17268:15;17306:4;17303:1;17296:15;17160:161;;16947:380;;;:::o;17332:336::-;17534:2;17516:21;;;17573:2;17553:18;;;17546:30;-1:-1:-1;;;17607:2:238;17592:18;;17585:42;17659:2;17644:18;;17332:336::o;17982:127::-;18043:10;18038:3;18034:20;18031:1;18024:31;18074:4;18071:1;18064:15;18098:4;18095:1;18088:15;19126:127;19187:10;19182:3;19178:20;19175:1;19168:31;19218:4;19215:1;19208:15;19242:4;19239:1;19232:15;19258:128;19325:9;;;19346:11;;;19343:37;;;19360:18;;:::i;19391:125::-;19456:9;;;19477:10;;;19474:36;;;19490:18;;:::i;19521:194::-;19592:4;-1:-1:-1;;;;;19617:6:238;19614:30;19611:56;;;19647:18;;:::i;:::-;-1:-1:-1;19692:1:238;19688:14;19704:4;19684:25;;19521:194::o;19720:2587::-;19898:9;19933:75;19949:58;20000:6;19949:58;:::i;19933:75::-;20042:19;;;20080:4;20100:12;;;;20030:3;20131:1;20166:15;;;20155:27;;20205:14;20194:26;;20191:46;;;20233:1;20230;20223:12;20191:46;20257:5;20271:2003;20287:6;20282:3;20279:15;20271:2003;;;20373:3;20360:17;-1:-1:-1;;;;;20450:2:238;20437:11;20434:19;20431:109;;;20494:1;20523:2;20519;20512:14;20431:109;20574:11;20567:5;20563:23;20553:33;;20631:4;20626:2;20610:14;20606:23;20602:34;20599:124;;;20677:1;20706:2;20702;20695:14;20599:124;20751:22;;:::i;:::-;20814:2;20801:16;20830:33;20855:7;20830:33;:::i;:::-;20876:24;;20940:11;;;20927:25;20968:14;;;20965:104;;;21023:1;21052:2;21048;21041:14;20965:104;21092:15;;;;;21149:14;21142:4;21134:13;;21130:34;21120:132;;21206:1;21235:2;21231;21224:14;21120:132;21289:2;21276:16;21318:72;21334:55;21385:3;21334:55;:::i;21318:72::-;21434:18;;;21529:12;;;21521:21;;21517:30;;;21474:14;;;;21576;21563:28;;21560:121;;;21633:1;21663:3;21658;21651:16;21560:121;21715:2;21711;21707:11;21731:424;21749:8;21742:5;21739:19;21731:424;;;21851:5;21838:19;21895:2;21880:13;21877:21;21874:130;;;21948:1;21982:3;21977;21970:16;21874:130;22035:65;22085:14;22080:2;22064:13;22060:2;22056:22;22052:31;22035:65;:::i;:::-;22021:80;;-1:-1:-1;22127:14:238;;;;21770;;21731:424;;;-1:-1:-1;22175:16:238;;;22168:31;-1:-1:-1;;22212:20:238;;-1:-1:-1;;22252:12:238;;;;20304;;20271:2003;;;-1:-1:-1;22296:5:238;;19720:2587;-1:-1:-1;;;;;;;19720:2587:238:o;23444:135::-;23483:3;23504:17;;;23501:43;;23524:18;;:::i;:::-;-1:-1:-1;23571:1:238;23560:13;;23444:135::o;23584:903::-;23678:6;23686;23694;23747:2;23735:9;23726:7;23722:23;23718:32;23715:52;;;23763:1;23760;23753:12;23715:52;23795:9;23789:16;23814:31;23839:5;23814:31;:::i;:::-;23914:2;23899:18;;23893:25;23864:5;;-1:-1:-1;23927:30:238;23893:25;23927:30;:::i;:::-;24027:2;24012:18;;24006:25;23976:7;;-1:-1:-1;;;;;;24043:30:238;;24040:50;;;24086:1;24083;24076:12;24040:50;24109:22;;24162:4;24154:13;;24150:27;-1:-1:-1;24140:55:238;;24191:1;24188;24181:12;24140:55;24220:2;24214:9;24245:48;24261:31;24289:2;24261:31;:::i;24245:48::-;24316:2;24309:5;24302:17;24356:7;24351:2;24346;24342;24338:11;24334:20;24331:33;24328:53;;;24377:1;24374;24367:12;24328:53;24390:67;24454:2;24449;24442:5;24438:14;24433:2;24429;24425:11;24390:67;:::i;:::-;24476:5;24466:15;;;;;23584:903;;;;;:::o;24617:544::-;24718:2;24713:3;24710:11;24707:448;;;24754:1;24779:5;24775:2;24768:17;24824:4;24820:2;24810:19;24894:2;24882:10;24878:19;24875:1;24871:27;24865:4;24861:38;24930:4;24918:10;24915:20;24912:47;;;-1:-1:-1;24953:4:238;24912:47;25008:2;25003:3;24999:12;24996:1;24992:20;24986:4;24982:31;24972:41;;25063:82;25081:2;25074:5;25071:13;25063:82;;;25126:17;;;25107:1;25096:13;25063:82;;25337:1348;25461:3;25455:10;-1:-1:-1;;;;;25480:6:238;25477:30;25474:56;;;25510:18;;:::i;:::-;25539:96;25628:6;25588:38;25620:4;25614:11;25588:38;:::i;:::-;25582:4;25539:96;:::i;:::-;25690:4;;25754:2;25743:14;;25771:1;25766:662;;;;26472:1;26489:6;26486:89;;;-1:-1:-1;26541:19:238;;;26535:26;26486:89;-1:-1:-1;;25294:1:238;25290:11;;;25286:24;25282:29;25272:40;25318:1;25314:11;;;25269:57;26588:81;;25736:943;;25766:662;24564:1;24557:14;;;24601:4;24588:18;;-1:-1:-1;;25802:20:238;;;25919:236;25933:7;25930:1;25927:14;25919:236;;;26022:19;;;26016:26;26001:42;;26114:27;;;;26082:1;26070:14;;;;25949:19;;25919:236;;;25923:3;26183:6;26174:7;26171:19;26168:201;;;26244:19;;;26238:26;-1:-1:-1;;26327:1:238;26323:14;;;26339:3;26319:24;26315:37;26311:42;26296:58;26281:74;;26168:201;-1:-1:-1;;;;;26415:1:238;26399:14;;;26395:22;26382:36;;-1:-1:-1;25337:1348:238:o;26987:245::-;27054:6;27107:2;27095:9;27086:7;27082:23;27078:32;27075:52;;;27123:1;27120;27113:12;27075:52;27155:9;27149:16;27174:28;27196:5;27174:28;:::i;27237:521::-;27314:4;27320:6;27380:11;27367:25;27474:2;27470:7;27459:8;27443:14;27439:29;27435:43;27415:18;27411:68;27401:96;;27493:1;27490;27483:12;27401:96;27520:33;;27572:20;;;-1:-1:-1;;;;;;27604:30:238;;27601:50;;;27647:1;27644;27637:12;27601:50;27680:4;27668:17;;-1:-1:-1;27711:14:238;27707:27;;;27697:38;;27694:58;;;27748:1;27745;27738:12;28067:171;28135:6;28174:10;;;28162;;;28158:27;;28197:12;;;28194:38;;;28212:18;;:::i;28243:168::-;28310:6;28336:10;;;28348;;;28332:27;;28371:11;;;28368:37;;;28385:18;;:::i;30507:599::-;30822:2;30811:9;30804:21;30785:4;30848:63;30907:2;30896:9;30892:18;30884:6;30848:63;:::i;:::-;30959:9;30951:6;30947:22;30942:2;30931:9;30927:18;30920:50;30987:44;31024:6;31016;30987:44;:::i;:::-;30979:52;;;31096:1;31092;31087:3;31083:11;31079:19;31071:6;31067:32;31062:2;31051:9;31047:18;31040:60;30507:599;;;;;;:::o;31111:184::-;31181:6;31234:2;31222:9;31213:7;31209:23;31205:32;31202:52;;;31250:1;31247;31240:12;31202:52;-1:-1:-1;31273:16:238;;31111:184;-1:-1:-1;31111:184:238:o;31300:1032::-;31786:3;31775:9;31768:22;31749:4;31813:64;31872:3;31861:9;31857:19;31849:6;31813:64;:::i;:::-;31925:9;31917:6;31913:22;31908:2;31897:9;31893:18;31886:50;31959:44;31996:6;31988;31959:44;:::i;:::-;31945:58;;32051:9;32043:6;32039:22;32034:2;32023:9;32019:18;32012:50;32085:51;32129:6;32121;32085:51;:::i;:::-;32071:65;;32184:9;32176:6;32172:22;32167:2;32156:9;32152:18;32145:50;32212:44;32249:6;32241;32212:44;:::i;:::-;32204:52;;;32322:1;32318;32313:3;32309:11;32305:19;32297:6;32293:32;32287:3;32276:9;32272:19;32265:61;31300:1032;;;;;;;;:::o;32337:771::-;32386:3;32427:5;32421:12;32456:36;32482:9;32456:36;:::i;:::-;32501:19;;;32539:4;32562:1;32579:18;;;32606:146;;;;32766:1;32761:341;;;;32572:530;;32606:146;-1:-1:-1;;32648:24:238;;32634:12;;;32627:46;32720:14;;32713:22;32710:1;32706:30;32697:40;;32693:49;;;-1:-1:-1;32606:146:238;;32761:341;32792:5;32789:1;32782:16;32839:2;32836:1;32826:16;32864:1;32878:174;32892:6;32889:1;32886:13;32878:174;;;32979:14;;32961:11;;;32957:20;;32950:44;33022:16;;;;32907:10;;32878:174;;;33076:11;;33072:20;;;-1:-1:-1;;32572:530:238;;;;;;32337:771;;;;:::o;33113:222::-;33257:2;33246:9;33239:21;33220:4;33277:52;33325:2;33314:9;33310:18;33302:6;33277:52;:::i;33340:266::-;33425:6;33478:2;33466:9;33457:7;33453:23;33449:32;33446:52;;;33494:1;33491;33484:12;33446:52;33526:9;33520:16;33545:31;33570:5;33545:31;:::i;33611:844::-;33741:3;33770:1;33803:6;33797:13;33833:36;33859:9;33833:36;:::i;:::-;33888:1;33905:18;;;33932:133;;;;34079:1;34074:356;;;;33898:532;;33932:133;-1:-1:-1;;33965:24:238;;33953:37;;34038:14;;34031:22;34019:35;;34010:45;;;-1:-1:-1;33932:133:238;;34074:356;34105:6;34102:1;34095:17;34135:4;34180:2;34177:1;34167:16;34205:1;34219:165;34233:6;34230:1;34227:13;34219:165;;;34311:14;;34298:11;;;34291:35;34354:16;;;;34248:10;;34219:165;;;34223:3;;;34413:6;34408:3;34404:16;34397:23;;33898:532;-1:-1:-1;34446:3:238;;33611:844;-1:-1:-1;;;;;;33611:844:238:o;34954:315::-;-1:-1:-1;;;;;35129:32:238;;35111:51;;35198:2;35193;35178:18;;35171:30;;;-1:-1:-1;;35218:45:238;;35244:18;;35236:6;35218:45;:::i;35274:136::-;35313:3;35341:5;35331:39;;35350:18;;:::i;:::-;-1:-1:-1;;;35386:18:238;;35274:136::o;35671:387::-;35858:2;35847:9;35840:21;35821:4;35884:52;35932:2;35921:9;35917:18;35909:6;35884:52;:::i;:::-;35984:9;35976:6;35972:22;35967:2;35956:9;35952:18;35945:50;36012:40;36045:6;36037;36012:40;:::i;:::-;36004:48;35671:387;-1:-1:-1;;;;;35671:387:238:o;36411:127::-;36472:10;36467:3;36463:20;36460:1;36453:31;36503:4;36500:1;36493:15;36527:4;36524:1;36517:15;36892:287;37021:3;37059:6;37053:13;37075:66;37134:6;37129:3;37122:4;37114:6;37110:17;37075:66;:::i;:::-;37157:16;;;;;36892:287;-1:-1:-1;;36892:287:238:o;37184:422::-;37273:1;37316:5;37273:1;37330:270;37351:7;37341:8;37338:21;37330:270;;;37410:4;37406:1;37402:6;37398:17;37392:4;37389:27;37386:53;;;37419:18;;:::i;:::-;37469:7;37459:8;37455:22;37452:55;;;37489:16;;;;37452:55;37568:22;;;;37528:15;;;;37330:270;;;37334:3;37184:422;;;;;:::o;37611:806::-;37660:5;37690:8;37680:80;;-1:-1:-1;37731:1:238;37745:5;;37680:80;37779:4;37769:76;;-1:-1:-1;37816:1:238;37830:5;;37769:76;37861:4;37879:1;37874:59;;;;37947:1;37942:130;;;;37854:218;;37874:59;37904:1;37895:10;;37918:5;;;37942:130;37979:3;37969:8;37966:17;37963:43;;;37986:18;;:::i;:::-;-1:-1:-1;;38042:1:238;38028:16;;38057:5;;37854:218;;38156:2;38146:8;38143:16;38137:3;38131:4;38128:13;38124:36;38118:2;38108:8;38105:16;38100:2;38094:4;38091:12;38087:35;38084:77;38081:159;;;-1:-1:-1;38193:19:238;;;38225:5;;38081:159;38272:34;38297:8;38291:4;38272:34;:::i;:::-;38342:6;38338:1;38334:6;38330:19;38321:7;38318:32;38315:58;;;38353:18;;:::i;:::-;38391:20;;37611:806;-1:-1:-1;;;37611:806:238:o;38422:140::-;38480:5;38509:47;38550:4;38540:8;38536:19;38530:4;38509:47;:::i;38567:247::-;38635:6;38688:2;38676:9;38667:7;38663:23;38659:32;38656:52;;;38704:1;38701;38694:12;38656:52;38736:9;38730:16;38755:29;38778:5;38755:29;:::i;38819:168::-;38892:9;;;38923;;38940:15;;;38934:22;;38920:37;38910:71;;38961:18;;:::i;38992:217::-;39032:1;39058;39048:132;;39102:10;39097:3;39093:20;39090:1;39083:31;39137:4;39134:1;39127:15;39165:4;39162:1;39155:15;39048:132;-1:-1:-1;39194:9:238;;38992:217::o;39572:557::-;39797:25;;;-1:-1:-1;;;;;39858:32:238;;39853:2;39838:18;;39831:60;39927:3;39922:2;39907:18;;39900:31;;;-1:-1:-1;;39954:53:238;;39987:19;;39979:6;39954:53;:::i;:::-;40055:9;40047:6;40043:22;40038:2;40027:9;40023:18;40016:50;40083:40;40116:6;40108;40083:40;:::i;40134:458::-;40349:6;40338:9;40331:25;40392:2;40387;40376:9;40372:18;40365:30;40312:4;40418:52;40466:2;40455:9;40451:18;40443:6;40418:52;:::i;:::-;40518:9;40510:6;40506:22;40501:2;40490:9;40486:18;40479:50;40546:40;40579:6;40571;40546:40;:::i\",\n    \"linkReferences\": {},\n    \"immutableReferences\": {\n      \"50264\": [\n        {\n          \"start\": 1606,\n          \"length\": 32\n        },\n        {\n          \"start\": 3404,\n          \"length\": 32\n        },\n        {\n          \"start\": 3469,\n          \"length\": 32\n        },\n        {\n          \"start\": 13065,\n          \"length\": 32\n        },\n        {\n          \"start\": 13770,\n          \"length\": 32\n        },\n        {\n          \"start\": 15436,\n          \"length\": 32\n        },\n        {\n          \"start\": 18006,\n          \"length\": 32\n        }\n      ],\n      \"50767\": [\n        {\n          \"start\": 1503,\n          \"length\": 32\n        },\n        {\n          \"start\": 18132,\n          \"length\": 32\n        }\n      ],\n      \"50781\": [\n        {\n          \"start\": 4247,\n          \"length\": 32\n        }\n      ],\n      \"50783\": [\n        {\n          \"start\": 4295,\n          \"length\": 32\n        }\n      ],\n      \"56665\": [\n        {\n          \"start\": 2102,\n          \"length\": 32\n        },\n        {\n          \"start\": 5130,\n          \"length\": 32\n        },\n        {\n          \"start\": 6102,\n          \"length\": 32\n        },\n        {\n          \"start\": 8723,\n          \"length\": 32\n        },\n        {\n          \"start\": 9643,\n          \"length\": 32\n        },\n        {\n          \"start\": 10401,\n          \"length\": 32\n        },\n        {\n          \"start\": 12522,\n          \"length\": 32\n        },\n        {\n          \"start\": 16465,\n          \"length\": 32\n        },\n        {\n          \"start\": 17296,\n          \"length\": 32\n        }\n      ]\n    }\n  },\n  \"methodIdentifiers\": {\n    \"DOMAIN_SEPARATOR()\": \"3644e515\",\n    \"GRAVITY_BRIDGE_REGISTRY_SLOT()\": \"cd82f8b1\",\n    \"MAXIMUM_SHARE_LOCK_PERIOD()\": \"0402ab63\",\n    \"MAX_FEE_CUT()\": \"eef33eca\",\n    \"MAX_PLATFORM_FEE()\": \"3998a681\",\n    \"MAX_POSITIONS()\": \"f7b24e08\",\n    \"MAX_REBALANCE_DEVIATION()\": \"6ff1c02a\",\n    \"MINIMUM_SHARE_LOCK_PERIOD()\": \"0051a3b7\",\n    \"PRICE_ROUTER_REGISTRY_SLOT()\": \"5a400d25\",\n    \"adaptorCatalogue(address)\": \"18d4c143\",\n    \"addAdaptorToCatalogue(address)\": \"3d8ab1e5\",\n    \"addPosition(uint32,uint32,bytes,bool)\": \"9955a9d4\",\n    \"addPositionToCatalogue(uint32)\": \"501eb4fe\",\n    \"allowance(address,address)\": \"dd62ed3e\",\n    \"allowedRebalanceDeviation()\": \"c244245a\",\n    \"approve(address,uint256)\": \"095ea7b3\",\n    \"asset()\": \"38d52e0f\",\n    \"automationActions()\": \"88c4caba\",\n    \"balanceOf(address)\": \"70a08231\",\n    \"blockExternalReceiver()\": \"4c4602da\",\n    \"cachePriceRouter(bool,uint16,address)\": \"c588d8d6\",\n    \"callOnAdaptor((address,bytes[])[])\": \"4e84befe\",\n    \"convertToAssets(uint256)\": \"07a2d13a\",\n    \"convertToShares(uint256)\": \"c6e6f592\",\n    \"creditPositions(uint256)\": \"59d20b4e\",\n    \"debtPositions(uint256)\": \"e1b1acb7\",\n    \"decimals()\": \"313ce567\",\n    \"decreaseShareSupplyCap(uint192)\": \"575bbce6\",\n    \"deposit(uint256,address)\": \"6e553f65\",\n    \"feeData()\": \"e753e600\",\n    \"forcePositionOut(uint32,uint32,bool)\": \"a07bee0b\",\n    \"getCreditPositions()\": \"71e99dc2\",\n    \"getDebtPositions()\": \"3e3382ba\",\n    \"getPositionData(uint32)\": \"7384504f\",\n    \"holdingPosition()\": \"9c5f00c2\",\n    \"ignorePause()\": \"9959af94\",\n    \"increaseShareSupplyCap(uint192)\": \"b0646e27\",\n    \"initiateShutdown()\": \"0a680e18\",\n    \"isPaused()\": \"b187bd26\",\n    \"isPositionUsed(uint256)\": \"93bbeac0\",\n    \"isShutdown()\": \"bf86d690\",\n    \"liftShutdown()\": \"5e2c576e\",\n    \"locked()\": \"cf309012\",\n    \"maxDeposit(address)\": \"402d267d\",\n    \"maxMint(address)\": \"c63d75b6\",\n    \"maxRedeem(address)\": \"d905777e\",\n    \"maxWithdraw(address)\": \"ce96cb77\",\n    \"mint(uint256,address)\": \"94bf804d\",\n    \"multicall(bytes[])\": \"ac9650d8\",\n    \"name()\": \"06fdde03\",\n    \"nonces(address)\": \"7ecebe00\",\n    \"onERC721Received(address,address,uint256,bytes)\": \"150b7a02\",\n    \"owner()\": \"8da5cb5b\",\n    \"permit(address,address,uint256,uint256,uint8,bytes32,bytes32)\": \"d505accf\",\n    \"positionCatalogue(uint32)\": \"cbdf33d0\",\n    \"previewDeposit(uint256)\": \"ef8b30f7\",\n    \"previewMint(uint256)\": \"b3d7f6b9\",\n    \"previewRedeem(uint256)\": \"4cdad506\",\n    \"previewWithdraw(uint256)\": \"0a28a477\",\n    \"priceRouter()\": \"d7d4bf45\",\n    \"redeem(uint256,address,address)\": \"ba087652\",\n    \"registry()\": \"7b103999\",\n    \"removeAdaptorFromCatalogue(address)\": \"5f6b88a0\",\n    \"removePosition(uint32,bool)\": \"33e15be2\",\n    \"removePositionFromCatalogue(uint32)\": \"d1e88404\",\n    \"setAutomationActions(uint256,address)\": \"c8e81950\",\n    \"setHoldingPosition(uint32)\": \"0780fd3a\",\n    \"setRebalanceDeviation(uint256)\": \"530a3714\",\n    \"setShareLockPeriod(uint256)\": \"9c552ca8\",\n    \"setStrategistPayoutAddress(address)\": \"b0a75d36\",\n    \"setStrategistPlatformCut(uint64)\": \"b5292a99\",\n    \"shareLockPeriod()\": \"9fdb11b6\",\n    \"shareSupplyCap()\": \"d446bbcc\",\n    \"swapPositions(uint32,uint32,bool)\": \"379e0b13\",\n    \"symbol()\": \"95d89b41\",\n    \"toggleIgnorePause()\": \"a373e3ff\",\n    \"totalAssets()\": \"01e1d114\",\n    \"totalAssetsWithdrawable()\": \"a8144e48\",\n    \"totalSupply()\": \"18160ddd\",\n    \"transfer(address,uint256)\": \"a9059cbb\",\n    \"transferFrom(address,address,uint256)\": \"23b872dd\",\n    \"transferOwnership(address)\": \"f2fde38b\",\n    \"userShareLockStartTime(address)\": \"687c2b50\",\n    \"viewPositionBalances()\": \"78e0233e\",\n    \"withdraw(uint256,address,address)\": \"b460af94\"\n  },\n  \"rawMetadata\": \"{\\\"compiler\\\":{\\\"version\\\":\\\"0.8.21+commit.d9974bed\\\"},\\\"language\\\":\\\"Solidity\\\",\\\"output\\\":{\\\"abi\\\":[{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"_owner\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"contract Registry\\\",\\\"name\\\":\\\"_registry\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"contract ERC20\\\",\\\"name\\\":\\\"_asset\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"string\\\",\\\"name\\\":\\\"_name\\\",\\\"type\\\":\\\"string\\\"},{\\\"internalType\\\":\\\"string\\\",\\\"name\\\":\\\"_symbol\\\",\\\"type\\\":\\\"string\\\"},{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"_holdingPosition\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"_holdingPositionConfig\\\",\\\"type\\\":\\\"bytes\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"_initialDeposit\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"_strategistPlatformCut\\\",\\\"type\\\":\\\"uint64\\\"},{\\\"internalType\\\":\\\"uint192\\\",\\\"name\\\":\\\"_shareSupplyCap\\\",\\\"type\\\":\\\"uint192\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"constructor\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"asset\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"expectedAsset\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"Cellar__AssetMismatch\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"Cellar__CallToAdaptorNotAllowed\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__CallerNotApprovedToRebalance\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ContractNotShutdown\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ContractShutdown\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"Cellar__DebtMismatch\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ExpectedAddressDoesNotMatchActual\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__FailedToForceOutPosition\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"illiquidPosition\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"Cellar__IlliquidWithdraw\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assetsOwed\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__IncompleteWithdraw\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__InvalidFee\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__InvalidFeeCut\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"Cellar__InvalidHoldingPosition\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"requested\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"max\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__InvalidRebalanceDeviation\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__InvalidShareLockPeriod\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__InvalidShareSupplyCap\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__MinimumConstructorMintNotMet\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"depositor\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"Cellar__NotApprovedToDepositOnBehalf\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__Paused\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"Cellar__PositionAlreadyUsed\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"maxPositions\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__PositionArrayFull\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"sharesRemaining\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__PositionNotEmpty\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"Cellar__PositionNotInCatalogue\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"Cellar__PositionNotUsed\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__RemovingHoldingPosition\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__SettingValueToRegistryIdZeroIsProhibited\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ShareSupplyCapExceeded\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"timeSharesAreUnlocked\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"currentBlock\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__SharesAreLocked\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"min\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"max\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__TotalAssetDeviatedOutsideRange\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"current\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"expected\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Cellar__TotalSharesMustRemainConstant\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ZeroAssets\\\",\\\"type\\\":\\\"error\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"Cellar__ZeroShares\\\",\\\"type\\\":\\\"error\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"data\\\",\\\"type\\\":\\\"bytes\\\"}],\\\"name\\\":\\\"AdaptorCalled\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inCatalogue\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"AdaptorCatalogueAltered\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"spender\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"amount\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Approval\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"newAutomationActions\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"Cellar__AutomationActionsUpdated\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"caller\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Deposit\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"user\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"newOwner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"OwnershipTransferred\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"index\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"PositionAdded\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inCatalogue\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"PositionCatalogueAltered\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"position\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"index\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"PositionRemoved\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"newPosition1\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"newPosition2\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"index1\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"index2\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"PositionSwapped\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"oldDeviation\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"newDeviation\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"RebalanceDeviationChanged\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"oldPeriod\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"newPeriod\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"ShareLockingPeriodChanged\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"isShutdown\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"ShutdownChanged\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"oldPayoutAddress\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"newPayoutAddress\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"StrategistPayoutAddressChanged\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"oldPlatformCut\\\",\\\"type\\\":\\\"uint64\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"newPlatformCut\\\",\\\"type\\\":\\\"uint64\\\"}],\\\"name\\\":\\\"StrategistPlatformCutChanged\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"from\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"to\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"amount\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Transfer\\\",\\\"type\\\":\\\"event\\\"},{\\\"anonymous\\\":false,\\\"inputs\\\":[{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"caller\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"receiver\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":true,\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"indexed\\\":false,\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"Withdraw\\\",\\\"type\\\":\\\"event\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"DOMAIN_SEPARATOR\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bytes32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bytes32\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"GRAVITY_BRIDGE_REGISTRY_SLOT\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MAXIMUM_SHARE_LOCK_PERIOD\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MAX_FEE_CUT\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint64\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MAX_PLATFORM_FEE\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint64\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MAX_POSITIONS\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MAX_REBALANCE_DEVIATION\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint64\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"MINIMUM_SHARE_LOCK_PERIOD\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"PRICE_ROUTER_REGISTRY_SLOT\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"adaptorCatalogue\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"addAdaptorToCatalogue\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"index\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"configurationData\\\",\\\"type\\\":\\\"bytes\\\"},{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inDebtArray\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"addPosition\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"addPositionToCatalogue\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"allowance\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"allowedRebalanceDeviation\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"spender\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"amount\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"approve\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"asset\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"contract ERC20\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"automationActions\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"balanceOf\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"blockExternalReceiver\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"checkTotalAssets\\\",\\\"type\\\":\\\"bool\\\"},{\\\"internalType\\\":\\\"uint16\\\",\\\"name\\\":\\\"allowableRange\\\",\\\"type\\\":\\\"uint16\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"expectedPriceRouter\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"cachePriceRouter\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"components\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"bytes[]\\\",\\\"name\\\":\\\"callData\\\",\\\"type\\\":\\\"bytes[]\\\"}],\\\"internalType\\\":\\\"struct Cellar.AdaptorCall[]\\\",\\\"name\\\":\\\"data\\\",\\\"type\\\":\\\"tuple[]\\\"}],\\\"name\\\":\\\"callOnAdaptor\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"convertToAssets\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"convertToShares\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"creditPositions\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"debtPositions\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"decimals\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint8\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint8\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint192\\\",\\\"name\\\":\\\"_newShareSupplyCap\\\",\\\"type\\\":\\\"uint192\\\"}],\\\"name\\\":\\\"decreaseShareSupplyCap\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"receiver\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"deposit\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"feeData\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"strategistPlatformCut\\\",\\\"type\\\":\\\"uint64\\\"},{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"platformFee\\\",\\\"type\\\":\\\"uint64\\\"},{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"lastAccrual\\\",\\\"type\\\":\\\"uint64\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"strategistPayoutAddress\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"index\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inDebtArray\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"forcePositionOut\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"getCreditPositions\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint32[]\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32[]\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"getDebtPositions\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint32[]\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32[]\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"getPositionData\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"isDebt\\\",\\\"type\\\":\\\"bool\\\"},{\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"adaptorData\\\",\\\"type\\\":\\\"bytes\\\"},{\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"configurationData\\\",\\\"type\\\":\\\"bytes\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"holdingPosition\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"ignorePause\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint192\\\",\\\"name\\\":\\\"_newShareSupplyCap\\\",\\\"type\\\":\\\"uint192\\\"}],\\\"name\\\":\\\"increaseShareSupplyCap\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"initiateShutdown\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"isPaused\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"isPositionUsed\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"isShutdown\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"liftShutdown\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"locked\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"maxDeposit\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"maxMint\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"maxRedeem\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"maxWithdraw\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"receiver\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"mint\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"bytes[]\\\",\\\"name\\\":\\\"data\\\",\\\"type\\\":\\\"bytes[]\\\"}],\\\"name\\\":\\\"multicall\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"name\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"string\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"string\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"nonces\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"bytes\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bytes\\\"}],\\\"name\\\":\\\"onERC721Received\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bytes4\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bytes4\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"owner\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"spender\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"value\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"deadline\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"uint8\\\",\\\"name\\\":\\\"v\\\",\\\"type\\\":\\\"uint8\\\"},{\\\"internalType\\\":\\\"bytes32\\\",\\\"name\\\":\\\"r\\\",\\\"type\\\":\\\"bytes32\\\"},{\\\"internalType\\\":\\\"bytes32\\\",\\\"name\\\":\\\"s\\\",\\\"type\\\":\\\"bytes32\\\"}],\\\"name\\\":\\\"permit\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"positionCatalogue\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"previewDeposit\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"previewMint\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"previewRedeem\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"previewWithdraw\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"priceRouter\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"contract PriceRouter\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"receiver\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"redeem\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"registry\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"contract Registry\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"adaptor\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"removeAdaptorFromCatalogue\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"index\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inDebtArray\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"removePosition\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"removePositionFromCatalogue\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"_registryId\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"_expectedAutomationActions\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"setAutomationActions\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"positionId\\\",\\\"type\\\":\\\"uint32\\\"}],\\\"name\\\":\\\"setHoldingPosition\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"newDeviation\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"setRebalanceDeviation\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"newLock\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"setShareLockPeriod\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"payout\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"setStrategistPayoutAddress\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint64\\\",\\\"name\\\":\\\"cut\\\",\\\"type\\\":\\\"uint64\\\"}],\\\"name\\\":\\\"setStrategistPlatformCut\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"shareLockPeriod\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"shareSupplyCap\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint192\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint192\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"index1\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"uint32\\\",\\\"name\\\":\\\"index2\\\",\\\"type\\\":\\\"uint32\\\"},{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"inDebtArray\\\",\\\"type\\\":\\\"bool\\\"}],\\\"name\\\":\\\"swapPositions\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"symbol\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"string\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"string\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"toggleIgnorePause\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"totalAssets\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"totalAssetsWithdrawable\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"totalSupply\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"to\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"amount\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"transfer\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"from\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"to\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"amount\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"name\\\":\\\"transferFrom\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"bool\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"bool\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"newOwner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"transferOwnership\\\",\\\"outputs\\\":[],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"userShareLockStartTime\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[],\\\"name\\\":\\\"viewPositionBalances\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"contract ERC20[]\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"address[]\\\"},{\\\"internalType\\\":\\\"uint256[]\\\",\\\"name\\\":\\\"balances\\\",\\\"type\\\":\\\"uint256[]\\\"},{\\\"internalType\\\":\\\"bool[]\\\",\\\"name\\\":\\\"isDebt\\\",\\\"type\\\":\\\"bool[]\\\"}],\\\"stateMutability\\\":\\\"view\\\",\\\"type\\\":\\\"function\\\"},{\\\"inputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"assets\\\",\\\"type\\\":\\\"uint256\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"receiver\\\",\\\"type\\\":\\\"address\\\"},{\\\"internalType\\\":\\\"address\\\",\\\"name\\\":\\\"owner\\\",\\\"type\\\":\\\"address\\\"}],\\\"name\\\":\\\"withdraw\\\",\\\"outputs\\\":[{\\\"internalType\\\":\\\"uint256\\\",\\\"name\\\":\\\"shares\\\",\\\"type\\\":\\\"uint256\\\"}],\\\"stateMutability\\\":\\\"nonpayable\\\",\\\"type\\\":\\\"function\\\"}],\\\"devdoc\\\":{\\\"errors\\\":{\\\"Cellar__AssetMismatch(address,address)\\\":[{\\\"params\\\":{\\\"asset\\\":\\\"address of the asset\\\",\\\"expectedAsset\\\":\\\"address of the expected asset\\\"}}],\\\"Cellar__CallToAdaptorNotAllowed(address)\\\":[{\\\"params\\\":{\\\"adaptor\\\":\\\"the adaptor address that is paused or not trusted.\\\"}}],\\\"Cellar__DebtMismatch(uint32)\\\":[{\\\"params\\\":{\\\"position\\\":\\\"the posiiton id that was mismatched\\\"}}],\\\"Cellar__IlliquidWithdraw(address)\\\":[{\\\"params\\\":{\\\"illiquidPosition\\\":\\\"the illiquid position.\\\"}}],\\\"Cellar__IncompleteWithdraw(uint256)\\\":[{\\\"params\\\":{\\\"assetsOwed\\\":\\\"the remaining assets owed that were not withdrawn.\\\"}}],\\\"Cellar__InvalidHoldingPosition(uint32)\\\":[{\\\"params\\\":{\\\"positionId\\\":\\\"the id of the invalid position.\\\"}}],\\\"Cellar__InvalidRebalanceDeviation(uint256,uint256)\\\":[{\\\"params\\\":{\\\"max\\\":\\\"the max rebalance deviation.\\\",\\\"requested\\\":\\\"the requested rebalance  deviation\\\"}}],\\\"Cellar__PositionAlreadyUsed(uint32)\\\":[{\\\"params\\\":{\\\"position\\\":\\\"id of the position\\\"}}],\\\"Cellar__PositionArrayFull(uint256)\\\":[{\\\"params\\\":{\\\"maxPositions\\\":\\\"maximum number of positions that can be used\\\"}}],\\\"Cellar__PositionNotEmpty(uint32,uint256)\\\":[{\\\"params\\\":{\\\"position\\\":\\\"address of the non-empty position\\\",\\\"sharesRemaining\\\":\\\"amount of shares remaining in the position\\\"}}],\\\"Cellar__PositionNotInCatalogue(uint32)\\\":[{\\\"params\\\":{\\\"position\\\":\\\"id of the position\\\"}}],\\\"Cellar__PositionNotUsed(uint32)\\\":[{\\\"params\\\":{\\\"position\\\":\\\"id of the position\\\"}}],\\\"Cellar__SharesAreLocked(uint256,uint256)\\\":[{\\\"params\\\":{\\\"currentBlock\\\":\\\"the current block number.\\\",\\\"timeSharesAreUnlocked\\\":\\\"time when caller can transfer/redeem shares\\\"}}],\\\"Cellar__TotalAssetDeviatedOutsideRange(uint256,uint256,uint256)\\\":[{\\\"params\\\":{\\\"assets\\\":\\\"the total assets in the cellar\\\",\\\"max\\\":\\\"the maximum allowed assets\\\",\\\"min\\\":\\\"the minimum allowed assets\\\"}}],\\\"Cellar__TotalSharesMustRemainConstant(uint256,uint256)\\\":[{\\\"params\\\":{\\\"current\\\":\\\"the current amount of total shares\\\",\\\"expected\\\":\\\"the expected amount of total shares\\\"}}]},\\\"events\\\":{\\\"PositionAdded(uint32,uint256)\\\":{\\\"params\\\":{\\\"index\\\":\\\"index that position was added at\\\",\\\"position\\\":\\\"id of position that was added\\\"}},\\\"PositionRemoved(uint32,uint256)\\\":{\\\"params\\\":{\\\"index\\\":\\\"index that position was removed from\\\",\\\"position\\\":\\\"id of position that was removed\\\"}},\\\"PositionSwapped(uint32,uint32,uint256,uint256)\\\":{\\\"params\\\":{\\\"index1\\\":\\\"index of first position involved in the swap\\\",\\\"index2\\\":\\\"index of second position involved in the swap.\\\",\\\"newPosition1\\\":\\\"id of position (previously at index2) that replaced index1.\\\",\\\"newPosition2\\\":\\\"id of position (previously at index1) that replaced index2.\\\"}},\\\"RebalanceDeviationChanged(uint256,uint256)\\\":{\\\"params\\\":{\\\"newDeviation\\\":\\\"the new rebalance deviation\\\",\\\"oldDeviation\\\":\\\"the old rebalance deviation\\\"}},\\\"ShareLockingPeriodChanged(uint256,uint256)\\\":{\\\"params\\\":{\\\"newPeriod\\\":\\\"the new locking period\\\",\\\"oldPeriod\\\":\\\"the old locking period\\\"}},\\\"ShutdownChanged(bool)\\\":{\\\"params\\\":{\\\"isShutdown\\\":\\\"whether the cellar is shutdown\\\"}},\\\"StrategistPayoutAddressChanged(address,address)\\\":{\\\"params\\\":{\\\"newPayoutAddress\\\":\\\"value strategists payout address was changed to\\\",\\\"oldPayoutAddress\\\":\\\"value strategists payout address was changed from\\\"}},\\\"StrategistPlatformCutChanged(uint64,uint64)\\\":{\\\"params\\\":{\\\"newPlatformCut\\\":\\\"value strategist platform fee cut was changed to\\\",\\\"oldPlatformCut\\\":\\\"value strategist platform fee cut was changed from\\\"}}},\\\"kind\\\":\\\"dev\\\",\\\"methods\\\":{\\\"addAdaptorToCatalogue(address)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\"},\\\"addPosition(uint32,uint32,bytes,bool)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\",\\\"params\\\":{\\\"configurationData\\\":\\\"data used to configure how the position behaves\\\",\\\"index\\\":\\\"index at which to insert the position\\\",\\\"positionId\\\":\\\"id of position to add\\\"}},\\\"addPositionToCatalogue(uint32)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\"},\\\"cachePriceRouter(bool,uint16,address)\\\":{\\\"details\\\":\\\"`allowableRange` reverts from arithmetic underflow if it is greater than 10_000, this is      desired behavior.Callable by Sommelier Governance.\\\",\\\"params\\\":{\\\"allowableRange\\\":\\\"The +- range the total assets may deviate between the old and new price router.                       - 1_000 == 10%                       - 500 == 5%\\\",\\\"checkTotalAssets\\\":\\\"If true totalAssets is checked before and after updating the price router,        and is verified to be withing a +- 5% envelope.        If false totalAssets is only called after updating the price router.]\\\",\\\"expectedPriceRouter\\\":\\\"The registry price router differed from the expected price router.\\\"}},\\\"callOnAdaptor((address,bytes[])[])\\\":{\\\"details\\\":\\\"There are several safety checks in this function to prevent strategists from abusing it.      - `blockExternalReceiver`      - `totalAssets` must not change by much      - `totalShares` must remain constant      - adaptors must be set up to be used with this cellarSince `totalAssets` is allowed to deviate slightly, strategists could abuse this by sending      multiple `callOnAdaptor` calls rapidly, to gradually change the share price.      To mitigate this, rate limiting will be put in place on the Sommelier side.Callable by Sommelier Strategist, and Automation Actions contract.\\\"},\\\"convertToAssets(uint256)\\\":{\\\"details\\\":\\\"Use preview functions to get accurate assets.Under estimates assets.\\\",\\\"params\\\":{\\\"shares\\\":\\\"amount of shares to convert\\\"},\\\"returns\\\":{\\\"assets\\\":\\\"the shares can be exchanged for\\\"}},\\\"convertToShares(uint256)\\\":{\\\"details\\\":\\\"Use preview functions to get accurate shares.Under estimates shares.\\\",\\\"params\\\":{\\\"assets\\\":\\\"amount of assets to convert\\\"},\\\"returns\\\":{\\\"shares\\\":\\\"the assets can be exchanged for\\\"}},\\\"decreaseShareSupplyCap(uint192)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"deposit(uint256,address)\\\":{\\\"params\\\":{\\\"assets\\\":\\\"amount of assets deposited by user.\\\",\\\"receiver\\\":\\\"address to receive the shares.\\\"},\\\"returns\\\":{\\\"shares\\\":\\\"amount of shares given for deposit.\\\"}},\\\"forcePositionOut(uint32,uint32,bool)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\"},\\\"increaseShareSupplyCap(uint192)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\"},\\\"initiateShutdown()\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"liftShutdown()\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"maxDeposit(address)\\\":{\\\"returns\\\":{\\\"_0\\\":\\\"assets maximum amount of assets that can be deposited\\\"}},\\\"maxMint(address)\\\":{\\\"returns\\\":{\\\"_0\\\":\\\"shares maximum amount of shares that can be minted\\\"}},\\\"maxRedeem(address)\\\":{\\\"details\\\":\\\"EIP4626 states maxRedeem must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.\\\",\\\"params\\\":{\\\"owner\\\":\\\"address to check maxRedeem of.\\\"},\\\"returns\\\":{\\\"_0\\\":\\\"the max amount of shares redeemable by `owner`.\\\"}},\\\"maxWithdraw(address)\\\":{\\\"details\\\":\\\"EIP4626 states maxWithdraw must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.\\\",\\\"params\\\":{\\\"owner\\\":\\\"address to check maxWithdraw of.\\\"},\\\"returns\\\":{\\\"_0\\\":\\\"the max amount of assets withdrawable by `owner`.\\\"}},\\\"mint(uint256,address)\\\":{\\\"params\\\":{\\\"receiver\\\":\\\"address to receive the shares.\\\",\\\"shares\\\":\\\"amount of shares requested by user.\\\"},\\\"returns\\\":{\\\"assets\\\":\\\"amount of assets deposited into the cellar.\\\"}},\\\"multicall(bytes[])\\\":{\\\"details\\\":\\\"Does NOT return the function return values.\\\"},\\\"onERC721Received(address,address,uint256,bytes)\\\":{\\\"details\\\":\\\"See {IERC721Receiver-onERC721Received}. Always returns `IERC721Receiver.onERC721Received.selector`.\\\"},\\\"previewDeposit(uint256)\\\":{\\\"params\\\":{\\\"assets\\\":\\\"amount of assets to deposit\\\"},\\\"returns\\\":{\\\"shares\\\":\\\"that will be minted\\\"}},\\\"previewMint(uint256)\\\":{\\\"params\\\":{\\\"shares\\\":\\\"amount of shares to mint\\\"},\\\"returns\\\":{\\\"assets\\\":\\\"that will be deposited\\\"}},\\\"previewRedeem(uint256)\\\":{\\\"params\\\":{\\\"shares\\\":\\\"amount of shares to redeem\\\"},\\\"returns\\\":{\\\"assets\\\":\\\"that will be returned\\\"}},\\\"previewWithdraw(uint256)\\\":{\\\"params\\\":{\\\"assets\\\":\\\"amount of assets to withdraw\\\"},\\\"returns\\\":{\\\"shares\\\":\\\"that will be redeemed\\\"}},\\\"redeem(uint256,address,address)\\\":{\\\"details\\\":\\\"Unlike conventional ERC4626 contracts, this may not always return one asset to the receiver.      Since there are no swaps involved in this function, the receiver may receive multiple      assets. The value of all the assets returned will be equal to the amount defined by      `assets` denominated in the `asset` of the cellar (eg. if `asset` is USDC and `assets`      is 1000, then the receiver will receive $1000 worth of assets in either one or many      tokens).\\\",\\\"params\\\":{\\\"owner\\\":\\\"address that owns the shares being redeemed\\\",\\\"receiver\\\":\\\"address that will receive withdrawn assets\\\",\\\"shares\\\":\\\"amount of shares to redeem\\\"},\\\"returns\\\":{\\\"assets\\\":\\\"equivalent value of the assets withdrawn, denominated in the cellar's asset\\\"}},\\\"removeAdaptorFromCatalogue(address)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"removePosition(uint32,bool)\\\":{\\\"details\\\":\\\"Called by strategist.Callable by Sommelier Strategist.\\\",\\\"params\\\":{\\\"index\\\":\\\"index at which to remove the position\\\"}},\\\"removePositionFromCatalogue(uint32)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"setAutomationActions(uint256,address)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\",\\\"params\\\":{\\\"_expectedAutomationActions\\\":\\\"The registry automation actions differed from the expected automation actions.\\\",\\\"_registryId\\\":\\\"Registry Id to get the automation action.\\\"}},\\\"setHoldingPosition(uint32)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\"},\\\"setRebalanceDeviation(uint256)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\",\\\"params\\\":{\\\"newDeviation\\\":\\\"the new rebalance deviation value.\\\"}},\\\"setShareLockPeriod(uint256)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\",\\\"params\\\":{\\\"newLock\\\":\\\"the new lock period\\\"}},\\\"setStrategistPayoutAddress(address)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\",\\\"params\\\":{\\\"payout\\\":\\\"the new strategist payout address\\\"}},\\\"setStrategistPlatformCut(uint64)\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\",\\\"params\\\":{\\\"cut\\\":\\\"the platform cut for the strategist\\\"}},\\\"swapPositions(uint32,uint32,bool)\\\":{\\\"details\\\":\\\"Callable by Sommelier Strategist.\\\",\\\"params\\\":{\\\"inDebtArray\\\":\\\"bool indicating to switch positions in the debt array, or the credit array.\\\",\\\"index1\\\":\\\"index of first position to swap\\\",\\\"index2\\\":\\\"index of second position to swap\\\"}},\\\"toggleIgnorePause()\\\":{\\\"details\\\":\\\"Callable by Sommelier Governance.\\\"},\\\"totalAssets()\\\":{\\\"details\\\":\\\"EIP4626 states totalAssets needs to be inclusive of fees. Since performance fees mint shares, total assets remains unchanged, so this implementation is inclusive of fees even though it does not explicitly show it.EIP4626 states totalAssets must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.Run a re-entrancy check because totalAssets can be wrong if re-entering from deposit/withdraws.\\\"},\\\"totalAssetsWithdrawable()\\\":{\\\"details\\\":\\\"Run a re-entrancy check because totalAssetsWithdrawable can be wrong if re-entering from deposit/withdraws.\\\"},\\\"withdraw(uint256,address,address)\\\":{\\\"details\\\":\\\"Unlike conventional ERC4626 contracts, this may not always return one asset to the receiver.      Since there are no swaps involved in this function, the receiver may receive multiple      assets. The value of all the assets returned will be equal to the amount defined by      `assets` denominated in the `asset` of the cellar (eg. if `asset` is USDC and `assets`      is 1000, then the receiver will receive $1000 worth of assets in either one or many      tokens).\\\",\\\"params\\\":{\\\"assets\\\":\\\"equivalent value of the assets withdrawn, denominated in the cellar's asset\\\",\\\"owner\\\":\\\"address that owns the shares being redeemed\\\",\\\"receiver\\\":\\\"address that will receive withdrawn assets\\\"},\\\"returns\\\":{\\\"shares\\\":\\\"amount of shares redeemed\\\"}}},\\\"version\\\":1},\\\"userdoc\\\":{\\\"errors\\\":{\\\"Cellar__AssetMismatch(address,address)\\\":[{\\\"notice\\\":\\\"Attempted an operation with an asset that was different then the one expected.\\\"}],\\\"Cellar__CallToAdaptorNotAllowed(address)\\\":[{\\\"notice\\\":\\\"Strategist attempted to use an adaptor that is either paused or is not trusted by governance.\\\"}],\\\"Cellar__CallerNotApprovedToRebalance()\\\":[{\\\"notice\\\":\\\"Emitted when sender is not approved to call `callOnAdaptor`.\\\"}],\\\"Cellar__ContractNotShutdown()\\\":[{\\\"notice\\\":\\\"Attempted action was prevented due to contract not being shutdown.\\\"}],\\\"Cellar__ContractShutdown()\\\":[{\\\"notice\\\":\\\"Attempted action was prevented due to contract being shutdown.\\\"}],\\\"Cellar__DebtMismatch(uint32)\\\":[{\\\"notice\\\":\\\"Attempted to add a position, with mismatched debt.\\\"}],\\\"Cellar__ExpectedAddressDoesNotMatchActual()\\\":[{\\\"notice\\\":\\\"Attempted to use an address from the registry, but address was not expected.\\\"}],\\\"Cellar__FailedToForceOutPosition()\\\":[{\\\"notice\\\":\\\"Attempted to force out the wrong position.\\\"}],\\\"Cellar__IlliquidWithdraw(address)\\\":[{\\\"notice\\\":\\\"Attempted to withdraw an illiquid position.\\\"}],\\\"Cellar__IncompleteWithdraw(uint256)\\\":[{\\\"notice\\\":\\\"Withdraw did not withdraw all assets.\\\"}],\\\"Cellar__InvalidFee()\\\":[{\\\"notice\\\":\\\"Attempted to change platform fee with invalid value.\\\"}],\\\"Cellar__InvalidFeeCut()\\\":[{\\\"notice\\\":\\\"Attempted to change strategist fee cut with invalid value.\\\"}],\\\"Cellar__InvalidHoldingPosition(uint32)\\\":[{\\\"notice\\\":\\\"Attempted to add an invalid holding position.\\\"}],\\\"Cellar__InvalidRebalanceDeviation(uint256,uint256)\\\":[{\\\"notice\\\":\\\"Total shares in a cellar changed when they should stay constant.\\\"}],\\\"Cellar__InvalidShareLockPeriod()\\\":[{\\\"notice\\\":\\\"Attempted to set `shareLockPeriod` to an invalid number.\\\"}],\\\"Cellar__InvalidShareSupplyCap()\\\":[{\\\"notice\\\":\\\"Proposed share supply cap is not logical.\\\"}],\\\"Cellar__MinimumConstructorMintNotMet()\\\":[{\\\"notice\\\":\\\"Attempted to deploy contract without minting enough shares.\\\"}],\\\"Cellar__NotApprovedToDepositOnBehalf(address)\\\":[{\\\"notice\\\":\\\"Attempted deposit on behalf of a user without being approved.\\\"}],\\\"Cellar__Paused()\\\":[{\\\"notice\\\":\\\"Attempted to interact with the cellar when it is paused.\\\"}],\\\"Cellar__PositionAlreadyUsed(uint32)\\\":[{\\\"notice\\\":\\\"Attempted to add a position that is already being used.\\\"}],\\\"Cellar__PositionArrayFull(uint256)\\\":[{\\\"notice\\\":\\\"Attempted to add a position when the position array is full.\\\"}],\\\"Cellar__PositionNotEmpty(uint32,uint256)\\\":[{\\\"notice\\\":\\\"Attempted an action on a position that is required to be empty before the action can be performed.\\\"}],\\\"Cellar__PositionNotInCatalogue(uint32)\\\":[{\\\"notice\\\":\\\"Attempted to add a position that is not in the catalogue.\\\"}],\\\"Cellar__PositionNotUsed(uint32)\\\":[{\\\"notice\\\":\\\"Attempted to make an unused position the holding position.\\\"}],\\\"Cellar__RemovingHoldingPosition()\\\":[{\\\"notice\\\":\\\"Attempted to remove the Cellars holding position.\\\"}],\\\"Cellar__SettingValueToRegistryIdZeroIsProhibited()\\\":[{\\\"notice\\\":\\\"Attempted to set an address to registry Id 0.\\\"}],\\\"Cellar__ShareSupplyCapExceeded()\\\":[{\\\"notice\\\":\\\"Attempted entry would raise totalSupply above Share Supply Cap.\\\"}],\\\"Cellar__SharesAreLocked(uint256,uint256)\\\":[{\\\"notice\\\":\\\"Attempted to burn shares when they are locked.\\\"}],\\\"Cellar__TotalAssetDeviatedOutsideRange(uint256,uint256,uint256)\\\":[{\\\"notice\\\":\\\"totalAssets deviated outside the range set by `allowedRebalanceDeviation`.\\\"}],\\\"Cellar__TotalSharesMustRemainConstant(uint256,uint256)\\\":[{\\\"notice\\\":\\\"Total shares in a cellar changed when they should stay constant.\\\"}],\\\"Cellar__ZeroAssets()\\\":[{\\\"notice\\\":\\\"Attempted an action with zero assets.\\\"}],\\\"Cellar__ZeroShares()\\\":[{\\\"notice\\\":\\\"Attempted an action with zero shares.\\\"}]},\\\"events\\\":{\\\"AdaptorCalled(address,bytes)\\\":{\\\"notice\\\":\\\"Emitted when adaptor calls are made.\\\"},\\\"AdaptorCatalogueAltered(address,bool)\\\":{\\\"notice\\\":\\\"Emitted when Governance adds/removes an adaptor to/from the cellars catalogue.\\\"},\\\"Cellar__AutomationActionsUpdated(address)\\\":{\\\"notice\\\":\\\"Emitted when `setAutomationActions` is called.\\\"},\\\"PositionAdded(uint32,uint256)\\\":{\\\"notice\\\":\\\"Emitted when a position is added.\\\"},\\\"PositionCatalogueAltered(uint32,bool)\\\":{\\\"notice\\\":\\\"Emitted when Governance adds/removes a position to/from the cellars catalogue.\\\"},\\\"PositionRemoved(uint32,uint256)\\\":{\\\"notice\\\":\\\"Emitted when a position is removed.\\\"},\\\"PositionSwapped(uint32,uint32,uint256,uint256)\\\":{\\\"notice\\\":\\\"Emitted when the positions at two indexes are swapped.\\\"},\\\"RebalanceDeviationChanged(uint256,uint256)\\\":{\\\"notice\\\":\\\"Emitted on when the rebalance deviation is changed.\\\"},\\\"ShareLockingPeriodChanged(uint256,uint256)\\\":{\\\"notice\\\":\\\"Emitted when share locking period is changed.\\\"},\\\"ShutdownChanged(bool)\\\":{\\\"notice\\\":\\\"Emitted when cellar emergency state is changed.\\\"},\\\"StrategistPayoutAddressChanged(address,address)\\\":{\\\"notice\\\":\\\"Emitted when strategists payout address is changed.\\\"},\\\"StrategistPlatformCutChanged(uint64,uint64)\\\":{\\\"notice\\\":\\\"Emitted when strategist platform fee cut is changed.\\\"}},\\\"kind\\\":\\\"user\\\",\\\"methods\\\":{\\\"GRAVITY_BRIDGE_REGISTRY_SLOT()\\\":{\\\"notice\\\":\\\"Id to get the gravity bridge from the registry.\\\"},\\\"MAXIMUM_SHARE_LOCK_PERIOD()\\\":{\\\"notice\\\":\\\"Shares can be locked for at most 2 days after minting.\\\"},\\\"MAX_FEE_CUT()\\\":{\\\"notice\\\":\\\"Sets the max possible fee cut for this cellar.\\\"},\\\"MAX_PLATFORM_FEE()\\\":{\\\"notice\\\":\\\"Sets the max possible performance fee for this cellar.\\\"},\\\"MAX_POSITIONS()\\\":{\\\"notice\\\":\\\"Maximum amount of positions a cellar can have in it's credit/debt arrays.\\\"},\\\"MAX_REBALANCE_DEVIATION()\\\":{\\\"notice\\\":\\\"Stores the max possible rebalance deviation for this cellar.\\\"},\\\"MINIMUM_SHARE_LOCK_PERIOD()\\\":{\\\"notice\\\":\\\"Shares must be locked for at least 5 minutes after minting.\\\"},\\\"PRICE_ROUTER_REGISTRY_SLOT()\\\":{\\\"notice\\\":\\\"Id to get the price router from the registry.\\\"},\\\"adaptorCatalogue(address)\\\":{\\\"notice\\\":\\\"Adaptors the strategist is approved to use without any governance intervention.\\\"},\\\"addAdaptorToCatalogue(address)\\\":{\\\"notice\\\":\\\"Allows Governance to add adaptors to this cellar's catalogue.\\\"},\\\"addPosition(uint32,uint32,bytes,bool)\\\":{\\\"notice\\\":\\\"Insert a trusted position to the list of positions used by the cellar at a given index.\\\"},\\\"addPositionToCatalogue(uint32)\\\":{\\\"notice\\\":\\\"Allows Governance to add positions to this cellar's catalogue.\\\"},\\\"allowedRebalanceDeviation()\\\":{\\\"notice\\\":\\\"The percent the total assets of a cellar may deviate during a `callOnAdaptor`(rebalance) call.\\\"},\\\"automationActions()\\\":{\\\"notice\\\":\\\"The Automation Actions contract that can rebalance this Cellar.\\\"},\\\"blockExternalReceiver()\\\":{\\\"notice\\\":\\\"This bool is used to stop strategists from abusing Base Adaptor functions(deposit/withdraw).\\\"},\\\"cachePriceRouter(bool,uint16,address)\\\":{\\\"notice\\\":\\\"Updates the cellar to use the lastest price router in the registry.\\\"},\\\"callOnAdaptor((address,bytes[])[])\\\":{\\\"notice\\\":\\\"Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.\\\"},\\\"convertToAssets(uint256)\\\":{\\\"notice\\\":\\\"The amount of assets that the cellar would exchange for the amount of shares provided.\\\"},\\\"convertToShares(uint256)\\\":{\\\"notice\\\":\\\"The amount of shares that the cellar would exchange for the amount of assets provided.\\\"},\\\"creditPositions(uint256)\\\":{\\\"notice\\\":\\\"Array of uint32s made up of cellars credit positions Ids.\\\"},\\\"debtPositions(uint256)\\\":{\\\"notice\\\":\\\"Array of uint32s made up of cellars debt positions Ids.\\\"},\\\"decreaseShareSupplyCap(uint192)\\\":{\\\"notice\\\":\\\"Decreases the share supply cap.\\\"},\\\"deposit(uint256,address)\\\":{\\\"notice\\\":\\\"Deposits assets into the cellar, and returns shares to receiver.\\\"},\\\"feeData()\\\":{\\\"notice\\\":\\\"Stores all fee data for cellar.\\\"},\\\"forcePositionOut(uint32,uint32,bool)\\\":{\\\"notice\\\":\\\"Allows Sommelier Governance to forceably remove a position from the Cellar without checking its balance is zero.\\\"},\\\"getCreditPositions()\\\":{\\\"notice\\\":\\\"Get the ids of the credit positions currently used by the cellar.\\\"},\\\"getDebtPositions()\\\":{\\\"notice\\\":\\\"Get the ids of the debt positions currently used by the cellar.\\\"},\\\"getPositionData(uint32)\\\":{\\\"notice\\\":\\\"Get position data given position id.\\\"},\\\"holdingPosition()\\\":{\\\"notice\\\":\\\"Stores the position id of the holding position in the creditPositions array.\\\"},\\\"ignorePause()\\\":{\\\"notice\\\":\\\"Pauses all user entry/exits, and strategist rebalances.\\\"},\\\"increaseShareSupplyCap(uint192)\\\":{\\\"notice\\\":\\\"Increases the share supply cap.\\\"},\\\"initiateShutdown()\\\":{\\\"notice\\\":\\\"Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.\\\"},\\\"isPaused()\\\":{\\\"notice\\\":\\\"View function external contracts can use to see if the cellar is paused.\\\"},\\\"isPositionUsed(uint256)\\\":{\\\"notice\\\":\\\"Tell whether a position is currently used.\\\"},\\\"isShutdown()\\\":{\\\"notice\\\":\\\"Whether or not the contract is shutdown in case of an emergency.\\\"},\\\"liftShutdown()\\\":{\\\"notice\\\":\\\"Restart the cellar.\\\"},\\\"locked()\\\":{\\\"notice\\\":\\\"`locked` is public, so that the state can be checked even during view function calls.\\\"},\\\"maxDeposit(address)\\\":{\\\"notice\\\":\\\"Total amount of assets that can be deposited for a user.\\\"},\\\"maxMint(address)\\\":{\\\"notice\\\":\\\"Total amount of shares that can be minted for a user.\\\"},\\\"maxRedeem(address)\\\":{\\\"notice\\\":\\\"Returns the max amount shares redeemable by a user\\\"},\\\"maxWithdraw(address)\\\":{\\\"notice\\\":\\\"Returns the max amount withdrawable by a user inclusive of performance fees\\\"},\\\"mint(uint256,address)\\\":{\\\"notice\\\":\\\"Mints shares from the cellar, and returns shares to receiver.\\\"},\\\"multicall(bytes[])\\\":{\\\"notice\\\":\\\"Allows caller to call multiple functions in a single TX.\\\"},\\\"positionCatalogue(uint32)\\\":{\\\"notice\\\":\\\"Positions the strategist is approved to use without any governance intervention.\\\"},\\\"previewDeposit(uint256)\\\":{\\\"notice\\\":\\\"Simulate the effects of depositing assets at the current block, given current on-chain conditions.\\\"},\\\"previewMint(uint256)\\\":{\\\"notice\\\":\\\"Simulate the effects of minting shares at the current block, given current on-chain conditions.\\\"},\\\"previewRedeem(uint256)\\\":{\\\"notice\\\":\\\"Simulate the effects of redeeming shares at the current block, given current on-chain conditions.\\\"},\\\"previewWithdraw(uint256)\\\":{\\\"notice\\\":\\\"Simulate the effects of withdrawing assets at the current block, given current on-chain conditions.\\\"},\\\"priceRouter()\\\":{\\\"notice\\\":\\\"Cached price router contract.\\\"},\\\"redeem(uint256,address,address)\\\":{\\\"notice\\\":\\\"Redeem shares to withdraw assets from the cellar.\\\"},\\\"registry()\\\":{\\\"notice\\\":\\\"Address of the platform's registry contract. Used to get the latest address of modules.\\\"},\\\"removeAdaptorFromCatalogue(address)\\\":{\\\"notice\\\":\\\"Allows Governance to remove adaptors from this cellar's catalogue.\\\"},\\\"removePosition(uint32,bool)\\\":{\\\"notice\\\":\\\"Remove the position at a given index from the list of positions used by the cellar.\\\"},\\\"removePositionFromCatalogue(uint32)\\\":{\\\"notice\\\":\\\"Allows Governance to remove positions from this cellar's catalogue.\\\"},\\\"setAutomationActions(uint256,address)\\\":{\\\"notice\\\":\\\"Set the Automation Actions contract.\\\"},\\\"setHoldingPosition(uint32)\\\":{\\\"notice\\\":\\\"Allows owner to change the holding position.\\\"},\\\"setRebalanceDeviation(uint256)\\\":{\\\"notice\\\":\\\"Allows governance to change this cellars rebalance deviation.\\\"},\\\"setShareLockPeriod(uint256)\\\":{\\\"notice\\\":\\\"Allows share lock period to be updated.\\\"},\\\"setStrategistPayoutAddress(address)\\\":{\\\"notice\\\":\\\"Sets the Strategists payout address\\\"},\\\"setStrategistPlatformCut(uint64)\\\":{\\\"notice\\\":\\\"Sets the Strategists cut of platform fees\\\"},\\\"shareLockPeriod()\\\":{\\\"notice\\\":\\\"After deposits users must wait `shareLockPeriod` time before being able to transfer or withdraw their shares.\\\"},\\\"shareSupplyCap()\\\":{\\\"notice\\\":\\\"The maximum amount of shares that can be in circulation.\\\"},\\\"swapPositions(uint32,uint32,bool)\\\":{\\\"notice\\\":\\\"Swap the positions at two given indexes.\\\"},\\\"toggleIgnorePause()\\\":{\\\"notice\\\":\\\"Allows governance to choose whether or not to respect a pause.\\\"},\\\"totalAssets()\\\":{\\\"notice\\\":\\\"The total amount of assets in the cellar.\\\"},\\\"totalAssetsWithdrawable()\\\":{\\\"notice\\\":\\\"The total amount of withdrawable assets in the cellar.\\\"},\\\"transfer(address,uint256)\\\":{\\\"notice\\\":\\\"Override `transfer` to add share lock check.\\\"},\\\"transferFrom(address,address,uint256)\\\":{\\\"notice\\\":\\\"Override `transferFrom` to add share lock check.\\\"},\\\"userShareLockStartTime(address)\\\":{\\\"notice\\\":\\\"mapping that stores every users last time stamp they minted shares.\\\"},\\\"viewPositionBalances()\\\":{\\\"notice\\\":\\\"View the amount of assets in each Cellar Position.\\\"},\\\"withdraw(uint256,address,address)\\\":{\\\"notice\\\":\\\"Withdraw assets from the cellar by redeeming shares.\\\"}},\\\"version\\\":1}},\\\"settings\\\":{\\\"compilationTarget\\\":{\\\"src/base/permutations/CellarWithShareLockPeriod.sol\\\":\\\"CellarWithShareLockPeriod\\\"},\\\"evmVersion\\\":\\\"paris\\\",\\\"libraries\\\":{},\\\"metadata\\\":{\\\"bytecodeHash\\\":\\\"ipfs\\\"},\\\"optimizer\\\":{\\\"enabled\\\":true,\\\"runs\\\":200},\\\"remappings\\\":[\\\":@balancer-labs/=lib/balancer-v2-monorepo/../../node_modules/@balancer-labs/\\\",\\\":@balancer/=lib/balancer-v2-monorepo/pkg/\\\",\\\":@chainlink/=lib/chainlink/\\\",\\\":@ds-test/=lib/forge-std/lib/ds-test/src/\\\",\\\":@forge-std/=lib/forge-std/src/\\\",\\\":@openzeppelin/=lib/openzeppelin-contracts/\\\",\\\":@solmate/=lib/solmate/src/\\\",\\\":@uniswap/v3-core/=lib/v3-core/\\\",\\\":@uniswap/v3-periphery/=lib/v3-periphery/\\\",\\\":@uniswapV3C/=lib/v3-core/contracts/\\\",\\\":@uniswapV3P/=lib/v3-periphery/contracts/\\\",\\\":axelar-gmp-sdk-solidity/=lib/axelar-gmp-sdk-solidity/contracts/\\\",\\\":balancer-v2-monorepo/=lib/balancer-v2-monorepo/\\\",\\\":chainlink/=lib/chainlink/integration-tests/contracts/ethereum/src/\\\",\\\":ds-test/=lib/forge-std/lib/ds-test/src/\\\",\\\":forge-std/=lib/forge-std/src/\\\",\\\":openzeppelin-contracts/=lib/openzeppelin-contracts/\\\",\\\":solmate/=lib/solmate/src/\\\",\\\":v3-core/=lib/v3-core/contracts/\\\",\\\":v3-periphery/=lib/v3-periphery/contracts/\\\"]},\\\"sources\\\":{\\\"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorInterface.sol\\\":{\\\"keccak256\\\":\\\"0xb496651006b9a2a07920ffe116928b11e2a6458e21361cecca51409522488ca7\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://f39ad60071af2c115e064ebeb1686097efa83b26da0e2c814c635538538b7465\\\",\\\"dweb:/ipfs/QmYRARVDA1XZUqZNKNnArYHWbffNeeSVZQjt67ZXKGm85a\\\"]},\\\"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorV2V3Interface.sol\\\":{\\\"keccak256\\\":\\\"0x4a7757ff7bbafe044cd49c2a45c7c18ec50eff7c7af6869face5e1e9cda976f2\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://7c3f481f69f3ee07d6bb91b38d1cd61f9fa72de29c63d778c98956db70ecd57b\\\",\\\"dweb:/ipfs/QmPeJrNHTZF8CrXk3BgLJCamwf1dUEzHyQsMYrdd4v1NEG\\\"]},\\\"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol\\\":{\\\"keccak256\\\":\\\"0x6e6e4b0835904509406b070ee173b5bc8f677c19421b76be38aea3b1b3d30846\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://b3beaa37ee61e4ab615e250fbf01601ae481de843fd0ef55e6b44fd9d5fff8a7\\\",\\\"dweb:/ipfs/QmeZUVwd26LzK4Mfp8Zba5JbQNkZFfTzFu1A6FVMMZDg9c\\\"]},\\\"lib/openzeppelin-contracts/contracts/access/Ownable.sol\\\":{\\\"keccak256\\\":\\\"0xa94b34880e3c1b0b931662cb1c09e5dfa6662f31cba80e07c5ee71cd135c9673\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://40fb1b5102468f783961d0af743f91b9980cf66b50d1d12009f6bb1869cea4d2\\\",\\\"dweb:/ipfs/QmYqEbJML4jB1GHbzD4cUZDtJg5wVwNm3vDJq1GbyDus8y\\\"]},\\\"lib/openzeppelin-contracts/contracts/token/ERC721/IERC721Receiver.sol\\\":{\\\"keccak256\\\":\\\"0xa82b58eca1ee256be466e536706850163d2ec7821945abd6b4778cfb3bee37da\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://6e75cf83beb757b8855791088546b8337e9d4684e169400c20d44a515353b708\\\",\\\"dweb:/ipfs/QmYvPafLfoquiDMEj7CKHtvbgHu7TJNPSVPSCjrtjV8HjV\\\"]},\\\"lib/openzeppelin-contracts/contracts/token/ERC721/utils/ERC721Holder.sol\\\":{\\\"keccak256\\\":\\\"0x0108bf6a6ebd5f96678bed33a35947537263f96766131ee91461fb6485805028\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://ae2d274bf3d56a6d49a9bbd0a4871c54997a82551eb3eb1c0c39dc98698ff8bf\\\",\\\"dweb:/ipfs/QmTT7ty5DPGAmRnx94Xu3TUDYGSPDVLN2bppJAjjedrg1e\\\"]},\\\"lib/openzeppelin-contracts/contracts/utils/Address.sol\\\":{\\\"keccak256\\\":\\\"0xb94eac067c85cd79a4195c0a1f4a878e9827329045c12475a0199f1ae17b9700\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://2ad84b5dbf40ba9e944cc25bd0a98c51bafd49cff30efe5ef5aef921a70081de\\\",\\\"dweb:/ipfs/Qme8iCeqe9VdNgWktTTsSxUfHcJEXuvPaJpshWDzoWj56V\\\"]},\\\"lib/openzeppelin-contracts/contracts/utils/Context.sol\\\":{\\\"keccak256\\\":\\\"0xe2e337e6dde9ef6b680e07338c493ebea1b5fd09b43424112868e9cc1706bca7\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://6df0ddf21ce9f58271bdfaa85cde98b200ef242a05a3f85c2bc10a8294800a92\\\",\\\"dweb:/ipfs/QmRK2Y5Yc6BK7tGKkgsgn3aJEQGi5aakeSPZvS65PV8Xp3\\\"]},\\\"lib/openzeppelin-contracts/contracts/utils/math/SafeCast.sol\\\":{\\\"keccak256\\\":\\\"0x182ad835742e188a50bc98b938287d28bf74ad87d01e2bbc1d207c2ba36e1adb\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://c2ba26b6252bb52b39ffb18b2de027544619e3f71b78e5476eba52becfaae929\\\",\\\"dweb:/ipfs/Qmb7NqEzs7aWkrzpskxXCRb799XmPenZMDtpzRvNUph1Bg\\\"]},\\\"lib/solmate/src/auth/Owned.sol\\\":{\\\"keccak256\\\":\\\"0xfedb27d14c508342c33eb067c9a02eabcdb0f9dcf93b04ded1001f580d12d0ea\\\",\\\"license\\\":\\\"AGPL-3.0-only\\\",\\\"urls\\\":[\\\"bzz-raw://1ff52bbee698b9cf9e4574615e6550be0887ccf355f6571e23d6f25b332e79b4\\\",\\\"dweb:/ipfs/QmVorA2apojVRStzS7h8aFccR3Uv32G6HVtBtFHZrE7YXx\\\"]},\\\"lib/solmate/src/mixins/ERC4626.sol\\\":{\\\"keccak256\\\":\\\"0xa404f6f45bd53f24a90cc5ffe95e16b52e3f2dfd88f0d7a1edcb35f815919a7b\\\",\\\"license\\\":\\\"AGPL-3.0-only\\\",\\\"urls\\\":[\\\"bzz-raw://9f01e32d713e05cc58c1563e9938a1c5e096b1da9f52c7ea8424f2317b94adc1\\\",\\\"dweb:/ipfs/QmVt5SsbA3kezM5pyovupN7iZLy6QVqY5qQRZKLFqxKJUs\\\"]},\\\"lib/solmate/src/tokens/ERC20.sol\\\":{\\\"keccak256\\\":\\\"0xcdfd8db76b2a3415620e4d18cc5545f3d50de792dbf2c3dd5adb40cbe6f94b10\\\",\\\"license\\\":\\\"AGPL-3.0-only\\\",\\\"urls\\\":[\\\"bzz-raw://57b3ab70cde374af1cf2c9888636e8de6cf660f087b1c9abd805e9271e19fa35\\\",\\\"dweb:/ipfs/QmNrLDBAHYFjpjSd12jerm1AdBkDqEYUUaXgnT854BUZ97\\\"]},\\\"lib/solmate/src/utils/FixedPointMathLib.sol\\\":{\\\"keccak256\\\":\\\"0x1b62af9baf5b8e991ed7531bc87f45550ba9d61e8dbff5caf237ccaf3a3fd843\\\",\\\"license\\\":\\\"AGPL-3.0-only\\\",\\\"urls\\\":[\\\"bzz-raw://b7b38b977c5305b18ceefbeed4c9ceaaaefa419b520de62de6604ea661f8c0a9\\\",\\\"dweb:/ipfs/QmecMRzgfMyDVa2pvBqMMDLYBappaj7Aa3qcMoQYEQrhWi\\\"]},\\\"lib/solmate/src/utils/SafeTransferLib.sol\\\":{\\\"keccak256\\\":\\\"0xbadf3d708cf532b12f75f78a1d423135954b63774a6d4ba15914a551d348db8a\\\",\\\"license\\\":\\\"AGPL-3.0-only\\\",\\\"urls\\\":[\\\"bzz-raw://88ac8256bd520d1b8e6f9c4ac9e8777bffdc4a6c8afb1a848f596665779a55b4\\\",\\\"dweb:/ipfs/QmXx7X1dxe6f5VM91vgQ5BA4r2eF97GWDcQDrgHytcvfjU\\\"]},\\\"lib/v3-core/contracts/interfaces/IUniswapV3Pool.sol\\\":{\\\"keccak256\\\":\\\"0x4e64844c56061cd90e0a80de73534a9166704c43eed579eb83f90bc2780ce968\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://cba4fab5cebdddf644b901994a7f0f52b98885d4c56012f4dc51d52c2bf9de0e\\\",\\\"dweb:/ipfs/QmVyyrRmqXrAiapewWunRVgiPVFJHpH2hKiE1py1svMSNV\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolActions.sol\\\":{\\\"keccak256\\\":\\\"0x9453dd0e7442188667d01d9b65de3f1e14e9511ff3e303179a15f6fc267f7634\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://982f4328f956c3e60e67501e759eb292ac487f76460c774c50e9ae4fcc92aae5\\\",\\\"dweb:/ipfs/QmRnzEDsaqtd9PJEVcgQi7p5aV5pMSvRUoGZJAdwFUJxgZ\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolDerivedState.sol\\\":{\\\"keccak256\\\":\\\"0xe603ac5b17ecdee73ba2b27efdf386c257a19c14206e87eee77e2017b742d9e5\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://8febc9bdb399a4d94bb89f5377732652e2400e4a8dee808201ade6848f9004e7\\\",\\\"dweb:/ipfs/QmaKDqYYFU4d2W2iN77aDHptfbFmYZRrMYXHeGpJmM8C1c\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolErrors.sol\\\":{\\\"keccak256\\\":\\\"0xf80abf13fb1fafc127ba4e792f240dd8ea7c8c893978cdfd8439c27fae9a037b\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://b04fc72a656bbf3631e9c2e67b9870a2d9d235185e833fe050e9606e6816a9aa\\\",\\\"dweb:/ipfs/QmUcz4bkEkJ9pwzFu1C3n97hBQ3st9U6qTAqCdyFwddKco\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolEvents.sol\\\":{\\\"keccak256\\\":\\\"0x8071514d0fe5d17d6fbd31c191cdfb703031c24e0ece3621d88ab10e871375cd\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://d0b571930cc7488b1d546a7e9cea7c52d8b3c4e207da657ed0e0db7343b8cd03\\\",\\\"dweb:/ipfs/QmaGK6vVwB95QSTR1XMYvrh7ivYAYZxi3fD7v6VMA4jZ39\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolImmutables.sol\\\":{\\\"keccak256\\\":\\\"0xf6e5d2cd1139c4c276bdbc8e1d2b256e456c866a91f1b868da265c6d2685c3f7\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://b99c8c9ae8e27ee6559e5866bea82cbc9ffc8247f8d15b7422a4deb287d4d047\\\",\\\"dweb:/ipfs/QmfL8gaqt3ffAnm6nVj5ksuNpLygXuL3xq5VBqrkwC2JJ3\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolOwnerActions.sol\\\":{\\\"keccak256\\\":\\\"0x759b78a2918af9e99e246dc3af084f654e48ef32bb4e4cb8a966aa3dcaece235\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://64144fb96e1c7fdba87305acadb98a198d26a3d46c097cb3a666e567f6f29735\\\",\\\"dweb:/ipfs/QmUnWVwN9FKB9uV5Pr8YfLpWZnYM2DENnRMaadZ492JS9u\\\"]},\\\"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolState.sol\\\":{\\\"keccak256\\\":\\\"0x44fa2ce1182f6c2f6bead3d1737804bf7e112252ae86e0f2e92f9b8249603f43\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://54154e8665b98d65f5dc91b256551852cb47882180b501b260657122d207c0ea\\\",\\\"dweb:/ipfs/QmcfemK1A2PXYrWB5SBFGERpMCderbFRb8BtTzQDj1sUBp\\\"]},\\\"lib/v3-core/contracts/libraries/FullMath.sol\\\":{\\\"keccak256\\\":\\\"0x7825565a4bb2a34a1dc96bbfead755785dfb0df8ef81bd934c43023689685645\\\",\\\"license\\\":\\\"MIT\\\",\\\"urls\\\":[\\\"bzz-raw://8f44f4614d31e3d4864c7eb13620555253b84f6a69180f8745b7c6e246a9d125\\\",\\\"dweb:/ipfs/QmfNQUcXj3KL8h9u5PqbtEC6yRtwDbKNb48uMPjdwxsKnd\\\"]},\\\"lib/v3-core/contracts/libraries/TickMath.sol\\\":{\\\"keccak256\\\":\\\"0x5c57de03a91cc2ec8939865dbbcb0197bb6c353b711075eefd8e0fca5e102129\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://1e994c24fd891ef8a3f5dbf5eba42b34feaf05e0376a29f91322faa18054449c\\\",\\\"dweb:/ipfs/QmNdUJGUQxd1dPkMbnA5f5UNqakxRkQE5r7bTZJAuHeapS\\\"]},\\\"lib/v3-periphery/contracts/libraries/OracleLibrary.sol\\\":{\\\"keccak256\\\":\\\"0xe313f89c69c0f1c91f0722868313b4ceb14479b3e7a0abf52a1b9bbd9c18e81b\\\",\\\"license\\\":\\\"GPL-2.0-or-later\\\",\\\"urls\\\":[\\\"bzz-raw://8ad09b061f3f533c116618c20cf01510108eebebb7ff8847b0f245b8ba368d53\\\",\\\"dweb:/ipfs/QmU6vdi5JjJfjK2KqonWvRg9NeuQWarj4B1YFN22Dh9VJM\\\"]},\\\"src/Registry.sol\\\":{\\\"keccak256\\\":\\\"0x2f5f6d61ffc1c9336c628a2cff52b424377feb20c3390f37418fbef1c8995edd\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://1eabb4a5f6bc4ea2c407f3a70efefae52ebe43a5c200cbf828d0e0a3ed676af2\\\",\\\"dweb:/ipfs/QmT6nEdMQx6WzTj8x8ZLayH6hpK5NyXSb97vu2juQ4kS4h\\\"]},\\\"src/base/Cellar.sol\\\":{\\\"keccak256\\\":\\\"0xc5c29f881503e070d2ccc098a7a46d9b03dd47144ae213cc086add626fda9d1a\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://5cf3ddc46f4ac3fe22ad9b1a54e2da8f7c7080d17d9bab1f0483eb0e425c4744\\\",\\\"dweb:/ipfs/QmNRfJvUqSkzSUX5KjiFygjr8sKHkVvERTy7W9LZ9DNc6q\\\"]},\\\"src/base/permutations/CellarWithShareLockPeriod.sol\\\":{\\\"keccak256\\\":\\\"0x619f0e7139aaf886ee9610f29d4ccdec5272cf366c038d680afc34ffd01df50d\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://82a7a2f85e29178dc28c58a17ff5ab5f19665ec533426ae71ad8c8b3068f6580\\\",\\\"dweb:/ipfs/Qmf6741axQbRFUvayW1uqMJ5BHdXS32wXVa7jizWtt2PJv\\\"]},\\\"src/interfaces/external/IChainlinkAggregator.sol\\\":{\\\"keccak256\\\":\\\"0x6cabe293cd867cfd1b4e5c378f08aac66951cbdfacbd37627c2fe5c02661808f\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://87b4e658a11189c690571f456a47fc644ff15926bdf634c66d05a1f711bea3ac\\\",\\\"dweb:/ipfs/QmUDdZ8YxHEt3dTk1vWmNVDQFRp3kEqhwUUZEnRS8Sqb3u\\\"]},\\\"src/interfaces/external/UniswapV3Pool.sol\\\":{\\\"keccak256\\\":\\\"0xf6a71bfbc0665b1d0aad7148b685f3b2254e1be2a3672c2c6671b7fc20db26bb\\\",\\\"urls\\\":[\\\"bzz-raw://393871fdf75656c49a3bb83bc8110a93af2cbd805df9cc6574a058f3cbf100a9\\\",\\\"dweb:/ipfs/QmZugnPctGBiWkFjXXtiKy3AQTWphA9tjZmSBVZRMg9urh\\\"]},\\\"src/modules/adaptors/BaseAdaptor.sol\\\":{\\\"keccak256\\\":\\\"0x82433df76f3d9eb1bf6c5c001fafd263a3f9ebc12c44b90291cf6aa71ff9e91d\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://a297535259dfe38aaf94e95c290ec172758f3ede175b9b1f5e0f06405b1f23a6\\\",\\\"dweb:/ipfs/Qmd8xS3DkUFnzcpqeGB8TEyHYYHUiFQ6L7jdKiEaxZ7bHT\\\"]},\\\"src/modules/price-router/Extensions/Extension.sol\\\":{\\\"keccak256\\\":\\\"0x974e612d503d25b2a0c2e424dd9743ba605a3b3660bd69cafbeb143dabdb3101\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://bf97ce81e995cd034dd814ae1a04832a4864b22d0372c07ecc31779643d9e3ad\\\",\\\"dweb:/ipfs/Qmauuf11TpCS2rZ7SU4iexnkuSBPJWmyF6nisSXseTqPF4\\\"]},\\\"src/modules/price-router/PriceRouter.sol\\\":{\\\"keccak256\\\":\\\"0x6e2e31227c601a10289dbbe7ae7a96adebcf3a62065b9b9d9edbcb4f57cc79dd\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://25b1132fee46f95204107215d5d82eb1ffc0ceebfd360ac9495d43451861510e\\\",\\\"dweb:/ipfs/QmWsY9usC61zsdAzafPEvmrv44vUMyCtzAtEESr3ioDTN1\\\"]},\\\"src/utils/Math.sol\\\":{\\\"keccak256\\\":\\\"0x8cc188510f9657ad17e7903cfeded703dd36901ef39069dd6019ffb7e9cfb8bc\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://d1a66351eb53f333f62652f62c754f4c23ad76c98093e39265d06b84aef165c1\\\",\\\"dweb:/ipfs/QmZNqeAaVwAvZosvKdwUFvnjHsYUcJDwDfDupEZ8uKx949\\\"]},\\\"src/utils/Uint32Array.sol\\\":{\\\"keccak256\\\":\\\"0xd887a816c5b1b66163cab06aa453e0a44e734e2d90484018d0b602ed2f42adb7\\\",\\\"license\\\":\\\"Apache-2.0\\\",\\\"urls\\\":[\\\"bzz-raw://99cfce0b643a810adfa861893a485e786f76be344a3c6c9348a0791f9f225218\\\",\\\"dweb:/ipfs/QmPWf4GbBe6xqUuZJFLYp5Gwv9KTVc3h5phYEb4BMh7fxf\\\"]}},\\\"version\\\":1}\",\n  \"metadata\": {\n    \"compiler\": {\n      \"version\": \"0.8.21+commit.d9974bed\"\n    },\n    \"language\": \"Solidity\",\n    \"output\": {\n      \"abi\": [\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"_owner\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"contract Registry\",\n              \"name\": \"_registry\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"contract ERC20\",\n              \"name\": \"_asset\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"string\",\n              \"name\": \"_name\",\n              \"type\": \"string\"\n            },\n            {\n              \"internalType\": \"string\",\n              \"name\": \"_symbol\",\n              \"type\": \"string\"\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"_holdingPosition\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"_holdingPositionConfig\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"_initialDeposit\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"_strategistPlatformCut\",\n              \"type\": \"uint64\"\n            },\n            {\n              \"internalType\": \"uint192\",\n              \"name\": \"_shareSupplyCap\",\n              \"type\": \"uint192\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"constructor\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"asset\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"expectedAsset\",\n              \"type\": \"address\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__AssetMismatch\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__CallToAdaptorNotAllowed\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__CallerNotApprovedToRebalance\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ContractNotShutdown\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ContractShutdown\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__DebtMismatch\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ExpectedAddressDoesNotMatchActual\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__FailedToForceOutPosition\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"illiquidPosition\",\n              \"type\": \"address\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__IlliquidWithdraw\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assetsOwed\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__IncompleteWithdraw\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidFee\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidFeeCut\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidHoldingPosition\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"requested\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"max\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidRebalanceDeviation\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidShareLockPeriod\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__InvalidShareSupplyCap\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__MinimumConstructorMintNotMet\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"depositor\",\n              \"type\": \"address\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__NotApprovedToDepositOnBehalf\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__Paused\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__PositionAlreadyUsed\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"maxPositions\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__PositionArrayFull\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"sharesRemaining\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__PositionNotEmpty\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__PositionNotInCatalogue\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__PositionNotUsed\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__RemovingHoldingPosition\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__SettingValueToRegistryIdZeroIsProhibited\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ShareSupplyCapExceeded\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"timeSharesAreUnlocked\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"currentBlock\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__SharesAreLocked\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"min\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"max\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__TotalAssetDeviatedOutsideRange\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"current\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"expected\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"type\": \"error\",\n          \"name\": \"Cellar__TotalSharesMustRemainConstant\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ZeroAssets\"\n        },\n        {\n          \"inputs\": [],\n          \"type\": \"error\",\n          \"name\": \"Cellar__ZeroShares\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"data\",\n              \"type\": \"bytes\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"AdaptorCalled\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inCatalogue\",\n              \"type\": \"bool\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"AdaptorCatalogueAltered\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"spender\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"Approval\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"newAutomationActions\",\n              \"type\": \"address\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"Cellar__AutomationActionsUpdated\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"caller\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"Deposit\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"user\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"newOwner\",\n              \"type\": \"address\",\n              \"indexed\": true\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"OwnershipTransferred\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"index\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"PositionAdded\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inCatalogue\",\n              \"type\": \"bool\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"PositionCatalogueAltered\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"position\",\n              \"type\": \"uint32\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"index\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"PositionRemoved\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"newPosition1\",\n              \"type\": \"uint32\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"newPosition2\",\n              \"type\": \"uint32\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"index1\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"index2\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"PositionSwapped\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"oldDeviation\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"newDeviation\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"RebalanceDeviationChanged\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"oldPeriod\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"newPeriod\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"ShareLockingPeriodChanged\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"isShutdown\",\n              \"type\": \"bool\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"ShutdownChanged\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"oldPayoutAddress\",\n              \"type\": \"address\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"newPayoutAddress\",\n              \"type\": \"address\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"StrategistPayoutAddressChanged\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"oldPlatformCut\",\n              \"type\": \"uint64\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"newPlatformCut\",\n              \"type\": \"uint64\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"StrategistPlatformCutChanged\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"from\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"to\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"Transfer\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"caller\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"receiver\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\",\n              \"indexed\": true\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\",\n              \"indexed\": false\n            }\n          ],\n          \"type\": \"event\",\n          \"name\": \"Withdraw\",\n          \"anonymous\": false\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"DOMAIN_SEPARATOR\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bytes32\",\n              \"name\": \"\",\n              \"type\": \"bytes32\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"GRAVITY_BRIDGE_REGISTRY_SLOT\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MAX_FEE_CUT\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"\",\n              \"type\": \"uint64\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MAX_PLATFORM_FEE\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"\",\n              \"type\": \"uint64\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MAX_POSITIONS\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MAX_REBALANCE_DEVIATION\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"\",\n              \"type\": \"uint64\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"MINIMUM_SHARE_LOCK_PERIOD\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"PRICE_ROUTER_REGISTRY_SLOT\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"adaptorCatalogue\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"addAdaptorToCatalogue\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"index\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"configurationData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inDebtArray\",\n              \"type\": \"bool\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"addPosition\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"addPositionToCatalogue\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"allowance\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"allowedRebalanceDeviation\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"spender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"approve\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"asset\",\n          \"outputs\": [\n            {\n              \"internalType\": \"contract ERC20\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"automationActions\",\n          \"outputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"balanceOf\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"blockExternalReceiver\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"checkTotalAssets\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"uint16\",\n              \"name\": \"allowableRange\",\n              \"type\": \"uint16\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"expectedPriceRouter\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"cachePriceRouter\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"struct Cellar.AdaptorCall[]\",\n              \"name\": \"data\",\n              \"type\": \"tuple[]\",\n              \"components\": [\n                {\n                  \"internalType\": \"address\",\n                  \"name\": \"adaptor\",\n                  \"type\": \"address\"\n                },\n                {\n                  \"internalType\": \"bytes[]\",\n                  \"name\": \"callData\",\n                  \"type\": \"bytes[]\"\n                }\n              ]\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"callOnAdaptor\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"convertToAssets\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"convertToShares\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"creditPositions\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"\",\n              \"type\": \"uint32\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"debtPositions\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"\",\n              \"type\": \"uint32\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"decimals\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint8\",\n              \"name\": \"\",\n              \"type\": \"uint8\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint192\",\n              \"name\": \"_newShareSupplyCap\",\n              \"type\": \"uint192\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"decreaseShareSupplyCap\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"receiver\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"deposit\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"feeData\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"strategistPlatformCut\",\n              \"type\": \"uint64\"\n            },\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"platformFee\",\n              \"type\": \"uint64\"\n            },\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"lastAccrual\",\n              \"type\": \"uint64\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"strategistPayoutAddress\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"index\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inDebtArray\",\n              \"type\": \"bool\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"forcePositionOut\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"getCreditPositions\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint32[]\",\n              \"name\": \"\",\n              \"type\": \"uint32[]\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"getDebtPositions\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint32[]\",\n              \"name\": \"\",\n              \"type\": \"uint32[]\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"getPositionData\",\n          \"outputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"isDebt\",\n              \"type\": \"bool\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"adaptorData\",\n              \"type\": \"bytes\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"configurationData\",\n              \"type\": \"bytes\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"holdingPosition\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"\",\n              \"type\": \"uint32\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"ignorePause\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint192\",\n              \"name\": \"_newShareSupplyCap\",\n              \"type\": \"uint192\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"increaseShareSupplyCap\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"initiateShutdown\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"isPaused\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"isPositionUsed\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"isShutdown\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"liftShutdown\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"locked\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"maxDeposit\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"maxMint\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"maxRedeem\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"maxWithdraw\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"receiver\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"mint\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"bytes[]\",\n              \"name\": \"data\",\n              \"type\": \"bytes[]\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"multicall\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"name\",\n          \"outputs\": [\n            {\n              \"internalType\": \"string\",\n              \"name\": \"\",\n              \"type\": \"string\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"nonces\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"bytes\",\n              \"name\": \"\",\n              \"type\": \"bytes\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"onERC721Received\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bytes4\",\n              \"name\": \"\",\n              \"type\": \"bytes4\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"owner\",\n          \"outputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"spender\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"value\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"deadline\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"uint8\",\n              \"name\": \"v\",\n              \"type\": \"uint8\"\n            },\n            {\n              \"internalType\": \"bytes32\",\n              \"name\": \"r\",\n              \"type\": \"bytes32\"\n            },\n            {\n              \"internalType\": \"bytes32\",\n              \"name\": \"s\",\n              \"type\": \"bytes32\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"permit\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"positionCatalogue\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"previewDeposit\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"previewMint\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"previewRedeem\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"previewWithdraw\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"priceRouter\",\n          \"outputs\": [\n            {\n              \"internalType\": \"contract PriceRouter\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"receiver\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"redeem\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"registry\",\n          \"outputs\": [\n            {\n              \"internalType\": \"contract Registry\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"removeAdaptorFromCatalogue\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"index\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inDebtArray\",\n              \"type\": \"bool\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"removePosition\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"removePositionFromCatalogue\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"_registryId\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"_expectedAutomationActions\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setAutomationActions\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"positionId\",\n              \"type\": \"uint32\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setHoldingPosition\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"newDeviation\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setRebalanceDeviation\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"newLock\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setShareLockPeriod\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"payout\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setStrategistPayoutAddress\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint64\",\n              \"name\": \"cut\",\n              \"type\": \"uint64\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"setStrategistPlatformCut\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"shareLockPeriod\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"shareSupplyCap\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint192\",\n              \"name\": \"\",\n              \"type\": \"uint192\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"index1\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"uint32\",\n              \"name\": \"index2\",\n              \"type\": \"uint32\"\n            },\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"inDebtArray\",\n              \"type\": \"bool\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"swapPositions\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"symbol\",\n          \"outputs\": [\n            {\n              \"internalType\": \"string\",\n              \"name\": \"\",\n              \"type\": \"string\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"toggleIgnorePause\"\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"totalAssets\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"totalAssetsWithdrawable\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"totalSupply\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"to\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"transfer\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"from\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"to\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"amount\",\n              \"type\": \"uint256\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"transferFrom\",\n          \"outputs\": [\n            {\n              \"internalType\": \"bool\",\n              \"name\": \"\",\n              \"type\": \"bool\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"newOwner\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"transferOwnership\"\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"userShareLockStartTime\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"\",\n              \"type\": \"uint256\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [],\n          \"stateMutability\": \"view\",\n          \"type\": \"function\",\n          \"name\": \"viewPositionBalances\",\n          \"outputs\": [\n            {\n              \"internalType\": \"contract ERC20[]\",\n              \"name\": \"assets\",\n              \"type\": \"address[]\"\n            },\n            {\n              \"internalType\": \"uint256[]\",\n              \"name\": \"balances\",\n              \"type\": \"uint256[]\"\n            },\n            {\n              \"internalType\": \"bool[]\",\n              \"name\": \"isDebt\",\n              \"type\": \"bool[]\"\n            }\n          ]\n        },\n        {\n          \"inputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"assets\",\n              \"type\": \"uint256\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"receiver\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"address\",\n              \"name\": \"owner\",\n              \"type\": \"address\"\n            }\n          ],\n          \"stateMutability\": \"nonpayable\",\n          \"type\": \"function\",\n          \"name\": \"withdraw\",\n          \"outputs\": [\n            {\n              \"internalType\": \"uint256\",\n              \"name\": \"shares\",\n              \"type\": \"uint256\"\n            }\n          ]\n        }\n      ],\n      \"devdoc\": {\n        \"kind\": \"dev\",\n        \"methods\": {\n          \"addAdaptorToCatalogue(address)\": {\n            \"details\": \"Callable by Sommelier Governance.\"\n          },\n          \"addPosition(uint32,uint32,bytes,bool)\": {\n            \"details\": \"Callable by Sommelier Strategist.\",\n            \"params\": {\n              \"configurationData\": \"data used to configure how the position behaves\",\n              \"index\": \"index at which to insert the position\",\n              \"positionId\": \"id of position to add\"\n            }\n          },\n          \"addPositionToCatalogue(uint32)\": {\n            \"details\": \"Callable by Sommelier Governance.\"\n          },\n          \"cachePriceRouter(bool,uint16,address)\": {\n            \"details\": \"`allowableRange` reverts from arithmetic underflow if it is greater than 10_000, this is      desired behavior.Callable by Sommelier Governance.\",\n            \"params\": {\n              \"allowableRange\": \"The +- range the total assets may deviate between the old and new price router.                       - 1_000 == 10%                       - 500 == 5%\",\n              \"checkTotalAssets\": \"If true totalAssets is checked before and after updating the price router,        and is verified to be withing a +- 5% envelope.        If false totalAssets is only called after updating the price router.]\",\n              \"expectedPriceRouter\": \"The registry price router differed from the expected price router.\"\n            }\n          },\n          \"callOnAdaptor((address,bytes[])[])\": {\n            \"details\": \"There are several safety checks in this function to prevent strategists from abusing it.      - `blockExternalReceiver`      - `totalAssets` must not change by much      - `totalShares` must remain constant      - adaptors must be set up to be used with this cellarSince `totalAssets` is allowed to deviate slightly, strategists could abuse this by sending      multiple `callOnAdaptor` calls rapidly, to gradually change the share price.      To mitigate this, rate limiting will be put in place on the Sommelier side.Callable by Sommelier Strategist, and Automation Actions contract.\"\n          },\n          \"convertToAssets(uint256)\": {\n            \"details\": \"Use preview functions to get accurate assets.Under estimates assets.\",\n            \"params\": {\n              \"shares\": \"amount of shares to convert\"\n            },\n            \"returns\": {\n              \"assets\": \"the shares can be exchanged for\"\n            }\n          },\n          \"convertToShares(uint256)\": {\n            \"details\": \"Use preview functions to get accurate shares.Under estimates shares.\",\n            \"params\": {\n              \"assets\": \"amount of assets to convert\"\n            },\n            \"returns\": {\n              \"shares\": \"the assets can be exchanged for\"\n            }\n          },\n          \"decreaseShareSupplyCap(uint192)\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"deposit(uint256,address)\": {\n            \"params\": {\n              \"assets\": \"amount of assets deposited by user.\",\n              \"receiver\": \"address to receive the shares.\"\n            },\n            \"returns\": {\n              \"shares\": \"amount of shares given for deposit.\"\n            }\n          },\n          \"forcePositionOut(uint32,uint32,bool)\": {\n            \"details\": \"Callable by Sommelier Governance.\"\n          },\n          \"increaseShareSupplyCap(uint192)\": {\n            \"details\": \"Callable by Sommelier Governance.\"\n          },\n          \"initiateShutdown()\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"liftShutdown()\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"maxDeposit(address)\": {\n            \"returns\": {\n              \"_0\": \"assets maximum amount of assets that can be deposited\"\n            }\n          },\n          \"maxMint(address)\": {\n            \"returns\": {\n              \"_0\": \"shares maximum amount of shares that can be minted\"\n            }\n          },\n          \"maxRedeem(address)\": {\n            \"details\": \"EIP4626 states maxRedeem must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.\",\n            \"params\": {\n              \"owner\": \"address to check maxRedeem of.\"\n            },\n            \"returns\": {\n              \"_0\": \"the max amount of shares redeemable by `owner`.\"\n            }\n          },\n          \"maxWithdraw(address)\": {\n            \"details\": \"EIP4626 states maxWithdraw must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.\",\n            \"params\": {\n              \"owner\": \"address to check maxWithdraw of.\"\n            },\n            \"returns\": {\n              \"_0\": \"the max amount of assets withdrawable by `owner`.\"\n            }\n          },\n          \"mint(uint256,address)\": {\n            \"params\": {\n              \"receiver\": \"address to receive the shares.\",\n              \"shares\": \"amount of shares requested by user.\"\n            },\n            \"returns\": {\n              \"assets\": \"amount of assets deposited into the cellar.\"\n            }\n          },\n          \"multicall(bytes[])\": {\n            \"details\": \"Does NOT return the function return values.\"\n          },\n          \"onERC721Received(address,address,uint256,bytes)\": {\n            \"details\": \"See {IERC721Receiver-onERC721Received}. Always returns `IERC721Receiver.onERC721Received.selector`.\"\n          },\n          \"previewDeposit(uint256)\": {\n            \"params\": {\n              \"assets\": \"amount of assets to deposit\"\n            },\n            \"returns\": {\n              \"shares\": \"that will be minted\"\n            }\n          },\n          \"previewMint(uint256)\": {\n            \"params\": {\n              \"shares\": \"amount of shares to mint\"\n            },\n            \"returns\": {\n              \"assets\": \"that will be deposited\"\n            }\n          },\n          \"previewRedeem(uint256)\": {\n            \"params\": {\n              \"shares\": \"amount of shares to redeem\"\n            },\n            \"returns\": {\n              \"assets\": \"that will be returned\"\n            }\n          },\n          \"previewWithdraw(uint256)\": {\n            \"params\": {\n              \"assets\": \"amount of assets to withdraw\"\n            },\n            \"returns\": {\n              \"shares\": \"that will be redeemed\"\n            }\n          },\n          \"redeem(uint256,address,address)\": {\n            \"details\": \"Unlike conventional ERC4626 contracts, this may not always return one asset to the receiver.      Since there are no swaps involved in this function, the receiver may receive multiple      assets. The value of all the assets returned will be equal to the amount defined by      `assets` denominated in the `asset` of the cellar (eg. if `asset` is USDC and `assets`      is 1000, then the receiver will receive $1000 worth of assets in either one or many      tokens).\",\n            \"params\": {\n              \"owner\": \"address that owns the shares being redeemed\",\n              \"receiver\": \"address that will receive withdrawn assets\",\n              \"shares\": \"amount of shares to redeem\"\n            },\n            \"returns\": {\n              \"assets\": \"equivalent value of the assets withdrawn, denominated in the cellar's asset\"\n            }\n          },\n          \"removeAdaptorFromCatalogue(address)\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"removePosition(uint32,bool)\": {\n            \"details\": \"Called by strategist.Callable by Sommelier Strategist.\",\n            \"params\": {\n              \"index\": \"index at which to remove the position\"\n            }\n          },\n          \"removePositionFromCatalogue(uint32)\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"setAutomationActions(uint256,address)\": {\n            \"details\": \"Callable by Sommelier Governance.\",\n            \"params\": {\n              \"_expectedAutomationActions\": \"The registry automation actions differed from the expected automation actions.\",\n              \"_registryId\": \"Registry Id to get the automation action.\"\n            }\n          },\n          \"setHoldingPosition(uint32)\": {\n            \"details\": \"Callable by Sommelier Strategist.\"\n          },\n          \"setRebalanceDeviation(uint256)\": {\n            \"details\": \"Callable by Sommelier Governance.\",\n            \"params\": {\n              \"newDeviation\": \"the new rebalance deviation value.\"\n            }\n          },\n          \"setShareLockPeriod(uint256)\": {\n            \"details\": \"Callable by Sommelier Strategist.\",\n            \"params\": {\n              \"newLock\": \"the new lock period\"\n            }\n          },\n          \"setStrategistPayoutAddress(address)\": {\n            \"details\": \"Callable by Sommelier Strategist.\",\n            \"params\": {\n              \"payout\": \"the new strategist payout address\"\n            }\n          },\n          \"setStrategistPlatformCut(uint64)\": {\n            \"details\": \"Callable by Sommelier Governance.\",\n            \"params\": {\n              \"cut\": \"the platform cut for the strategist\"\n            }\n          },\n          \"swapPositions(uint32,uint32,bool)\": {\n            \"details\": \"Callable by Sommelier Strategist.\",\n            \"params\": {\n              \"inDebtArray\": \"bool indicating to switch positions in the debt array, or the credit array.\",\n              \"index1\": \"index of first position to swap\",\n              \"index2\": \"index of second position to swap\"\n            }\n          },\n          \"toggleIgnorePause()\": {\n            \"details\": \"Callable by Sommelier Governance.\"\n          },\n          \"totalAssets()\": {\n            \"details\": \"EIP4626 states totalAssets needs to be inclusive of fees. Since performance fees mint shares, total assets remains unchanged, so this implementation is inclusive of fees even though it does not explicitly show it.EIP4626 states totalAssets must not revert, but it is possible for `totalAssets` to revert so it does NOT conform to ERC4626 standards.Run a re-entrancy check because totalAssets can be wrong if re-entering from deposit/withdraws.\"\n          },\n          \"totalAssetsWithdrawable()\": {\n            \"details\": \"Run a re-entrancy check because totalAssetsWithdrawable can be wrong if re-entering from deposit/withdraws.\"\n          },\n          \"withdraw(uint256,address,address)\": {\n            \"details\": \"Unlike conventional ERC4626 contracts, this may not always return one asset to the receiver.      Since there are no swaps involved in this function, the receiver may receive multiple      assets. The value of all the assets returned will be equal to the amount defined by      `assets` denominated in the `asset` of the cellar (eg. if `asset` is USDC and `assets`      is 1000, then the receiver will receive $1000 worth of assets in either one or many      tokens).\",\n            \"params\": {\n              \"assets\": \"equivalent value of the assets withdrawn, denominated in the cellar's asset\",\n              \"owner\": \"address that owns the shares being redeemed\",\n              \"receiver\": \"address that will receive withdrawn assets\"\n            },\n            \"returns\": {\n              \"shares\": \"amount of shares redeemed\"\n            }\n          }\n        },\n        \"version\": 1\n      },\n      \"userdoc\": {\n        \"kind\": \"user\",\n        \"methods\": {\n          \"GRAVITY_BRIDGE_REGISTRY_SLOT()\": {\n            \"notice\": \"Id to get the gravity bridge from the registry.\"\n          },\n          \"MAXIMUM_SHARE_LOCK_PERIOD()\": {\n            \"notice\": \"Shares can be locked for at most 2 days after minting.\"\n          },\n          \"MAX_FEE_CUT()\": {\n            \"notice\": \"Sets the max possible fee cut for this cellar.\"\n          },\n          \"MAX_PLATFORM_FEE()\": {\n            \"notice\": \"Sets the max possible performance fee for this cellar.\"\n          },\n          \"MAX_POSITIONS()\": {\n            \"notice\": \"Maximum amount of positions a cellar can have in it's credit/debt arrays.\"\n          },\n          \"MAX_REBALANCE_DEVIATION()\": {\n            \"notice\": \"Stores the max possible rebalance deviation for this cellar.\"\n          },\n          \"MINIMUM_SHARE_LOCK_PERIOD()\": {\n            \"notice\": \"Shares must be locked for at least 5 minutes after minting.\"\n          },\n          \"PRICE_ROUTER_REGISTRY_SLOT()\": {\n            \"notice\": \"Id to get the price router from the registry.\"\n          },\n          \"adaptorCatalogue(address)\": {\n            \"notice\": \"Adaptors the strategist is approved to use without any governance intervention.\"\n          },\n          \"addAdaptorToCatalogue(address)\": {\n            \"notice\": \"Allows Governance to add adaptors to this cellar's catalogue.\"\n          },\n          \"addPosition(uint32,uint32,bytes,bool)\": {\n            \"notice\": \"Insert a trusted position to the list of positions used by the cellar at a given index.\"\n          },\n          \"addPositionToCatalogue(uint32)\": {\n            \"notice\": \"Allows Governance to add positions to this cellar's catalogue.\"\n          },\n          \"allowedRebalanceDeviation()\": {\n            \"notice\": \"The percent the total assets of a cellar may deviate during a `callOnAdaptor`(rebalance) call.\"\n          },\n          \"automationActions()\": {\n            \"notice\": \"The Automation Actions contract that can rebalance this Cellar.\"\n          },\n          \"blockExternalReceiver()\": {\n            \"notice\": \"This bool is used to stop strategists from abusing Base Adaptor functions(deposit/withdraw).\"\n          },\n          \"cachePriceRouter(bool,uint16,address)\": {\n            \"notice\": \"Updates the cellar to use the lastest price router in the registry.\"\n          },\n          \"callOnAdaptor((address,bytes[])[])\": {\n            \"notice\": \"Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.\"\n          },\n          \"convertToAssets(uint256)\": {\n            \"notice\": \"The amount of assets that the cellar would exchange for the amount of shares provided.\"\n          },\n          \"convertToShares(uint256)\": {\n            \"notice\": \"The amount of shares that the cellar would exchange for the amount of assets provided.\"\n          },\n          \"creditPositions(uint256)\": {\n            \"notice\": \"Array of uint32s made up of cellars credit positions Ids.\"\n          },\n          \"debtPositions(uint256)\": {\n            \"notice\": \"Array of uint32s made up of cellars debt positions Ids.\"\n          },\n          \"decreaseShareSupplyCap(uint192)\": {\n            \"notice\": \"Decreases the share supply cap.\"\n          },\n          \"deposit(uint256,address)\": {\n            \"notice\": \"Deposits assets into the cellar, and returns shares to receiver.\"\n          },\n          \"feeData()\": {\n            \"notice\": \"Stores all fee data for cellar.\"\n          },\n          \"forcePositionOut(uint32,uint32,bool)\": {\n            \"notice\": \"Allows Sommelier Governance to forceably remove a position from the Cellar without checking its balance is zero.\"\n          },\n          \"getCreditPositions()\": {\n            \"notice\": \"Get the ids of the credit positions currently used by the cellar.\"\n          },\n          \"getDebtPositions()\": {\n            \"notice\": \"Get the ids of the debt positions currently used by the cellar.\"\n          },\n          \"getPositionData(uint32)\": {\n            \"notice\": \"Get position data given position id.\"\n          },\n          \"holdingPosition()\": {\n            \"notice\": \"Stores the position id of the holding position in the creditPositions array.\"\n          },\n          \"ignorePause()\": {\n            \"notice\": \"Pauses all user entry/exits, and strategist rebalances.\"\n          },\n          \"increaseShareSupplyCap(uint192)\": {\n            \"notice\": \"Increases the share supply cap.\"\n          },\n          \"initiateShutdown()\": {\n            \"notice\": \"Shutdown the cellar. Used in an emergency or if the cellar has been deprecated.\"\n          },\n          \"isPaused()\": {\n            \"notice\": \"View function external contracts can use to see if the cellar is paused.\"\n          },\n          \"isPositionUsed(uint256)\": {\n            \"notice\": \"Tell whether a position is currently used.\"\n          },\n          \"isShutdown()\": {\n            \"notice\": \"Whether or not the contract is shutdown in case of an emergency.\"\n          },\n          \"liftShutdown()\": {\n            \"notice\": \"Restart the cellar.\"\n          },\n          \"locked()\": {\n            \"notice\": \"`locked` is public, so that the state can be checked even during view function calls.\"\n          },\n          \"maxDeposit(address)\": {\n            \"notice\": \"Total amount of assets that can be deposited for a user.\"\n          },\n          \"maxMint(address)\": {\n            \"notice\": \"Total amount of shares that can be minted for a user.\"\n          },\n          \"maxRedeem(address)\": {\n            \"notice\": \"Returns the max amount shares redeemable by a user\"\n          },\n          \"maxWithdraw(address)\": {\n            \"notice\": \"Returns the max amount withdrawable by a user inclusive of performance fees\"\n          },\n          \"mint(uint256,address)\": {\n            \"notice\": \"Mints shares from the cellar, and returns shares to receiver.\"\n          },\n          \"multicall(bytes[])\": {\n            \"notice\": \"Allows caller to call multiple functions in a single TX.\"\n          },\n          \"positionCatalogue(uint32)\": {\n            \"notice\": \"Positions the strategist is approved to use without any governance intervention.\"\n          },\n          \"previewDeposit(uint256)\": {\n            \"notice\": \"Simulate the effects of depositing assets at the current block, given current on-chain conditions.\"\n          },\n          \"previewMint(uint256)\": {\n            \"notice\": \"Simulate the effects of minting shares at the current block, given current on-chain conditions.\"\n          },\n          \"previewRedeem(uint256)\": {\n            \"notice\": \"Simulate the effects of redeeming shares at the current block, given current on-chain conditions.\"\n          },\n          \"previewWithdraw(uint256)\": {\n            \"notice\": \"Simulate the effects of withdrawing assets at the current block, given current on-chain conditions.\"\n          },\n          \"priceRouter()\": {\n            \"notice\": \"Cached price router contract.\"\n          },\n          \"redeem(uint256,address,address)\": {\n            \"notice\": \"Redeem shares to withdraw assets from the cellar.\"\n          },\n          \"registry()\": {\n            \"notice\": \"Address of the platform's registry contract. Used to get the latest address of modules.\"\n          },\n          \"removeAdaptorFromCatalogue(address)\": {\n            \"notice\": \"Allows Governance to remove adaptors from this cellar's catalogue.\"\n          },\n          \"removePosition(uint32,bool)\": {\n            \"notice\": \"Remove the position at a given index from the list of positions used by the cellar.\"\n          },\n          \"removePositionFromCatalogue(uint32)\": {\n            \"notice\": \"Allows Governance to remove positions from this cellar's catalogue.\"\n          },\n          \"setAutomationActions(uint256,address)\": {\n            \"notice\": \"Set the Automation Actions contract.\"\n          },\n          \"setHoldingPosition(uint32)\": {\n            \"notice\": \"Allows owner to change the holding position.\"\n          },\n          \"setRebalanceDeviation(uint256)\": {\n            \"notice\": \"Allows governance to change this cellars rebalance deviation.\"\n          },\n          \"setShareLockPeriod(uint256)\": {\n            \"notice\": \"Allows share lock period to be updated.\"\n          },\n          \"setStrategistPayoutAddress(address)\": {\n            \"notice\": \"Sets the Strategists payout address\"\n          },\n          \"setStrategistPlatformCut(uint64)\": {\n            \"notice\": \"Sets the Strategists cut of platform fees\"\n          },\n          \"shareLockPeriod()\": {\n            \"notice\": \"After deposits users must wait `shareLockPeriod` time before being able to transfer or withdraw their shares.\"\n          },\n          \"shareSupplyCap()\": {\n            \"notice\": \"The maximum amount of shares that can be in circulation.\"\n          },\n          \"swapPositions(uint32,uint32,bool)\": {\n            \"notice\": \"Swap the positions at two given indexes.\"\n          },\n          \"toggleIgnorePause()\": {\n            \"notice\": \"Allows governance to choose whether or not to respect a pause.\"\n          },\n          \"totalAssets()\": {\n            \"notice\": \"The total amount of assets in the cellar.\"\n          },\n          \"totalAssetsWithdrawable()\": {\n            \"notice\": \"The total amount of withdrawable assets in the cellar.\"\n          },\n          \"transfer(address,uint256)\": {\n            \"notice\": \"Override `transfer` to add share lock check.\"\n          },\n          \"transferFrom(address,address,uint256)\": {\n            \"notice\": \"Override `transferFrom` to add share lock check.\"\n          },\n          \"userShareLockStartTime(address)\": {\n            \"notice\": \"mapping that stores every users last time stamp they minted shares.\"\n          },\n          \"viewPositionBalances()\": {\n            \"notice\": \"View the amount of assets in each Cellar Position.\"\n          },\n          \"withdraw(uint256,address,address)\": {\n            \"notice\": \"Withdraw assets from the cellar by redeeming shares.\"\n          }\n        },\n        \"version\": 1\n      }\n    },\n    \"settings\": {\n      \"remappings\": [\n        \"@balancer-labs/=lib/balancer-v2-monorepo/../../node_modules/@balancer-labs/\",\n        \"@balancer/=lib/balancer-v2-monorepo/pkg/\",\n        \"@chainlink/=lib/chainlink/\",\n        \"@ds-test/=lib/forge-std/lib/ds-test/src/\",\n        \"@forge-std/=lib/forge-std/src/\",\n        \"@openzeppelin/=lib/openzeppelin-contracts/\",\n        \"@solmate/=lib/solmate/src/\",\n        \"@uniswap/v3-core/=lib/v3-core/\",\n        \"@uniswap/v3-periphery/=lib/v3-periphery/\",\n        \"@uniswapV3C/=lib/v3-core/contracts/\",\n        \"@uniswapV3P/=lib/v3-periphery/contracts/\",\n        \"axelar-gmp-sdk-solidity/=lib/axelar-gmp-sdk-solidity/contracts/\",\n        \"balancer-v2-monorepo/=lib/balancer-v2-monorepo/\",\n        \"chainlink/=lib/chainlink/integration-tests/contracts/ethereum/src/\",\n        \"ds-test/=lib/forge-std/lib/ds-test/src/\",\n        \"forge-std/=lib/forge-std/src/\",\n        \"openzeppelin-contracts/=lib/openzeppelin-contracts/\",\n        \"solmate/=lib/solmate/src/\",\n        \"v3-core/=lib/v3-core/contracts/\",\n        \"v3-periphery/=lib/v3-periphery/contracts/\"\n      ],\n      \"optimizer\": {\n        \"enabled\": true,\n        \"runs\": 200\n      },\n      \"metadata\": {\n        \"bytecodeHash\": \"ipfs\"\n      },\n      \"compilationTarget\": {\n        \"src/base/permutations/CellarWithShareLockPeriod.sol\": \"CellarWithShareLockPeriod\"\n      },\n      \"libraries\": {}\n    },\n    \"sources\": {\n      \"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorInterface.sol\": {\n        \"keccak256\": \"0xb496651006b9a2a07920ffe116928b11e2a6458e21361cecca51409522488ca7\",\n        \"urls\": [\n          \"bzz-raw://f39ad60071af2c115e064ebeb1686097efa83b26da0e2c814c635538538b7465\",\n          \"dweb:/ipfs/QmYRARVDA1XZUqZNKNnArYHWbffNeeSVZQjt67ZXKGm85a\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorV2V3Interface.sol\": {\n        \"keccak256\": \"0x4a7757ff7bbafe044cd49c2a45c7c18ec50eff7c7af6869face5e1e9cda976f2\",\n        \"urls\": [\n          \"bzz-raw://7c3f481f69f3ee07d6bb91b38d1cd61f9fa72de29c63d778c98956db70ecd57b\",\n          \"dweb:/ipfs/QmPeJrNHTZF8CrXk3BgLJCamwf1dUEzHyQsMYrdd4v1NEG\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol\": {\n        \"keccak256\": \"0x6e6e4b0835904509406b070ee173b5bc8f677c19421b76be38aea3b1b3d30846\",\n        \"urls\": [\n          \"bzz-raw://b3beaa37ee61e4ab615e250fbf01601ae481de843fd0ef55e6b44fd9d5fff8a7\",\n          \"dweb:/ipfs/QmeZUVwd26LzK4Mfp8Zba5JbQNkZFfTzFu1A6FVMMZDg9c\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/access/Ownable.sol\": {\n        \"keccak256\": \"0xa94b34880e3c1b0b931662cb1c09e5dfa6662f31cba80e07c5ee71cd135c9673\",\n        \"urls\": [\n          \"bzz-raw://40fb1b5102468f783961d0af743f91b9980cf66b50d1d12009f6bb1869cea4d2\",\n          \"dweb:/ipfs/QmYqEbJML4jB1GHbzD4cUZDtJg5wVwNm3vDJq1GbyDus8y\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/token/ERC721/IERC721Receiver.sol\": {\n        \"keccak256\": \"0xa82b58eca1ee256be466e536706850163d2ec7821945abd6b4778cfb3bee37da\",\n        \"urls\": [\n          \"bzz-raw://6e75cf83beb757b8855791088546b8337e9d4684e169400c20d44a515353b708\",\n          \"dweb:/ipfs/QmYvPafLfoquiDMEj7CKHtvbgHu7TJNPSVPSCjrtjV8HjV\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/token/ERC721/utils/ERC721Holder.sol\": {\n        \"keccak256\": \"0x0108bf6a6ebd5f96678bed33a35947537263f96766131ee91461fb6485805028\",\n        \"urls\": [\n          \"bzz-raw://ae2d274bf3d56a6d49a9bbd0a4871c54997a82551eb3eb1c0c39dc98698ff8bf\",\n          \"dweb:/ipfs/QmTT7ty5DPGAmRnx94Xu3TUDYGSPDVLN2bppJAjjedrg1e\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/utils/Address.sol\": {\n        \"keccak256\": \"0xb94eac067c85cd79a4195c0a1f4a878e9827329045c12475a0199f1ae17b9700\",\n        \"urls\": [\n          \"bzz-raw://2ad84b5dbf40ba9e944cc25bd0a98c51bafd49cff30efe5ef5aef921a70081de\",\n          \"dweb:/ipfs/Qme8iCeqe9VdNgWktTTsSxUfHcJEXuvPaJpshWDzoWj56V\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/utils/Context.sol\": {\n        \"keccak256\": \"0xe2e337e6dde9ef6b680e07338c493ebea1b5fd09b43424112868e9cc1706bca7\",\n        \"urls\": [\n          \"bzz-raw://6df0ddf21ce9f58271bdfaa85cde98b200ef242a05a3f85c2bc10a8294800a92\",\n          \"dweb:/ipfs/QmRK2Y5Yc6BK7tGKkgsgn3aJEQGi5aakeSPZvS65PV8Xp3\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/openzeppelin-contracts/contracts/utils/math/SafeCast.sol\": {\n        \"keccak256\": \"0x182ad835742e188a50bc98b938287d28bf74ad87d01e2bbc1d207c2ba36e1adb\",\n        \"urls\": [\n          \"bzz-raw://c2ba26b6252bb52b39ffb18b2de027544619e3f71b78e5476eba52becfaae929\",\n          \"dweb:/ipfs/Qmb7NqEzs7aWkrzpskxXCRb799XmPenZMDtpzRvNUph1Bg\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/solmate/src/auth/Owned.sol\": {\n        \"keccak256\": \"0xfedb27d14c508342c33eb067c9a02eabcdb0f9dcf93b04ded1001f580d12d0ea\",\n        \"urls\": [\n          \"bzz-raw://1ff52bbee698b9cf9e4574615e6550be0887ccf355f6571e23d6f25b332e79b4\",\n          \"dweb:/ipfs/QmVorA2apojVRStzS7h8aFccR3Uv32G6HVtBtFHZrE7YXx\"\n        ],\n        \"license\": \"AGPL-3.0-only\"\n      },\n      \"lib/solmate/src/mixins/ERC4626.sol\": {\n        \"keccak256\": \"0xa404f6f45bd53f24a90cc5ffe95e16b52e3f2dfd88f0d7a1edcb35f815919a7b\",\n        \"urls\": [\n          \"bzz-raw://9f01e32d713e05cc58c1563e9938a1c5e096b1da9f52c7ea8424f2317b94adc1\",\n          \"dweb:/ipfs/QmVt5SsbA3kezM5pyovupN7iZLy6QVqY5qQRZKLFqxKJUs\"\n        ],\n        \"license\": \"AGPL-3.0-only\"\n      },\n      \"lib/solmate/src/tokens/ERC20.sol\": {\n        \"keccak256\": \"0xcdfd8db76b2a3415620e4d18cc5545f3d50de792dbf2c3dd5adb40cbe6f94b10\",\n        \"urls\": [\n          \"bzz-raw://57b3ab70cde374af1cf2c9888636e8de6cf660f087b1c9abd805e9271e19fa35\",\n          \"dweb:/ipfs/QmNrLDBAHYFjpjSd12jerm1AdBkDqEYUUaXgnT854BUZ97\"\n        ],\n        \"license\": \"AGPL-3.0-only\"\n      },\n      \"lib/solmate/src/utils/FixedPointMathLib.sol\": {\n        \"keccak256\": \"0x1b62af9baf5b8e991ed7531bc87f45550ba9d61e8dbff5caf237ccaf3a3fd843\",\n        \"urls\": [\n          \"bzz-raw://b7b38b977c5305b18ceefbeed4c9ceaaaefa419b520de62de6604ea661f8c0a9\",\n          \"dweb:/ipfs/QmecMRzgfMyDVa2pvBqMMDLYBappaj7Aa3qcMoQYEQrhWi\"\n        ],\n        \"license\": \"AGPL-3.0-only\"\n      },\n      \"lib/solmate/src/utils/SafeTransferLib.sol\": {\n        \"keccak256\": \"0xbadf3d708cf532b12f75f78a1d423135954b63774a6d4ba15914a551d348db8a\",\n        \"urls\": [\n          \"bzz-raw://88ac8256bd520d1b8e6f9c4ac9e8777bffdc4a6c8afb1a848f596665779a55b4\",\n          \"dweb:/ipfs/QmXx7X1dxe6f5VM91vgQ5BA4r2eF97GWDcQDrgHytcvfjU\"\n        ],\n        \"license\": \"AGPL-3.0-only\"\n      },\n      \"lib/v3-core/contracts/interfaces/IUniswapV3Pool.sol\": {\n        \"keccak256\": \"0x4e64844c56061cd90e0a80de73534a9166704c43eed579eb83f90bc2780ce968\",\n        \"urls\": [\n          \"bzz-raw://cba4fab5cebdddf644b901994a7f0f52b98885d4c56012f4dc51d52c2bf9de0e\",\n          \"dweb:/ipfs/QmVyyrRmqXrAiapewWunRVgiPVFJHpH2hKiE1py1svMSNV\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolActions.sol\": {\n        \"keccak256\": \"0x9453dd0e7442188667d01d9b65de3f1e14e9511ff3e303179a15f6fc267f7634\",\n        \"urls\": [\n          \"bzz-raw://982f4328f956c3e60e67501e759eb292ac487f76460c774c50e9ae4fcc92aae5\",\n          \"dweb:/ipfs/QmRnzEDsaqtd9PJEVcgQi7p5aV5pMSvRUoGZJAdwFUJxgZ\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolDerivedState.sol\": {\n        \"keccak256\": \"0xe603ac5b17ecdee73ba2b27efdf386c257a19c14206e87eee77e2017b742d9e5\",\n        \"urls\": [\n          \"bzz-raw://8febc9bdb399a4d94bb89f5377732652e2400e4a8dee808201ade6848f9004e7\",\n          \"dweb:/ipfs/QmaKDqYYFU4d2W2iN77aDHptfbFmYZRrMYXHeGpJmM8C1c\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolErrors.sol\": {\n        \"keccak256\": \"0xf80abf13fb1fafc127ba4e792f240dd8ea7c8c893978cdfd8439c27fae9a037b\",\n        \"urls\": [\n          \"bzz-raw://b04fc72a656bbf3631e9c2e67b9870a2d9d235185e833fe050e9606e6816a9aa\",\n          \"dweb:/ipfs/QmUcz4bkEkJ9pwzFu1C3n97hBQ3st9U6qTAqCdyFwddKco\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolEvents.sol\": {\n        \"keccak256\": \"0x8071514d0fe5d17d6fbd31c191cdfb703031c24e0ece3621d88ab10e871375cd\",\n        \"urls\": [\n          \"bzz-raw://d0b571930cc7488b1d546a7e9cea7c52d8b3c4e207da657ed0e0db7343b8cd03\",\n          \"dweb:/ipfs/QmaGK6vVwB95QSTR1XMYvrh7ivYAYZxi3fD7v6VMA4jZ39\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolImmutables.sol\": {\n        \"keccak256\": \"0xf6e5d2cd1139c4c276bdbc8e1d2b256e456c866a91f1b868da265c6d2685c3f7\",\n        \"urls\": [\n          \"bzz-raw://b99c8c9ae8e27ee6559e5866bea82cbc9ffc8247f8d15b7422a4deb287d4d047\",\n          \"dweb:/ipfs/QmfL8gaqt3ffAnm6nVj5ksuNpLygXuL3xq5VBqrkwC2JJ3\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolOwnerActions.sol\": {\n        \"keccak256\": \"0x759b78a2918af9e99e246dc3af084f654e48ef32bb4e4cb8a966aa3dcaece235\",\n        \"urls\": [\n          \"bzz-raw://64144fb96e1c7fdba87305acadb98a198d26a3d46c097cb3a666e567f6f29735\",\n          \"dweb:/ipfs/QmUnWVwN9FKB9uV5Pr8YfLpWZnYM2DENnRMaadZ492JS9u\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/interfaces/pool/IUniswapV3PoolState.sol\": {\n        \"keccak256\": \"0x44fa2ce1182f6c2f6bead3d1737804bf7e112252ae86e0f2e92f9b8249603f43\",\n        \"urls\": [\n          \"bzz-raw://54154e8665b98d65f5dc91b256551852cb47882180b501b260657122d207c0ea\",\n          \"dweb:/ipfs/QmcfemK1A2PXYrWB5SBFGERpMCderbFRb8BtTzQDj1sUBp\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-core/contracts/libraries/FullMath.sol\": {\n        \"keccak256\": \"0x7825565a4bb2a34a1dc96bbfead755785dfb0df8ef81bd934c43023689685645\",\n        \"urls\": [\n          \"bzz-raw://8f44f4614d31e3d4864c7eb13620555253b84f6a69180f8745b7c6e246a9d125\",\n          \"dweb:/ipfs/QmfNQUcXj3KL8h9u5PqbtEC6yRtwDbKNb48uMPjdwxsKnd\"\n        ],\n        \"license\": \"MIT\"\n      },\n      \"lib/v3-core/contracts/libraries/TickMath.sol\": {\n        \"keccak256\": \"0x5c57de03a91cc2ec8939865dbbcb0197bb6c353b711075eefd8e0fca5e102129\",\n        \"urls\": [\n          \"bzz-raw://1e994c24fd891ef8a3f5dbf5eba42b34feaf05e0376a29f91322faa18054449c\",\n          \"dweb:/ipfs/QmNdUJGUQxd1dPkMbnA5f5UNqakxRkQE5r7bTZJAuHeapS\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"lib/v3-periphery/contracts/libraries/OracleLibrary.sol\": {\n        \"keccak256\": \"0xe313f89c69c0f1c91f0722868313b4ceb14479b3e7a0abf52a1b9bbd9c18e81b\",\n        \"urls\": [\n          \"bzz-raw://8ad09b061f3f533c116618c20cf01510108eebebb7ff8847b0f245b8ba368d53\",\n          \"dweb:/ipfs/QmU6vdi5JjJfjK2KqonWvRg9NeuQWarj4B1YFN22Dh9VJM\"\n        ],\n        \"license\": \"GPL-2.0-or-later\"\n      },\n      \"src/Registry.sol\": {\n        \"keccak256\": \"0x2f5f6d61ffc1c9336c628a2cff52b424377feb20c3390f37418fbef1c8995edd\",\n        \"urls\": [\n          \"bzz-raw://1eabb4a5f6bc4ea2c407f3a70efefae52ebe43a5c200cbf828d0e0a3ed676af2\",\n          \"dweb:/ipfs/QmT6nEdMQx6WzTj8x8ZLayH6hpK5NyXSb97vu2juQ4kS4h\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/base/Cellar.sol\": {\n        \"keccak256\": \"0xc5c29f881503e070d2ccc098a7a46d9b03dd47144ae213cc086add626fda9d1a\",\n        \"urls\": [\n          \"bzz-raw://5cf3ddc46f4ac3fe22ad9b1a54e2da8f7c7080d17d9bab1f0483eb0e425c4744\",\n          \"dweb:/ipfs/QmNRfJvUqSkzSUX5KjiFygjr8sKHkVvERTy7W9LZ9DNc6q\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/base/permutations/CellarWithShareLockPeriod.sol\": {\n        \"keccak256\": \"0x619f0e7139aaf886ee9610f29d4ccdec5272cf366c038d680afc34ffd01df50d\",\n        \"urls\": [\n          \"bzz-raw://82a7a2f85e29178dc28c58a17ff5ab5f19665ec533426ae71ad8c8b3068f6580\",\n          \"dweb:/ipfs/Qmf6741axQbRFUvayW1uqMJ5BHdXS32wXVa7jizWtt2PJv\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/interfaces/external/IChainlinkAggregator.sol\": {\n        \"keccak256\": \"0x6cabe293cd867cfd1b4e5c378f08aac66951cbdfacbd37627c2fe5c02661808f\",\n        \"urls\": [\n          \"bzz-raw://87b4e658a11189c690571f456a47fc644ff15926bdf634c66d05a1f711bea3ac\",\n          \"dweb:/ipfs/QmUDdZ8YxHEt3dTk1vWmNVDQFRp3kEqhwUUZEnRS8Sqb3u\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/interfaces/external/UniswapV3Pool.sol\": {\n        \"keccak256\": \"0xf6a71bfbc0665b1d0aad7148b685f3b2254e1be2a3672c2c6671b7fc20db26bb\",\n        \"urls\": [\n          \"bzz-raw://393871fdf75656c49a3bb83bc8110a93af2cbd805df9cc6574a058f3cbf100a9\",\n          \"dweb:/ipfs/QmZugnPctGBiWkFjXXtiKy3AQTWphA9tjZmSBVZRMg9urh\"\n        ],\n        \"license\": null\n      },\n      \"src/modules/adaptors/BaseAdaptor.sol\": {\n        \"keccak256\": \"0x82433df76f3d9eb1bf6c5c001fafd263a3f9ebc12c44b90291cf6aa71ff9e91d\",\n        \"urls\": [\n          \"bzz-raw://a297535259dfe38aaf94e95c290ec172758f3ede175b9b1f5e0f06405b1f23a6\",\n          \"dweb:/ipfs/Qmd8xS3DkUFnzcpqeGB8TEyHYYHUiFQ6L7jdKiEaxZ7bHT\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/modules/price-router/Extensions/Extension.sol\": {\n        \"keccak256\": \"0x974e612d503d25b2a0c2e424dd9743ba605a3b3660bd69cafbeb143dabdb3101\",\n        \"urls\": [\n          \"bzz-raw://bf97ce81e995cd034dd814ae1a04832a4864b22d0372c07ecc31779643d9e3ad\",\n          \"dweb:/ipfs/Qmauuf11TpCS2rZ7SU4iexnkuSBPJWmyF6nisSXseTqPF4\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/modules/price-router/PriceRouter.sol\": {\n        \"keccak256\": \"0x6e2e31227c601a10289dbbe7ae7a96adebcf3a62065b9b9d9edbcb4f57cc79dd\",\n        \"urls\": [\n          \"bzz-raw://25b1132fee46f95204107215d5d82eb1ffc0ceebfd360ac9495d43451861510e\",\n          \"dweb:/ipfs/QmWsY9usC61zsdAzafPEvmrv44vUMyCtzAtEESr3ioDTN1\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/utils/Math.sol\": {\n        \"keccak256\": \"0x8cc188510f9657ad17e7903cfeded703dd36901ef39069dd6019ffb7e9cfb8bc\",\n        \"urls\": [\n          \"bzz-raw://d1a66351eb53f333f62652f62c754f4c23ad76c98093e39265d06b84aef165c1\",\n          \"dweb:/ipfs/QmZNqeAaVwAvZosvKdwUFvnjHsYUcJDwDfDupEZ8uKx949\"\n        ],\n        \"license\": \"Apache-2.0\"\n      },\n      \"src/utils/Uint32Array.sol\": {\n        \"keccak256\": \"0xd887a816c5b1b66163cab06aa453e0a44e734e2d90484018d0b602ed2f42adb7\",\n        \"urls\": [\n          \"bzz-raw://99cfce0b643a810adfa861893a485e786f76be344a3c6c9348a0791f9f225218\",\n          \"dweb:/ipfs/QmPWf4GbBe6xqUuZJFLYp5Gwv9KTVc3h5phYEb4BMh7fxf\"\n        ],\n        \"license\": \"Apache-2.0\"\n      }\n    },\n    \"version\": 1\n  },\n  \"ast\": {\n    \"absolutePath\": \"src/base/permutations/CellarWithShareLockPeriod.sol\",\n    \"id\": 60935,\n    \"exportedSymbols\": {\n      \"Cellar\": [\n        58760\n      ],\n      \"CellarWithShareLockPeriod\": [\n        60934\n      ],\n      \"ERC20\": [\n        51130\n      ],\n      \"Math\": [\n        86922\n      ],\n      \"Registry\": [\n        55623\n      ]\n    },\n    \"nodeType\": \"SourceUnit\",\n    \"src\": \"39:5772:99\",\n    \"nodes\": [\n      {\n        \"id\": 60592,\n        \"nodeType\": \"PragmaDirective\",\n        \"src\": \"39:23:99\",\n        \"nodes\": [],\n        \"literals\": [\n          \"solidity\",\n          \"0.8\",\n          \".21\"\n        ]\n      },\n      {\n        \"id\": 60597,\n        \"nodeType\": \"ImportDirective\",\n        \"src\": \"64:68:99\",\n        \"nodes\": [],\n        \"absolutePath\": \"src/base/Cellar.sol\",\n        \"file\": \"src/base/Cellar.sol\",\n        \"nameLocation\": \"-1:-1:-1\",\n        \"scope\": 60935,\n        \"sourceUnit\": 58761,\n        \"symbolAliases\": [\n          {\n            \"foreign\": {\n              \"id\": 60593,\n              \"name\": \"Cellar\",\n              \"nodeType\": \"Identifier\",\n              \"overloadedDeclarations\": [],\n              \"referencedDeclaration\": 58760,\n              \"src\": \"73:6:99\",\n              \"typeDescriptions\": {}\n            },\n            \"nameLocation\": \"-1:-1:-1\"\n          },\n          {\n            \"foreign\": {\n              \"id\": 60594,\n              \"name\": \"Registry\",\n              \"nodeType\": \"Identifier\",\n              \"overloadedDeclarations\": [],\n              \"referencedDeclaration\": 55623,\n              \"src\": \"81:8:99\",\n              \"typeDescriptions\": {}\n            },\n            \"nameLocation\": \"-1:-1:-1\"\n          },\n          {\n            \"foreign\": {\n              \"id\": 60595,\n              \"name\": \"ERC20\",\n              \"nodeType\": \"Identifier\",\n              \"overloadedDeclarations\": [],\n              \"referencedDeclaration\": 51130,\n              \"src\": \"91:5:99\",\n              \"typeDescriptions\": {}\n            },\n            \"nameLocation\": \"-1:-1:-1\"\n          },\n          {\n            \"foreign\": {\n              \"id\": 60596,\n              \"name\": \"Math\",\n              \"nodeType\": \"Identifier\",\n              \"overloadedDeclarations\": [],\n              \"referencedDeclaration\": 86922,\n              \"src\": \"98:4:99\",\n              \"typeDescriptions\": {}\n            },\n            \"nameLocation\": \"-1:-1:-1\"\n          }\n        ],\n        \"unitAlias\": \"\"\n      },\n      {\n        \"id\": 60934,\n        \"nodeType\": \"ContractDefinition\",\n        \"src\": \"134:5676:99\",\n        \"nodes\": [\n          {\n            \"id\": 60602,\n            \"nodeType\": \"UsingForDirective\",\n            \"src\": \"185:23:99\",\n            \"nodes\": [],\n            \"global\": false,\n            \"libraryName\": {\n              \"id\": 60600,\n              \"name\": \"Math\",\n              \"nameLocations\": [\n                \"191:4:99\"\n              ],\n              \"nodeType\": \"IdentifierPath\",\n              \"referencedDeclaration\": 86922,\n              \"src\": \"191:4:99\"\n            },\n            \"typeName\": {\n              \"id\": 60601,\n              \"name\": \"uint256\",\n              \"nodeType\": \"ElementaryTypeName\",\n              \"src\": \"200:7:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_uint256\",\n                \"typeString\": \"uint256\"\n              }\n            }\n          },\n          {\n            \"id\": 60640,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"214:629:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60639,\n              \"nodeType\": \"Block\",\n              \"src\": \"841:2:99\",\n              \"nodes\": [],\n              \"statements\": []\n            },\n            \"implemented\": true,\n            \"kind\": \"constructor\",\n            \"modifiers\": [\n              {\n                \"arguments\": [\n                  {\n                    \"id\": 60627,\n                    \"name\": \"_owner\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60604,\n                    \"src\": \"577:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  {\n                    \"id\": 60628,\n                    \"name\": \"_registry\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60607,\n                    \"src\": \"597:9:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_contract$_Registry_$55623\",\n                      \"typeString\": \"contract Registry\"\n                    }\n                  },\n                  {\n                    \"id\": 60629,\n                    \"name\": \"_asset\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60610,\n                    \"src\": \"620:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_contract$_ERC20_$51130\",\n                      \"typeString\": \"contract ERC20\"\n                    }\n                  },\n                  {\n                    \"id\": 60630,\n                    \"name\": \"_name\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60612,\n                    \"src\": \"640:5:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_string_memory_ptr\",\n                      \"typeString\": \"string memory\"\n                    }\n                  },\n                  {\n                    \"id\": 60631,\n                    \"name\": \"_symbol\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60614,\n                    \"src\": \"659:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_string_memory_ptr\",\n                      \"typeString\": \"string memory\"\n                    }\n                  },\n                  {\n                    \"id\": 60632,\n                    \"name\": \"_holdingPosition\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60616,\n                    \"src\": \"680:16:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint32\",\n                      \"typeString\": \"uint32\"\n                    }\n                  },\n                  {\n                    \"id\": 60633,\n                    \"name\": \"_holdingPositionConfig\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60618,\n                    \"src\": \"710:22:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bytes_memory_ptr\",\n                      \"typeString\": \"bytes memory\"\n                    }\n                  },\n                  {\n                    \"id\": 60634,\n                    \"name\": \"_initialDeposit\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60620,\n                    \"src\": \"746:15:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  {\n                    \"id\": 60635,\n                    \"name\": \"_strategistPlatformCut\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60622,\n                    \"src\": \"775:22:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint64\",\n                      \"typeString\": \"uint64\"\n                    }\n                  },\n                  {\n                    \"id\": 60636,\n                    \"name\": \"_shareSupplyCap\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60624,\n                    \"src\": \"811:15:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint192\",\n                      \"typeString\": \"uint192\"\n                    }\n                  }\n                ],\n                \"id\": 60637,\n                \"kind\": \"baseConstructorSpecifier\",\n                \"modifierName\": {\n                  \"id\": 60626,\n                  \"name\": \"Cellar\",\n                  \"nameLocations\": [\n                    \"557:6:99\"\n                  ],\n                  \"nodeType\": \"IdentifierPath\",\n                  \"referencedDeclaration\": 58760,\n                  \"src\": \"557:6:99\"\n                },\n                \"nodeType\": \"ModifierInvocation\",\n                \"src\": \"557:279:99\"\n              }\n            ],\n            \"name\": \"\",\n            \"nameLocation\": \"-1:-1:-1\",\n            \"parameters\": {\n              \"id\": 60625,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60604,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_owner\",\n                  \"nameLocation\": \"243:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"235:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60603,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"235:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60607,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_registry\",\n                  \"nameLocation\": \"268:9:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"259:18:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_contract$_Registry_$55623\",\n                    \"typeString\": \"contract Registry\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60606,\n                    \"nodeType\": \"UserDefinedTypeName\",\n                    \"pathNode\": {\n                      \"id\": 60605,\n                      \"name\": \"Registry\",\n                      \"nameLocations\": [\n                        \"259:8:99\"\n                      ],\n                      \"nodeType\": \"IdentifierPath\",\n                      \"referencedDeclaration\": 55623,\n                      \"src\": \"259:8:99\"\n                    },\n                    \"referencedDeclaration\": 55623,\n                    \"src\": \"259:8:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_contract$_Registry_$55623\",\n                      \"typeString\": \"contract Registry\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60610,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_asset\",\n                  \"nameLocation\": \"293:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"287:12:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_contract$_ERC20_$51130\",\n                    \"typeString\": \"contract ERC20\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60609,\n                    \"nodeType\": \"UserDefinedTypeName\",\n                    \"pathNode\": {\n                      \"id\": 60608,\n                      \"name\": \"ERC20\",\n                      \"nameLocations\": [\n                        \"287:5:99\"\n                      ],\n                      \"nodeType\": \"IdentifierPath\",\n                      \"referencedDeclaration\": 51130,\n                      \"src\": \"287:5:99\"\n                    },\n                    \"referencedDeclaration\": 51130,\n                    \"src\": \"287:5:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_contract$_ERC20_$51130\",\n                      \"typeString\": \"contract ERC20\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60612,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_name\",\n                  \"nameLocation\": \"323:5:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"309:19:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"memory\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_string_memory_ptr\",\n                    \"typeString\": \"string\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60611,\n                    \"name\": \"string\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"309:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_string_storage_ptr\",\n                      \"typeString\": \"string\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60614,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_symbol\",\n                  \"nameLocation\": \"352:7:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"338:21:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"memory\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_string_memory_ptr\",\n                    \"typeString\": \"string\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60613,\n                    \"name\": \"string\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"338:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_string_storage_ptr\",\n                      \"typeString\": \"string\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60616,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_holdingPosition\",\n                  \"nameLocation\": \"376:16:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"369:23:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint32\",\n                    \"typeString\": \"uint32\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60615,\n                    \"name\": \"uint32\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"369:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint32\",\n                      \"typeString\": \"uint32\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60618,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_holdingPositionConfig\",\n                  \"nameLocation\": \"415:22:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"402:35:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"memory\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_bytes_memory_ptr\",\n                    \"typeString\": \"bytes\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60617,\n                    \"name\": \"bytes\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"402:5:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bytes_storage_ptr\",\n                      \"typeString\": \"bytes\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60620,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_initialDeposit\",\n                  \"nameLocation\": \"455:15:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"447:23:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60619,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"447:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60622,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_strategistPlatformCut\",\n                  \"nameLocation\": \"487:22:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"480:29:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint64\",\n                    \"typeString\": \"uint64\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60621,\n                    \"name\": \"uint64\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"480:6:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint64\",\n                      \"typeString\": \"uint64\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60624,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"_shareSupplyCap\",\n                  \"nameLocation\": \"527:15:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60640,\n                  \"src\": \"519:23:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint192\",\n                    \"typeString\": \"uint192\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60623,\n                    \"name\": \"uint192\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"519:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint192\",\n                      \"typeString\": \"uint192\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"225:323:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60638,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"841:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"nonpayable\",\n            \"virtual\": false,\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60647,\n            \"nodeType\": \"EventDefinition\",\n            \"src\": \"1020:70:99\",\n            \"nodes\": [],\n            \"anonymous\": false,\n            \"documentation\": {\n              \"id\": 60641,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"849:166:99\",\n              \"text\": \" @notice Emitted when share locking period is changed.\\n @param oldPeriod the old locking period\\n @param newPeriod the new locking period\"\n            },\n            \"eventSelector\": \"227ff5c6b5ffb395236b09fd1b472bb128b36eaa17556633feefe28e94411f24\",\n            \"name\": \"ShareLockingPeriodChanged\",\n            \"nameLocation\": \"1026:25:99\",\n            \"parameters\": {\n              \"id\": 60646,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60643,\n                  \"indexed\": false,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"oldPeriod\",\n                  \"nameLocation\": \"1060:9:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60647,\n                  \"src\": \"1052:17:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60642,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"1052:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60645,\n                  \"indexed\": false,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"newPeriod\",\n                  \"nameLocation\": \"1079:9:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60647,\n                  \"src\": \"1071:17:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60644,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"1071:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"1051:38:99\"\n            }\n          },\n          {\n            \"id\": 60650,\n            \"nodeType\": \"ErrorDefinition\",\n            \"src\": \"1184:39:99\",\n            \"nodes\": [],\n            \"documentation\": {\n              \"id\": 60648,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1096:83:99\",\n              \"text\": \" @notice Attempted to set `shareLockPeriod` to an invalid number.\"\n            },\n            \"errorSelector\": \"e9808cfc\",\n            \"name\": \"Cellar__InvalidShareLockPeriod\",\n            \"nameLocation\": \"1190:30:99\",\n            \"parameters\": {\n              \"id\": 60649,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"1220:2:99\"\n            }\n          },\n          {\n            \"id\": 60657,\n            \"nodeType\": \"ErrorDefinition\",\n            \"src\": \"1440:83:99\",\n            \"nodes\": [],\n            \"documentation\": {\n              \"id\": 60651,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1229:206:99\",\n              \"text\": \" @notice Attempted to burn shares when they are locked.\\n @param timeSharesAreUnlocked time when caller can transfer/redeem shares\\n @param currentBlock the current block number.\"\n            },\n            \"errorSelector\": \"1be3b8fc\",\n            \"name\": \"Cellar__SharesAreLocked\",\n            \"nameLocation\": \"1446:23:99\",\n            \"parameters\": {\n              \"id\": 60656,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60653,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"timeSharesAreUnlocked\",\n                  \"nameLocation\": \"1478:21:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60657,\n                  \"src\": \"1470:29:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60652,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"1470:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60655,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"currentBlock\",\n                  \"nameLocation\": \"1509:12:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60657,\n                  \"src\": \"1501:20:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60654,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"1501:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"1469:53:99\"\n            }\n          },\n          {\n            \"id\": 60662,\n            \"nodeType\": \"ErrorDefinition\",\n            \"src\": \"1622:62:99\",\n            \"nodes\": [],\n            \"documentation\": {\n              \"id\": 60658,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1529:88:99\",\n              \"text\": \" @notice Attempted deposit on behalf of a user without being approved.\"\n            },\n            \"errorSelector\": \"d21c7c94\",\n            \"name\": \"Cellar__NotApprovedToDepositOnBehalf\",\n            \"nameLocation\": \"1628:36:99\",\n            \"parameters\": {\n              \"id\": 60661,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60660,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"depositor\",\n                  \"nameLocation\": \"1673:9:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60662,\n                  \"src\": \"1665:17:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60659,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"1665:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"1664:19:99\"\n            }\n          },\n          {\n            \"id\": 60668,\n            \"nodeType\": \"VariableDeclaration\",\n            \"src\": \"1781:58:99\",\n            \"nodes\": [],\n            \"constant\": true,\n            \"documentation\": {\n              \"id\": 60663,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1690:86:99\",\n              \"text\": \" @notice Shares must be locked for at least 5 minutes after minting.\"\n            },\n            \"functionSelector\": \"0051a3b7\",\n            \"mutability\": \"constant\",\n            \"name\": \"MINIMUM_SHARE_LOCK_PERIOD\",\n            \"nameLocation\": \"1805:25:99\",\n            \"scope\": 60934,\n            \"stateVariable\": true,\n            \"storageLocation\": \"default\",\n            \"typeDescriptions\": {\n              \"typeIdentifier\": \"t_uint256\",\n              \"typeString\": \"uint256\"\n            },\n            \"typeName\": {\n              \"id\": 60664,\n              \"name\": \"uint256\",\n              \"nodeType\": \"ElementaryTypeName\",\n              \"src\": \"1781:7:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_uint256\",\n                \"typeString\": \"uint256\"\n              }\n            },\n            \"value\": {\n              \"commonType\": {\n                \"typeIdentifier\": \"t_rational_300_by_1\",\n                \"typeString\": \"int_const 300\"\n              },\n              \"id\": 60667,\n              \"isConstant\": false,\n              \"isLValue\": false,\n              \"isPure\": true,\n              \"lValueRequested\": false,\n              \"leftExpression\": {\n                \"hexValue\": \"35\",\n                \"id\": 60665,\n                \"isConstant\": false,\n                \"isLValue\": false,\n                \"isPure\": true,\n                \"kind\": \"number\",\n                \"lValueRequested\": false,\n                \"nodeType\": \"Literal\",\n                \"src\": \"1833:1:99\",\n                \"typeDescriptions\": {\n                  \"typeIdentifier\": \"t_rational_5_by_1\",\n                  \"typeString\": \"int_const 5\"\n                },\n                \"value\": \"5\"\n              },\n              \"nodeType\": \"BinaryOperation\",\n              \"operator\": \"*\",\n              \"rightExpression\": {\n                \"hexValue\": \"3630\",\n                \"id\": 60666,\n                \"isConstant\": false,\n                \"isLValue\": false,\n                \"isPure\": true,\n                \"kind\": \"number\",\n                \"lValueRequested\": false,\n                \"nodeType\": \"Literal\",\n                \"src\": \"1837:2:99\",\n                \"typeDescriptions\": {\n                  \"typeIdentifier\": \"t_rational_60_by_1\",\n                  \"typeString\": \"int_const 60\"\n                },\n                \"value\": \"60\"\n              },\n              \"src\": \"1833:6:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_rational_300_by_1\",\n                \"typeString\": \"int_const 300\"\n              }\n            },\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60672,\n            \"nodeType\": \"VariableDeclaration\",\n            \"src\": \"1932:58:99\",\n            \"nodes\": [],\n            \"constant\": true,\n            \"documentation\": {\n              \"id\": 60669,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1846:81:99\",\n              \"text\": \" @notice Shares can be locked for at most 2 days after minting.\"\n            },\n            \"functionSelector\": \"0402ab63\",\n            \"mutability\": \"constant\",\n            \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n            \"nameLocation\": \"1956:25:99\",\n            \"scope\": 60934,\n            \"stateVariable\": true,\n            \"storageLocation\": \"default\",\n            \"typeDescriptions\": {\n              \"typeIdentifier\": \"t_uint256\",\n              \"typeString\": \"uint256\"\n            },\n            \"typeName\": {\n              \"id\": 60670,\n              \"name\": \"uint256\",\n              \"nodeType\": \"ElementaryTypeName\",\n              \"src\": \"1932:7:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_uint256\",\n                \"typeString\": \"uint256\"\n              }\n            },\n            \"value\": {\n              \"hexValue\": \"32\",\n              \"id\": 60671,\n              \"isConstant\": false,\n              \"isLValue\": false,\n              \"isPure\": true,\n              \"kind\": \"number\",\n              \"lValueRequested\": false,\n              \"nodeType\": \"Literal\",\n              \"src\": \"1984:6:99\",\n              \"subdenomination\": \"days\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_rational_172800_by_1\",\n                \"typeString\": \"int_const 172800\"\n              },\n              \"value\": \"2\"\n            },\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60676,\n            \"nodeType\": \"VariableDeclaration\",\n            \"src\": \"2138:58:99\",\n            \"nodes\": [],\n            \"constant\": false,\n            \"documentation\": {\n              \"id\": 60673,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"1997:136:99\",\n              \"text\": \" @notice After deposits users must wait `shareLockPeriod` time before being able to transfer or withdraw their shares.\"\n            },\n            \"functionSelector\": \"9fdb11b6\",\n            \"mutability\": \"mutable\",\n            \"name\": \"shareLockPeriod\",\n            \"nameLocation\": \"2153:15:99\",\n            \"scope\": 60934,\n            \"stateVariable\": true,\n            \"storageLocation\": \"default\",\n            \"typeDescriptions\": {\n              \"typeIdentifier\": \"t_uint256\",\n              \"typeString\": \"uint256\"\n            },\n            \"typeName\": {\n              \"id\": 60674,\n              \"name\": \"uint256\",\n              \"nodeType\": \"ElementaryTypeName\",\n              \"src\": \"2138:7:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_uint256\",\n                \"typeString\": \"uint256\"\n              }\n            },\n            \"value\": {\n              \"id\": 60675,\n              \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n              \"nodeType\": \"Identifier\",\n              \"overloadedDeclarations\": [],\n              \"referencedDeclaration\": 60672,\n              \"src\": \"2171:25:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_uint256\",\n                \"typeString\": \"uint256\"\n              }\n            },\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60681,\n            \"nodeType\": \"VariableDeclaration\",\n            \"src\": \"2302:57:99\",\n            \"nodes\": [],\n            \"constant\": false,\n            \"documentation\": {\n              \"id\": 60677,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"2203:94:99\",\n              \"text\": \" @notice mapping that stores every users last time stamp they minted shares.\"\n            },\n            \"functionSelector\": \"687c2b50\",\n            \"mutability\": \"mutable\",\n            \"name\": \"userShareLockStartTime\",\n            \"nameLocation\": \"2337:22:99\",\n            \"scope\": 60934,\n            \"stateVariable\": true,\n            \"storageLocation\": \"default\",\n            \"typeDescriptions\": {\n              \"typeIdentifier\": \"t_mapping$_t_address_$_t_uint256_$\",\n              \"typeString\": \"mapping(address => uint256)\"\n            },\n            \"typeName\": {\n              \"id\": 60680,\n              \"keyName\": \"\",\n              \"keyNameLocation\": \"-1:-1:-1\",\n              \"keyType\": {\n                \"id\": 60678,\n                \"name\": \"address\",\n                \"nodeType\": \"ElementaryTypeName\",\n                \"src\": \"2310:7:99\",\n                \"typeDescriptions\": {\n                  \"typeIdentifier\": \"t_address\",\n                  \"typeString\": \"address\"\n                }\n              },\n              \"nodeType\": \"Mapping\",\n              \"src\": \"2302:27:99\",\n              \"typeDescriptions\": {\n                \"typeIdentifier\": \"t_mapping$_t_address_$_t_uint256_$\",\n                \"typeString\": \"mapping(address => uint256)\"\n              },\n              \"valueName\": \"\",\n              \"valueNameLocation\": \"-1:-1:-1\",\n              \"valueType\": {\n                \"id\": 60679,\n                \"name\": \"uint256\",\n                \"nodeType\": \"ElementaryTypeName\",\n                \"src\": \"2321:7:99\",\n                \"typeDescriptions\": {\n                  \"typeIdentifier\": \"t_uint256\",\n                  \"typeString\": \"uint256\"\n                }\n              }\n            },\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60714,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"2525:366:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60713,\n              \"nodeType\": \"Block\",\n              \"src\": \"2589:302:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"condition\": {\n                    \"commonType\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    },\n                    \"id\": 60695,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftExpression\": {\n                      \"commonType\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      },\n                      \"id\": 60691,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"leftExpression\": {\n                        \"id\": 60689,\n                        \"name\": \"newLock\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60684,\n                        \"src\": \"2603:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"nodeType\": \"BinaryOperation\",\n                      \"operator\": \"<\",\n                      \"rightExpression\": {\n                        \"id\": 60690,\n                        \"name\": \"MINIMUM_SHARE_LOCK_PERIOD\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60668,\n                        \"src\": \"2613:25:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"src\": \"2603:35:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_bool\",\n                        \"typeString\": \"bool\"\n                      }\n                    },\n                    \"nodeType\": \"BinaryOperation\",\n                    \"operator\": \"||\",\n                    \"rightExpression\": {\n                      \"commonType\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      },\n                      \"id\": 60694,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"leftExpression\": {\n                        \"id\": 60692,\n                        \"name\": \"newLock\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60684,\n                        \"src\": \"2642:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"nodeType\": \"BinaryOperation\",\n                      \"operator\": \">\",\n                      \"rightExpression\": {\n                        \"id\": 60693,\n                        \"name\": \"MAXIMUM_SHARE_LOCK_PERIOD\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60672,\n                        \"src\": \"2652:25:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"src\": \"2642:35:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_bool\",\n                        \"typeString\": \"bool\"\n                      }\n                    },\n                    \"src\": \"2603:74:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"id\": 60699,\n                  \"nodeType\": \"IfStatement\",\n                  \"src\": \"2599:131:99\",\n                  \"trueBody\": {\n                    \"errorCall\": {\n                      \"arguments\": [],\n                      \"expression\": {\n                        \"argumentTypes\": [],\n                        \"id\": 60696,\n                        \"name\": \"Cellar__InvalidShareLockPeriod\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60650,\n                        \"src\": \"2698:30:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_function_error_pure$__$returns$__$\",\n                          \"typeString\": \"function () pure\"\n                        }\n                      },\n                      \"id\": 60697,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"kind\": \"functionCall\",\n                      \"lValueRequested\": false,\n                      \"nameLocations\": [],\n                      \"names\": [],\n                      \"nodeType\": \"FunctionCall\",\n                      \"src\": \"2698:32:99\",\n                      \"tryCall\": false,\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_tuple$__$\",\n                        \"typeString\": \"tuple()\"\n                      }\n                    },\n                    \"id\": 60698,\n                    \"nodeType\": \"RevertStatement\",\n                    \"src\": \"2691:39:99\"\n                  }\n                },\n                {\n                  \"assignments\": [\n                    60701\n                  ],\n                  \"declarations\": [\n                    {\n                      \"constant\": false,\n                      \"id\": 60701,\n                      \"mutability\": \"mutable\",\n                      \"name\": \"oldLockingPeriod\",\n                      \"nameLocation\": \"2748:16:99\",\n                      \"nodeType\": \"VariableDeclaration\",\n                      \"scope\": 60713,\n                      \"src\": \"2740:24:99\",\n                      \"stateVariable\": false,\n                      \"storageLocation\": \"default\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      },\n                      \"typeName\": {\n                        \"id\": 60700,\n                        \"name\": \"uint256\",\n                        \"nodeType\": \"ElementaryTypeName\",\n                        \"src\": \"2740:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"visibility\": \"internal\"\n                    }\n                  ],\n                  \"id\": 60703,\n                  \"initialValue\": {\n                    \"id\": 60702,\n                    \"name\": \"shareLockPeriod\",\n                    \"nodeType\": \"Identifier\",\n                    \"overloadedDeclarations\": [],\n                    \"referencedDeclaration\": 60676,\n                    \"src\": \"2767:15:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"nodeType\": \"VariableDeclarationStatement\",\n                  \"src\": \"2740:42:99\"\n                },\n                {\n                  \"expression\": {\n                    \"id\": 60706,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftHandSide\": {\n                      \"id\": 60704,\n                      \"name\": \"shareLockPeriod\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60676,\n                      \"src\": \"2792:15:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"nodeType\": \"Assignment\",\n                    \"operator\": \"=\",\n                    \"rightHandSide\": {\n                      \"id\": 60705,\n                      \"name\": \"newLock\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60684,\n                      \"src\": \"2810:7:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"src\": \"2792:25:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"id\": 60707,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"2792:25:99\"\n                },\n                {\n                  \"eventCall\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60709,\n                        \"name\": \"oldLockingPeriod\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60701,\n                        \"src\": \"2858:16:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60710,\n                        \"name\": \"newLock\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60684,\n                        \"src\": \"2876:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      ],\n                      \"id\": 60708,\n                      \"name\": \"ShareLockingPeriodChanged\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60647,\n                      \"src\": \"2832:25:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_event_nonpayable$_t_uint256_$_t_uint256_$returns$__$\",\n                        \"typeString\": \"function (uint256,uint256)\"\n                      }\n                    },\n                    \"id\": 60711,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"2832:52:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60712,\n                  \"nodeType\": \"EmitStatement\",\n                  \"src\": \"2827:57:99\"\n                }\n              ]\n            },\n            \"documentation\": {\n              \"id\": 60682,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"2366:154:99\",\n              \"text\": \" @notice Allows share lock period to be updated.\\n @param newLock the new lock period\\n @dev Callable by Sommelier Strategist.\"\n            },\n            \"functionSelector\": \"9c552ca8\",\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [\n              {\n                \"id\": 60687,\n                \"kind\": \"modifierInvocation\",\n                \"modifierName\": {\n                  \"id\": 60686,\n                  \"name\": \"onlyOwner\",\n                  \"nameLocations\": [\n                    \"2579:9:99\"\n                  ],\n                  \"nodeType\": \"IdentifierPath\",\n                  \"referencedDeclaration\": 50184,\n                  \"src\": \"2579:9:99\"\n                },\n                \"nodeType\": \"ModifierInvocation\",\n                \"src\": \"2579:9:99\"\n              }\n            ],\n            \"name\": \"setShareLockPeriod\",\n            \"nameLocation\": \"2534:18:99\",\n            \"parameters\": {\n              \"id\": 60685,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60684,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"newLock\",\n                  \"nameLocation\": \"2561:7:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60714,\n                  \"src\": \"2553:15:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60683,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"2553:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"2552:17:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60688,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"2589:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"nonpayable\",\n            \"virtual\": false,\n            \"visibility\": \"external\"\n          },\n          {\n            \"id\": 60749,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"3050:380:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60748,\n              \"nodeType\": \"Block\",\n              \"src\": \"3109:321:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"assignments\": [\n                    60721\n                  ],\n                  \"declarations\": [\n                    {\n                      \"constant\": false,\n                      \"id\": 60721,\n                      \"mutability\": \"mutable\",\n                      \"name\": \"lockTime\",\n                      \"nameLocation\": \"3127:8:99\",\n                      \"nodeType\": \"VariableDeclaration\",\n                      \"scope\": 60748,\n                      \"src\": \"3119:16:99\",\n                      \"stateVariable\": false,\n                      \"storageLocation\": \"default\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      },\n                      \"typeName\": {\n                        \"id\": 60720,\n                        \"name\": \"uint256\",\n                        \"nodeType\": \"ElementaryTypeName\",\n                        \"src\": \"3119:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"visibility\": \"internal\"\n                    }\n                  ],\n                  \"id\": 60725,\n                  \"initialValue\": {\n                    \"baseExpression\": {\n                      \"id\": 60722,\n                      \"name\": \"userShareLockStartTime\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60681,\n                      \"src\": \"3138:22:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_mapping$_t_address_$_t_uint256_$\",\n                        \"typeString\": \"mapping(address => uint256)\"\n                      }\n                    },\n                    \"id\": 60724,\n                    \"indexExpression\": {\n                      \"id\": 60723,\n                      \"name\": \"owner\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60717,\n                      \"src\": \"3161:5:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_address\",\n                        \"typeString\": \"address\"\n                      }\n                    },\n                    \"isConstant\": false,\n                    \"isLValue\": true,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"nodeType\": \"IndexAccess\",\n                    \"src\": \"3138:29:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"nodeType\": \"VariableDeclarationStatement\",\n                  \"src\": \"3119:48:99\"\n                },\n                {\n                  \"condition\": {\n                    \"commonType\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    },\n                    \"id\": 60728,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftExpression\": {\n                      \"id\": 60726,\n                      \"name\": \"lockTime\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60721,\n                      \"src\": \"3181:8:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"nodeType\": \"BinaryOperation\",\n                    \"operator\": \"!=\",\n                    \"rightExpression\": {\n                      \"hexValue\": \"30\",\n                      \"id\": 60727,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": true,\n                      \"kind\": \"number\",\n                      \"lValueRequested\": false,\n                      \"nodeType\": \"Literal\",\n                      \"src\": \"3193:1:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_rational_0_by_1\",\n                        \"typeString\": \"int_const 0\"\n                      },\n                      \"value\": \"0\"\n                    },\n                    \"src\": \"3181:13:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"id\": 60747,\n                  \"nodeType\": \"IfStatement\",\n                  \"src\": \"3177:247:99\",\n                  \"trueBody\": {\n                    \"id\": 60746,\n                    \"nodeType\": \"Block\",\n                    \"src\": \"3196:228:99\",\n                    \"statements\": [\n                      {\n                        \"assignments\": [\n                          60730\n                        ],\n                        \"declarations\": [\n                          {\n                            \"constant\": false,\n                            \"id\": 60730,\n                            \"mutability\": \"mutable\",\n                            \"name\": \"timeSharesAreUnlocked\",\n                            \"nameLocation\": \"3218:21:99\",\n                            \"nodeType\": \"VariableDeclaration\",\n                            \"scope\": 60746,\n                            \"src\": \"3210:29:99\",\n                            \"stateVariable\": false,\n                            \"storageLocation\": \"default\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            },\n                            \"typeName\": {\n                              \"id\": 60729,\n                              \"name\": \"uint256\",\n                              \"nodeType\": \"ElementaryTypeName\",\n                              \"src\": \"3210:7:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_uint256\",\n                                \"typeString\": \"uint256\"\n                              }\n                            },\n                            \"visibility\": \"internal\"\n                          }\n                        ],\n                        \"id\": 60734,\n                        \"initialValue\": {\n                          \"commonType\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          },\n                          \"id\": 60733,\n                          \"isConstant\": false,\n                          \"isLValue\": false,\n                          \"isPure\": false,\n                          \"lValueRequested\": false,\n                          \"leftExpression\": {\n                            \"id\": 60731,\n                            \"name\": \"lockTime\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60721,\n                            \"src\": \"3242:8:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"nodeType\": \"BinaryOperation\",\n                          \"operator\": \"+\",\n                          \"rightExpression\": {\n                            \"id\": 60732,\n                            \"name\": \"shareLockPeriod\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60676,\n                            \"src\": \"3253:15:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"src\": \"3242:26:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          }\n                        },\n                        \"nodeType\": \"VariableDeclarationStatement\",\n                        \"src\": \"3210:58:99\"\n                      },\n                      {\n                        \"condition\": {\n                          \"commonType\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          },\n                          \"id\": 60738,\n                          \"isConstant\": false,\n                          \"isLValue\": false,\n                          \"isPure\": false,\n                          \"lValueRequested\": false,\n                          \"leftExpression\": {\n                            \"id\": 60735,\n                            \"name\": \"timeSharesAreUnlocked\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60730,\n                            \"src\": \"3286:21:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"nodeType\": \"BinaryOperation\",\n                          \"operator\": \">\",\n                          \"rightExpression\": {\n                            \"expression\": {\n                              \"id\": 60736,\n                              \"name\": \"block\",\n                              \"nodeType\": \"Identifier\",\n                              \"overloadedDeclarations\": [],\n                              \"referencedDeclaration\": -4,\n                              \"src\": \"3310:5:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_magic_block\",\n                                \"typeString\": \"block\"\n                              }\n                            },\n                            \"id\": 60737,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": false,\n                            \"lValueRequested\": false,\n                            \"memberLocation\": \"3316:9:99\",\n                            \"memberName\": \"timestamp\",\n                            \"nodeType\": \"MemberAccess\",\n                            \"src\": \"3310:15:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"src\": \"3286:39:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_bool\",\n                            \"typeString\": \"bool\"\n                          }\n                        },\n                        \"id\": 60745,\n                        \"nodeType\": \"IfStatement\",\n                        \"src\": \"3282:131:99\",\n                        \"trueBody\": {\n                          \"errorCall\": {\n                            \"arguments\": [\n                              {\n                                \"id\": 60740,\n                                \"name\": \"timeSharesAreUnlocked\",\n                                \"nodeType\": \"Identifier\",\n                                \"overloadedDeclarations\": [],\n                                \"referencedDeclaration\": 60730,\n                                \"src\": \"3374:21:99\",\n                                \"typeDescriptions\": {\n                                  \"typeIdentifier\": \"t_uint256\",\n                                  \"typeString\": \"uint256\"\n                                }\n                              },\n                              {\n                                \"expression\": {\n                                  \"id\": 60741,\n                                  \"name\": \"block\",\n                                  \"nodeType\": \"Identifier\",\n                                  \"overloadedDeclarations\": [],\n                                  \"referencedDeclaration\": -4,\n                                  \"src\": \"3397:5:99\",\n                                  \"typeDescriptions\": {\n                                    \"typeIdentifier\": \"t_magic_block\",\n                                    \"typeString\": \"block\"\n                                  }\n                                },\n                                \"id\": 60742,\n                                \"isConstant\": false,\n                                \"isLValue\": false,\n                                \"isPure\": false,\n                                \"lValueRequested\": false,\n                                \"memberLocation\": \"3403:9:99\",\n                                \"memberName\": \"timestamp\",\n                                \"nodeType\": \"MemberAccess\",\n                                \"src\": \"3397:15:99\",\n                                \"typeDescriptions\": {\n                                  \"typeIdentifier\": \"t_uint256\",\n                                  \"typeString\": \"uint256\"\n                                }\n                              }\n                            ],\n                            \"expression\": {\n                              \"argumentTypes\": [\n                                {\n                                  \"typeIdentifier\": \"t_uint256\",\n                                  \"typeString\": \"uint256\"\n                                },\n                                {\n                                  \"typeIdentifier\": \"t_uint256\",\n                                  \"typeString\": \"uint256\"\n                                }\n                              ],\n                              \"id\": 60739,\n                              \"name\": \"Cellar__SharesAreLocked\",\n                              \"nodeType\": \"Identifier\",\n                              \"overloadedDeclarations\": [],\n                              \"referencedDeclaration\": 60657,\n                              \"src\": \"3350:23:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_function_error_pure$_t_uint256_$_t_uint256_$returns$__$\",\n                                \"typeString\": \"function (uint256,uint256) pure\"\n                              }\n                            },\n                            \"id\": 60743,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": false,\n                            \"kind\": \"functionCall\",\n                            \"lValueRequested\": false,\n                            \"nameLocations\": [],\n                            \"names\": [],\n                            \"nodeType\": \"FunctionCall\",\n                            \"src\": \"3350:63:99\",\n                            \"tryCall\": false,\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_tuple$__$\",\n                              \"typeString\": \"tuple()\"\n                            }\n                          },\n                          \"id\": 60744,\n                          \"nodeType\": \"RevertStatement\",\n                          \"src\": \"3343:70:99\"\n                        }\n                      }\n                    ]\n                  }\n                }\n              ]\n            },\n            \"documentation\": {\n              \"id\": 60715,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"2897:148:99\",\n              \"text\": \" @notice helper function that checks enough time has passed to unlock shares.\\n @param owner the address of the user to check\"\n            },\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"_checkIfSharesLocked\",\n            \"nameLocation\": \"3059:20:99\",\n            \"parameters\": {\n              \"id\": 60718,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60717,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"owner\",\n                  \"nameLocation\": \"3088:5:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60749,\n                  \"src\": \"3080:13:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60716,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3080:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"3079:15:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60719,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"3109:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"view\",\n            \"virtual\": false,\n            \"visibility\": \"internal\"\n          },\n          {\n            \"id\": 60772,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"3512:169:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60771,\n              \"nodeType\": \"Block\",\n              \"src\": \"3589:92:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"expression\": {\n                          \"id\": 60761,\n                          \"name\": \"msg\",\n                          \"nodeType\": \"Identifier\",\n                          \"overloadedDeclarations\": [],\n                          \"referencedDeclaration\": -15,\n                          \"src\": \"3620:3:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_magic_message\",\n                            \"typeString\": \"msg\"\n                          }\n                        },\n                        \"id\": 60762,\n                        \"isConstant\": false,\n                        \"isLValue\": false,\n                        \"isPure\": false,\n                        \"lValueRequested\": false,\n                        \"memberLocation\": \"3624:6:99\",\n                        \"memberName\": \"sender\",\n                        \"nodeType\": \"MemberAccess\",\n                        \"src\": \"3620:10:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"id\": 60760,\n                      \"name\": \"_checkIfSharesLocked\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60749,\n                      \"src\": \"3599:20:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_view$_t_address_$returns$__$\",\n                        \"typeString\": \"function (address) view\"\n                      }\n                    },\n                    \"id\": 60763,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"3599:32:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60764,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"3599:32:99\"\n                },\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60767,\n                        \"name\": \"to\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60752,\n                        \"src\": \"3663:2:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      },\n                      {\n                        \"id\": 60768,\n                        \"name\": \"amount\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60754,\n                        \"src\": \"3667:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      ],\n                      \"expression\": {\n                        \"id\": 60765,\n                        \"name\": \"super\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -25,\n                        \"src\": \"3648:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                          \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                        }\n                      },\n                      \"id\": 60766,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"3654:8:99\",\n                      \"memberName\": \"transfer\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"referencedDeclaration\": 50880,\n                      \"src\": \"3648:14:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_nonpayable$_t_address_$_t_uint256_$returns$_t_bool_$\",\n                        \"typeString\": \"function (address,uint256) returns (bool)\"\n                      }\n                    },\n                    \"id\": 60769,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"3648:26:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"functionReturnParameters\": 60759,\n                  \"id\": 60770,\n                  \"nodeType\": \"Return\",\n                  \"src\": \"3641:33:99\"\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              50880\n            ],\n            \"documentation\": {\n              \"id\": 60750,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"3436:71:99\",\n              \"text\": \" @notice Override `transfer` to add share lock check.\"\n            },\n            \"functionSelector\": \"a9059cbb\",\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"transfer\",\n            \"nameLocation\": \"3521:8:99\",\n            \"overrides\": {\n              \"id\": 60756,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"3565:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60755,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60752,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"to\",\n                  \"nameLocation\": \"3538:2:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60772,\n                  \"src\": \"3530:10:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60751,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3530:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60754,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"amount\",\n                  \"nameLocation\": \"3550:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60772,\n                  \"src\": \"3542:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60753,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3542:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"3529:28:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60759,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60758,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"\",\n                  \"nameLocation\": \"-1:-1:-1\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60772,\n                  \"src\": \"3583:4:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_bool\",\n                    \"typeString\": \"bool\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60757,\n                    \"name\": \"bool\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3583:4:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"3582:6:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"nonpayable\",\n            \"virtual\": false,\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60797,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"3767:191:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60796,\n              \"nodeType\": \"Block\",\n              \"src\": \"3862:96:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60786,\n                        \"name\": \"from\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60775,\n                        \"src\": \"3893:4:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"id\": 60785,\n                      \"name\": \"_checkIfSharesLocked\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60749,\n                      \"src\": \"3872:20:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_view$_t_address_$returns$__$\",\n                        \"typeString\": \"function (address) view\"\n                      }\n                    },\n                    \"id\": 60787,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"3872:26:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60788,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"3872:26:99\"\n                },\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60791,\n                        \"name\": \"from\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60775,\n                        \"src\": \"3934:4:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      },\n                      {\n                        \"id\": 60792,\n                        \"name\": \"to\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60777,\n                        \"src\": \"3940:2:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      },\n                      {\n                        \"id\": 60793,\n                        \"name\": \"amount\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60779,\n                        \"src\": \"3944:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      ],\n                      \"expression\": {\n                        \"id\": 60789,\n                        \"name\": \"super\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -25,\n                        \"src\": \"3915:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                          \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                        }\n                      },\n                      \"id\": 60790,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"3921:12:99\",\n                      \"memberName\": \"transferFrom\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"referencedDeclaration\": 50941,\n                      \"src\": \"3915:18:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_nonpayable$_t_address_$_t_address_$_t_uint256_$returns$_t_bool_$\",\n                        \"typeString\": \"function (address,address,uint256) returns (bool)\"\n                      }\n                    },\n                    \"id\": 60794,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"3915:36:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"functionReturnParameters\": 60784,\n                  \"id\": 60795,\n                  \"nodeType\": \"Return\",\n                  \"src\": \"3908:43:99\"\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              50941\n            ],\n            \"documentation\": {\n              \"id\": 60773,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"3687:75:99\",\n              \"text\": \" @notice Override `transferFrom` to add share lock check.\"\n            },\n            \"functionSelector\": \"23b872dd\",\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"transferFrom\",\n            \"nameLocation\": \"3776:12:99\",\n            \"overrides\": {\n              \"id\": 60781,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"3838:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60780,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60775,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"from\",\n                  \"nameLocation\": \"3797:4:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60797,\n                  \"src\": \"3789:12:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60774,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3789:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60777,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"to\",\n                  \"nameLocation\": \"3811:2:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60797,\n                  \"src\": \"3803:10:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60776,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3803:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60779,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"amount\",\n                  \"nameLocation\": \"3823:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60797,\n                  \"src\": \"3815:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60778,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3815:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"3788:42:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60784,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60783,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"\",\n                  \"nameLocation\": \"-1:-1:-1\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60797,\n                  \"src\": \"3856:4:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_bool\",\n                    \"typeString\": \"bool\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60782,\n                    \"name\": \"bool\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"3856:4:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"3855:6:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"nonpayable\",\n            \"virtual\": false,\n            \"visibility\": \"public\"\n          },\n          {\n            \"id\": 60835,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"4141:345:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60834,\n              \"nodeType\": \"Block\",\n              \"src\": \"4237:249:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60811,\n                        \"name\": \"assets\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60800,\n                        \"src\": \"4267:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60812,\n                        \"name\": \"shares\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60802,\n                        \"src\": \"4275:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60813,\n                        \"name\": \"receiver\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60804,\n                        \"src\": \"4283:8:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"expression\": {\n                        \"id\": 60808,\n                        \"name\": \"super\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -25,\n                        \"src\": \"4247:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                          \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                        }\n                      },\n                      \"id\": 60810,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"4253:13:99\",\n                      \"memberName\": \"beforeDeposit\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"referencedDeclaration\": 56805,\n                      \"src\": \"4247:19:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_view$_t_uint256_$_t_uint256_$_t_address_$returns$__$\",\n                        \"typeString\": \"function (uint256,uint256,address) view\"\n                      }\n                    },\n                    \"id\": 60814,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"4247:45:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60815,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"4247:45:99\"\n                },\n                {\n                  \"condition\": {\n                    \"commonType\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    },\n                    \"id\": 60819,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftExpression\": {\n                      \"expression\": {\n                        \"id\": 60816,\n                        \"name\": \"msg\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -15,\n                        \"src\": \"4306:3:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_magic_message\",\n                          \"typeString\": \"msg\"\n                        }\n                      },\n                      \"id\": 60817,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"4310:6:99\",\n                      \"memberName\": \"sender\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"src\": \"4306:10:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_address\",\n                        \"typeString\": \"address\"\n                      }\n                    },\n                    \"nodeType\": \"BinaryOperation\",\n                    \"operator\": \"!=\",\n                    \"rightExpression\": {\n                      \"id\": 60818,\n                      \"name\": \"receiver\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60804,\n                      \"src\": \"4320:8:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_address\",\n                        \"typeString\": \"address\"\n                      }\n                    },\n                    \"src\": \"4306:22:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"id\": 60833,\n                  \"nodeType\": \"IfStatement\",\n                  \"src\": \"4302:178:99\",\n                  \"trueBody\": {\n                    \"id\": 60832,\n                    \"nodeType\": \"Block\",\n                    \"src\": \"4330:150:99\",\n                    \"statements\": [\n                      {\n                        \"condition\": {\n                          \"id\": 60825,\n                          \"isConstant\": false,\n                          \"isLValue\": false,\n                          \"isPure\": false,\n                          \"lValueRequested\": false,\n                          \"nodeType\": \"UnaryOperation\",\n                          \"operator\": \"!\",\n                          \"prefix\": true,\n                          \"src\": \"4348:48:99\",\n                          \"subExpression\": {\n                            \"arguments\": [\n                              {\n                                \"expression\": {\n                                  \"id\": 60822,\n                                  \"name\": \"msg\",\n                                  \"nodeType\": \"Identifier\",\n                                  \"overloadedDeclarations\": [],\n                                  \"referencedDeclaration\": -15,\n                                  \"src\": \"4385:3:99\",\n                                  \"typeDescriptions\": {\n                                    \"typeIdentifier\": \"t_magic_message\",\n                                    \"typeString\": \"msg\"\n                                  }\n                                },\n                                \"id\": 60823,\n                                \"isConstant\": false,\n                                \"isLValue\": false,\n                                \"isPure\": false,\n                                \"lValueRequested\": false,\n                                \"memberLocation\": \"4389:6:99\",\n                                \"memberName\": \"sender\",\n                                \"nodeType\": \"MemberAccess\",\n                                \"src\": \"4385:10:99\",\n                                \"typeDescriptions\": {\n                                  \"typeIdentifier\": \"t_address\",\n                                  \"typeString\": \"address\"\n                                }\n                              }\n                            ],\n                            \"expression\": {\n                              \"argumentTypes\": [\n                                {\n                                  \"typeIdentifier\": \"t_address\",\n                                  \"typeString\": \"address\"\n                                }\n                              ],\n                              \"expression\": {\n                                \"id\": 60820,\n                                \"name\": \"registry\",\n                                \"nodeType\": \"Identifier\",\n                                \"overloadedDeclarations\": [],\n                                \"referencedDeclaration\": 56665,\n                                \"src\": \"4349:8:99\",\n                                \"typeDescriptions\": {\n                                  \"typeIdentifier\": \"t_contract$_Registry_$55623\",\n                                  \"typeString\": \"contract Registry\"\n                                }\n                              },\n                              \"id\": 60821,\n                              \"isConstant\": false,\n                              \"isLValue\": false,\n                              \"isPure\": false,\n                              \"lValueRequested\": false,\n                              \"memberLocation\": \"4358:26:99\",\n                              \"memberName\": \"approvedForDepositOnBehalf\",\n                              \"nodeType\": \"MemberAccess\",\n                              \"referencedDeclaration\": 54708,\n                              \"src\": \"4349:35:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_function_external_view$_t_address_$returns$_t_bool_$\",\n                                \"typeString\": \"function (address) view external returns (bool)\"\n                              }\n                            },\n                            \"id\": 60824,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": false,\n                            \"kind\": \"functionCall\",\n                            \"lValueRequested\": false,\n                            \"nameLocations\": [],\n                            \"names\": [],\n                            \"nodeType\": \"FunctionCall\",\n                            \"src\": \"4349:47:99\",\n                            \"tryCall\": false,\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_bool\",\n                              \"typeString\": \"bool\"\n                            }\n                          },\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_bool\",\n                            \"typeString\": \"bool\"\n                          }\n                        },\n                        \"id\": 60831,\n                        \"nodeType\": \"IfStatement\",\n                        \"src\": \"4344:125:99\",\n                        \"trueBody\": {\n                          \"errorCall\": {\n                            \"arguments\": [\n                              {\n                                \"expression\": {\n                                  \"id\": 60827,\n                                  \"name\": \"msg\",\n                                  \"nodeType\": \"Identifier\",\n                                  \"overloadedDeclarations\": [],\n                                  \"referencedDeclaration\": -15,\n                                  \"src\": \"4458:3:99\",\n                                  \"typeDescriptions\": {\n                                    \"typeIdentifier\": \"t_magic_message\",\n                                    \"typeString\": \"msg\"\n                                  }\n                                },\n                                \"id\": 60828,\n                                \"isConstant\": false,\n                                \"isLValue\": false,\n                                \"isPure\": false,\n                                \"lValueRequested\": false,\n                                \"memberLocation\": \"4462:6:99\",\n                                \"memberName\": \"sender\",\n                                \"nodeType\": \"MemberAccess\",\n                                \"src\": \"4458:10:99\",\n                                \"typeDescriptions\": {\n                                  \"typeIdentifier\": \"t_address\",\n                                  \"typeString\": \"address\"\n                                }\n                              }\n                            ],\n                            \"expression\": {\n                              \"argumentTypes\": [\n                                {\n                                  \"typeIdentifier\": \"t_address\",\n                                  \"typeString\": \"address\"\n                                }\n                              ],\n                              \"id\": 60826,\n                              \"name\": \"Cellar__NotApprovedToDepositOnBehalf\",\n                              \"nodeType\": \"Identifier\",\n                              \"overloadedDeclarations\": [],\n                              \"referencedDeclaration\": 60662,\n                              \"src\": \"4421:36:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_function_error_pure$_t_address_$returns$__$\",\n                                \"typeString\": \"function (address) pure\"\n                              }\n                            },\n                            \"id\": 60829,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": false,\n                            \"kind\": \"functionCall\",\n                            \"lValueRequested\": false,\n                            \"nameLocations\": [],\n                            \"names\": [],\n                            \"nodeType\": \"FunctionCall\",\n                            \"src\": \"4421:48:99\",\n                            \"tryCall\": false,\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_tuple$__$\",\n                              \"typeString\": \"tuple()\"\n                            }\n                          },\n                          \"id\": 60830,\n                          \"nodeType\": \"RevertStatement\",\n                          \"src\": \"4414:55:99\"\n                        }\n                      }\n                    ]\n                  }\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              56805\n            ],\n            \"documentation\": {\n              \"id\": 60798,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"3964:172:99\",\n              \"text\": \" @notice called at the beginning of deposit.\\n @param assets amount of assets deposited by user.\\n @param receiver address receiving the shares.\"\n            },\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"beforeDeposit\",\n            \"nameLocation\": \"4150:13:99\",\n            \"overrides\": {\n              \"id\": 60806,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"4228:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60805,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60800,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"assets\",\n                  \"nameLocation\": \"4172:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60835,\n                  \"src\": \"4164:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60799,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4164:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60802,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"shares\",\n                  \"nameLocation\": \"4188:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60835,\n                  \"src\": \"4180:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60801,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4180:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60804,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"receiver\",\n                  \"nameLocation\": \"4204:8:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60835,\n                  \"src\": \"4196:16:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60803,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4196:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"4163:50:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60807,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"4237:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"view\",\n            \"virtual\": false,\n            \"visibility\": \"internal\"\n          },\n          {\n            \"id\": 60862,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"4610:211:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60861,\n              \"nodeType\": \"Block\",\n              \"src\": \"4700:121:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"id\": 60851,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftHandSide\": {\n                      \"baseExpression\": {\n                        \"id\": 60846,\n                        \"name\": \"userShareLockStartTime\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60681,\n                        \"src\": \"4710:22:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_mapping$_t_address_$_t_uint256_$\",\n                          \"typeString\": \"mapping(address => uint256)\"\n                        }\n                      },\n                      \"id\": 60848,\n                      \"indexExpression\": {\n                        \"id\": 60847,\n                        \"name\": \"receiver\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60842,\n                        \"src\": \"4733:8:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      },\n                      \"isConstant\": false,\n                      \"isLValue\": true,\n                      \"isPure\": false,\n                      \"lValueRequested\": true,\n                      \"nodeType\": \"IndexAccess\",\n                      \"src\": \"4710:32:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"nodeType\": \"Assignment\",\n                    \"operator\": \"=\",\n                    \"rightHandSide\": {\n                      \"expression\": {\n                        \"id\": 60849,\n                        \"name\": \"block\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -4,\n                        \"src\": \"4745:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_magic_block\",\n                          \"typeString\": \"block\"\n                        }\n                      },\n                      \"id\": 60850,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"4751:9:99\",\n                      \"memberName\": \"timestamp\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"src\": \"4745:15:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"src\": \"4710:50:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"id\": 60852,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"4710:50:99\"\n                },\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60856,\n                        \"name\": \"assets\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60838,\n                        \"src\": \"4789:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60857,\n                        \"name\": \"shares\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60840,\n                        \"src\": \"4797:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60858,\n                        \"name\": \"receiver\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60842,\n                        \"src\": \"4805:8:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"expression\": {\n                        \"id\": 60853,\n                        \"name\": \"super\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -25,\n                        \"src\": \"4770:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                          \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                        }\n                      },\n                      \"id\": 60855,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"4776:12:99\",\n                      \"memberName\": \"afterDeposit\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"referencedDeclaration\": 56821,\n                      \"src\": \"4770:18:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_nonpayable$_t_uint256_$_t_uint256_$_t_address_$returns$__$\",\n                        \"typeString\": \"function (uint256,uint256,address)\"\n                      }\n                    },\n                    \"id\": 60859,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"4770:44:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60860,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"4770:44:99\"\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              56821\n            ],\n            \"documentation\": {\n              \"id\": 60836,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"4492:113:99\",\n              \"text\": \" @notice called at the end of deposit.\\n @param assets amount of assets deposited by user.\"\n            },\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"afterDeposit\",\n            \"nameLocation\": \"4619:12:99\",\n            \"overrides\": {\n              \"id\": 60844,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"4691:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60843,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60838,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"assets\",\n                  \"nameLocation\": \"4640:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60862,\n                  \"src\": \"4632:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60837,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4632:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60840,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"shares\",\n                  \"nameLocation\": \"4656:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60862,\n                  \"src\": \"4648:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60839,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4648:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60842,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"receiver\",\n                  \"nameLocation\": \"4672:8:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60862,\n                  \"src\": \"4664:16:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60841,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4664:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"4631:50:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60845,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"4700:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"nonpayable\",\n            \"virtual\": false,\n            \"visibility\": \"internal\"\n          },\n          {\n            \"id\": 60889,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"4895:219:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60888,\n              \"nodeType\": \"Block\",\n              \"src\": \"5007:107:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60876,\n                        \"name\": \"owner\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60871,\n                        \"src\": \"5038:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"id\": 60875,\n                      \"name\": \"_checkIfSharesLocked\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60749,\n                      \"src\": \"5017:20:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_view$_t_address_$returns$__$\",\n                        \"typeString\": \"function (address) view\"\n                      }\n                    },\n                    \"id\": 60877,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"5017:27:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60878,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"5017:27:99\"\n                },\n                {\n                  \"expression\": {\n                    \"arguments\": [\n                      {\n                        \"id\": 60882,\n                        \"name\": \"assets\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60865,\n                        \"src\": \"5075:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60883,\n                        \"name\": \"shares\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60867,\n                        \"src\": \"5083:6:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      {\n                        \"id\": 60884,\n                        \"name\": \"receiver\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60869,\n                        \"src\": \"5091:8:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      },\n                      {\n                        \"id\": 60885,\n                        \"name\": \"owner\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": 60871,\n                        \"src\": \"5101:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      }\n                    ],\n                    \"expression\": {\n                      \"argumentTypes\": [\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        },\n                        {\n                          \"typeIdentifier\": \"t_address\",\n                          \"typeString\": \"address\"\n                        }\n                      ],\n                      \"expression\": {\n                        \"id\": 60879,\n                        \"name\": \"super\",\n                        \"nodeType\": \"Identifier\",\n                        \"overloadedDeclarations\": [],\n                        \"referencedDeclaration\": -25,\n                        \"src\": \"5054:5:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                          \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                        }\n                      },\n                      \"id\": 60881,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"lValueRequested\": false,\n                      \"memberLocation\": \"5060:14:99\",\n                      \"memberName\": \"beforeWithdraw\",\n                      \"nodeType\": \"MemberAccess\",\n                      \"referencedDeclaration\": 56837,\n                      \"src\": \"5054:20:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_function_internal_view$_t_uint256_$_t_uint256_$_t_address_$_t_address_$returns$__$\",\n                        \"typeString\": \"function (uint256,uint256,address,address) view\"\n                      }\n                    },\n                    \"id\": 60886,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"kind\": \"functionCall\",\n                    \"lValueRequested\": false,\n                    \"nameLocations\": [],\n                    \"names\": [],\n                    \"nodeType\": \"FunctionCall\",\n                    \"src\": \"5054:53:99\",\n                    \"tryCall\": false,\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_tuple$__$\",\n                      \"typeString\": \"tuple()\"\n                    }\n                  },\n                  \"id\": 60887,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"5054:53:99\"\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              56837\n            ],\n            \"documentation\": {\n              \"id\": 60863,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"4827:63:99\",\n              \"text\": \" @notice called at the beginning of withdraw.\"\n            },\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"beforeWithdraw\",\n            \"nameLocation\": \"4904:14:99\",\n            \"overrides\": {\n              \"id\": 60873,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"4998:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60872,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60865,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"assets\",\n                  \"nameLocation\": \"4927:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60889,\n                  \"src\": \"4919:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60864,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4919:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60867,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"shares\",\n                  \"nameLocation\": \"4943:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60889,\n                  \"src\": \"4935:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60866,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4935:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60869,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"receiver\",\n                  \"nameLocation\": \"4959:8:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60889,\n                  \"src\": \"4951:16:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60868,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4951:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60871,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"owner\",\n                  \"nameLocation\": \"4977:5:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60889,\n                  \"src\": \"4969:13:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60870,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"4969:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"4918:65:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60874,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [],\n              \"src\": \"5007:0:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"view\",\n            \"virtual\": false,\n            \"visibility\": \"internal\"\n          },\n          {\n            \"id\": 60933,\n            \"nodeType\": \"FunctionDefinition\",\n            \"src\": \"5419:389:99\",\n            \"nodes\": [],\n            \"body\": {\n              \"id\": 60932,\n              \"nodeType\": \"Block\",\n              \"src\": \"5515:293:99\",\n              \"nodes\": [],\n              \"statements\": [\n                {\n                  \"expression\": {\n                    \"id\": 60906,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftHandSide\": {\n                      \"id\": 60900,\n                      \"name\": \"maxOut\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60898,\n                      \"src\": \"5525:6:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"nodeType\": \"Assignment\",\n                    \"operator\": \"=\",\n                    \"rightHandSide\": {\n                      \"arguments\": [\n                        {\n                          \"id\": 60903,\n                          \"name\": \"owner\",\n                          \"nodeType\": \"Identifier\",\n                          \"overloadedDeclarations\": [],\n                          \"referencedDeclaration\": 60892,\n                          \"src\": \"5549:5:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_address\",\n                            \"typeString\": \"address\"\n                          }\n                        },\n                        {\n                          \"id\": 60904,\n                          \"name\": \"inShares\",\n                          \"nodeType\": \"Identifier\",\n                          \"overloadedDeclarations\": [],\n                          \"referencedDeclaration\": 60894,\n                          \"src\": \"5556:8:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_bool\",\n                            \"typeString\": \"bool\"\n                          }\n                        }\n                      ],\n                      \"expression\": {\n                        \"argumentTypes\": [\n                          {\n                            \"typeIdentifier\": \"t_address\",\n                            \"typeString\": \"address\"\n                          },\n                          {\n                            \"typeIdentifier\": \"t_bool\",\n                            \"typeString\": \"bool\"\n                          }\n                        ],\n                        \"expression\": {\n                          \"id\": 60901,\n                          \"name\": \"super\",\n                          \"nodeType\": \"Identifier\",\n                          \"overloadedDeclarations\": [],\n                          \"referencedDeclaration\": -25,\n                          \"src\": \"5534:5:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_type$_t_super$_CellarWithShareLockPeriod_$60934_$\",\n                            \"typeString\": \"type(contract super CellarWithShareLockPeriod)\"\n                          }\n                        },\n                        \"id\": 60902,\n                        \"isConstant\": false,\n                        \"isLValue\": false,\n                        \"isPure\": false,\n                        \"lValueRequested\": false,\n                        \"memberLocation\": \"5540:8:99\",\n                        \"memberName\": \"_findMax\",\n                        \"nodeType\": \"MemberAccess\",\n                        \"referencedDeclaration\": 57832,\n                        \"src\": \"5534:14:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_function_internal_view$_t_address_$_t_bool_$returns$_t_uint256_$\",\n                          \"typeString\": \"function (address,bool) view returns (uint256)\"\n                        }\n                      },\n                      \"id\": 60905,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": false,\n                      \"kind\": \"functionCall\",\n                      \"lValueRequested\": false,\n                      \"nameLocations\": [],\n                      \"names\": [],\n                      \"nodeType\": \"FunctionCall\",\n                      \"src\": \"5534:31:99\",\n                      \"tryCall\": false,\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"src\": \"5525:40:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"id\": 60907,\n                  \"nodeType\": \"ExpressionStatement\",\n                  \"src\": \"5525:40:99\"\n                },\n                {\n                  \"assignments\": [\n                    60909\n                  ],\n                  \"declarations\": [\n                    {\n                      \"constant\": false,\n                      \"id\": 60909,\n                      \"mutability\": \"mutable\",\n                      \"name\": \"lockTime\",\n                      \"nameLocation\": \"5583:8:99\",\n                      \"nodeType\": \"VariableDeclaration\",\n                      \"scope\": 60932,\n                      \"src\": \"5575:16:99\",\n                      \"stateVariable\": false,\n                      \"storageLocation\": \"default\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      },\n                      \"typeName\": {\n                        \"id\": 60908,\n                        \"name\": \"uint256\",\n                        \"nodeType\": \"ElementaryTypeName\",\n                        \"src\": \"5575:7:99\",\n                        \"typeDescriptions\": {\n                          \"typeIdentifier\": \"t_uint256\",\n                          \"typeString\": \"uint256\"\n                        }\n                      },\n                      \"visibility\": \"internal\"\n                    }\n                  ],\n                  \"id\": 60913,\n                  \"initialValue\": {\n                    \"baseExpression\": {\n                      \"id\": 60910,\n                      \"name\": \"userShareLockStartTime\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60681,\n                      \"src\": \"5594:22:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_mapping$_t_address_$_t_uint256_$\",\n                        \"typeString\": \"mapping(address => uint256)\"\n                      }\n                    },\n                    \"id\": 60912,\n                    \"indexExpression\": {\n                      \"id\": 60911,\n                      \"name\": \"owner\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60892,\n                      \"src\": \"5617:5:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_address\",\n                        \"typeString\": \"address\"\n                      }\n                    },\n                    \"isConstant\": false,\n                    \"isLValue\": true,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"nodeType\": \"IndexAccess\",\n                    \"src\": \"5594:29:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"nodeType\": \"VariableDeclarationStatement\",\n                  \"src\": \"5575:48:99\"\n                },\n                {\n                  \"condition\": {\n                    \"commonType\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    },\n                    \"id\": 60916,\n                    \"isConstant\": false,\n                    \"isLValue\": false,\n                    \"isPure\": false,\n                    \"lValueRequested\": false,\n                    \"leftExpression\": {\n                      \"id\": 60914,\n                      \"name\": \"lockTime\",\n                      \"nodeType\": \"Identifier\",\n                      \"overloadedDeclarations\": [],\n                      \"referencedDeclaration\": 60909,\n                      \"src\": \"5637:8:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_uint256\",\n                        \"typeString\": \"uint256\"\n                      }\n                    },\n                    \"nodeType\": \"BinaryOperation\",\n                    \"operator\": \"!=\",\n                    \"rightExpression\": {\n                      \"hexValue\": \"30\",\n                      \"id\": 60915,\n                      \"isConstant\": false,\n                      \"isLValue\": false,\n                      \"isPure\": true,\n                      \"kind\": \"number\",\n                      \"lValueRequested\": false,\n                      \"nodeType\": \"Literal\",\n                      \"src\": \"5649:1:99\",\n                      \"typeDescriptions\": {\n                        \"typeIdentifier\": \"t_rational_0_by_1\",\n                        \"typeString\": \"int_const 0\"\n                      },\n                      \"value\": \"0\"\n                    },\n                    \"src\": \"5637:13:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"id\": 60931,\n                  \"nodeType\": \"IfStatement\",\n                  \"src\": \"5633:169:99\",\n                  \"trueBody\": {\n                    \"id\": 60930,\n                    \"nodeType\": \"Block\",\n                    \"src\": \"5652:150:99\",\n                    \"statements\": [\n                      {\n                        \"assignments\": [\n                          60918\n                        ],\n                        \"declarations\": [\n                          {\n                            \"constant\": false,\n                            \"id\": 60918,\n                            \"mutability\": \"mutable\",\n                            \"name\": \"timeSharesAreUnlocked\",\n                            \"nameLocation\": \"5674:21:99\",\n                            \"nodeType\": \"VariableDeclaration\",\n                            \"scope\": 60930,\n                            \"src\": \"5666:29:99\",\n                            \"stateVariable\": false,\n                            \"storageLocation\": \"default\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            },\n                            \"typeName\": {\n                              \"id\": 60917,\n                              \"name\": \"uint256\",\n                              \"nodeType\": \"ElementaryTypeName\",\n                              \"src\": \"5666:7:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_uint256\",\n                                \"typeString\": \"uint256\"\n                              }\n                            },\n                            \"visibility\": \"internal\"\n                          }\n                        ],\n                        \"id\": 60922,\n                        \"initialValue\": {\n                          \"commonType\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          },\n                          \"id\": 60921,\n                          \"isConstant\": false,\n                          \"isLValue\": false,\n                          \"isPure\": false,\n                          \"lValueRequested\": false,\n                          \"leftExpression\": {\n                            \"id\": 60919,\n                            \"name\": \"lockTime\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60909,\n                            \"src\": \"5698:8:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"nodeType\": \"BinaryOperation\",\n                          \"operator\": \"+\",\n                          \"rightExpression\": {\n                            \"id\": 60920,\n                            \"name\": \"shareLockPeriod\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60676,\n                            \"src\": \"5709:15:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"src\": \"5698:26:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          }\n                        },\n                        \"nodeType\": \"VariableDeclarationStatement\",\n                        \"src\": \"5666:58:99\"\n                      },\n                      {\n                        \"condition\": {\n                          \"commonType\": {\n                            \"typeIdentifier\": \"t_uint256\",\n                            \"typeString\": \"uint256\"\n                          },\n                          \"id\": 60926,\n                          \"isConstant\": false,\n                          \"isLValue\": false,\n                          \"isPure\": false,\n                          \"lValueRequested\": false,\n                          \"leftExpression\": {\n                            \"id\": 60923,\n                            \"name\": \"timeSharesAreUnlocked\",\n                            \"nodeType\": \"Identifier\",\n                            \"overloadedDeclarations\": [],\n                            \"referencedDeclaration\": 60918,\n                            \"src\": \"5742:21:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"nodeType\": \"BinaryOperation\",\n                          \"operator\": \">\",\n                          \"rightExpression\": {\n                            \"expression\": {\n                              \"id\": 60924,\n                              \"name\": \"block\",\n                              \"nodeType\": \"Identifier\",\n                              \"overloadedDeclarations\": [],\n                              \"referencedDeclaration\": -4,\n                              \"src\": \"5766:5:99\",\n                              \"typeDescriptions\": {\n                                \"typeIdentifier\": \"t_magic_block\",\n                                \"typeString\": \"block\"\n                              }\n                            },\n                            \"id\": 60925,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": false,\n                            \"lValueRequested\": false,\n                            \"memberLocation\": \"5772:9:99\",\n                            \"memberName\": \"timestamp\",\n                            \"nodeType\": \"MemberAccess\",\n                            \"src\": \"5766:15:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_uint256\",\n                              \"typeString\": \"uint256\"\n                            }\n                          },\n                          \"src\": \"5742:39:99\",\n                          \"typeDescriptions\": {\n                            \"typeIdentifier\": \"t_bool\",\n                            \"typeString\": \"bool\"\n                          }\n                        },\n                        \"id\": 60929,\n                        \"nodeType\": \"IfStatement\",\n                        \"src\": \"5738:53:99\",\n                        \"trueBody\": {\n                          \"expression\": {\n                            \"hexValue\": \"30\",\n                            \"id\": 60927,\n                            \"isConstant\": false,\n                            \"isLValue\": false,\n                            \"isPure\": true,\n                            \"kind\": \"number\",\n                            \"lValueRequested\": false,\n                            \"nodeType\": \"Literal\",\n                            \"src\": \"5790:1:99\",\n                            \"typeDescriptions\": {\n                              \"typeIdentifier\": \"t_rational_0_by_1\",\n                              \"typeString\": \"int_const 0\"\n                            },\n                            \"value\": \"0\"\n                          },\n                          \"functionReturnParameters\": 60899,\n                          \"id\": 60928,\n                          \"nodeType\": \"Return\",\n                          \"src\": \"5783:8:99\"\n                        }\n                      }\n                    ]\n                  }\n                }\n              ]\n            },\n            \"baseFunctions\": [\n              57832\n            ],\n            \"documentation\": {\n              \"id\": 60890,\n              \"nodeType\": \"StructuredDocumentation\",\n              \"src\": \"5120:294:99\",\n              \"text\": \" @notice Finds the max amount of value an `owner` can remove from the cellar.\\n @param owner address of the user to find max value.\\n @param inShares if false, then returns value in terms of assets\\n                 if true then returns value in terms of shares\"\n            },\n            \"implemented\": true,\n            \"kind\": \"function\",\n            \"modifiers\": [],\n            \"name\": \"_findMax\",\n            \"nameLocation\": \"5428:8:99\",\n            \"overrides\": {\n              \"id\": 60896,\n              \"nodeType\": \"OverrideSpecifier\",\n              \"overrides\": [],\n              \"src\": \"5481:8:99\"\n            },\n            \"parameters\": {\n              \"id\": 60895,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60892,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"owner\",\n                  \"nameLocation\": \"5445:5:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60933,\n                  \"src\": \"5437:13:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_address\",\n                    \"typeString\": \"address\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60891,\n                    \"name\": \"address\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"5437:7:99\",\n                    \"stateMutability\": \"nonpayable\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_address\",\n                      \"typeString\": \"address\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                },\n                {\n                  \"constant\": false,\n                  \"id\": 60894,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"inShares\",\n                  \"nameLocation\": \"5457:8:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60933,\n                  \"src\": \"5452:13:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_bool\",\n                    \"typeString\": \"bool\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60893,\n                    \"name\": \"bool\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"5452:4:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_bool\",\n                      \"typeString\": \"bool\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"5436:30:99\"\n            },\n            \"returnParameters\": {\n              \"id\": 60899,\n              \"nodeType\": \"ParameterList\",\n              \"parameters\": [\n                {\n                  \"constant\": false,\n                  \"id\": 60898,\n                  \"mutability\": \"mutable\",\n                  \"name\": \"maxOut\",\n                  \"nameLocation\": \"5507:6:99\",\n                  \"nodeType\": \"VariableDeclaration\",\n                  \"scope\": 60933,\n                  \"src\": \"5499:14:99\",\n                  \"stateVariable\": false,\n                  \"storageLocation\": \"default\",\n                  \"typeDescriptions\": {\n                    \"typeIdentifier\": \"t_uint256\",\n                    \"typeString\": \"uint256\"\n                  },\n                  \"typeName\": {\n                    \"id\": 60897,\n                    \"name\": \"uint256\",\n                    \"nodeType\": \"ElementaryTypeName\",\n                    \"src\": \"5499:7:99\",\n                    \"typeDescriptions\": {\n                      \"typeIdentifier\": \"t_uint256\",\n                      \"typeString\": \"uint256\"\n                    }\n                  },\n                  \"visibility\": \"internal\"\n                }\n              ],\n              \"src\": \"5498:16:99\"\n            },\n            \"scope\": 60934,\n            \"stateMutability\": \"view\",\n            \"virtual\": false,\n            \"visibility\": \"internal\"\n          }\n        ],\n        \"abstract\": false,\n        \"baseContracts\": [\n          {\n            \"baseName\": {\n              \"id\": 60598,\n              \"name\": \"Cellar\",\n              \"nameLocations\": [\n                \"172:6:99\"\n              ],\n              \"nodeType\": \"IdentifierPath\",\n              \"referencedDeclaration\": 58760,\n              \"src\": \"172:6:99\"\n            },\n            \"id\": 60599,\n            \"nodeType\": \"InheritanceSpecifier\",\n            \"src\": \"172:6:99\"\n          }\n        ],\n        \"canonicalName\": \"CellarWithShareLockPeriod\",\n        \"contractDependencies\": [],\n        \"contractKind\": \"contract\",\n        \"fullyImplemented\": true,\n        \"linearizedBaseContracts\": [\n          60934,\n          58760,\n          46579,\n          46494,\n          50221,\n          50742,\n          51130\n        ],\n        \"name\": \"CellarWithShareLockPeriod\",\n        \"nameLocation\": \"143:25:99\",\n        \"scope\": 60935,\n        \"usedErrors\": [\n          55862,\n          55867,\n          55872,\n          55879,\n          55886,\n          55891,\n          55896,\n          55899,\n          55904,\n          55907,\n          56445,\n          56448,\n          56536,\n          56539,\n          56542,\n          56661,\n          56775,\n          56778,\n          56783,\n          56788,\n          57961,\n          58009,\n          58016,\n          58023,\n          58028,\n          58273,\n          58276,\n          58589,\n          58592,\n          60650,\n          60657,\n          60662\n        ],\n        \"usedEvents\": [\n          50170,\n          50249,\n          50261,\n          50753,\n          50761,\n          55825,\n          55832,\n          55843,\n          55850,\n          55857,\n          56435,\n          56442,\n          56533,\n          57966,\n          58000,\n          58081,\n          60647\n        ]\n      }\n    ],\n    \"license\": \"Apache-2.0\"\n  },\n  \"id\": 99\n}") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CellarWithShareLockPeriodV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CellarWithShareLockPeriodV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CellarWithShareLockPeriodV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CellarWithShareLockPeriodV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CellarWithShareLockPeriodV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                CELLARWITHSHARELOCKPERIODV1_ABI.clone(),
                client,
            );
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
        #[doc = "Calls the contract's `adaptorCatalogue` (0x18d4c143) function"]
        pub fn adaptor_catalogue(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 212, 193, 67], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addAdaptorToCatalogue` (0x3d8ab1e5) function"]
        pub fn add_adaptor_to_catalogue(
            &self,
            adaptor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 138, 177, 229], adaptor)
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
        #[doc = "Calls the contract's `addPositionToCatalogue` (0x501eb4fe) function"]
        pub fn add_position_to_catalogue(
            &self,
            position_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 30, 180, 254], position_id)
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
        #[doc = "Calls the contract's `automationActions` (0x88c4caba) function"]
        pub fn automation_actions(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([136, 196, 202, 186], ())
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
        #[doc = "Calls the contract's `cachePriceRouter` (0xc588d8d6) function"]
        pub fn cache_price_router(
            &self,
            check_total_assets: bool,
            allowable_range: u16,
            expected_price_router: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [197, 136, 216, 214],
                    (check_total_assets, allowable_range, expected_price_router),
                )
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
        #[doc = "Calls the contract's `decreaseShareSupplyCap` (0x575bbce6) function"]
        pub fn decrease_share_supply_cap(
            &self,
            new_share_supply_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 91, 188, 230], new_share_supply_cap)
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
        #[doc = "Calls the contract's `forcePositionOut` (0xa07bee0b) function"]
        pub fn force_position_out(
            &self,
            index: u32,
            position_id: u32,
            in_debt_array: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 123, 238, 11], (index, position_id, in_debt_array))
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
        #[doc = "Calls the contract's `ignorePause` (0x9959af94) function"]
        pub fn ignore_pause(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 89, 175, 148], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseShareSupplyCap` (0xb0646e27) function"]
        pub fn increase_share_supply_cap(
            &self,
            new_share_supply_cap: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 100, 110, 39], new_share_supply_cap)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initiateShutdown` (0x0a680e18) function"]
        pub fn initiate_shutdown(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 104, 14, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPaused` (0xb187bd26) function"]
        pub fn is_paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
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
        pub fn locked(&self) -> ethers::contract::builders::ContractCall<M, bool> {
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
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<ethers::core::types::Bytes>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        #[doc = "Calls the contract's `positionCatalogue` (0xcbdf33d0) function"]
        pub fn position_catalogue(
            &self,
            p0: u32,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 223, 51, 208], p0)
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
        #[doc = "Calls the contract's `priceRouter` (0xd7d4bf45) function"]
        pub fn price_router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([215, 212, 191, 69], ())
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
        #[doc = "Calls the contract's `removeAdaptorFromCatalogue` (0x5f6b88a0) function"]
        pub fn remove_adaptor_from_catalogue(
            &self,
            adaptor: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 107, 136, 160], adaptor)
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
        #[doc = "Calls the contract's `removePositionFromCatalogue` (0xd1e88404) function"]
        pub fn remove_position_from_catalogue(
            &self,
            position_id: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 232, 132, 4], position_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAutomationActions` (0xc8e81950) function"]
        pub fn set_automation_actions(
            &self,
            registry_id: ethers::core::types::U256,
            expected_automation_actions: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 232, 25, 80],
                    (registry_id, expected_automation_actions),
                )
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
        #[doc = "Calls the contract's `shareLockPeriod` (0x9fdb11b6) function"]
        pub fn share_lock_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 219, 17, 182], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shareSupplyCap` (0xd446bbcc) function"]
        pub fn share_supply_cap(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([212, 70, 187, 204], ())
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
        #[doc = "Calls the contract's `toggleIgnorePause` (0xa373e3ff) function"]
        pub fn toggle_ignore_pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 115, 227, 255], ())
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
        #[doc = "Calls the contract's `viewPositionBalances` (0x78e0233e) function"]
        pub fn view_position_balances(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<ethers::core::types::Address>,
                ::std::vec::Vec<ethers::core::types::U256>,
                ::std::vec::Vec<bool>,
            ),
        > {
            self.0
                .method_hash([120, 224, 35, 62], ())
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
        #[doc = "Gets the contract's `AdaptorCalled` event"]
        pub fn adaptor_called_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdaptorCalledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AdaptorCatalogueAltered` event"]
        pub fn adaptor_catalogue_altered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdaptorCatalogueAlteredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Cellar__AutomationActionsUpdated` event"]
        pub fn cellar_automation_actions_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CellarAutomationActionsUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionAdded` event"]
        pub fn position_added_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionAddedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PositionCatalogueAltered` event"]
        pub fn position_catalogue_altered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PositionCatalogueAlteredFilter> {
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
        pub fn events(
            &self,
        ) -> ethers::contract::builders::Event<M, CellarWithShareLockPeriodV1Events> {
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
    #[ethevent(name = "AdaptorCalled", abi = "AdaptorCalled(address,bytes)")]
    pub struct AdaptorCalledFilter {
        pub adaptor: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
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
        name = "AdaptorCatalogueAltered",
        abi = "AdaptorCatalogueAltered(address,bool)"
    )]
    pub struct AdaptorCatalogueAlteredFilter {
        pub adaptor: ethers::core::types::Address,
        pub in_catalogue: bool,
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
    #[ethevent(
        name = "Cellar__AutomationActionsUpdated",
        abi = "Cellar__AutomationActionsUpdated(address)"
    )]
    pub struct CellarAutomationActionsUpdatedFilter {
        pub new_automation_actions: ethers::core::types::Address,
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
    #[ethevent(
        name = "PositionCatalogueAltered",
        abi = "PositionCatalogueAltered(uint32,bool)"
    )]
    pub struct PositionCatalogueAlteredFilter {
        pub position_id: u32,
        pub in_catalogue: bool,
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
    pub enum CellarWithShareLockPeriodV1Events {
        AdaptorCalledFilter(AdaptorCalledFilter),
        AdaptorCatalogueAlteredFilter(AdaptorCatalogueAlteredFilter),
        ApprovalFilter(ApprovalFilter),
        CellarAutomationActionsUpdatedFilter(CellarAutomationActionsUpdatedFilter),
        DepositFilter(DepositFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionCatalogueAlteredFilter(PositionCatalogueAlteredFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        ShareLockingPeriodChangedFilter(ShareLockingPeriodChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for CellarWithShareLockPeriodV1Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdaptorCalledFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::AdaptorCalledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = AdaptorCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::AdaptorCatalogueAlteredFilter(decoded),
                );
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = CellarAutomationActionsUpdatedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::CellarAutomationActionsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::PositionAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PositionCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::PositionCatalogueAlteredFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::PositionRemovedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::PositionSwappedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::RebalanceDeviationChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ShareLockingPeriodChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::ShareLockingPeriodChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::ShutdownChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::StrategistPayoutAddressChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::StrategistPlatformCutChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CellarWithShareLockPeriodV1Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarWithShareLockPeriodV1Events::AdaptorCalledFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::AdaptorCatalogueAlteredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::ApprovalFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::CellarAutomationActionsUpdatedFilter(
                    element,
                ) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::DepositFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::PositionAddedFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::PositionCatalogueAlteredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::PositionRemovedFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::PositionSwappedFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::RebalanceDeviationChangedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::ShareLockingPeriodChangedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::ShutdownChangedFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::StrategistPayoutAddressChangedFilter(
                    element,
                ) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::StrategistPlatformCutChangedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Events::TransferFilter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Events::WithdrawFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `adaptorCatalogue`function with signature `adaptorCatalogue(address)` and selector `[24, 212, 193, 67]`"]
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
    #[ethcall(name = "adaptorCatalogue", abi = "adaptorCatalogue(address)")]
    pub struct AdaptorCatalogueCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `addAdaptorToCatalogue`function with signature `addAdaptorToCatalogue(address)` and selector `[61, 138, 177, 229]`"]
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
    #[ethcall(name = "addAdaptorToCatalogue", abi = "addAdaptorToCatalogue(address)")]
    pub struct AddAdaptorToCatalogueCall {
        pub adaptor: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `addPositionToCatalogue`function with signature `addPositionToCatalogue(uint32)` and selector `[80, 30, 180, 254]`"]
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
        name = "addPositionToCatalogue",
        abi = "addPositionToCatalogue(uint32)"
    )]
    pub struct AddPositionToCatalogueCall {
        pub position_id: u32,
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
    #[doc = "Container type for all input parameters for the `automationActions`function with signature `automationActions()` and selector `[136, 196, 202, 186]`"]
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
    #[ethcall(name = "automationActions", abi = "automationActions()")]
    pub struct AutomationActionsCall;
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
    #[doc = "Container type for all input parameters for the `cachePriceRouter`function with signature `cachePriceRouter(bool,uint16,address)` and selector `[197, 136, 216, 214]`"]
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
        name = "cachePriceRouter",
        abi = "cachePriceRouter(bool,uint16,address)"
    )]
    pub struct CachePriceRouterCall {
        pub check_total_assets: bool,
        pub allowable_range: u16,
        pub expected_price_router: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `decreaseShareSupplyCap`function with signature `decreaseShareSupplyCap(uint192)` and selector `[87, 91, 188, 230]`"]
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
        name = "decreaseShareSupplyCap",
        abi = "decreaseShareSupplyCap(uint192)"
    )]
    pub struct DecreaseShareSupplyCapCall {
        pub new_share_supply_cap: ethers::core::types::U256,
    }
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
    #[doc = "Container type for all input parameters for the `forcePositionOut`function with signature `forcePositionOut(uint32,uint32,bool)` and selector `[160, 123, 238, 11]`"]
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
        name = "forcePositionOut",
        abi = "forcePositionOut(uint32,uint32,bool)"
    )]
    pub struct ForcePositionOutCall {
        pub index: u32,
        pub position_id: u32,
        pub in_debt_array: bool,
    }
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
    #[doc = "Container type for all input parameters for the `ignorePause`function with signature `ignorePause()` and selector `[153, 89, 175, 148]`"]
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
    #[ethcall(name = "ignorePause", abi = "ignorePause()")]
    pub struct IgnorePauseCall;
    #[doc = "Container type for all input parameters for the `increaseShareSupplyCap`function with signature `increaseShareSupplyCap(uint192)` and selector `[176, 100, 110, 39]`"]
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
        name = "increaseShareSupplyCap",
        abi = "increaseShareSupplyCap(uint192)"
    )]
    pub struct IncreaseShareSupplyCapCall {
        pub new_share_supply_cap: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `positionCatalogue`function with signature `positionCatalogue(uint32)` and selector `[203, 223, 51, 208]`"]
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
    #[ethcall(name = "positionCatalogue", abi = "positionCatalogue(uint32)")]
    pub struct PositionCatalogueCall(pub u32);
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
    #[doc = "Container type for all input parameters for the `priceRouter`function with signature `priceRouter()` and selector `[215, 212, 191, 69]`"]
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
    #[ethcall(name = "priceRouter", abi = "priceRouter()")]
    pub struct PriceRouterCall;
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
    #[doc = "Container type for all input parameters for the `removeAdaptorFromCatalogue`function with signature `removeAdaptorFromCatalogue(address)` and selector `[95, 107, 136, 160]`"]
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
        name = "removeAdaptorFromCatalogue",
        abi = "removeAdaptorFromCatalogue(address)"
    )]
    pub struct RemoveAdaptorFromCatalogueCall {
        pub adaptor: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `removePositionFromCatalogue`function with signature `removePositionFromCatalogue(uint32)` and selector `[209, 232, 132, 4]`"]
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
        name = "removePositionFromCatalogue",
        abi = "removePositionFromCatalogue(uint32)"
    )]
    pub struct RemovePositionFromCatalogueCall {
        pub position_id: u32,
    }
    #[doc = "Container type for all input parameters for the `setAutomationActions`function with signature `setAutomationActions(uint256,address)` and selector `[200, 232, 25, 80]`"]
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
        name = "setAutomationActions",
        abi = "setAutomationActions(uint256,address)"
    )]
    pub struct SetAutomationActionsCall {
        pub registry_id: ethers::core::types::U256,
        pub expected_automation_actions: ethers::core::types::Address,
    }
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
    #[doc = "Container type for all input parameters for the `shareSupplyCap`function with signature `shareSupplyCap()` and selector `[212, 70, 187, 204]`"]
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
    #[ethcall(name = "shareSupplyCap", abi = "shareSupplyCap()")]
    pub struct ShareSupplyCapCall;
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
    #[doc = "Container type for all input parameters for the `toggleIgnorePause`function with signature `toggleIgnorePause()` and selector `[163, 115, 227, 255]`"]
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
    #[ethcall(name = "toggleIgnorePause", abi = "toggleIgnorePause()")]
    pub struct ToggleIgnorePauseCall;
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
    #[doc = "Container type for all input parameters for the `viewPositionBalances`function with signature `viewPositionBalances()` and selector `[120, 224, 35, 62]`"]
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
    #[ethcall(name = "viewPositionBalances", abi = "viewPositionBalances()")]
    pub struct ViewPositionBalancesCall;
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
    pub enum CellarWithShareLockPeriodV1Calls {
        DomainSeparator(DomainSeparatorCall),
        GravityBridgeRegistrySlot(GravityBridgeRegistrySlotCall),
        MaximumShareLockPeriod(MaximumShareLockPeriodCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        MinimumShareLockPeriod(MinimumShareLockPeriodCall),
        PriceRouterRegistrySlot(PriceRouterRegistrySlotCall),
        AdaptorCatalogue(AdaptorCatalogueCall),
        AddAdaptorToCatalogue(AddAdaptorToCatalogueCall),
        AddPosition(AddPositionCall),
        AddPositionToCatalogue(AddPositionToCatalogueCall),
        Allowance(AllowanceCall),
        AllowedRebalanceDeviation(AllowedRebalanceDeviationCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        AutomationActions(AutomationActionsCall),
        BalanceOf(BalanceOfCall),
        BlockExternalReceiver(BlockExternalReceiverCall),
        CachePriceRouter(CachePriceRouterCall),
        CallOnAdaptor(CallOnAdaptorCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CreditPositions(CreditPositionsCall),
        DebtPositions(DebtPositionsCall),
        Decimals(DecimalsCall),
        DecreaseShareSupplyCap(DecreaseShareSupplyCapCall),
        Deposit(DepositCall),
        FeeData(FeeDataCall),
        ForcePositionOut(ForcePositionOutCall),
        GetCreditPositions(GetCreditPositionsCall),
        GetDebtPositions(GetDebtPositionsCall),
        GetPositionData(GetPositionDataCall),
        HoldingPosition(HoldingPositionCall),
        IgnorePause(IgnorePauseCall),
        IncreaseShareSupplyCap(IncreaseShareSupplyCapCall),
        InitiateShutdown(InitiateShutdownCall),
        IsPaused(IsPausedCall),
        IsPositionUsed(IsPositionUsedCall),
        IsShutdown(IsShutdownCall),
        LiftShutdown(LiftShutdownCall),
        Locked(LockedCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        Nonces(NoncesCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        PositionCatalogue(PositionCatalogueCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PriceRouter(PriceRouterCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemoveAdaptorFromCatalogue(RemoveAdaptorFromCatalogueCall),
        RemovePosition(RemovePositionCall),
        RemovePositionFromCatalogue(RemovePositionFromCatalogueCall),
        SetAutomationActions(SetAutomationActionsCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetShareLockPeriod(SetShareLockPeriodCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        ShareLockPeriod(ShareLockPeriodCall),
        ShareSupplyCap(ShareSupplyCapCall),
        SwapPositions(SwapPositionsCall),
        Symbol(SymbolCall),
        ToggleIgnorePause(ToggleIgnorePauseCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithdrawable(TotalAssetsWithdrawableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        UserShareLockStartTime(UserShareLockStartTimeCall),
        ViewPositionBalances(ViewPositionBalancesCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for CellarWithShareLockPeriodV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <GravityBridgeRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::GravityBridgeRegistrySlot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MaximumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaximumShareLockPeriod(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MaxFeeCutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxFeeCut(decoded));
            }
            if let Ok(decoded) =
                <MaxPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <MaxPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxPositions(decoded));
            }
            if let Ok(decoded) =
                <MaxRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxRebalanceDeviation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MinimumShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MinimumShareLockPeriod(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PriceRouterRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PriceRouterRegistrySlot(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AdaptorCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AdaptorCatalogue(decoded));
            }
            if let Ok(decoded) =
                <AddAdaptorToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AddAdaptorToCatalogue(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AddPositionToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AddPositionToCatalogue(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AllowedRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AllowedRebalanceDeviation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Asset(decoded));
            }
            if let Ok(decoded) =
                <AutomationActionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::AutomationActions(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BlockExternalReceiverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::BlockExternalReceiver(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CachePriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::CachePriceRouter(decoded));
            }
            if let Ok(decoded) =
                <CallOnAdaptorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::CallOnAdaptor(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <CreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::CreditPositions(decoded));
            }
            if let Ok(decoded) =
                <DebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::DebtPositions(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::DecreaseShareSupplyCap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FeeDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::FeeData(decoded));
            }
            if let Ok(decoded) =
                <ForcePositionOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ForcePositionOut(decoded));
            }
            if let Ok(decoded) =
                <GetCreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::GetCreditPositions(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetDebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::GetDebtPositions(decoded));
            }
            if let Ok(decoded) =
                <GetPositionDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::GetPositionData(decoded));
            }
            if let Ok(decoded) =
                <HoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::HoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <IgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::IgnorePause(decoded));
            }
            if let Ok(decoded) =
                <IncreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::IncreaseShareSupplyCap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::InitiateShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::IsPaused(decoded));
            }
            if let Ok(decoded) =
                <IsPositionUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::IsPositionUsed(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Locked(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarWithShareLockPeriodV1Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Multicall(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarWithShareLockPeriodV1Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PositionCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PositionCatalogue(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::PriceRouter(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RemoveAdaptorFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::RemoveAdaptorFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::RemovePosition(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::RemovePositionFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <SetAutomationActionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetAutomationActions(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetHoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetHoldingPosition(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetRebalanceDeviation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetShareLockPeriod(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetStrategistPayoutAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPlatformCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SetStrategistPlatformCut(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ShareLockPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ShareLockPeriod(decoded));
            }
            if let Ok(decoded) =
                <ShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ShareSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <SwapPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <ToggleIgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ToggleIgnorePause(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::TotalAssetsWithdrawable(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UserShareLockStartTimeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::UserShareLockStartTime(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ViewPositionBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::ViewPositionBalances(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithShareLockPeriodV1Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CellarWithShareLockPeriodV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CellarWithShareLockPeriodV1Calls::DomainSeparator(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::GravityBridgeRegistrySlot(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::MaximumShareLockPeriod(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::MaxFeeCut(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxPlatformFee(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxRebalanceDeviation(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::MinimumShareLockPeriod(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::PriceRouterRegistrySlot(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::AdaptorCatalogue(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::AddAdaptorToCatalogue(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::AddPosition(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::AddPositionToCatalogue(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::Allowance(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::AllowedRebalanceDeviation(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::Approve(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Asset(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::AutomationActions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::BalanceOf(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::BlockExternalReceiver(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::CachePriceRouter(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::CallOnAdaptor(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::ConvertToAssets(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::ConvertToShares(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::CreditPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::DebtPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Decimals(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::DecreaseShareSupplyCap(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::Deposit(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::FeeData(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::ForcePositionOut(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::GetCreditPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::GetDebtPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::GetPositionData(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::HoldingPosition(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::IgnorePause(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::IncreaseShareSupplyCap(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::InitiateShutdown(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::IsPaused(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::IsPositionUsed(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::IsShutdown(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::LiftShutdown(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Locked(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxDeposit(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxMint(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxRedeem(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::MaxWithdraw(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Mint(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Multicall(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Name(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Nonces(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::OnERC721Received(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Owner(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Permit(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PositionCatalogue(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PreviewDeposit(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PreviewMint(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PreviewRedeem(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PreviewWithdraw(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::PriceRouter(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Redeem(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Registry(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::RemoveAdaptorFromCatalogue(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::RemovePosition(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::RemovePositionFromCatalogue(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::SetAutomationActions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::SetHoldingPosition(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::SetRebalanceDeviation(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::SetShareLockPeriod(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::SetStrategistPayoutAddress(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::SetStrategistPlatformCut(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::ShareLockPeriod(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::ShareSupplyCap(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::SwapPositions(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Symbol(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::ToggleIgnorePause(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::TotalAssets(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::TotalAssetsWithdrawable(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::TotalSupply(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Transfer(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::TransferFrom(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::TransferOwnership(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::UserShareLockStartTime(element) => {
                    element.encode()
                }
                CellarWithShareLockPeriodV1Calls::ViewPositionBalances(element) => element.encode(),
                CellarWithShareLockPeriodV1Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CellarWithShareLockPeriodV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarWithShareLockPeriodV1Calls::DomainSeparator(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::GravityBridgeRegistrySlot(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::MaximumShareLockPeriod(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxFeeCut(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxPlatformFee(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxRebalanceDeviation(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MinimumShareLockPeriod(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PriceRouterRegistrySlot(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::AdaptorCatalogue(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::AddAdaptorToCatalogue(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::AddPosition(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::AddPositionToCatalogue(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Allowance(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::AllowedRebalanceDeviation(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::Approve(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Asset(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::AutomationActions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::BalanceOf(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::BlockExternalReceiver(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::CachePriceRouter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::CallOnAdaptor(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ConvertToAssets(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ConvertToShares(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::CreditPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::DebtPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Decimals(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::DecreaseShareSupplyCap(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Deposit(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::FeeData(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ForcePositionOut(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::GetCreditPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::GetDebtPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::GetPositionData(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::HoldingPosition(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::IgnorePause(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::IncreaseShareSupplyCap(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::InitiateShutdown(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::IsPaused(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::IsPositionUsed(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::IsShutdown(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::LiftShutdown(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Locked(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxDeposit(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxMint(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxRedeem(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::MaxWithdraw(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Mint(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Multicall(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Name(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Nonces(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::OnERC721Received(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Owner(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Permit(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PositionCatalogue(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PreviewDeposit(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PreviewMint(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PreviewRedeem(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PreviewWithdraw(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::PriceRouter(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Redeem(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Registry(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::RemoveAdaptorFromCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::RemovePosition(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::RemovePositionFromCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::SetAutomationActions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::SetHoldingPosition(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::SetRebalanceDeviation(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::SetShareLockPeriod(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::SetStrategistPayoutAddress(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::SetStrategistPlatformCut(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::ShareLockPeriod(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ShareSupplyCap(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::SwapPositions(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Symbol(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ToggleIgnorePause(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::TotalAssets(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::TotalAssetsWithdrawable(element) => {
                    element.fmt(f)
                }
                CellarWithShareLockPeriodV1Calls::TotalSupply(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Transfer(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::TransferFrom(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::TransferOwnership(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::UserShareLockStartTime(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::ViewPositionBalances(element) => element.fmt(f),
                CellarWithShareLockPeriodV1Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            CellarWithShareLockPeriodV1Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<GravityBridgeRegistrySlotCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: GravityBridgeRegistrySlotCall) -> Self {
            CellarWithShareLockPeriodV1Calls::GravityBridgeRegistrySlot(var)
        }
    }
    impl ::std::convert::From<MaximumShareLockPeriodCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaximumShareLockPeriodCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaximumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<MaxFeeCutCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxFeeCutCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxFeeCut(var)
        }
    }
    impl ::std::convert::From<MaxPlatformFeeCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxPlatformFeeCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxPlatformFee(var)
        }
    }
    impl ::std::convert::From<MaxPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxPositions(var)
        }
    }
    impl ::std::convert::From<MaxRebalanceDeviationCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxRebalanceDeviationCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<MinimumShareLockPeriodCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MinimumShareLockPeriodCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MinimumShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<PriceRouterRegistrySlotCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PriceRouterRegistrySlotCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PriceRouterRegistrySlot(var)
        }
    }
    impl ::std::convert::From<AdaptorCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AdaptorCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AdaptorCatalogue(var)
        }
    }
    impl ::std::convert::From<AddAdaptorToCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AddAdaptorToCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AddAdaptorToCatalogue(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AddPositionCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AddPositionToCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AddPositionToCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AddPositionToCatalogue(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AllowanceCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<AllowedRebalanceDeviationCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AllowedRebalanceDeviationCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AllowedRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ApproveCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Approve(var)
        }
    }
    impl ::std::convert::From<AssetCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AssetCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Asset(var)
        }
    }
    impl ::std::convert::From<AutomationActionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: AutomationActionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::AutomationActions(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            CellarWithShareLockPeriodV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BlockExternalReceiverCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: BlockExternalReceiverCall) -> Self {
            CellarWithShareLockPeriodV1Calls::BlockExternalReceiver(var)
        }
    }
    impl ::std::convert::From<CachePriceRouterCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: CachePriceRouterCall) -> Self {
            CellarWithShareLockPeriodV1Calls::CachePriceRouter(var)
        }
    }
    impl ::std::convert::From<CallOnAdaptorCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: CallOnAdaptorCall) -> Self {
            CellarWithShareLockPeriodV1Calls::CallOnAdaptor(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ConvertToAssetsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ConvertToSharesCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<CreditPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: CreditPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::CreditPositions(var)
        }
    }
    impl ::std::convert::From<DebtPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: DebtPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::DebtPositions(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: DecimalsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseShareSupplyCapCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: DecreaseShareSupplyCapCall) -> Self {
            CellarWithShareLockPeriodV1Calls::DecreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: DepositCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<FeeDataCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: FeeDataCall) -> Self {
            CellarWithShareLockPeriodV1Calls::FeeData(var)
        }
    }
    impl ::std::convert::From<ForcePositionOutCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ForcePositionOutCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ForcePositionOut(var)
        }
    }
    impl ::std::convert::From<GetCreditPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: GetCreditPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::GetCreditPositions(var)
        }
    }
    impl ::std::convert::From<GetDebtPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: GetDebtPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::GetDebtPositions(var)
        }
    }
    impl ::std::convert::From<GetPositionDataCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: GetPositionDataCall) -> Self {
            CellarWithShareLockPeriodV1Calls::GetPositionData(var)
        }
    }
    impl ::std::convert::From<HoldingPositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: HoldingPositionCall) -> Self {
            CellarWithShareLockPeriodV1Calls::HoldingPosition(var)
        }
    }
    impl ::std::convert::From<IgnorePauseCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: IgnorePauseCall) -> Self {
            CellarWithShareLockPeriodV1Calls::IgnorePause(var)
        }
    }
    impl ::std::convert::From<IncreaseShareSupplyCapCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: IncreaseShareSupplyCapCall) -> Self {
            CellarWithShareLockPeriodV1Calls::IncreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: InitiateShutdownCall) -> Self {
            CellarWithShareLockPeriodV1Calls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsPausedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: IsPausedCall) -> Self {
            CellarWithShareLockPeriodV1Calls::IsPaused(var)
        }
    }
    impl ::std::convert::From<IsPositionUsedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: IsPositionUsedCall) -> Self {
            CellarWithShareLockPeriodV1Calls::IsPositionUsed(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: IsShutdownCall) -> Self {
            CellarWithShareLockPeriodV1Calls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: LiftShutdownCall) -> Self {
            CellarWithShareLockPeriodV1Calls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LockedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: LockedCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Locked(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxDepositCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxMintCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxRedeemCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MaxWithdrawCall) -> Self {
            CellarWithShareLockPeriodV1Calls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MintCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Mint(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: MulticallCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Multicall(var)
        }
    }
    impl ::std::convert::From<NameCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: NameCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: NoncesCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            CellarWithShareLockPeriodV1Calls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: OwnerCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PermitCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Permit(var)
        }
    }
    impl ::std::convert::From<PositionCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PositionCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PositionCatalogue(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PreviewDepositCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PreviewMintCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PreviewRedeemCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PreviewWithdrawCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<PriceRouterCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: PriceRouterCall) -> Self {
            CellarWithShareLockPeriodV1Calls::PriceRouter(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: RedeemCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Redeem(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: RegistryCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Registry(var)
        }
    }
    impl ::std::convert::From<RemoveAdaptorFromCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: RemoveAdaptorFromCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::RemoveAdaptorFromCatalogue(var)
        }
    }
    impl ::std::convert::From<RemovePositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: RemovePositionCall) -> Self {
            CellarWithShareLockPeriodV1Calls::RemovePosition(var)
        }
    }
    impl ::std::convert::From<RemovePositionFromCatalogueCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: RemovePositionFromCatalogueCall) -> Self {
            CellarWithShareLockPeriodV1Calls::RemovePositionFromCatalogue(var)
        }
    }
    impl ::std::convert::From<SetAutomationActionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetAutomationActionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetAutomationActions(var)
        }
    }
    impl ::std::convert::From<SetHoldingPositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetHoldingPositionCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetHoldingPosition(var)
        }
    }
    impl ::std::convert::From<SetRebalanceDeviationCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetRebalanceDeviationCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<SetShareLockPeriodCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetShareLockPeriodCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<SetStrategistPayoutAddressCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetStrategistPayoutAddressCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetStrategistPayoutAddress(var)
        }
    }
    impl ::std::convert::From<SetStrategistPlatformCutCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SetStrategistPlatformCutCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SetStrategistPlatformCut(var)
        }
    }
    impl ::std::convert::From<ShareLockPeriodCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ShareLockPeriodCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ShareLockPeriod(var)
        }
    }
    impl ::std::convert::From<ShareSupplyCapCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ShareSupplyCapCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<SwapPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SwapPositionsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::SwapPositions(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: SymbolCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<ToggleIgnorePauseCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ToggleIgnorePauseCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ToggleIgnorePause(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TotalAssetsCall) -> Self {
            CellarWithShareLockPeriodV1Calls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalAssetsWithdrawableCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TotalAssetsWithdrawableCall) -> Self {
            CellarWithShareLockPeriodV1Calls::TotalAssetsWithdrawable(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TotalSupplyCall) -> Self {
            CellarWithShareLockPeriodV1Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TransferCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TransferFromCall) -> Self {
            CellarWithShareLockPeriodV1Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CellarWithShareLockPeriodV1Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UserShareLockStartTimeCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: UserShareLockStartTimeCall) -> Self {
            CellarWithShareLockPeriodV1Calls::UserShareLockStartTime(var)
        }
    }
    impl ::std::convert::From<ViewPositionBalancesCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: ViewPositionBalancesCall) -> Self {
            CellarWithShareLockPeriodV1Calls::ViewPositionBalances(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CellarWithShareLockPeriodV1Calls {
        fn from(var: WithdrawCall) -> Self {
            CellarWithShareLockPeriodV1Calls::Withdraw(var)
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
