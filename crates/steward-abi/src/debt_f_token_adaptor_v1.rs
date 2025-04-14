pub use debt_f_token_adaptor_v1::*;
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
pub mod debt_f_token_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/DebtFTokenAdaptorV1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_accountForInterest"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_frax"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_healthFactor"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ACCOUNT_FOR_INTEREST"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ACCOUNT_FOR_INTEREST",
                            ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FRAX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("FRAX"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("borrowFromFraxlend"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowFromFraxlend"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fraxlendPair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToBorrow"),
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
                    ::std::borrow::ToOwned::to_owned("callAddInterest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callAddInterest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fraxlendPair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFToken"),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("minimumHealthFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "minimumHealthFactor",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("repayFraxlendDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayFraxlendDebt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_fraxlendPair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IFToken"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_debtTokenRepayAmount",
                                    ),
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
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::string::String::new(),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                (
                    ::std::borrow::ToOwned::to_owned(
                        "DebtFTokenAdaptor__CannotRepayNoDebt",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DebtFTokenAdaptor__CannotRepayNoDebt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fraxlendPair"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fraxlendPair"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "DebtFTokenAdaptor__HealthFactorTooLow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DebtFTokenAdaptor__HealthFactorTooLow",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fraxlendPair"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEBTFTOKENADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x15\xCB8\x03\x80a\x15\xCB\x839\x81\x01`@\x81\x90Ra\0/\x91a\0~V[a\08\x81a\0RV[\x91\x15\x15`\xA0R`\x01`\x01`\xA0\x1B\x03\x16`\x80R`\xC0Ra\0\xD2V[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\0{W`@Qc\x97\xED_A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\0\x93W`\0\x80\xFD[\x83Q\x80\x15\x15\x81\x14a\0\xA3W`\0\x80\xFD[` \x85\x01Q\x90\x93P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC0W`\0\x80\xFD[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa\x14\xBCa\x01\x0F`\09`\0\x81\x81a\x01[\x01Ra\x02\xCD\x01R`\0\x81\x81a\x01\x1F\x01Ra\x03~\x01R`\0a\x02\x1B\x01Ra\x14\xBC`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x97W\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x02UW\x80c\xD3\xBF\xE7j\x14a\x02hW\x80c\xE1p\xA9\xBF\x14a\x02{W\x80c\xFAP\xE5\xD2\x14a\x02\x8EW`\0\x80\xFD[\x80c\x895:\t\x14a\x01\xDCW\x80c\x8B\x9B\x9D@\x14a\x01\xE3W\x80c\xAE\xFF\xDD\xDE\x14a\x01\xF6W\x80c\xB0\xE4Uo\x14a\x02\x16W`\0\x80\xFD[\x80ciD\\1\x11a\0\xD3W\x80ciD\\1\x14a\x01\x9BW\x80cm\xD6Y\xFD\x14a\x01\xAEW\x80cxASe\x14a\x01\xC1W\x80cy\x98\xA1\xC4\x14a\x01\xD4W`\0\x80\xFD[\x80c\x15\x86\xFF\xBB\x14a\x01\x05W\x80c\x19\x87uW\x14a\x01\x1AW\x80c\x1C\xAF\xF8\xB1\x14a\x01VW\x80c>\x03*;\x14a\x01\x8BW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\r\xF0V[a\x02\xA1V[\0[a\x01A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01MV[`@Qa#(\x81R` \x01a\x01MV[a\x01\x18a\x01\xA96`\x04a\x0E\xE8V[a\x03 V[a\x01\x18a\x01\xBC6`\x04a\x0FUV[a\x039V[a\x01}a\x01\xCF6`\x04a\x0FrV[a\x03NV[a\x01}a\x03\xAAV[`\x01a\x01AV[a\x01\x18a\x01\xF16`\x04a\r\xF0V[a\x04\x13V[a\x02\ta\x02\x046`\x04a\x0FrV[a\x04\xC1V[`@Qa\x01M\x91\x90a\x0F\xA7V[a\x02=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01MV[a\x01\x18a\x02c6`\x04a\x0F\xF4V[a\x05$V[a\x01\x18a\x02v6`\x04a\x10tV[a\x05=V[a\x02=a\x02\x896`\x04a\x0FrV[a\x05VV[a\x01}a\x02\x9C6`\x04a\x10\xADV[a\x05\x7FV[a\x02\xAA\x82a\x05\x88V[a\x02\xB4\x81\x83a\x07BV[`\0a\x02\xBF\x83a\x07\xA6V[\x90Pa\x02\xCB\x83\x82a\x08\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15a\x03\x1BW`@Qc\xC4\xBB\xF5\xFD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03B\x81a\x05\x88V[a\x03K\x81a\x08\xE1V[PV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03e\x91\x90a\x11\x11V[\x90P`\0a\x03s\x823a\tVV[\x90Pa\x03\xA2\x82\x82`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xC5V[\x94\x93PPPPV[`\0`@Q` \x01a\x03\xF8\x90` \x80\x82R`\"\x90\x82\x01R\x7FFraxLend debtTokenV2 Adaptor V 1`@\x82\x01Ra\x02\xE3`\xF4\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x04\x1C\x82a\x05\x88V[`\0a\x04'\x83a\nKV[\x90P`\0a\x045\x82\x84a\n\xAFV[\x90P`\0a\x04G\x85\x83`\0`\x01a\x0B0V[\x90P`\0a\x04U\x860a\tVV[\x90P\x80`\0\x03a\x04\x83W`@Qcx\xD1xM`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x03\x12V[\x81\x81\x10\x15a\x04\x8FW\x80\x91P[a\x04\xA5`\x01`\x01`\xA0\x1B\x03\x85\x16\x87`\0\x19a\x0BpV[a\x04\xAF\x86\x83a\x0B\xEDV[a\x04\xB9\x84\x87a\x0C V[PPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x04\xEC\x82a\x05VV[\x81`\0\x81Q\x81\x10a\x04\xFFWa\x04\xFFa\x11.V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05R`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0BpV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05m\x91\x90a\x11\x11V[\x90Pa\x05x\x81a\nKV[\x93\x92PPPV[`\0[\x92\x91PPV[`\0a\x05\x92a\x03\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\x01\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05\xC8\x93\x92\x91` \x01a\x11DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06D\x91\x90a\x11\x11V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06q\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a\x11\xBCV[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1A\x91\x90a\x11\xD7V[a\x03\x1BW`@Qc}h@\xEF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\x12V[`@Qcr\xF8\x9D\x8B`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\0`$\x82\x01R0`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE5\xF1;\x16\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xB9W=`\0\x80>=`\0\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFB\xBB\xF9L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\n\x91\x90a\x11\xF9V[`\x80\x01Q\x92\x91PPV[`\0\x80a\x08!\x840a\tVV[\x90P`\0a\x082\x85\x83`\x01\x80a\t\xC5V[\x90P\x80`\0\x03a\x08NWg\x0E\x92Yo\xD6)\0\0\x92PPPa\x05\x82V[`\0a\x08Z\x860a\x0C\xAFV[\x90P\x80`\0\x03a\x08pW`\0\x93PPPPa\x05\x82V[`\0\x80a\x08|\x88a\x0C\xE1V[\x90\x92P\x90P`\0\x83\x83\x83a\x08\x90\x8B\x89a\x12mV[a\x08\x9A\x91\x90a\x12\x92V[a\x08\xA4\x91\x90a\x12mV[a\x08\xAE\x91\x90a\x12\x92V[\x90P`\0a\x08\xBB\x8Aa\rXV[\x90P`\0a\x08\xD2\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\r\xBCV[\x9B\x9APPPPPPPPPPPV[`@Qc\x1Cl\x95\x97`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x1Cl\x95\x97\x90`$\x01a\x01\x80`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tM\x91\x90a\x13JV[PPPPPPPV[`@QcO\xD4\"\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90cO\xD4\"\xDF\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05x\x91\x90a\x14\x10V[`@Qc~\xC4\xB5q`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c~\xC4\xB5q\x90`d\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a\x14\x10V[\x95\x94PPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a\x11\x11V[`\0`\0\x19\x82\x03a\x0B)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\"\x91\x90a\x14\x10V[\x90Pa\x05\x82V[P\x80a\x05\x82V[`@Qc\x1C%\x91\xD3`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x1C%\x91\xD3\x90`d\x01a\n\x01V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0B\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x03\x12V[PPPPV[`@Qc=A}-`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c=A}-\x90`D\x01a\x07xV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x94\x91\x90a\x14\x10V[\x11\x15a\x05RWa\x05R`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0BpV[`@QcZ\xD7\x981`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90c\xB5\xAF0b\x90`$\x01a\t\x84V[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c\x9A)^s`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rG\x91\x90a\x14)V[P\x95\x99\x91\x98P\x90\x96PPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x84\xBD\x05`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a\x14\x10V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xD4W`\0\x80\xFD[\x04\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03KW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x03W`\0\x80\xFD[\x825a\x0E\x0E\x81a\r\xDBV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EUWa\x0EUa\x0E\x1CV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0ElW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x87Wa\x0E\x87a\x0E\x1CV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xAFWa\x0E\xAFa\x0E\x1CV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xC8W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xFDW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x1CW`\0\x80\xFD[a\x0F(\x87\x83\x88\x01a\x0E[V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0F>W`\0\x80\xFD[Pa\x0FK\x86\x82\x87\x01a\x0E[V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0FgW`\0\x80\xFD[\x815a\x05x\x81a\r\xDBV[`\0` \x82\x84\x03\x12\x15a\x0F\x84W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9BW`\0\x80\xFD[a\x03\xA2\x84\x82\x85\x01a\x0E[V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xE8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\xC3V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\nW`\0\x80\xFD[\x845\x93P` \x85\x015a\x10\x1C\x81a\r\xDBV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x109W`\0\x80\xFD[a\x10E\x88\x83\x89\x01a\x0E[V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x10[W`\0\x80\xFD[Pa\x10h\x87\x82\x88\x01a\x0E[V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x87W`\0\x80\xFD[\x825a\x10\x92\x81a\r\xDBV[\x91P` \x83\x015a\x10\xA2\x81a\r\xDBV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xC0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xD8W`\0\x80\xFD[a\x10\xE4\x86\x83\x87\x01a\x0E[V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x10\xFAW`\0\x80\xFD[Pa\x11\x07\x85\x82\x86\x01a\x0E[V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11#W`\0\x80\xFD[\x81Qa\x05x\x81a\r\xDBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`\0` \x84\x15\x15\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x11\x80W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x11dV[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xCEW`\0\x80\xFD[a\x05x\x82a\x11\xA3V[`\0` \x82\x84\x03\x12\x15a\x11\xE9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05xW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x12\x0BW`\0\x80\xFD[a\x12\x13a\x0E2V[\x82Qa\x12\x1E\x81a\r\xDBV[\x81Ra\x12,` \x84\x01a\x11\xA3V[` \x82\x01R`@\x83\x01Q`\x01`\x01`\xB8\x1B\x03\x81\x16\x81\x14a\x12KW`\0\x80\xFD[`@\x82\x01R``\x83\x81\x01Q\x90\x82\x01R`\x80\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x82WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x12\xAFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x12\xFEW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13!Wa\x13!a\x0E\x1CV[`@R\x90P\x80a\x130\x83a\x12\xCCV[\x81Ra\x13>` \x84\x01a\x12\xCCV[` \x82\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01\x80\x81\x12\x15a\x13eW`\0\x80\xFD[\x87Q\x96P` \x88\x01Q\x95P`@\x88\x01Q\x94P`\xA0`_\x19\x82\x01\x12\x15a\x13\x89W`\0\x80\xFD[Pa\x13\x92a\x0E2V[a\x13\x9E``\x89\x01a\x11\xA3V[\x81Ra\x13\xAC`\x80\x89\x01a\x11\xA3V[` \x82\x01Ra\x13\xBD`\xA0\x89\x01a\x12\xB4V[`@\x82\x01Ra\x13\xCE`\xC0\x89\x01a\x12\xB4V[``\x82\x01Ra\x13\xDF`\xE0\x89\x01a\x12\xB4V[`\x80\x82\x01R\x92Pa\x13\xF4\x88a\x01\0\x89\x01a\x12\xECV[\x91Pa\x14\x04\x88a\x01@\x89\x01a\x12\xECV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x14\"W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x14FW`\0\x80\xFD[PP\x86Q` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x8C\x01Q`\xC0\x8D\x01Q`\xE0\x90\x9D\x01Q\x95\x9E\x94\x9DP\x92\x9B\x91\x9AP\x98P\x90\x96P\x94P\x90\x92P\x90PV\xFE\xA2dipfsX\"\x12 6\x90\xB6\xA8\x9E\x1A\xEB\x08\xC9i\x97\xE5z\xD2G\x97\x0F\xB2G\xB9\xD9oL\x84\x0C\xBB\t\x19U\x08_\xB2dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static DEBTFTOKENADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\0W`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x97W\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x02UW\x80c\xD3\xBF\xE7j\x14a\x02hW\x80c\xE1p\xA9\xBF\x14a\x02{W\x80c\xFAP\xE5\xD2\x14a\x02\x8EW`\0\x80\xFD[\x80c\x895:\t\x14a\x01\xDCW\x80c\x8B\x9B\x9D@\x14a\x01\xE3W\x80c\xAE\xFF\xDD\xDE\x14a\x01\xF6W\x80c\xB0\xE4Uo\x14a\x02\x16W`\0\x80\xFD[\x80ciD\\1\x11a\0\xD3W\x80ciD\\1\x14a\x01\x9BW\x80cm\xD6Y\xFD\x14a\x01\xAEW\x80cxASe\x14a\x01\xC1W\x80cy\x98\xA1\xC4\x14a\x01\xD4W`\0\x80\xFD[\x80c\x15\x86\xFF\xBB\x14a\x01\x05W\x80c\x19\x87uW\x14a\x01\x1AW\x80c\x1C\xAF\xF8\xB1\x14a\x01VW\x80c>\x03*;\x14a\x01\x8BW[`\0\x80\xFD[a\x01\x18a\x01\x136`\x04a\r\xF0V[a\x02\xA1V[\0[a\x01A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x01MV[`@Qa#(\x81R` \x01a\x01MV[a\x01\x18a\x01\xA96`\x04a\x0E\xE8V[a\x03 V[a\x01\x18a\x01\xBC6`\x04a\x0FUV[a\x039V[a\x01}a\x01\xCF6`\x04a\x0FrV[a\x03NV[a\x01}a\x03\xAAV[`\x01a\x01AV[a\x01\x18a\x01\xF16`\x04a\r\xF0V[a\x04\x13V[a\x02\ta\x02\x046`\x04a\x0FrV[a\x04\xC1V[`@Qa\x01M\x91\x90a\x0F\xA7V[a\x02=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01MV[a\x01\x18a\x02c6`\x04a\x0F\xF4V[a\x05$V[a\x01\x18a\x02v6`\x04a\x10tV[a\x05=V[a\x02=a\x02\x896`\x04a\x0FrV[a\x05VV[a\x01}a\x02\x9C6`\x04a\x10\xADV[a\x05\x7FV[a\x02\xAA\x82a\x05\x88V[a\x02\xB4\x81\x83a\x07BV[`\0a\x02\xBF\x83a\x07\xA6V[\x90Pa\x02\xCB\x83\x82a\x08\x14V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15a\x03\x1BW`@Qc\xC4\xBB\xF5\xFD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03B\x81a\x05\x88V[a\x03K\x81a\x08\xE1V[PV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03e\x91\x90a\x11\x11V[\x90P`\0a\x03s\x823a\tVV[\x90Pa\x03\xA2\x82\x82`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\xC5V[\x94\x93PPPPV[`\0`@Q` \x01a\x03\xF8\x90` \x80\x82R`\"\x90\x82\x01R\x7FFraxLend debtTokenV2 Adaptor V 1`@\x82\x01Ra\x02\xE3`\xF4\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x04\x1C\x82a\x05\x88V[`\0a\x04'\x83a\nKV[\x90P`\0a\x045\x82\x84a\n\xAFV[\x90P`\0a\x04G\x85\x83`\0`\x01a\x0B0V[\x90P`\0a\x04U\x860a\tVV[\x90P\x80`\0\x03a\x04\x83W`@Qcx\xD1xM`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x03\x12V[\x81\x81\x10\x15a\x04\x8FW\x80\x91P[a\x04\xA5`\x01`\x01`\xA0\x1B\x03\x85\x16\x87`\0\x19a\x0BpV[a\x04\xAF\x86\x83a\x0B\xEDV[a\x04\xB9\x84\x87a\x0C V[PPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x04\xEC\x82a\x05VV[\x81`\0\x81Q\x81\x10a\x04\xFFWa\x04\xFFa\x11.V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05R`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0BpV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x05m\x91\x90a\x11\x11V[\x90Pa\x05x\x81a\nKV[\x93\x92PPPV[`\0[\x92\x91PPV[`\0a\x05\x92a\x03\xAAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\x01\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05\xC8\x93\x92\x91` \x01a\x11DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06D\x91\x90a\x11\x11V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06q\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a\x11\xBCV[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1A\x91\x90a\x11\xD7V[a\x03\x1BW`@Qc}h@\xEF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x03\x12V[`@Qcr\xF8\x9D\x8B`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`\0`$\x82\x01R0`D\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xE5\xF1;\x16\x90`d\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x92W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04\xB9W=`\0\x80>=`\0\xFD[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFB\xBB\xF9L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\n\x91\x90a\x11\xF9V[`\x80\x01Q\x92\x91PPV[`\0\x80a\x08!\x840a\tVV[\x90P`\0a\x082\x85\x83`\x01\x80a\t\xC5V[\x90P\x80`\0\x03a\x08NWg\x0E\x92Yo\xD6)\0\0\x92PPPa\x05\x82V[`\0a\x08Z\x860a\x0C\xAFV[\x90P\x80`\0\x03a\x08pW`\0\x93PPPPa\x05\x82V[`\0\x80a\x08|\x88a\x0C\xE1V[\x90\x92P\x90P`\0\x83\x83\x83a\x08\x90\x8B\x89a\x12mV[a\x08\x9A\x91\x90a\x12\x92V[a\x08\xA4\x91\x90a\x12mV[a\x08\xAE\x91\x90a\x12\x92V[\x90P`\0a\x08\xBB\x8Aa\rXV[\x90P`\0a\x08\xD2\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\r\xBCV[\x9B\x9APPPPPPPPPPPV[`@Qc\x1Cl\x95\x97`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x1Cl\x95\x97\x90`$\x01a\x01\x80`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tM\x91\x90a\x13JV[PPPPPPPV[`@QcO\xD4\"\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90cO\xD4\"\xDF\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05x\x91\x90a\x14\x10V[`@Qc~\xC4\xB5q`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c~\xC4\xB5q\x90`d\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nB\x91\x90a\x14\x10V[\x95\x94PPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x8BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a\x11\x11V[`\0`\0\x19\x82\x03a\x0B)W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xFEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\"\x91\x90a\x14\x10V[\x90Pa\x05\x82V[P\x80a\x05\x82V[`@Qc\x1C%\x91\xD3`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c\x1C%\x91\xD3\x90`d\x01a\n\x01V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0B\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x03\x12V[PPPPV[`@Qc=A}-`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c=A}-\x90`D\x01a\x07xV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x94\x91\x90a\x14\x10V[\x11\x15a\x05RWa\x05R`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0BpV[`@QcZ\xD7\x981`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90c\xB5\xAF0b\x90`$\x01a\t\x84V[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c\x9A)^s`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rG\x91\x90a\x14)V[P\x95\x99\x91\x98P\x90\x96PPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x84\xBD\x05`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x82\x91\x90a\x14\x10V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\r\xD4W`\0\x80\xFD[\x04\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03KW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x03W`\0\x80\xFD[\x825a\x0E\x0E\x81a\r\xDBV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0EUWa\x0EUa\x0E\x1CV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0ElW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x87Wa\x0E\x87a\x0E\x1CV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xAFWa\x0E\xAFa\x0E\x1CV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xC8W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xFDW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\x1CW`\0\x80\xFD[a\x0F(\x87\x83\x88\x01a\x0E[V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0F>W`\0\x80\xFD[Pa\x0FK\x86\x82\x87\x01a\x0E[V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0FgW`\0\x80\xFD[\x815a\x05x\x81a\r\xDBV[`\0` \x82\x84\x03\x12\x15a\x0F\x84W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9BW`\0\x80\xFD[a\x03\xA2\x84\x82\x85\x01a\x0E[V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xE8W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\xC3V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\nW`\0\x80\xFD[\x845\x93P` \x85\x015a\x10\x1C\x81a\r\xDBV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x109W`\0\x80\xFD[a\x10E\x88\x83\x89\x01a\x0E[V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x10[W`\0\x80\xFD[Pa\x10h\x87\x82\x88\x01a\x0E[V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\x87W`\0\x80\xFD[\x825a\x10\x92\x81a\r\xDBV[\x91P` \x83\x015a\x10\xA2\x81a\r\xDBV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xC0W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10\xD8W`\0\x80\xFD[a\x10\xE4\x86\x83\x87\x01a\x0E[V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x10\xFAW`\0\x80\xFD[Pa\x11\x07\x85\x82\x86\x01a\x0E[V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11#W`\0\x80\xFD[\x81Qa\x05x\x81a\r\xDBV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`\0` \x84\x15\x15\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x11\x80W\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x11dV[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x11\xCEW`\0\x80\xFD[a\x05x\x82a\x11\xA3V[`\0` \x82\x84\x03\x12\x15a\x11\xE9W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05xW`\0\x80\xFD[`\0`\xA0\x82\x84\x03\x12\x15a\x12\x0BW`\0\x80\xFD[a\x12\x13a\x0E2V[\x82Qa\x12\x1E\x81a\r\xDBV[\x81Ra\x12,` \x84\x01a\x11\xA3V[` \x82\x01R`@\x83\x01Q`\x01`\x01`\xB8\x1B\x03\x81\x16\x81\x14a\x12KW`\0\x80\xFD[`@\x82\x01R``\x83\x81\x01Q\x90\x82\x01R`\x80\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x82WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x12\xAFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xB7W`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a\x12\xFEW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x13!Wa\x13!a\x0E\x1CV[`@R\x90P\x80a\x130\x83a\x12\xCCV[\x81Ra\x13>` \x84\x01a\x12\xCCV[` \x82\x01RP\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01\x80\x81\x12\x15a\x13eW`\0\x80\xFD[\x87Q\x96P` \x88\x01Q\x95P`@\x88\x01Q\x94P`\xA0`_\x19\x82\x01\x12\x15a\x13\x89W`\0\x80\xFD[Pa\x13\x92a\x0E2V[a\x13\x9E``\x89\x01a\x11\xA3V[\x81Ra\x13\xAC`\x80\x89\x01a\x11\xA3V[` \x82\x01Ra\x13\xBD`\xA0\x89\x01a\x12\xB4V[`@\x82\x01Ra\x13\xCE`\xC0\x89\x01a\x12\xB4V[``\x82\x01Ra\x13\xDF`\xE0\x89\x01a\x12\xB4V[`\x80\x82\x01R\x92Pa\x13\xF4\x88a\x01\0\x89\x01a\x12\xECV[\x91Pa\x14\x04\x88a\x01@\x89\x01a\x12\xECV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x14\"W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x14FW`\0\x80\xFD[PP\x86Q` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x8C\x01Q`\xC0\x8D\x01Q`\xE0\x90\x9D\x01Q\x95\x9E\x94\x9DP\x92\x9B\x91\x9AP\x98P\x90\x96P\x94P\x90\x92P\x90PV\xFE\xA2dipfsX\"\x12 6\x90\xB6\xA8\x9E\x1A\xEB\x08\xC9i\x97\xE5z\xD2G\x97\x0F\xB2G\xB9\xD9oL\x84\x0C\xBB\t\x19U\x08_\xB2dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static DEBTFTOKENADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DebtFTokenAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DebtFTokenAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DebtFTokenAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DebtFTokenAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DebtFTokenAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DebtFTokenAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DebtFTokenAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEBTFTOKENADAPTORV1_ABI.clone(),
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
                DEBTFTOKENADAPTORV1_ABI.clone(),
                DEBTFTOKENADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `ACCOUNT_FOR_INTEREST` (0x19877557) function
        pub fn account_for_interest(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([25, 135, 117, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FRAX` (0xb0e4556f) function
        pub fn frax(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([176, 228, 85, 111], ())
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `borrowFromFraxlend` (0x1586ffbb) function
        pub fn borrow_from_fraxlend(
            &self,
            fraxlend_pair: ::ethers::core::types::Address,
            amount_to_borrow: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 134, 255, 187], (fraxlend_pair, amount_to_borrow))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callAddInterest` (0x6dd659fd) function
        pub fn call_add_interest(
            &self,
            fraxlend_pair: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 214, 89, 253], fraxlend_pair)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x69445c31) function
        pub fn deposit(
            &self,
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::Bytes,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 68, 92, 49], (p0, p1, p2))
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
        ///Calls the contract's `minimumHealthFactor` (0x1caff8b1) function
        pub fn minimum_health_factor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([28, 175, 248, 177], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `repayFraxlendDebt` (0x8b9b9d40) function
        pub fn repay_fraxlend_debt(
            &self,
            fraxlend_pair: ::ethers::core::types::Address,
            debt_token_repay_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [139, 155, 157, 64],
                    (fraxlend_pair, debt_token_repay_amount),
                )
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
            p0: ::ethers::core::types::U256,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 17, 27, 215], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawableFrom` (0xfa50e5d2) function
        pub fn withdrawable_from(
            &self,
            p0: ::ethers::core::types::Bytes,
            p1: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DebtFTokenAdaptorV1<M> {
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
    ///Custom Error type `DebtFTokenAdaptor__CannotRepayNoDebt` with signature `DebtFTokenAdaptor__CannotRepayNoDebt(address)` and selector `0x78d1784d`
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
        name = "DebtFTokenAdaptor__CannotRepayNoDebt",
        abi = "DebtFTokenAdaptor__CannotRepayNoDebt(address)"
    )]
    pub struct DebtFTokenAdaptor__CannotRepayNoDebt {
        pub fraxlend_pair: ::ethers::core::types::Address,
    }
    ///Custom Error type `DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked` with signature `DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(address)` and selector `0xfad081de`
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
        name = "DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
        abi = "DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(address)"
    )]
    pub struct DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked {
        pub fraxlend_pair: ::ethers::core::types::Address,
    }
    ///Custom Error type `DebtFTokenAdaptor__HealthFactorTooLow` with signature `DebtFTokenAdaptor__HealthFactorTooLow(address)` and selector `0xc4bbf5fd`
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
        name = "DebtFTokenAdaptor__HealthFactorTooLow",
        abi = "DebtFTokenAdaptor__HealthFactorTooLow(address)"
    )]
    pub struct DebtFTokenAdaptor__HealthFactorTooLow {
        pub fraxlend_pair: ::ethers::core::types::Address,
    }
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
    pub enum DebtFTokenAdaptorV1Errors {
        BaseAdaptor__ConstructorHealthFactorTooLow(
            BaseAdaptor__ConstructorHealthFactorTooLow,
        ),
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        DebtFTokenAdaptor__CannotRepayNoDebt(DebtFTokenAdaptor__CannotRepayNoDebt),
        DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(
            DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked,
        ),
        DebtFTokenAdaptor__HealthFactorTooLow(DebtFTokenAdaptor__HealthFactorTooLow),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DebtFTokenAdaptorV1Errors {
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
            if let Ok(decoded) = <DebtFTokenAdaptor__CannotRepayNoDebt as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtFTokenAdaptor__CannotRepayNoDebt(decoded));
            }
            if let Ok(decoded) = <DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(decoded),
                );
            }
            if let Ok(decoded) = <DebtFTokenAdaptor__HealthFactorTooLow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtFTokenAdaptor__HealthFactorTooLow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DebtFTokenAdaptorV1Errors {
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
                Self::DebtFTokenAdaptor__CannotRepayNoDebt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtFTokenAdaptor__HealthFactorTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DebtFTokenAdaptorV1Errors {
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
                _ if selector
                    == <DebtFTokenAdaptor__CannotRepayNoDebt as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <DebtFTokenAdaptor__HealthFactorTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DebtFTokenAdaptorV1Errors {
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
                Self::DebtFTokenAdaptor__CannotRepayNoDebt(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DebtFTokenAdaptor__HealthFactorTooLow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DebtFTokenAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ConstructorHealthFactorTooLow>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__ConstructorHealthFactorTooLow) -> Self {
            Self::BaseAdaptor__ConstructorHealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage> for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    impl ::core::convert::From<DebtFTokenAdaptor__CannotRepayNoDebt>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: DebtFTokenAdaptor__CannotRepayNoDebt) -> Self {
            Self::DebtFTokenAdaptor__CannotRepayNoDebt(value)
        }
    }
    impl ::core::convert::From<DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked) -> Self {
            Self::DebtFTokenAdaptor__FraxlendPairPositionsMustBeTracked(value)
        }
    }
    impl ::core::convert::From<DebtFTokenAdaptor__HealthFactorTooLow>
    for DebtFTokenAdaptorV1Errors {
        fn from(value: DebtFTokenAdaptor__HealthFactorTooLow) -> Self {
            Self::DebtFTokenAdaptor__HealthFactorTooLow(value)
        }
    }
    ///Container type for all input parameters for the `ACCOUNT_FOR_INTEREST` function with signature `ACCOUNT_FOR_INTEREST()` and selector `0x19877557`
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
    #[ethcall(name = "ACCOUNT_FOR_INTEREST", abi = "ACCOUNT_FOR_INTEREST()")]
    pub struct AccountForInterestCall;
    ///Container type for all input parameters for the `FRAX` function with signature `FRAX()` and selector `0xb0e4556f`
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
    #[ethcall(name = "FRAX", abi = "FRAX()")]
    pub struct FraxCall;
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
    ///Container type for all input parameters for the `borrowFromFraxlend` function with signature `borrowFromFraxlend(address,uint256)` and selector `0x1586ffbb`
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
    #[ethcall(name = "borrowFromFraxlend", abi = "borrowFromFraxlend(address,uint256)")]
    pub struct BorrowFromFraxlendCall {
        pub fraxlend_pair: ::ethers::core::types::Address,
        pub amount_to_borrow: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `callAddInterest` function with signature `callAddInterest(address)` and selector `0x6dd659fd`
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
    #[ethcall(name = "callAddInterest", abi = "callAddInterest(address)")]
    pub struct CallAddInterestCall {
        pub fraxlend_pair: ::ethers::core::types::Address,
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
    pub struct DepositCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
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
    ///Container type for all input parameters for the `minimumHealthFactor` function with signature `minimumHealthFactor()` and selector `0x1caff8b1`
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
    #[ethcall(name = "minimumHealthFactor", abi = "minimumHealthFactor()")]
    pub struct MinimumHealthFactorCall;
    ///Container type for all input parameters for the `repayFraxlendDebt` function with signature `repayFraxlendDebt(address,uint256)` and selector `0x8b9b9d40`
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
    #[ethcall(name = "repayFraxlendDebt", abi = "repayFraxlendDebt(address,uint256)")]
    pub struct RepayFraxlendDebtCall {
        pub fraxlend_pair: ::ethers::core::types::Address,
        pub debt_token_repay_amount: ::ethers::core::types::U256,
    }
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
    pub struct WithdrawCall(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
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
    pub struct WithdrawableFromCall(
        pub ::ethers::core::types::Bytes,
        pub ::ethers::core::types::Bytes,
    );
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
    pub enum DebtFTokenAdaptorV1Calls {
        AccountForInterest(AccountForInterestCall),
        Frax(FraxCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BorrowFromFraxlend(BorrowFromFraxlendCall),
        CallAddInterest(CallAddInterestCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        MinimumHealthFactor(MinimumHealthFactorCall),
        RepayFraxlendDebt(RepayFraxlendDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for DebtFTokenAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccountForInterestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccountForInterest(decoded));
            }
            if let Ok(decoded) = <FraxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Frax(decoded));
            }
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
            if let Ok(decoded) = <BorrowFromFraxlendCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BorrowFromFraxlend(decoded));
            }
            if let Ok(decoded) = <CallAddInterestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallAddInterest(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
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
            if let Ok(decoded) = <MinimumHealthFactorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumHealthFactor(decoded));
            }
            if let Ok(decoded) = <RepayFraxlendDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayFraxlendDebt(decoded));
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
            if let Ok(decoded) = <WithdrawableFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DebtFTokenAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AccountForInterest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Frax(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowFromFraxlend(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallAddInterest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinimumHealthFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RepayFraxlendDebt(element) => {
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
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DebtFTokenAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccountForInterest(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Frax(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowFromFraxlend(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CallAddInterest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumHealthFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RepayFraxlendDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountForInterestCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: AccountForInterestCall) -> Self {
            Self::AccountForInterest(value)
        }
    }
    impl ::core::convert::From<FraxCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: FraxCall) -> Self {
            Self::Frax(value)
        }
    }
    impl ::core::convert::From<AssetOfCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BorrowFromFraxlendCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: BorrowFromFraxlendCall) -> Self {
            Self::BorrowFromFraxlend(value)
        }
    }
    impl ::core::convert::From<CallAddInterestCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: CallAddInterestCall) -> Self {
            Self::CallAddInterest(value)
        }
    }
    impl ::core::convert::From<DepositCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<MinimumHealthFactorCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: MinimumHealthFactorCall) -> Self {
            Self::MinimumHealthFactor(value)
        }
    }
    impl ::core::convert::From<RepayFraxlendDebtCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: RepayFraxlendDebtCall) -> Self {
            Self::RepayFraxlendDebt(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for DebtFTokenAdaptorV1Calls {
        fn from(value: WithdrawableFromCall) -> Self {
            Self::WithdrawableFrom(value)
        }
    }
    ///Container type for all return fields from the `ACCOUNT_FOR_INTEREST` function with signature `ACCOUNT_FOR_INTEREST()` and selector `0x19877557`
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
    pub struct AccountForInterestReturn(pub bool);
    ///Container type for all return fields from the `FRAX` function with signature `FRAX()` and selector `0xb0e4556f`
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
    pub struct FraxReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `minimumHealthFactor` function with signature `minimumHealthFactor()` and selector `0x1caff8b1`
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
    pub struct MinimumHealthFactorReturn(pub ::ethers::core::types::U256);
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
