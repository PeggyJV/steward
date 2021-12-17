pub use aavecellar_mod::*;
#[allow(clippy::too_many_arguments)]
mod aavecellar_mod {
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
    #[doc = "AaveCellar was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVECELLAR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"name\": \"_from\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"name\": \"_to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Transfer\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"anonymous\": false,\r\n        \"inputs\": [\r\n            {\r\n                \"indexed\": true,\r\n                \"name\": \"_owner\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": true,\r\n                \"name\": \"_spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"indexed\": false,\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"Approval\",\r\n        \"type\": \"event\"\r\n    },\r\n    {\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_name\",\r\n                \"type\": \"string\"\r\n            },\r\n            {\r\n                \"name\": \"_symbol\",\r\n                \"type\": \"string\"\r\n            },\r\n            {\r\n                \"name\": \"_lendingPool\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_uToken\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"constructor\"\r\n    },\r\n    {\r\n        \"gas\": 426,\r\n        \"inputs\": [],\r\n        \"name\": \"decimals\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"pure\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 76329,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"transfer\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 112771,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_from\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_to\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"transferFrom\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 40418,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"approve\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 40554,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"increaseAllowance\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 40578,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_spender\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"_value\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"decreaseAllowance\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"bool\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 99670,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"deposit\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 83479,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"amount\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"withdraw\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 168933,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"route\",\r\n                \"type\": \"bytes\"\r\n            },\r\n            {\r\n                \"name\": \"minPrice\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"reinvest\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 36612,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_lendingPool\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"setLendingPool\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 36485,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"_serviceFee\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"name\": \"setServiceFee\",\r\n        \"outputs\": [],\r\n        \"stateMutability\": \"nonpayable\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 8023,\r\n        \"inputs\": [],\r\n        \"name\": \"name\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"string\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 7076,\r\n        \"inputs\": [],\r\n        \"name\": \"symbol\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"string\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1888,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"arg0\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"balanceOf\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 2190,\r\n        \"inputs\": [\r\n            {\r\n                \"name\": \"arg0\",\r\n                \"type\": \"address\"\r\n            },\r\n            {\r\n                \"name\": \"arg1\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"name\": \"allowance\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1676,\r\n        \"inputs\": [],\r\n        \"name\": \"totalSupply\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1706,\r\n        \"inputs\": [],\r\n        \"name\": \"u_token\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1736,\r\n        \"inputs\": [],\r\n        \"name\": \"a_token\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1766,\r\n        \"inputs\": [],\r\n        \"name\": \"lendingPool\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1796,\r\n        \"inputs\": [],\r\n        \"name\": \"owner\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"address\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    },\r\n    {\r\n        \"gas\": 1826,\r\n        \"inputs\": [],\r\n        \"name\": \"serviceFee\",\r\n        \"outputs\": [\r\n            {\r\n                \"name\": \"\",\r\n                \"type\": \"uint256\"\r\n            }\r\n        ],\r\n        \"stateMutability\": \"view\",\r\n        \"type\": \"function\"\r\n    }\r\n]") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AaveCellar<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AaveCellar<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveCellar<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveCellar))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AaveCellar<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), AAVECELLAR_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `a_token` (0x00e5f3c4) function"]
        pub fn a_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([0, 229, 243, 196], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(
            &self,
            arg_0: ethers::core::types::Address,
            arg_1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (arg_0, arg_1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            arg_0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], arg_0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deposit` (0xb6b55f25) function"]
        pub fn deposit(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 181, 95, 37], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lendingPool` (0xa59a9973) function"]
        pub fn lending_pool(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([165, 154, 153, 115], ())
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
        #[doc = "Calls the contract's `reinvest` (0xd200a692) function"]
        pub fn reinvest(
            &self,
            route: ethers::core::types::Bytes,
            min_price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 0, 166, 146], (route, min_price))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `serviceFee` (0x8abdf5aa) function"]
        pub fn service_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([138, 189, 245, 170], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLendingPool` (0x113aa8b1) function"]
        pub fn set_lending_pool(
            &self,
            lending_pool: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([17, 58, 168, 177], lending_pool)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setServiceFee` (0x5cdf76f8) function"]
        pub fn set_service_fee(
            &self,
            service_fee: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 223, 118, 248], service_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
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
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `u_token` (0x10aace9b) function"]
        pub fn u_token(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([16, 170, 206, 155], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x2e1a7d4d) function"]
        pub fn withdraw(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, AaveCellarEvents> {
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveCellarEvents {
        ApprovalFilter(ApprovalFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for AaveCellarEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AaveCellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AaveCellarEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for AaveCellarEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveCellarEvents::ApprovalFilter(element) => element.fmt(f),
                AaveCellarEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `a_token`function with signature `a_token()` and selector `[0, 229, 243, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "a_token", abi = "a_token()")]
    pub struct ATokenCall;
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
        pub arg_0: ethers::core::types::Address,
        pub arg_1: ethers::core::types::Address,
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
        pub value: ethers::core::types::U256,
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
        pub arg_0: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `decreaseAllowance`function with signature `decreaseAllowance(address,uint256)` and selector `[164, 87, 194, 215]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `deposit`function with signature `deposit(uint256)` and selector `[182, 181, 95, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256)")]
    pub struct DepositCall {
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `increaseAllowance`function with signature `increaseAllowance(address,uint256)` and selector `[57, 80, 147, 81]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `lendingPool`function with signature `lendingPool()` and selector `[165, 154, 153, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "lendingPool", abi = "lendingPool()")]
    pub struct LendingPoolCall;
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
    #[doc = "Container type for all input parameters for the `reinvest`function with signature `reinvest(bytes,uint256)` and selector `[210, 0, 166, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "reinvest", abi = "reinvest(bytes,uint256)")]
    pub struct ReinvestCall {
        pub route: ethers::core::types::Bytes,
        pub min_price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `serviceFee`function with signature `serviceFee()` and selector `[138, 189, 245, 170]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "serviceFee", abi = "serviceFee()")]
    pub struct ServiceFeeCall;
    #[doc = "Container type for all input parameters for the `setLendingPool`function with signature `setLendingPool(address)` and selector `[17, 58, 168, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setLendingPool", abi = "setLendingPool(address)")]
    pub struct SetLendingPoolCall {
        pub lending_pool: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setServiceFee`function with signature `setServiceFee(uint256)` and selector `[92, 223, 118, 248]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "setServiceFee", abi = "setServiceFee(uint256)")]
    pub struct SetServiceFeeCall {
        pub service_fee: ethers::core::types::U256,
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
        pub value: ethers::core::types::U256,
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
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `u_token`function with signature `u_token()` and selector `[16, 170, 206, 155]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "u_token", abi = "u_token()")]
    pub struct UTokenCall;
    #[doc = "Container type for all input parameters for the `withdraw`function with signature `withdraw(uint256)` and selector `[46, 26, 125, 77]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum AaveCellarCalls {
        AToken(ATokenCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        Deposit(DepositCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        LendingPool(LendingPoolCall),
        Name(NameCall),
        Owner(OwnerCall),
        Reinvest(ReinvestCall),
        ServiceFee(ServiceFeeCall),
        SetLendingPool(SetLendingPoolCall),
        SetServiceFee(SetServiceFeeCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        UToken(UTokenCall),
        Withdraw(WithdrawCall),
    }
    impl ethers::core::abi::AbiDecode for AaveCellarCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <ATokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::AToken(decoded));
            }
            if let Ok(decoded) =
                <AllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Allowance(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DecimalsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DecreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <IncreaseAllowanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) =
                <LendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::LendingPool(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveCellarCalls::Name(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <ReinvestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Reinvest(decoded));
            }
            if let Ok(decoded) =
                <ServiceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::ServiceFee(decoded));
            }
            if let Ok(decoded) =
                <SetLendingPoolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::SetLendingPool(decoded));
            }
            if let Ok(decoded) =
                <SetServiceFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::SetServiceFee(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Transfer(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) = <UTokenCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::UToken(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveCellarCalls::Withdraw(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveCellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveCellarCalls::AToken(element) => element.encode(),
                AaveCellarCalls::Allowance(element) => element.encode(),
                AaveCellarCalls::Approve(element) => element.encode(),
                AaveCellarCalls::BalanceOf(element) => element.encode(),
                AaveCellarCalls::Decimals(element) => element.encode(),
                AaveCellarCalls::DecreaseAllowance(element) => element.encode(),
                AaveCellarCalls::Deposit(element) => element.encode(),
                AaveCellarCalls::IncreaseAllowance(element) => element.encode(),
                AaveCellarCalls::LendingPool(element) => element.encode(),
                AaveCellarCalls::Name(element) => element.encode(),
                AaveCellarCalls::Owner(element) => element.encode(),
                AaveCellarCalls::Reinvest(element) => element.encode(),
                AaveCellarCalls::ServiceFee(element) => element.encode(),
                AaveCellarCalls::SetLendingPool(element) => element.encode(),
                AaveCellarCalls::SetServiceFee(element) => element.encode(),
                AaveCellarCalls::Symbol(element) => element.encode(),
                AaveCellarCalls::TotalSupply(element) => element.encode(),
                AaveCellarCalls::Transfer(element) => element.encode(),
                AaveCellarCalls::TransferFrom(element) => element.encode(),
                AaveCellarCalls::UToken(element) => element.encode(),
                AaveCellarCalls::Withdraw(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveCellarCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveCellarCalls::AToken(element) => element.fmt(f),
                AaveCellarCalls::Allowance(element) => element.fmt(f),
                AaveCellarCalls::Approve(element) => element.fmt(f),
                AaveCellarCalls::BalanceOf(element) => element.fmt(f),
                AaveCellarCalls::Decimals(element) => element.fmt(f),
                AaveCellarCalls::DecreaseAllowance(element) => element.fmt(f),
                AaveCellarCalls::Deposit(element) => element.fmt(f),
                AaveCellarCalls::IncreaseAllowance(element) => element.fmt(f),
                AaveCellarCalls::LendingPool(element) => element.fmt(f),
                AaveCellarCalls::Name(element) => element.fmt(f),
                AaveCellarCalls::Owner(element) => element.fmt(f),
                AaveCellarCalls::Reinvest(element) => element.fmt(f),
                AaveCellarCalls::ServiceFee(element) => element.fmt(f),
                AaveCellarCalls::SetLendingPool(element) => element.fmt(f),
                AaveCellarCalls::SetServiceFee(element) => element.fmt(f),
                AaveCellarCalls::Symbol(element) => element.fmt(f),
                AaveCellarCalls::TotalSupply(element) => element.fmt(f),
                AaveCellarCalls::Transfer(element) => element.fmt(f),
                AaveCellarCalls::TransferFrom(element) => element.fmt(f),
                AaveCellarCalls::UToken(element) => element.fmt(f),
                AaveCellarCalls::Withdraw(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ATokenCall> for AaveCellarCalls {
        fn from(var: ATokenCall) -> Self {
            AaveCellarCalls::AToken(var)
        }
    }
    impl ::std::convert::From<AllowanceCall> for AaveCellarCalls {
        fn from(var: AllowanceCall) -> Self {
            AaveCellarCalls::Allowance(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for AaveCellarCalls {
        fn from(var: ApproveCall) -> Self {
            AaveCellarCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for AaveCellarCalls {
        fn from(var: BalanceOfCall) -> Self {
            AaveCellarCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DecimalsCall> for AaveCellarCalls {
        fn from(var: DecimalsCall) -> Self {
            AaveCellarCalls::Decimals(var)
        }
    }
    impl ::std::convert::From<DecreaseAllowanceCall> for AaveCellarCalls {
        fn from(var: DecreaseAllowanceCall) -> Self {
            AaveCellarCalls::DecreaseAllowance(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveCellarCalls {
        fn from(var: DepositCall) -> Self {
            AaveCellarCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<IncreaseAllowanceCall> for AaveCellarCalls {
        fn from(var: IncreaseAllowanceCall) -> Self {
            AaveCellarCalls::IncreaseAllowance(var)
        }
    }
    impl ::std::convert::From<LendingPoolCall> for AaveCellarCalls {
        fn from(var: LendingPoolCall) -> Self {
            AaveCellarCalls::LendingPool(var)
        }
    }
    impl ::std::convert::From<NameCall> for AaveCellarCalls {
        fn from(var: NameCall) -> Self {
            AaveCellarCalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for AaveCellarCalls {
        fn from(var: OwnerCall) -> Self {
            AaveCellarCalls::Owner(var)
        }
    }
    impl ::std::convert::From<ReinvestCall> for AaveCellarCalls {
        fn from(var: ReinvestCall) -> Self {
            AaveCellarCalls::Reinvest(var)
        }
    }
    impl ::std::convert::From<ServiceFeeCall> for AaveCellarCalls {
        fn from(var: ServiceFeeCall) -> Self {
            AaveCellarCalls::ServiceFee(var)
        }
    }
    impl ::std::convert::From<SetLendingPoolCall> for AaveCellarCalls {
        fn from(var: SetLendingPoolCall) -> Self {
            AaveCellarCalls::SetLendingPool(var)
        }
    }
    impl ::std::convert::From<SetServiceFeeCall> for AaveCellarCalls {
        fn from(var: SetServiceFeeCall) -> Self {
            AaveCellarCalls::SetServiceFee(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for AaveCellarCalls {
        fn from(var: SymbolCall) -> Self {
            AaveCellarCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for AaveCellarCalls {
        fn from(var: TotalSupplyCall) -> Self {
            AaveCellarCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferCall> for AaveCellarCalls {
        fn from(var: TransferCall) -> Self {
            AaveCellarCalls::Transfer(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for AaveCellarCalls {
        fn from(var: TransferFromCall) -> Self {
            AaveCellarCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<UTokenCall> for AaveCellarCalls {
        fn from(var: UTokenCall) -> Self {
            AaveCellarCalls::UToken(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AaveCellarCalls {
        fn from(var: WithdrawCall) -> Self {
            AaveCellarCalls::Withdraw(var)
        }
    }
}
