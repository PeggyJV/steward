pub use compound_c_token_adaptor_v2::*;
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
pub mod compound_c_token_adaptor_v2 {
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
                    ::std::borrow::ToOwned::to_owned("claimComp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimComp"),
                            inputs: ::std::vec![],
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("depositToCompound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositToCompound"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("market"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CErc20"),
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
                                    name: ::std::string::String::new(),
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
                    ::std::borrow::ToOwned::to_owned("withdrawFromCompound"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "withdrawFromCompound",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("market"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract CErc20"),
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
                    ::std::borrow::ToOwned::to_owned("CTokenAdaptor__MarketNotListed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CTokenAdaptor__MarketNotListed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("market"),
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
                        "CTokenAdaptor__NonZeroCompoundErrorCode",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CTokenAdaptor__NonZeroCompoundErrorCode",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("errorCode"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static COMPOUNDCTOKENADAPTORV2_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x121\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\xAD\x8D\xA9k\x11a\0\x8CW\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x01\x84W\x80c\xD3\xBF\xE7j\x14a\x01\x97W\x80c\xE1p\xA9\xBF\x14a\x01\xAAW\x80c\xFAP\xE5\xD2\x14a\x01\xD5W`\0\x80\xFD[\x80c\xAD\x8D\xA9k\x14a\x01>W\x80c\xAE\xFF\xDD\xDE\x14a\x01QW\x80c\xC8\x8D\xA5\xF3\x14a\x01qW`\0\x80\xFD[\x80c\x1B\xD8[\xDB\x14a\0\xD4W\x80c>\x03*;\x14a\0\xDEW\x80ciD\\1\x14a\0\xF3W\x80cxASe\x14a\x01\x06W\x80cy\x98\xA1\xC4\x14a\x01'W\x80c\x895:\t\x14a\x01/W[`\0\x80\xFD[a\0\xDCa\x01\xE8V[\0[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDCa\x01\x016`\x04a\x0E\xF6V[a\x02ZV[a\x01\x19a\x01\x146`\x04a\x0FcV[a\x03\x9EV[`@Q\x90\x81R` \x01a\0\xEAV[a\x01\x19a\x04\xA2V[`@Q`\0\x81R` \x01a\0\xEAV[a\0\xDCa\x01L6`\x04a\x0F\xADV[a\x05\0V[a\x01da\x01_6`\x04a\x0FcV[a\x06+V[`@Qa\0\xEA\x91\x90a\x0F\xD9V[a\0\xDCa\x01\x7F6`\x04a\x0F\xADV[a\x06\xCCV[a\0\xDCa\x01\x926`\x04a\x10&V[a\x08MV[a\0\xDCa\x01\xA56`\x04a\x10\xA6V[a\t\x80V[a\x01\xBDa\x01\xB86`\x04a\x0FcV[a\t\x99V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEAV[a\x01\x19a\x01\xE36`\x04a\x10\xDFV[a\n\x1BV[s=\x98\x19!\n1\xB4\x96\x1B0\xEFT\xBE*\xEDy\xB9\xC9\xCD;`@Qct\xD7\x81I`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE9\xAF\x02\x92\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02TW=`\0\x80>=`\0\xFD[PPPPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02p\x91\x90a\x11CV[\x90Pa\x02{\x81a\n\xEEV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x11CV[\x90Pa\x02\xF5`\x01`\x01`\xA0\x1B\x03\x82\x16\x83\x87a\x0B\x9EV[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03c\x91\x90a\x11`V[\x90P\x80\x15a\x03\x8CW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x96\x82\x84a\x0C\x15V[PPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03\xB5\x91\x90a\x11CV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04#\x91\x90a\x11`V[\x90Pa\x04\x9A\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18-\xF0\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8A\x91\x90a\x11`V[\x82\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xA4V[\x94\x93PPPPV[`\0`@Q` \x01a\x04\xE5\x90` \x80\x82R`\x1D\x90\x82\x01R\x7FCompound cToken Adaptor V 1.0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x05\t\x82a\n\xEEV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05m\x91\x90a\x11CV[\x90Pa\x05y\x81\x83a\x0C\xC3V[\x91Pa\x05\x8F`\x01`\x01`\xA0\x1B\x03\x82\x16\x84\x84a\x0B\x9EV[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFD\x91\x90a\x11`V[\x90P\x80\x15a\x06!W`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[a\x02T\x82\x85a\x0C\x15V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x06T\x82a\t\x99V[\x81`\0\x81Q\x81\x10a\x06gWa\x06ga\x11yV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Rs\xC0\x0E\x94\xCBf,5 (.oW\x17!@\x04\xA7\xF2h\x88\x81`\x01\x81Q\x81\x10a\x06\xA7Wa\x06\xA7a\x11yV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x06\xD5\x82a\n\xEEV[`\0`\0\x19\x82\x03a\x07\xB8W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xDB\0ju\x90\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07P\x91\x90a\x11`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xB1\x91\x90a\x11`V[\x90Pa\x08&V[`@Qc\x85*\x12\xE3`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x85*\x12\xE3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08#\x91\x90a\x11`V[\x90P[\x80\x15a\x08HW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[PPPV[a\x08V\x83a\rDV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x08l\x91\x90a\x11CV[\x90Pa\x08w\x81a\n\xEEV[`@Qc\x85*\x12\xE3`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x85*\x12\xE3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE5\x91\x90a\x11`V[\x90P\x80\x15a\t\tW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[a\x03\x96\x85\x87\x84`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tp\x91\x90a\x11CV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\r\xDBV[a\t\x95`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0B\x9EV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\t\xB0\x91\x90a\x11CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x14\x91\x90a\x11CV[\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\n2\x91\x90a\x11CV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA0\x91\x90a\x11`V[\x90Pa\n\xE3\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18-\xF0\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04fW=`\0\x80>=`\0\xFD[\x92PPP[\x92\x91PPV[`\0s=\x98\x19!\n1\xB4\x96\x1B0\xEFT\xBE*\xEDy\xB9\xC9\xCD;`@Qc\x8E\x8F)K`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\x8E\x8F)K\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BMW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bq\x91\x90a\x11\xA4V[PP\x90P\x80a\t\x95W`@Qc\x0C\xEE\xDC\x9D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\x83V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x03\x83V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90a\x11`V[\x11\x15a\t\x95Wa\t\x95`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0B\x9EV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\xBCW`\0\x80\xFD[\x04\x92\x91PPV[`\0`\0\x19\x82\x03a\r=W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r6\x91\x90a\x11`V[\x90Pa\n\xE8V[P\x80a\n\xE8V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\r\xBAWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xBA\x91\x90a\x11\xE0V[\x15a\r\xD8W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x03\x83V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0EzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x95Wa\x0E\x95a\x0ESV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xBDWa\x0E\xBDa\x0ESV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xD6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x0BW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F*W`\0\x80\xFD[a\x0F6\x87\x83\x88\x01a\x0EiV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0FLW`\0\x80\xFD[Pa\x0FY\x86\x82\x87\x01a\x0EiV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0FuW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x8CW`\0\x80\xFD[a\x04\x9A\x84\x82\x85\x01a\x0EiV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xD8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xC0W`\0\x80\xFD[\x825a\x0F\xCB\x81a\x0F\x98V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10\x1AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\xF5V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10<W`\0\x80\xFD[\x845\x93P` \x85\x015a\x10N\x81a\x0F\x98V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10kW`\0\x80\xFD[a\x10w\x88\x83\x89\x01a\x0EiV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x10\x8DW`\0\x80\xFD[Pa\x10\x9A\x87\x82\x88\x01a\x0EiV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xB9W`\0\x80\xFD[\x825a\x10\xC4\x81a\x0F\x98V[\x91P` \x83\x015a\x10\xD4\x81a\x0F\x98V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xF2W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\nW`\0\x80\xFD[a\x11\x16\x86\x83\x87\x01a\x0EiV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x11,W`\0\x80\xFD[Pa\x119\x85\x82\x86\x01a\x0EiV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11UW`\0\x80\xFD[\x81Qa\n\x14\x81a\x0F\x98V[`\0` \x82\x84\x03\x12\x15a\x11rW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q\x80\x15\x15\x81\x14a\x11\x9FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xB9W`\0\x80\xFD[a\x11\xC2\x84a\x11\x8FV[\x92P` \x84\x01Q\x91Pa\x11\xD7`@\x85\x01a\x11\x8FV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x11\xF2W`\0\x80\xFD[a\n\x14\x82a\x11\x8FV\xFE\xA2dipfsX\"\x12 i\xA9\xB3\xEAW\xD9\x8A\xD1\xEB&\xF7\xA9v\x84h\xF1#\xCE\xF4\xA5<\xDC\xCF\xEE)\xE6\xBF8TG\xFBKdsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static COMPOUNDCTOKENADAPTORV2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\xAD\x8D\xA9k\x11a\0\x8CW\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x01\x84W\x80c\xD3\xBF\xE7j\x14a\x01\x97W\x80c\xE1p\xA9\xBF\x14a\x01\xAAW\x80c\xFAP\xE5\xD2\x14a\x01\xD5W`\0\x80\xFD[\x80c\xAD\x8D\xA9k\x14a\x01>W\x80c\xAE\xFF\xDD\xDE\x14a\x01QW\x80c\xC8\x8D\xA5\xF3\x14a\x01qW`\0\x80\xFD[\x80c\x1B\xD8[\xDB\x14a\0\xD4W\x80c>\x03*;\x14a\0\xDEW\x80ciD\\1\x14a\0\xF3W\x80cxASe\x14a\x01\x06W\x80cy\x98\xA1\xC4\x14a\x01'W\x80c\x895:\t\x14a\x01/W[`\0\x80\xFD[a\0\xDCa\x01\xE8V[\0[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDCa\x01\x016`\x04a\x0E\xF6V[a\x02ZV[a\x01\x19a\x01\x146`\x04a\x0FcV[a\x03\x9EV[`@Q\x90\x81R` \x01a\0\xEAV[a\x01\x19a\x04\xA2V[`@Q`\0\x81R` \x01a\0\xEAV[a\0\xDCa\x01L6`\x04a\x0F\xADV[a\x05\0V[a\x01da\x01_6`\x04a\x0FcV[a\x06+V[`@Qa\0\xEA\x91\x90a\x0F\xD9V[a\0\xDCa\x01\x7F6`\x04a\x0F\xADV[a\x06\xCCV[a\0\xDCa\x01\x926`\x04a\x10&V[a\x08MV[a\0\xDCa\x01\xA56`\x04a\x10\xA6V[a\t\x80V[a\x01\xBDa\x01\xB86`\x04a\x0FcV[a\t\x99V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEAV[a\x01\x19a\x01\xE36`\x04a\x10\xDFV[a\n\x1BV[s=\x98\x19!\n1\xB4\x96\x1B0\xEFT\xBE*\xEDy\xB9\xC9\xCD;`@Qct\xD7\x81I`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xE9\xAF\x02\x92\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02@W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02TW=`\0\x80>=`\0\xFD[PPPPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02p\x91\x90a\x11CV[\x90Pa\x02{\x81a\n\xEEV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x11CV[\x90Pa\x02\xF5`\x01`\x01`\xA0\x1B\x03\x82\x16\x83\x87a\x0B\x9EV[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03?W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03c\x91\x90a\x11`V[\x90P\x80\x15a\x03\x8CW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x96\x82\x84a\x0C\x15V[PPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03\xB5\x91\x90a\x11CV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04#\x91\x90a\x11`V[\x90Pa\x04\x9A\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18-\xF0\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04fW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x8A\x91\x90a\x11`V[\x82\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xA4V[\x94\x93PPPPV[`\0`@Q` \x01a\x04\xE5\x90` \x80\x82R`\x1D\x90\x82\x01R\x7FCompound cToken Adaptor V 1.0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x05\t\x82a\n\xEEV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05m\x91\x90a\x11CV[\x90Pa\x05y\x81\x83a\x0C\xC3V[\x91Pa\x05\x8F`\x01`\x01`\xA0\x1B\x03\x82\x16\x84\x84a\x0B\x9EV[`@Qc\x14\x0E%\xAD`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xA0q-h\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x05\xD9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xFD\x91\x90a\x11`V[\x90P\x80\x15a\x06!W`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[a\x02T\x82\x85a\x0C\x15V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x06T\x82a\t\x99V[\x81`\0\x81Q\x81\x10a\x06gWa\x06ga\x11yV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Rs\xC0\x0E\x94\xCBf,5 (.oW\x17!@\x04\xA7\xF2h\x88\x81`\x01\x81Q\x81\x10a\x06\xA7Wa\x06\xA7a\x11yV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x06\xD5\x82a\n\xEEV[`\0`\0\x19\x82\x03a\x07\xB8W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xDB\0ju\x90\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07P\x91\x90a\x11`V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07n\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xB1\x91\x90a\x11`V[\x90Pa\x08&V[`@Qc\x85*\x12\xE3`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x85*\x12\xE3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\xFFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08#\x91\x90a\x11`V[\x90P[\x80\x15a\x08HW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[PPPV[a\x08V\x83a\rDV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x08l\x91\x90a\x11CV[\x90Pa\x08w\x81a\n\xEEV[`@Qc\x85*\x12\xE3`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x85*\x12\xE3\x90`$\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x08\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xE5\x91\x90a\x11`V[\x90P\x80\x15a\t\tW`@Qck\xB7\x83e`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x83V[a\x03\x96\x85\x87\x84`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tLW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tp\x91\x90a\x11CV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\r\xDBV[a\t\x95`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0B\x9EV[PPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\t\xB0\x91\x90a\x11CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16co0}\xC3`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x14\x91\x90a\x11CV[\x93\x92PPPV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\n2\x91\x90a\x11CV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n|W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xA0\x91\x90a\x11`V[\x90Pa\n\xE3\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18-\xF0\xF5`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04fW=`\0\x80>=`\0\xFD[\x92PPP[\x92\x91PPV[`\0s=\x98\x19!\n1\xB4\x96\x1B0\xEFT\xBE*\xEDy\xB9\xC9\xCD;`@Qc\x8E\x8F)K`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90c\x8E\x8F)K\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BMW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bq\x91\x90a\x11\xA4V[PP\x90P\x80a\t\x95W`@Qc\x0C\xEE\xDC\x9D`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\x83V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x03\x83V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CeW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x89\x91\x90a\x11`V[\x11\x15a\t\x95Wa\t\x95`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0B\x9EV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\xBCW`\0\x80\xFD[\x04\x92\x91PPV[`\0`\0\x19\x82\x03a\r=W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r6\x91\x90a\x11`V[\x90Pa\n\xE8V[P\x80a\n\xE8V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\r\xBAWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xBA\x91\x90a\x11\xE0V[\x15a\r\xD8W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x02TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x03\x83V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0EzW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x95Wa\x0E\x95a\x0ESV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0E\xBDWa\x0E\xBDa\x0ESV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0E\xD6W`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0F\x0BW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F*W`\0\x80\xFD[a\x0F6\x87\x83\x88\x01a\x0EiV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x0FLW`\0\x80\xFD[Pa\x0FY\x86\x82\x87\x01a\x0EiV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0FuW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x8CW`\0\x80\xFD[a\x04\x9A\x84\x82\x85\x01a\x0EiV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\xD8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\xC0W`\0\x80\xFD[\x825a\x0F\xCB\x81a\x0F\x98V[\x94` \x93\x90\x93\x015\x93PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10\x1AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\xF5V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10<W`\0\x80\xFD[\x845\x93P` \x85\x015a\x10N\x81a\x0F\x98V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10kW`\0\x80\xFD[a\x10w\x88\x83\x89\x01a\x0EiV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x10\x8DW`\0\x80\xFD[Pa\x10\x9A\x87\x82\x88\x01a\x0EiV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xB9W`\0\x80\xFD[\x825a\x10\xC4\x81a\x0F\x98V[\x91P` \x83\x015a\x10\xD4\x81a\x0F\x98V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xF2W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\nW`\0\x80\xFD[a\x11\x16\x86\x83\x87\x01a\x0EiV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x11,W`\0\x80\xFD[Pa\x119\x85\x82\x86\x01a\x0EiV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x11UW`\0\x80\xFD[\x81Qa\n\x14\x81a\x0F\x98V[`\0` \x82\x84\x03\x12\x15a\x11rW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80Q\x80\x15\x15\x81\x14a\x11\x9FW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xB9W`\0\x80\xFD[a\x11\xC2\x84a\x11\x8FV[\x92P` \x84\x01Q\x91Pa\x11\xD7`@\x85\x01a\x11\x8FV[\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x11\xF2W`\0\x80\xFD[a\n\x14\x82a\x11\x8FV\xFE\xA2dipfsX\"\x12 i\xA9\xB3\xEAW\xD9\x8A\xD1\xEB&\xF7\xA9v\x84h\xF1#\xCE\xF4\xA5<\xDC\xCF\xEE)\xE6\xBF8TG\xFBKdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static COMPOUNDCTOKENADAPTORV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CompoundCTokenAdaptorV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CompoundCTokenAdaptorV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CompoundCTokenAdaptorV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CompoundCTokenAdaptorV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CompoundCTokenAdaptorV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CompoundCTokenAdaptorV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CompoundCTokenAdaptorV2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COMPOUNDCTOKENADAPTORV2_ABI.clone(),
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
                COMPOUNDCTOKENADAPTORV2_ABI.clone(),
                COMPOUNDCTOKENADAPTORV2_BYTECODE.clone().into(),
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
        ///Calls the contract's `claimComp` (0x1bd85bdb) function
        pub fn claim_comp(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 216, 91, 219], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x69445c31) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            adaptor_data: ::ethers::core::types::Bytes,
            p2: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 68, 92, 49], (assets, adaptor_data, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositToCompound` (0xad8da96b) function
        pub fn deposit_to_compound(
            &self,
            market: ::ethers::core::types::Address,
            amount_to_deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 141, 169, 107], (market, amount_to_deposit))
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
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 17, 27, 215], (assets, receiver, adaptor_data, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFromCompound` (0xc88da5f3) function
        pub fn withdraw_from_compound(
            &self,
            market: ::ethers::core::types::Address,
            amount_to_withdraw: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 141, 165, 243], (market, amount_to_withdraw))
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
    for CompoundCTokenAdaptorV2<M> {
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
    ///Custom Error type `CTokenAdaptor__MarketNotListed` with signature `CTokenAdaptor__MarketNotListed(address)` and selector `0x0ceedc9d`
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
        name = "CTokenAdaptor__MarketNotListed",
        abi = "CTokenAdaptor__MarketNotListed(address)"
    )]
    pub struct CTokenAdaptor__MarketNotListed {
        pub market: ::ethers::core::types::Address,
    }
    ///Custom Error type `CTokenAdaptor__NonZeroCompoundErrorCode` with signature `CTokenAdaptor__NonZeroCompoundErrorCode(uint256)` and selector `0xd76f06ca`
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
        name = "CTokenAdaptor__NonZeroCompoundErrorCode",
        abi = "CTokenAdaptor__NonZeroCompoundErrorCode(uint256)"
    )]
    pub struct CTokenAdaptor__NonZeroCompoundErrorCode {
        pub error_code: ::ethers::core::types::U256,
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
    pub enum CompoundCTokenAdaptorV2Errors {
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        CTokenAdaptor__MarketNotListed(CTokenAdaptor__MarketNotListed),
        CTokenAdaptor__NonZeroCompoundErrorCode(CTokenAdaptor__NonZeroCompoundErrorCode),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CompoundCTokenAdaptorV2Errors {
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
            if let Ok(decoded) = <CTokenAdaptor__MarketNotListed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CTokenAdaptor__MarketNotListed(decoded));
            }
            if let Ok(decoded) = <CTokenAdaptor__NonZeroCompoundErrorCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CTokenAdaptor__NonZeroCompoundErrorCode(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CompoundCTokenAdaptorV2Errors {
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
                Self::CTokenAdaptor__MarketNotListed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CTokenAdaptor__NonZeroCompoundErrorCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CompoundCTokenAdaptorV2Errors {
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
                _ if selector
                    == <CTokenAdaptor__MarketNotListed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CTokenAdaptor__NonZeroCompoundErrorCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CompoundCTokenAdaptorV2Errors {
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
                Self::CTokenAdaptor__MarketNotListed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CTokenAdaptor__NonZeroCompoundErrorCode(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CompoundCTokenAdaptorV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage> for CompoundCTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    impl ::core::convert::From<CTokenAdaptor__MarketNotListed>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: CTokenAdaptor__MarketNotListed) -> Self {
            Self::CTokenAdaptor__MarketNotListed(value)
        }
    }
    impl ::core::convert::From<CTokenAdaptor__NonZeroCompoundErrorCode>
    for CompoundCTokenAdaptorV2Errors {
        fn from(value: CTokenAdaptor__NonZeroCompoundErrorCode) -> Self {
            Self::CTokenAdaptor__NonZeroCompoundErrorCode(value)
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
    ///Container type for all input parameters for the `claimComp` function with signature `claimComp()` and selector `0x1bd85bdb`
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
    #[ethcall(name = "claimComp", abi = "claimComp()")]
    pub struct ClaimCompCall;
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
        pub p2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `depositToCompound` function with signature `depositToCompound(address,uint256)` and selector `0xad8da96b`
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
    #[ethcall(name = "depositToCompound", abi = "depositToCompound(address,uint256)")]
    pub struct DepositToCompoundCall {
        pub market: ::ethers::core::types::Address,
        pub amount_to_deposit: ::ethers::core::types::U256,
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
        pub p3: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdrawFromCompound` function with signature `withdrawFromCompound(address,uint256)` and selector `0xc88da5f3`
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
        name = "withdrawFromCompound",
        abi = "withdrawFromCompound(address,uint256)"
    )]
    pub struct WithdrawFromCompoundCall {
        pub market: ::ethers::core::types::Address,
        pub amount_to_withdraw: ::ethers::core::types::U256,
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
    pub enum CompoundCTokenAdaptorV2Calls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        ClaimComp(ClaimCompCall),
        Deposit(DepositCall),
        DepositToCompound(DepositToCompoundCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawFromCompound(WithdrawFromCompoundCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for CompoundCTokenAdaptorV2Calls {
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
            if let Ok(decoded) = <ClaimCompCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimComp(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositToCompoundCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositToCompound(decoded));
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
            if let Ok(decoded) = <WithdrawFromCompoundCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawFromCompound(decoded));
            }
            if let Ok(decoded) = <WithdrawableFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CompoundCTokenAdaptorV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimComp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositToCompound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawFromCompound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CompoundCTokenAdaptorV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimComp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositToCompound(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFromCompound(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetOfCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ClaimCompCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: ClaimCompCall) -> Self {
            Self::ClaimComp(value)
        }
    }
    impl ::core::convert::From<DepositCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositToCompoundCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: DepositToCompoundCall) -> Self {
            Self::DepositToCompound(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CompoundCTokenAdaptorV2Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFromCompoundCall>
    for CompoundCTokenAdaptorV2Calls {
        fn from(value: WithdrawFromCompoundCall) -> Self {
            Self::WithdrawFromCompound(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for CompoundCTokenAdaptorV2Calls {
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
