pub use aave_debt_token_adaptor_v2::*;
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
pub mod aave_debt_token_adaptor_v2 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/AaveDebtTokenAdaptorV2.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("borrowFromAave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("borrowFromAave"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("debtTokenToBorrow"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
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
                    ::std::borrow::ToOwned::to_owned("flashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("flashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loanToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("loanAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
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
                    ::std::borrow::ToOwned::to_owned("repayAaveDebt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("repayAaveDebt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenToRepay"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToRepay"),
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
                        "AaveDebtTokenAdaptor__DebtPositionsMustBeTracked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AaveDebtTokenAdaptor__DebtPositionsMustBeTracked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "untrackedDebtPosition",
                                    ),
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
                        "AaveDebtTokenAdaptor__HealthFactorTooLow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AaveDebtTokenAdaptor__HealthFactorTooLow",
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
    pub static AAVEDEBTTOKENADAPTORV2_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x11M\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x9A\x85\xE1\xD3\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\x8FW\x80c\xD7\x0C\x159\x14a\x01\xA2W\x80c\xE1p\xA9\xBF\x14a\x01\xB5W\x80c\xFAP\xE5\xD2\x14a\x01\xE0W`\0\x80\xFD[\x80c\x9A\x85\xE1\xD3\x14a\x01IW\x80c\xAE\xFF\xDD\xDE\x14a\x01\\W\x80c\xC9\x11\x1B\xD7\x14a\x01|W`\0\x80\xFD[\x80c>\x03*;\x14a\0\xD4W\x80ciD\\1\x14a\0\xE9W\x80cxASe\x14a\0\xFEW\x80cx\x8F\xB4\x84\x14a\x01\x1FW\x80cy\x98\xA1\xC4\x14a\x012W\x80c\x895:\t\x14a\x01:W[`\0\x80\xFD[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFCa\0\xF76`\x04a\x0B2V[a\x01\xF6V[\0[a\x01\x11a\x01\x0C6`\x04a\x0B\x9FV[a\x02\x0FV[`@Q\x90\x81R` \x01a\0\xE0V[a\0\xFCa\x01-6`\x04a\x0C\x83V[a\x02\x98V[a\x01\x11a\x03\xACV[`@Q`\x01\x81R` \x01a\0\xE0V[a\0\xFCa\x01W6`\x04a\r<V[a\x04\nV[a\x01oa\x01j6`\x04a\x0B\x9FV[a\x07\x81V[`@Qa\0\xE0\x91\x90a\rhV[a\0\xFCa\x01\x8A6`\x04a\r\xB5V[a\x07\xE4V[a\0\xFCa\x01\x9D6`\x04a\x0E5V[a\x07\xFDV[a\0\xFCa\x01\xB06`\x04a\r<V[a\x08\x16V[a\x01\xC8a\x01\xC36`\x04a\x0B\x9FV[a\x08\xF4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE0V[a\x01\x11a\x01\xEE6`\x04a\x0EnV[`\0\x92\x91PPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x02&\x91\x90a\x0E\xD2V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x91\x91\x90a\x0E\xEFV[\x93\x92PPPV[\x81Q\x83Q\x14a\x02\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB78:\xBA\x1062\xB73\xBA4\x106\xB4\xB9\xB6\xB0\xBA1\xB4\x17`Q\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x03Wa\x03\x03a\n{V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03,W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Qc\xAB\x9CK]`\xE0\x1B\x81R\x90\x91Ps}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x90c\xAB\x9CK]\x90a\x03t\x900\x90\x88\x90\x88\x90\x87\x90\x84\x90\x8A\x90`\0\x90`\x04\x01a\x0F\x89V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xA2W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0`@Q` \x01a\x03\xEF\x90` \x80\x82R`\x1C\x90\x82\x01R\x7FAave debtToken Adaptor V 1.0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0a\x04\x14a\x03\xACV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R`\x01\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04J\x93\x92\x91` \x01a\x10EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC6\x91\x90a\x0E\xD2V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x054\x91\x90a\x10oV[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9C\x91\x90a\x10\x95V[a\x05\xC4W`@Qc;\xA3\x97a`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xDEV[s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9`\x01`\x01`\xA0\x1B\x03\x16c\xA4\x15\xBC\xAD\x85`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06I\x91\x90a\x0E\xD2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x02`D\x82\x01R`\0`d\x82\x01R0`\x84\x82\x01R`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xB9W=`\0\x80>=`\0\xFD[PPPP`\0a\x06\xDAs}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07D\x91\x90a\x10\xB7V[\x95PPPPPPa\x07Zg\x10\xA7A\xA4bx\0\0\x90V[\x81\x10\x15a\x07zW`@Qc`\xF7\xF1\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07\xAC\x82a\x08\xF4V[\x81`\0\x81Q\x81\x10a\x07\xBFWa\x07\xBFa\x11\x01V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x12`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\toV[PPV[a\x08>`\x01`\x01`\xA0\x1B\x03\x83\x16s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x83a\toV[s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9`@QcW:\xDE\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x02`D\x83\x01R0`d\x83\x01R\x91\x90\x91\x16\x90cW:\xDE\x81\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD5\x91\x90a\x0E\xEFV[Pa\x08\x12\x82s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9a\t\xECV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\t\x0B\x91\x90a\x0E\xD2V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x91\x91\x90a\x0E\xD2V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\t\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x02\xDEV[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n`\x91\x90a\x0E\xEFV[\x11\x15a\x08\x12Wa\x08\x12`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\toV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBAWa\n\xBAa\n{V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\n\xD3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xEDWa\n\xEDa\n{V[a\x0B\0`\x1F\x82\x01`\x1F\x19\x16` \x01a\n\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\x15W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BGW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BfW`\0\x80\xFD[a\x0Br\x87\x83\x88\x01a\n\xC2V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0B\x88W`\0\x80\xFD[Pa\x0B\x95\x86\x82\x87\x01a\n\xC2V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0B\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC8W`\0\x80\xFD[a\x0B\xD4\x84\x82\x85\x01a\n\xC2V[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xF6Wa\x0B\xF6a\n{V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x15W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x0C)W`\0\x80\xFD[\x815` a\x0C>a\x0C9\x83a\x0B\xDCV[a\n\x91V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C]W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0CxW\x805\x83R\x91\x83\x01\x91\x83\x01a\x0CaV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x98W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xB0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0C\xC4W`\0\x80\xFD[\x815` a\x0C\xD4a\x0C9\x83a\x0B\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x0C\xF3W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\r\x1AW\x855a\r\x0B\x81a\x0C\0V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0C\xF8V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a\r0W`\0\x80\xFD[a\x0Br\x87\x83\x88\x01a\x0C\x18V[`\0\x80`@\x83\x85\x03\x12\x15a\rOW`\0\x80\xFD[\x825a\rZ\x81a\x0C\0V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\r\xA9W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\r\x84V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xCBW`\0\x80\xFD[\x845\x93P` \x85\x015a\r\xDD\x81a\x0C\0V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xFAW`\0\x80\xFD[a\x0E\x06\x88\x83\x89\x01a\n\xC2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\x1CW`\0\x80\xFD[Pa\x0E)\x87\x82\x88\x01a\n\xC2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0EHW`\0\x80\xFD[\x825a\x0ES\x81a\x0C\0V[\x91P` \x83\x015a\x0Ec\x81a\x0C\0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x99W`\0\x80\xFD[a\x0E\xA5\x86\x83\x87\x01a\n\xC2V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0E\xBBW`\0\x80\xFD[Pa\x0E\xC8\x85\x82\x86\x01a\n\xC2V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xE4W`\0\x80\xFD[\x81Qa\x02\x91\x81a\x0C\0V[`\0` \x82\x84\x03\x12\x15a\x0F\x01W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0F8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0F\x1CV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0FiW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0FMV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x0F\xD8W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x0F\xBAV[PP\x85\x81\x03`@\x87\x01Ra\x0F\xEC\x81\x8Ca\x0F\x08V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x10\x03\x81\x88a\x0F\x08V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x10&\x81\x86a\x0FCV[\x91PPa\x109`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x10f``\x83\x01\x84a\x0FCV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\x81W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\x91W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xA7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x91W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x10\xD0W`\0\x80\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xCA[\xE1\xDD\top\xD3\x06\xB1\xD3\xEA\xB2I]\x13\x16jp\xCB\xAD3\xC0\t\xF6\x89p-c B\x8DdsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static AAVEDEBTTOKENADAPTORV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x9A\x85\xE1\xD3\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\x8FW\x80c\xD7\x0C\x159\x14a\x01\xA2W\x80c\xE1p\xA9\xBF\x14a\x01\xB5W\x80c\xFAP\xE5\xD2\x14a\x01\xE0W`\0\x80\xFD[\x80c\x9A\x85\xE1\xD3\x14a\x01IW\x80c\xAE\xFF\xDD\xDE\x14a\x01\\W\x80c\xC9\x11\x1B\xD7\x14a\x01|W`\0\x80\xFD[\x80c>\x03*;\x14a\0\xD4W\x80ciD\\1\x14a\0\xE9W\x80cxASe\x14a\0\xFEW\x80cx\x8F\xB4\x84\x14a\x01\x1FW\x80cy\x98\xA1\xC4\x14a\x012W\x80c\x895:\t\x14a\x01:W[`\0\x80\xFD[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xFCa\0\xF76`\x04a\x0B2V[a\x01\xF6V[\0[a\x01\x11a\x01\x0C6`\x04a\x0B\x9FV[a\x02\x0FV[`@Q\x90\x81R` \x01a\0\xE0V[a\0\xFCa\x01-6`\x04a\x0C\x83V[a\x02\x98V[a\x01\x11a\x03\xACV[`@Q`\x01\x81R` \x01a\0\xE0V[a\0\xFCa\x01W6`\x04a\r<V[a\x04\nV[a\x01oa\x01j6`\x04a\x0B\x9FV[a\x07\x81V[`@Qa\0\xE0\x91\x90a\rhV[a\0\xFCa\x01\x8A6`\x04a\r\xB5V[a\x07\xE4V[a\0\xFCa\x01\x9D6`\x04a\x0E5V[a\x07\xFDV[a\0\xFCa\x01\xB06`\x04a\r<V[a\x08\x16V[a\x01\xC8a\x01\xC36`\x04a\x0B\x9FV[a\x08\xF4V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xE0V[a\x01\x11a\x01\xEE6`\x04a\x0EnV[`\0\x92\x91PPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x02&\x91\x90a\x0E\xD2V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x91\x91\x90a\x0E\xEFV[\x93\x92PPPV[\x81Q\x83Q\x14a\x02\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru$\xB78:\xBA\x1062\xB73\xBA4\x106\xB4\xB9\xB6\xB0\xBA1\xB4\x17`Q\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x03Wa\x03\x03a\n{V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03,W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@Qc\xAB\x9CK]`\xE0\x1B\x81R\x90\x91Ps}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x90c\xAB\x9CK]\x90a\x03t\x900\x90\x88\x90\x88\x90\x87\x90\x84\x90\x8A\x90`\0\x90`\x04\x01a\x0F\x89V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03\xA2W=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0`@Q` \x01a\x03\xEF\x90` \x80\x82R`\x1C\x90\x82\x01R\x7FAave debtToken Adaptor V 1.0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0a\x04\x14a\x03\xACV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x16` \x82\x01R`\x01\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04J\x93\x92\x91` \x01a\x10EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xC6\x91\x90a\x0E\xD2V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF3\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x054\x91\x90a\x10oV[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x9C\x91\x90a\x10\x95V[a\x05\xC4W`@Qc;\xA3\x97a`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x02\xDEV[s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9`\x01`\x01`\xA0\x1B\x03\x16c\xA4\x15\xBC\xAD\x85`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06I\x91\x90a\x0E\xD2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`\x02`D\x82\x01R`\0`d\x82\x01R0`\x84\x82\x01R`\xA4\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xB9W=`\0\x80>=`\0\xFD[PPPP`\0a\x06\xDAs}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07 W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07D\x91\x90a\x10\xB7V[\x95PPPPPPa\x07Zg\x10\xA7A\xA4bx\0\0\x90V[\x81\x10\x15a\x07zW`@Qc`\xF7\xF1\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07\xAC\x82a\x08\xF4V[\x81`\0\x81Q\x81\x10a\x07\xBFWa\x07\xBFa\x11\x01V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\x12`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\toV[PPV[a\x08>`\x01`\x01`\xA0\x1B\x03\x83\x16s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9\x83a\toV[s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9`@QcW:\xDE\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x02`D\x83\x01R0`d\x83\x01R\x91\x90\x91\x16\x90cW:\xDE\x81\x90`\x84\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD5\x91\x90a\x0E\xEFV[Pa\x08\x12\x82s}'h\xDE2\xB0\xB8\x0Bz4T\xC0k\xDA\xC9Ji\xDD\xC7\xA9a\t\xECV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\t\x0B\x91\x90a\x0E\xD2V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tKW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x91\x91\x90a\x0E\xD2V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\t\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x02\xDEV[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n`\x91\x90a\x0E\xEFV[\x11\x15a\x08\x12Wa\x08\x12`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\toV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBAWa\n\xBAa\n{V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a\n\xD3W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\xEDWa\n\xEDa\n{V[a\x0B\0`\x1F\x82\x01`\x1F\x19\x16` \x01a\n\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\x15W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0BGW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BfW`\0\x80\xFD[a\x0Br\x87\x83\x88\x01a\n\xC2V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0B\x88W`\0\x80\xFD[Pa\x0B\x95\x86\x82\x87\x01a\n\xC2V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0B\xB1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xC8W`\0\x80\xFD[a\x0B\xD4\x84\x82\x85\x01a\n\xC2V[\x94\x93PPPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0B\xF6Wa\x0B\xF6a\n{V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\x15W`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x0C)W`\0\x80\xFD[\x815` a\x0C>a\x0C9\x83a\x0B\xDCV[a\n\x91V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C]W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0CxW\x805\x83R\x91\x83\x01\x91\x83\x01a\x0CaV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x98W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xB0W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x0C\xC4W`\0\x80\xFD[\x815` a\x0C\xD4a\x0C9\x83a\x0B\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x0C\xF3W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\r\x1AW\x855a\r\x0B\x81a\x0C\0V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0C\xF8V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a\r0W`\0\x80\xFD[a\x0Br\x87\x83\x88\x01a\x0C\x18V[`\0\x80`@\x83\x85\x03\x12\x15a\rOW`\0\x80\xFD[\x825a\rZ\x81a\x0C\0V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\r\xA9W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\r\x84V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xCBW`\0\x80\xFD[\x845\x93P` \x85\x015a\r\xDD\x81a\x0C\0V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xFAW`\0\x80\xFD[a\x0E\x06\x88\x83\x89\x01a\n\xC2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\x1CW`\0\x80\xFD[Pa\x0E)\x87\x82\x88\x01a\n\xC2V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0EHW`\0\x80\xFD[\x825a\x0ES\x81a\x0C\0V[\x91P` \x83\x015a\x0Ec\x81a\x0C\0V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\x81W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x99W`\0\x80\xFD[a\x0E\xA5\x86\x83\x87\x01a\n\xC2V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0E\xBBW`\0\x80\xFD[Pa\x0E\xC8\x85\x82\x86\x01a\n\xC2V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xE4W`\0\x80\xFD[\x81Qa\x02\x91\x81a\x0C\0V[`\0` \x82\x84\x03\x12\x15a\x0F\x01W`\0\x80\xFD[PQ\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a\x0F8W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0F\x1CV[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0FiW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0FMV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x82R`\xE0` \x80\x84\x01\x82\x90R\x89Q\x91\x84\x01\x82\x90R`\0\x92\x8A\x82\x01\x92\x90\x91\x90a\x01\0\x86\x01\x90\x85[\x81\x81\x10\x15a\x0F\xD8W\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x0F\xBAV[PP\x85\x81\x03`@\x87\x01Ra\x0F\xEC\x81\x8Ca\x0F\x08V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x10\x03\x81\x88a\x0F\x08V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R\x90P\x82\x81\x03`\xA0\x84\x01Ra\x10&\x81\x86a\x0FCV[\x91PPa\x109`\xC0\x83\x01\x84a\xFF\xFF\x16\x90RV[\x98\x97PPPPPPPPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a\x10f``\x83\x01\x84a\x0FCV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x10\x81W`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02\x91W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xA7W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x02\x91W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x10\xD0W`\0\x80\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xCA[\xE1\xDD\top\xD3\x06\xB1\xD3\xEA\xB2I]\x13\x16jp\xCB\xAD3\xC0\t\xF6\x89p-c B\x8DdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEDEBTTOKENADAPTORV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveDebtTokenAdaptorV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveDebtTokenAdaptorV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveDebtTokenAdaptorV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveDebtTokenAdaptorV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveDebtTokenAdaptorV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveDebtTokenAdaptorV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveDebtTokenAdaptorV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEDEBTTOKENADAPTORV2_ABI.clone(),
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
                AAVEDEBTTOKENADAPTORV2_ABI.clone(),
                AAVEDEBTTOKENADAPTORV2_BYTECODE.clone().into(),
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
        ///Calls the contract's `borrowFromAave` (0x9a85e1d3) function
        pub fn borrow_from_aave(
            &self,
            debt_token_to_borrow: ::ethers::core::types::Address,
            amount_to_borrow: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [154, 133, 225, 211],
                    (debt_token_to_borrow, amount_to_borrow),
                )
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
        ///Calls the contract's `flashLoan` (0x788fb484) function
        pub fn flash_loan(
            &self,
            loan_token: ::std::vec::Vec<::ethers::core::types::Address>,
            loan_amount: ::std::vec::Vec<::ethers::core::types::U256>,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 143, 180, 132], (loan_token, loan_amount, params))
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
        ///Calls the contract's `repayAaveDebt` (0xd70c1539) function
        pub fn repay_aave_debt(
            &self,
            token_to_repay: ::ethers::core::types::Address,
            amount_to_repay: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 12, 21, 57], (token_to_repay, amount_to_repay))
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
    for AaveDebtTokenAdaptorV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AaveDebtTokenAdaptor__DebtPositionsMustBeTracked` with signature `AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(address)` and selector `0x77472ec2`
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
        name = "AaveDebtTokenAdaptor__DebtPositionsMustBeTracked",
        abi = "AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(address)"
    )]
    pub struct AaveDebtTokenAdaptor__DebtPositionsMustBeTracked {
        pub untracked_debt_position: ::ethers::core::types::Address,
    }
    ///Custom Error type `AaveDebtTokenAdaptor__HealthFactorTooLow` with signature `AaveDebtTokenAdaptor__HealthFactorTooLow()` and selector `0xc1efe3fa`
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
        name = "AaveDebtTokenAdaptor__HealthFactorTooLow",
        abi = "AaveDebtTokenAdaptor__HealthFactorTooLow()"
    )]
    pub struct AaveDebtTokenAdaptor__HealthFactorTooLow;
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
    pub enum AaveDebtTokenAdaptorV2Errors {
        AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(
            AaveDebtTokenAdaptor__DebtPositionsMustBeTracked,
        ),
        AaveDebtTokenAdaptor__HealthFactorTooLow(
            AaveDebtTokenAdaptor__HealthFactorTooLow,
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
    impl ::ethers::core::abi::AbiDecode for AaveDebtTokenAdaptorV2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AaveDebtTokenAdaptor__DebtPositionsMustBeTracked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(decoded),
                );
            }
            if let Ok(decoded) = <AaveDebtTokenAdaptor__HealthFactorTooLow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AaveDebtTokenAdaptor__HealthFactorTooLow(decoded));
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
    impl ::ethers::core::abi::AbiEncode for AaveDebtTokenAdaptorV2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AaveDebtTokenAdaptor__HealthFactorTooLow(element) => {
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
    impl ::ethers::contract::ContractRevert for AaveDebtTokenAdaptorV2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AaveDebtTokenAdaptor__DebtPositionsMustBeTracked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AaveDebtTokenAdaptor__HealthFactorTooLow as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for AaveDebtTokenAdaptorV2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AaveDebtTokenAdaptor__HealthFactorTooLow(element) => {
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
    impl ::core::convert::From<::std::string::String> for AaveDebtTokenAdaptorV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AaveDebtTokenAdaptor__DebtPositionsMustBeTracked>
    for AaveDebtTokenAdaptorV2Errors {
        fn from(value: AaveDebtTokenAdaptor__DebtPositionsMustBeTracked) -> Self {
            Self::AaveDebtTokenAdaptor__DebtPositionsMustBeTracked(value)
        }
    }
    impl ::core::convert::From<AaveDebtTokenAdaptor__HealthFactorTooLow>
    for AaveDebtTokenAdaptorV2Errors {
        fn from(value: AaveDebtTokenAdaptor__HealthFactorTooLow) -> Self {
            Self::AaveDebtTokenAdaptor__HealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for AaveDebtTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for AaveDebtTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage> for AaveDebtTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for AaveDebtTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for AaveDebtTokenAdaptorV2Errors {
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
    ///Container type for all input parameters for the `borrowFromAave` function with signature `borrowFromAave(address,uint256)` and selector `0x9a85e1d3`
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
    #[ethcall(name = "borrowFromAave", abi = "borrowFromAave(address,uint256)")]
    pub struct BorrowFromAaveCall {
        pub debt_token_to_borrow: ::ethers::core::types::Address,
        pub amount_to_borrow: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `flashLoan` function with signature `flashLoan(address[],uint256[],bytes)` and selector `0x788fb484`
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
    #[ethcall(name = "flashLoan", abi = "flashLoan(address[],uint256[],bytes)")]
    pub struct FlashLoanCall {
        pub loan_token: ::std::vec::Vec<::ethers::core::types::Address>,
        pub loan_amount: ::std::vec::Vec<::ethers::core::types::U256>,
        pub params: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `repayAaveDebt` function with signature `repayAaveDebt(address,uint256)` and selector `0xd70c1539`
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
    #[ethcall(name = "repayAaveDebt", abi = "repayAaveDebt(address,uint256)")]
    pub struct RepayAaveDebtCall {
        pub token_to_repay: ::ethers::core::types::Address,
        pub amount_to_repay: ::ethers::core::types::U256,
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
    pub enum AaveDebtTokenAdaptorV2Calls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BorrowFromAave(BorrowFromAaveCall),
        Deposit(DepositCall),
        FlashLoan(FlashLoanCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        RepayAaveDebt(RepayAaveDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for AaveDebtTokenAdaptorV2Calls {
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
            if let Ok(decoded) = <BorrowFromAaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BorrowFromAave(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <FlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FlashLoan(decoded));
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
            if let Ok(decoded) = <RepayAaveDebtCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RepayAaveDebt(decoded));
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
    impl ::ethers::core::abi::AbiEncode for AaveDebtTokenAdaptorV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BorrowFromAave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RepayAaveDebt(element) => {
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
    impl ::core::fmt::Display for AaveDebtTokenAdaptorV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BorrowFromAave(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RepayAaveDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetOfCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BorrowFromAaveCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: BorrowFromAaveCall) -> Self {
            Self::BorrowFromAave(value)
        }
    }
    impl ::core::convert::From<DepositCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FlashLoanCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: FlashLoanCall) -> Self {
            Self::FlashLoan(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<RepayAaveDebtCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: RepayAaveDebtCall) -> Self {
            Self::RepayAaveDebt(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for AaveDebtTokenAdaptorV2Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for AaveDebtTokenAdaptorV2Calls {
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
