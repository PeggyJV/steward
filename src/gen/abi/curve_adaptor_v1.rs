pub use curveadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod curveadaptorv1_mod {
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
    #[doc = "CurveAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CURVEADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"_nativeWrapper\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"_curveSlippage\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"constructor\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ConstructorHealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"BaseAdaptor__PricingNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__Slippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"positionId\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"name\": \"CurveAdaptor__CurvePositionNotUsed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveAdaptor___InvalidConstructorSlippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveAdaptor___NonStandardDecimals\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveAdaptor___Slippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___CallerDoesNotUseOracle\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___CallerMustImplementDecimals\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___MismatchedLengths\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___NativeAssetRepeated\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___PoolHasMoreTokensThanExpected\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___PoolInReenteredState\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___Reentrancy\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___StorageSlotNotInitialized\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CurveHelper___ZeroTransferAmount\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"CURVE_ETH\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"adaptorAddress\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address payable\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedUnderlyingTokenAmounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minLPAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"addLiquidity\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedUnderlyingTokenAmounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minLPAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"addLiquidityETH\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"underlyingTokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedUnderlyingTokenAmounts\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"minLPAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"addLiquidityETHViaProxy\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenDeltaBalance\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"balance\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"claimRewards\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"curveSlippage\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"lockedStoragePosition\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"nativeWrapper\",\n        \"outputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedMinimumUnderlyingTokenAmountsOut\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"removeLiquidity\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedMinimumUnderlyingTokenAmountsOut\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            },\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"removeLiquidityETH\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"lpTokenAmount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"underlyingTokens\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"orderedMinimumUnderlyingTokenAmountsOut\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"useUnderlying\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"removeLiquidityETHViaProxy\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"balanceDelta\",\n                \"type\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"slippage\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint32\",\n                \"name\": \"\",\n                \"type\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"lpToken\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract CurvePool\",\n                \"name\": \"pool\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes4\",\n                \"name\": \"selector\",\n                \"type\": \"bytes4\"\n            }\n        ],\n        \"name\": \"stakeInGauge\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract CurveGauge\",\n                \"name\": \"gauge\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"unStakeFromGauge\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"configurationData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"stateMutability\": \"payable\",\n        \"type\": \"receive\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct CurveAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for CurveAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for CurveAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CurveAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> CurveAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), CURVEADAPTORV1_ABI.clone(), client);
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
        #[doc = "Calls the contract's `adaptorAddress` (0x8c419161) function"]
        pub fn adaptor_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([140, 65, 145, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidity` (0xccad65a7) function"]
        pub fn add_liquidity(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
            min_lp_amount: ethers::core::types::U256,
            gauge: ethers::core::types::Address,
            selector: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [204, 173, 101, 167],
                    (
                        pool,
                        lp_token,
                        ordered_underlying_token_amounts,
                        min_lp_amount,
                        gauge,
                        selector,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityETH` (0xfebd9e35) function"]
        pub fn add_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
            min_lp_amount: ethers::core::types::U256,
            use_underlying: bool,
            gauge: ethers::core::types::Address,
            selector: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [254, 189, 158, 53],
                    (
                        pool,
                        lp_token,
                        ordered_underlying_token_amounts,
                        min_lp_amount,
                        use_underlying,
                        gauge,
                        selector,
                    ),
                )
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
        #[doc = "Calls the contract's `claimRewards` (0xef5cfb8c) function"]
        pub fn claim_rewards(
            &self,
            gauge: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 92, 251, 140], gauge)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `curveSlippage` (0x830ada80) function"]
        pub fn curve_slippage(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([131, 10, 218, 128], ())
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
        #[doc = "Calls the contract's `removeLiquidity` (0x6be61ed1) function"]
        pub fn remove_liquidity(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            lp_token_amount: ethers::core::types::U256,
            ordered_minimum_underlying_token_amounts_out: ::std::vec::Vec<
                ethers::core::types::U256,
            >,
            gauge: ethers::core::types::Address,
            selector: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [107, 230, 30, 209],
                    (
                        pool,
                        lp_token,
                        lp_token_amount,
                        ordered_minimum_underlying_token_amounts_out,
                        gauge,
                        selector,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityETH` (0x73c70457) function"]
        pub fn remove_liquidity_eth(
            &self,
            pool: ethers::core::types::Address,
            lp_token: ethers::core::types::Address,
            lp_token_amount: ethers::core::types::U256,
            ordered_minimum_underlying_token_amounts_out: ::std::vec::Vec<
                ethers::core::types::U256,
            >,
            use_underlying: bool,
            gauge: ethers::core::types::Address,
            selector: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [115, 199, 4, 87],
                    (
                        pool,
                        lp_token,
                        lp_token_amount,
                        ordered_minimum_underlying_token_amounts_out,
                        use_underlying,
                        gauge,
                        selector,
                    ),
                )
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
        #[doc = "Calls the contract's `stakeInGauge` (0x81a12a09) function"]
        pub fn stake_in_gauge(
            &self,
            lp_token: ethers::core::types::Address,
            gauge: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            pool: ethers::core::types::Address,
            selector: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 161, 42, 9], (lp_token, gauge, amount, pool, selector))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unStakeFromGauge` (0x9eff5191) function"]
        pub fn un_stake_from_gauge(
            &self,
            gauge: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([158, 255, 81, 145], (gauge, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xc9111bd7) function"]
        pub fn withdraw(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            adaptor_data: ethers::core::types::Bytes,
            configuration_data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (assets, receiver, adaptor_data, configuration_data),
                )
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
    #[doc = "Container type for all input parameters for the `adaptorAddress`function with signature `adaptorAddress()` and selector `[140, 65, 145, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "adaptorAddress", abi = "adaptorAddress()")]
    pub struct AdaptorAddressCall;
    #[doc = "Container type for all input parameters for the `addLiquidity`function with signature `addLiquidity(address,address,uint256[],uint256,address,bytes4)` and selector `[204, 173, 101, 167]`"]
    #[derive(
        Clone,
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
        name = "addLiquidity",
        abi = "addLiquidity(address,address,uint256[],uint256,address,bytes4)"
    )]
    pub struct AddLiquidityCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub min_lp_amount: ethers::core::types::U256,
        pub gauge: ethers::core::types::Address,
        pub selector: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `addLiquidityETH`function with signature `addLiquidityETH(address,address,uint256[],uint256,bool,address,bytes4)` and selector `[254, 189, 158, 53]`"]
    #[derive(
        Clone,
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
        name = "addLiquidityETH",
        abi = "addLiquidityETH(address,address,uint256[],uint256,bool,address,bytes4)"
    )]
    pub struct AddLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub ordered_underlying_token_amounts: ::std::vec::Vec<ethers::core::types::U256>,
        pub min_lp_amount: ethers::core::types::U256,
        pub use_underlying: bool,
        pub gauge: ethers::core::types::Address,
        pub selector: [u8; 4],
    }
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
    #[doc = "Container type for all input parameters for the `claimRewards`function with signature `claimRewards(address)` and selector `[239, 92, 251, 140]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(address)")]
    pub struct ClaimRewardsCall {
        pub gauge: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `curveSlippage`function with signature `curveSlippage()` and selector `[131, 10, 218, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "curveSlippage", abi = "curveSlippage()")]
    pub struct CurveSlippageCall;
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
    #[doc = "Container type for all input parameters for the `removeLiquidity`function with signature `removeLiquidity(address,address,uint256,uint256[],address,bytes4)` and selector `[107, 230, 30, 209]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidity",
        abi = "removeLiquidity(address,address,uint256,uint256[],address,bytes4)"
    )]
    pub struct RemoveLiquidityCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub lp_token_amount: ethers::core::types::U256,
        pub ordered_minimum_underlying_token_amounts_out:
            ::std::vec::Vec<ethers::core::types::U256>,
        pub gauge: ethers::core::types::Address,
        pub selector: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityETH`function with signature `removeLiquidityETH(address,address,uint256,uint256[],bool,address,bytes4)` and selector `[115, 199, 4, 87]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidityETH",
        abi = "removeLiquidityETH(address,address,uint256,uint256[],bool,address,bytes4)"
    )]
    pub struct RemoveLiquidityETHCall {
        pub pool: ethers::core::types::Address,
        pub lp_token: ethers::core::types::Address,
        pub lp_token_amount: ethers::core::types::U256,
        pub ordered_minimum_underlying_token_amounts_out:
            ::std::vec::Vec<ethers::core::types::U256>,
        pub use_underlying: bool,
        pub gauge: ethers::core::types::Address,
        pub selector: [u8; 4],
    }
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
    #[doc = "Container type for all input parameters for the `stakeInGauge`function with signature `stakeInGauge(address,address,uint256,address,bytes4)` and selector `[129, 161, 42, 9]`"]
    #[derive(
        Clone,
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
        name = "stakeInGauge",
        abi = "stakeInGauge(address,address,uint256,address,bytes4)"
    )]
    pub struct StakeInGaugeCall {
        pub lp_token: ethers::core::types::Address,
        pub gauge: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub pool: ethers::core::types::Address,
        pub selector: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `unStakeFromGauge`function with signature `unStakeFromGauge(address,uint256)` and selector `[158, 255, 81, 145]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "unStakeFromGauge", abi = "unStakeFromGauge(address,uint256)")]
    pub struct UnStakeFromGaugeCall {
        pub gauge: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
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
    pub struct WithdrawCall {
        pub assets: ethers::core::types::U256,
        pub receiver: ethers::core::types::Address,
        pub adaptor_data: ethers::core::types::Bytes,
        pub configuration_data: ethers::core::types::Bytes,
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
    pub enum CurveAdaptorV1Calls {
        CurveEth(CurveEthCall),
        AdaptorAddress(AdaptorAddressCall),
        AddLiquidity(AddLiquidityCall),
        AddLiquidityETH(AddLiquidityETHCall),
        AddLiquidityETHViaProxy(AddLiquidityETHViaProxyCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        ClaimRewards(ClaimRewardsCall),
        CurveSlippage(CurveSlippageCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        LockedStoragePosition(LockedStoragePositionCall),
        NativeWrapper(NativeWrapperCall),
        RemoveLiquidity(RemoveLiquidityCall),
        RemoveLiquidityETH(RemoveLiquidityETHCall),
        RemoveLiquidityETHViaProxy(RemoveLiquidityETHViaProxyCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        StakeInGauge(StakeInGaugeCall),
        UnStakeFromGauge(UnStakeFromGaugeCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for CurveAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CurveEthCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::CurveEth(decoded));
            }
            if let Ok(decoded) =
                <AdaptorAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AdaptorAddress(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AddLiquidity(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AddLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityETHViaProxyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AddLiquidityETHViaProxy(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ClaimRewardsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::ClaimRewards(decoded));
            }
            if let Ok(decoded) =
                <CurveSlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::CurveSlippage(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <LockedStoragePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::LockedStoragePosition(decoded));
            }
            if let Ok(decoded) =
                <NativeWrapperCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::NativeWrapper(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::RemoveLiquidity(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::RemoveLiquidityETH(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityETHViaProxyCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(CurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) =
                <StakeInGaugeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::StakeInGauge(decoded));
            }
            if let Ok(decoded) =
                <UnStakeFromGaugeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::UnStakeFromGauge(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(CurveAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for CurveAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                CurveAdaptorV1Calls::CurveEth(element) => element.encode(),
                CurveAdaptorV1Calls::AdaptorAddress(element) => element.encode(),
                CurveAdaptorV1Calls::AddLiquidity(element) => element.encode(),
                CurveAdaptorV1Calls::AddLiquidityETH(element) => element.encode(),
                CurveAdaptorV1Calls::AddLiquidityETHViaProxy(element) => element.encode(),
                CurveAdaptorV1Calls::AssetOf(element) => element.encode(),
                CurveAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                CurveAdaptorV1Calls::BalanceOf(element) => element.encode(),
                CurveAdaptorV1Calls::ClaimRewards(element) => element.encode(),
                CurveAdaptorV1Calls::CurveSlippage(element) => element.encode(),
                CurveAdaptorV1Calls::Deposit(element) => element.encode(),
                CurveAdaptorV1Calls::Identifier(element) => element.encode(),
                CurveAdaptorV1Calls::IsDebt(element) => element.encode(),
                CurveAdaptorV1Calls::LockedStoragePosition(element) => element.encode(),
                CurveAdaptorV1Calls::NativeWrapper(element) => element.encode(),
                CurveAdaptorV1Calls::RemoveLiquidity(element) => element.encode(),
                CurveAdaptorV1Calls::RemoveLiquidityETH(element) => element.encode(),
                CurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(element) => element.encode(),
                CurveAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                CurveAdaptorV1Calls::Slippage(element) => element.encode(),
                CurveAdaptorV1Calls::StakeInGauge(element) => element.encode(),
                CurveAdaptorV1Calls::UnStakeFromGauge(element) => element.encode(),
                CurveAdaptorV1Calls::Withdraw(element) => element.encode(),
                CurveAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for CurveAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CurveAdaptorV1Calls::CurveEth(element) => element.fmt(f),
                CurveAdaptorV1Calls::AdaptorAddress(element) => element.fmt(f),
                CurveAdaptorV1Calls::AddLiquidity(element) => element.fmt(f),
                CurveAdaptorV1Calls::AddLiquidityETH(element) => element.fmt(f),
                CurveAdaptorV1Calls::AddLiquidityETHViaProxy(element) => element.fmt(f),
                CurveAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                CurveAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                CurveAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                CurveAdaptorV1Calls::ClaimRewards(element) => element.fmt(f),
                CurveAdaptorV1Calls::CurveSlippage(element) => element.fmt(f),
                CurveAdaptorV1Calls::Deposit(element) => element.fmt(f),
                CurveAdaptorV1Calls::Identifier(element) => element.fmt(f),
                CurveAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                CurveAdaptorV1Calls::LockedStoragePosition(element) => element.fmt(f),
                CurveAdaptorV1Calls::NativeWrapper(element) => element.fmt(f),
                CurveAdaptorV1Calls::RemoveLiquidity(element) => element.fmt(f),
                CurveAdaptorV1Calls::RemoveLiquidityETH(element) => element.fmt(f),
                CurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(element) => element.fmt(f),
                CurveAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                CurveAdaptorV1Calls::Slippage(element) => element.fmt(f),
                CurveAdaptorV1Calls::StakeInGauge(element) => element.fmt(f),
                CurveAdaptorV1Calls::UnStakeFromGauge(element) => element.fmt(f),
                CurveAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                CurveAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CurveEthCall> for CurveAdaptorV1Calls {
        fn from(var: CurveEthCall) -> Self {
            CurveAdaptorV1Calls::CurveEth(var)
        }
    }
    impl ::std::convert::From<AdaptorAddressCall> for CurveAdaptorV1Calls {
        fn from(var: AdaptorAddressCall) -> Self {
            CurveAdaptorV1Calls::AdaptorAddress(var)
        }
    }
    impl ::std::convert::From<AddLiquidityCall> for CurveAdaptorV1Calls {
        fn from(var: AddLiquidityCall) -> Self {
            CurveAdaptorV1Calls::AddLiquidity(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHCall> for CurveAdaptorV1Calls {
        fn from(var: AddLiquidityETHCall) -> Self {
            CurveAdaptorV1Calls::AddLiquidityETH(var)
        }
    }
    impl ::std::convert::From<AddLiquidityETHViaProxyCall> for CurveAdaptorV1Calls {
        fn from(var: AddLiquidityETHViaProxyCall) -> Self {
            CurveAdaptorV1Calls::AddLiquidityETHViaProxy(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for CurveAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            CurveAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for CurveAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            CurveAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for CurveAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            CurveAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ClaimRewardsCall> for CurveAdaptorV1Calls {
        fn from(var: ClaimRewardsCall) -> Self {
            CurveAdaptorV1Calls::ClaimRewards(var)
        }
    }
    impl ::std::convert::From<CurveSlippageCall> for CurveAdaptorV1Calls {
        fn from(var: CurveSlippageCall) -> Self {
            CurveAdaptorV1Calls::CurveSlippage(var)
        }
    }
    impl ::std::convert::From<DepositCall> for CurveAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            CurveAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for CurveAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            CurveAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for CurveAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            CurveAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<LockedStoragePositionCall> for CurveAdaptorV1Calls {
        fn from(var: LockedStoragePositionCall) -> Self {
            CurveAdaptorV1Calls::LockedStoragePosition(var)
        }
    }
    impl ::std::convert::From<NativeWrapperCall> for CurveAdaptorV1Calls {
        fn from(var: NativeWrapperCall) -> Self {
            CurveAdaptorV1Calls::NativeWrapper(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityCall> for CurveAdaptorV1Calls {
        fn from(var: RemoveLiquidityCall) -> Self {
            CurveAdaptorV1Calls::RemoveLiquidity(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHCall> for CurveAdaptorV1Calls {
        fn from(var: RemoveLiquidityETHCall) -> Self {
            CurveAdaptorV1Calls::RemoveLiquidityETH(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityETHViaProxyCall> for CurveAdaptorV1Calls {
        fn from(var: RemoveLiquidityETHViaProxyCall) -> Self {
            CurveAdaptorV1Calls::RemoveLiquidityETHViaProxy(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for CurveAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            CurveAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for CurveAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            CurveAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<StakeInGaugeCall> for CurveAdaptorV1Calls {
        fn from(var: StakeInGaugeCall) -> Self {
            CurveAdaptorV1Calls::StakeInGauge(var)
        }
    }
    impl ::std::convert::From<UnStakeFromGaugeCall> for CurveAdaptorV1Calls {
        fn from(var: UnStakeFromGaugeCall) -> Self {
            CurveAdaptorV1Calls::UnStakeFromGauge(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for CurveAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            CurveAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for CurveAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            CurveAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
}
