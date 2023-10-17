pub use cellarv2_5_mod::*;
#[allow(clippy::too_many_arguments)]
mod cellarv2_5_mod {
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
    #[doc = "CellarV2_5 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CELLARV2_5_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract Registry\",\n                \"name\": \"_registry\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"_asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"string\",\n                \"name\": \"_name\",\n                \"type\": \"string\"\n            },\n            {\n                \"internalType\": \"string\",\n                \"name\": \"_symbol\",\n                \"type\": \"string\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"_holdingPosition\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"_holdingPositionConfig\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_initialDeposit\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"_strategistPlatformCut\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"uint192\",\n                \"name\": \"_shareSupplyCap\",\n                \"type\": \"uint192\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_balancerVault\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"expectedAsset\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__AssetMismatch\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__CallToAdaptorNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__CallerNotApprovedToRebalance\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__CallerNotBalancerVault\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ContractNotShutdown\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ContractShutdown\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__DebtMismatch\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ExpectedAddressDoesNotMatchActual\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ExternalInitiator\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__FailedToForceOutPosition\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"illiquidPosition\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__IlliquidWithdraw\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assetsOwed\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__IncompleteWithdraw\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidFee\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidFeeCut\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__InvalidHoldingPosition\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"requested\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"max\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__InvalidRebalanceDeviation\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__InvalidShareSupplyCap\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__MinimumConstructorMintNotMet\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__OracleFailure\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__Paused\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__PositionAlreadyUsed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"maxPositions\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__PositionArrayFull\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"sharesRemaining\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__PositionNotEmpty\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__PositionNotInCatalogue\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"Cellar__PositionNotUsed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__RemovingHoldingPosition\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__SettingValueToRegistryIdZeroIsProhibited\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ShareSupplyCapExceeded\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"max\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__TotalAssetDeviatedOutsideRange\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"current\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"expected\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Cellar__TotalSharesMustRemainConstant\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ZeroAssets\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"Cellar__ZeroShares\",\n        \"type\": \"error\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bytes\",\n                \"name\": \"data\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"AdaptorCalled\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"inCatalogue\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"AdaptorCatalogueAltered\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Approval\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"newAutomationActions\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"Cellar__AutomationActionsUpdated\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"caller\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Deposit\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"user\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"OwnershipTransferred\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionAdded\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"inCatalogue\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"PositionCatalogueAltered\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"position\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionRemoved\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"newPosition1\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint32\",\n                \"name\": \"newPosition2\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index2\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"PositionSwapped\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"oldDeviation\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"newDeviation\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"RebalanceDeviationChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"newOracle\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"SharePriceOracleUpdated\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"bool\",\n                \"name\": \"isShutdown\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"ShutdownChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"oldPayoutAddress\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"address\",\n                \"name\": \"newPayoutAddress\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"StrategistPayoutAddressChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"oldPlatformCut\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint64\",\n                \"name\": \"newPlatformCut\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"StrategistPlatformCutChanged\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Transfer\",\n        \"type\": \"event\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"caller\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": true,\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"Withdraw\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"DOMAIN_SEPARATOR\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"GRAVITY_BRIDGE_REGISTRY_SLOT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_FEE_CUT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_PLATFORM_FEE\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_POSITIONS\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"MAX_REBALANCE_DEVIATION\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"ORACLE_DECIMALS\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"PRICE_ROUTER_REGISTRY_SLOT\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"adaptorCatalogue\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"addAdaptorToCatalogue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"addPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"addPositionToCatalogue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"allowance\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"allowedRebalanceDeviation\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"approve\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"asset\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"automationActions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"balancerVault\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"blockExternalReceiver\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"checkTotalAssets\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"uint16\",\n                \"name\": \"allowableRange\",\n                \"type\": \"uint16\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"expectedPriceRouter\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"cachePriceRouter\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"adaptor\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"bytes[]\",\n                        \"name\": \"callData\",\n                        \"type\": \"bytes[]\"\n                    }\n                ],\n                \"internalType\": \"struct Cellar.AdaptorCall[]\",\n                \"name\": \"data\",\n                \"type\": \"tuple[]\"\n            }\n        ],\n        \"name\": \"callOnAdaptor\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"convertToAssets\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"convertToShares\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"creditPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"debtPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"decimals\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"\",\n                \"type\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint192\",\n                \"name\": \"_newShareSupplyCap\",\n                \"type\": \"uint192\"\n            }\n        ],\n        \"name\": \"decreaseShareSupplyCap\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"feeData\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"strategistPlatformCut\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"platformFee\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"lastAccrual\",\n                \"type\": \"uint64\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"strategistPayoutAddress\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"forcePositionOut\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getCreditPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32[]\",\n                \"name\": \"\",\n                \"type\": \"uint32[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"getDebtPositions\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32[]\",\n                \"name\": \"\",\n                \"type\": \"uint32[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"getPositionData\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"isDebt\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"holdingPosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"ignorePause\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint192\",\n                \"name\": \"_newShareSupplyCap\",\n                \"type\": \"uint192\"\n            }\n        ],\n        \"name\": \"increaseShareSupplyCap\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"initiateShutdown\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isPaused\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"isPositionUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isShutdown\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"liftShutdown\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"locked\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxDeposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxMint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxRedeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"maxWithdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"mint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes[]\",\n                \"name\": \"data\",\n                \"type\": \"bytes[]\"\n            }\n        ],\n        \"name\": \"multicall\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"name\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"nonces\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"onERC721Received\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"owner\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"value\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"deadline\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint8\",\n                \"name\": \"v\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"r\",\n                \"type\": \"bytes32\"\n            },\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"s\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"name\": \"permit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"positionCatalogue\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewDeposit\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewMint\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewRedeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"previewWithdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"priceRouter\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract PriceRouter\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract IERC20[]\",\n                \"name\": \"tokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"amounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"feeAmounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"userData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"receiveFlashLoan\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"redeem\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"registry\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract Registry\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"adaptor\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"removeAdaptorFromCatalogue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"removePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"removePositionFromCatalogue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_registryId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_expectedAutomationActions\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setAutomationActions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"setHoldingPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"newDeviation\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"setRebalanceDeviation\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_registryId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract ERC4626SharePriceOracle\",\n                \"name\": \"_sharePriceOracle\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setSharePriceOracle\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"payout\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"setStrategistPayoutAddress\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"cut\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"setStrategistPlatformCut\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"sharePriceOracle\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC4626SharePriceOracle\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"shareSupplyCap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint192\",\n                \"name\": \"\",\n                \"type\": \"uint192\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index1\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"index2\",\n                \"type\": \"uint32\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"inDebtArray\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"swapPositions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"symbol\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string\",\n                \"name\": \"\",\n                \"type\": \"string\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"toggleIgnorePause\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalAssets\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalAssetsWithdrawable\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"totalSupply\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transfer\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"from\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"to\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"transferFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"newOwner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"transferOwnership\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"viewPositionBalances\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"balances\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bool[]\",\n                \"name\": \"isDebt\",\n                \"type\": \"bool[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"owner\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"shares\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CellarV2_5<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CellarV2_5<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CellarV2_5<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CellarV2_5))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CellarV2_5<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CELLARV2_5_ABI.clone(), client);
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
        #[doc = "Calls the contract's `ORACLE_DECIMALS` (0x1ea15502) function"]
        pub fn oracle_decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([30, 161, 85, 2], ())
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
        #[doc = "Calls the contract's `balancerVault` (0x158274a5) function"]
        pub fn balancer_vault(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([21, 130, 116, 165], ())
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
        #[doc = "Calls the contract's `receiveFlashLoan` (0xf04f2707) function"]
        pub fn receive_flash_loan(
            &self,
            tokens: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
            user_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 79, 39, 7], (tokens, amounts, fee_amounts, user_data))
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
        #[doc = "Calls the contract's `setSharePriceOracle` (0xf5743bc9) function"]
        pub fn set_share_price_oracle(
            &self,
            registry_id: ethers::core::types::U256,
            share_price_oracle: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 116, 59, 201], (registry_id, share_price_oracle))
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
        #[doc = "Calls the contract's `sharePriceOracle` (0x196e8285) function"]
        pub fn share_price_oracle(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([25, 110, 130, 133], ())
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
        #[doc = "Gets the contract's `SharePriceOracleUpdated` event"]
        pub fn share_price_oracle_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SharePriceOracleUpdatedFilter> {
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, CellarV2_5Events> {
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
        name = "SharePriceOracleUpdated",
        abi = "SharePriceOracleUpdated(address)"
    )]
    pub struct SharePriceOracleUpdatedFilter {
        pub new_oracle: ethers::core::types::Address,
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
    pub enum CellarV2_5Events {
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
        SharePriceOracleUpdatedFilter(SharePriceOracleUpdatedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for CellarV2_5Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdaptorCalledFilter::decode_log(log) {
                return Ok(CellarV2_5Events::AdaptorCalledFilter(decoded));
            }
            if let Ok(decoded) = AdaptorCatalogueAlteredFilter::decode_log(log) {
                return Ok(CellarV2_5Events::AdaptorCatalogueAlteredFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarV2_5Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = CellarAutomationActionsUpdatedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::CellarAutomationActionsUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarV2_5Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CellarV2_5Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::PositionAddedFilter(decoded));
            }
            if let Ok(decoded) = PositionCatalogueAlteredFilter::decode_log(log) {
                return Ok(CellarV2_5Events::PositionCatalogueAlteredFilter(decoded));
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::PositionRemovedFilter(decoded));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::PositionSwappedFilter(decoded));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::RebalanceDeviationChangedFilter(decoded));
            }
            if let Ok(decoded) = SharePriceOracleUpdatedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::SharePriceOracleUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::ShutdownChangedFilter(decoded));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::StrategistPayoutAddressChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(CellarV2_5Events::StrategistPlatformCutChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarV2_5Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarV2_5Events::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CellarV2_5Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV2_5Events::AdaptorCalledFilter(element) => element.fmt(f),
                CellarV2_5Events::AdaptorCatalogueAlteredFilter(element) => element.fmt(f),
                CellarV2_5Events::ApprovalFilter(element) => element.fmt(f),
                CellarV2_5Events::CellarAutomationActionsUpdatedFilter(element) => element.fmt(f),
                CellarV2_5Events::DepositFilter(element) => element.fmt(f),
                CellarV2_5Events::OwnershipTransferredFilter(element) => element.fmt(f),
                CellarV2_5Events::PositionAddedFilter(element) => element.fmt(f),
                CellarV2_5Events::PositionCatalogueAlteredFilter(element) => element.fmt(f),
                CellarV2_5Events::PositionRemovedFilter(element) => element.fmt(f),
                CellarV2_5Events::PositionSwappedFilter(element) => element.fmt(f),
                CellarV2_5Events::RebalanceDeviationChangedFilter(element) => element.fmt(f),
                CellarV2_5Events::SharePriceOracleUpdatedFilter(element) => element.fmt(f),
                CellarV2_5Events::ShutdownChangedFilter(element) => element.fmt(f),
                CellarV2_5Events::StrategistPayoutAddressChangedFilter(element) => element.fmt(f),
                CellarV2_5Events::StrategistPlatformCutChangedFilter(element) => element.fmt(f),
                CellarV2_5Events::TransferFilter(element) => element.fmt(f),
                CellarV2_5Events::WithdrawFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `ORACLE_DECIMALS`function with signature `ORACLE_DECIMALS()` and selector `[30, 161, 85, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "ORACLE_DECIMALS", abi = "ORACLE_DECIMALS()")]
    pub struct OracleDecimalsCall;
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
    #[doc = "Container type for all input parameters for the `balancerVault`function with signature `balancerVault()` and selector `[21, 130, 116, 165]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "balancerVault", abi = "balancerVault()")]
    pub struct BalancerVaultCall;
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
    #[doc = "Container type for all input parameters for the `receiveFlashLoan`function with signature `receiveFlashLoan(address[],uint256[],uint256[],bytes)` and selector `[240, 79, 39, 7]`"]
    #[derive(
        Clone,
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
        name = "receiveFlashLoan",
        abi = "receiveFlashLoan(address[],uint256[],uint256[],bytes)"
    )]
    pub struct ReceiveFlashLoanCall {
        pub tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub fee_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub user_data: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `setSharePriceOracle`function with signature `setSharePriceOracle(uint256,address)` and selector `[245, 116, 59, 201]`"]
    #[derive(
        Clone,
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
        name = "setSharePriceOracle",
        abi = "setSharePriceOracle(uint256,address)"
    )]
    pub struct SetSharePriceOracleCall {
        pub registry_id: ethers::core::types::U256,
        pub share_price_oracle: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `sharePriceOracle`function with signature `sharePriceOracle()` and selector `[25, 110, 130, 133]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "sharePriceOracle", abi = "sharePriceOracle()")]
    pub struct SharePriceOracleCall;
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
    pub enum CellarV2_5Calls {
        DomainSeparator(DomainSeparatorCall),
        GravityBridgeRegistrySlot(GravityBridgeRegistrySlotCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        OracleDecimals(OracleDecimalsCall),
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
        BalancerVault(BalancerVaultCall),
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
        ReceiveFlashLoan(ReceiveFlashLoanCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemoveAdaptorFromCatalogue(RemoveAdaptorFromCatalogueCall),
        RemovePosition(RemovePositionCall),
        RemovePositionFromCatalogue(RemovePositionFromCatalogueCall),
        SetAutomationActions(SetAutomationActionsCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetSharePriceOracle(SetSharePriceOracleCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        SharePriceOracle(SharePriceOracleCall),
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
        ViewPositionBalances(ViewPositionBalancesCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for CellarV2_5Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <GravityBridgeRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::GravityBridgeRegistrySlot(decoded));
            }
            if let Ok(decoded) =
                <MaxFeeCutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxFeeCut(decoded));
            }
            if let Ok(decoded) =
                <MaxPlatformFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) =
                <MaxPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxPositions(decoded));
            }
            if let Ok(decoded) =
                <MaxRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <OracleDecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::OracleDecimals(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterRegistrySlotCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PriceRouterRegistrySlot(decoded));
            }
            if let Ok(decoded) =
                <AdaptorCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::AdaptorCatalogue(decoded));
            }
            if let Ok(decoded) =
                <AddAdaptorToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::AddAdaptorToCatalogue(decoded));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AddPositionToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::AddPositionToCatalogue(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AllowedRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::AllowedRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Asset(decoded));
            }
            if let Ok(decoded) =
                <AutomationActionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::AutomationActions(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalancerVaultCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::BalancerVault(decoded));
            }
            if let Ok(decoded) =
                <BlockExternalReceiverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::BlockExternalReceiver(decoded));
            }
            if let Ok(decoded) =
                <CachePriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::CachePriceRouter(decoded));
            }
            if let Ok(decoded) =
                <CallOnAdaptorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::CallOnAdaptor(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <CreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::CreditPositions(decoded));
            }
            if let Ok(decoded) =
                <DebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::DebtPositions(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::DecreaseShareSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FeeDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::FeeData(decoded));
            }
            if let Ok(decoded) =
                <ForcePositionOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ForcePositionOut(decoded));
            }
            if let Ok(decoded) =
                <GetCreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::GetCreditPositions(decoded));
            }
            if let Ok(decoded) =
                <GetDebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::GetDebtPositions(decoded));
            }
            if let Ok(decoded) =
                <GetPositionDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::GetPositionData(decoded));
            }
            if let Ok(decoded) =
                <HoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::HoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <IgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::IgnorePause(decoded));
            }
            if let Ok(decoded) =
                <IncreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::IncreaseShareSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::InitiateShutdown(decoded));
            }
            if let Ok(decoded) =
                <IsPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::IsPaused(decoded));
            }
            if let Ok(decoded) =
                <IsPositionUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::IsPositionUsed(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Locked(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV2_5Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Multicall(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarV2_5Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PositionCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PositionCatalogue(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::PriceRouter(decoded));
            }
            if let Ok(decoded) =
                <ReceiveFlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ReceiveFlashLoan(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RemoveAdaptorFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::RemoveAdaptorFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::RemovePosition(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::RemovePositionFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <SetAutomationActionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SetAutomationActions(decoded));
            }
            if let Ok(decoded) =
                <SetHoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SetHoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <SetRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SetRebalanceDeviation(decoded));
            }
            if let Ok(decoded) =
                <SetSharePriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SetSharePriceOracle(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPayoutAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPlatformCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarV2_5Calls::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) =
                <SharePriceOracleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SharePriceOracle(decoded));
            }
            if let Ok(decoded) =
                <ShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ShareSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <SwapPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <ToggleIgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ToggleIgnorePause(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::TotalAssetsWithdrawable(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ViewPositionBalancesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::ViewPositionBalances(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarV2_5Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CellarV2_5Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CellarV2_5Calls::DomainSeparator(element) => element.encode(),
                CellarV2_5Calls::GravityBridgeRegistrySlot(element) => element.encode(),
                CellarV2_5Calls::MaxFeeCut(element) => element.encode(),
                CellarV2_5Calls::MaxPlatformFee(element) => element.encode(),
                CellarV2_5Calls::MaxPositions(element) => element.encode(),
                CellarV2_5Calls::MaxRebalanceDeviation(element) => element.encode(),
                CellarV2_5Calls::OracleDecimals(element) => element.encode(),
                CellarV2_5Calls::PriceRouterRegistrySlot(element) => element.encode(),
                CellarV2_5Calls::AdaptorCatalogue(element) => element.encode(),
                CellarV2_5Calls::AddAdaptorToCatalogue(element) => element.encode(),
                CellarV2_5Calls::AddPosition(element) => element.encode(),
                CellarV2_5Calls::AddPositionToCatalogue(element) => element.encode(),
                CellarV2_5Calls::Allowance(element) => element.encode(),
                CellarV2_5Calls::AllowedRebalanceDeviation(element) => element.encode(),
                CellarV2_5Calls::Approve(element) => element.encode(),
                CellarV2_5Calls::Asset(element) => element.encode(),
                CellarV2_5Calls::AutomationActions(element) => element.encode(),
                CellarV2_5Calls::BalanceOf(element) => element.encode(),
                CellarV2_5Calls::BalancerVault(element) => element.encode(),
                CellarV2_5Calls::BlockExternalReceiver(element) => element.encode(),
                CellarV2_5Calls::CachePriceRouter(element) => element.encode(),
                CellarV2_5Calls::CallOnAdaptor(element) => element.encode(),
                CellarV2_5Calls::ConvertToAssets(element) => element.encode(),
                CellarV2_5Calls::ConvertToShares(element) => element.encode(),
                CellarV2_5Calls::CreditPositions(element) => element.encode(),
                CellarV2_5Calls::DebtPositions(element) => element.encode(),
                CellarV2_5Calls::Decimals(element) => element.encode(),
                CellarV2_5Calls::DecreaseShareSupplyCap(element) => element.encode(),
                CellarV2_5Calls::Deposit(element) => element.encode(),
                CellarV2_5Calls::FeeData(element) => element.encode(),
                CellarV2_5Calls::ForcePositionOut(element) => element.encode(),
                CellarV2_5Calls::GetCreditPositions(element) => element.encode(),
                CellarV2_5Calls::GetDebtPositions(element) => element.encode(),
                CellarV2_5Calls::GetPositionData(element) => element.encode(),
                CellarV2_5Calls::HoldingPosition(element) => element.encode(),
                CellarV2_5Calls::IgnorePause(element) => element.encode(),
                CellarV2_5Calls::IncreaseShareSupplyCap(element) => element.encode(),
                CellarV2_5Calls::InitiateShutdown(element) => element.encode(),
                CellarV2_5Calls::IsPaused(element) => element.encode(),
                CellarV2_5Calls::IsPositionUsed(element) => element.encode(),
                CellarV2_5Calls::IsShutdown(element) => element.encode(),
                CellarV2_5Calls::LiftShutdown(element) => element.encode(),
                CellarV2_5Calls::Locked(element) => element.encode(),
                CellarV2_5Calls::MaxDeposit(element) => element.encode(),
                CellarV2_5Calls::MaxMint(element) => element.encode(),
                CellarV2_5Calls::MaxRedeem(element) => element.encode(),
                CellarV2_5Calls::MaxWithdraw(element) => element.encode(),
                CellarV2_5Calls::Mint(element) => element.encode(),
                CellarV2_5Calls::Multicall(element) => element.encode(),
                CellarV2_5Calls::Name(element) => element.encode(),
                CellarV2_5Calls::Nonces(element) => element.encode(),
                CellarV2_5Calls::OnERC721Received(element) => element.encode(),
                CellarV2_5Calls::Owner(element) => element.encode(),
                CellarV2_5Calls::Permit(element) => element.encode(),
                CellarV2_5Calls::PositionCatalogue(element) => element.encode(),
                CellarV2_5Calls::PreviewDeposit(element) => element.encode(),
                CellarV2_5Calls::PreviewMint(element) => element.encode(),
                CellarV2_5Calls::PreviewRedeem(element) => element.encode(),
                CellarV2_5Calls::PreviewWithdraw(element) => element.encode(),
                CellarV2_5Calls::PriceRouter(element) => element.encode(),
                CellarV2_5Calls::ReceiveFlashLoan(element) => element.encode(),
                CellarV2_5Calls::Redeem(element) => element.encode(),
                CellarV2_5Calls::Registry(element) => element.encode(),
                CellarV2_5Calls::RemoveAdaptorFromCatalogue(element) => element.encode(),
                CellarV2_5Calls::RemovePosition(element) => element.encode(),
                CellarV2_5Calls::RemovePositionFromCatalogue(element) => element.encode(),
                CellarV2_5Calls::SetAutomationActions(element) => element.encode(),
                CellarV2_5Calls::SetHoldingPosition(element) => element.encode(),
                CellarV2_5Calls::SetRebalanceDeviation(element) => element.encode(),
                CellarV2_5Calls::SetSharePriceOracle(element) => element.encode(),
                CellarV2_5Calls::SetStrategistPayoutAddress(element) => element.encode(),
                CellarV2_5Calls::SetStrategistPlatformCut(element) => element.encode(),
                CellarV2_5Calls::SharePriceOracle(element) => element.encode(),
                CellarV2_5Calls::ShareSupplyCap(element) => element.encode(),
                CellarV2_5Calls::SwapPositions(element) => element.encode(),
                CellarV2_5Calls::Symbol(element) => element.encode(),
                CellarV2_5Calls::ToggleIgnorePause(element) => element.encode(),
                CellarV2_5Calls::TotalAssets(element) => element.encode(),
                CellarV2_5Calls::TotalAssetsWithdrawable(element) => element.encode(),
                CellarV2_5Calls::TotalSupply(element) => element.encode(),
                CellarV2_5Calls::Transfer(element) => element.encode(),
                CellarV2_5Calls::TransferFrom(element) => element.encode(),
                CellarV2_5Calls::TransferOwnership(element) => element.encode(),
                CellarV2_5Calls::ViewPositionBalances(element) => element.encode(),
                CellarV2_5Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CellarV2_5Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarV2_5Calls::DomainSeparator(element) => element.fmt(f),
                CellarV2_5Calls::GravityBridgeRegistrySlot(element) => element.fmt(f),
                CellarV2_5Calls::MaxFeeCut(element) => element.fmt(f),
                CellarV2_5Calls::MaxPlatformFee(element) => element.fmt(f),
                CellarV2_5Calls::MaxPositions(element) => element.fmt(f),
                CellarV2_5Calls::MaxRebalanceDeviation(element) => element.fmt(f),
                CellarV2_5Calls::OracleDecimals(element) => element.fmt(f),
                CellarV2_5Calls::PriceRouterRegistrySlot(element) => element.fmt(f),
                CellarV2_5Calls::AdaptorCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::AddAdaptorToCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::AddPosition(element) => element.fmt(f),
                CellarV2_5Calls::AddPositionToCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::Allowance(element) => element.fmt(f),
                CellarV2_5Calls::AllowedRebalanceDeviation(element) => element.fmt(f),
                CellarV2_5Calls::Approve(element) => element.fmt(f),
                CellarV2_5Calls::Asset(element) => element.fmt(f),
                CellarV2_5Calls::AutomationActions(element) => element.fmt(f),
                CellarV2_5Calls::BalanceOf(element) => element.fmt(f),
                CellarV2_5Calls::BalancerVault(element) => element.fmt(f),
                CellarV2_5Calls::BlockExternalReceiver(element) => element.fmt(f),
                CellarV2_5Calls::CachePriceRouter(element) => element.fmt(f),
                CellarV2_5Calls::CallOnAdaptor(element) => element.fmt(f),
                CellarV2_5Calls::ConvertToAssets(element) => element.fmt(f),
                CellarV2_5Calls::ConvertToShares(element) => element.fmt(f),
                CellarV2_5Calls::CreditPositions(element) => element.fmt(f),
                CellarV2_5Calls::DebtPositions(element) => element.fmt(f),
                CellarV2_5Calls::Decimals(element) => element.fmt(f),
                CellarV2_5Calls::DecreaseShareSupplyCap(element) => element.fmt(f),
                CellarV2_5Calls::Deposit(element) => element.fmt(f),
                CellarV2_5Calls::FeeData(element) => element.fmt(f),
                CellarV2_5Calls::ForcePositionOut(element) => element.fmt(f),
                CellarV2_5Calls::GetCreditPositions(element) => element.fmt(f),
                CellarV2_5Calls::GetDebtPositions(element) => element.fmt(f),
                CellarV2_5Calls::GetPositionData(element) => element.fmt(f),
                CellarV2_5Calls::HoldingPosition(element) => element.fmt(f),
                CellarV2_5Calls::IgnorePause(element) => element.fmt(f),
                CellarV2_5Calls::IncreaseShareSupplyCap(element) => element.fmt(f),
                CellarV2_5Calls::InitiateShutdown(element) => element.fmt(f),
                CellarV2_5Calls::IsPaused(element) => element.fmt(f),
                CellarV2_5Calls::IsPositionUsed(element) => element.fmt(f),
                CellarV2_5Calls::IsShutdown(element) => element.fmt(f),
                CellarV2_5Calls::LiftShutdown(element) => element.fmt(f),
                CellarV2_5Calls::Locked(element) => element.fmt(f),
                CellarV2_5Calls::MaxDeposit(element) => element.fmt(f),
                CellarV2_5Calls::MaxMint(element) => element.fmt(f),
                CellarV2_5Calls::MaxRedeem(element) => element.fmt(f),
                CellarV2_5Calls::MaxWithdraw(element) => element.fmt(f),
                CellarV2_5Calls::Mint(element) => element.fmt(f),
                CellarV2_5Calls::Multicall(element) => element.fmt(f),
                CellarV2_5Calls::Name(element) => element.fmt(f),
                CellarV2_5Calls::Nonces(element) => element.fmt(f),
                CellarV2_5Calls::OnERC721Received(element) => element.fmt(f),
                CellarV2_5Calls::Owner(element) => element.fmt(f),
                CellarV2_5Calls::Permit(element) => element.fmt(f),
                CellarV2_5Calls::PositionCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::PreviewDeposit(element) => element.fmt(f),
                CellarV2_5Calls::PreviewMint(element) => element.fmt(f),
                CellarV2_5Calls::PreviewRedeem(element) => element.fmt(f),
                CellarV2_5Calls::PreviewWithdraw(element) => element.fmt(f),
                CellarV2_5Calls::PriceRouter(element) => element.fmt(f),
                CellarV2_5Calls::ReceiveFlashLoan(element) => element.fmt(f),
                CellarV2_5Calls::Redeem(element) => element.fmt(f),
                CellarV2_5Calls::Registry(element) => element.fmt(f),
                CellarV2_5Calls::RemoveAdaptorFromCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::RemovePosition(element) => element.fmt(f),
                CellarV2_5Calls::RemovePositionFromCatalogue(element) => element.fmt(f),
                CellarV2_5Calls::SetAutomationActions(element) => element.fmt(f),
                CellarV2_5Calls::SetHoldingPosition(element) => element.fmt(f),
                CellarV2_5Calls::SetRebalanceDeviation(element) => element.fmt(f),
                CellarV2_5Calls::SetSharePriceOracle(element) => element.fmt(f),
                CellarV2_5Calls::SetStrategistPayoutAddress(element) => element.fmt(f),
                CellarV2_5Calls::SetStrategistPlatformCut(element) => element.fmt(f),
                CellarV2_5Calls::SharePriceOracle(element) => element.fmt(f),
                CellarV2_5Calls::ShareSupplyCap(element) => element.fmt(f),
                CellarV2_5Calls::SwapPositions(element) => element.fmt(f),
                CellarV2_5Calls::Symbol(element) => element.fmt(f),
                CellarV2_5Calls::ToggleIgnorePause(element) => element.fmt(f),
                CellarV2_5Calls::TotalAssets(element) => element.fmt(f),
                CellarV2_5Calls::TotalAssetsWithdrawable(element) => element.fmt(f),
                CellarV2_5Calls::TotalSupply(element) => element.fmt(f),
                CellarV2_5Calls::Transfer(element) => element.fmt(f),
                CellarV2_5Calls::TransferFrom(element) => element.fmt(f),
                CellarV2_5Calls::TransferOwnership(element) => element.fmt(f),
                CellarV2_5Calls::ViewPositionBalances(element) => element.fmt(f),
                CellarV2_5Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for CellarV2_5Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            CellarV2_5Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<GravityBridgeRegistrySlotCall> for CellarV2_5Calls {
        fn from(var: GravityBridgeRegistrySlotCall) -> Self {
            CellarV2_5Calls::GravityBridgeRegistrySlot(var)
        }
    }
    impl ::std::convert::From<MaxFeeCutCall> for CellarV2_5Calls {
        fn from(var: MaxFeeCutCall) -> Self {
            CellarV2_5Calls::MaxFeeCut(var)
        }
    }
    impl ::std::convert::From<MaxPlatformFeeCall> for CellarV2_5Calls {
        fn from(var: MaxPlatformFeeCall) -> Self {
            CellarV2_5Calls::MaxPlatformFee(var)
        }
    }
    impl ::std::convert::From<MaxPositionsCall> for CellarV2_5Calls {
        fn from(var: MaxPositionsCall) -> Self {
            CellarV2_5Calls::MaxPositions(var)
        }
    }
    impl ::std::convert::From<MaxRebalanceDeviationCall> for CellarV2_5Calls {
        fn from(var: MaxRebalanceDeviationCall) -> Self {
            CellarV2_5Calls::MaxRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<OracleDecimalsCall> for CellarV2_5Calls {
        fn from(var: OracleDecimalsCall) -> Self {
            CellarV2_5Calls::OracleDecimals(var)
        }
    }
    impl ::std::convert::From<PriceRouterRegistrySlotCall> for CellarV2_5Calls {
        fn from(var: PriceRouterRegistrySlotCall) -> Self {
            CellarV2_5Calls::PriceRouterRegistrySlot(var)
        }
    }
    impl ::std::convert::From<AdaptorCatalogueCall> for CellarV2_5Calls {
        fn from(var: AdaptorCatalogueCall) -> Self {
            CellarV2_5Calls::AdaptorCatalogue(var)
        }
    }
    impl ::std::convert::From<AddAdaptorToCatalogueCall> for CellarV2_5Calls {
        fn from(var: AddAdaptorToCatalogueCall) -> Self {
            CellarV2_5Calls::AddAdaptorToCatalogue(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for CellarV2_5Calls {
        fn from(var: AddPositionCall) -> Self {
            CellarV2_5Calls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AddPositionToCatalogueCall> for CellarV2_5Calls {
        fn from(var: AddPositionToCatalogueCall) -> Self {
            CellarV2_5Calls::AddPositionToCatalogue(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CellarV2_5Calls {
        fn from(var: AllowanceCall) -> Self {
            CellarV2_5Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<AllowedRebalanceDeviationCall> for CellarV2_5Calls {
        fn from(var: AllowedRebalanceDeviationCall) -> Self {
            CellarV2_5Calls::AllowedRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CellarV2_5Calls {
        fn from(var: ApproveCall) -> Self {
            CellarV2_5Calls::Approve(var)
        }
    }
    impl ::std::convert::From<AssetCall> for CellarV2_5Calls {
        fn from(var: AssetCall) -> Self {
            CellarV2_5Calls::Asset(var)
        }
    }
    impl ::std::convert::From<AutomationActionsCall> for CellarV2_5Calls {
        fn from(var: AutomationActionsCall) -> Self {
            CellarV2_5Calls::AutomationActions(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CellarV2_5Calls {
        fn from(var: BalanceOfCall) -> Self {
            CellarV2_5Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BalancerVaultCall> for CellarV2_5Calls {
        fn from(var: BalancerVaultCall) -> Self {
            CellarV2_5Calls::BalancerVault(var)
        }
    }
    impl ::std::convert::From<BlockExternalReceiverCall> for CellarV2_5Calls {
        fn from(var: BlockExternalReceiverCall) -> Self {
            CellarV2_5Calls::BlockExternalReceiver(var)
        }
    }
    impl ::std::convert::From<CachePriceRouterCall> for CellarV2_5Calls {
        fn from(var: CachePriceRouterCall) -> Self {
            CellarV2_5Calls::CachePriceRouter(var)
        }
    }
    impl ::std::convert::From<CallOnAdaptorCall> for CellarV2_5Calls {
        fn from(var: CallOnAdaptorCall) -> Self {
            CellarV2_5Calls::CallOnAdaptor(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for CellarV2_5Calls {
        fn from(var: ConvertToAssetsCall) -> Self {
            CellarV2_5Calls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for CellarV2_5Calls {
        fn from(var: ConvertToSharesCall) -> Self {
            CellarV2_5Calls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<CreditPositionsCall> for CellarV2_5Calls {
        fn from(var: CreditPositionsCall) -> Self {
            CellarV2_5Calls::CreditPositions(var)
        }
    }
    impl ::std::convert::From<DebtPositionsCall> for CellarV2_5Calls {
        fn from(var: DebtPositionsCall) -> Self {
            CellarV2_5Calls::DebtPositions(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CellarV2_5Calls {
        fn from(var: DecimalsCall) -> Self {
            CellarV2_5Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseShareSupplyCapCall> for CellarV2_5Calls {
        fn from(var: DecreaseShareSupplyCapCall) -> Self {
            CellarV2_5Calls::DecreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CellarV2_5Calls {
        fn from(var: DepositCall) -> Self {
            CellarV2_5Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<FeeDataCall> for CellarV2_5Calls {
        fn from(var: FeeDataCall) -> Self {
            CellarV2_5Calls::FeeData(var)
        }
    }
    impl ::std::convert::From<ForcePositionOutCall> for CellarV2_5Calls {
        fn from(var: ForcePositionOutCall) -> Self {
            CellarV2_5Calls::ForcePositionOut(var)
        }
    }
    impl ::std::convert::From<GetCreditPositionsCall> for CellarV2_5Calls {
        fn from(var: GetCreditPositionsCall) -> Self {
            CellarV2_5Calls::GetCreditPositions(var)
        }
    }
    impl ::std::convert::From<GetDebtPositionsCall> for CellarV2_5Calls {
        fn from(var: GetDebtPositionsCall) -> Self {
            CellarV2_5Calls::GetDebtPositions(var)
        }
    }
    impl ::std::convert::From<GetPositionDataCall> for CellarV2_5Calls {
        fn from(var: GetPositionDataCall) -> Self {
            CellarV2_5Calls::GetPositionData(var)
        }
    }
    impl ::std::convert::From<HoldingPositionCall> for CellarV2_5Calls {
        fn from(var: HoldingPositionCall) -> Self {
            CellarV2_5Calls::HoldingPosition(var)
        }
    }
    impl ::std::convert::From<IgnorePauseCall> for CellarV2_5Calls {
        fn from(var: IgnorePauseCall) -> Self {
            CellarV2_5Calls::IgnorePause(var)
        }
    }
    impl ::std::convert::From<IncreaseShareSupplyCapCall> for CellarV2_5Calls {
        fn from(var: IncreaseShareSupplyCapCall) -> Self {
            CellarV2_5Calls::IncreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for CellarV2_5Calls {
        fn from(var: InitiateShutdownCall) -> Self {
            CellarV2_5Calls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsPausedCall> for CellarV2_5Calls {
        fn from(var: IsPausedCall) -> Self {
            CellarV2_5Calls::IsPaused(var)
        }
    }
    impl ::std::convert::From<IsPositionUsedCall> for CellarV2_5Calls {
        fn from(var: IsPositionUsedCall) -> Self {
            CellarV2_5Calls::IsPositionUsed(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for CellarV2_5Calls {
        fn from(var: IsShutdownCall) -> Self {
            CellarV2_5Calls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for CellarV2_5Calls {
        fn from(var: LiftShutdownCall) -> Self {
            CellarV2_5Calls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LockedCall> for CellarV2_5Calls {
        fn from(var: LockedCall) -> Self {
            CellarV2_5Calls::Locked(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for CellarV2_5Calls {
        fn from(var: MaxDepositCall) -> Self {
            CellarV2_5Calls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for CellarV2_5Calls {
        fn from(var: MaxMintCall) -> Self {
            CellarV2_5Calls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for CellarV2_5Calls {
        fn from(var: MaxRedeemCall) -> Self {
            CellarV2_5Calls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for CellarV2_5Calls {
        fn from(var: MaxWithdrawCall) -> Self {
            CellarV2_5Calls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for CellarV2_5Calls {
        fn from(var: MintCall) -> Self {
            CellarV2_5Calls::Mint(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for CellarV2_5Calls {
        fn from(var: MulticallCall) -> Self {
            CellarV2_5Calls::Multicall(var)
        }
    }
    impl ::std::convert::From<NameCall> for CellarV2_5Calls {
        fn from(var: NameCall) -> Self {
            CellarV2_5Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CellarV2_5Calls {
        fn from(var: NoncesCall) -> Self {
            CellarV2_5Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for CellarV2_5Calls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            CellarV2_5Calls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CellarV2_5Calls {
        fn from(var: OwnerCall) -> Self {
            CellarV2_5Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for CellarV2_5Calls {
        fn from(var: PermitCall) -> Self {
            CellarV2_5Calls::Permit(var)
        }
    }
    impl ::std::convert::From<PositionCatalogueCall> for CellarV2_5Calls {
        fn from(var: PositionCatalogueCall) -> Self {
            CellarV2_5Calls::PositionCatalogue(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for CellarV2_5Calls {
        fn from(var: PreviewDepositCall) -> Self {
            CellarV2_5Calls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for CellarV2_5Calls {
        fn from(var: PreviewMintCall) -> Self {
            CellarV2_5Calls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for CellarV2_5Calls {
        fn from(var: PreviewRedeemCall) -> Self {
            CellarV2_5Calls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for CellarV2_5Calls {
        fn from(var: PreviewWithdrawCall) -> Self {
            CellarV2_5Calls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<PriceRouterCall> for CellarV2_5Calls {
        fn from(var: PriceRouterCall) -> Self {
            CellarV2_5Calls::PriceRouter(var)
        }
    }
    impl ::std::convert::From<ReceiveFlashLoanCall> for CellarV2_5Calls {
        fn from(var: ReceiveFlashLoanCall) -> Self {
            CellarV2_5Calls::ReceiveFlashLoan(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CellarV2_5Calls {
        fn from(var: RedeemCall) -> Self {
            CellarV2_5Calls::Redeem(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for CellarV2_5Calls {
        fn from(var: RegistryCall) -> Self {
            CellarV2_5Calls::Registry(var)
        }
    }
    impl ::std::convert::From<RemoveAdaptorFromCatalogueCall> for CellarV2_5Calls {
        fn from(var: RemoveAdaptorFromCatalogueCall) -> Self {
            CellarV2_5Calls::RemoveAdaptorFromCatalogue(var)
        }
    }
    impl ::std::convert::From<RemovePositionCall> for CellarV2_5Calls {
        fn from(var: RemovePositionCall) -> Self {
            CellarV2_5Calls::RemovePosition(var)
        }
    }
    impl ::std::convert::From<RemovePositionFromCatalogueCall> for CellarV2_5Calls {
        fn from(var: RemovePositionFromCatalogueCall) -> Self {
            CellarV2_5Calls::RemovePositionFromCatalogue(var)
        }
    }
    impl ::std::convert::From<SetAutomationActionsCall> for CellarV2_5Calls {
        fn from(var: SetAutomationActionsCall) -> Self {
            CellarV2_5Calls::SetAutomationActions(var)
        }
    }
    impl ::std::convert::From<SetHoldingPositionCall> for CellarV2_5Calls {
        fn from(var: SetHoldingPositionCall) -> Self {
            CellarV2_5Calls::SetHoldingPosition(var)
        }
    }
    impl ::std::convert::From<SetRebalanceDeviationCall> for CellarV2_5Calls {
        fn from(var: SetRebalanceDeviationCall) -> Self {
            CellarV2_5Calls::SetRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<SetSharePriceOracleCall> for CellarV2_5Calls {
        fn from(var: SetSharePriceOracleCall) -> Self {
            CellarV2_5Calls::SetSharePriceOracle(var)
        }
    }
    impl ::std::convert::From<SetStrategistPayoutAddressCall> for CellarV2_5Calls {
        fn from(var: SetStrategistPayoutAddressCall) -> Self {
            CellarV2_5Calls::SetStrategistPayoutAddress(var)
        }
    }
    impl ::std::convert::From<SetStrategistPlatformCutCall> for CellarV2_5Calls {
        fn from(var: SetStrategistPlatformCutCall) -> Self {
            CellarV2_5Calls::SetStrategistPlatformCut(var)
        }
    }
    impl ::std::convert::From<SharePriceOracleCall> for CellarV2_5Calls {
        fn from(var: SharePriceOracleCall) -> Self {
            CellarV2_5Calls::SharePriceOracle(var)
        }
    }
    impl ::std::convert::From<ShareSupplyCapCall> for CellarV2_5Calls {
        fn from(var: ShareSupplyCapCall) -> Self {
            CellarV2_5Calls::ShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<SwapPositionsCall> for CellarV2_5Calls {
        fn from(var: SwapPositionsCall) -> Self {
            CellarV2_5Calls::SwapPositions(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CellarV2_5Calls {
        fn from(var: SymbolCall) -> Self {
            CellarV2_5Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<ToggleIgnorePauseCall> for CellarV2_5Calls {
        fn from(var: ToggleIgnorePauseCall) -> Self {
            CellarV2_5Calls::ToggleIgnorePause(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for CellarV2_5Calls {
        fn from(var: TotalAssetsCall) -> Self {
            CellarV2_5Calls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalAssetsWithdrawableCall> for CellarV2_5Calls {
        fn from(var: TotalAssetsWithdrawableCall) -> Self {
            CellarV2_5Calls::TotalAssetsWithdrawable(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CellarV2_5Calls {
        fn from(var: TotalSupplyCall) -> Self {
            CellarV2_5Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CellarV2_5Calls {
        fn from(var: TransferCall) -> Self {
            CellarV2_5Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CellarV2_5Calls {
        fn from(var: TransferFromCall) -> Self {
            CellarV2_5Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CellarV2_5Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CellarV2_5Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<ViewPositionBalancesCall> for CellarV2_5Calls {
        fn from(var: ViewPositionBalancesCall) -> Self {
            CellarV2_5Calls::ViewPositionBalances(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CellarV2_5Calls {
        fn from(var: WithdrawCall) -> Self {
            CellarV2_5Calls::Withdraw(var)
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
