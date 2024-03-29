pub use uniswapv3adaptorv2_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv3adaptorv2_mod {
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
    #[doc = "UniswapV3AdaptorV2 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV3ADAPTORV2_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__BadSlippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExchangeNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"T\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"UniswapV3Adaptor__NotTheOwner\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"UniswapV3Adaptor__PurgingPositionWithLiquidity\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"UniswapV3Adaptor__TokenIdNotFoundInTracker\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"token1\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"UniswapV3Adaptor__UntrackedLiquidity\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"addToPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min1\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"closePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount0\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"amount1\",\n                \"type\": \"uint128\"\n            }\n        ],\n        \"name\": \"collectFees\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token1\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint24\",\n                \"name\": \"poolFee\",\n                \"type\": \"uint24\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickLower\",\n                \"type\": \"int24\"\n            },\n            {\n                \"internalType\": \"int24\",\n                \"name\": \"tickUpper\",\n                \"type\": \"int24\"\n            }\n        ],\n        \"name\": \"openPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"slippage\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"oracleSwap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token1\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"purgeAllZeroLiquidityPositions\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"purgeSinglePosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token0\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"token1\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"removeUnOwnedPositionFromTracker\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"tokenId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint128\",\n                \"name\": \"liquidity\",\n                \"type\": \"uint128\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min0\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"min1\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"takeFees\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"name\": \"takeFromPosition\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV3AdaptorV2<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV3AdaptorV2<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV3AdaptorV2<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3AdaptorV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV3AdaptorV2<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                UNISWAPV3ADAPTORV2_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `addToPosition` (0x5ef54f36) function"]
        pub fn add_to_position(
            &self,
            token_id: ethers::core::types::U256,
            amount_0: ethers::core::types::U256,
            amount_1: ethers::core::types::U256,
            min_0: ethers::core::types::U256,
            min_1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [94, 245, 79, 54],
                    (token_id, amount_0, amount_1, min_0, min_1),
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
        #[doc = "Calls the contract's `closePosition` (0xb35648d7) function"]
        pub fn close_position(
            &self,
            token_id: ethers::core::types::U256,
            min_0: ethers::core::types::U256,
            min_1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 86, 72, 215], (token_id, min_0, min_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `collectFees` (0xb14d3ec5) function"]
        pub fn collect_fees(
            &self,
            token_id: ethers::core::types::U256,
            amount_0: u128,
            amount_1: u128,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 77, 62, 197], (token_id, amount_0, amount_1))
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
        #[doc = "Calls the contract's `openPosition` (0x48368d0f) function"]
        pub fn open_position(
            &self,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
            pool_fee: u32,
            amount_0: ethers::core::types::U256,
            amount_1: ethers::core::types::U256,
            min_0: ethers::core::types::U256,
            min_1: ethers::core::types::U256,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 54, 141, 15],
                    (
                        token_0, token_1, pool_fee, amount_0, amount_1, min_0, min_1, tick_lower,
                        tick_upper,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `oracleSwap` (0x17d964df) function"]
        pub fn oracle_swap(
            &self,
            asset_in: ethers::core::types::Address,
            asset_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            exchange: u8,
            params: ethers::core::types::Bytes,
            slippage: u64,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [23, 217, 100, 223],
                    (asset_in, asset_out, amount_in, exchange, params, slippage),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `purgeAllZeroLiquidityPositions` (0xcfe93f77) function"]
        pub fn purge_all_zero_liquidity_positions(
            &self,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 233, 63, 119], (token_0, token_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `purgeSinglePosition` (0x24f9f461) function"]
        pub fn purge_single_position(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 249, 244, 97], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeUnOwnedPositionFromTracker` (0xea97960c) function"]
        pub fn remove_un_owned_position_from_tracker(
            &self,
            token_id: ethers::core::types::U256,
            token_0: ethers::core::types::Address,
            token_1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 151, 150, 12], (token_id, token_0, token_1))
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
        #[doc = "Calls the contract's `swap` (0xf2879a8d) function"]
        pub fn swap(
            &self,
            asset_in: ethers::core::types::Address,
            asset_out: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            exchange: u8,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash(
                    [242, 135, 154, 141],
                    (asset_in, asset_out, amount_in, exchange, params),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `takeFromPosition` (0xd496b99c) function"]
        pub fn take_from_position(
            &self,
            token_id: ethers::core::types::U256,
            liquidity: u128,
            min_0: ethers::core::types::U256,
            min_1: ethers::core::types::U256,
            take_fees: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [212, 150, 185, 156],
                    (token_id, liquidity, min_0, min_1, take_fees),
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
    #[doc = "Container type for all input parameters for the `addToPosition`function with signature `addToPosition(uint256,uint256,uint256,uint256,uint256)` and selector `[94, 245, 79, 54]`"]
    #[derive(
        Clone,
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
        name = "addToPosition",
        abi = "addToPosition(uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct AddToPositionCall {
        pub token_id: ethers::core::types::U256,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub min_0: ethers::core::types::U256,
        pub min_1: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `closePosition`function with signature `closePosition(uint256,uint256,uint256)` and selector `[179, 86, 72, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "closePosition", abi = "closePosition(uint256,uint256,uint256)")]
    pub struct ClosePositionCall {
        pub token_id: ethers::core::types::U256,
        pub min_0: ethers::core::types::U256,
        pub min_1: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `collectFees`function with signature `collectFees(uint256,uint128,uint128)` and selector `[177, 77, 62, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "collectFees", abi = "collectFees(uint256,uint128,uint128)")]
    pub struct CollectFeesCall {
        pub token_id: ethers::core::types::U256,
        pub amount_0: u128,
        pub amount_1: u128,
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
    #[doc = "Container type for all input parameters for the `openPosition`function with signature `openPosition(address,address,uint24,uint256,uint256,uint256,uint256,int24,int24)` and selector `[72, 54, 141, 15]`"]
    #[derive(
        Clone,
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
        name = "openPosition",
        abi = "openPosition(address,address,uint24,uint256,uint256,uint256,uint256,int24,int24)"
    )]
    pub struct OpenPositionCall {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
        pub pool_fee: u32,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
        pub min_0: ethers::core::types::U256,
        pub min_1: ethers::core::types::U256,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    #[doc = "Container type for all input parameters for the `oracleSwap`function with signature `oracleSwap(address,address,uint256,uint8,bytes,uint64)` and selector `[23, 217, 100, 223]`"]
    #[derive(
        Clone,
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
        name = "oracleSwap",
        abi = "oracleSwap(address,address,uint256,uint8,bytes,uint64)"
    )]
    pub struct OracleSwapCall {
        pub asset_in: ethers::core::types::Address,
        pub asset_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub exchange: u8,
        pub params: ethers::core::types::Bytes,
        pub slippage: u64,
    }
    #[doc = "Container type for all input parameters for the `purgeAllZeroLiquidityPositions`function with signature `purgeAllZeroLiquidityPositions(address,address)` and selector `[207, 233, 63, 119]`"]
    #[derive(
        Clone,
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
        name = "purgeAllZeroLiquidityPositions",
        abi = "purgeAllZeroLiquidityPositions(address,address)"
    )]
    pub struct PurgeAllZeroLiquidityPositionsCall {
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `purgeSinglePosition`function with signature `purgeSinglePosition(uint256)` and selector `[36, 249, 244, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "purgeSinglePosition", abi = "purgeSinglePosition(uint256)")]
    pub struct PurgeSinglePositionCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `removeUnOwnedPositionFromTracker`function with signature `removeUnOwnedPositionFromTracker(uint256,address,address)` and selector `[234, 151, 150, 12]`"]
    #[derive(
        Clone,
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
        name = "removeUnOwnedPositionFromTracker",
        abi = "removeUnOwnedPositionFromTracker(uint256,address,address)"
    )]
    pub struct RemoveUnOwnedPositionFromTrackerCall {
        pub token_id: ethers::core::types::U256,
        pub token_0: ethers::core::types::Address,
        pub token_1: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `swap`function with signature `swap(address,address,uint256,uint8,bytes)` and selector `[242, 135, 154, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "swap", abi = "swap(address,address,uint256,uint8,bytes)")]
    pub struct SwapCall {
        pub asset_in: ethers::core::types::Address,
        pub asset_out: ethers::core::types::Address,
        pub amount_in: ethers::core::types::U256,
        pub exchange: u8,
        pub params: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `takeFromPosition`function with signature `takeFromPosition(uint256,uint128,uint256,uint256,bool)` and selector `[212, 150, 185, 156]`"]
    #[derive(
        Clone,
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
        name = "takeFromPosition",
        abi = "takeFromPosition(uint256,uint128,uint256,uint256,bool)"
    )]
    pub struct TakeFromPositionCall {
        pub token_id: ethers::core::types::U256,
        pub liquidity: u128,
        pub min_0: ethers::core::types::U256,
        pub min_1: ethers::core::types::U256,
        pub take_fees: bool,
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
    pub enum UniswapV3AdaptorV2Calls {
        AddToPosition(AddToPositionCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        ClosePosition(ClosePositionCall),
        CollectFees(CollectFeesCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        OpenPosition(OpenPositionCall),
        OracleSwap(OracleSwapCall),
        PurgeAllZeroLiquidityPositions(PurgeAllZeroLiquidityPositionsCall),
        PurgeSinglePosition(PurgeSinglePositionCall),
        RemoveUnOwnedPositionFromTracker(RemoveUnOwnedPositionFromTrackerCall),
        RevokeApproval(RevokeApprovalCall),
        Swap(SwapCall),
        TakeFromPosition(TakeFromPositionCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV3AdaptorV2Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddToPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::AddToPosition(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <ClosePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::ClosePosition(decoded));
            }
            if let Ok(decoded) =
                <CollectFeesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::CollectFees(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <OpenPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::OpenPosition(decoded));
            }
            if let Ok(decoded) =
                <OracleSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::OracleSwap(decoded));
            }
            if let Ok(decoded) =
                <PurgeAllZeroLiquidityPositionsCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3AdaptorV2Calls::PurgeAllZeroLiquidityPositions(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <PurgeSinglePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::PurgeSinglePosition(decoded));
            }
            if let Ok(decoded) =
                <RemoveUnOwnedPositionFromTrackerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3AdaptorV2Calls::RemoveUnOwnedPositionFromTracker(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3AdaptorV2Calls::Swap(decoded));
            }
            if let Ok(decoded) =
                <TakeFromPositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::TakeFromPosition(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3AdaptorV2Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV3AdaptorV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3AdaptorV2Calls::AddToPosition(element) => element.encode(),
                UniswapV3AdaptorV2Calls::AssetOf(element) => element.encode(),
                UniswapV3AdaptorV2Calls::AssetsUsed(element) => element.encode(),
                UniswapV3AdaptorV2Calls::BalanceOf(element) => element.encode(),
                UniswapV3AdaptorV2Calls::ClosePosition(element) => element.encode(),
                UniswapV3AdaptorV2Calls::CollectFees(element) => element.encode(),
                UniswapV3AdaptorV2Calls::Deposit(element) => element.encode(),
                UniswapV3AdaptorV2Calls::Identifier(element) => element.encode(),
                UniswapV3AdaptorV2Calls::IsDebt(element) => element.encode(),
                UniswapV3AdaptorV2Calls::OpenPosition(element) => element.encode(),
                UniswapV3AdaptorV2Calls::OracleSwap(element) => element.encode(),
                UniswapV3AdaptorV2Calls::PurgeAllZeroLiquidityPositions(element) => {
                    element.encode()
                }
                UniswapV3AdaptorV2Calls::PurgeSinglePosition(element) => element.encode(),
                UniswapV3AdaptorV2Calls::RemoveUnOwnedPositionFromTracker(element) => {
                    element.encode()
                }
                UniswapV3AdaptorV2Calls::RevokeApproval(element) => element.encode(),
                UniswapV3AdaptorV2Calls::Swap(element) => element.encode(),
                UniswapV3AdaptorV2Calls::TakeFromPosition(element) => element.encode(),
                UniswapV3AdaptorV2Calls::Withdraw(element) => element.encode(),
                UniswapV3AdaptorV2Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3AdaptorV2Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3AdaptorV2Calls::AddToPosition(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::AssetOf(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::AssetsUsed(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::BalanceOf(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::ClosePosition(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::CollectFees(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::Deposit(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::Identifier(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::IsDebt(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::OpenPosition(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::OracleSwap(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::PurgeAllZeroLiquidityPositions(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::PurgeSinglePosition(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::RemoveUnOwnedPositionFromTracker(element) => {
                    element.fmt(f)
                }
                UniswapV3AdaptorV2Calls::RevokeApproval(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::Swap(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::TakeFromPosition(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::Withdraw(element) => element.fmt(f),
                UniswapV3AdaptorV2Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddToPositionCall> for UniswapV3AdaptorV2Calls {
        fn from(var: AddToPositionCall) -> Self {
            UniswapV3AdaptorV2Calls::AddToPosition(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for UniswapV3AdaptorV2Calls {
        fn from(var: AssetOfCall) -> Self {
            UniswapV3AdaptorV2Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for UniswapV3AdaptorV2Calls {
        fn from(var: AssetsUsedCall) -> Self {
            UniswapV3AdaptorV2Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for UniswapV3AdaptorV2Calls {
        fn from(var: BalanceOfCall) -> Self {
            UniswapV3AdaptorV2Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<ClosePositionCall> for UniswapV3AdaptorV2Calls {
        fn from(var: ClosePositionCall) -> Self {
            UniswapV3AdaptorV2Calls::ClosePosition(var)
        }
    }
    impl ::std::convert::From<CollectFeesCall> for UniswapV3AdaptorV2Calls {
        fn from(var: CollectFeesCall) -> Self {
            UniswapV3AdaptorV2Calls::CollectFees(var)
        }
    }
    impl ::std::convert::From<DepositCall> for UniswapV3AdaptorV2Calls {
        fn from(var: DepositCall) -> Self {
            UniswapV3AdaptorV2Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for UniswapV3AdaptorV2Calls {
        fn from(var: IdentifierCall) -> Self {
            UniswapV3AdaptorV2Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for UniswapV3AdaptorV2Calls {
        fn from(var: IsDebtCall) -> Self {
            UniswapV3AdaptorV2Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<OpenPositionCall> for UniswapV3AdaptorV2Calls {
        fn from(var: OpenPositionCall) -> Self {
            UniswapV3AdaptorV2Calls::OpenPosition(var)
        }
    }
    impl ::std::convert::From<OracleSwapCall> for UniswapV3AdaptorV2Calls {
        fn from(var: OracleSwapCall) -> Self {
            UniswapV3AdaptorV2Calls::OracleSwap(var)
        }
    }
    impl ::std::convert::From<PurgeAllZeroLiquidityPositionsCall> for UniswapV3AdaptorV2Calls {
        fn from(var: PurgeAllZeroLiquidityPositionsCall) -> Self {
            UniswapV3AdaptorV2Calls::PurgeAllZeroLiquidityPositions(var)
        }
    }
    impl ::std::convert::From<PurgeSinglePositionCall> for UniswapV3AdaptorV2Calls {
        fn from(var: PurgeSinglePositionCall) -> Self {
            UniswapV3AdaptorV2Calls::PurgeSinglePosition(var)
        }
    }
    impl ::std::convert::From<RemoveUnOwnedPositionFromTrackerCall> for UniswapV3AdaptorV2Calls {
        fn from(var: RemoveUnOwnedPositionFromTrackerCall) -> Self {
            UniswapV3AdaptorV2Calls::RemoveUnOwnedPositionFromTracker(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for UniswapV3AdaptorV2Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            UniswapV3AdaptorV2Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SwapCall> for UniswapV3AdaptorV2Calls {
        fn from(var: SwapCall) -> Self {
            UniswapV3AdaptorV2Calls::Swap(var)
        }
    }
    impl ::std::convert::From<TakeFromPositionCall> for UniswapV3AdaptorV2Calls {
        fn from(var: TakeFromPositionCall) -> Self {
            UniswapV3AdaptorV2Calls::TakeFromPosition(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for UniswapV3AdaptorV2Calls {
        fn from(var: WithdrawCall) -> Self {
            UniswapV3AdaptorV2Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for UniswapV3AdaptorV2Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            UniswapV3AdaptorV2Calls::WithdrawableFrom(var)
        }
    }
}
