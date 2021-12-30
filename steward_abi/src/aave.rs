pub use aave_mod::*;
#[allow(clippy::too_many_arguments)]
mod aave_mod {
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
    #[doc = "Aave was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"borrowRateMode\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"borrowRate\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referral\",\r\n                \"type\": \"uint16\"\r\n            }\r\n        ],\r\n        \"name\": \"Borrow\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referral\",\r\n                \"type\": \"uint16\"\r\n            }\r\n        ],\r\n        \"name\": \"Deposit\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"target\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"initiator\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"premium\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referralCode\",\r\n                \"type\": \"uint16\"\r\n            }\r\n        ],\r\n        \"name\": \"FlashLoan\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"collateralAsset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"debtAsset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"debtToCover\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"liquidatedCollateralAmount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"liquidator\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"receiveAToken\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"LiquidationCall\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [],\r\n        \"name\": \"Paused\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"RebalanceStableBorrowRate\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"repayer\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Repay\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"liquidityRate\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"stableBorrowRate\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"variableBorrowRate\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"liquidityIndex\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"variableBorrowIndex\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"ReserveDataUpdated\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"ReserveUsedAsCollateralDisabled\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"ReserveUsedAsCollateralEnabled\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"rateMode\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Swap\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [],\r\n        \"name\": \"Unpaused\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"reserve\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Withdraw\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"FLASHLOAN_PREMIUM_TOTAL\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"LENDINGPOOL_REVISION\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"MAX_NUMBER_RESERVES\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"MAX_STABLE_RATE_BORROW_SIZE_PERCENT\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"interestRateMode\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referralCode\",\r\n                \"type\": \"uint16\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"borrow\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referralCode\",\r\n                \"type\": \"uint16\"\r\n            }\r\n        ],\r\n        \"name\": \"deposit\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"from\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"balanceFromBefore\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"balanceToBefore\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"finalizeTransfer\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"receiverAddress\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address[]\",\r\n                \"name\": \"assets\",\r\n                \"type\": \"address[]\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256[]\",\r\n                \"name\": \"amounts\",\r\n                \"type\": \"uint256[]\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256[]\",\r\n                \"name\": \"modes\",\r\n                \"type\": \"uint256[]\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"bytes\",\r\n                \"name\": \"params\",\r\n                \"type\": \"bytes\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint16\",\r\n                \"name\": \"referralCode\",\r\n                \"type\": \"uint16\"\r\n            }\r\n        ],\r\n        \"name\": \"flashLoan\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"getAddressesProvider\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"contract ILendingPoolAddressesProvider\",\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getConfiguration\",\r\n        \"outputs\": [\r\n            {\r\n                \"components\": [\r\n                    {\r\n                        \"internalType\": \"uint256\",\r\n                        \"name\": \"data\",\r\n                        \"type\": \"uint256\"\r\n                    }\r\n                ],\r\n                \"internalType\": \"struct DataTypes.ReserveConfigurationMap\",\r\n                \"name\": \"\",\r\n                \"type\": \"tuple\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getReserveData\",\r\n        \"outputs\": [\r\n            {\r\n                \"components\": [\r\n                    {\r\n                        \"components\": [\r\n                            {\r\n                                \"internalType\": \"uint256\",\r\n                                \"name\": \"data\",\r\n                                \"type\": \"uint256\"\r\n                            }\r\n                        ],\r\n                        \"internalType\": \"struct DataTypes.ReserveConfigurationMap\",\r\n                        \"name\": \"configuration\",\r\n                        \"type\": \"tuple\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint128\",\r\n                        \"name\": \"liquidityIndex\",\r\n                        \"type\": \"uint128\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint128\",\r\n                        \"name\": \"variableBorrowIndex\",\r\n                        \"type\": \"uint128\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint128\",\r\n                        \"name\": \"currentLiquidityRate\",\r\n                        \"type\": \"uint128\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint128\",\r\n                        \"name\": \"currentVariableBorrowRate\",\r\n                        \"type\": \"uint128\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint128\",\r\n                        \"name\": \"currentStableBorrowRate\",\r\n                        \"type\": \"uint128\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint40\",\r\n                        \"name\": \"lastUpdateTimestamp\",\r\n                        \"type\": \"uint40\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"address\",\r\n                        \"name\": \"aTokenAddress\",\r\n                        \"type\": \"address\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"address\",\r\n                        \"name\": \"stableDebtTokenAddress\",\r\n                        \"type\": \"address\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"address\",\r\n                        \"name\": \"variableDebtTokenAddress\",\r\n                        \"type\": \"address\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"address\",\r\n                        \"name\": \"interestRateStrategyAddress\",\r\n                        \"type\": \"address\"\r\n                    },\r\n                    {\r\n                        \"internalType\": \"uint8\",\r\n                        \"name\": \"id\",\r\n                        \"type\": \"uint8\"\r\n                    }\r\n                ],\r\n                \"internalType\": \"struct DataTypes.ReserveData\",\r\n                \"name\": \"\",\r\n                \"type\": \"tuple\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getReserveNormalizedIncome\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getReserveNormalizedVariableDebt\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"getReservesList\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"address[]\",\r\n                \"name\": \"\",\r\n                \"type\": \"address[]\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getUserAccountData\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"totalCollateralETH\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"totalDebtETH\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"availableBorrowsETH\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"currentLiquidationThreshold\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"ltv\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"healthFactor\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"getUserConfiguration\",\r\n        \"outputs\": [\r\n            {\r\n                \"components\": [\r\n                    {\r\n                        \"internalType\": \"uint256\",\r\n                        \"name\": \"data\",\r\n                        \"type\": \"uint256\"\r\n                    }\r\n                ],\r\n                \"internalType\": \"struct DataTypes.UserConfigurationMap\",\r\n                \"name\": \"\",\r\n                \"type\": \"tuple\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"aTokenAddress\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"stableDebtAddress\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"variableDebtAddress\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"interestRateStrategyAddress\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"initReserve\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"contract ILendingPoolAddressesProvider\",\r\n                \"name\": \"provider\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"initialize\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"collateralAsset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"debtAsset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"debtToCover\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"receiveAToken\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"liquidationCall\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [],\r\n        \"name\": \"paused\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"user\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"rebalanceStableBorrowRate\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"rateMode\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"onBehalfOf\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"repay\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"configuration\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"setConfiguration\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"val\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"setPause\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"rateStrategyAddress\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"setReserveInterestRateStrategyAddress\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"bool\",\r\n                \"name\": \"useAsCollateral\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"name\": \"setUserUseReserveAsCollateral\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"rateMode\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"swapBorrowRateMode\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"asset\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            },\r\n            {\r\n                \"internalType\": \"address\",\r\n                \"name\": \"to\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"withdraw\",\r\n        \"outputs\": [\r\n            {\r\n                \"internalType\": \"uint256\",\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    }\r\n]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct Aave<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for Aave<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Aave<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Aave))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> Aave<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), AAVE_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `FLASHLOAN_PREMIUM_TOTAL` (0x074b2e43) function"]
        pub fn flashloan_premium_total(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([7, 75, 46, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `LENDINGPOOL_REVISION` (0x8afaff02) function"]
        pub fn lendingpool_revision(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([138, 250, 255, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_NUMBER_RESERVES` (0xf8119d51) function"]
        pub fn max_number_reserves(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 17, 157, 81], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_STABLE_RATE_BORROW_SIZE_PERCENT` (0xe82fec2f) function"]
        pub fn max_stable_rate_borrow_size_percent(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([232, 47, 236, 47], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0xa415bcad) function"]
        pub fn borrow(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            interest_rate_mode: ethers::core::types::U256,
            referral_code: u16,
            on_behalf_of: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 21, 188, 173],
                    (
                        asset,
                        amount,
                        interest_rate_mode,
                        referral_code,
                        on_behalf_of,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xe8eda9df) function"]
        pub fn deposit(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 237, 169, 223],
                    (asset, amount, on_behalf_of, referral_code),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `finalizeTransfer` (0xd5ed3933) function"]
        pub fn finalize_transfer(
            &self,
            asset: ethers::core::types::Address,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            balance_from_before: ethers::core::types::U256,
            balance_to_before: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 237, 57, 51],
                    (
                        asset,
                        from,
                        to,
                        amount,
                        balance_from_before,
                        balance_to_before,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashLoan` (0xab9c4b5d) function"]
        pub fn flash_loan(
            &self,
            receiver_address: ethers::core::types::Address,
            assets: ::std::vec::Vec<ethers::core::types::Address>,
            amounts: ::std::vec::Vec<ethers::core::types::U256>,
            modes: ::std::vec::Vec<ethers::core::types::U256>,
            on_behalf_of: ethers::core::types::Address,
            params: ethers::core::types::Bytes,
            referral_code: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [171, 156, 75, 93],
                    (
                        receiver_address,
                        assets,
                        amounts,
                        modes,
                        on_behalf_of,
                        params,
                        referral_code,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAddressesProvider` (0xfe65acfe) function"]
        pub fn get_addresses_provider(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([254, 101, 172, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfiguration` (0xc44b11f7) function"]
        pub fn get_configuration(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ReserveConfigurationMap> {
            self.0
                .method_hash([196, 75, 17, 247], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveData` (0x35ea6a75) function"]
        pub fn get_reserve_data(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ReserveData> {
            self.0
                .method_hash([53, 234, 106, 117], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedIncome` (0xd15e0053) function"]
        pub fn get_reserve_normalized_income(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([209, 94, 0, 83], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReserveNormalizedVariableDebt` (0x386497fd) function"]
        pub fn get_reserve_normalized_variable_debt(
            &self,
            asset: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([56, 100, 151, 253], asset)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getReservesList` (0xd1946dbc) function"]
        pub fn get_reserves_list(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([209, 148, 109, 188], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserAccountData` (0xbf92857c) function"]
        pub fn get_user_account_data(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([191, 146, 133, 124], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUserConfiguration` (0x4417a583) function"]
        pub fn get_user_configuration(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, UserConfigurationMap> {
            self.0
                .method_hash([68, 23, 165, 131], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initReserve` (0x7a708e92) function"]
        pub fn init_reserve(
            &self,
            asset: ethers::core::types::Address,
            a_token_address: ethers::core::types::Address,
            stable_debt_address: ethers::core::types::Address,
            variable_debt_address: ethers::core::types::Address,
            interest_rate_strategy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [122, 112, 142, 146],
                    (
                        asset,
                        a_token_address,
                        stable_debt_address,
                        variable_debt_address,
                        interest_rate_strategy_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            provider: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], provider)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidationCall` (0x00a718a9) function"]
        pub fn liquidation_call(
            &self,
            collateral_asset: ethers::core::types::Address,
            debt_asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
            debt_to_cover: ethers::core::types::U256,
            receive_a_token: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [0, 167, 24, 169],
                    (
                        collateral_asset,
                        debt_asset,
                        user,
                        debt_to_cover,
                        receive_a_token,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rebalanceStableBorrowRate` (0xcd112382) function"]
        pub fn rebalance_stable_borrow_rate(
            &self,
            asset: ethers::core::types::Address,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 17, 35, 130], (asset, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repay` (0x573ade81) function"]
        pub fn repay(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            rate_mode: ethers::core::types::U256,
            on_behalf_of: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([87, 58, 222, 129], (asset, amount, rate_mode, on_behalf_of))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setConfiguration` (0xb8d29276) function"]
        pub fn set_configuration(
            &self,
            asset: ethers::core::types::Address,
            configuration: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 210, 146, 118], (asset, configuration))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPause` (0xbedb86fb) function"]
        pub fn set_pause(&self, val: bool) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 219, 134, 251], val)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setReserveInterestRateStrategyAddress` (0x1d2118f9) function"]
        pub fn set_reserve_interest_rate_strategy_address(
            &self,
            asset: ethers::core::types::Address,
            rate_strategy_address: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 33, 24, 249], (asset, rate_strategy_address))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUserUseReserveAsCollateral` (0x5a3b74b9) function"]
        pub fn set_user_use_reserve_as_collateral(
            &self,
            asset: ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 59, 116, 185], (asset, use_as_collateral))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapBorrowRateMode` (0x94ba89a2) function"]
        pub fn swap_borrow_rate_mode(
            &self,
            asset: ethers::core::types::Address,
            rate_mode: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 186, 137, 162], (asset, rate_mode))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x69328dec) function"]
        pub fn withdraw(
            &self,
            asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([105, 50, 141, 236], (asset, amount, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Borrow` event"]
        pub fn borrow_filter(&self) -> ethers::contract::builders::Event<M, BorrowFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `FlashLoan` event"]
        pub fn flash_loan_filter(&self) -> ethers::contract::builders::Event<M, FlashLoanFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LiquidationCall` event"]
        pub fn liquidation_call_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LiquidationCallFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RebalanceStableBorrowRate` event"]
        pub fn rebalance_stable_borrow_rate_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RebalanceStableBorrowRateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Repay` event"]
        pub fn repay_filter(&self) -> ethers::contract::builders::Event<M, RepayFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveDataUpdated` event"]
        pub fn reserve_data_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveDataUpdatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralDisabled` event"]
        pub fn reserve_used_as_collateral_disabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveUsedAsCollateralDisabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ReserveUsedAsCollateralEnabled` event"]
        pub fn reserve_used_as_collateral_enabled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ReserveUsedAsCollateralEnabledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Swap` event"]
        pub fn swap_filter(&self) -> ethers::contract::builders::Event<M, SwapFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Withdraw` event"]
        pub fn withdraw_filter(&self) -> ethers::contract::builders::Event<M, WithdrawFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AaveEvents> {
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
        name = "Borrow",
        abi = "Borrow(address,address,address,uint256,uint256,uint256,uint16)"
    )]
    pub struct BorrowFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub borrow_rate_mode: ethers::core::types::U256,
        pub borrow_rate: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral: u16,
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
        abi = "Deposit(address,address,address,uint256,uint16)"
    )]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub on_behalf_of: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub referral: u16,
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
        name = "FlashLoan",
        abi = "FlashLoan(address,address,address,uint256,uint256,uint16)"
    )]
    pub struct FlashLoanFilter {
        #[ethevent(indexed)]
        pub target: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub initiator: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub premium: ethers::core::types::U256,
        pub referral_code: u16,
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
        name = "LiquidationCall",
        abi = "LiquidationCall(address,address,address,uint256,uint256,address,bool)"
    )]
    pub struct LiquidationCallFilter {
        #[ethevent(indexed)]
        pub collateral_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub debt_asset: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub debt_to_cover: ethers::core::types::U256,
        pub liquidated_collateral_amount: ethers::core::types::U256,
        pub liquidator: ethers::core::types::Address,
        pub receive_a_token: bool,
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
    #[ethevent(name = "Paused", abi = "Paused()")]
    pub struct PausedFilter();
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
        name = "RebalanceStableBorrowRate",
        abi = "RebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "Repay", abi = "Repay(address,address,address,uint256)")]
    pub struct RepayFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub repayer: ethers::core::types::Address,
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
        name = "ReserveDataUpdated",
        abi = "ReserveDataUpdated(address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct ReserveDataUpdatedFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        pub liquidity_rate: ethers::core::types::U256,
        pub stable_borrow_rate: ethers::core::types::U256,
        pub variable_borrow_rate: ethers::core::types::U256,
        pub liquidity_index: ethers::core::types::U256,
        pub variable_borrow_index: ethers::core::types::U256,
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
        name = "ReserveUsedAsCollateralDisabled",
        abi = "ReserveUsedAsCollateralDisabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralDisabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
        name = "ReserveUsedAsCollateralEnabled",
        abi = "ReserveUsedAsCollateralEnabled(address,address)"
    )]
    pub struct ReserveUsedAsCollateralEnabledFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
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
    #[ethevent(name = "Swap", abi = "Swap(address,address,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        pub rate_mode: ethers::core::types::U256,
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
    #[ethevent(name = "Unpaused", abi = "Unpaused()")]
    pub struct UnpausedFilter();
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
    #[ethevent(name = "Withdraw", abi = "Withdraw(address,address,address,uint256)")]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub reserve: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub user: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveEvents {
        BorrowFilter(BorrowFilter),
        DepositFilter(DepositFilter),
        FlashLoanFilter(FlashLoanFilter),
        LiquidationCallFilter(LiquidationCallFilter),
        PausedFilter(PausedFilter),
        RebalanceStableBorrowRateFilter(RebalanceStableBorrowRateFilter),
        RepayFilter(RepayFilter),
        ReserveDataUpdatedFilter(ReserveDataUpdatedFilter),
        ReserveUsedAsCollateralDisabledFilter(ReserveUsedAsCollateralDisabledFilter),
        ReserveUsedAsCollateralEnabledFilter(ReserveUsedAsCollateralEnabledFilter),
        SwapFilter(SwapFilter),
        UnpausedFilter(UnpausedFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ethers::contract::EthLogDecode for AaveEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = BorrowFilter::decode_log(log) {
                return Ok(AaveEvents::BorrowFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AaveEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = FlashLoanFilter::decode_log(log) {
                return Ok(AaveEvents::FlashLoanFilter(decoded));
            }
            if let Ok(decoded) = LiquidationCallFilter::decode_log(log) {
                return Ok(AaveEvents::LiquidationCallFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(AaveEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = RebalanceStableBorrowRateFilter::decode_log(log) {
                return Ok(AaveEvents::RebalanceStableBorrowRateFilter(decoded));
            }
            if let Ok(decoded) = RepayFilter::decode_log(log) {
                return Ok(AaveEvents::RepayFilter(decoded));
            }
            if let Ok(decoded) = ReserveDataUpdatedFilter::decode_log(log) {
                return Ok(AaveEvents::ReserveDataUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralDisabledFilter::decode_log(log) {
                return Ok(AaveEvents::ReserveUsedAsCollateralDisabledFilter(decoded));
            }
            if let Ok(decoded) = ReserveUsedAsCollateralEnabledFilter::decode_log(log) {
                return Ok(AaveEvents::ReserveUsedAsCollateralEnabledFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(AaveEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(AaveEvents::UnpausedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(AaveEvents::WithdrawFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AaveEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveEvents::BorrowFilter(element) => element.fmt(f),
                AaveEvents::DepositFilter(element) => element.fmt(f),
                AaveEvents::FlashLoanFilter(element) => element.fmt(f),
                AaveEvents::LiquidationCallFilter(element) => element.fmt(f),
                AaveEvents::PausedFilter(element) => element.fmt(f),
                AaveEvents::RebalanceStableBorrowRateFilter(element) => element.fmt(f),
                AaveEvents::RepayFilter(element) => element.fmt(f),
                AaveEvents::ReserveDataUpdatedFilter(element) => element.fmt(f),
                AaveEvents::ReserveUsedAsCollateralDisabledFilter(element) => element.fmt(f),
                AaveEvents::ReserveUsedAsCollateralEnabledFilter(element) => element.fmt(f),
                AaveEvents::SwapFilter(element) => element.fmt(f),
                AaveEvents::UnpausedFilter(element) => element.fmt(f),
                AaveEvents::WithdrawFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FLASHLOAN_PREMIUM_TOTAL`function with signature `FLASHLOAN_PREMIUM_TOTAL()` and selector `[7, 75, 46, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "FLASHLOAN_PREMIUM_TOTAL", abi = "FLASHLOAN_PREMIUM_TOTAL()")]
    pub struct FlashloanPremiumTotalCall;
    #[doc = "Container type for all input parameters for the `LENDINGPOOL_REVISION`function with signature `LENDINGPOOL_REVISION()` and selector `[138, 250, 255, 2]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "LENDINGPOOL_REVISION", abi = "LENDINGPOOL_REVISION()")]
    pub struct LendingpoolRevisionCall;
    #[doc = "Container type for all input parameters for the `MAX_NUMBER_RESERVES`function with signature `MAX_NUMBER_RESERVES()` and selector `[248, 17, 157, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "MAX_NUMBER_RESERVES", abi = "MAX_NUMBER_RESERVES()")]
    pub struct MaxNumberReservesCall;
    #[doc = "Container type for all input parameters for the `MAX_STABLE_RATE_BORROW_SIZE_PERCENT`function with signature `MAX_STABLE_RATE_BORROW_SIZE_PERCENT()` and selector `[232, 47, 236, 47]`"]
    #[derive(
        Clone,
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
        name = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT",
        abi = "MAX_STABLE_RATE_BORROW_SIZE_PERCENT()"
    )]
    pub struct MaxStableRateBorrowSizePercentCall;
    #[doc = "Container type for all input parameters for the `borrow`function with signature `borrow(address,uint256,uint256,uint16,address)` and selector `[164, 21, 188, 173]`"]
    #[derive(
        Clone,
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
        name = "borrow",
        abi = "borrow(address,uint256,uint256,uint16,address)"
    )]
    pub struct BorrowCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub interest_rate_mode: ethers::core::types::U256,
        pub referral_code: u16,
        pub on_behalf_of: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(address,uint256,address,uint16)` and selector `[232, 237, 169, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "deposit", abi = "deposit(address,uint256,address,uint16)")]
    pub struct DepositCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `finalizeTransfer`function with signature `finalizeTransfer(address,address,address,uint256,uint256,uint256)` and selector `[213, 237, 57, 51]`"]
    #[derive(
        Clone,
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
        name = "finalizeTransfer",
        abi = "finalizeTransfer(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct FinalizeTransferCall {
        pub asset: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub balance_from_before: ethers::core::types::U256,
        pub balance_to_before: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `flashLoan`function with signature `flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)` and selector `[171, 156, 75, 93]`"]
    #[derive(
        Clone,
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
        name = "flashLoan",
        abi = "flashLoan(address,address[],uint256[],uint256[],address,bytes,uint16)"
    )]
    pub struct FlashLoanCall {
        pub receiver_address: ethers::core::types::Address,
        pub assets: ::std::vec::Vec<ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub modes: ::std::vec::Vec<ethers::core::types::U256>,
        pub on_behalf_of: ethers::core::types::Address,
        pub params: ethers::core::types::Bytes,
        pub referral_code: u16,
    }
    #[doc = "Container type for all input parameters for the `getAddressesProvider`function with signature `getAddressesProvider()` and selector `[254, 101, 172, 254]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getAddressesProvider", abi = "getAddressesProvider()")]
    pub struct GetAddressesProviderCall;
    #[doc = "Container type for all input parameters for the `getConfiguration`function with signature `getConfiguration(address)` and selector `[196, 75, 17, 247]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getConfiguration", abi = "getConfiguration(address)")]
    pub struct GetConfigurationCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveData`function with signature `getReserveData(address)` and selector `[53, 234, 106, 117]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getReserveData", abi = "getReserveData(address)")]
    pub struct GetReserveDataCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedIncome`function with signature `getReserveNormalizedIncome(address)` and selector `[209, 94, 0, 83]`"]
    #[derive(
        Clone,
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
        name = "getReserveNormalizedIncome",
        abi = "getReserveNormalizedIncome(address)"
    )]
    pub struct GetReserveNormalizedIncomeCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReserveNormalizedVariableDebt`function with signature `getReserveNormalizedVariableDebt(address)` and selector `[56, 100, 151, 253]`"]
    #[derive(
        Clone,
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
        name = "getReserveNormalizedVariableDebt",
        abi = "getReserveNormalizedVariableDebt(address)"
    )]
    pub struct GetReserveNormalizedVariableDebtCall {
        pub asset: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getReservesList`function with signature `getReservesList()` and selector `[209, 148, 109, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getReservesList", abi = "getReservesList()")]
    pub struct GetReservesListCall;
    #[doc = "Container type for all input parameters for the `getUserAccountData`function with signature `getUserAccountData(address)` and selector `[191, 146, 133, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getUserAccountData", abi = "getUserAccountData(address)")]
    pub struct GetUserAccountDataCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getUserConfiguration`function with signature `getUserConfiguration(address)` and selector `[68, 23, 165, 131]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getUserConfiguration", abi = "getUserConfiguration(address)")]
    pub struct GetUserConfigurationCall {
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initReserve`function with signature `initReserve(address,address,address,address,address)` and selector `[122, 112, 142, 146]`"]
    #[derive(
        Clone,
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
        name = "initReserve",
        abi = "initReserve(address,address,address,address,address)"
    )]
    pub struct InitReserveCall {
        pub asset: ethers::core::types::Address,
        pub a_token_address: ethers::core::types::Address,
        pub stable_debt_address: ethers::core::types::Address,
        pub variable_debt_address: ethers::core::types::Address,
        pub interest_rate_strategy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `initialize`function with signature `initialize(address)` and selector `[196, 214, 109, 232]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub provider: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `liquidationCall`function with signature `liquidationCall(address,address,address,uint256,bool)` and selector `[0, 167, 24, 169]`"]
    #[derive(
        Clone,
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
        name = "liquidationCall",
        abi = "liquidationCall(address,address,address,uint256,bool)"
    )]
    pub struct LiquidationCallCall {
        pub collateral_asset: ethers::core::types::Address,
        pub debt_asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
        pub debt_to_cover: ethers::core::types::U256,
        pub receive_a_token: bool,
    }
    #[doc = "Container type for all input parameters for the `paused`function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `rebalanceStableBorrowRate`function with signature `rebalanceStableBorrowRate(address,address)` and selector `[205, 17, 35, 130]`"]
    #[derive(
        Clone,
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
        name = "rebalanceStableBorrowRate",
        abi = "rebalanceStableBorrowRate(address,address)"
    )]
    pub struct RebalanceStableBorrowRateCall {
        pub asset: ethers::core::types::Address,
        pub user: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `repay`function with signature `repay(address,uint256,uint256,address)` and selector `[87, 58, 222, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "repay", abi = "repay(address,uint256,uint256,address)")]
    pub struct RepayCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub rate_mode: ethers::core::types::U256,
        pub on_behalf_of: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setConfiguration`function with signature `setConfiguration(address,uint256)` and selector `[184, 210, 146, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setConfiguration", abi = "setConfiguration(address,uint256)")]
    pub struct SetConfigurationCall {
        pub asset: ethers::core::types::Address,
        pub configuration: ethers::core::types::U256,
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
        pub val: bool,
    }
    #[doc = "Container type for all input parameters for the `setReserveInterestRateStrategyAddress`function with signature `setReserveInterestRateStrategyAddress(address,address)` and selector `[29, 33, 24, 249]`"]
    #[derive(
        Clone,
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
        name = "setReserveInterestRateStrategyAddress",
        abi = "setReserveInterestRateStrategyAddress(address,address)"
    )]
    pub struct SetReserveInterestRateStrategyAddressCall {
        pub asset: ethers::core::types::Address,
        pub rate_strategy_address: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setUserUseReserveAsCollateral`function with signature `setUserUseReserveAsCollateral(address,bool)` and selector `[90, 59, 116, 185]`"]
    #[derive(
        Clone,
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
        name = "setUserUseReserveAsCollateral",
        abi = "setUserUseReserveAsCollateral(address,bool)"
    )]
    pub struct SetUserUseReserveAsCollateralCall {
        pub asset: ethers::core::types::Address,
        pub use_as_collateral: bool,
    }
    #[doc = "Container type for all input parameters for the `swapBorrowRateMode`function with signature `swapBorrowRateMode(address,uint256)` and selector `[148, 186, 137, 162]`"]
    #[derive(
        Clone,
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
        name = "swapBorrowRateMode",
        abi = "swapBorrowRateMode(address,uint256)"
    )]
    pub struct SwapBorrowRateModeCall {
        pub asset: ethers::core::types::Address,
        pub rate_mode: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(address,uint256,address)` and selector `[105, 50, 141, 236]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(address,uint256,address)")]
    pub struct WithdrawCall {
        pub asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub to: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveCalls {
        FlashloanPremiumTotal(FlashloanPremiumTotalCall),
        LendingpoolRevision(LendingpoolRevisionCall),
        MaxNumberReserves(MaxNumberReservesCall),
        MaxStableRateBorrowSizePercent(MaxStableRateBorrowSizePercentCall),
        Borrow(BorrowCall),
        Deposit(DepositCall),
        FinalizeTransfer(FinalizeTransferCall),
        FlashLoan(FlashLoanCall),
        GetAddressesProvider(GetAddressesProviderCall),
        GetConfiguration(GetConfigurationCall),
        GetReserveData(GetReserveDataCall),
        GetReserveNormalizedIncome(GetReserveNormalizedIncomeCall),
        GetReserveNormalizedVariableDebt(GetReserveNormalizedVariableDebtCall),
        GetReservesList(GetReservesListCall),
        GetUserAccountData(GetUserAccountDataCall),
        GetUserConfiguration(GetUserConfigurationCall),
        InitReserve(InitReserveCall),
        Initialize(InitializeCall),
        LiquidationCall(LiquidationCallCall),
        Paused(PausedCall),
        RebalanceStableBorrowRate(RebalanceStableBorrowRateCall),
        Repay(RepayCall),
        SetConfiguration(SetConfigurationCall),
        SetPause(SetPauseCall),
        SetReserveInterestRateStrategyAddress(SetReserveInterestRateStrategyAddressCall),
        SetUserUseReserveAsCollateral(SetUserUseReserveAsCollateralCall),
        SwapBorrowRateMode(SwapBorrowRateModeCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for AaveCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FlashloanPremiumTotalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::FlashloanPremiumTotal(decoded));
            }
            if let Ok(decoded) =
                <LendingpoolRevisionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::LendingpoolRevision(decoded));
            }
            if let Ok(decoded) =
                <MaxNumberReservesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::MaxNumberReserves(decoded));
            }
            if let Ok(decoded) =
                <MaxStableRateBorrowSizePercentCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::MaxStableRateBorrowSizePercent(decoded));
            }
            if let Ok(decoded) = <BorrowCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Borrow(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FinalizeTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::FinalizeTransfer(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <GetAddressesProviderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetAddressesProvider(decoded));
            }
            if let Ok(decoded) =
                <GetConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetConfiguration(decoded));
            }
            if let Ok(decoded) =
                <GetReserveDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetReserveData(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedIncomeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::GetReserveNormalizedIncome(decoded));
            }
            if let Ok(decoded) =
                <GetReserveNormalizedVariableDebtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::GetReserveNormalizedVariableDebt(decoded));
            }
            if let Ok(decoded) =
                <GetReservesListCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetReservesList(decoded));
            }
            if let Ok(decoded) =
                <GetUserAccountDataCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetUserAccountData(decoded));
            }
            if let Ok(decoded) =
                <GetUserConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::GetUserConfiguration(decoded));
            }
            if let Ok(decoded) =
                <InitReserveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::InitReserve(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LiquidationCallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::LiquidationCall(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Paused(decoded));
            }
            if let Ok(decoded) =
                <RebalanceStableBorrowRateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::RebalanceStableBorrowRate(decoded));
            }
            if let Ok(decoded) = <RepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Repay(decoded));
            }
            if let Ok(decoded) =
                <SetConfigurationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::SetConfiguration(decoded));
            }
            if let Ok(decoded) =
                <SetPauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::SetPause(decoded));
            }
            if let Ok(decoded) =
                <SetReserveInterestRateStrategyAddressCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::SetReserveInterestRateStrategyAddress(decoded));
            }
            if let Ok(decoded) =
                <SetUserUseReserveAsCollateralCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(AaveCalls::SetUserUseReserveAsCollateral(decoded));
            }
            if let Ok(decoded) =
                <SwapBorrowRateModeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::SwapBorrowRateMode(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveCalls::FlashloanPremiumTotal(element) => element.encode(),
                AaveCalls::LendingpoolRevision(element) => element.encode(),
                AaveCalls::MaxNumberReserves(element) => element.encode(),
                AaveCalls::MaxStableRateBorrowSizePercent(element) => element.encode(),
                AaveCalls::Borrow(element) => element.encode(),
                AaveCalls::Deposit(element) => element.encode(),
                AaveCalls::FinalizeTransfer(element) => element.encode(),
                AaveCalls::FlashLoan(element) => element.encode(),
                AaveCalls::GetAddressesProvider(element) => element.encode(),
                AaveCalls::GetConfiguration(element) => element.encode(),
                AaveCalls::GetReserveData(element) => element.encode(),
                AaveCalls::GetReserveNormalizedIncome(element) => element.encode(),
                AaveCalls::GetReserveNormalizedVariableDebt(element) => element.encode(),
                AaveCalls::GetReservesList(element) => element.encode(),
                AaveCalls::GetUserAccountData(element) => element.encode(),
                AaveCalls::GetUserConfiguration(element) => element.encode(),
                AaveCalls::InitReserve(element) => element.encode(),
                AaveCalls::Initialize(element) => element.encode(),
                AaveCalls::LiquidationCall(element) => element.encode(),
                AaveCalls::Paused(element) => element.encode(),
                AaveCalls::RebalanceStableBorrowRate(element) => element.encode(),
                AaveCalls::Repay(element) => element.encode(),
                AaveCalls::SetConfiguration(element) => element.encode(),
                AaveCalls::SetPause(element) => element.encode(),
                AaveCalls::SetReserveInterestRateStrategyAddress(element) => element.encode(),
                AaveCalls::SetUserUseReserveAsCollateral(element) => element.encode(),
                AaveCalls::SwapBorrowRateMode(element) => element.encode(),
                AaveCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveCalls::FlashloanPremiumTotal(element) => element.fmt(f),
                AaveCalls::LendingpoolRevision(element) => element.fmt(f),
                AaveCalls::MaxNumberReserves(element) => element.fmt(f),
                AaveCalls::MaxStableRateBorrowSizePercent(element) => element.fmt(f),
                AaveCalls::Borrow(element) => element.fmt(f),
                AaveCalls::Deposit(element) => element.fmt(f),
                AaveCalls::FinalizeTransfer(element) => element.fmt(f),
                AaveCalls::FlashLoan(element) => element.fmt(f),
                AaveCalls::GetAddressesProvider(element) => element.fmt(f),
                AaveCalls::GetConfiguration(element) => element.fmt(f),
                AaveCalls::GetReserveData(element) => element.fmt(f),
                AaveCalls::GetReserveNormalizedIncome(element) => element.fmt(f),
                AaveCalls::GetReserveNormalizedVariableDebt(element) => element.fmt(f),
                AaveCalls::GetReservesList(element) => element.fmt(f),
                AaveCalls::GetUserAccountData(element) => element.fmt(f),
                AaveCalls::GetUserConfiguration(element) => element.fmt(f),
                AaveCalls::InitReserve(element) => element.fmt(f),
                AaveCalls::Initialize(element) => element.fmt(f),
                AaveCalls::LiquidationCall(element) => element.fmt(f),
                AaveCalls::Paused(element) => element.fmt(f),
                AaveCalls::RebalanceStableBorrowRate(element) => element.fmt(f),
                AaveCalls::Repay(element) => element.fmt(f),
                AaveCalls::SetConfiguration(element) => element.fmt(f),
                AaveCalls::SetPause(element) => element.fmt(f),
                AaveCalls::SetReserveInterestRateStrategyAddress(element) => element.fmt(f),
                AaveCalls::SetUserUseReserveAsCollateral(element) => element.fmt(f),
                AaveCalls::SwapBorrowRateMode(element) => element.fmt(f),
                AaveCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FlashloanPremiumTotalCall> for AaveCalls {
        fn from(var: FlashloanPremiumTotalCall) -> Self {
            AaveCalls::FlashloanPremiumTotal(var)
        }
    }
    impl ::std::convert::From<LendingpoolRevisionCall> for AaveCalls {
        fn from(var: LendingpoolRevisionCall) -> Self {
            AaveCalls::LendingpoolRevision(var)
        }
    }
    impl ::std::convert::From<MaxNumberReservesCall> for AaveCalls {
        fn from(var: MaxNumberReservesCall) -> Self {
            AaveCalls::MaxNumberReserves(var)
        }
    }
    impl ::std::convert::From<MaxStableRateBorrowSizePercentCall> for AaveCalls {
        fn from(var: MaxStableRateBorrowSizePercentCall) -> Self {
            AaveCalls::MaxStableRateBorrowSizePercent(var)
        }
    }
    impl ::std::convert::From<BorrowCall> for AaveCalls {
        fn from(var: BorrowCall) -> Self {
            AaveCalls::Borrow(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveCalls {
        fn from(var: DepositCall) -> Self {
            AaveCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<FinalizeTransferCall> for AaveCalls {
        fn from(var: FinalizeTransferCall) -> Self {
            AaveCalls::FinalizeTransfer(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for AaveCalls {
        fn from(var: FlashLoanCall) -> Self {
            AaveCalls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<GetAddressesProviderCall> for AaveCalls {
        fn from(var: GetAddressesProviderCall) -> Self {
            AaveCalls::GetAddressesProvider(var)
        }
    }
    impl ::std::convert::From<GetConfigurationCall> for AaveCalls {
        fn from(var: GetConfigurationCall) -> Self {
            AaveCalls::GetConfiguration(var)
        }
    }
    impl ::std::convert::From<GetReserveDataCall> for AaveCalls {
        fn from(var: GetReserveDataCall) -> Self {
            AaveCalls::GetReserveData(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedIncomeCall> for AaveCalls {
        fn from(var: GetReserveNormalizedIncomeCall) -> Self {
            AaveCalls::GetReserveNormalizedIncome(var)
        }
    }
    impl ::std::convert::From<GetReserveNormalizedVariableDebtCall> for AaveCalls {
        fn from(var: GetReserveNormalizedVariableDebtCall) -> Self {
            AaveCalls::GetReserveNormalizedVariableDebt(var)
        }
    }
    impl ::std::convert::From<GetReservesListCall> for AaveCalls {
        fn from(var: GetReservesListCall) -> Self {
            AaveCalls::GetReservesList(var)
        }
    }
    impl ::std::convert::From<GetUserAccountDataCall> for AaveCalls {
        fn from(var: GetUserAccountDataCall) -> Self {
            AaveCalls::GetUserAccountData(var)
        }
    }
    impl ::std::convert::From<GetUserConfigurationCall> for AaveCalls {
        fn from(var: GetUserConfigurationCall) -> Self {
            AaveCalls::GetUserConfiguration(var)
        }
    }
    impl ::std::convert::From<InitReserveCall> for AaveCalls {
        fn from(var: InitReserveCall) -> Self {
            AaveCalls::InitReserve(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for AaveCalls {
        fn from(var: InitializeCall) -> Self {
            AaveCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<LiquidationCallCall> for AaveCalls {
        fn from(var: LiquidationCallCall) -> Self {
            AaveCalls::LiquidationCall(var)
        }
    }
    impl ::std::convert::From<PausedCall> for AaveCalls {
        fn from(var: PausedCall) -> Self {
            AaveCalls::Paused(var)
        }
    }
    impl ::std::convert::From<RebalanceStableBorrowRateCall> for AaveCalls {
        fn from(var: RebalanceStableBorrowRateCall) -> Self {
            AaveCalls::RebalanceStableBorrowRate(var)
        }
    }
    impl ::std::convert::From<RepayCall> for AaveCalls {
        fn from(var: RepayCall) -> Self {
            AaveCalls::Repay(var)
        }
    }
    impl ::std::convert::From<SetConfigurationCall> for AaveCalls {
        fn from(var: SetConfigurationCall) -> Self {
            AaveCalls::SetConfiguration(var)
        }
    }
    impl ::std::convert::From<SetPauseCall> for AaveCalls {
        fn from(var: SetPauseCall) -> Self {
            AaveCalls::SetPause(var)
        }
    }
    impl ::std::convert::From<SetReserveInterestRateStrategyAddressCall> for AaveCalls {
        fn from(var: SetReserveInterestRateStrategyAddressCall) -> Self {
            AaveCalls::SetReserveInterestRateStrategyAddress(var)
        }
    }
    impl ::std::convert::From<SetUserUseReserveAsCollateralCall> for AaveCalls {
        fn from(var: SetUserUseReserveAsCollateralCall) -> Self {
            AaveCalls::SetUserUseReserveAsCollateral(var)
        }
    }
    impl ::std::convert::From<SwapBorrowRateModeCall> for AaveCalls {
        fn from(var: SwapBorrowRateModeCall) -> Self {
            AaveCalls::SwapBorrowRateMode(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AaveCalls {
        fn from(var: WithdrawCall) -> Self {
            AaveCalls::Withdraw(var)
        }
    }
    #[doc = "`ReserveConfigurationMap(uint256)`"]
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
    pub struct ReserveConfigurationMap {
        pub data: ethers::core::types::U256,
    }
    #[doc = "`ReserveData((uint256),uint128,uint128,uint128,uint128,uint128,uint40,address,address,address,address,uint8)`"]
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
    pub struct ReserveData {
        pub configuration: ReserveConfigurationMap,
        pub liquidity_index: u128,
        pub variable_borrow_index: u128,
        pub current_liquidity_rate: u128,
        pub current_variable_borrow_rate: u128,
        pub current_stable_borrow_rate: u128,
        pub last_update_timestamp: u64,
        pub a_token_address: ethers::core::types::Address,
        pub stable_debt_token_address: ethers::core::types::Address,
        pub variable_debt_token_address: ethers::core::types::Address,
        pub interest_rate_strategy_address: ethers::core::types::Address,
        pub id: u8,
    }
    #[doc = "`UserConfigurationMap(uint256)`"]
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
    pub struct UserConfigurationMap {
        pub data: ethers::core::types::U256,
    }
}
