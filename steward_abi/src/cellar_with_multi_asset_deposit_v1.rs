pub use cellarwithmultiassetdepositv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod cellarwithmultiassetdepositv1_mod {
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
    #[doc = "CellarWithMultiAssetDepositV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CELLARWITHMULTIASSETDEPOSITV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"_owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract Registry\",\n          \"name\": \"_registry\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"_name\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"string\",\n          \"name\": \"_symbol\",\n          \"type\": \"string\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"_holdingPosition\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"_holdingPositionConfig\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"_initialDeposit\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"_strategistPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_shareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"CellarWithMultiAssetDeposit__AlternativeAssetFeeTooLarge\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"CellarWithMultiAssetDeposit__AlternativeAssetNotSupported\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"CellarWithMultiAssetDeposit__CallDataLengthNotSupported\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"expectedAsset\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__AssetMismatch\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__CallToAdaptorNotAllowed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ContractNotShutdown\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ContractShutdown\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__DebtMismatch\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ExpectedAddressDoesNotMatchActual\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__FailedToForceOutPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"illiquidPosition\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"Cellar__IlliquidWithdraw\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assetsOwed\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__IncompleteWithdraw\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidFee\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidFeeCut\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__InvalidHoldingPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"requested\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"max\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__InvalidRebalanceDeviation\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__InvalidShareSupplyCap\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__MinimumConstructorMintNotMet\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__Paused\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionAlreadyUsed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"maxPositions\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__PositionArrayFull\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"sharesRemaining\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotEmpty\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotInCatalogue\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"Cellar__PositionNotUsed\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__RemovingHoldingPosition\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__SettingValueToRegistryIdZeroIsProhibited\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ShareSupplyCapExceeded\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"min\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"max\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__TotalAssetDeviatedOutsideRange\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"current\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"expected\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Cellar__TotalSharesMustRemainConstant\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ZeroAssets\",\n      \"type\": \"error\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"Cellar__ZeroShares\",\n      \"type\": \"error\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bytes\",\n          \"name\": \"data\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"AdaptorCalled\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"inCatalogue\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"AdaptorCatalogueAltered\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"asset\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"AlternativeAssetDropped\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"asset\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"holdingPosition\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"depositFee\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"AlternativeAssetUpdated\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Approval\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Deposit\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"depositAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Deposit\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"OwnershipTransferred\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionAdded\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"inCatalogue\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"PositionCatalogueAltered\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"position\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionRemoved\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"newPosition1\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint32\",\n          \"name\": \"newPosition2\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index1\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"index2\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"PositionSwapped\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"oldDeviation\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"newDeviation\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"RebalanceDeviationChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"isShutdown\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"ShutdownChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"oldPayoutAddress\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"newPayoutAddress\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"StrategistPayoutAddressChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"oldPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint64\",\n          \"name\": \"newPlatformCut\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"StrategistPlatformCutChanged\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Transfer\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"caller\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Withdraw\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DOMAIN_SEPARATOR\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"addAdaptorToCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"configurationData\",\n          \"type\": \"bytes\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"addPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"addPositionToCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"allowance\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"alternativeAssetData\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"isSupported\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"holdingPosition\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"depositFee\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"approve\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"asset\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"balanceOf\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"blockExternalReceiver\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"checkTotalAssets\",\n          \"type\": \"bool\"\n        },\n        {\n          \"internalType\": \"uint16\",\n          \"name\": \"allowableRange\",\n          \"type\": \"uint16\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"expectedPriceRouter\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"cachePriceRouter\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"components\": [\n            {\n              \"internalType\": \"address\",\n              \"name\": \"adaptor\",\n              \"type\": \"address\"\n            },\n            {\n              \"internalType\": \"bytes[]\",\n              \"name\": \"callData\",\n              \"type\": \"bytes[]\"\n            }\n          ],\n          \"internalType\": \"struct Cellar.AdaptorCall[]\",\n          \"name\": \"data\",\n          \"type\": \"tuple[]\"\n        }\n      ],\n      \"name\": \"callOnAdaptor\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"convertToShares\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"decimals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"\",\n          \"type\": \"uint8\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_newShareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"decreaseShareSupplyCap\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"deposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_alternativeAsset\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"dropAlternativeAssetData\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"feeData\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"strategistPlatformCut\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"platformFee\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"lastAccrual\",\n          \"type\": \"uint64\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"strategistPayoutAddress\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"forcePositionOut\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getCreditPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32[]\",\n          \"name\": \"\",\n          \"type\": \"uint32[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"getDebtPositions\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32[]\",\n          \"name\": \"\",\n          \"type\": \"uint32[]\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"holdingPosition\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"ignorePause\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"_newShareSupplyCap\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"name\": \"increaseShareSupplyCap\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"initiateShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"isPaused\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"isPositionUsed\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"isShutdown\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"liftShutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"locked\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"maxWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"mint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"depositAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"multiAssetDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"bytes[]\",\n          \"name\": \"data\",\n          \"type\": \"bytes[]\"\n        }\n      ],\n      \"name\": \"multicall\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"name\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"nonces\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"bytes\",\n          \"name\": \"\",\n          \"type\": \"bytes\"\n        }\n      ],\n      \"name\": \"onERC721Received\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes4\",\n          \"name\": \"\",\n          \"type\": \"bytes4\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"spender\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"value\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"deadline\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"uint8\",\n          \"name\": \"v\",\n          \"type\": \"uint8\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"r\",\n          \"type\": \"bytes32\"\n        },\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"s\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"name\": \"permit\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewMint\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"depositAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewMultiAssetDeposit\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewRedeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"previewWithdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"priceRouter\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract PriceRouter\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"redeem\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"registry\",\n      \"outputs\": [\n        {\n          \"internalType\": \"contract Registry\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"adaptor\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"removeAdaptorFromCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"removePosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"removePositionFromCatalogue\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"contract ERC20\",\n          \"name\": \"_alternativeAsset\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"_alternativeHoldingPosition\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"_alternativeAssetFee\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"setAlternativeAssetData\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"positionId\",\n          \"type\": \"uint32\"\n        }\n      ],\n      \"name\": \"setHoldingPosition\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"newDeviation\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"setRebalanceDeviation\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"payout\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"setStrategistPayoutAddress\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint64\",\n          \"name\": \"cut\",\n          \"type\": \"uint64\"\n        }\n      ],\n      \"name\": \"setStrategistPlatformCut\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"shareSupplyCap\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint192\",\n          \"name\": \"\",\n          \"type\": \"uint192\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index1\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"uint32\",\n          \"name\": \"index2\",\n          \"type\": \"uint32\"\n        },\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"inDebtArray\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"swapPositions\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"symbol\",\n      \"outputs\": [\n        {\n          \"internalType\": \"string\",\n          \"name\": \"\",\n          \"type\": \"string\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"toggleIgnorePause\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalAssets\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalAssetsWithdrawable\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totalSupply\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transfer\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"amount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"transferFrom\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"transferOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"assets\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"receiver\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"owner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"withdraw\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"shares\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n]\n  \n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CellarWithMultiAssetDepositV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CellarWithMultiAssetDepositV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CellarWithMultiAssetDepositV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CellarWithMultiAssetDepositV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CellarWithMultiAssetDepositV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                CELLARWITHMULTIASSETDEPOSITV1_ABI.clone(),
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
        #[doc = "Calls the contract's `alternativeAssetData` (0x6419111e) function"]
        pub fn alternative_asset_data(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, (bool, u32, u32)> {
            self.0
                .method_hash([100, 25, 17, 30], p0)
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
        #[doc = "Calls the contract's `dropAlternativeAssetData` (0x217bb34d) function"]
        pub fn drop_alternative_asset_data(
            &self,
            alternative_asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 123, 179, 77], alternative_asset)
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
        #[doc = "Calls the contract's `multiAssetDeposit` (0x2b91c5de) function"]
        pub fn multi_asset_deposit(
            &self,
            deposit_asset: ethers::core::types::Address,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([43, 145, 197, 222], (deposit_asset, assets, receiver))
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
        #[doc = "Calls the contract's `previewMultiAssetDeposit` (0x7ab92915) function"]
        pub fn preview_multi_asset_deposit(
            &self,
            deposit_asset: ethers::core::types::Address,
            assets: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([122, 185, 41, 21], (deposit_asset, assets))
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
        #[doc = "Calls the contract's `setAlternativeAssetData` (0x855bccb3) function"]
        pub fn set_alternative_asset_data(
            &self,
            alternative_asset: ethers::core::types::Address,
            alternative_holding_position: u32,
            alternative_asset_fee: u32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [133, 91, 204, 179],
                    (
                        alternative_asset,
                        alternative_holding_position,
                        alternative_asset_fee,
                    ),
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
        #[doc = "Gets the contract's `AlternativeAssetDropped` event"]
        pub fn alternative_asset_dropped_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AlternativeAssetDroppedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `AlternativeAssetUpdated` event"]
        pub fn alternative_asset_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AlternativeAssetUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
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
        ) -> ethers::contract::builders::Event<M, CellarWithMultiAssetDepositV1Events> {
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
    #[ethevent(
        name = "AlternativeAssetDropped",
        abi = "AlternativeAssetDropped(address)"
    )]
    pub struct AlternativeAssetDroppedFilter {
        pub asset: ethers::core::types::Address,
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
        name = "AlternativeAssetUpdated",
        abi = "AlternativeAssetUpdated(address,uint32,uint32)"
    )]
    pub struct AlternativeAssetUpdatedFilter {
        pub asset: ethers::core::types::Address,
        pub holding_position: u32,
        pub deposit_fee: u32,
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
    #[ethevent(
        name = "Deposit",
        abi = "Deposit(address,address,address,uint256,uint256)"
    )]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        pub deposit_asset: ethers::core::types::Address,
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
    pub enum CellarWithMultiAssetDepositV1Events {
        AdaptorCalledFilter(AdaptorCalledFilter),
        AdaptorCatalogueAlteredFilter(AdaptorCatalogueAlteredFilter),
        AlternativeAssetDroppedFilter(AlternativeAssetDroppedFilter),
        AlternativeAssetUpdatedFilter(AlternativeAssetUpdatedFilter),
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        DepositFilter(DepositFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionCatalogueAlteredFilter(PositionCatalogueAlteredFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for CellarWithMultiAssetDepositV1Events {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdaptorCalledFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::AdaptorCalledFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = AdaptorCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::AdaptorCatalogueAlteredFilter(decoded),
                );
            }
            if let Ok(decoded) = AlternativeAssetDroppedFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::AlternativeAssetDroppedFilter(decoded),
                );
            }
            if let Ok(decoded) = AlternativeAssetUpdatedFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::AlternativeAssetUpdatedFilter(decoded),
                );
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::PositionAddedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PositionCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::PositionCatalogueAlteredFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::PositionRemovedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::PositionSwappedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::RebalanceDeviationChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::ShutdownChangedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::StrategistPayoutAddressChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithMultiAssetDepositV1Events::StrategistPlatformCutChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarWithMultiAssetDepositV1Events::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for CellarWithMultiAssetDepositV1Events {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarWithMultiAssetDepositV1Events::AdaptorCalledFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::AdaptorCatalogueAlteredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::AlternativeAssetDroppedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::AlternativeAssetUpdatedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::ApprovalFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::DepositFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::DepositFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::OwnershipTransferredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::PositionAddedFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::PositionCatalogueAlteredFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::PositionRemovedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::PositionSwappedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::RebalanceDeviationChangedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::ShutdownChangedFilter(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Events::StrategistPayoutAddressChangedFilter(
                    element,
                ) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::StrategistPlatformCutChangedFilter(
                    element,
                ) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::TransferFilter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Events::WithdrawFilter(element) => element.fmt(f),
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
    #[doc = "Container type for all input parameters for the `alternativeAssetData`function with signature `alternativeAssetData(address)` and selector `[100, 25, 17, 30]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "alternativeAssetData", abi = "alternativeAssetData(address)")]
    pub struct AlternativeAssetDataCall(pub ethers::core::types::Address);
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
    #[doc = "Container type for all input parameters for the `dropAlternativeAssetData`function with signature `dropAlternativeAssetData(address)` and selector `[33, 123, 179, 77]`"]
    #[derive(
        Clone,
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
        name = "dropAlternativeAssetData",
        abi = "dropAlternativeAssetData(address)"
    )]
    pub struct DropAlternativeAssetDataCall {
        pub alternative_asset: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `multiAssetDeposit`function with signature `multiAssetDeposit(address,uint256,address)` and selector `[43, 145, 197, 222]`"]
    #[derive(
        Clone,
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
        name = "multiAssetDeposit",
        abi = "multiAssetDeposit(address,uint256,address)"
    )]
    pub struct MultiAssetDepositCall {
        pub deposit_asset: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `previewMultiAssetDeposit`function with signature `previewMultiAssetDeposit(address,uint256)` and selector `[122, 185, 41, 21]`"]
    #[derive(
        Clone,
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
        name = "previewMultiAssetDeposit",
        abi = "previewMultiAssetDeposit(address,uint256)"
    )]
    pub struct PreviewMultiAssetDepositCall {
        pub deposit_asset: ethers::core::types::Address,
        pub assets: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `setAlternativeAssetData`function with signature `setAlternativeAssetData(address,uint32,uint32)` and selector `[133, 91, 204, 179]`"]
    #[derive(
        Clone,
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
        name = "setAlternativeAssetData",
        abi = "setAlternativeAssetData(address,uint32,uint32)"
    )]
    pub struct SetAlternativeAssetDataCall {
        pub alternative_asset: ethers::core::types::Address,
        pub alternative_holding_position: u32,
        pub alternative_asset_fee: u32,
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
    pub enum CellarWithMultiAssetDepositV1Calls {
        DomainSeparator(DomainSeparatorCall),
        AddAdaptorToCatalogue(AddAdaptorToCatalogueCall),
        AddPosition(AddPositionCall),
        AddPositionToCatalogue(AddPositionToCatalogueCall),
        Allowance(AllowanceCall),
        AlternativeAssetData(AlternativeAssetDataCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        BalanceOf(BalanceOfCall),
        BlockExternalReceiver(BlockExternalReceiverCall),
        CachePriceRouter(CachePriceRouterCall),
        CallOnAdaptor(CallOnAdaptorCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        Decimals(DecimalsCall),
        DecreaseShareSupplyCap(DecreaseShareSupplyCapCall),
        Deposit(DepositCall),
        DropAlternativeAssetData(DropAlternativeAssetDataCall),
        FeeData(FeeDataCall),
        ForcePositionOut(ForcePositionOutCall),
        GetCreditPositions(GetCreditPositionsCall),
        GetDebtPositions(GetDebtPositionsCall),
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
        MultiAssetDeposit(MultiAssetDepositCall),
        Multicall(MulticallCall),
        Name(NameCall),
        Nonces(NoncesCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewMultiAssetDeposit(PreviewMultiAssetDepositCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PriceRouter(PriceRouterCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemoveAdaptorFromCatalogue(RemoveAdaptorFromCatalogueCall),
        RemovePosition(RemovePositionCall),
        RemovePositionFromCatalogue(RemovePositionFromCatalogueCall),
        SetAlternativeAssetData(SetAlternativeAssetDataCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
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
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for CellarWithMultiAssetDepositV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <AddAdaptorToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::AddAdaptorToCatalogue(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AddPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::AddPosition(decoded));
            }
            if let Ok(decoded) =
                <AddPositionToCatalogueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::AddPositionToCatalogue(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <AlternativeAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::AlternativeAssetData(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Asset(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BlockExternalReceiverCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::BlockExternalReceiver(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CachePriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::CachePriceRouter(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <CallOnAdaptorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::CallOnAdaptor(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::DecreaseShareSupplyCap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DropAlternativeAssetDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::DropAlternativeAssetData(decoded));
            }
            if let Ok(decoded) =
                <FeeDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::FeeData(decoded));
            }
            if let Ok(decoded) =
                <ForcePositionOutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::ForcePositionOut(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetCreditPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::GetCreditPositions(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetDebtPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::GetDebtPositions(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <HoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::HoldingPosition(decoded));
            }
            if let Ok(decoded) =
                <IgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::IgnorePause(decoded));
            }
            if let Ok(decoded) =
                <IncreaseShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::IncreaseShareSupplyCap(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <InitiateShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::InitiateShutdown(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <IsPausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::IsPaused(decoded));
            }
            if let Ok(decoded) =
                <IsPositionUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::IsPositionUsed(decoded));
            }
            if let Ok(decoded) =
                <IsShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::IsShutdown(decoded));
            }
            if let Ok(decoded) =
                <LiftShutdownCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LockedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Locked(decoded));
            }
            if let Ok(decoded) =
                <MaxDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::MaxMint(decoded));
            }
            if let Ok(decoded) =
                <MaxRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::MaxRedeem(decoded));
            }
            if let Ok(decoded) =
                <MaxWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarWithMultiAssetDepositV1Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MultiAssetDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::MultiAssetDeposit(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Multicall(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(CellarWithMultiAssetDepositV1Calls::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Nonces(decoded));
            }
            if let Ok(decoded) =
                <OnERC721ReceivedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::OnERC721Received(
                    decoded,
                ));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Permit(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewMintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PreviewMint(decoded));
            }
            if let Ok(decoded) =
                <PreviewMultiAssetDepositCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PreviewMultiAssetDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PreviewRedeem(decoded));
            }
            if let Ok(decoded) =
                <PreviewWithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) =
                <PriceRouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::PriceRouter(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Redeem(decoded));
            }
            if let Ok(decoded) =
                <RegistryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Registry(decoded));
            }
            if let Ok(decoded) =
                <RemoveAdaptorFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::RemoveAdaptorFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::RemovePosition(decoded));
            }
            if let Ok(decoded) =
                <RemovePositionFromCatalogueCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::RemovePositionFromCatalogue(decoded));
            }
            if let Ok(decoded) =
                <SetAlternativeAssetDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SetAlternativeAssetData(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetHoldingPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SetHoldingPosition(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetRebalanceDeviationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SetRebalanceDeviation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <SetStrategistPayoutAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) =
                <SetStrategistPlatformCutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) =
                <ShareSupplyCapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::ShareSupplyCap(decoded));
            }
            if let Ok(decoded) =
                <SwapPositionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <ToggleIgnorePauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::ToggleIgnorePause(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TotalAssetsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::TotalAssets(decoded));
            }
            if let Ok(decoded) =
                <TotalAssetsWithdrawableCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::TotalAssetsWithdrawable(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::TransferOwnership(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CellarWithMultiAssetDepositV1Calls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CellarWithMultiAssetDepositV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CellarWithMultiAssetDepositV1Calls::DomainSeparator(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::AddAdaptorToCatalogue(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::AddPosition(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::AddPositionToCatalogue(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::Allowance(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::AlternativeAssetData(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::Approve(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Asset(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::BalanceOf(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::BlockExternalReceiver(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::CachePriceRouter(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::CallOnAdaptor(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::ConvertToAssets(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::ConvertToShares(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Decimals(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::DecreaseShareSupplyCap(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::Deposit(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::DropAlternativeAssetData(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::FeeData(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::ForcePositionOut(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::GetCreditPositions(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::GetDebtPositions(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::HoldingPosition(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::IgnorePause(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::IncreaseShareSupplyCap(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::InitiateShutdown(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::IsPaused(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::IsPositionUsed(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::IsShutdown(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::LiftShutdown(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Locked(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::MaxDeposit(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::MaxMint(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::MaxRedeem(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::MaxWithdraw(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Mint(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::MultiAssetDeposit(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Multicall(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Name(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Nonces(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::OnERC721Received(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Owner(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Permit(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::PreviewDeposit(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::PreviewMint(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::PreviewMultiAssetDeposit(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::PreviewRedeem(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::PreviewWithdraw(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::PriceRouter(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Redeem(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Registry(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::RemoveAdaptorFromCatalogue(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::RemovePosition(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::RemovePositionFromCatalogue(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::SetAlternativeAssetData(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::SetHoldingPosition(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::SetRebalanceDeviation(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::SetStrategistPayoutAddress(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::SetStrategistPlatformCut(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::ShareSupplyCap(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::SwapPositions(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Symbol(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::ToggleIgnorePause(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::TotalAssets(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::TotalAssetsWithdrawable(element) => {
                    element.encode()
                }
                CellarWithMultiAssetDepositV1Calls::TotalSupply(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Transfer(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::TransferFrom(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::TransferOwnership(element) => element.encode(),
                CellarWithMultiAssetDepositV1Calls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CellarWithMultiAssetDepositV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CellarWithMultiAssetDepositV1Calls::DomainSeparator(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::AddAdaptorToCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::AddPosition(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::AddPositionToCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::Allowance(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::AlternativeAssetData(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Approve(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Asset(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::BalanceOf(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::BlockExternalReceiver(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::CachePriceRouter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::CallOnAdaptor(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::ConvertToAssets(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::ConvertToShares(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Decimals(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::DecreaseShareSupplyCap(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::Deposit(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::DropAlternativeAssetData(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::FeeData(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::ForcePositionOut(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::GetCreditPositions(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::GetDebtPositions(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::HoldingPosition(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::IgnorePause(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::IncreaseShareSupplyCap(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::InitiateShutdown(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::IsPaused(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::IsPositionUsed(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::IsShutdown(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::LiftShutdown(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Locked(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::MaxDeposit(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::MaxMint(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::MaxRedeem(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::MaxWithdraw(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Mint(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::MultiAssetDeposit(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Multicall(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Name(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Nonces(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::OnERC721Received(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Owner(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Permit(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::PreviewDeposit(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::PreviewMint(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::PreviewMultiAssetDeposit(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::PreviewRedeem(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::PreviewWithdraw(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::PriceRouter(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Redeem(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Registry(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::RemoveAdaptorFromCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::RemovePosition(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::RemovePositionFromCatalogue(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::SetAlternativeAssetData(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::SetHoldingPosition(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::SetRebalanceDeviation(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::SetStrategistPayoutAddress(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::SetStrategistPlatformCut(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::ShareSupplyCap(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::SwapPositions(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Symbol(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::ToggleIgnorePause(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::TotalAssets(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::TotalAssetsWithdrawable(element) => {
                    element.fmt(f)
                }
                CellarWithMultiAssetDepositV1Calls::TotalSupply(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Transfer(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::TransferFrom(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::TransferOwnership(element) => element.fmt(f),
                CellarWithMultiAssetDepositV1Calls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<AddAdaptorToCatalogueCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AddAdaptorToCatalogueCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::AddAdaptorToCatalogue(var)
        }
    }
    impl ::std::convert::From<AddPositionCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AddPositionCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::AddPosition(var)
        }
    }
    impl ::std::convert::From<AddPositionToCatalogueCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AddPositionToCatalogueCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::AddPositionToCatalogue(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AllowanceCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Allowance(var)
        }
    }
    impl ::std::convert::From<AlternativeAssetDataCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AlternativeAssetDataCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::AlternativeAssetData(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ApproveCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Approve(var)
        }
    }
    impl ::std::convert::From<AssetCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: AssetCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Asset(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BlockExternalReceiverCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: BlockExternalReceiverCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::BlockExternalReceiver(var)
        }
    }
    impl ::std::convert::From<CachePriceRouterCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: CachePriceRouterCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::CachePriceRouter(var)
        }
    }
    impl ::std::convert::From<CallOnAdaptorCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: CallOnAdaptorCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::CallOnAdaptor(var)
        }
    }
    impl ::std::convert::From<ConvertToAssetsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ConvertToAssetsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::ConvertToAssets(var)
        }
    }
    impl ::std::convert::From<ConvertToSharesCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ConvertToSharesCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::ConvertToShares(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: DecimalsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseShareSupplyCapCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: DecreaseShareSupplyCapCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::DecreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: DepositCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<DropAlternativeAssetDataCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: DropAlternativeAssetDataCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::DropAlternativeAssetData(var)
        }
    }
    impl ::std::convert::From<FeeDataCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: FeeDataCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::FeeData(var)
        }
    }
    impl ::std::convert::From<ForcePositionOutCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ForcePositionOutCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::ForcePositionOut(var)
        }
    }
    impl ::std::convert::From<GetCreditPositionsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: GetCreditPositionsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::GetCreditPositions(var)
        }
    }
    impl ::std::convert::From<GetDebtPositionsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: GetDebtPositionsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::GetDebtPositions(var)
        }
    }
    impl ::std::convert::From<HoldingPositionCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: HoldingPositionCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::HoldingPosition(var)
        }
    }
    impl ::std::convert::From<IgnorePauseCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: IgnorePauseCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::IgnorePause(var)
        }
    }
    impl ::std::convert::From<IncreaseShareSupplyCapCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: IncreaseShareSupplyCapCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::IncreaseShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<InitiateShutdownCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: InitiateShutdownCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::InitiateShutdown(var)
        }
    }
    impl ::std::convert::From<IsPausedCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: IsPausedCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::IsPaused(var)
        }
    }
    impl ::std::convert::From<IsPositionUsedCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: IsPositionUsedCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::IsPositionUsed(var)
        }
    }
    impl ::std::convert::From<IsShutdownCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: IsShutdownCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::IsShutdown(var)
        }
    }
    impl ::std::convert::From<LiftShutdownCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: LiftShutdownCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::LiftShutdown(var)
        }
    }
    impl ::std::convert::From<LockedCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: LockedCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Locked(var)
        }
    }
    impl ::std::convert::From<MaxDepositCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MaxDepositCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::MaxDeposit(var)
        }
    }
    impl ::std::convert::From<MaxMintCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MaxMintCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::MaxMint(var)
        }
    }
    impl ::std::convert::From<MaxRedeemCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MaxRedeemCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::MaxRedeem(var)
        }
    }
    impl ::std::convert::From<MaxWithdrawCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MaxWithdrawCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::MaxWithdraw(var)
        }
    }
    impl ::std::convert::From<MintCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MintCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Mint(var)
        }
    }
    impl ::std::convert::From<MultiAssetDepositCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MultiAssetDepositCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::MultiAssetDeposit(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: MulticallCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Multicall(var)
        }
    }
    impl ::std::convert::From<NameCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: NameCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Name(var)
        }
    }
    impl ::std::convert::From<NoncesCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: NoncesCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Nonces(var)
        }
    }
    impl ::std::convert::From<OnERC721ReceivedCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: OnERC721ReceivedCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::OnERC721Received(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: OwnerCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Owner(var)
        }
    }
    impl ::std::convert::From<PermitCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PermitCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Permit(var)
        }
    }
    impl ::std::convert::From<PreviewDepositCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PreviewDepositCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PreviewDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewMintCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PreviewMintCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PreviewMint(var)
        }
    }
    impl ::std::convert::From<PreviewMultiAssetDepositCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PreviewMultiAssetDepositCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PreviewMultiAssetDeposit(var)
        }
    }
    impl ::std::convert::From<PreviewRedeemCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PreviewRedeemCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PreviewRedeem(var)
        }
    }
    impl ::std::convert::From<PreviewWithdrawCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PreviewWithdrawCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PreviewWithdraw(var)
        }
    }
    impl ::std::convert::From<PriceRouterCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: PriceRouterCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::PriceRouter(var)
        }
    }
    impl ::std::convert::From<RedeemCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: RedeemCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Redeem(var)
        }
    }
    impl ::std::convert::From<RegistryCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: RegistryCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Registry(var)
        }
    }
    impl ::std::convert::From<RemoveAdaptorFromCatalogueCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: RemoveAdaptorFromCatalogueCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::RemoveAdaptorFromCatalogue(var)
        }
    }
    impl ::std::convert::From<RemovePositionCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: RemovePositionCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::RemovePosition(var)
        }
    }
    impl ::std::convert::From<RemovePositionFromCatalogueCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: RemovePositionFromCatalogueCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::RemovePositionFromCatalogue(var)
        }
    }
    impl ::std::convert::From<SetAlternativeAssetDataCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SetAlternativeAssetDataCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SetAlternativeAssetData(var)
        }
    }
    impl ::std::convert::From<SetHoldingPositionCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SetHoldingPositionCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SetHoldingPosition(var)
        }
    }
    impl ::std::convert::From<SetRebalanceDeviationCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SetRebalanceDeviationCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SetRebalanceDeviation(var)
        }
    }
    impl ::std::convert::From<SetStrategistPayoutAddressCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SetStrategistPayoutAddressCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SetStrategistPayoutAddress(var)
        }
    }
    impl ::std::convert::From<SetStrategistPlatformCutCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SetStrategistPlatformCutCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SetStrategistPlatformCut(var)
        }
    }
    impl ::std::convert::From<ShareSupplyCapCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ShareSupplyCapCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::ShareSupplyCap(var)
        }
    }
    impl ::std::convert::From<SwapPositionsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SwapPositionsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::SwapPositions(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: SymbolCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Symbol(var)
        }
    }
    impl ::std::convert::From<ToggleIgnorePauseCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: ToggleIgnorePauseCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::ToggleIgnorePause(var)
        }
    }
    impl ::std::convert::From<TotalAssetsCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TotalAssetsCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::TotalAssets(var)
        }
    }
    impl ::std::convert::From<TotalAssetsWithdrawableCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TotalAssetsWithdrawableCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::TotalAssetsWithdrawable(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TotalSupplyCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TransferCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TransferFromCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CellarWithMultiAssetDepositV1Calls {
        fn from(var: WithdrawCall) -> Self {
            CellarWithMultiAssetDepositV1Calls::Withdraw(var)
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
