pub use convexcurveadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod convexcurveadaptorv1_mod {
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
    #[doc = "ConvexCurveAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONVEXCURVEADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_booster\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_nativeWrapper\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ConstructorHealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"BaseAdaptor__PricingNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__Slippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"pid\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"baseRewardPool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpt\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract CurvePool\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"ConvexAdaptor__ConvexBoosterPositionsDoesNotMatchAdaptorData\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"pid\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"baseRewardPool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpt\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract CurvePool\",\n                \"name\": \"_curvePool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"_selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"ConvexAdaptor__ConvexBoosterPositionsMustBeTracked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___CallerDoesNotUseOracle\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___CallerMustImplementDecimals\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___MismatchedLengths\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___NativeAssetRepeated\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___PoolHasMoreTokensThanExpected\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___PoolInReenteredState\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___Reentrancy\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___StorageSlotNotInitialized\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___ZeroTransferAmount\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CURVE_ETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"underlyingTokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedUnderlyingTokenAmounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minLPAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"addLiquidityETHViaProxy\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenDeltaBalance\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"booster\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract IBooster\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_pid\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_baseRewardPool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"_lpt\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract CurvePool\",\n                \"name\": \"_pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"_selector\",\n                \"type\": \"bytes4\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"depositLPTInConvexAndStake\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_baseRewardPool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_claimExtras\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"getRewards\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"lockedStoragePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"nativeWrapper\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"underlyingTokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedMinimumUnderlyingTokenAmountsOut\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"removeLiquidityETHViaProxy\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"balanceDelta\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"slippage\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_baseRewardPool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"_amount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"_claim\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"withdrawFromBaseRewardPoolAsLPT\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"stateMutability\": \"payable\",\n        \"type\": \"receive\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct ConvexCurveAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for ConvexCurveAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConvexCurveAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConvexCurveAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> ConvexCurveAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                CONVEXCURVEADAPTORV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `CURVE_ETH` (0xf258631c) function"]
        pub fn curve_eth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([242, 88, 99, 28], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETHViaProxy` (0x17526554) function"]
        pub fn add_liquidity_eth_via_proxy(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            underlying_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
            min_lp_amount: ethers::core::types::U256,
            use_underlying: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [23, 82, 101, 84],
                    (
                        pool,
                        lp_token,
                        underlying_tokens,
                        ordered_underlying_token_amounts,
                        min_lp_amount,
                        use_underlying,
                    ),
                )
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
        #[doc = "Calls the contract's `booster` (0xc6def076) function"]
        pub fn booster(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([198, 222, 240, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0x69445c31) function"]
        pub fn deposit(
            &self,
            assets: ethers::core::types::U256,
            adaptor_data: ethers::core::types::Bytes,
            p2: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 68, 92, 49], (assets, adaptor_data, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `depositLPTInConvexAndStake` (0xe0e29b76) function"]
        pub fn deposit_lpt_in_convex_and_stake(
            &self,
            pid: ethers::core::types::U256,
            base_reward_pool: ethers::core::types::Address,
            lpt: ethers::core::types::Address,
            pool: ethers::core::types::Address,
            selector: [u8; 4],
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 226, 155, 118],
                    (pid, base_reward_pool, lpt, pool, selector, amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRewards` (0x21201e93) function"]
        pub fn get_rewards(
            &self,
            base_reward_pool: ethers::core::types::Address,
            claim_extras: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 32, 30, 147], (base_reward_pool, claim_extras))
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
        #[doc = "Calls the contract's `lockedStoragePosition` (0x7ba3410c) function"]
        pub fn locked_storage_position(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([123, 163, 65, 12], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nativeWrapper` (0x0b48a8b8) function"]
        pub fn native_wrapper(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([11, 72, 168, 184], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETHViaProxy` (0xc316964d) function"]
        pub fn remove_liquidity_eth_via_proxy(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            lp_token_amount: ethers::core::types::U256,
            underlying_tokens: ::std::vec::Vec<ethers::core::types::Address>,
            ordered_minimum_underlying_token_amounts_out: ::std::vec::Vec<
                ethers::core::types::U256,
            >,
            use_underlying: bool,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash(
                    [195, 22, 150, 77],
                    (
                        pool,
                        lp_token,
                        lp_token_amount,
                        underlying_tokens,
                        ordered_minimum_underlying_token_amounts_out,
                        use_underlying,
                    ),
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
        #[doc = "Calls the contract's `slippage` (0x3e032a3b) function"]
        pub fn slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xc9111bd7) function"]
        pub fn withdraw(
            &self,
            amount: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            adaptor_data: ethers::core::types::Bytes,
            configuration_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (amount, receiver, adaptor_data, configuration_data),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromBaseRewardPoolAsLPT` (0xe7487fc3) function"]
        pub fn withdraw_from_base_reward_pool_as_lpt(
            &self,
            base_reward_pool: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            claim: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([231, 72, 127, 195], (base_reward_pool, amount, claim))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawableFrom` (0xfa50e5d2) function"]
        pub fn withdrawable_from(
            &self,
            adaptor_data: ethers::core::types::Bytes,
            configuration_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (adaptor_data, configuration_data))
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `CURVE_ETH`function with signature `CURVE_ETH()` and selector `[242, 88, 99, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "CURVE_ETH", abi = "CURVE_ETH()")]
    pub struct CurveEthCall;
    #[doc = "Container type for all input parameters for the `addLiquidityETHViaProxy`function with signature `addLiquidityETHViaProxy(address,address,address[],uint256[],uint256,bool)` and selector `[23, 82, 101, 84]`"]
    #[derive(
        Clone,
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
        name = "addLiquidityETHViaProxy",
        abi = "addLiquidityETHViaProxy(address,address,address[],uint256[],uint256,bool)"
    )]
    pub struct AddLiquidityETHViaProxyCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub underlying_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub min_lp_amount: ethers::core::types::U256,
        pub use_underlying: bool,
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
    #[doc = "Container type for all input parameters for the `booster`function with signature `booster()` and selector `[198, 222, 240, 118]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "booster", abi = "booster()")]
    pub struct BoosterCall;
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
    pub struct DepositCall {
        pub assets: ethers::core::types::U256,
        pub adaptor_data: ethers::core::types::Bytes,
        pub p2: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `depositLPTInConvexAndStake`function with signature `depositLPTInConvexAndStake(uint256,address,address,address,bytes4,uint256)` and selector `[224, 226, 155, 118]`"]
    #[derive(
        Clone,
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
        name = "depositLPTInConvexAndStake",
        abi = "depositLPTInConvexAndStake(uint256,address,address,address,bytes4,uint256)"
    )]
    pub struct DepositLPTInConvexAndStakeCall {
        pub pid: ethers::core::types::U256,
        pub base_reward_pool: ethers::core::types::Address,
        pub lpt: ethers::core::types::Address,
        pub pool: ethers::core::types::Address,
        pub selector: [u8; 4],
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getRewards`function with signature `getRewards(address,bool)` and selector `[33, 32, 30, 147]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "getRewards", abi = "getRewards(address,bool)")]
    pub struct GetRewardsCall {
        pub base_reward_pool: ethers::core::types::Address,
        pub claim_extras: bool,
    }
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
    #[doc = "Container type for all input parameters for the `lockedStoragePosition`function with signature `lockedStoragePosition()` and selector `[123, 163, 65, 12]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "lockedStoragePosition", abi = "lockedStoragePosition()")]
    pub struct LockedStoragePositionCall;
    #[doc = "Container type for all input parameters for the `nativeWrapper`function with signature `nativeWrapper()` and selector `[11, 72, 168, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "nativeWrapper", abi = "nativeWrapper()")]
    pub struct NativeWrapperCall;
    #[doc = "Container type for all input parameters for the `removeLiquidityETHViaProxy`function with signature `removeLiquidityETHViaProxy(address,address,uint256,address[],uint256[],bool)` and selector `[195, 22, 150, 77]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidityETHViaProxy",
        abi = "removeLiquidityETHViaProxy(address,address,uint256,address[],uint256[],bool)"
    )]
    pub struct RemoveLiquidityETHViaProxyCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub lp_token_amount: ethers::core::types::U256,
        pub underlying_tokens: ::std::vec::Vec<ethers::core::types::Address>,
        pub ordered_minimum_underlying_token_amounts_out:
            ::std::vec::Vec<ethers::core::types::U256>,
        pub use_underlying: bool,
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
    pub struct WithdrawCall {
        pub amount: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub adaptor_data: ethers::core::types::Bytes,
        pub configuration_data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `withdrawFromBaseRewardPoolAsLPT`function with signature `withdrawFromBaseRewardPoolAsLPT(address,uint256,bool)` and selector `[231, 72, 127, 195]`"]
    #[derive(
        Clone,
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
        name = "withdrawFromBaseRewardPoolAsLPT",
        abi = "withdrawFromBaseRewardPoolAsLPT(address,uint256,bool)"
    )]
    pub struct WithdrawFromBaseRewardPoolAsLPTCall {
        pub base_reward_pool: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub claim: bool,
    }
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
    pub struct WithdrawableFromCall {
        pub adaptor_data: ethers::core::types::Bytes,
        pub configuration_data: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConvexCurveAdaptorV1Calls {
        CurveEth(CurveEthCall),
        AddLiquidityETHViaProxy(AddLiquidityETHViaProxyCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Booster(BoosterCall),
        Deposit(DepositCall),
        DepositLPTInConvexAndStake(DepositLPTInConvexAndStakeCall),
        GetRewards(GetRewardsCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        LockedStoragePosition(LockedStoragePositionCall),
        NativeWrapper(NativeWrapperCall),
        RemoveLiquidityETHViaProxy(RemoveLiquidityETHViaProxyCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawFromBaseRewardPoolAsLPT(WithdrawFromBaseRewardPoolAsLPTCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for ConvexCurveAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurveEthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::CurveEth(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHViaProxyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::AddLiquidityETHViaProxy(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BoosterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::Booster(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositLPTInConvexAndStakeCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConvexCurveAdaptorV1Calls::DepositLPTInConvexAndStake(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <GetRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::GetRewards(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <LockedStoragePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::LockedStoragePosition(decoded));
            }
            if let Ok(decoded) =
                <NativeWrapperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::NativeWrapper(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHViaProxyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConvexCurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFromBaseRewardPoolAsLPTCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ConvexCurveAdaptorV1Calls::WithdrawFromBaseRewardPoolAsLPT(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConvexCurveAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConvexCurveAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConvexCurveAdaptorV1Calls::CurveEth(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::AddLiquidityETHViaProxy(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::AssetOf(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::BalanceOf(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::Booster(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::Deposit(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::DepositLPTInConvexAndStake(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::GetRewards(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::Identifier(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::IsDebt(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::LockedStoragePosition(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::NativeWrapper(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::Slippage(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::Withdraw(element) => element.encode(),
                ConvexCurveAdaptorV1Calls::WithdrawFromBaseRewardPoolAsLPT(element) => {
                    element.encode()
                }
                ConvexCurveAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConvexCurveAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConvexCurveAdaptorV1Calls::CurveEth(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::AddLiquidityETHViaProxy(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::Booster(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::Deposit(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::DepositLPTInConvexAndStake(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::GetRewards(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::Identifier(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::LockedStoragePosition(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::NativeWrapper(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::Slippage(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                ConvexCurveAdaptorV1Calls::WithdrawFromBaseRewardPoolAsLPT(element) => {
                    element.fmt(f)
                }
                ConvexCurveAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurveEthCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: CurveEthCall) -> Self {
            ConvexCurveAdaptorV1Calls::CurveEth(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHViaProxyCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: AddLiquidityETHViaProxyCall) -> Self {
            ConvexCurveAdaptorV1Calls::AddLiquidityETHViaProxy(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            ConvexCurveAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            ConvexCurveAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            ConvexCurveAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BoosterCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: BoosterCall) -> Self {
            ConvexCurveAdaptorV1Calls::Booster(var)
        }
    }
    impl ::std::convert::From<DepositCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            ConvexCurveAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositLPTInConvexAndStakeCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: DepositLPTInConvexAndStakeCall) -> Self {
            ConvexCurveAdaptorV1Calls::DepositLPTInConvexAndStake(var)
        }
    }
    impl ::std::convert::From<GetRewardsCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: GetRewardsCall) -> Self {
            ConvexCurveAdaptorV1Calls::GetRewards(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            ConvexCurveAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            ConvexCurveAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<LockedStoragePositionCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: LockedStoragePositionCall) -> Self {
            ConvexCurveAdaptorV1Calls::LockedStoragePosition(var)
        }
    }
    impl ::std::convert::From<NativeWrapperCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: NativeWrapperCall) -> Self {
            ConvexCurveAdaptorV1Calls::NativeWrapper(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHViaProxyCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: RemoveLiquidityETHViaProxyCall) -> Self {
            ConvexCurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            ConvexCurveAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            ConvexCurveAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            ConvexCurveAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawFromBaseRewardPoolAsLPTCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: WithdrawFromBaseRewardPoolAsLPTCall) -> Self {
            ConvexCurveAdaptorV1Calls::WithdrawFromBaseRewardPoolAsLPT(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for ConvexCurveAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            ConvexCurveAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
}
