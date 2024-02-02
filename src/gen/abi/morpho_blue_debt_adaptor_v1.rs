pub use morphobluedebtadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod morphobluedebtadaptorv1_mod {
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
    #[doc = "MorphoBlueDebtAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MORPHOBLUEDEBTADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_morphoBlue\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_healthFactor\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ConstructorHealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"BaseAdaptor__PricingNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__Slippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"loanToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"collateralToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"oracle\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"irm\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"uint256\",\n                        \"name\": \"lltv\",\n                        \"type\": \"uint256\"\n                    }\n                ],\n                \"internalType\": \"struct MarketParams\",\n                \"name\": \"market\",\n                \"type\": \"tuple\"\n            }\n        ],\n        \"name\": \"MorphoBlueAdaptors__MarketPositionsMustBeTracked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"loanToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"collateralToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"oracle\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"irm\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"uint256\",\n                        \"name\": \"lltv\",\n                        \"type\": \"uint256\"\n                    }\n                ],\n                \"internalType\": \"struct MarketParams\",\n                \"name\": \"market\",\n                \"type\": \"tuple\"\n            }\n        ],\n        \"name\": \"MorphoBlueDebtAdaptor__HealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"loanToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"collateralToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"oracle\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"irm\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"uint256\",\n                        \"name\": \"lltv\",\n                        \"type\": \"uint256\"\n                    }\n                ],\n                \"internalType\": \"struct MarketParams\",\n                \"name\": \"_market\",\n                \"type\": \"tuple\"\n            }\n        ],\n        \"name\": \"accrueInterest\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"loanToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"collateralToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"oracle\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"irm\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"uint256\",\n                        \"name\": \"lltv\",\n                        \"type\": \"uint256\"\n                    }\n                ],\n                \"internalType\": \"struct MarketParams\",\n                \"name\": \"_market\",\n                \"type\": \"tuple\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amountToBorrow\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"borrowFromMorphoBlue\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"minimumHealthFactor\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"morphoBlue\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract IMorpho\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"components\": [\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"loanToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"collateralToken\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"oracle\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"address\",\n                        \"name\": \"irm\",\n                        \"type\": \"address\"\n                    },\n                    {\n                        \"internalType\": \"uint256\",\n                        \"name\": \"lltv\",\n                        \"type\": \"uint256\"\n                    }\n                ],\n                \"internalType\": \"struct MarketParams\",\n                \"name\": \"_market\",\n                \"type\": \"tuple\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_debtTokenRepayAmount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"repayMorphoBlueDebt\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"slippage\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct MorphoBlueDebtAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MorphoBlueDebtAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MorphoBlueDebtAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MorphoBlueDebtAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MorphoBlueDebtAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                MORPHOBLUEDEBTADAPTORV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `accrueInterest` (0x151c1ade) function"]
        pub fn accrue_interest(
            &self,
            market: MarketParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 28, 26, 222], (market,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetOf` (0xe170a9bf) function"]
        pub fn asset_of(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([225, 112, 169, 191], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetsUsed` (0xaeffddde) function"]
        pub fn assets_used(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([174, 255, 221, 222], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x78415365) function"]
        pub fn balance_of(
            &self,
            adaptor_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrowFromMorphoBlue` (0x13b70aba) function"]
        pub fn borrow_from_morpho_blue(
            &self,
            market: MarketParams,
            amount_to_borrow: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 183, 10, 186], (market, amount_to_borrow))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x69445c31) function"]
        pub fn deposit(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Bytes,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 68, 92, 49], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `identifier` (0x7998a1c4) function"]
        pub fn identifier(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 152, 161, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isDebt` (0x89353a09) function"]
        pub fn is_debt(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([137, 53, 58, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `minimumHealthFactor` (0x1caff8b1) function"]
        pub fn minimum_health_factor(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([28, 175, 248, 177], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `morphoBlue` (0xe8ce1bfa) function"]
        pub fn morpho_blue(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 206, 27, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayMorphoBlueDebt` (0x8fb75ebe) function"]
        pub fn repay_morpho_blue_debt(
            &self,
            market: MarketParams,
            debt_token_repay_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 183, 94, 190], (market, debt_token_repay_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeApproval` (0xd3bfe76a) function"]
        pub fn revoke_approval(
            &self,
            asset: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 191, 231, 106], (asset, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slippage` (0x3e032a3b) function"]
        pub fn slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xc9111bd7) function"]
        pub fn withdraw(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
            p2: ethers::core::types::Bytes,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 17, 27, 215], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawableFrom` (0xfa50e5d2) function"]
        pub fn withdrawable_from(
            &self,
            p0: ethers::core::types::Bytes,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `accrueInterest`function with signature `accrueInterest((address,address,address,address,uint256))` and selector `[21, 28, 26, 222]`"]
    #[derive(
        Clone,
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
        name = "accrueInterest",
        abi = "accrueInterest((address,address,address,address,uint256))"
    )]
    pub struct AccrueInterestCall {
        pub market: MarketParams,
    }
    #[doc = "Container type for all input parameters for the `assetOf`function with signature `assetOf(bytes)` and selector `[225, 112, 169, 191]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "assetOf", abi = "assetOf(bytes)")]
    pub struct AssetOfCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `assetsUsed`function with signature `assetsUsed(bytes)` and selector `[174, 255, 221, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "assetsUsed", abi = "assetsUsed(bytes)")]
    pub struct AssetsUsedCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `balanceOf`function with signature `balanceOf(bytes)` and selector `[120, 65, 83, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(bytes)")]
    pub struct BalanceOfCall {
        pub adaptor_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `borrowFromMorphoBlue`function with signature `borrowFromMorphoBlue((address,address,address,address,uint256),uint256)` and selector `[19, 183, 10, 186]`"]
    #[derive(
        Clone,
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
        name = "borrowFromMorphoBlue",
        abi = "borrowFromMorphoBlue((address,address,address,address,uint256),uint256)"
    )]
    pub struct BorrowFromMorphoBlueCall {
        pub market: MarketParams,
        pub amount_to_borrow: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256,bytes,bytes)` and selector `[105, 68, 92, 49]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,bytes,bytes)")]
    pub struct DepositCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `identifier`function with signature `identifier()` and selector `[121, 152, 161, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "identifier", abi = "identifier()")]
    pub struct IdentifierCall;
    #[doc = "Container type for all input parameters for the `isDebt`function with signature `isDebt()` and selector `[137, 53, 58, 9]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "isDebt", abi = "isDebt()")]
    pub struct IsDebtCall;
    #[doc = "Container type for all input parameters for the `minimumHealthFactor`function with signature `minimumHealthFactor()` and selector `[28, 175, 248, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "minimumHealthFactor", abi = "minimumHealthFactor()")]
    pub struct MinimumHealthFactorCall;
    #[doc = "Container type for all input parameters for the `morphoBlue`function with signature `morphoBlue()` and selector `[232, 206, 27, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "morphoBlue", abi = "morphoBlue()")]
    pub struct MorphoBlueCall;
    #[doc = "Container type for all input parameters for the `repayMorphoBlueDebt`function with signature `repayMorphoBlueDebt((address,address,address,address,uint256),uint256)` and selector `[143, 183, 94, 190]`"]
    #[derive(
        Clone,
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
        name = "repayMorphoBlueDebt",
        abi = "repayMorphoBlueDebt((address,address,address,address,uint256),uint256)"
    )]
    pub struct RepayMorphoBlueDebtCall {
        pub market: MarketParams,
        pub debt_token_repay_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `revokeApproval`function with signature `revokeApproval(address,address)` and selector `[211, 191, 231, 106]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "revokeApproval", abi = "revokeApproval(address,address)")]
    pub struct RevokeApprovalCall {
        pub asset: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `slippage`function with signature `slippage()` and selector `[62, 3, 42, 59]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "slippage", abi = "slippage()")]
    pub struct SlippageCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256,address,bytes,bytes)` and selector `[201, 17, 27, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,bytes,bytes)")]
    pub struct WithdrawCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `withdrawableFrom`function with signature `withdrawableFrom(bytes,bytes)` and selector `[250, 80, 229, 210]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdrawableFrom", abi = "withdrawableFrom(bytes,bytes)")]
    pub struct WithdrawableFromCall(
        pub ethers::core::types::Bytes,
        pub ethers::core::types::Bytes,
    );
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MorphoBlueDebtAdaptorV1Calls {
        AccrueInterest(AccrueInterestCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BorrowFromMorphoBlue(BorrowFromMorphoBlueCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        MinimumHealthFactor(MinimumHealthFactorCall),
        MorphoBlue(MorphoBlueCall),
        RepayMorphoBlueDebt(RepayMorphoBlueDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for MorphoBlueDebtAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AccrueInterestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::AccrueInterest(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowFromMorphoBlueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::BorrowFromMorphoBlue(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <MinimumHealthFactorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::MinimumHealthFactor(decoded));
            }
            if let Ok(decoded) =
                <MorphoBlueCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::MorphoBlue(decoded));
            }
            if let Ok(decoded) =
                <RepayMorphoBlueDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::RepayMorphoBlueDebt(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MorphoBlueDebtAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MorphoBlueDebtAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MorphoBlueDebtAdaptorV1Calls::AccrueInterest(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::AssetOf(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::BalanceOf(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::BorrowFromMorphoBlue(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::Deposit(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::Identifier(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::IsDebt(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::MinimumHealthFactor(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::MorphoBlue(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::RepayMorphoBlueDebt(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::Slippage(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::Withdraw(element) => element.encode(),
                MorphoBlueDebtAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MorphoBlueDebtAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MorphoBlueDebtAdaptorV1Calls::AccrueInterest(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::BorrowFromMorphoBlue(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::Deposit(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::Identifier(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::MinimumHealthFactor(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::MorphoBlue(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::RepayMorphoBlueDebt(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::Slippage(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                MorphoBlueDebtAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AccrueInterestCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: AccrueInterestCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::AccrueInterest(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowFromMorphoBlueCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: BorrowFromMorphoBlueCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::BorrowFromMorphoBlue(var)
        }
    }
    impl ::std::convert::From<DepositCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<MinimumHealthFactorCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: MinimumHealthFactorCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::MinimumHealthFactor(var)
        }
    }
    impl ::std::convert::From<MorphoBlueCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: MorphoBlueCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::MorphoBlue(var)
        }
    }
    impl ::std::convert::From<RepayMorphoBlueDebtCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: RepayMorphoBlueDebtCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::RepayMorphoBlueDebt(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for MorphoBlueDebtAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            MorphoBlueDebtAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
    #[doc = "`MarketParams(address,address,address,address,uint256)`"]
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
    pub struct MarketParams {
        pub loan_token: ethers::core::types::Address,
        pub collateral_token: ethers::core::types::Address,
        pub oracle: ethers::core::types::Address,
        pub irm: ethers::core::types::Address,
        pub lltv: ethers::core::types::U256,
    }
}
