pub use stakingadaptorv1_mod::*;
#[allow(clippy::too_many_arguments)]
mod stakingadaptorv1_mod {
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
    #[doc = "StakingAdaptorV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static STAKINGADAPTORV1_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"type\": \"constructor\",\n        \"inputs\": [\n            {\n                \"name\": \"_wrappedPrimitive\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"_maximumRequests\",\n                \"type\": \"uint8\",\n                \"internalType\": \"uint8\"\n            }\n        ],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"addRequestId\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"assetOf\",\n        \"inputs\": [\n            {\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"contract ERC20\"\n            }\n        ],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"assetsUsed\",\n        \"inputs\": [\n            {\n                \"name\": \"adaptorData\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"assets\",\n                \"type\": \"address[]\",\n                \"internalType\": \"contract ERC20[]\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"balanceOf\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"cancelBurn\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"completeBurn\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minAmountOut\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"deposit\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"getRequestIds\",\n        \"inputs\": [\n            {\n                \"name\": \"user\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256[]\",\n                \"internalType\": \"uint256[]\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"identifier\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bytes32\",\n                \"internalType\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"isDebt\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bool\",\n                \"internalType\": \"bool\"\n            }\n        ],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"lockedStoragePosition\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bytes32\",\n                \"internalType\": \"bytes32\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"mint\",\n        \"inputs\": [\n            {\n                \"name\": \"amount\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minAmountOut\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"mintERC20\",\n        \"inputs\": [\n            {\n                \"name\": \"depositAsset\",\n                \"type\": \"address\",\n                \"internalType\": \"contract ERC20\"\n            },\n            {\n                \"name\": \"amount\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minAmountOut\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"removeClaimedRequest\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"removeRequestId\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"requestBurn\",\n        \"inputs\": [\n            {\n                \"name\": \"amount\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"requestIds\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"revokeApproval\",\n        \"inputs\": [\n            {\n                \"name\": \"asset\",\n                \"type\": \"address\",\n                \"internalType\": \"contract ERC20\"\n            },\n            {\n                \"name\": \"spender\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"slippage\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint32\",\n                \"internalType\": \"uint32\"\n            }\n        ],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"unwrap\",\n        \"inputs\": [\n            {\n                \"name\": \"amount\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minAmountOut\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"withdraw\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"withdrawableFrom\",\n        \"inputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            },\n            {\n                \"name\": \"\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"pure\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"wrap\",\n        \"inputs\": [\n            {\n                \"name\": \"amount\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minAmountOut\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"wildcard\",\n                \"type\": \"bytes\",\n                \"internalType\": \"bytes\"\n            }\n        ],\n        \"outputs\": [],\n        \"stateMutability\": \"nonpayable\"\n    },\n    {\n        \"type\": \"function\",\n        \"name\": \"wrappedPrimitive\",\n        \"inputs\": [],\n        \"outputs\": [\n            {\n                \"name\": \"\",\n                \"type\": \"address\",\n                \"internalType\": \"contract IWETH9\"\n            }\n        ],\n        \"stateMutability\": \"view\"\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__ConstructorHealthFactorTooLow\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__ExternalReceiverBlocked\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__PricingNotSupported\",\n        \"inputs\": [\n            {\n                \"name\": \"asset\",\n                \"type\": \"address\",\n                \"internalType\": \"address\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__Slippage\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__UserDepositsNotAllowed\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"BaseAdaptor__UserWithdrawsNotAllowed\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__DuplicateRequest\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__MaximumRequestsExceeded\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__MinimumAmountNotMet\",\n        \"inputs\": [\n            {\n                \"name\": \"actual\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            },\n            {\n                \"name\": \"minimum\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__NotSupported\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__RequestNotClaimed\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__RequestNotFound\",\n        \"inputs\": [\n            {\n                \"name\": \"id\",\n                \"type\": \"uint256\",\n                \"internalType\": \"uint256\"\n            }\n        ]\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor__ZeroAmount\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor___Reentrancy\",\n        \"inputs\": []\n    },\n    {\n        \"type\": \"error\",\n        \"name\": \"StakingAdaptor___StorageSlotNotInitialized\",\n        \"inputs\": []\n    }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct StakingAdaptorV1<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for StakingAdaptorV1<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for StakingAdaptorV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(StakingAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> StakingAdaptorV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract = ethers::contract::Contract::new(
                address.into(),
                STAKINGADAPTORV1_ABI.clone(),
                client,
            );
            Self(contract)
        }
        #[doc = "Calls the contract's `addRequestId` (0x03c26e5f) function"]
        pub fn add_request_id(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 194, 110, 95], id)
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
            p0: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelBurn` (0xc618a073) function"]
        pub fn cancel_burn(
            &self,
            id: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 24, 160, 115], (id, wildcard))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `completeBurn` (0x713bffa0) function"]
        pub fn complete_burn(
            &self,
            id: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 59, 255, 160], (id, min_amount_out, wildcard))
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
        #[doc = "Calls the contract's `getRequestIds` (0x4761b6fb) function"]
        pub fn get_request_ids(
            &self,
            user: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([71, 97, 182, 251], user)
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
        #[doc = "Calls the contract's `mint` (0x08dc9f42) function"]
        pub fn mint(
            &self,
            amount: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 220, 159, 66], (amount, min_amount_out, wildcard))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mintERC20` (0xb70d6385) function"]
        pub fn mint_erc20(
            &self,
            deposit_asset: ethers::core::types::Address,
            amount: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [183, 13, 99, 133],
                    (deposit_asset, amount, min_amount_out, wildcard),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeClaimedRequest` (0xe6eaa89a) function"]
        pub fn remove_claimed_request(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 234, 168, 154], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeRequestId` (0xf5b71b7a) function"]
        pub fn remove_request_id(
            &self,
            id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 183, 27, 122], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requestBurn` (0x808adc39) function"]
        pub fn request_burn(
            &self,
            amount: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 138, 220, 57], (amount, wildcard))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `requestIds` (0xc1c3f908) function"]
        pub fn request_ids(
            &self,
            p0: ethers::core::types::Address,
            p1: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([193, 195, 249, 8], (p0, p1))
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
        #[doc = "Calls the contract's `unwrap` (0xeb8fa9e0) function"]
        pub fn unwrap(
            &self,
            amount: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 143, 169, 224], (amount, min_amount_out, wildcard))
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
        #[doc = "Calls the contract's `wrap` (0xc63594b3) function"]
        pub fn wrap(
            &self,
            amount: ethers::core::types::U256,
            min_amount_out: ethers::core::types::U256,
            wildcard: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 53, 148, 179], (amount, min_amount_out, wildcard))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wrappedPrimitive` (0x55c99d40) function"]
        pub fn wrapped_primitive(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([85, 201, 157, 64], ())
                .expect("method not found (this should never happen)")
        }
    }
    #[doc = "Container type for all input parameters for the `addRequestId`function with signature `addRequestId(uint256)` and selector `[3, 194, 110, 95]`"]
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
    #[ethcall(name = "addRequestId", abi = "addRequestId(uint256)")]
    pub struct AddRequestIdCall {
        pub id: ethers::core::types::U256,
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
    pub struct BalanceOfCall(pub ethers::core::types::Bytes);
    #[doc = "Container type for all input parameters for the `cancelBurn`function with signature `cancelBurn(uint256,bytes)` and selector `[198, 24, 160, 115]`"]
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
    #[ethcall(name = "cancelBurn", abi = "cancelBurn(uint256,bytes)")]
    pub struct CancelBurnCall {
        pub id: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `completeBurn`function with signature `completeBurn(uint256,uint256,bytes)` and selector `[113, 59, 255, 160]`"]
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
    #[ethcall(name = "completeBurn", abi = "completeBurn(uint256,uint256,bytes)")]
    pub struct CompleteBurnCall {
        pub id: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `getRequestIds`function with signature `getRequestIds(address)` and selector `[71, 97, 182, 251]`"]
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
    #[ethcall(name = "getRequestIds", abi = "getRequestIds(address)")]
    pub struct GetRequestIdsCall {
        pub user: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `mint`function with signature `mint(uint256,uint256,bytes)` and selector `[8, 220, 159, 66]`"]
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
    #[ethcall(name = "mint", abi = "mint(uint256,uint256,bytes)")]
    pub struct MintCall {
        pub amount: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `mintERC20`function with signature `mintERC20(address,uint256,uint256,bytes)` and selector `[183, 13, 99, 133]`"]
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
    #[ethcall(name = "mintERC20", abi = "mintERC20(address,uint256,uint256,bytes)")]
    pub struct MintERC20Call {
        pub deposit_asset: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `removeClaimedRequest`function with signature `removeClaimedRequest(uint256,bytes)` and selector `[230, 234, 168, 154]`"]
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
        name = "removeClaimedRequest",
        abi = "removeClaimedRequest(uint256,bytes)"
    )]
    pub struct RemoveClaimedRequestCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Bytes,
    );
    #[doc = "Container type for all input parameters for the `removeRequestId`function with signature `removeRequestId(uint256)` and selector `[245, 183, 27, 122]`"]
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
    #[ethcall(name = "removeRequestId", abi = "removeRequestId(uint256)")]
    pub struct RemoveRequestIdCall {
        pub id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `requestBurn`function with signature `requestBurn(uint256,bytes)` and selector `[128, 138, 220, 57]`"]
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
    #[ethcall(name = "requestBurn", abi = "requestBurn(uint256,bytes)")]
    pub struct RequestBurnCall {
        pub amount: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `requestIds`function with signature `requestIds(address,uint256)` and selector `[193, 195, 249, 8]`"]
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
    #[ethcall(name = "requestIds", abi = "requestIds(address,uint256)")]
    pub struct RequestIdsCall(
        pub ethers::core::types::Address,
        pub ethers::core::types::U256,
    );
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
    #[doc = "Container type for all input parameters for the `unwrap`function with signature `unwrap(uint256,uint256,bytes)` and selector `[235, 143, 169, 224]`"]
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
    #[ethcall(name = "unwrap", abi = "unwrap(uint256,uint256,bytes)")]
    pub struct UnwrapCall {
        pub amount: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
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
    #[doc = "Container type for all input parameters for the `wrap`function with signature `wrap(uint256,uint256,bytes)` and selector `[198, 53, 148, 179]`"]
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
    #[ethcall(name = "wrap", abi = "wrap(uint256,uint256,bytes)")]
    pub struct WrapCall {
        pub amount: ethers::core::types::U256,
        pub min_amount_out: ethers::core::types::U256,
        pub wildcard: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `wrappedPrimitive`function with signature `wrappedPrimitive()` and selector `[85, 201, 157, 64]`"]
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
    #[ethcall(name = "wrappedPrimitive", abi = "wrappedPrimitive()")]
    pub struct WrappedPrimitiveCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum StakingAdaptorV1Calls {
        AddRequestId(AddRequestIdCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        CancelBurn(CancelBurnCall),
        CompleteBurn(CompleteBurnCall),
        Deposit(DepositCall),
        GetRequestIds(GetRequestIdsCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        LockedStoragePosition(LockedStoragePositionCall),
        Mint(MintCall),
        MintERC20(MintERC20Call),
        RemoveClaimedRequest(RemoveClaimedRequestCall),
        RemoveRequestId(RemoveRequestIdCall),
        RequestBurn(RequestBurnCall),
        RequestIds(RequestIdsCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Unwrap(UnwrapCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
        Wrap(WrapCall),
        WrappedPrimitive(WrappedPrimitiveCall),
    }
    impl ethers::core::abi::AbiDecode for StakingAdaptorV1Calls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddRequestIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::AddRequestId(decoded));
            }
            if let Ok(decoded) =
                <AssetOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::AssetOf(decoded));
            }
            if let Ok(decoded) =
                <AssetsUsedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::AssetsUsed(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <CancelBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::CancelBurn(decoded));
            }
            if let Ok(decoded) =
                <CompleteBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::CompleteBurn(decoded));
            }
            if let Ok(decoded) =
                <DepositCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::Deposit(decoded));
            }
            if let Ok(decoded) =
                <GetRequestIdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::GetRequestIds(decoded));
            }
            if let Ok(decoded) =
                <IdentifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::IsDebt(decoded));
            }
            if let Ok(decoded) =
                <LockedStoragePositionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::LockedStoragePosition(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StakingAdaptorV1Calls::Mint(decoded));
            }
            if let Ok(decoded) =
                <MintERC20Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::MintERC20(decoded));
            }
            if let Ok(decoded) =
                <RemoveClaimedRequestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::RemoveClaimedRequest(decoded));
            }
            if let Ok(decoded) =
                <RemoveRequestIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::RemoveRequestId(decoded));
            }
            if let Ok(decoded) =
                <RequestBurnCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::RequestBurn(decoded));
            }
            if let Ok(decoded) =
                <RequestIdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::RequestIds(decoded));
            }
            if let Ok(decoded) =
                <RevokeApprovalCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::RevokeApproval(decoded));
            }
            if let Ok(decoded) =
                <SlippageCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::Slippage(decoded));
            }
            if let Ok(decoded) = <UnwrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::Unwrap(decoded));
            }
            if let Ok(decoded) =
                <WithdrawCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::Withdraw(decoded));
            }
            if let Ok(decoded) =
                <WithdrawableFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::WithdrawableFrom(decoded));
            }
            if let Ok(decoded) = <WrapCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StakingAdaptorV1Calls::Wrap(decoded));
            }
            if let Ok(decoded) =
                <WrappedPrimitiveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(StakingAdaptorV1Calls::WrappedPrimitive(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for StakingAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                StakingAdaptorV1Calls::AddRequestId(element) => element.encode(),
                StakingAdaptorV1Calls::AssetOf(element) => element.encode(),
                StakingAdaptorV1Calls::AssetsUsed(element) => element.encode(),
                StakingAdaptorV1Calls::BalanceOf(element) => element.encode(),
                StakingAdaptorV1Calls::CancelBurn(element) => element.encode(),
                StakingAdaptorV1Calls::CompleteBurn(element) => element.encode(),
                StakingAdaptorV1Calls::Deposit(element) => element.encode(),
                StakingAdaptorV1Calls::GetRequestIds(element) => element.encode(),
                StakingAdaptorV1Calls::Identifier(element) => element.encode(),
                StakingAdaptorV1Calls::IsDebt(element) => element.encode(),
                StakingAdaptorV1Calls::LockedStoragePosition(element) => element.encode(),
                StakingAdaptorV1Calls::Mint(element) => element.encode(),
                StakingAdaptorV1Calls::MintERC20(element) => element.encode(),
                StakingAdaptorV1Calls::RemoveClaimedRequest(element) => element.encode(),
                StakingAdaptorV1Calls::RemoveRequestId(element) => element.encode(),
                StakingAdaptorV1Calls::RequestBurn(element) => element.encode(),
                StakingAdaptorV1Calls::RequestIds(element) => element.encode(),
                StakingAdaptorV1Calls::RevokeApproval(element) => element.encode(),
                StakingAdaptorV1Calls::Slippage(element) => element.encode(),
                StakingAdaptorV1Calls::Unwrap(element) => element.encode(),
                StakingAdaptorV1Calls::Withdraw(element) => element.encode(),
                StakingAdaptorV1Calls::WithdrawableFrom(element) => element.encode(),
                StakingAdaptorV1Calls::Wrap(element) => element.encode(),
                StakingAdaptorV1Calls::WrappedPrimitive(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for StakingAdaptorV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StakingAdaptorV1Calls::AddRequestId(element) => element.fmt(f),
                StakingAdaptorV1Calls::AssetOf(element) => element.fmt(f),
                StakingAdaptorV1Calls::AssetsUsed(element) => element.fmt(f),
                StakingAdaptorV1Calls::BalanceOf(element) => element.fmt(f),
                StakingAdaptorV1Calls::CancelBurn(element) => element.fmt(f),
                StakingAdaptorV1Calls::CompleteBurn(element) => element.fmt(f),
                StakingAdaptorV1Calls::Deposit(element) => element.fmt(f),
                StakingAdaptorV1Calls::GetRequestIds(element) => element.fmt(f),
                StakingAdaptorV1Calls::Identifier(element) => element.fmt(f),
                StakingAdaptorV1Calls::IsDebt(element) => element.fmt(f),
                StakingAdaptorV1Calls::LockedStoragePosition(element) => element.fmt(f),
                StakingAdaptorV1Calls::Mint(element) => element.fmt(f),
                StakingAdaptorV1Calls::MintERC20(element) => element.fmt(f),
                StakingAdaptorV1Calls::RemoveClaimedRequest(element) => element.fmt(f),
                StakingAdaptorV1Calls::RemoveRequestId(element) => element.fmt(f),
                StakingAdaptorV1Calls::RequestBurn(element) => element.fmt(f),
                StakingAdaptorV1Calls::RequestIds(element) => element.fmt(f),
                StakingAdaptorV1Calls::RevokeApproval(element) => element.fmt(f),
                StakingAdaptorV1Calls::Slippage(element) => element.fmt(f),
                StakingAdaptorV1Calls::Unwrap(element) => element.fmt(f),
                StakingAdaptorV1Calls::Withdraw(element) => element.fmt(f),
                StakingAdaptorV1Calls::WithdrawableFrom(element) => element.fmt(f),
                StakingAdaptorV1Calls::Wrap(element) => element.fmt(f),
                StakingAdaptorV1Calls::WrappedPrimitive(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddRequestIdCall> for StakingAdaptorV1Calls {
        fn from(var: AddRequestIdCall) -> Self {
            StakingAdaptorV1Calls::AddRequestId(var)
        }
    }
    impl ::std::convert::From<AssetOfCall> for StakingAdaptorV1Calls {
        fn from(var: AssetOfCall) -> Self {
            StakingAdaptorV1Calls::AssetOf(var)
        }
    }
    impl ::std::convert::From<AssetsUsedCall> for StakingAdaptorV1Calls {
        fn from(var: AssetsUsedCall) -> Self {
            StakingAdaptorV1Calls::AssetsUsed(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for StakingAdaptorV1Calls {
        fn from(var: BalanceOfCall) -> Self {
            StakingAdaptorV1Calls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<CancelBurnCall> for StakingAdaptorV1Calls {
        fn from(var: CancelBurnCall) -> Self {
            StakingAdaptorV1Calls::CancelBurn(var)
        }
    }
    impl ::std::convert::From<CompleteBurnCall> for StakingAdaptorV1Calls {
        fn from(var: CompleteBurnCall) -> Self {
            StakingAdaptorV1Calls::CompleteBurn(var)
        }
    }
    impl ::std::convert::From<DepositCall> for StakingAdaptorV1Calls {
        fn from(var: DepositCall) -> Self {
            StakingAdaptorV1Calls::Deposit(var)
        }
    }
    impl ::std::convert::From<GetRequestIdsCall> for StakingAdaptorV1Calls {
        fn from(var: GetRequestIdsCall) -> Self {
            StakingAdaptorV1Calls::GetRequestIds(var)
        }
    }
    impl ::std::convert::From<IdentifierCall> for StakingAdaptorV1Calls {
        fn from(var: IdentifierCall) -> Self {
            StakingAdaptorV1Calls::Identifier(var)
        }
    }
    impl ::std::convert::From<IsDebtCall> for StakingAdaptorV1Calls {
        fn from(var: IsDebtCall) -> Self {
            StakingAdaptorV1Calls::IsDebt(var)
        }
    }
    impl ::std::convert::From<LockedStoragePositionCall> for StakingAdaptorV1Calls {
        fn from(var: LockedStoragePositionCall) -> Self {
            StakingAdaptorV1Calls::LockedStoragePosition(var)
        }
    }
    impl ::std::convert::From<MintCall> for StakingAdaptorV1Calls {
        fn from(var: MintCall) -> Self {
            StakingAdaptorV1Calls::Mint(var)
        }
    }
    impl ::std::convert::From<MintERC20Call> for StakingAdaptorV1Calls {
        fn from(var: MintERC20Call) -> Self {
            StakingAdaptorV1Calls::MintERC20(var)
        }
    }
    impl ::std::convert::From<RemoveClaimedRequestCall> for StakingAdaptorV1Calls {
        fn from(var: RemoveClaimedRequestCall) -> Self {
            StakingAdaptorV1Calls::RemoveClaimedRequest(var)
        }
    }
    impl ::std::convert::From<RemoveRequestIdCall> for StakingAdaptorV1Calls {
        fn from(var: RemoveRequestIdCall) -> Self {
            StakingAdaptorV1Calls::RemoveRequestId(var)
        }
    }
    impl ::std::convert::From<RequestBurnCall> for StakingAdaptorV1Calls {
        fn from(var: RequestBurnCall) -> Self {
            StakingAdaptorV1Calls::RequestBurn(var)
        }
    }
    impl ::std::convert::From<RequestIdsCall> for StakingAdaptorV1Calls {
        fn from(var: RequestIdsCall) -> Self {
            StakingAdaptorV1Calls::RequestIds(var)
        }
    }
    impl ::std::convert::From<RevokeApprovalCall> for StakingAdaptorV1Calls {
        fn from(var: RevokeApprovalCall) -> Self {
            StakingAdaptorV1Calls::RevokeApproval(var)
        }
    }
    impl ::std::convert::From<SlippageCall> for StakingAdaptorV1Calls {
        fn from(var: SlippageCall) -> Self {
            StakingAdaptorV1Calls::Slippage(var)
        }
    }
    impl ::std::convert::From<UnwrapCall> for StakingAdaptorV1Calls {
        fn from(var: UnwrapCall) -> Self {
            StakingAdaptorV1Calls::Unwrap(var)
        }
    }
    impl ::std::convert::From<WithdrawCall> for StakingAdaptorV1Calls {
        fn from(var: WithdrawCall) -> Self {
            StakingAdaptorV1Calls::Withdraw(var)
        }
    }
    impl ::std::convert::From<WithdrawableFromCall> for StakingAdaptorV1Calls {
        fn from(var: WithdrawableFromCall) -> Self {
            StakingAdaptorV1Calls::WithdrawableFrom(var)
        }
    }
    impl ::std::convert::From<WrapCall> for StakingAdaptorV1Calls {
        fn from(var: WrapCall) -> Self {
            StakingAdaptorV1Calls::Wrap(var)
        }
    }
    impl ::std::convert::From<WrappedPrimitiveCall> for StakingAdaptorV1Calls {
        fn from(var: WrappedPrimitiveCall) -> Self {
            StakingAdaptorV1Calls::WrappedPrimitive(var)
        }
    }
}
