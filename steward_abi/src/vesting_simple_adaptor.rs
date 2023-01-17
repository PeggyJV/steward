pub use vestingsimpleadaptor_mod::*;
#[allow(clippy::too_many_arguments)]
mod vestingsimpleadaptor_mod {
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
    #[doc = "VestingSimpleAdaptor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VESTINGSIMPLEADAPTOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__BadSlippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExchangeNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract VestingSimple\",\n                \"name\": \"vestingContract\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToDeposit\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"depositToVesting\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"slippage\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"oracleSwap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"assets\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"receiver\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract VestingSimple\",\n                \"name\": \"vestingContract\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"withdrawAllFromVesting\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract VestingSimple\",\n                \"name\": \"vestingContract\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToWithdraw\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"withdrawAnyFromVesting\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract VestingSimple\",\n                \"name\": \"vestingContract\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"depositId\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToWithdraw\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"withdrawFromVesting\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct VestingSimpleAdaptor<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for VestingSimpleAdaptor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for VestingSimpleAdaptor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VestingSimpleAdaptor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> VestingSimpleAdaptor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                VESTINGSIMPLEADAPTOR_ABI.clone(),
                client,
            );
            Self(contract)
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
        #[doc = "Calls the contract's `depositToVesting` (0xe0464e30) function"]
        pub fn deposit_to_vesting(
            &self,
            vesting_contract: ethers::core::types::Address,
            amount_to_deposit: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 70, 78, 48], (vesting_contract, amount_to_deposit))
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
        #[doc = "Calls the contract's `withdraw` (0xc9111bd7) function"]
        pub fn withdraw(
            &self,
            assets: ethers::core::types::U256,
            receiver: ethers::core::types::Address,
            adaptor_data: ethers::core::types::Bytes,
            p3: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 17, 27, 215], (assets, receiver, adaptor_data, p3))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAllFromVesting` (0x62cb2f26) function"]
        pub fn withdraw_all_from_vesting(
            &self,
            vesting_contract: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 203, 47, 38], vesting_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawAnyFromVesting` (0xe46aff38) function"]
        pub fn withdraw_any_from_vesting(
            &self,
            vesting_contract: ethers::core::types::Address,
            amount_to_withdraw: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 106, 255, 56], (vesting_contract, amount_to_withdraw))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromVesting` (0x89e19754) function"]
        pub fn withdraw_from_vesting(
            &self,
            vesting_contract: ethers::core::types::Address,
            deposit_id: ethers::core::types::U256,
            amount_to_withdraw: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [137, 225, 151, 84],
                    (vesting_contract, deposit_id, amount_to_withdraw),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawableFrom` (0xfa50e5d2) function"]
        pub fn withdrawable_from(
            &self,
            adaptor_data: ethers::core::types::Bytes,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (adaptor_data, p1))
                .expect("method not found (this should never happen)")
        }
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
    #[doc = "Container type for all input parameters for the `depositToVesting`function with signature `depositToVesting(address,uint256)` and selector `[224, 70, 78, 48]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        serde :: Deserialize,
        serde :: Serialize,
    )]
    #[ethcall(name = "depositToVesting", abi = "depositToVesting(address,uint256)")]
    pub struct DepositToVestingCall {
        pub vesting_contract: ethers::core::types::Address,
        pub amount_to_deposit: ethers::core::types::U256,
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
        pub p3: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `withdrawAllFromVesting`function with signature `withdrawAllFromVesting(address)` and selector `[98, 203, 47, 38]`"]
    #[derive(
        Clone,
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
        name = "withdrawAllFromVesting",
        abi = "withdrawAllFromVesting(address)"
    )]
    pub struct WithdrawAllFromVestingCall {
        pub vesting_contract: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `withdrawAnyFromVesting`function with signature `withdrawAnyFromVesting(address,uint256)` and selector `[228, 106, 255, 56]`"]
    #[derive(
        Clone,
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
        name = "withdrawAnyFromVesting",
        abi = "withdrawAnyFromVesting(address,uint256)"
    )]
    pub struct WithdrawAnyFromVestingCall {
        pub vesting_contract: ethers::core::types::Address,
        pub amount_to_withdraw: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `withdrawFromVesting`function with signature `withdrawFromVesting(address,uint256,uint256)` and selector `[137, 225, 151, 84]`"]
    #[derive(
        Clone,
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
        name = "withdrawFromVesting",
        abi = "withdrawFromVesting(address,uint256,uint256)"
    )]
    pub struct WithdrawFromVestingCall {
        pub vesting_contract: ethers::core::types::Address,
        pub deposit_id: ethers::core::types::U256,
        pub amount_to_withdraw: ethers::core::types::U256,
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
        pub p1: ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum VestingSimpleAdaptorCalls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Deposit(DepositCall),
        DepositToVesting(DepositToVestingCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        OracleSwap(OracleSwapCall),
        RevokeApproval(RevokeApprovalCall),
        Swap(SwapCall),
        Withdraw(WithdrawCall),
        WithdrawAllFromVesting(WithdrawAllFromVestingCall),
        WithdrawAnyFromVesting(WithdrawAnyFromVestingCall),
        WithdrawFromVesting(WithdrawFromVestingCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for VestingSimpleAdaptorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DepositToVestingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::DepositToVesting(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <OracleSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::OracleSwap(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(VestingSimpleAdaptorCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAllFromVestingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::WithdrawAllFromVesting(decoded));
            }
            if let Ok(decoded) =
                <WithdrawAnyFromVestingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(decoded));
            }
            if let Ok(decoded) =
                <WithdrawFromVestingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::WithdrawFromVesting(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(VestingSimpleAdaptorCalls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for VestingSimpleAdaptorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                VestingSimpleAdaptorCalls::AssetOf(element) => element.encode(),
                VestingSimpleAdaptorCalls::AssetsUsed(element) => element.encode(),
                VestingSimpleAdaptorCalls::BalanceOf(element) => element.encode(),
                VestingSimpleAdaptorCalls::Deposit(element) => element.encode(),
                VestingSimpleAdaptorCalls::DepositToVesting(element) => element.encode(),
                VestingSimpleAdaptorCalls::Identifier(element) => element.encode(),
                VestingSimpleAdaptorCalls::IsDebt(element) => element.encode(),
                VestingSimpleAdaptorCalls::OracleSwap(element) => element.encode(),
                VestingSimpleAdaptorCalls::RevokeApproval(element) => element.encode(),
                VestingSimpleAdaptorCalls::Swap(element) => element.encode(),
                VestingSimpleAdaptorCalls::Withdraw(element) => element.encode(),
                VestingSimpleAdaptorCalls::WithdrawAllFromVesting(element) => element.encode(),
                VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(element) => element.encode(),
                VestingSimpleAdaptorCalls::WithdrawFromVesting(element) => element.encode(),
                VestingSimpleAdaptorCalls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for VestingSimpleAdaptorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                VestingSimpleAdaptorCalls::AssetOf(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::AssetsUsed(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::BalanceOf(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::Deposit(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::DepositToVesting(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::Identifier(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::IsDebt(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::OracleSwap(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::RevokeApproval(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::Swap(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::Withdraw(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::WithdrawAllFromVesting(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::WithdrawFromVesting(element) => element.fmt(f),
                VestingSimpleAdaptorCalls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetOfCall> for VestingSimpleAdaptorCalls {
        fn from(var: AssetOfCall) -> Self {
            VestingSimpleAdaptorCalls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for VestingSimpleAdaptorCalls {
        fn from(var: AssetsUsedCall) -> Self {
            VestingSimpleAdaptorCalls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for VestingSimpleAdaptorCalls {
        fn from(var: BalanceOfCall) -> Self {
            VestingSimpleAdaptorCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<DepositCall> for VestingSimpleAdaptorCalls {
        fn from(var: DepositCall) -> Self {
            VestingSimpleAdaptorCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<DepositToVestingCall> for VestingSimpleAdaptorCalls {
        fn from(var: DepositToVestingCall) -> Self {
            VestingSimpleAdaptorCalls::DepositToVesting(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for VestingSimpleAdaptorCalls {
        fn from(var: IdentifierCall) -> Self {
            VestingSimpleAdaptorCalls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for VestingSimpleAdaptorCalls {
        fn from(var: IsDebtCall) -> Self {
            VestingSimpleAdaptorCalls::IsDebt(var)
        }
    }
    impl ::std::convert::From<OracleSwapCall> for VestingSimpleAdaptorCalls {
        fn from(var: OracleSwapCall) -> Self {
            VestingSimpleAdaptorCalls::OracleSwap(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for VestingSimpleAdaptorCalls {
        fn from(var: RevokeApprovalCall) -> Self {
            VestingSimpleAdaptorCalls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SwapCall> for VestingSimpleAdaptorCalls {
        fn from(var: SwapCall) -> Self {
            VestingSimpleAdaptorCalls::Swap(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for VestingSimpleAdaptorCalls {
        fn from(var: WithdrawCall) -> Self {
            VestingSimpleAdaptorCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawAllFromVestingCall> for VestingSimpleAdaptorCalls {
        fn from(var: WithdrawAllFromVestingCall) -> Self {
            VestingSimpleAdaptorCalls::WithdrawAllFromVesting(var)
        }
    }
    impl ::std::convert::From<WithdrawAnyFromVestingCall> for VestingSimpleAdaptorCalls {
        fn from(var: WithdrawAnyFromVestingCall) -> Self {
            VestingSimpleAdaptorCalls::WithdrawAnyFromVesting(var)
        }
    }
    impl ::std::convert::From<WithdrawFromVestingCall> for VestingSimpleAdaptorCalls {
        fn from(var: WithdrawFromVestingCall) -> Self {
            VestingSimpleAdaptorCalls::WithdrawFromVesting(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for VestingSimpleAdaptorCalls {
        fn from(var: WithdrawableFromCall) -> Self {
            VestingSimpleAdaptorCalls::WithdrawableFrom(var)
        }
    }
}
