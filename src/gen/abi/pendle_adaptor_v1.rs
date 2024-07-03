pub use pendleadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod pendleadaptorv1_mod {
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
    #[doc = "PendleAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static PENDLEADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("{\n    \"abi\": [\n        {\n            \"type\": \"constructor\",\n            \"inputs\": [\n                {\n                    \"name\": \"_marketFactory\",\n                    \"type\": \"address\",\n                    \"internalType\": \"address\"\n                },\n                {\n                    \"name\": \"_router\",\n                    \"type\": \"address\",\n                    \"internalType\": \"address\"\n                }\n            ],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"addLiquidityDualSyAndPt\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"netSyDesired\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"netPtDesired\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minLpOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"assetOf\",\n            \"inputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract ERC20\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"assetsUsed\",\n            \"inputs\": [\n                {\n                    \"name\": \"adaptorData\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [\n                {\n                    \"name\": \"assets\",\n                    \"type\": \"address[]\",\n                    \"internalType\": \"contract ERC20[]\"\n                }\n            ],\n            \"stateMutability\": \"view\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"balanceOf\",\n            \"inputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"deposit\",\n            \"inputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"identifier\",\n            \"inputs\": [],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes32\",\n                    \"internalType\": \"bytes32\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"isDebt\",\n            \"inputs\": [],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"bool\",\n                    \"internalType\": \"bool\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"marketFactory\",\n            \"inputs\": [],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IMarketFactory\"\n                }\n            ],\n            \"stateMutability\": \"view\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"mintPyFromSy\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"netSyIn\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minPyOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"mintSyFromToken\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"minSyOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"input\",\n                    \"type\": \"tuple\",\n                    \"internalType\": \"struct TokenInput\",\n                    \"components\": [\n                        {\n                            \"name\": \"tokenIn\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"netTokenIn\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"tokenMintSy\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"pendleSwap\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"swapData\",\n                            \"type\": \"tuple\",\n                            \"internalType\": \"struct SwapData\",\n                            \"components\": [\n                                {\n                                    \"name\": \"swapType\",\n                                    \"type\": \"uint8\",\n                                    \"internalType\": \"enum SwapType\"\n                                },\n                                {\n                                    \"name\": \"extRouter\",\n                                    \"type\": \"address\",\n                                    \"internalType\": \"address\"\n                                },\n                                {\n                                    \"name\": \"extCalldata\",\n                                    \"type\": \"bytes\",\n                                    \"internalType\": \"bytes\"\n                                },\n                                {\n                                    \"name\": \"needScale\",\n                                    \"type\": \"bool\",\n                                    \"internalType\": \"bool\"\n                                }\n                            ]\n                        }\n                    ]\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"redeemPyToSy\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"netPyIn\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minSyOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"redeemSyToToken\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"netSyIn\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"output\",\n                    \"type\": \"tuple\",\n                    \"internalType\": \"struct TokenOutput\",\n                    \"components\": [\n                        {\n                            \"name\": \"tokenOut\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"minTokenOut\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"tokenRedeemSy\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"pendleSwap\",\n                            \"type\": \"address\",\n                            \"internalType\": \"address\"\n                        },\n                        {\n                            \"name\": \"swapData\",\n                            \"type\": \"tuple\",\n                            \"internalType\": \"struct SwapData\",\n                            \"components\": [\n                                {\n                                    \"name\": \"swapType\",\n                                    \"type\": \"uint8\",\n                                    \"internalType\": \"enum SwapType\"\n                                },\n                                {\n                                    \"name\": \"extRouter\",\n                                    \"type\": \"address\",\n                                    \"internalType\": \"address\"\n                                },\n                                {\n                                    \"name\": \"extCalldata\",\n                                    \"type\": \"bytes\",\n                                    \"internalType\": \"bytes\"\n                                },\n                                {\n                                    \"name\": \"needScale\",\n                                    \"type\": \"bool\",\n                                    \"internalType\": \"bool\"\n                                }\n                            ]\n                        }\n                    ]\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"removeLiquidityDualSyAndPt\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"netLpToRemove\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minSyOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minPtOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"revokeApproval\",\n            \"inputs\": [\n                {\n                    \"name\": \"asset\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract ERC20\"\n                },\n                {\n                    \"name\": \"spender\",\n                    \"type\": \"address\",\n                    \"internalType\": \"address\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"router\",\n            \"inputs\": [],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPAllActionV3\"\n                }\n            ],\n            \"stateMutability\": \"view\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"slippage\",\n            \"inputs\": [],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"uint32\",\n                    \"internalType\": \"uint32\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"swapExactPtForYt\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"exactPtIn\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minYtOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"guessTotalYtToSwap\",\n                    \"type\": \"tuple\",\n                    \"internalType\": \"struct ApproxParams\",\n                    \"components\": [\n                        {\n                            \"name\": \"guessMin\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"guessMax\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"guessOffchain\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"maxIteration\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"eps\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        }\n                    ]\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"swapExactYtForPt\",\n            \"inputs\": [\n                {\n                    \"name\": \"market\",\n                    \"type\": \"address\",\n                    \"internalType\": \"contract IPendleMarket\"\n                },\n                {\n                    \"name\": \"exactYtIn\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"minPtOut\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"guessTotalPtFromSwap\",\n                    \"type\": \"tuple\",\n                    \"internalType\": \"struct ApproxParams\",\n                    \"components\": [\n                        {\n                            \"name\": \"guessMin\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"guessMax\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"guessOffchain\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"maxIteration\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        },\n                        {\n                            \"name\": \"eps\",\n                            \"type\": \"uint256\",\n                            \"internalType\": \"uint256\"\n                        }\n                    ]\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"nonpayable\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"withdraw\",\n            \"inputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"address\",\n                    \"internalType\": \"address\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"function\",\n            \"name\": \"withdrawableFrom\",\n            \"inputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                },\n                {\n                    \"name\": \"\",\n                    \"type\": \"bytes\",\n                    \"internalType\": \"bytes\"\n                }\n            ],\n            \"outputs\": [\n                {\n                    \"name\": \"\",\n                    \"type\": \"uint256\",\n                    \"internalType\": \"uint256\"\n                }\n            ],\n            \"stateMutability\": \"pure\"\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__ConstructorHealthFactorTooLow\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__PricingNotSupported\",\n            \"inputs\": [\n                {\n                    \"name\": \"asset\",\n                    \"type\": \"address\",\n                    \"internalType\": \"address\"\n                }\n            ]\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__Slippage\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"PendleAdaptor__BadMarket\",\n            \"inputs\": []\n        },\n        {\n            \"type\": \"error\",\n            \"name\": \"PendleAdaptor__UseAggregatorToSwap\",\n            \"inputs\": []\n        }\n    ]\n}\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct PendleAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for PendleAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for PendleAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PendleAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> PendleAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                PENDLEADAPTORV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `addLiquidityDualSyAndPt` (0x3188a48d) function"]
        pub fn add_liquidity_dual_sy_and_pt(
            &self,
            market: ethers::core::types::Address,
            net_sy_desired: ethers::core::types::U256,
            net_pt_desired: ethers::core::types::U256,
            min_lp_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [49, 136, 164, 141],
                    (market, net_sy_desired, net_pt_desired, min_lp_out),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `assetOf` (0xe170a9bf) function"]
        pub fn asset_of(
            &self,
            p0: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([225, 112, 169, 191], p0)
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
            p0: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], p0)
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
        #[doc = "Calls the contract's `marketFactory` (0x06ae7095) function"]
        pub fn market_factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([6, 174, 112, 149], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintPyFromSy` (0x55456c79) function"]
        pub fn mint_py_from_sy(
            &self,
            market: ethers::core::types::Address,
            net_sy_in: ethers::core::types::U256,
            min_py_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 69, 108, 121], (market, net_sy_in, min_py_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintSyFromToken` (0x66c74d33) function"]
        pub fn mint_sy_from_token(
            &self,
            market: ethers::core::types::Address,
            min_sy_out: ethers::core::types::U256,
            input: TokenInput,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 199, 77, 51], (market, min_sy_out, input))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemPyToSy` (0x5e46cd65) function"]
        pub fn redeem_py_to_sy(
            &self,
            market: ethers::core::types::Address,
            net_py_in: ethers::core::types::U256,
            min_sy_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 70, 205, 101], (market, net_py_in, min_sy_out))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeemSyToToken` (0x303d28fa) function"]
        pub fn redeem_sy_to_token(
            &self,
            market: ethers::core::types::Address,
            net_sy_in: ethers::core::types::U256,
            output: TokenOutput,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 61, 40, 250], (market, net_sy_in, output))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityDualSyAndPt` (0x2a9b3568) function"]
        pub fn remove_liquidity_dual_sy_and_pt(
            &self,
            market: ethers::core::types::Address,
            net_lp_to_remove: ethers::core::types::U256,
            min_sy_out: ethers::core::types::U256,
            min_pt_out: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [42, 155, 53, 104],
                    (market, net_lp_to_remove, min_sy_out, min_pt_out),
                )
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
        #[doc = "Calls the contract's `router` (0xf887ea40) function"]
        pub fn router(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 135, 234, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slippage` (0x3e032a3b) function"]
        pub fn slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactPtForYt` (0xa1464f13) function"]
        pub fn swap_exact_pt_for_yt(
            &self,
            market: ethers::core::types::Address,
            exact_pt_in: ethers::core::types::U256,
            min_yt_out: ethers::core::types::U256,
            guess_total_yt_to_swap: ApproxParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [161, 70, 79, 19],
                    (market, exact_pt_in, min_yt_out, guess_total_yt_to_swap),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `swapExactYtForPt` (0x4f5eeadf) function"]
        pub fn swap_exact_yt_for_pt(
            &self,
            market: ethers::core::types::Address,
            exact_yt_in: ethers::core::types::U256,
            min_pt_out: ethers::core::types::U256,
            guess_total_pt_from_swap: ApproxParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [79, 94, 234, 223],
                    (market, exact_yt_in, min_pt_out, guess_total_pt_from_swap),
                )
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
    #[doc = "Container type for all input parameters for the `addLiquidityDualSyAndPt`function with signature `addLiquidityDualSyAndPt(address,uint256,uint256,uint256)` and selector `[49, 136, 164, 141]`"]
    #[derive(
        Clone,
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
        name = "addLiquidityDualSyAndPt",
        abi = "addLiquidityDualSyAndPt(address,uint256,uint256,uint256)"
    )]
    pub struct AddLiquidityDualSyAndPtCall {
        pub market: ethers::core::types::Address,
        pub net_sy_desired: ethers::core::types::U256,
        pub net_pt_desired: ethers::core::types::U256,
        pub min_lp_out: ethers::core::types::U256,
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
    pub struct AssetOfCall(pub ethers::core::types::Bytes);
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
    pub struct BalanceOfCall(pub ethers::core::types::Bytes);
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
    #[doc = "Container type for all input parameters for the `marketFactory`function with signature `marketFactory()` and selector `[6, 174, 112, 149]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "marketFactory", abi = "marketFactory()")]
    pub struct MarketFactoryCall;
    #[doc = "Container type for all input parameters for the `mintPyFromSy`function with signature `mintPyFromSy(address,uint256,uint256)` and selector `[85, 69, 108, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "mintPyFromSy", abi = "mintPyFromSy(address,uint256,uint256)")]
    pub struct MintPyFromSyCall {
        pub market: ethers::core::types::Address,
        pub net_sy_in: ethers::core::types::U256,
        pub min_py_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mintSyFromToken`function with signature `mintSyFromToken(address,uint256,(address,uint256,address,address,(uint8,address,bytes,bool)))` and selector `[102, 199, 77, 51]`"]
    #[derive(
        Clone,
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
        name = "mintSyFromToken",
        abi = "mintSyFromToken(address,uint256,(address,uint256,address,address,(uint8,address,bytes,bool)))"
    )]
    pub struct MintSyFromTokenCall {
        pub market: ethers::core::types::Address,
        pub min_sy_out: ethers::core::types::U256,
        pub input: TokenInput,
    }
    #[doc = "Container type for all input parameters for the `redeemPyToSy`function with signature `redeemPyToSy(address,uint256,uint256)` and selector `[94, 70, 205, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "redeemPyToSy", abi = "redeemPyToSy(address,uint256,uint256)")]
    pub struct RedeemPyToSyCall {
        pub market: ethers::core::types::Address,
        pub net_py_in: ethers::core::types::U256,
        pub min_sy_out: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `redeemSyToToken`function with signature `redeemSyToToken(address,uint256,(address,uint256,address,address,(uint8,address,bytes,bool)))` and selector `[48, 61, 40, 250]`"]
    #[derive(
        Clone,
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
        name = "redeemSyToToken",
        abi = "redeemSyToToken(address,uint256,(address,uint256,address,address,(uint8,address,bytes,bool)))"
    )]
    pub struct RedeemSyToTokenCall {
        pub market: ethers::core::types::Address,
        pub net_sy_in: ethers::core::types::U256,
        pub output: TokenOutput,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityDualSyAndPt`function with signature `removeLiquidityDualSyAndPt(address,uint256,uint256,uint256)` and selector `[42, 155, 53, 104]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidityDualSyAndPt",
        abi = "removeLiquidityDualSyAndPt(address,uint256,uint256,uint256)"
    )]
    pub struct RemoveLiquidityDualSyAndPtCall {
        pub market: ethers::core::types::Address,
        pub net_lp_to_remove: ethers::core::types::U256,
        pub min_sy_out: ethers::core::types::U256,
        pub min_pt_out: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `router`function with signature `router()` and selector `[248, 135, 234, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "router", abi = "router()")]
    pub struct RouterCall;
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
    #[doc = "Container type for all input parameters for the `swapExactPtForYt`function with signature `swapExactPtForYt(address,uint256,uint256,(uint256,uint256,uint256,uint256,uint256))` and selector `[161, 70, 79, 19]`"]
    #[derive(
        Clone,
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
        name = "swapExactPtForYt",
        abi = "swapExactPtForYt(address,uint256,uint256,(uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SwapExactPtForYtCall {
        pub market: ethers::core::types::Address,
        pub exact_pt_in: ethers::core::types::U256,
        pub min_yt_out: ethers::core::types::U256,
        pub guess_total_yt_to_swap: ApproxParams,
    }
    #[doc = "Container type for all input parameters for the `swapExactYtForPt`function with signature `swapExactYtForPt(address,uint256,uint256,(uint256,uint256,uint256,uint256,uint256))` and selector `[79, 94, 234, 223]`"]
    #[derive(
        Clone,
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
        name = "swapExactYtForPt",
        abi = "swapExactYtForPt(address,uint256,uint256,(uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct SwapExactYtForPtCall {
        pub market: ethers::core::types::Address,
        pub exact_yt_in: ethers::core::types::U256,
        pub min_pt_out: ethers::core::types::U256,
        pub guess_total_pt_from_swap: ApproxParams,
    }
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
    pub enum PendleAdaptorV1Calls {
        AddLiquidityDualSyAndPt(AddLiquidityDualSyAndPtCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        MarketFactory(MarketFactoryCall),
        MintPyFromSy(MintPyFromSyCall),
        MintSyFromToken(MintSyFromTokenCall),
        RedeemPyToSy(RedeemPyToSyCall),
        RedeemSyToToken(RedeemSyToTokenCall),
        RemoveLiquidityDualSyAndPt(RemoveLiquidityDualSyAndPtCall),
        RevokeApproval(RevokeApprovalCall),
        Router(RouterCall),
        Slippage(SlippageCall),
        SwapExactPtForYt(SwapExactPtForYtCall),
        SwapExactYtForPt(SwapExactYtForPtCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for PendleAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddLiquidityDualSyAndPtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::AddLiquidityDualSyAndPt(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <MarketFactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::MarketFactory(decoded));
            }
            if let Ok(decoded) =
                <MintPyFromSyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::MintPyFromSy(decoded));
            }
            if let Ok(decoded) =
                <MintSyFromTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::MintSyFromToken(decoded));
            }
            if let Ok(decoded) =
                <RedeemPyToSyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::RedeemPyToSy(decoded));
            }
            if let Ok(decoded) =
                <RedeemSyToTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::RedeemSyToToken(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityDualSyAndPtCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PendleAdaptorV1Calls::RemoveLiquidityDualSyAndPt(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <RouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::Router(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) =
                <SwapExactPtForYtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::SwapExactPtForYt(decoded));
            }
            if let Ok(decoded) =
                <SwapExactYtForPtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::SwapExactYtForPt(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PendleAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PendleAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                PendleAdaptorV1Calls::AddLiquidityDualSyAndPt(element) => element.encode(),
                PendleAdaptorV1Calls::AssetOf(element) => element.encode(),
                PendleAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                PendleAdaptorV1Calls::BalanceOf(element) => element.encode(),
                PendleAdaptorV1Calls::Deposit(element) => element.encode(),
                PendleAdaptorV1Calls::Identifier(element) => element.encode(),
                PendleAdaptorV1Calls::IsDebt(element) => element.encode(),
                PendleAdaptorV1Calls::MarketFactory(element) => element.encode(),
                PendleAdaptorV1Calls::MintPyFromSy(element) => element.encode(),
                PendleAdaptorV1Calls::MintSyFromToken(element) => element.encode(),
                PendleAdaptorV1Calls::RedeemPyToSy(element) => element.encode(),
                PendleAdaptorV1Calls::RedeemSyToToken(element) => element.encode(),
                PendleAdaptorV1Calls::RemoveLiquidityDualSyAndPt(element) => element.encode(),
                PendleAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                PendleAdaptorV1Calls::Router(element) => element.encode(),
                PendleAdaptorV1Calls::Slippage(element) => element.encode(),
                PendleAdaptorV1Calls::SwapExactPtForYt(element) => element.encode(),
                PendleAdaptorV1Calls::SwapExactYtForPt(element) => element.encode(),
                PendleAdaptorV1Calls::Withdraw(element) => element.encode(),
                PendleAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PendleAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PendleAdaptorV1Calls::AddLiquidityDualSyAndPt(element) => element.fmt(f),
                PendleAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                PendleAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                PendleAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                PendleAdaptorV1Calls::Deposit(element) => element.fmt(f),
                PendleAdaptorV1Calls::Identifier(element) => element.fmt(f),
                PendleAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                PendleAdaptorV1Calls::MarketFactory(element) => element.fmt(f),
                PendleAdaptorV1Calls::MintPyFromSy(element) => element.fmt(f),
                PendleAdaptorV1Calls::MintSyFromToken(element) => element.fmt(f),
                PendleAdaptorV1Calls::RedeemPyToSy(element) => element.fmt(f),
                PendleAdaptorV1Calls::RedeemSyToToken(element) => element.fmt(f),
                PendleAdaptorV1Calls::RemoveLiquidityDualSyAndPt(element) => element.fmt(f),
                PendleAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                PendleAdaptorV1Calls::Router(element) => element.fmt(f),
                PendleAdaptorV1Calls::Slippage(element) => element.fmt(f),
                PendleAdaptorV1Calls::SwapExactPtForYt(element) => element.fmt(f),
                PendleAdaptorV1Calls::SwapExactYtForPt(element) => element.fmt(f),
                PendleAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                PendleAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddLiquidityDualSyAndPtCall> for PendleAdaptorV1Calls {
        fn from(var: AddLiquidityDualSyAndPtCall) -> Self {
            PendleAdaptorV1Calls::AddLiquidityDualSyAndPt(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for PendleAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            PendleAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for PendleAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            PendleAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for PendleAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            PendleAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DepositCall> for PendleAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            PendleAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for PendleAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            PendleAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for PendleAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            PendleAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<MarketFactoryCall> for PendleAdaptorV1Calls {
        fn from(var: MarketFactoryCall) -> Self {
            PendleAdaptorV1Calls::MarketFactory(var)
        }
    }
    impl ::std::convert::From<MintPyFromSyCall> for PendleAdaptorV1Calls {
        fn from(var: MintPyFromSyCall) -> Self {
            PendleAdaptorV1Calls::MintPyFromSy(var)
        }
    }
    impl ::std::convert::From<MintSyFromTokenCall> for PendleAdaptorV1Calls {
        fn from(var: MintSyFromTokenCall) -> Self {
            PendleAdaptorV1Calls::MintSyFromToken(var)
        }
    }
    impl ::std::convert::From<RedeemPyToSyCall> for PendleAdaptorV1Calls {
        fn from(var: RedeemPyToSyCall) -> Self {
            PendleAdaptorV1Calls::RedeemPyToSy(var)
        }
    }
    impl ::std::convert::From<RedeemSyToTokenCall> for PendleAdaptorV1Calls {
        fn from(var: RedeemSyToTokenCall) -> Self {
            PendleAdaptorV1Calls::RedeemSyToToken(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityDualSyAndPtCall> for PendleAdaptorV1Calls {
        fn from(var: RemoveLiquidityDualSyAndPtCall) -> Self {
            PendleAdaptorV1Calls::RemoveLiquidityDualSyAndPt(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for PendleAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            PendleAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<RouterCall> for PendleAdaptorV1Calls {
        fn from(var: RouterCall) -> Self {
            PendleAdaptorV1Calls::Router(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for PendleAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            PendleAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<SwapExactPtForYtCall> for PendleAdaptorV1Calls {
        fn from(var: SwapExactPtForYtCall) -> Self {
            PendleAdaptorV1Calls::SwapExactPtForYt(var)
        }
    }
    impl ::std::convert::From<SwapExactYtForPtCall> for PendleAdaptorV1Calls {
        fn from(var: SwapExactYtForPtCall) -> Self {
            PendleAdaptorV1Calls::SwapExactYtForPt(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for PendleAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            PendleAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for PendleAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            PendleAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
    #[doc = "`ApproxParams(uint256,uint256,uint256,uint256,uint256)`"]
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
    pub struct ApproxParams {
        pub guess_min: ethers::core::types::U256,
        pub guess_max: ethers::core::types::U256,
        pub guess_offchain: ethers::core::types::U256,
        pub max_iteration: ethers::core::types::U256,
        pub eps: ethers::core::types::U256,
    }
    #[doc = "`SwapData(uint8,address,bytes,bool)`"]
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
    pub struct SwapData {
        pub swap_type: u8,
        pub ext_router: ethers::core::types::Address,
        pub ext_calldata: ethers::core::types::Bytes,
        pub need_scale: bool,
    }
    #[doc = "`TokenInput(address,uint256,address,address,(uint8,address,bytes,bool))`"]
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
    pub struct TokenInput {
        pub token_in: ethers::core::types::Address,
        pub net_token_in: ethers::core::types::U256,
        pub token_mint_sy: ethers::core::types::Address,
        pub pendle_swap: ethers::core::types::Address,
        pub swap_data: SwapData,
    }
    #[doc = "`TokenOutput(address,uint256,address,address,(uint8,address,bytes,bool))`"]
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
    pub struct TokenOutput {
        pub token_out: ethers::core::types::Address,
        pub min_token_out: ethers::core::types::U256,
        pub token_redeem_sy: ethers::core::types::Address,
        pub pendle_swap: ethers::core::types::Address,
        pub swap_data: SwapData,
    }
}
