pub use fees_and_reserves_adaptor_v1::*;
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
pub mod fees_and_reserves_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/FeesAndReservesAdaptorV1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("addAssetsToReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addAssetsToReserves",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("assetOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetOf"),
                            inputs: ::std::vec![
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
                (
                    ::std::borrow::ToOwned::to_owned("changeUpkeepFrequency"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "changeUpkeepFrequency",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newFrequency"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("changeUpkeepMaxGas"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeUpkeepMaxGas"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMaxGas"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("feesAndReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feesAndReserves"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract FeesAndReserves"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("prepareFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("setupMetaData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setupMetaData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("managementFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("performanceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("updateManagementFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateManagementFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("managementFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("updatePerformanceFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updatePerformanceFee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("performanceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawAssetsFromReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawAssetsFromReserves",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
    pub static FEESANDRESERVESADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\rd\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c\xAE\xFF\xDD\xDE\x11a\0\xADW\x80c\xE1p\xA9\xBF\x11a\0qW\x80c\xE1p\xA9\xBF\x14a\x02\\W\x80c\xF2\xA4MN\x14a\x02jW\x80c\xF5\xF3\n^\x14a\x02}W\x80c\xFAP\xE5\xD2\x14a\x02\x90W\x80c\xFAS\x12U\x14a\x02\xA3W`\0\x80\xFD[\x80c\xAE\xFF\xDD\xDE\x14a\x01\xF0W\x80c\xC1\xCC\x8D\x7F\x14a\x02\x10W\x80c\xC9\x11\x1B\xD7\x14a\x02#W\x80c\xCA\xBA\xD5\n\x14a\x026W\x80c\xD3\xBF\xE7j\x14a\x02IW`\0\x80\xFD[\x80ciD\\1\x11a\0\xF4W\x80ciD\\1\x14a\x01\x91W\x80cxASe\x14a\x01\xA4W\x80cy\x98\xA1\xC4\x14a\x01\xC6W\x80c\x895:\t\x14a\x01\xCEW\x80c\xA5\xA3m\xD2\x14a\x01\xDDW`\0\x80\xFD[\x80c\x0B\x05\x07\t\x14a\x01&W\x80c>\x03*;\x14a\x01YW\x80cS\x03\x90}\x14a\x01iW\x80cUm\x18\xDF\x14a\x01~W[`\0\x80\xFD[s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qa#(\x81R` \x01a\x01PV[a\x01|a\x01w6`\x04a\x08\xBEV[a\x02\xB6V[\0[a\x01|a\x01\x8C6`\x04a\x08\xECV[a\x03\x1FV[a\x01|a\x01\x9F6`\x04a\t\xB3V[a\x03\\V[a\x01\xB8a\x01\xB26`\x04a\n V[P`\0\x90V[`@Q\x90\x81R` \x01a\x01PV[a\x01\xB8a\x03uV[`@Q`\0\x81R` \x01a\x01PV[a\x01|a\x01\xEB6`\x04a\nsV[a\x03\xD3V[a\x02\x03a\x01\xFE6`\x04a\n V[a\x04\x14V[`@Qa\x01P\x91\x90a\n\x90V[a\x01|a\x02\x1E6`\x04a\x08\xECV[a\x04pV[a\x01|a\x0216`\x04a\n\xF2V[a\x04\xADV[a\x01|a\x02D6`\x04a\x08\xBEV[a\x04\xC6V[a\x01|a\x02W6`\x04a\x0BrV[a\x06\x14V[a\x01<a\x01\xB26`\x04a\n V[a\x01|a\x02x6`\x04a\nsV[a\x06)V[a\x01|a\x02\x8B6`\x04a\x08\xBEV[a\x06jV[a\x01\xB8a\x02\x9E6`\x04a\x0B\xABV[a\x06\xA2V[a\x01|a\x02\xB16`\x04a\x0C\x0FV[a\x06\xABV[`@QcS\x03\x90}`\xE0\x1B\x81R`\x04\x81\x01\x82\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90cS\x03\x90}\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x18W=`\0\x80>=`\0\xFD[PPPPPV[`@QcUm\x18\xDF`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90cUm\x18\xDF\x90`$\x01a\x02\xEAV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q` \x01a\x03\xB8\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FFees And Reserves Adaptor V 1.0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@QcR\xD1\xB6\xE9`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xA5\xA3m\xD2\x90`$\x01a\x02\xEAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x04KWa\x04Ka\x0C=V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc\xC1\xCC\x8D\x7F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xC1\xCC\x8D\x7F\x90`$\x01a\x02\xEAV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98`@Qc\x06\x86\xDD\xD9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x06\x86\xDD\xD9\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05H\x91\x90a\x0CiV[PPPPPPPPP\x90Pa\x05]\x81\x83a\x07-V[\x91Pa\x05\x87`\x01`\x01`\xA0\x1B\x03\x82\x16s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x84a\x07\xAEV[`@Qce]j\x85`\xE1\x1B\x81R`\x04\x81\x01\x83\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xCA\xBA\xD5\n\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xE8W=`\0\x80>=`\0\xFD[PPPPa\x06\x10\x81a\x06\x0Bs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90V[a\x08/V[PPV[a\x06\x10`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x07\xAEV[`@QcyR&\xA7`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xF2\xA4MN\x90`$\x01a\x02\xEAV[`@Qcz\xF9\x85/`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xF5\xF3\n^\x90`$\x01a\x02\xEAV[`\0[\x92\x91PPV[s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98`@Qc\xFAS\x12U`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xFAS\x12U\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07%W=`\0\x80>=`\0\xFD[PPPPPPV[`\0`\0\x19\x82\x03a\x07\xA7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA0\x91\x90a\r\x15V[\x90Pa\x06\xA5V[P\x80a\x06\xA5V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x08)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA3\x91\x90a\r\x15V[\x11\x15a\x06\x10Wa\x06\x10`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x07\xAEV[`\0` \x82\x84\x03\x12\x15a\x08\xD0W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x08\xFEW`\0\x80\xFD[\x815a\t\t\x81a\x08\xD7V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\t7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tRWa\tRa\t\x10V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\tzWa\tza\t\x10V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\t\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xC8W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xE7W`\0\x80\xFD[a\t\xF3\x87\x83\x88\x01a\t&V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\n\tW`\0\x80\xFD[Pa\n\x16\x86\x82\x87\x01a\t&V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\n2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nIW`\0\x80\xFD[a\nU\x84\x82\x85\x01a\t&V[\x94\x93PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\x85W`\0\x80\xFD[\x815a\t\t\x81a\n]V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\n\xD1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\n\xACV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0B\x08W`\0\x80\xFD[\x845\x93P` \x85\x015a\x0B\x1A\x81a\n\xDDV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B7W`\0\x80\xFD[a\x0BC\x88\x83\x89\x01a\t&V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0BYW`\0\x80\xFD[Pa\x0Bf\x87\x82\x88\x01a\t&V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x85W`\0\x80\xFD[\x825a\x0B\x90\x81a\n\xDDV[\x91P` \x83\x015a\x0B\xA0\x81a\n\xDDV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xBEW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xD6W`\0\x80\xFD[a\x0B\xE2\x86\x83\x87\x01a\t&V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0B\xF8W`\0\x80\xFD[Pa\x0C\x05\x85\x82\x86\x01a\t&V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\"W`\0\x80\xFD[\x825a\x0C-\x81a\x08\xD7V[\x91P` \x83\x015a\x0B\xA0\x81a\x08\xD7V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q`\xFF\x81\x16\x81\x14a\x0CdW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\x0C\x89W`\0\x80\xFD[\x8AQa\x0C\x94\x81a\n\xDDV[` \x8C\x01Q\x90\x9APa\x0C\xA5\x81a\x08\xD7V[`@\x8C\x01Q\x90\x99Pa\x0C\xB6\x81a\n]V[\x80\x98PP``\x8B\x01Q\x96P`\x80\x8B\x01Q\x95P`\xA0\x8B\x01Q\x94P`\xC0\x8B\x01Q\x93Pa\x0C\xE2`\xE0\x8C\x01a\x0CSV[\x92Pa\x0C\xF1a\x01\0\x8C\x01a\x0CSV[\x91Pa\x01 \x8B\x01Qa\r\x02\x81a\x08\xD7V[\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\r'W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0C\x03EJ\xA1\xF5~\xC2Tt 5\xD0\xB0bV:\xE6?\xC8l\xE5\x1F\n2\x05m\x10\x90\x82\xBF\xB5dsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static FEESANDRESERVESADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01!W`\x005`\xE0\x1C\x80c\xAE\xFF\xDD\xDE\x11a\0\xADW\x80c\xE1p\xA9\xBF\x11a\0qW\x80c\xE1p\xA9\xBF\x14a\x02\\W\x80c\xF2\xA4MN\x14a\x02jW\x80c\xF5\xF3\n^\x14a\x02}W\x80c\xFAP\xE5\xD2\x14a\x02\x90W\x80c\xFAS\x12U\x14a\x02\xA3W`\0\x80\xFD[\x80c\xAE\xFF\xDD\xDE\x14a\x01\xF0W\x80c\xC1\xCC\x8D\x7F\x14a\x02\x10W\x80c\xC9\x11\x1B\xD7\x14a\x02#W\x80c\xCA\xBA\xD5\n\x14a\x026W\x80c\xD3\xBF\xE7j\x14a\x02IW`\0\x80\xFD[\x80ciD\\1\x11a\0\xF4W\x80ciD\\1\x14a\x01\x91W\x80cxASe\x14a\x01\xA4W\x80cy\x98\xA1\xC4\x14a\x01\xC6W\x80c\x895:\t\x14a\x01\xCEW\x80c\xA5\xA3m\xD2\x14a\x01\xDDW`\0\x80\xFD[\x80c\x0B\x05\x07\t\x14a\x01&W\x80c>\x03*;\x14a\x01YW\x80cS\x03\x90}\x14a\x01iW\x80cUm\x18\xDF\x14a\x01~W[`\0\x80\xFD[s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qa#(\x81R` \x01a\x01PV[a\x01|a\x01w6`\x04a\x08\xBEV[a\x02\xB6V[\0[a\x01|a\x01\x8C6`\x04a\x08\xECV[a\x03\x1FV[a\x01|a\x01\x9F6`\x04a\t\xB3V[a\x03\\V[a\x01\xB8a\x01\xB26`\x04a\n V[P`\0\x90V[`@Q\x90\x81R` \x01a\x01PV[a\x01\xB8a\x03uV[`@Q`\0\x81R` \x01a\x01PV[a\x01|a\x01\xEB6`\x04a\nsV[a\x03\xD3V[a\x02\x03a\x01\xFE6`\x04a\n V[a\x04\x14V[`@Qa\x01P\x91\x90a\n\x90V[a\x01|a\x02\x1E6`\x04a\x08\xECV[a\x04pV[a\x01|a\x0216`\x04a\n\xF2V[a\x04\xADV[a\x01|a\x02D6`\x04a\x08\xBEV[a\x04\xC6V[a\x01|a\x02W6`\x04a\x0BrV[a\x06\x14V[a\x01<a\x01\xB26`\x04a\n V[a\x01|a\x02x6`\x04a\nsV[a\x06)V[a\x01|a\x02\x8B6`\x04a\x08\xBEV[a\x06jV[a\x01\xB8a\x02\x9E6`\x04a\x0B\xABV[a\x06\xA2V[a\x01|a\x02\xB16`\x04a\x0C\x0FV[a\x06\xABV[`@QcS\x03\x90}`\xE0\x1B\x81R`\x04\x81\x01\x82\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90cS\x03\x90}\x90`$\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x04W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\x18W=`\0\x80>=`\0\xFD[PPPPPV[`@QcUm\x18\xDF`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90cUm\x18\xDF\x90`$\x01a\x02\xEAV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q` \x01a\x03\xB8\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FFees And Reserves Adaptor V 1.0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@QcR\xD1\xB6\xE9`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xA5\xA3m\xD2\x90`$\x01a\x02\xEAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x04KWa\x04Ka\x0C=V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc\xC1\xCC\x8D\x7F`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xC1\xCC\x8D\x7F\x90`$\x01a\x02\xEAV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98`@Qc\x06\x86\xDD\xD9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\x06\x86\xDD\xD9\x90`$\x01a\x01@`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05H\x91\x90a\x0CiV[PPPPPPPPP\x90Pa\x05]\x81\x83a\x07-V[\x91Pa\x05\x87`\x01`\x01`\xA0\x1B\x03\x82\x16s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x84a\x07\xAEV[`@Qce]j\x85`\xE1\x1B\x81R`\x04\x81\x01\x83\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xCA\xBA\xD5\n\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x05\xD4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xE8W=`\0\x80>=`\0\xFD[PPPPa\x06\x10\x81a\x06\x0Bs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90V[a\x08/V[PPV[a\x06\x10`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x07\xAEV[`@QcyR&\xA7`\xE1\x1B\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xF2\xA4MN\x90`$\x01a\x02\xEAV[`@Qcz\xF9\x85/`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98\x90c\xF5\xF3\n^\x90`$\x01a\x02\xEAV[`\0[\x92\x91PPV[s\xA0\xCB\x88\x97\x07\xD4&\xA7\xA3\x86\x87\n\x03\xBCp\xD1\xB0iu\x98`@Qc\xFAS\x12U`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xFAS\x12U\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07%W=`\0\x80>=`\0\xFD[PPPPPPV[`\0`\0\x19\x82\x03a\x07\xA7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA0\x91\x90a\r\x15V[\x90Pa\x06\xA5V[P\x80a\x06\xA5V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x08)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA3\x91\x90a\r\x15V[\x11\x15a\x06\x10Wa\x06\x10`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x07\xAEV[`\0` \x82\x84\x03\x12\x15a\x08\xD0W`\0\x80\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x08\xFEW`\0\x80\xFD[\x815a\t\t\x81a\x08\xD7V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\t7W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\tRWa\tRa\t\x10V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\tzWa\tza\t\x10V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\t\x93W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\t\xC8W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xE7W`\0\x80\xFD[a\t\xF3\x87\x83\x88\x01a\t&V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\n\tW`\0\x80\xFD[Pa\n\x16\x86\x82\x87\x01a\t&V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\n2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nIW`\0\x80\xFD[a\nU\x84\x82\x85\x01a\t&V[\x94\x93PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\n\x85W`\0\x80\xFD[\x815a\t\t\x81a\n]V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\n\xD1W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\n\xACV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xE9W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0B\x08W`\0\x80\xFD[\x845\x93P` \x85\x015a\x0B\x1A\x81a\n\xDDV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B7W`\0\x80\xFD[a\x0BC\x88\x83\x89\x01a\t&V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0BYW`\0\x80\xFD[Pa\x0Bf\x87\x82\x88\x01a\t&V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\x85W`\0\x80\xFD[\x825a\x0B\x90\x81a\n\xDDV[\x91P` \x83\x015a\x0B\xA0\x81a\n\xDDV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\xBEW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xD6W`\0\x80\xFD[a\x0B\xE2\x86\x83\x87\x01a\t&V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0B\xF8W`\0\x80\xFD[Pa\x0C\x05\x85\x82\x86\x01a\t&V[\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\"W`\0\x80\xFD[\x825a\x0C-\x81a\x08\xD7V[\x91P` \x83\x015a\x0B\xA0\x81a\x08\xD7V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q`\xFF\x81\x16\x81\x14a\x0CdW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15a\x0C\x89W`\0\x80\xFD[\x8AQa\x0C\x94\x81a\n\xDDV[` \x8C\x01Q\x90\x9APa\x0C\xA5\x81a\x08\xD7V[`@\x8C\x01Q\x90\x99Pa\x0C\xB6\x81a\n]V[\x80\x98PP``\x8B\x01Q\x96P`\x80\x8B\x01Q\x95P`\xA0\x8B\x01Q\x94P`\xC0\x8B\x01Q\x93Pa\x0C\xE2`\xE0\x8C\x01a\x0CSV[\x92Pa\x0C\xF1a\x01\0\x8C\x01a\x0CSV[\x91Pa\x01 \x8B\x01Qa\r\x02\x81a\x08\xD7V[\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15a\r'W`\0\x80\xFD[PQ\x91\x90PV\xFE\xA2dipfsX\"\x12 \x0C\x03EJ\xA1\xF5~\xC2Tt 5\xD0\xB0bV:\xE6?\xC8l\xE5\x1F\n2\x05m\x10\x90\x82\xBF\xB5dsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static FEESANDRESERVESADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FeesAndReservesAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FeesAndReservesAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FeesAndReservesAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FeesAndReservesAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FeesAndReservesAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FeesAndReservesAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FeesAndReservesAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FEESANDRESERVESADAPTORV1_ABI.clone(),
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
                FEESANDRESERVESADAPTORV1_ABI.clone(),
                FEESANDRESERVESADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `addAssetsToReserves` (0xcabad50a) function
        pub fn add_assets_to_reserves(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 186, 213, 10], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetOf` (0xe170a9bf) function
        pub fn asset_of(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([225, 112, 169, 191], p0)
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
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([120, 65, 83, 101], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeUpkeepFrequency` (0xa5a36dd2) function
        pub fn change_upkeep_frequency(
            &self,
            new_frequency: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 163, 109, 210], new_frequency)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeUpkeepMaxGas` (0xf2a44d4e) function
        pub fn change_upkeep_max_gas(
            &self,
            new_max_gas: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 164, 77, 78], new_max_gas)
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
        ///Calls the contract's `feesAndReserves` (0x0b050709) function
        pub fn fees_and_reserves(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([11, 5, 7, 9], ())
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
        ///Calls the contract's `prepareFees` (0x5303907d) function
        pub fn prepare_fees(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 3, 144, 125], amount)
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
        ///Calls the contract's `setupMetaData` (0xfa531255) function
        pub fn setup_meta_data(
            &self,
            management_fee: u32,
            performance_fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 83, 18, 85], (management_fee, performance_fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slippage` (0x3e032a3b) function
        pub fn slippage(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([62, 3, 42, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateManagementFee` (0xc1cc8d7f) function
        pub fn update_management_fee(
            &self,
            management_fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 204, 141, 127], management_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePerformanceFee` (0x556d18df) function
        pub fn update_performance_fee(
            &self,
            performance_fee: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 109, 24, 223], performance_fee)
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
        ///Calls the contract's `withdrawAssetsFromReserves` (0xf5f30a5e) function
        pub fn withdraw_assets_from_reserves(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 243, 10, 94], amount)
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
    for FeesAndReservesAdaptorV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum FeesAndReservesAdaptorV1Errors {
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FeesAndReservesAdaptorV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
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
    impl ::ethers::core::abi::AbiEncode for FeesAndReservesAdaptorV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
    impl ::ethers::contract::ContractRevert for FeesAndReservesAdaptorV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
    impl ::core::fmt::Display for FeesAndReservesAdaptorV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
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
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage>
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for FeesAndReservesAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    ///Container type for all input parameters for the `addAssetsToReserves` function with signature `addAssetsToReserves(uint256)` and selector `0xcabad50a`
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
    #[ethcall(name = "addAssetsToReserves", abi = "addAssetsToReserves(uint256)")]
    pub struct AddAssetsToReservesCall {
        pub amount: ::ethers::core::types::U256,
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
    pub struct AssetOfCall(pub ::ethers::core::types::Bytes);
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
    pub struct BalanceOfCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `changeUpkeepFrequency` function with signature `changeUpkeepFrequency(uint64)` and selector `0xa5a36dd2`
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
    #[ethcall(name = "changeUpkeepFrequency", abi = "changeUpkeepFrequency(uint64)")]
    pub struct ChangeUpkeepFrequencyCall {
        pub new_frequency: u64,
    }
    ///Container type for all input parameters for the `changeUpkeepMaxGas` function with signature `changeUpkeepMaxGas(uint64)` and selector `0xf2a44d4e`
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
    #[ethcall(name = "changeUpkeepMaxGas", abi = "changeUpkeepMaxGas(uint64)")]
    pub struct ChangeUpkeepMaxGasCall {
        pub new_max_gas: u64,
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
    ///Container type for all input parameters for the `feesAndReserves` function with signature `feesAndReserves()` and selector `0x0b050709`
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
    #[ethcall(name = "feesAndReserves", abi = "feesAndReserves()")]
    pub struct FeesAndReservesCall;
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
    ///Container type for all input parameters for the `prepareFees` function with signature `prepareFees(uint256)` and selector `0x5303907d`
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
    #[ethcall(name = "prepareFees", abi = "prepareFees(uint256)")]
    pub struct PrepareFeesCall {
        pub amount: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setupMetaData` function with signature `setupMetaData(uint32,uint32)` and selector `0xfa531255`
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
    #[ethcall(name = "setupMetaData", abi = "setupMetaData(uint32,uint32)")]
    pub struct SetupMetaDataCall {
        pub management_fee: u32,
        pub performance_fee: u32,
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
    ///Container type for all input parameters for the `updateManagementFee` function with signature `updateManagementFee(uint32)` and selector `0xc1cc8d7f`
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
    #[ethcall(name = "updateManagementFee", abi = "updateManagementFee(uint32)")]
    pub struct UpdateManagementFeeCall {
        pub management_fee: u32,
    }
    ///Container type for all input parameters for the `updatePerformanceFee` function with signature `updatePerformanceFee(uint32)` and selector `0x556d18df`
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
    #[ethcall(name = "updatePerformanceFee", abi = "updatePerformanceFee(uint32)")]
    pub struct UpdatePerformanceFeeCall {
        pub performance_fee: u32,
    }
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
    ///Container type for all input parameters for the `withdrawAssetsFromReserves` function with signature `withdrawAssetsFromReserves(uint256)` and selector `0xf5f30a5e`
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
        name = "withdrawAssetsFromReserves",
        abi = "withdrawAssetsFromReserves(uint256)"
    )]
    pub struct WithdrawAssetsFromReservesCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum FeesAndReservesAdaptorV1Calls {
        AddAssetsToReserves(AddAssetsToReservesCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        ChangeUpkeepFrequency(ChangeUpkeepFrequencyCall),
        ChangeUpkeepMaxGas(ChangeUpkeepMaxGasCall),
        Deposit(DepositCall),
        FeesAndReserves(FeesAndReservesCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        PrepareFees(PrepareFeesCall),
        RevokeApproval(RevokeApprovalCall),
        SetupMetaData(SetupMetaDataCall),
        Slippage(SlippageCall),
        UpdateManagementFee(UpdateManagementFeeCall),
        UpdatePerformanceFee(UpdatePerformanceFeeCall),
        Withdraw(WithdrawCall),
        WithdrawAssetsFromReserves(WithdrawAssetsFromReservesCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for FeesAndReservesAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AddAssetsToReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAssetsToReserves(decoded));
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
            if let Ok(decoded) = <ChangeUpkeepFrequencyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeUpkeepFrequency(decoded));
            }
            if let Ok(decoded) = <ChangeUpkeepMaxGasCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeUpkeepMaxGas(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <FeesAndReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeesAndReserves(decoded));
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
            if let Ok(decoded) = <PrepareFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareFees(decoded));
            }
            if let Ok(decoded) = <RevokeApprovalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokeApproval(decoded));
            }
            if let Ok(decoded) = <SetupMetaDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetupMetaData(decoded));
            }
            if let Ok(decoded) = <SlippageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Slippage(decoded));
            }
            if let Ok(decoded) = <UpdateManagementFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateManagementFee(decoded));
            }
            if let Ok(decoded) = <UpdatePerformanceFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdatePerformanceFee(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawAssetsFromReservesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawAssetsFromReserves(decoded));
            }
            if let Ok(decoded) = <WithdrawableFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FeesAndReservesAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddAssetsToReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeUpkeepFrequency(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeUpkeepMaxGas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeesAndReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevokeApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetupMetaData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateManagementFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdatePerformanceFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawAssetsFromReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FeesAndReservesAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddAssetsToReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeUpkeepFrequency(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeUpkeepMaxGas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesAndReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetupMetaData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateManagementFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdatePerformanceFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawAssetsFromReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddAssetsToReservesCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: AddAssetsToReservesCall) -> Self {
            Self::AddAssetsToReserves(value)
        }
    }
    impl ::core::convert::From<AssetOfCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ChangeUpkeepFrequencyCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: ChangeUpkeepFrequencyCall) -> Self {
            Self::ChangeUpkeepFrequency(value)
        }
    }
    impl ::core::convert::From<ChangeUpkeepMaxGasCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: ChangeUpkeepMaxGasCall) -> Self {
            Self::ChangeUpkeepMaxGas(value)
        }
    }
    impl ::core::convert::From<DepositCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FeesAndReservesCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: FeesAndReservesCall) -> Self {
            Self::FeesAndReserves(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<PrepareFeesCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: PrepareFeesCall) -> Self {
            Self::PrepareFees(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SetupMetaDataCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: SetupMetaDataCall) -> Self {
            Self::SetupMetaData(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<UpdateManagementFeeCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: UpdateManagementFeeCall) -> Self {
            Self::UpdateManagementFee(value)
        }
    }
    impl ::core::convert::From<UpdatePerformanceFeeCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: UpdatePerformanceFeeCall) -> Self {
            Self::UpdatePerformanceFee(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for FeesAndReservesAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawAssetsFromReservesCall>
    for FeesAndReservesAdaptorV1Calls {
        fn from(value: WithdrawAssetsFromReservesCall) -> Self {
            Self::WithdrawAssetsFromReserves(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for FeesAndReservesAdaptorV1Calls {
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
    ///Container type for all return fields from the `feesAndReserves` function with signature `feesAndReserves()` and selector `0x0b050709`
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
    pub struct FeesAndReservesReturn(pub ::ethers::core::types::Address);
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
