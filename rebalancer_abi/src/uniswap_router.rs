pub use uniswaprouter_mod::*;
#[allow(clippy::too_many_arguments)]
mod uniswaprouter_mod {
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
    #[doc = "UniswapRouter was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UNISWAPROUTER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"_factory\", \"type\": \"address\" },\n      { \"internalType\": \"address\", \"name\": \"_WETH9\", \"type\": \"address\" }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH9\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"bytes\", \"name\": \"path\", \"type\": \"bytes\" },\n          { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n          { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n          { \"internalType\": \"uint256\", \"name\": \"amountIn\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountOutMinimum\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ISwapRouter.ExactInputParams\",\n        \"name\": \"params\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"exactInput\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"address\", \"name\": \"tokenIn\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"tokenOut\", \"type\": \"address\" },\n          { \"internalType\": \"uint24\", \"name\": \"fee\", \"type\": \"uint24\" },\n          { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n          { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n          { \"internalType\": \"uint256\", \"name\": \"amountIn\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountOutMinimum\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint160\",\n            \"name\": \"sqrtPriceLimitX96\",\n            \"type\": \"uint160\"\n          }\n        ],\n        \"internalType\": \"struct ISwapRouter.ExactInputSingleParams\",\n        \"name\": \"params\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"exactInputSingle\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"bytes\", \"name\": \"path\", \"type\": \"bytes\" },\n          { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n          { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n          { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountInMaximum\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct ISwapRouter.ExactOutputParams\",\n        \"name\": \"params\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"exactOutput\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountIn\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          { \"internalType\": \"address\", \"name\": \"tokenIn\", \"type\": \"address\" },\n          { \"internalType\": \"address\", \"name\": \"tokenOut\", \"type\": \"address\" },\n          { \"internalType\": \"uint24\", \"name\": \"fee\", \"type\": \"uint24\" },\n          { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n          { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n          { \"internalType\": \"uint256\", \"name\": \"amountOut\", \"type\": \"uint256\" },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amountInMaximum\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint160\",\n            \"name\": \"sqrtPriceLimitX96\",\n            \"type\": \"uint160\"\n          }\n        ],\n        \"internalType\": \"struct ISwapRouter.ExactOutputSingleParams\",\n        \"name\": \"params\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"exactOutputSingle\",\n    \"outputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountIn\", \"type\": \"uint256\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [{ \"internalType\": \"address\", \"name\": \"\", \"type\": \"address\" }],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"bytes[]\", \"name\": \"data\", \"type\": \"bytes[]\" }\n    ],\n    \"name\": \"multicall\",\n    \"outputs\": [\n      { \"internalType\": \"bytes[]\", \"name\": \"results\", \"type\": \"bytes[]\" }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"refundETH\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"selfPermit\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"nonce\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"expiry\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"selfPermitAllowed\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"nonce\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"expiry\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"selfPermitAllowedIfNecessary\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"value\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint256\", \"name\": \"deadline\", \"type\": \"uint256\" },\n      { \"internalType\": \"uint8\", \"name\": \"v\", \"type\": \"uint8\" },\n      { \"internalType\": \"bytes32\", \"name\": \"r\", \"type\": \"bytes32\" },\n      { \"internalType\": \"bytes32\", \"name\": \"s\", \"type\": \"bytes32\" }\n    ],\n    \"name\": \"selfPermitIfNecessary\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amountMinimum\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" }\n    ],\n    \"name\": \"sweepToken\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"address\", \"name\": \"token\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"amountMinimum\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"feeBips\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"feeRecipient\", \"type\": \"address\" }\n    ],\n    \"name\": \"sweepTokenWithFee\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"int256\", \"name\": \"amount0Delta\", \"type\": \"int256\" },\n      { \"internalType\": \"int256\", \"name\": \"amount1Delta\", \"type\": \"int256\" },\n      { \"internalType\": \"bytes\", \"name\": \"_data\", \"type\": \"bytes\" }\n    ],\n    \"name\": \"uniswapV3SwapCallback\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountMinimum\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" }\n    ],\n    \"name\": \"unwrapWETH9\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      { \"internalType\": \"uint256\", \"name\": \"amountMinimum\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"recipient\", \"type\": \"address\" },\n      { \"internalType\": \"uint256\", \"name\": \"feeBips\", \"type\": \"uint256\" },\n      { \"internalType\": \"address\", \"name\": \"feeRecipient\", \"type\": \"address\" }\n    ],\n    \"name\": \"unwrapWETH9WithFee\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  { \"stateMutability\": \"payable\", \"type\": \"receive\" }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct UniswapRouter<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for UniswapRouter<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for UniswapRouter<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(UniswapRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> UniswapRouter<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), UNISWAPROUTER_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH9` (0x4aa4a4fc) function"]
        pub fn weth9(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([74, 164, 164, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInput` (0xc04b8d59) function"]
        pub fn exact_input(
            &self,
            params: ExactInputParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([192, 75, 141, 89], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactInputSingle` (0x414bf389) function"]
        pub fn exact_input_single(
            &self,
            params: ExactInputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([65, 75, 243, 137], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutput` (0xf28c0498) function"]
        pub fn exact_output(
            &self,
            params: ExactOutputParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 140, 4, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `exactOutputSingle` (0xdb3e2198) function"]
        pub fn exact_output_single(
            &self,
            params: ExactOutputSingleParams,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([219, 62, 33, 152], (params,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `factory` (0xc45a0155) function"]
        pub fn factory(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multicall` (0xac9650d8) function"]
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<Vec<u8>>,
        ) -> ethers::contract::builders::ContractCall<M, Vec<Vec<u8>>> {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `refundETH` (0x12210e8a) function"]
        pub fn refund_eth(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 33, 14, 138], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermit` (0xf3995c67) function"]
        pub fn self_permit(
            &self,
            token: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 153, 92, 103], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowed` (0x4659a494) function"]
        pub fn self_permit_allowed(
            &self,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 89, 164, 148], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitAllowedIfNecessary` (0xa4a78f0c) function"]
        pub fn self_permit_allowed_if_necessary(
            &self,
            token: ethers::core::types::Address,
            nonce: ethers::core::types::U256,
            expiry: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([164, 167, 143, 12], (token, nonce, expiry, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `selfPermitIfNecessary` (0xc2e3140a) function"]
        pub fn self_permit_if_necessary(
            &self,
            token: ethers::core::types::Address,
            value: ethers::core::types::U256,
            deadline: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 227, 20, 10], (token, value, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepToken` (0xdf2ab5bb) function"]
        pub fn sweep_token(
            &self,
            token: ethers::core::types::Address,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 42, 181, 187], (token, amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sweepTokenWithFee` (0xe0e189a0) function"]
        pub fn sweep_token_with_fee(
            &self,
            token: ethers::core::types::Address,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            fee_bips: ethers::core::types::U256,
            fee_recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [224, 225, 137, 160],
                    (token, amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function"]
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: I256,
            amount_1_delta: I256,
            data: Vec<u8>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9` (0x49404b7c) function"]
        pub fn unwrap_weth9(
            &self,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 64, 75, 124], (amount_minimum, recipient))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unwrapWETH9WithFee` (0x9b2c0a37) function"]
        pub fn unwrap_weth9_with_fee(
            &self,
            amount_minimum: ethers::core::types::U256,
            recipient: ethers::core::types::Address,
            fee_bips: ethers::core::types::U256,
            fee_recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [155, 44, 10, 55],
                    (amount_minimum, recipient, fee_bips, fee_recipient),
                )
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `WETH9`function with signature `WETH9()` and selector `[74, 164, 164, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "WETH9", abi = "WETH9()")]
    pub struct Weth9Call;
    #[doc = "Container type for all input parameters for the `exactInput`function with signature `exactInput((bytes,address,uint256,uint256,uint256))` and selector `[192, 75, 141, 89]`"]
    #[derive(
        Clone,
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
        name = "exactInput",
        abi = "exactInput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactInputCall {
        pub params: ExactInputParams,
    }
    #[doc = "Container type for all input parameters for the `exactInputSingle`function with signature `exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[65, 75, 243, 137]`"]
    #[derive(
        Clone,
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
        name = "exactInputSingle",
        abi = "exactInputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactInputSingleCall {
        pub params: ExactInputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutput`function with signature `exactOutput((bytes,address,uint256,uint256,uint256))` and selector `[242, 140, 4, 152]`"]
    #[derive(
        Clone,
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
        name = "exactOutput",
        abi = "exactOutput((bytes,address,uint256,uint256,uint256))"
    )]
    pub struct ExactOutputCall {
        pub params: ExactOutputParams,
    }
    #[doc = "Container type for all input parameters for the `exactOutputSingle`function with signature `exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))` and selector `[219, 62, 33, 152]`"]
    #[derive(
        Clone,
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
        name = "exactOutputSingle",
        abi = "exactOutputSingle((address,address,uint24,address,uint256,uint256,uint256,uint160))"
    )]
    pub struct ExactOutputSingleCall {
        pub params: ExactOutputSingleParams,
    }
    #[doc = "Container type for all input parameters for the `factory`function with signature `factory()` and selector `[196, 90, 1, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
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
        pub data: ::std::vec::Vec<Vec<u8>>,
    }
    #[doc = "Container type for all input parameters for the `refundETH`function with signature `refundETH()` and selector `[18, 33, 14, 138]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "refundETH", abi = "refundETH()")]
    pub struct RefundETHCall;
    #[doc = "Container type for all input parameters for the `selfPermit`function with signature `selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[243, 153, 92, 103]`"]
    #[derive(
        Clone,
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
        name = "selfPermit",
        abi = "selfPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitCall {
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitAllowed`function with signature `selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[70, 89, 164, 148]`"]
    #[derive(
        Clone,
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
        name = "selfPermitAllowed",
        abi = "selfPermitAllowed(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedCall {
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitAllowedIfNecessary`function with signature `selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[164, 167, 143, 12]`"]
    #[derive(
        Clone,
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
        name = "selfPermitAllowedIfNecessary",
        abi = "selfPermitAllowedIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitAllowedIfNecessaryCall {
        pub token: ethers::core::types::Address,
        pub nonce: ethers::core::types::U256,
        pub expiry: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `selfPermitIfNecessary`function with signature `selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `[194, 227, 20, 10]`"]
    #[derive(
        Clone,
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
        name = "selfPermitIfNecessary",
        abi = "selfPermitIfNecessary(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct SelfPermitIfNecessaryCall {
        pub token: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub deadline: ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `sweepToken`function with signature `sweepToken(address,uint256,address)` and selector `[223, 42, 181, 187]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "sweepToken", abi = "sweepToken(address,uint256,address)")]
    pub struct SweepTokenCall {
        pub token: ethers::core::types::Address,
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `sweepTokenWithFee`function with signature `sweepTokenWithFee(address,uint256,address,uint256,address)` and selector `[224, 225, 137, 160]`"]
    #[derive(
        Clone,
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
        name = "sweepTokenWithFee",
        abi = "sweepTokenWithFee(address,uint256,address,uint256,address)"
    )]
    pub struct SweepTokenWithFeeCall {
        pub token: ethers::core::types::Address,
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub fee_bips: ethers::core::types::U256,
        pub fee_recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `uniswapV3SwapCallback`function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `[250, 70, 30, 51]`"]
    #[derive(
        Clone,
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: I256,
        pub amount_1_delta: I256,
        pub data: Vec<u8>,
    }
    #[doc = "Container type for all input parameters for the `unwrapWETH9`function with signature `unwrapWETH9(uint256,address)` and selector `[73, 64, 75, 124]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "unwrapWETH9", abi = "unwrapWETH9(uint256,address)")]
    pub struct UnwrapWETH9Call {
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `unwrapWETH9WithFee`function with signature `unwrapWETH9WithFee(uint256,address,uint256,address)` and selector `[155, 44, 10, 55]`"]
    #[derive(
        Clone,
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
        name = "unwrapWETH9WithFee",
        abi = "unwrapWETH9WithFee(uint256,address,uint256,address)"
    )]
    pub struct UnwrapWETH9WithFeeCall {
        pub amount_minimum: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
        pub fee_bips: ethers::core::types::U256,
        pub fee_recipient: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UniswapRouterCalls {
        Weth9(Weth9Call),
        ExactInput(ExactInputCall),
        ExactInputSingle(ExactInputSingleCall),
        ExactOutput(ExactOutputCall),
        ExactOutputSingle(ExactOutputSingleCall),
        Factory(FactoryCall),
        Multicall(MulticallCall),
        RefundETH(RefundETHCall),
        SelfPermit(SelfPermitCall),
        SelfPermitAllowed(SelfPermitAllowedCall),
        SelfPermitAllowedIfNecessary(SelfPermitAllowedIfNecessaryCall),
        SelfPermitIfNecessary(SelfPermitIfNecessaryCall),
        SweepToken(SweepTokenCall),
        SweepTokenWithFee(SweepTokenWithFeeCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
        UnwrapWETH9(UnwrapWETH9Call),
        UnwrapWETH9WithFee(UnwrapWETH9WithFeeCall),
    }
    impl ethers::core::abi::AbiDecode for UniswapRouterCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <Weth9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::Weth9(decoded));
            }
            if let Ok(decoded) =
                <ExactInputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::ExactInput(decoded));
            }
            if let Ok(decoded) =
                <ExactInputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::ExactInputSingle(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::ExactOutput(decoded));
            }
            if let Ok(decoded) =
                <ExactOutputSingleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::ExactOutputSingle(decoded));
            }
            if let Ok(decoded) =
                <FactoryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::Factory(decoded));
            }
            if let Ok(decoded) =
                <MulticallCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::Multicall(decoded));
            }
            if let Ok(decoded) =
                <RefundETHCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::RefundETH(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::SelfPermit(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::SelfPermitAllowed(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitAllowedIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(UniswapRouterCalls::SelfPermitAllowedIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SelfPermitIfNecessaryCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::SelfPermitIfNecessary(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::SweepToken(decoded));
            }
            if let Ok(decoded) =
                <SweepTokenWithFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::SweepTokenWithFee(decoded));
            }
            if let Ok(decoded) =
                <UniswapV3SwapCallbackCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::UniswapV3SwapCallback(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETH9Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::UnwrapWETH9(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWETH9WithFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UniswapRouterCalls::UnwrapWETH9WithFee(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UniswapRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UniswapRouterCalls::Weth9(element) => element.encode(),
                UniswapRouterCalls::ExactInput(element) => element.encode(),
                UniswapRouterCalls::ExactInputSingle(element) => element.encode(),
                UniswapRouterCalls::ExactOutput(element) => element.encode(),
                UniswapRouterCalls::ExactOutputSingle(element) => element.encode(),
                UniswapRouterCalls::Factory(element) => element.encode(),
                UniswapRouterCalls::Multicall(element) => element.encode(),
                UniswapRouterCalls::RefundETH(element) => element.encode(),
                UniswapRouterCalls::SelfPermit(element) => element.encode(),
                UniswapRouterCalls::SelfPermitAllowed(element) => element.encode(),
                UniswapRouterCalls::SelfPermitAllowedIfNecessary(element) => element.encode(),
                UniswapRouterCalls::SelfPermitIfNecessary(element) => element.encode(),
                UniswapRouterCalls::SweepToken(element) => element.encode(),
                UniswapRouterCalls::SweepTokenWithFee(element) => element.encode(),
                UniswapRouterCalls::UniswapV3SwapCallback(element) => element.encode(),
                UniswapRouterCalls::UnwrapWETH9(element) => element.encode(),
                UniswapRouterCalls::UnwrapWETH9WithFee(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UniswapRouterCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UniswapRouterCalls::Weth9(element) => element.fmt(f),
                UniswapRouterCalls::ExactInput(element) => element.fmt(f),
                UniswapRouterCalls::ExactInputSingle(element) => element.fmt(f),
                UniswapRouterCalls::ExactOutput(element) => element.fmt(f),
                UniswapRouterCalls::ExactOutputSingle(element) => element.fmt(f),
                UniswapRouterCalls::Factory(element) => element.fmt(f),
                UniswapRouterCalls::Multicall(element) => element.fmt(f),
                UniswapRouterCalls::RefundETH(element) => element.fmt(f),
                UniswapRouterCalls::SelfPermit(element) => element.fmt(f),
                UniswapRouterCalls::SelfPermitAllowed(element) => element.fmt(f),
                UniswapRouterCalls::SelfPermitAllowedIfNecessary(element) => element.fmt(f),
                UniswapRouterCalls::SelfPermitIfNecessary(element) => element.fmt(f),
                UniswapRouterCalls::SweepToken(element) => element.fmt(f),
                UniswapRouterCalls::SweepTokenWithFee(element) => element.fmt(f),
                UniswapRouterCalls::UniswapV3SwapCallback(element) => element.fmt(f),
                UniswapRouterCalls::UnwrapWETH9(element) => element.fmt(f),
                UniswapRouterCalls::UnwrapWETH9WithFee(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<Weth9Call> for UniswapRouterCalls {
        fn from(var: Weth9Call) -> Self {
            UniswapRouterCalls::Weth9(var)
        }
    }
    impl ::std::convert::From<ExactInputCall> for UniswapRouterCalls {
        fn from(var: ExactInputCall) -> Self {
            UniswapRouterCalls::ExactInput(var)
        }
    }
    impl ::std::convert::From<ExactInputSingleCall> for UniswapRouterCalls {
        fn from(var: ExactInputSingleCall) -> Self {
            UniswapRouterCalls::ExactInputSingle(var)
        }
    }
    impl ::std::convert::From<ExactOutputCall> for UniswapRouterCalls {
        fn from(var: ExactOutputCall) -> Self {
            UniswapRouterCalls::ExactOutput(var)
        }
    }
    impl ::std::convert::From<ExactOutputSingleCall> for UniswapRouterCalls {
        fn from(var: ExactOutputSingleCall) -> Self {
            UniswapRouterCalls::ExactOutputSingle(var)
        }
    }
    impl ::std::convert::From<FactoryCall> for UniswapRouterCalls {
        fn from(var: FactoryCall) -> Self {
            UniswapRouterCalls::Factory(var)
        }
    }
    impl ::std::convert::From<MulticallCall> for UniswapRouterCalls {
        fn from(var: MulticallCall) -> Self {
            UniswapRouterCalls::Multicall(var)
        }
    }
    impl ::std::convert::From<RefundETHCall> for UniswapRouterCalls {
        fn from(var: RefundETHCall) -> Self {
            UniswapRouterCalls::RefundETH(var)
        }
    }
    impl ::std::convert::From<SelfPermitCall> for UniswapRouterCalls {
        fn from(var: SelfPermitCall) -> Self {
            UniswapRouterCalls::SelfPermit(var)
        }
    }
    impl ::std::convert::From<SelfPermitAllowedCall> for UniswapRouterCalls {
        fn from(var: SelfPermitAllowedCall) -> Self {
            UniswapRouterCalls::SelfPermitAllowed(var)
        }
    }
    impl ::std::convert::From<SelfPermitAllowedIfNecessaryCall> for UniswapRouterCalls {
        fn from(var: SelfPermitAllowedIfNecessaryCall) -> Self {
            UniswapRouterCalls::SelfPermitAllowedIfNecessary(var)
        }
    }
    impl ::std::convert::From<SelfPermitIfNecessaryCall> for UniswapRouterCalls {
        fn from(var: SelfPermitIfNecessaryCall) -> Self {
            UniswapRouterCalls::SelfPermitIfNecessary(var)
        }
    }
    impl ::std::convert::From<SweepTokenCall> for UniswapRouterCalls {
        fn from(var: SweepTokenCall) -> Self {
            UniswapRouterCalls::SweepToken(var)
        }
    }
    impl ::std::convert::From<SweepTokenWithFeeCall> for UniswapRouterCalls {
        fn from(var: SweepTokenWithFeeCall) -> Self {
            UniswapRouterCalls::SweepTokenWithFee(var)
        }
    }
    impl ::std::convert::From<UniswapV3SwapCallbackCall> for UniswapRouterCalls {
        fn from(var: UniswapV3SwapCallbackCall) -> Self {
            UniswapRouterCalls::UniswapV3SwapCallback(var)
        }
    }
    impl ::std::convert::From<UnwrapWETH9Call> for UniswapRouterCalls {
        fn from(var: UnwrapWETH9Call) -> Self {
            UniswapRouterCalls::UnwrapWETH9(var)
        }
    }
    impl ::std::convert::From<UnwrapWETH9WithFeeCall> for UniswapRouterCalls {
        fn from(var: UnwrapWETH9WithFeeCall) -> Self {
            UniswapRouterCalls::UnwrapWETH9WithFee(var)
        }
    }
    #[doc = "`ExactInputParams(bytes,address,uint256,uint256,uint256)`"]
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
    pub struct ExactInputParams {
        pub path: Vec<u8>,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_minimum: ethers::core::types::U256,
    }
    #[doc = "`ExactInputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
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
    pub struct ExactInputSingleParams {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub fee: u32,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub amount_in: ethers::core::types::U256,
        pub amount_out_minimum: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
    #[doc = "`ExactOutputParams(bytes,address,uint256,uint256,uint256)`"]
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
    pub struct ExactOutputParams {
        pub path: Vec<u8>,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
        pub amount_in_maximum: ethers::core::types::U256,
    }
    #[doc = "`ExactOutputSingleParams(address,address,uint24,address,uint256,uint256,uint256,uint160)`"]
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
    pub struct ExactOutputSingleParams {
        pub token_in: ethers::core::types::Address,
        pub token_out: ethers::core::types::Address,
        pub fee: u32,
        pub recipient: ethers::core::types::Address,
        pub deadline: ethers::core::types::U256,
        pub amount_out: ethers::core::types::U256,
        pub amount_in_maximum: ethers::core::types::U256,
        pub sqrt_price_limit_x96: ethers::core::types::U256,
    }
}
