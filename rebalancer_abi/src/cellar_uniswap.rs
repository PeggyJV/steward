pub use uniswapv3cellar_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswapv3cellar_mod {
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
    #[doc = "UniswapV3Cellar was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPV3CELLAR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"symbol_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_token0\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_token1\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"_feeLevel\",\n        \"type\": \"uint24\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint184\",\n            \"name\": \"tokenId\",\n            \"type\": \"uint184\"\n          },\n          {\n            \"internalType\": \"int24\",\n            \"name\": \"tickUpper\",\n            \"type\": \"int24\"\n          },\n          {\n            \"internalType\": \"int24\",\n            \"name\": \"tickLower\",\n            \"type\": \"int24\"\n          },\n          {\n            \"internalType\": \"uint24\",\n            \"name\": \"weight\",\n            \"type\": \"uint24\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarTickInfo[]\",\n        \"name\": \"_cellarTickInfo\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token0\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token1\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AddedLiquidity\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token0\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"token1\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RemovedLiquidity\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"FEEDOMINATOR\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NONFUNGIBLEPOSITIONMANAGER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"SWAPROUTER\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UNISWAPV3FACTORY\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Desired\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Desired\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarAddParams\",\n        \"name\": \"cellarParams\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"addLiquidityEthForUniV3\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Desired\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Desired\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarAddParams\",\n        \"name\": \"cellarParams\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"addLiquidityForUniV3\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"cellarTickInfo\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint184\",\n        \"name\": \"tokenId\",\n        \"type\": \"uint184\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"weight\",\n        \"type\": \"uint24\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fee\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeLevel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"\",\n        \"type\": \"uint24\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint184\",\n            \"name\": \"tokenId\",\n            \"type\": \"uint184\"\n          },\n          {\n            \"internalType\": \"int24\",\n            \"name\": \"tickUpper\",\n            \"type\": \"int24\"\n          },\n          {\n            \"internalType\": \"int24\",\n            \"name\": \"tickLower\",\n            \"type\": \"int24\"\n          },\n          {\n            \"internalType\": \"uint24\",\n            \"name\": \"weight\",\n            \"type\": \"uint24\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarTickInfo[]\",\n        \"name\": \"_cellarTickInfo\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"rebalance\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"reinvest\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"tokenAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarRemoveParams\",\n        \"name\": \"cellarParams\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"removeLiquidityEthFromUniV3\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"tokenAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount0Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount1Min\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"deadline\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ICellarPoolShare.CellarRemoveParams\",\n        \"name\": \"cellarParams\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"removeLiquidityFromUniV3\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"newFee\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"setFee\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_validator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"value\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"setValidator\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token0\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token1\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"validator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapV3Cellar<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapV3Cellar<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapV3Cellar<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3Cellar))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapV3Cellar<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                UNISWAPV3CELLAR_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `FEEDOMINATOR` (0x57a5a6fa) function"]
        pub fn feedominator(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([87, 165, 166, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `NONFUNGIBLEPOSITIONMANAGER` (0x8eb5ab77) function"]
        pub fn nonfungiblepositionmanager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([142, 181, 171, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `SWAPROUTER` (0xcbd7be97) function"]
        pub fn swaprouter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([203, 215, 190, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNISWAPV3FACTORY` (0xfc2aaa4c) function"]
        pub fn uniswapv3factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([252, 42, 170, 76], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityEthForUniV3` (0x70e032b1) function"]
        pub fn add_liquidity_eth_for_uni_v3(
            &self,
            cellar_params: CellarAddParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 224, 50, 177], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addLiquidityForUniV3` (0x33bc230a) function"]
        pub fn add_liquidity_for_uni_v3(
            &self,
            cellar_params: CellarAddParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 188, 35, 10], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            owner: ethers::core::types::Address,
            spender: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
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
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cellarTickInfo` (0x15723866) function"]
        pub fn cellar_tick_info(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::U256, i32, i32, u32)>
        {
            self.0
                .method_hash([21, 114, 56, 102], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fee` (0xddca3f43) function"]
        pub fn fee(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `feeLevel` (0x7cf134cb) function"]
        pub fn fee_level(&self) -> ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([124, 241, 52, 203], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
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
        #[doc = "Calls the contract's `rebalance` (0x135d4f24) function"]
        pub fn rebalance(
            &self,
            cellar_tick_info: ::std::vec::Vec<CellarTickInfo>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 93, 79, 36], cellar_tick_info)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `reinvest` (0xfdb5a03e) function"]
        pub fn reinvest(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 181, 160, 62], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityEthFromUniV3` (0xa2d96f5f) function"]
        pub fn remove_liquidity_eth_from_uni_v3(
            &self,
            cellar_params: CellarRemoveParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 217, 111, 95], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeLiquidityFromUniV3` (0x276cd920) function"]
        pub fn remove_liquidity_from_uni_v3(
            &self,
            cellar_params: CellarRemoveParams,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 108, 217, 32], (cellar_params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setFee` (0x8e005553) function"]
        pub fn set_fee(&self, new_fee: u16) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 0, 85, 83], new_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setValidator` (0x4623c91d) function"]
        pub fn set_validator(
            &self,
            validator: ethers::core::types::Address,
            value: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 35, 201, 29], (validator, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token0` (0x0dfe1681) function"]
        pub fn token_0(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([13, 254, 22, 129], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `token1` (0xd21220a7) function"]
        pub fn token_1(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([210, 18, 32, 167], ())
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
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: ethers::core::types::Address,
            recipient: ethers::core::types::Address,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
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
        #[doc = "Calls the contract's `validator` (0x223b3b7a) function"]
        pub fn validator(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([34, 59, 59, 122], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AddedLiquidity` event"]
        pub fn added_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AddedLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RemovedLiquidity` event"]
        pub fn removed_liquidity_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RemovedLiquidityFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UniswapV3CellarEvents> {
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
        name = "AddedLiquidity",
        abi = "AddedLiquidity(address,address,uint128,uint256,uint256)"
    )]
    pub struct AddedLiquidityFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
        pub value: ethers::core::types::U256,
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
        name = "RemovedLiquidity",
        abi = "RemovedLiquidity(address,address,uint128,uint256,uint256)"
    )]
    pub struct RemovedLiquidityFilter {
        #[ethevent(indexed)]
        pub token_0: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_1: ethers::core::types::Address,
        pub liquidity: u128,
        pub amount_0: ethers::core::types::U256,
        pub amount_1: ethers::core::types::U256,
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
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3CellarEvents {
        AddedLiquidityFilter(AddedLiquidityFilter),
        ApprovalFilter(ApprovalFilter),
        RemovedLiquidityFilter(RemovedLiquidityFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for UniswapV3CellarEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AddedLiquidityFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::AddedLiquidityFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = RemovedLiquidityFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::RemovedLiquidityFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(UniswapV3CellarEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UniswapV3CellarEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3CellarEvents::AddedLiquidityFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::ApprovalFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::RemovedLiquidityFilter(element) => element.fmt(f),
                UniswapV3CellarEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FEEDOMINATOR`function with signature `FEEDOMINATOR()` and selector `[87, 165, 166, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "FEEDOMINATOR", abi = "FEEDOMINATOR()")]
    pub struct FeedominatorCall;
    #[doc = "Container type for all input parameters for the `NONFUNGIBLEPOSITIONMANAGER`function with signature `NONFUNGIBLEPOSITIONMANAGER()` and selector `[142, 181, 171, 119]`"]
    #[derive(
        Clone,
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
        name = "NONFUNGIBLEPOSITIONMANAGER",
        abi = "NONFUNGIBLEPOSITIONMANAGER()"
    )]
    pub struct NonfungiblepositionmanagerCall;
    #[doc = "Container type for all input parameters for the `SWAPROUTER`function with signature `SWAPROUTER()` and selector `[203, 215, 190, 151]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "SWAPROUTER", abi = "SWAPROUTER()")]
    pub struct SwaprouterCall;
    #[doc = "Container type for all input parameters for the `UNISWAPV3FACTORY`function with signature `UNISWAPV3FACTORY()` and selector `[252, 42, 170, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "UNISWAPV3FACTORY", abi = "UNISWAPV3FACTORY()")]
    pub struct Uniswapv3FactoryCall;
    #[doc = "Container type for all input parameters for the `WETH`function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `addLiquidityEthForUniV3`function with signature `addLiquidityEthForUniV3((uint256,uint256,uint256,uint256,address,uint256))` and selector `[112, 224, 50, 177]`"]
    #[derive(
        Clone,
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
        name = "addLiquidityEthForUniV3",
        abi = "addLiquidityEthForUniV3((uint256,uint256,uint256,uint256,address,uint256))"
    )]
    pub struct AddLiquidityEthForUniV3Call {
        pub cellar_params: CellarAddParams,
    }
    #[doc = "Container type for all input parameters for the `addLiquidityForUniV3`function with signature `addLiquidityForUniV3((uint256,uint256,uint256,uint256,address,uint256))` and selector `[51, 188, 35, 10]`"]
    #[derive(
        Clone,
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
        name = "addLiquidityForUniV3",
        abi = "addLiquidityForUniV3((uint256,uint256,uint256,uint256,address,uint256))"
    )]
    pub struct AddLiquidityForUniV3Call {
        pub cellar_params: CellarAddParams,
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
    pub struct AllowanceCall {
        pub owner: ethers::core::types::Address,
        pub spender: ethers::core::types::Address,
    }
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
    pub struct BalanceOfCall {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cellarTickInfo`function with signature `cellarTickInfo(uint256)` and selector `[21, 114, 56, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "cellarTickInfo", abi = "cellarTickInfo(uint256)")]
    pub struct CellarTickInfoCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `fee`function with signature `fee()` and selector `[221, 202, 63, 67]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    #[doc = "Container type for all input parameters for the `feeLevel`function with signature `feeLevel()` and selector `[124, 241, 52, 203]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "feeLevel", abi = "feeLevel()")]
    pub struct FeeLevelCall;
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
    #[doc = "Container type for all input parameters for the `rebalance`function with signature `rebalance((uint184,int24,int24,uint24)[])` and selector `[19, 93, 79, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "rebalance", abi = "rebalance((uint184,int24,int24,uint24)[])")]
    pub struct RebalanceCall {
        pub cellar_tick_info: ::std::vec::Vec<CellarTickInfo>,
    }
    #[doc = "Container type for all input parameters for the `reinvest`function with signature `reinvest()` and selector `[253, 181, 160, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "reinvest", abi = "reinvest()")]
    pub struct ReinvestCall;
    #[doc = "Container type for all input parameters for the `removeLiquidityEthFromUniV3`function with signature `removeLiquidityEthFromUniV3((uint256,uint256,uint256,address,uint256))` and selector `[162, 217, 111, 95]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidityEthFromUniV3",
        abi = "removeLiquidityEthFromUniV3((uint256,uint256,uint256,address,uint256))"
    )]
    pub struct RemoveLiquidityEthFromUniV3Call {
        pub cellar_params: CellarRemoveParams,
    }
    #[doc = "Container type for all input parameters for the `removeLiquidityFromUniV3`function with signature `removeLiquidityFromUniV3((uint256,uint256,uint256,address,uint256))` and selector `[39, 108, 217, 32]`"]
    #[derive(
        Clone,
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
        name = "removeLiquidityFromUniV3",
        abi = "removeLiquidityFromUniV3((uint256,uint256,uint256,address,uint256))"
    )]
    pub struct RemoveLiquidityFromUniV3Call {
        pub cellar_params: CellarRemoveParams,
    }
    #[doc = "Container type for all input parameters for the `setFee`function with signature `setFee(uint16)` and selector `[142, 0, 85, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setFee", abi = "setFee(uint16)")]
    pub struct SetFeeCall {
        pub new_fee: u16,
    }
    #[doc = "Container type for all input parameters for the `setValidator`function with signature `setValidator(address,bool)` and selector `[70, 35, 201, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setValidator", abi = "setValidator(address,bool)")]
    pub struct SetValidatorCall {
        pub validator: ethers::core::types::Address,
        pub value: bool,
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
    #[doc = "Container type for all input parameters for the `token0`function with signature `token0()` and selector `[13, 254, 22, 129]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token0", abi = "token0()")]
    pub struct Token0Call;
    #[doc = "Container type for all input parameters for the `token1`function with signature `token1()` and selector `[210, 18, 32, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "token1", abi = "token1()")]
    pub struct Token1Call;
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
        pub recipient: ethers::core::types::Address,
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
        pub sender: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `validator`function with signature `validator(address)` and selector `[34, 59, 59, 122]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "validator", abi = "validator(address)")]
    pub struct ValidatorCall(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapV3CellarCalls {
        Feedominator(FeedominatorCall),
        Nonfungiblepositionmanager(NonfungiblepositionmanagerCall),
        Swaprouter(SwaprouterCall),
        Uniswapv3Factory(Uniswapv3FactoryCall),
        Weth(WethCall),
        AddLiquidityEthForUniV3(AddLiquidityEthForUniV3Call),
        AddLiquidityForUniV3(AddLiquidityForUniV3Call),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        CellarTickInfo(CellarTickInfoCall),
        Decimals(DecimalsCall),
        Fee(FeeCall),
        FeeLevel(FeeLevelCall),
        Name(NameCall),
        Owner(OwnerCall),
        Rebalance(RebalanceCall),
        Reinvest(ReinvestCall),
        RemoveLiquidityEthFromUniV3(RemoveLiquidityEthFromUniV3Call),
        RemoveLiquidityFromUniV3(RemoveLiquidityFromUniV3Call),
        SetFee(SetFeeCall),
        SetValidator(SetValidatorCall),
        Symbol(SymbolCall),
        Token0(Token0Call),
        Token1(Token1Call),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Validator(ValidatorCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapV3CellarCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FeedominatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Feedominator(decoded));
            }
            if let Ok(decoded) =
                <NonfungiblepositionmanagerCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3CellarCalls::Nonfungiblepositionmanager(decoded));
            }
            if let Ok(decoded) =
                <SwaprouterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Swaprouter(decoded));
            }
            if let Ok(decoded) =
                <Uniswapv3FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Uniswapv3Factory(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3CellarCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityEthForUniV3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::AddLiquidityEthForUniV3(decoded));
            }
            if let Ok(decoded) =
                <AddLiquidityForUniV3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::AddLiquidityForUniV3(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CellarTickInfoCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::CellarTickInfo(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Decimals(decoded));
            }
            if let Ok(decoded) = <FeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3CellarCalls::Fee(decoded));
            }
            if let Ok(decoded) =
                <FeeLevelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::FeeLevel(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UniswapV3CellarCalls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <RebalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Rebalance(decoded));
            }
            if let Ok(decoded) =
                <ReinvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Reinvest(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityEthFromUniV3Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3CellarCalls::RemoveLiquidityEthFromUniV3(decoded));
            }
            if let Ok(decoded) =
                <RemoveLiquidityFromUniV3Call as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapV3CellarCalls::RemoveLiquidityFromUniV3(decoded));
            }
            if let Ok(decoded) = <SetFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetFee(decoded));
            }
            if let Ok(decoded) =
                <SetValidatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::SetValidator(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Symbol(decoded));
            }
            if let Ok(decoded) = <Token0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Token0(decoded));
            }
            if let Ok(decoded) = <Token1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Token1(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <ValidatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapV3CellarCalls::Validator(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapV3CellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapV3CellarCalls::Feedominator(element) => element.encode(),
                UniswapV3CellarCalls::Nonfungiblepositionmanager(element) => element.encode(),
                UniswapV3CellarCalls::Swaprouter(element) => element.encode(),
                UniswapV3CellarCalls::Uniswapv3Factory(element) => element.encode(),
                UniswapV3CellarCalls::Weth(element) => element.encode(),
                UniswapV3CellarCalls::AddLiquidityEthForUniV3(element) => element.encode(),
                UniswapV3CellarCalls::AddLiquidityForUniV3(element) => element.encode(),
                UniswapV3CellarCalls::Allowance(element) => element.encode(),
                UniswapV3CellarCalls::Approve(element) => element.encode(),
                UniswapV3CellarCalls::BalanceOf(element) => element.encode(),
                UniswapV3CellarCalls::CellarTickInfo(element) => element.encode(),
                UniswapV3CellarCalls::Decimals(element) => element.encode(),
                UniswapV3CellarCalls::Fee(element) => element.encode(),
                UniswapV3CellarCalls::FeeLevel(element) => element.encode(),
                UniswapV3CellarCalls::Name(element) => element.encode(),
                UniswapV3CellarCalls::Owner(element) => element.encode(),
                UniswapV3CellarCalls::Rebalance(element) => element.encode(),
                UniswapV3CellarCalls::Reinvest(element) => element.encode(),
                UniswapV3CellarCalls::RemoveLiquidityEthFromUniV3(element) => element.encode(),
                UniswapV3CellarCalls::RemoveLiquidityFromUniV3(element) => element.encode(),
                UniswapV3CellarCalls::SetFee(element) => element.encode(),
                UniswapV3CellarCalls::SetValidator(element) => element.encode(),
                UniswapV3CellarCalls::Symbol(element) => element.encode(),
                UniswapV3CellarCalls::Token0(element) => element.encode(),
                UniswapV3CellarCalls::Token1(element) => element.encode(),
                UniswapV3CellarCalls::TotalSupply(element) => element.encode(),
                UniswapV3CellarCalls::Transfer(element) => element.encode(),
                UniswapV3CellarCalls::TransferFrom(element) => element.encode(),
                UniswapV3CellarCalls::TransferOwnership(element) => element.encode(),
                UniswapV3CellarCalls::Validator(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapV3CellarCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapV3CellarCalls::Feedominator(element) => element.fmt(f),
                UniswapV3CellarCalls::Nonfungiblepositionmanager(element) => element.fmt(f),
                UniswapV3CellarCalls::Swaprouter(element) => element.fmt(f),
                UniswapV3CellarCalls::Uniswapv3Factory(element) => element.fmt(f),
                UniswapV3CellarCalls::Weth(element) => element.fmt(f),
                UniswapV3CellarCalls::AddLiquidityEthForUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::AddLiquidityForUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::Allowance(element) => element.fmt(f),
                UniswapV3CellarCalls::Approve(element) => element.fmt(f),
                UniswapV3CellarCalls::BalanceOf(element) => element.fmt(f),
                UniswapV3CellarCalls::CellarTickInfo(element) => element.fmt(f),
                UniswapV3CellarCalls::Decimals(element) => element.fmt(f),
                UniswapV3CellarCalls::Fee(element) => element.fmt(f),
                UniswapV3CellarCalls::FeeLevel(element) => element.fmt(f),
                UniswapV3CellarCalls::Name(element) => element.fmt(f),
                UniswapV3CellarCalls::Owner(element) => element.fmt(f),
                UniswapV3CellarCalls::Rebalance(element) => element.fmt(f),
                UniswapV3CellarCalls::Reinvest(element) => element.fmt(f),
                UniswapV3CellarCalls::RemoveLiquidityEthFromUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::RemoveLiquidityFromUniV3(element) => element.fmt(f),
                UniswapV3CellarCalls::SetFee(element) => element.fmt(f),
                UniswapV3CellarCalls::SetValidator(element) => element.fmt(f),
                UniswapV3CellarCalls::Symbol(element) => element.fmt(f),
                UniswapV3CellarCalls::Token0(element) => element.fmt(f),
                UniswapV3CellarCalls::Token1(element) => element.fmt(f),
                UniswapV3CellarCalls::TotalSupply(element) => element.fmt(f),
                UniswapV3CellarCalls::Transfer(element) => element.fmt(f),
                UniswapV3CellarCalls::TransferFrom(element) => element.fmt(f),
                UniswapV3CellarCalls::TransferOwnership(element) => element.fmt(f),
                UniswapV3CellarCalls::Validator(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FeedominatorCall> for UniswapV3CellarCalls {
        fn from(var: FeedominatorCall) -> Self {
            UniswapV3CellarCalls::Feedominator(var)
        }
    }
    impl ::std::convert::From<NonfungiblepositionmanagerCall> for UniswapV3CellarCalls {
        fn from(var: NonfungiblepositionmanagerCall) -> Self {
            UniswapV3CellarCalls::Nonfungiblepositionmanager(var)
        }
    }
    impl ::std::convert::From<SwaprouterCall> for UniswapV3CellarCalls {
        fn from(var: SwaprouterCall) -> Self {
            UniswapV3CellarCalls::Swaprouter(var)
        }
    }
    impl ::std::convert::From<Uniswapv3FactoryCall> for UniswapV3CellarCalls {
        fn from(var: Uniswapv3FactoryCall) -> Self {
            UniswapV3CellarCalls::Uniswapv3Factory(var)
        }
    }
    impl ::std::convert::From<WethCall> for UniswapV3CellarCalls {
        fn from(var: WethCall) -> Self {
            UniswapV3CellarCalls::Weth(var)
        }
    }
    impl ::std::convert::From<AddLiquidityEthForUniV3Call> for UniswapV3CellarCalls {
        fn from(var: AddLiquidityEthForUniV3Call) -> Self {
            UniswapV3CellarCalls::AddLiquidityEthForUniV3(var)
        }
    }
    impl ::std::convert::From<AddLiquidityForUniV3Call> for UniswapV3CellarCalls {
        fn from(var: AddLiquidityForUniV3Call) -> Self {
            UniswapV3CellarCalls::AddLiquidityForUniV3(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for UniswapV3CellarCalls {
        fn from(var: AllowanceCall) -> Self {
            UniswapV3CellarCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for UniswapV3CellarCalls {
        fn from(var: ApproveCall) -> Self {
            UniswapV3CellarCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for UniswapV3CellarCalls {
        fn from(var: BalanceOfCall) -> Self {
            UniswapV3CellarCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CellarTickInfoCall> for UniswapV3CellarCalls {
        fn from(var: CellarTickInfoCall) -> Self {
            UniswapV3CellarCalls::CellarTickInfo(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for UniswapV3CellarCalls {
        fn from(var: DecimalsCall) -> Self {
            UniswapV3CellarCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<FeeCall> for UniswapV3CellarCalls {
        fn from(var: FeeCall) -> Self {
            UniswapV3CellarCalls::Fee(var)
        }
    }
    impl ::std::convert::From<FeeLevelCall> for UniswapV3CellarCalls {
        fn from(var: FeeLevelCall) -> Self {
            UniswapV3CellarCalls::FeeLevel(var)
        }
    }
    impl ::std::convert::From<NameCall> for UniswapV3CellarCalls {
        fn from(var: NameCall) -> Self {
            UniswapV3CellarCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for UniswapV3CellarCalls {
        fn from(var: OwnerCall) -> Self {
            UniswapV3CellarCalls::Owner(var)
        }
    }
    impl ::std::convert::From<RebalanceCall> for UniswapV3CellarCalls {
        fn from(var: RebalanceCall) -> Self {
            UniswapV3CellarCalls::Rebalance(var)
        }
    }
    impl ::std::convert::From<ReinvestCall> for UniswapV3CellarCalls {
        fn from(var: ReinvestCall) -> Self {
            UniswapV3CellarCalls::Reinvest(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityEthFromUniV3Call> for UniswapV3CellarCalls {
        fn from(var: RemoveLiquidityEthFromUniV3Call) -> Self {
            UniswapV3CellarCalls::RemoveLiquidityEthFromUniV3(var)
        }
    }
    impl ::std::convert::From<RemoveLiquidityFromUniV3Call> for UniswapV3CellarCalls {
        fn from(var: RemoveLiquidityFromUniV3Call) -> Self {
            UniswapV3CellarCalls::RemoveLiquidityFromUniV3(var)
        }
    }
    impl ::std::convert::From<SetFeeCall> for UniswapV3CellarCalls {
        fn from(var: SetFeeCall) -> Self {
            UniswapV3CellarCalls::SetFee(var)
        }
    }
    impl ::std::convert::From<SetValidatorCall> for UniswapV3CellarCalls {
        fn from(var: SetValidatorCall) -> Self {
            UniswapV3CellarCalls::SetValidator(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for UniswapV3CellarCalls {
        fn from(var: SymbolCall) -> Self {
            UniswapV3CellarCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<Token0Call> for UniswapV3CellarCalls {
        fn from(var: Token0Call) -> Self {
            UniswapV3CellarCalls::Token0(var)
        }
    }
    impl ::std::convert::From<Token1Call> for UniswapV3CellarCalls {
        fn from(var: Token1Call) -> Self {
            UniswapV3CellarCalls::Token1(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for UniswapV3CellarCalls {
        fn from(var: TotalSupplyCall) -> Self {
            UniswapV3CellarCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for UniswapV3CellarCalls {
        fn from(var: TransferCall) -> Self {
            UniswapV3CellarCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for UniswapV3CellarCalls {
        fn from(var: TransferFromCall) -> Self {
            UniswapV3CellarCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for UniswapV3CellarCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            UniswapV3CellarCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<ValidatorCall> for UniswapV3CellarCalls {
        fn from(var: ValidatorCall) -> Self {
            UniswapV3CellarCalls::Validator(var)
        }
    }
    #[doc = "`CellarAddParams(uint256,uint256,uint256,uint256,address,uint256)`"]
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
    pub struct CellarAddParams {
        pub amount_0_desired: ethers::core::types::U256,
        pub amount_1_desired: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    impl CellarAddParams {
        pub fn new(
            amount_0_desired: U256,
            amount_1_desired: U256,
            amount_0_min: U256,
            amount_1_min: U256,
            recipient: H160,
            deadline: U256,
        ) -> Self {
            CellarAddParams {
                amount_0_desired,
                amount_1_desired,
                amount_0_min,
                amount_1_min,
                recipient,
                deadline,
            }
        }
    }
    #[doc = "`CellarRemoveParams(uint256,uint256,uint256,address,uint256)`"]
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
    pub struct CellarRemoveParams {
        pub token_amount: ethers::core::types::U256,
        pub amount_0_min: ethers::core::types::U256,
        pub amount_1_min: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
    }
    #[doc = "`CellarTickInfo(uint184,int24,int24,uint24)`"]
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
    pub struct CellarTickInfo {
        pub token_id: ethers::core::types::U256,
        pub tick_upper: i32,
        pub tick_lower: i32,
        pub weight: u32,
    }
}
