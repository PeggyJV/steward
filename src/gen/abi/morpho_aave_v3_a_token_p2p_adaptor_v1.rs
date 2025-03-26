pub use morpho_aave_v3a_token_p2p_adaptor_v1::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod morpho_aave_v3a_token_p2p_adaptor_v1 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_morpho"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rewardDistributor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("assetOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("assetsUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetsUsed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("configurationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositToAaveV3Morpho"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "depositToAaveV3Morpho",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenToDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToDeposit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxIterations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("identifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("identifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isDebt"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("morpho"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("morpho"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IMorphoV3"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("morphoRewardsDistributor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "morphoRewardsDistributor",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract RewardsDistributor",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("revokeApproval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("revokeApproval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("slippage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slippage"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("configurationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawFromAaveV3Morpho"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawFromAaveV3Morpho",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenToWithdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToWithdraw"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxIterations"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawableFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawableFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptorData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BaseAdaptor__ConstructorHealthFactorTooLow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__ConstructorHealthFactorTooLow",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BaseAdaptor__ExternalReceiverBlocked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__ExternalReceiverBlocked",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BaseAdaptor__PricingNotSupported"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__PricingNotSupported",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BaseAdaptor__Slippage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__Slippage",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BaseAdaptor__UserDepositsNotAllowed",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__UserDepositsNotAllowed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BaseAdaptor__UserWithdrawsNotAllowed",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BaseAdaptor__UserWithdrawsNotAllowed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MORPHOAAVEV3ATOKENP2PADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x11b8\x03\x80a\x11b\x839\x81\x01`@\x81\x90Ra\0/\x91a\0bV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x16`\xA0Ra\0\x95V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0]W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\0uW`\0\x80\xFD[a\0~\x83a\0FV[\x91Pa\0\x8C` \x84\x01a\0FV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x10[a\x01\x07`\09`\0\x81\x81a\x02\x1F\x01R\x81\x81a\x03=\x01R\x81\x81a\x03\xD7\x01R\x81\x81a\x04]\x01R\x81\x81a\x04\xD1\x01R\x81\x81a\x05;\x01R\x81\x81a\x06N\x01R\x81\x81a\x06\xA6\x01R\x81\x81a\x07\x1A\x01R\x81\x81a\x08\x1B\x01Ra\t\x02\x01R`\0\x81\x81a\x01<\x01Ra\x02~\x01Ra\x10[`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c|\xA4\xC4\xB7\x11a\0\x97W\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x02\x07W\x80c\xD8\xFB\xC83\x14a\x02\x1AW\x80c\xE1p\xA9\xBF\x14a\x02AW\x80c\xFAP\xE5\xD2\x14a\x02TW`\0\x80\xFD[\x80c|\xA4\xC4\xB7\x14a\x01\xB2W\x80c\x895:\t\x14a\x01\xC5W\x80c\xAE\xFF\xDD\xDE\x14a\x01\xD4W\x80c\xC9\x11\x1B\xD7\x14a\x01\xF4W`\0\x80\xFD[\x80c[]Mx\x11a\0\xD3W\x80c[]Mx\x14a\x017W\x80ciD\\1\x14a\x01vW\x80cxASe\x14a\x01\x89W\x80cy\x98\xA1\xC4\x14a\x01\xAAW`\0\x80\xFD[\x80c/R\xEB\xB7\x14a\0\xFAW\x80c54\n'\x14a\x01\x0FW\x80c>\x03*;\x14a\x01\"W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x0B\xE2V[a\x02gV[\0[a\x01\ra\x01\x1D6`\x04a\x0C\xA9V[a\x02\xEDV[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01.V[a\x01\ra\x01\x846`\x04a\rNV[a\x03\xB0V[a\x01\x9Ca\x01\x976`\x04a\r\xBBV[a\x04\xFCV[`@Q\x90\x81R` \x01a\x01.V[a\x01\x9Ca\x05\xAFV[a\x01\ra\x01\xC06`\x04a\x0C\xA9V[a\x06\x1DV[`@Q`\0\x81R` \x01a\x01.V[a\x01\xE7a\x01\xE26`\x04a\r\xBBV[a\x07CV[`@Qa\x01.\x91\x90a\r\xF0V[a\x01\ra\x02\x026`\x04a\x0E=V[a\x07\xA6V[a\x01\ra\x02\x156`\x04a\x0E\xBDV[a\x08\x93V[a\x01^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01^a\x02O6`\x04a\r\xBBV[a\x08\xACV[a\x01\x9Ca\x02b6`\x04a\x0E\xF6V[a\x08\xC3V[`@Qc\x0FD\xFE\x1D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\x13\xF8t\x90a\x02\xB7\x900\x90\x86\x90\x86\x90`\x04\x01a\x0FZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE5W=`\0\x80>=`\0\xFD[PPPPPPV[\x80\x15\x80a\x02\xFAWP`\n\x81\x11[\x15a\x03\x03WP`\x04[`@Qc*\x80\xF8\xE3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01\x81\x90R`d\x83\x01R`\x84\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cU\x01\xF1\xC6\x90`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAA\x91\x90a\x0F\xB7V[PPPPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x03\xC6\x91\x90a\x0F\xD0V[\x90Pa\x03\xFC`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\tyV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x04\x12\x91\x90a\x0F\xB7V[\x90P\x80\x15\x80a\x04!WP`\n\x81\x11[\x15a\x04*WP`\x04[`@Qc:q\xCDg`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R0`D\x83\x01R`d\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xC75\x9C\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCA\x91\x90a\x0F\xB7V[Pa\x04\xF5\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xF4V[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05\x13\x91\x90a\x0F\xD0V[`@QcQ\xE2\xE5\xB9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R3`$\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xC5\xCBr\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA8\x91\x90a\x0F\xB7V[\x93\x92PPPV[`\0`@Q` \x01a\x06\x02\x90` \x80\x82R`'\x90\x82\x01R\x7FMorpho Aave V3 aToken P2P Adapto`@\x82\x01Rf9\x10+\x10\x18\x97\x19`\xC9\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80\x15\x80a\x06*WP`\n\x81\x11[\x15a\x063WP`\x04[a\x06=\x83\x83a\n\x83V[\x91Pa\x06s`\x01`\x01`\xA0\x1B\x03\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\tyV[`@Qc:q\xCDg`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R`d\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xC75\x9C\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x13\x91\x90a\x0F\xB7V[Pa\x07>\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xF4V[PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07n\x82a\x08\xACV[\x81`\0\x81Q\x81\x10a\x07\x81Wa\x07\x81a\x0F\xEDV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x07\xAF\x83a\x0B\x04V[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xC5\x91\x90a\x0F\xD0V[\x90P`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xDD\x91\x90a\x0F\xB7V[`@Qc*\x80\xF8\xE3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x89\x90R0`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cU\x01\xF1\xC6\x90`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x8A\x91\x90a\x0F\xB7V[PPPPPPPV[a\x08\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\tyV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05\xA8\x91\x90a\x0F\xD0V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x08\xDA\x91\x90a\x0F\xD0V[`@QcQ\xE2\xE5\xB9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R3`$\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xC5\xCBr\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\to\x91\x90a\x0F\xB7V[\x91PP[\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nh\x91\x90a\x0F\xB7V[\x11\x15a\x08\xA8Wa\x08\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\tyV[`\0`\0\x19\x82\x03a\n\xFDW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xF6\x91\x90a\x0F\xB7V[\x90Pa\tsV[P\x80a\tsV[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\x0BzWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bz\x91\x90a\x10\x03V[\x15a\x0B\x98W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\xDAWa\x0B\xDAa\x0B\x9BV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF5W`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x15W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0C)W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0C;Wa\x0C;a\x0B\x9BV[\x80`\x05\x1B\x91Pa\x0CL\x84\x83\x01a\x0B\xB1V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x89\x84\x11\x15a\x0CfW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0C\x84W\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x0CkV[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x98W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xBEW`\0\x80\xFD[\x835a\x0C\xC9\x81a\x0C\x94V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0C\xEFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\tWa\r\ta\x0B\x9BV[a\r\x1C`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0B\xB1V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\rcW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x82W`\0\x80\xFD[a\r\x8E\x87\x83\x88\x01a\x0C\xDEV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\r\xA4W`\0\x80\xFD[Pa\r\xB1\x86\x82\x87\x01a\x0C\xDEV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\r\xCDW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE4W`\0\x80\xFD[a\to\x84\x82\x85\x01a\x0C\xDEV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x0CV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0ESW`\0\x80\xFD[\x845\x93P` \x85\x015a\x0Ee\x81a\x0C\x94V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x82W`\0\x80\xFD[a\x0E\x8E\x88\x83\x89\x01a\x0C\xDEV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xA4W`\0\x80\xFD[Pa\x0E\xB1\x87\x82\x88\x01a\x0C\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD0W`\0\x80\xFD[\x825a\x0E\xDB\x81a\x0C\x94V[\x91P` \x83\x015a\x0E\xEB\x81a\x0C\x94V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\tW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F!W`\0\x80\xFD[a\x0F-\x86\x83\x87\x01a\x0C\xDEV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0FCW`\0\x80\xFD[Pa\x0FP\x85\x82\x86\x01a\x0C\xDEV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x80\x82\x01\x84\x90R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R`\0\x91\x84\x81\x01\x91`\x80\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xA9W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0F\x8DV[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xE2W`\0\x80\xFD[\x81Qa\x05\xA8\x81a\x0C\x94V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x15W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xA8W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 5\xC4\xEF\xD5\xC60>0Pzvs\x8A\xDB\xF5G\tK\xC9\0\x85?\xEE\xA9\xEFA3\x8E9\xD5t\xC5dsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static MORPHOAAVEV3ATOKENP2PADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c|\xA4\xC4\xB7\x11a\0\x97W\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x02\x07W\x80c\xD8\xFB\xC83\x14a\x02\x1AW\x80c\xE1p\xA9\xBF\x14a\x02AW\x80c\xFAP\xE5\xD2\x14a\x02TW`\0\x80\xFD[\x80c|\xA4\xC4\xB7\x14a\x01\xB2W\x80c\x895:\t\x14a\x01\xC5W\x80c\xAE\xFF\xDD\xDE\x14a\x01\xD4W\x80c\xC9\x11\x1B\xD7\x14a\x01\xF4W`\0\x80\xFD[\x80c[]Mx\x11a\0\xD3W\x80c[]Mx\x14a\x017W\x80ciD\\1\x14a\x01vW\x80cxASe\x14a\x01\x89W\x80cy\x98\xA1\xC4\x14a\x01\xAAW`\0\x80\xFD[\x80c/R\xEB\xB7\x14a\0\xFAW\x80c54\n'\x14a\x01\x0FW\x80c>\x03*;\x14a\x01\"W[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x0B\xE2V[a\x02gV[\0[a\x01\ra\x01\x1D6`\x04a\x0C\xA9V[a\x02\xEDV[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01.V[a\x01\ra\x01\x846`\x04a\rNV[a\x03\xB0V[a\x01\x9Ca\x01\x976`\x04a\r\xBBV[a\x04\xFCV[`@Q\x90\x81R` \x01a\x01.V[a\x01\x9Ca\x05\xAFV[a\x01\ra\x01\xC06`\x04a\x0C\xA9V[a\x06\x1DV[`@Q`\0\x81R` \x01a\x01.V[a\x01\xE7a\x01\xE26`\x04a\r\xBBV[a\x07CV[`@Qa\x01.\x91\x90a\r\xF0V[a\x01\ra\x02\x026`\x04a\x0E=V[a\x07\xA6V[a\x01\ra\x02\x156`\x04a\x0E\xBDV[a\x08\x93V[a\x01^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01^a\x02O6`\x04a\r\xBBV[a\x08\xACV[a\x01\x9Ca\x02b6`\x04a\x0E\xF6V[a\x08\xC3V[`@Qc\x0FD\xFE\x1D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\x13\xF8t\x90a\x02\xB7\x900\x90\x86\x90\x86\x90`\x04\x01a\x0FZV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xD1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xE5W=`\0\x80>=`\0\xFD[PPPPPPV[\x80\x15\x80a\x02\xFAWP`\n\x81\x11[\x15a\x03\x03WP`\x04[`@Qc*\x80\xF8\xE3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01\x81\x90R`d\x83\x01R`\x84\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cU\x01\xF1\xC6\x90`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x86W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xAA\x91\x90a\x0F\xB7V[PPPPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x03\xC6\x91\x90a\x0F\xD0V[\x90Pa\x03\xFC`\x01`\x01`\xA0\x1B\x03\x82\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\tyV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x04\x12\x91\x90a\x0F\xB7V[\x90P\x80\x15\x80a\x04!WP`\n\x81\x11[\x15a\x04*WP`\x04[`@Qc:q\xCDg`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x87\x90R0`D\x83\x01R`d\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xC75\x9C\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xCA\x91\x90a\x0F\xB7V[Pa\x04\xF5\x82\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xF4V[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05\x13\x91\x90a\x0F\xD0V[`@QcQ\xE2\xE5\xB9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R3`$\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xC5\xCBr\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA8\x91\x90a\x0F\xB7V[\x93\x92PPPV[`\0`@Q` \x01a\x06\x02\x90` \x80\x82R`'\x90\x82\x01R\x7FMorpho Aave V3 aToken P2P Adapto`@\x82\x01Rf9\x10+\x10\x18\x97\x19`\xC9\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80\x15\x80a\x06*WP`\n\x81\x11[\x15a\x063WP`\x04[a\x06=\x83\x83a\n\x83V[\x91Pa\x06s`\x01`\x01`\xA0\x1B\x03\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\tyV[`@Qc:q\xCDg`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R`d\x82\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE9\xC75\x9C\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x13\x91\x90a\x0F\xB7V[Pa\x07>\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xF4V[PPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07n\x82a\x08\xACV[\x81`\0\x81Q\x81\x10a\x07\x81Wa\x07\x81a\x0F\xEDV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x07\xAF\x83a\x0B\x04V[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xC5\x91\x90a\x0F\xD0V[\x90P`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x07\xDD\x91\x90a\x0F\xB7V[`@Qc*\x80\xF8\xE3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x89\x90R0`D\x83\x01R\x87\x81\x16`d\x83\x01R`\x84\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cU\x01\xF1\xC6\x90`\xA4\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x8A\x91\x90a\x0F\xB7V[PPPPPPPV[a\x08\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\tyV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05\xA8\x91\x90a\x0F\xD0V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x08\xDA\x91\x90a\x0F\xD0V[`@QcQ\xE2\xE5\xB9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R3`$\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA3\xC5\xCBr\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\to\x91\x90a\x0F\xB7V[\x91PP[\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x03\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nh\x91\x90a\x0F\xB7V[\x11\x15a\x08\xA8Wa\x08\xA8`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\tyV[`\0`\0\x19\x82\x03a\n\xFDW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xF6\x91\x90a\x0F\xB7V[\x90Pa\tsV[P\x80a\tsV[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\x0BzWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bz\x91\x90a\x10\x03V[\x15a\x0B\x98W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\xDAWa\x0B\xDAa\x0B\x9BV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xF5W`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\x15W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0C)W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x0C;Wa\x0C;a\x0B\x9BV[\x80`\x05\x1B\x91Pa\x0CL\x84\x83\x01a\x0B\xB1V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x89\x84\x11\x15a\x0CfW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15a\x0C\x84W\x845\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x0CkV[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\x98W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xBEW`\0\x80\xFD[\x835a\x0C\xC9\x81a\x0C\x94V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0C\xEFW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\tWa\r\ta\x0B\x9BV[a\r\x1C`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0B\xB1V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\r1W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\rcW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\x82W`\0\x80\xFD[a\r\x8E\x87\x83\x88\x01a\x0C\xDEV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\r\xA4W`\0\x80\xFD[Pa\r\xB1\x86\x82\x87\x01a\x0C\xDEV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\r\xCDW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE4W`\0\x80\xFD[a\to\x84\x82\x85\x01a\x0C\xDEV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x0CV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0ESW`\0\x80\xFD[\x845\x93P` \x85\x015a\x0Ee\x81a\x0C\x94V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x82W`\0\x80\xFD[a\x0E\x8E\x88\x83\x89\x01a\x0C\xDEV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xA4W`\0\x80\xFD[Pa\x0E\xB1\x87\x82\x88\x01a\x0C\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xD0W`\0\x80\xFD[\x825a\x0E\xDB\x81a\x0C\x94V[\x91P` \x83\x015a\x0E\xEB\x81a\x0C\x94V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\tW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F!W`\0\x80\xFD[a\x0F-\x86\x83\x87\x01a\x0C\xDEV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0FCW`\0\x80\xFD[Pa\x0FP\x85\x82\x86\x01a\x0C\xDEV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x80\x82\x01\x84\x90R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R`\0\x91\x84\x81\x01\x91`\x80\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xA9W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0F\x8DV[P\x90\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x0F\xC9W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\xE2W`\0\x80\xFD[\x81Qa\x05\xA8\x81a\x0C\x94V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x15W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05\xA8W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 5\xC4\xEF\xD5\xC60>0Pzvs\x8A\xDB\xF5G\tK\xC9\0\x85?\xEE\xA9\xEFA3\x8E9\xD5t\xC5dsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static MORPHOAAVEV3ATOKENP2PADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MorphoAaveV3ATokenP2PAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MorphoAaveV3ATokenP2PAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MorphoAaveV3ATokenP2PAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MorphoAaveV3ATokenP2PAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MorphoAaveV3ATokenP2PAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MorphoAaveV3ATokenP2PAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MorphoAaveV3ATokenP2PAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MORPHOAAVEV3ATOKENP2PADAPTORV1_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MORPHOAAVEV3ATOKENP2PADAPTORV1_ABI.clone(),
                MORPHOAAVEV3ATOKENP2PADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `assetOf` (0xe170a9bf) function
        pub fn asset_of(
            &self,
            adaptor_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([225, 112, 169, 191], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetsUsed` (0xaeffddde) function
        pub fn assets_used(
            &self,
            adaptor_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([174, 255, 221, 222], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x78415365) function
        pub fn balance_of(
            &self,
            adaptor_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], adaptor_data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claim` (0x2f52ebb7) function
        pub fn claim(
            &self,
            claimable: ::ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 82, 235, 183], (claimable, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x69445c31) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            adaptor_data: ::ethers::core::types::Bytes,
            configuration_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [105, 68, 92, 49],
                    (assets, adaptor_data, configuration_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositToAaveV3Morpho` (0x7ca4c4b7) function
        pub fn deposit_to_aave_v3_morpho(
            &self,
            token_to_deposit: ::ethers::core::types::Address,
            amount_to_deposit: ::ethers::core::types::U256,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [124, 164, 196, 183],
                    (token_to_deposit, amount_to_deposit, max_iterations),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `identifier` (0x7998a1c4) function
        pub fn identifier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 152, 161, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDebt` (0x89353a09) function
        pub fn is_debt(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([137, 53, 58, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `morpho` (0xd8fbc833) function
        pub fn morpho(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([216, 251, 200, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `morphoRewardsDistributor` (0x5b5d4d78) function
        pub fn morpho_rewards_distributor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([91, 93, 77, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokeApproval` (0xd3bfe76a) function
        pub fn revoke_approval(
            &self,
            asset: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 191, 231, 106], (asset, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slippage` (0x3e032a3b) function
        pub fn slippage(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xc9111bd7) function
        pub fn withdraw(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            adaptor_data: ::ethers::core::types::Bytes,
            configuration_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (assets, receiver, adaptor_data, configuration_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFromAaveV3Morpho` (0x35340a27) function
        pub fn withdraw_from_aave_v3_morpho(
            &self,
            token_to_withdraw: ::ethers::core::types::Address,
            amount_to_withdraw: ::ethers::core::types::U256,
            max_iterations: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [53, 52, 10, 39],
                    (token_to_withdraw, amount_to_withdraw, max_iterations),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawableFrom` (0xfa50e5d2) function
        pub fn withdrawable_from(
            &self,
            adaptor_data: ::ethers::core::types::Bytes,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (adaptor_data, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MorphoAaveV3ATokenP2PAdaptorV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BaseAdaptor__ConstructorHealthFactorTooLow` with signature `BaseAdaptor__ConstructorHealthFactorTooLow()` and selector `0x97ed5f41`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BaseAdaptor__ConstructorHealthFactorTooLow",
        abi = "BaseAdaptor__ConstructorHealthFactorTooLow()"
    )]
    pub struct BaseAdaptor__ConstructorHealthFactorTooLow;
    ///Custom Error type `BaseAdaptor__ExternalReceiverBlocked` with signature `BaseAdaptor__ExternalReceiverBlocked()` and selector `0x1f7a6d44`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BaseAdaptor__ExternalReceiverBlocked",
        abi = "BaseAdaptor__ExternalReceiverBlocked()"
    )]
    pub struct BaseAdaptor__ExternalReceiverBlocked;
    ///Custom Error type `BaseAdaptor__PricingNotSupported` with signature `BaseAdaptor__PricingNotSupported(address)` and selector `0x7ab23849`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BaseAdaptor__PricingNotSupported",
        abi = "BaseAdaptor__PricingNotSupported(address)"
    )]
    pub struct BaseAdaptor__PricingNotSupported {
        pub asset: ::ethers::core::types::Address,
    }
    ///Custom Error type `BaseAdaptor__Slippage` with signature `BaseAdaptor__Slippage()` and selector `0x7a83ac04`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BaseAdaptor__Slippage", abi = "BaseAdaptor__Slippage()")]
    pub struct BaseAdaptor__Slippage;
    ///Custom Error type `BaseAdaptor__UserDepositsNotAllowed` with signature `BaseAdaptor__UserDepositsNotAllowed()` and selector `0xc813b56c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BaseAdaptor__UserDepositsNotAllowed",
        abi = "BaseAdaptor__UserDepositsNotAllowed()"
    )]
    pub struct BaseAdaptor__UserDepositsNotAllowed;
    ///Custom Error type `BaseAdaptor__UserWithdrawsNotAllowed` with signature `BaseAdaptor__UserWithdrawsNotAllowed()` and selector `0xbea0078a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BaseAdaptor__UserWithdrawsNotAllowed",
        abi = "BaseAdaptor__UserWithdrawsNotAllowed()"
    )]
    pub struct BaseAdaptor__UserWithdrawsNotAllowed;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Deserialize,
        serde::Serialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MorphoAaveV3ATokenP2PAdaptorV1Errors {
        BaseAdaptor__ConstructorHealthFactorTooLow(
            BaseAdaptor__ConstructorHealthFactorTooLow,
        ),
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__ConstructorHealthFactorTooLow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__ConstructorHealthFactorTooLow(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__ExternalReceiverBlocked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__ExternalReceiverBlocked(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__PricingNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__PricingNotSupported(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__Slippage as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__Slippage(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__UserDepositsNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__UserDepositsNotAllowed(decoded));
            }
            if let Ok(decoded) = <BaseAdaptor__UserWithdrawsNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BaseAdaptor__UserWithdrawsNotAllowed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BaseAdaptor__ConstructorHealthFactorTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseAdaptor__ExternalReceiverBlocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseAdaptor__PricingNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseAdaptor__Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseAdaptor__UserDepositsNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BaseAdaptor__UserWithdrawsNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BaseAdaptor__ConstructorHealthFactorTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BaseAdaptor__ExternalReceiverBlocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BaseAdaptor__PricingNotSupported as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BaseAdaptor__Slippage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BaseAdaptor__UserDepositsNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BaseAdaptor__UserWithdrawsNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BaseAdaptor__ConstructorHealthFactorTooLow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseAdaptor__ExternalReceiverBlocked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseAdaptor__PricingNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseAdaptor__Slippage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseAdaptor__UserDepositsNotAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BaseAdaptor__UserWithdrawsNotAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ConstructorHealthFactorTooLow>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__ConstructorHealthFactorTooLow) -> Self {
            Self::BaseAdaptor__ConstructorHealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for MorphoAaveV3ATokenP2PAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    ///Container type for all input parameters for the `assetOf` function with signature `assetOf(bytes)` and selector `0xe170a9bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "assetOf", abi = "assetOf(bytes)")]
    pub struct AssetOfCall {
        pub adaptor_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `assetsUsed` function with signature `assetsUsed(bytes)` and selector `0xaeffddde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "assetsUsed", abi = "assetsUsed(bytes)")]
    pub struct AssetsUsedCall {
        pub adaptor_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(bytes)` and selector `0x78415365`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(bytes)")]
    pub struct BalanceOfCall {
        pub adaptor_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `claim` function with signature `claim(uint256,bytes32[])` and selector `0x2f52ebb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claim", abi = "claim(uint256,bytes32[])")]
    pub struct ClaimCall {
        pub claimable: ::ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,bytes,bytes)` and selector `0x69445c31`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deposit", abi = "deposit(uint256,bytes,bytes)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub adaptor_data: ::ethers::core::types::Bytes,
        pub configuration_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `depositToAaveV3Morpho` function with signature `depositToAaveV3Morpho(address,uint256,uint256)` and selector `0x7ca4c4b7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "depositToAaveV3Morpho",
        abi = "depositToAaveV3Morpho(address,uint256,uint256)"
    )]
    pub struct DepositToAaveV3MorphoCall {
        pub token_to_deposit: ::ethers::core::types::Address,
        pub amount_to_deposit: ::ethers::core::types::U256,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `identifier` function with signature `identifier()` and selector `0x7998a1c4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "identifier", abi = "identifier()")]
    pub struct IdentifierCall;
    ///Container type for all input parameters for the `isDebt` function with signature `isDebt()` and selector `0x89353a09`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isDebt", abi = "isDebt()")]
    pub struct IsDebtCall;
    ///Container type for all input parameters for the `morpho` function with signature `morpho()` and selector `0xd8fbc833`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "morpho", abi = "morpho()")]
    pub struct MorphoCall;
    ///Container type for all input parameters for the `morphoRewardsDistributor` function with signature `morphoRewardsDistributor()` and selector `0x5b5d4d78`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "morphoRewardsDistributor", abi = "morphoRewardsDistributor()")]
    pub struct MorphoRewardsDistributorCall;
    ///Container type for all input parameters for the `revokeApproval` function with signature `revokeApproval(address,address)` and selector `0xd3bfe76a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokeApproval", abi = "revokeApproval(address,address)")]
    pub struct RevokeApprovalCall {
        pub asset: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `slippage` function with signature `slippage()` and selector `0x3e032a3b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "slippage", abi = "slippage()")]
    pub struct SlippageCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,bytes,bytes)` and selector `0xc9111bd7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,bytes,bytes)")]
    pub struct WithdrawCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub adaptor_data: ::ethers::core::types::Bytes,
        pub configuration_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdrawFromAaveV3Morpho` function with signature `withdrawFromAaveV3Morpho(address,uint256,uint256)` and selector `0x35340a27`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawFromAaveV3Morpho",
        abi = "withdrawFromAaveV3Morpho(address,uint256,uint256)"
    )]
    pub struct WithdrawFromAaveV3MorphoCall {
        pub token_to_withdraw: ::ethers::core::types::Address,
        pub amount_to_withdraw: ::ethers::core::types::U256,
        pub max_iterations: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawableFrom` function with signature `withdrawableFrom(bytes,bytes)` and selector `0xfa50e5d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "withdrawableFrom", abi = "withdrawableFrom(bytes,bytes)")]
    pub struct WithdrawableFromCall {
        pub adaptor_data: ::ethers::core::types::Bytes,
        pub p1: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Deserialize,
        serde::Serialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum MorphoAaveV3ATokenP2PAdaptorV1Calls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Claim(ClaimCall),
        Deposit(DepositCall),
        DepositToAaveV3Morpho(DepositToAaveV3MorphoCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        Morpho(MorphoCall),
        MorphoRewardsDistributor(MorphoRewardsDistributorCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawFromAaveV3Morpho(WithdrawFromAaveV3MorphoCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AssetOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetOf(decoded));
            }
            if let Ok(decoded) = <AssetsUsedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetsUsed(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositToAaveV3MorphoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositToAaveV3Morpho(decoded));
            }
            if let Ok(decoded) = <IdentifierCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Identifier(decoded));
            }
            if let Ok(decoded) = <IsDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsDebt(decoded));
            }
            if let Ok(decoded) = <MorphoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Morpho(decoded));
            }
            if let Ok(decoded) = <MorphoRewardsDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MorphoRewardsDistributor(decoded));
            }
            if let Ok(decoded) = <RevokeApprovalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SlippageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slippage(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawFromAaveV3MorphoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawFromAaveV3Morpho(decoded));
            }
            if let Ok(decoded) = <WithdrawableFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositToAaveV3Morpho(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Morpho(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MorphoRewardsDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFromAaveV3Morpho(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositToAaveV3Morpho(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Morpho(element) => ::core::fmt::Display::fmt(element, f),
                Self::MorphoRewardsDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFromAaveV3Morpho(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetOfCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ClaimCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<DepositCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositToAaveV3MorphoCall>
    for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: DepositToAaveV3MorphoCall) -> Self {
            Self::DepositToAaveV3Morpho(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<MorphoCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: MorphoCall) -> Self {
            Self::Morpho(value)
        }
    }
    impl ::core::convert::From<MorphoRewardsDistributorCall>
    for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: MorphoRewardsDistributorCall) -> Self {
            Self::MorphoRewardsDistributor(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall>
    for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFromAaveV3MorphoCall>
    for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: WithdrawFromAaveV3MorphoCall) -> Self {
            Self::WithdrawFromAaveV3Morpho(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall>
    for MorphoAaveV3ATokenP2PAdaptorV1Calls {
        fn from(value: WithdrawableFromCall) -> Self {
            Self::WithdrawableFrom(value)
        }
    }
    ///Container type for all return fields from the `assetOf` function with signature `assetOf(bytes)` and selector `0xe170a9bf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AssetOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `assetsUsed` function with signature `assetsUsed(bytes)` and selector `0xaeffddde`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AssetsUsedReturn {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(bytes)` and selector `0x78415365`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `identifier` function with signature `identifier()` and selector `0x7998a1c4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IdentifierReturn(pub [u8; 32]);
    ///Container type for all return fields from the `isDebt` function with signature `isDebt()` and selector `0x89353a09`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsDebtReturn(pub bool);
    ///Container type for all return fields from the `morpho` function with signature `morpho()` and selector `0xd8fbc833`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MorphoReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `morphoRewardsDistributor` function with signature `morphoRewardsDistributor()` and selector `0x5b5d4d78`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MorphoRewardsDistributorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `slippage` function with signature `slippage()` and selector `0x3e032a3b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SlippageReturn(pub u32);
    ///Container type for all return fields from the `withdrawableFrom` function with signature `withdrawableFrom(bytes,bytes)` and selector `0xfa50e5d2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WithdrawableFromReturn(pub ::ethers::core::types::U256);
}
