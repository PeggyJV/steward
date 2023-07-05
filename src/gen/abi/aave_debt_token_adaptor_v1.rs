pub use aavedebttokenadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod aavedebttokenadaptorv1_mod {
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
    #[doc = "AaveDebtTokenAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static AAVEDEBTTOKENADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address\",\n                \"name\": \"untrackedDebtPosition\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"AaveDebtTokenAdaptor__DebtPositionsMustBeTracked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"AaveDebtTokenAdaptor__HealthFactorTooLow\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__BadSlippage\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExchangeNotSupported\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"assetsUsed\",\n        \"outputs\": [\n            {\n                \"internalType\": \"contract ERC20[]\",\n                \"name\": \"assets\",\n                \"type\": \"address[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"balanceOf\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"debtTokenToBorrow\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToBorrow\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"borrowFromAave\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"deposit\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"address[]\",\n                \"name\": \"loanToken\",\n                \"type\": \"address[]\"\n            },\n            {\n                \"internalType\": \"uint256[]\",\n                \"name\": \"loanAmount\",\n                \"type\": \"uint256[]\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"flashLoan\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"identifier\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bytes32\",\n                \"name\": \"\",\n                \"type\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [],\n        \"name\": \"isDebt\",\n        \"outputs\": [\n            {\n                \"internalType\": \"bool\",\n                \"name\": \"\",\n                \"type\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"uint64\",\n                \"name\": \"slippage\",\n                \"type\": \"uint64\"\n            }\n        ],\n        \"name\": \"oracleSwap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenToRepay\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountToRepay\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"repayAaveDebt\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"asset\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"spender\",\n                \"type\": \"address\"\n            }\n        ],\n        \"name\": \"revokeApproval\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"assetOut\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swap\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountOut\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenIn\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"contract ERC20\",\n                \"name\": \"tokenToRepay\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"amountIn\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"enum SwapRouter.Exchange\",\n                \"name\": \"exchange\",\n                \"type\": \"uint8\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"params\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"swapAndRepay\",\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"address\",\n                \"name\": \"\",\n                \"type\": \"address\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdraw\",\n        \"outputs\": [],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            },\n            {\n                \"internalType\": \"bytes\",\n                \"name\": \"\",\n                \"type\": \"bytes\"\n            }\n        ],\n        \"name\": \"withdrawableFrom\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct AaveDebtTokenAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for AaveDebtTokenAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for AaveDebtTokenAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(AaveDebtTokenAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> AaveDebtTokenAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                AAVEDEBTTOKENADAPTORV1_ABI.clone(),
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
    pub enum AaveDebtTokenAdaptorV1Calls {
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
    impl ethers::core::abi::AbiDecode for AaveDebtTokenAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BorrowFromAaveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::BorrowFromAave(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <FlashLoanCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::FlashLoan(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <OracleSwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::OracleSwap(decoded));
            }
            if let Ok(decoded) =
                <RepayAaveDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::RepayAaveDebt(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SwapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(AaveDebtTokenAdaptorV1Calls::Swap(decoded));
            }
            if let Ok(decoded) =
                <SwapAndRepayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::SwapAndRepay(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(AaveDebtTokenAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for AaveDebtTokenAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                AaveDebtTokenAdaptorV1Calls::AssetOf(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::BalanceOf(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::BorrowFromAave(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::Deposit(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::FlashLoan(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::Identifier(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::IsDebt(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::OracleSwap(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::RepayAaveDebt(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::Swap(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::SwapAndRepay(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::Withdraw(element) => element.encode(),
                AaveDebtTokenAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for AaveDebtTokenAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                AaveDebtTokenAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::BorrowFromAave(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::Deposit(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::FlashLoan(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::Identifier(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::OracleSwap(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::RepayAaveDebt(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::Swap(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::SwapAndRepay(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                AaveDebtTokenAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AssetOfCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BorrowFromAaveCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: BorrowFromAaveCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::BorrowFromAave(var)
        }
    }
    impl ::std::convert::From<DepositCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<FlashLoanCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: FlashLoanCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::FlashLoan(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<OracleSwapCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: OracleSwapCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::OracleSwap(var)
        }
    }
    impl ::std::convert::From<RepayAaveDebtCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: RepayAaveDebtCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::RepayAaveDebt(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SwapCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: SwapCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::Swap(var)
        }
    }
    impl ::std::convert::From<SwapAndRepayCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: SwapAndRepayCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::SwapAndRepay(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for AaveDebtTokenAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            AaveDebtTokenAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
}
