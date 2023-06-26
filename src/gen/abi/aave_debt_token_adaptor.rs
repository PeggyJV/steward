pub use aavedebttokenadaptor_mod::*;
#[allow(clippy::too_many_arguments)]
mod aavedebttokenadaptor_mod {
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
    #[doc = "AaveDebtTokenAdaptor was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVEDEBTTOKENADAPTOR_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"untrackedDebtPosition\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"AaveDebtTokenAdaptor__DebtPositionsMustBeTracked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"AaveDebtTokenAdaptor__HealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__BadSlippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExchangeNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"debtTokenToBorrow\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToBorrow\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"borrowFromAave\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"loanToken\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"loanAmount\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"flashLoan\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"slippage\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"oracleSwap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenToRepay\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToRepay\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"repayAaveDebt\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenToRepay\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swapAndRepay\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AaveDebtTokenAdaptor<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AaveDebtTokenAdaptor<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveDebtTokenAdaptor<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveDebtTokenAdaptor))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AaveDebtTokenAdaptor<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                AAVEDEBTTOKENADAPTOR_ABI.clone(),
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
        #[doc = "Calls the contract's `borrowFromAave` (0x9a85e1d3) function"]
        pub fn borrow_from_aave(
            &self,
            debt_token_to_borrow: ethers::core::types::Address,
            amount_to_borrow: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 133, 225, 211],
                    (debt_token_to_borrow, amount_to_borrow),
                )
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
        #[doc = "Calls the contract's `flashLoan` (0x788fb484) function"]
        pub fn flash_loan(
            &self,
            loan_token: ::std::vec::Vec<ethers::core::types::Address>,
            loan_amount: ::std::vec::Vec<ethers::core::types::U256>,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 143, 180, 132], (loan_token, loan_amount, params))
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
        #[doc = "Calls the contract's `repayAaveDebt` (0xd70c1539) function"]
        pub fn repay_aave_debt(
            &self,
            token_to_repay: ethers::core::types::Address,
            amount_to_repay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 12, 21, 57], (token_to_repay, amount_to_repay))
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
        #[doc = "Calls the contract's `swapAndRepay` (0x642c2d8e) function"]
        pub fn swap_and_repay(
            &self,
            token_in: ethers::core::types::Address,
            token_to_repay: ethers::core::types::Address,
            amount_in: ethers::core::types::U256,
            exchange: u8,
            params: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [100, 44, 45, 142],
                    (token_in, token_to_repay, amount_in, exchange, params),
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
    #[doc = "Container type for all input parameters for the `borrowFromAave`function with signature `borrowFromAave(address,uint256)` and selector `[154, 133, 225, 211]`"]
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
    #[ethcall(name = "borrowFromAave", abi = "borrowFromAave(address,uint256)")]
    pub struct BorrowFromAaveCall {
        pub debt_token_to_borrow: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `flashLoan`function with signature `flashLoan(address[],uint256[],bytes)` and selector `[120, 143, 180, 132]`"]
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
    #[ethcall(name = "flashLoan", abi = "flashLoan(address[],uint256[],bytes)")]
    pub struct FlashLoanCall {
        pub loan_token: ::std::vec::Vec<ethers::core::types::Address>,
        pub loan_amount: ::std::vec::Vec<ethers::core::types::U256>,
        pub params: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `repayAaveDebt`function with signature `repayAaveDebt(address,uint256)` and selector `[215, 12, 21, 57]`"]
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
    #[ethcall(name = "repayAaveDebt", abi = "repayAaveDebt(address,uint256)")]
    pub struct RepayAaveDebtCall {
        pub token_to_repay: ethers::core::types::Address,
        pub amount_to_repay: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `swapAndRepay`function with signature `swapAndRepay(address,address,uint256,uint8,bytes)` and selector `[100, 44, 45, 142]`"]
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
        name = "swapAndRepay",
        abi = "swapAndRepay(address,address,uint256,uint8,bytes)"
    )]
    pub struct SwapAndRepayCall {
        pub token_in: ethers::core::types::Address,
        pub token_to_repay: ethers::core::types::Address,
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
    pub enum AaveDebtTokenAdaptorCalls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BorrowFromAave(BorrowFromAaveCall),
        Deposit(DepositCall),
        FlashLoan(FlashLoanCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        OracleSwap(OracleSwapCall),
        RepayAaveDebt(RepayAaveDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Swap(SwapCall),
        SwapAndRepay(SwapAndRepayCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ethers::core::abi::AbiDecode for AaveDebtTokenAdaptorCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowFromAaveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::BorrowFromAave(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <OracleSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::OracleSwap(decoded));
            }
            if let Ok(decoded) =
                <RepayAaveDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::RepayAaveDebt(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveDebtTokenAdaptorCalls::Swap(decoded));
            }
            if let Ok(decoded) =
                <SwapAndRepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::SwapAndRepay(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorCalls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveDebtTokenAdaptorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveDebtTokenAdaptorCalls::AssetOf(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::AssetsUsed(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::BalanceOf(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::BorrowFromAave(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::Deposit(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::FlashLoan(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::Identifier(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::IsDebt(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::OracleSwap(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::RepayAaveDebt(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::RevokeApproval(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::Swap(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::SwapAndRepay(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::Withdraw(element) => element.encode(),
                AaveDebtTokenAdaptorCalls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveDebtTokenAdaptorCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveDebtTokenAdaptorCalls::AssetOf(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::AssetsUsed(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::BalanceOf(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::BorrowFromAave(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::Deposit(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::FlashLoan(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::Identifier(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::IsDebt(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::OracleSwap(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::RepayAaveDebt(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::RevokeApproval(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::Swap(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::SwapAndRepay(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::Withdraw(element) => element.fmt(f),
                AaveDebtTokenAdaptorCalls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetOfCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: AssetOfCall) -> Self {
            AaveDebtTokenAdaptorCalls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: AssetsUsedCall) -> Self {
            AaveDebtTokenAdaptorCalls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: BalanceOfCall) -> Self {
            AaveDebtTokenAdaptorCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowFromAaveCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: BorrowFromAaveCall) -> Self {
            AaveDebtTokenAdaptorCalls::BorrowFromAave(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: DepositCall) -> Self {
            AaveDebtTokenAdaptorCalls::Deposit(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: FlashLoanCall) -> Self {
            AaveDebtTokenAdaptorCalls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: IdentifierCall) -> Self {
            AaveDebtTokenAdaptorCalls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: IsDebtCall) -> Self {
            AaveDebtTokenAdaptorCalls::IsDebt(var)
        }
    }
    impl ::std::convert::From<OracleSwapCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: OracleSwapCall) -> Self {
            AaveDebtTokenAdaptorCalls::OracleSwap(var)
        }
    }
    impl ::std::convert::From<RepayAaveDebtCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: RepayAaveDebtCall) -> Self {
            AaveDebtTokenAdaptorCalls::RepayAaveDebt(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: RevokeApprovalCall) -> Self {
            AaveDebtTokenAdaptorCalls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SwapCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: SwapCall) -> Self {
            AaveDebtTokenAdaptorCalls::Swap(var)
        }
    }
    impl ::std::convert::From<SwapAndRepayCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: SwapAndRepayCall) -> Self {
            AaveDebtTokenAdaptorCalls::SwapAndRepay(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: WithdrawCall) -> Self {
            AaveDebtTokenAdaptorCalls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for AaveDebtTokenAdaptorCalls {
        fn from(var: WithdrawableFromCall) -> Self {
            AaveDebtTokenAdaptorCalls::WithdrawableFrom(var)
        }
    }
}
