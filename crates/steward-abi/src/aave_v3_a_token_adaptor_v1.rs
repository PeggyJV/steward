pub use aave_v3a_token_adaptor_v1::*;
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
pub mod aave_v3a_token_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/AaveV3ATokenAdaptorV1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "adjustIsolationModeAssetAsCollateral",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "adjustIsolationModeAssetAsCollateral",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("useAsCollateral"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("changeEMode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("changeEMode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("categoryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
                    ::std::borrow::ToOwned::to_owned("depositToAave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositToAave"),
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
                                    name: ::std::borrow::ToOwned::to_owned("configData"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawFromAave"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawFromAave"),
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
                                    name: ::std::borrow::ToOwned::to_owned("configData"),
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
                        "AaveV3ATokenAdaptor__HealthFactorTooLow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AaveV3ATokenAdaptor__HealthFactorTooLow",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "AaveV3ATokenAdaptor__OracleUsesDifferentBase",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AaveV3ATokenAdaptor__OracleUsesDifferentBase",
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
    pub static AAVEV3ATOKENADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x1A\xF3\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\xBDW\x80c\xE1p\xA9\xBF\x14a\x01\xD0W\x80c\xEE\xB1I\xE7\x14a\x01\xFBW\x80c\xFAP\xE5\xD2\x14a\x02\x0EW`\0\x80\xFD[\x80c\x895:\t\x14a\x01{W\x80c\xAE\xFF\xDD\xDE\x14a\x01\x8AW\x80c\xC9\x11\x1B\xD7\x14a\x01\xAAW`\0\x80\xFD[\x80cJ\xC89\xF7\x11a\0\xC8W\x80cJ\xC89\xF7\x14a\x01,W\x80ciD\\1\x14a\x01?W\x80cxASe\x14a\x01RW\x80cy\x98\xA1\xC4\x14a\x01sW`\0\x80\xFD[\x80c#\xBF)\x8F\x14a\0\xEFW\x80c>\x03*;\x14a\x01\x04W\x80cH~\xDE\x04\x14a\x01\x19W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\x15BV[a\x02!V[\0[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01'6`\x04a\x15{V[a\x03QV[a\x01\x02a\x01:6`\x04a\x15\xB6V[a\x03\xF2V[a\x01\x02a\x01M6`\x04a\x16vV[a\x05\x1AV[a\x01ea\x01`6`\x04a\x16\xE3V[a\x06_V[`@Q\x90\x81R` \x01a\x01\x10V[a\x01ea\x06\xE8V[`@Q`\0\x81R` \x01a\x01\x10V[a\x01\x9Da\x01\x986`\x04a\x16\xE3V[a\x07FV[`@Qa\x01\x10\x91\x90a\x17 V[a\x01\x02a\x01\xB86`\x04a\x17mV[a\x08\xCCV[a\x01\x02a\x01\xCB6`\x04a\x17\xEDV[a\x0B\xFFV[a\x01\xE3a\x01\xDE6`\x04a\x16\xE3V[a\x0C\x14V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x10V[a\x01\x02a\x02\t6`\x04a\x15{V[a\x0C\x8FV[a\x01ea\x02\x1C6`\x04a\x18\x1BV[a\rVV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@QcZ;t\xB9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x15\x15`$\x83\x01R\x91\x90\x91\x16\x90cZ;t\xB9\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x91W=`\0\x80>=`\0\xFD[PPPP`\0a\x02\xAC`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x16\x91\x90a\x18\x7FV[\x95PPPPPPa\x03,g\x0E\x92Yo\xD6)\0\0\x90V[\x81\x10\x15a\x03LW`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\x1AL\xA3{`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R\x91\x90\x91\x16\x90ci2\x8D\xEC\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x18\xC9V[P`\0`\0\x80Q` a\x1A\x9E\x839\x81Q\x91Ra\x02\xACV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc(S\nG`\xE0\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c(S\nG\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04GW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04[W=`\0\x80>=`\0\xFD[PPPP`\0a\x04v`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE0\x91\x90a\x18\x7FV[\x95PPPPPPa\x04\xF6g\x0E\x92Yo\xD6)\0\0\x90V[\x81\x10\x15a\x05\x16W`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x050\x91\x90a\x18\xE2V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\x18\xE2V[\x90Pa\x05\xBA`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x87a\x12_V[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x88\x90R0`D\x83\x01R`\0`d\x83\x01R\x91\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\"W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x066W=`\0\x80>=`\0\xFD[PPPPa\x06X\x81a\x06S`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[a\x12\xE1V[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x06v\x91\x90a\x18\xE2V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a\x18\xC9V[\x93\x92PPPV[`\0`@Q` \x01a\x07+\x90` \x80\x82R`\x1C\x90\x82\x01R\x7FAave V3 aToken Adaptor V 1.0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07q\x82a\x0C\x14V[\x81`\0\x81Q\x81\x10a\x07\x84Wa\x07\x84a\x18\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\0a\x07\xB9sTXk\xE6.<5\x807Z\xE3r<\x14RS\x06\x0C\xA0\xC2\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\xE1\x9FG\0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1A\x91\x90a\x18\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80a\x08\xA9WPsTXk\xE6.<5\x807Z\xE3r<\x14RS\x06\x0C\xA0\xC2`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x89\xB6O`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA1\x91\x90a\x18\xC9V[c\x05\xF5\xE1\0\x14\x15[\x15a\x08\xC7W`@Qcixg5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x08\xD5\x83a\x13pV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xEB\x91\x90a\x18\xE2V[\x90P`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`\x01`\x01`\xA0\x1B\x03\x16ci2\x8D\xEC\x82`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tHW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tl\x91\x90a\x18\xE2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x88\x90R0`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE3\x91\x90a\x18\xC9V[P`\0\x80`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n`\x91\x90a\x18\x7FV[\x95PPPP\x92PP`\0\x82\x11\x15a\x0B\x7FW`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\xED\xDF\x1By`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xED\xDF\x1By\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEA\x91\x90a\x18\xC9V[\x15a\x0B\x08W`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x0B\x1E\x91\x90a\x18\xC9V[\x90P\x80`\0\x03a\x0BAW`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\x0B\\WPg\x0E\x92Yo\xD6)\0\0[\x80\x82\x10\x15a\x0B}W`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x0B\xF6\x86\x88\x85`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90a\x18\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x14\x07V[PPPPPPPV[a\x05\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x12_V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0C+\x91\x90a\x18\xE2V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a\x18\xE2V[a\x0C\x99\x82\x82a\x14\x7FV[\x90Pa\x0C\xBD`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x83a\x12_V[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R`\0`d\x83\x01R\x91\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r9W=`\0\x80>=`\0\xFD[PPPPa\x05\x16\x82a\x06S`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\rm\x91\x90a\x18\xE2V[\x90P`\0\x80\x80\x80`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc/\xE4\xA1_`\xE2\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a\x18\x7FV[\x95PP\x94PP\x93P\x93P\x82`\0\x03a\x0EsW`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eg\x91\x90a\x18\xC9V[\x95PPPPPPa\x12YV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\xED\xDF\x1By`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xED\xDF\x1By\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xEC\x91\x90a\x18\xC9V[\x15a\x0E\xFFW`\0\x95PPPPPPa\x12YV[`\0\x87\x80` \x01\x90Q\x81\x01\x90a\x0F\x15\x91\x90a\x18\xC9V[\x90P\x80`\0\x03a\x0F.W`\0\x96PPPPPPPa\x12YV[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\x0FIWPg\x0E\x92Yo\xD6)\0\0[`\0f#\x86\xF2o\xC1\0\0a\x0F]\x81\x84a\x19+V[\x92Pa\x0Fi\x81\x84a\x19+V[\x84\x10\x15a\x0F\x81W`\0\x98PPPPPPPPPa\x12YV[a\x0F\x9D\x86a\x0F\x95\x87eZ\xF3\x10z@\0a\x19>V[\x85\x91\x90a\x15\0V[a\x0F\xA7\x90\x88a\x19]V[\x91P`\0\x88`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\r\x91\x90a\x18\xE2V[\x90P`\x003`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10s\x91\x90a\x18\xE2V[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xDE\x91\x90a\x18\xE2V[`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x83\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11N\x91\x90a\x18\xC9V[\x90P`\0a\x11\xCA\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB7\x91\x90a\x19pV[a\x11\xC2\x90`\na\x1AqV[\x87\x90\x84a\x15\0V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8E\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x128\x91\x90a\x18\xC9V[\x90P\x80\x82\x11a\x12GW\x81a\x12IV[\x80[\x9DPPPPPPPPPPPPPP[\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x131W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13U\x91\x90a\x18\xC9V[\x11\x15a\x05\x16Wa\x05\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x12_V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\x13\xE6WP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE6\x91\x90a\x1A\x80V[\x15a\x14\x04W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x12\xD2V[`\0`\0\x19\x82\x03a\x14\xF9W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xF2\x91\x90a\x18\xC9V[\x90Pa\x12YV[P\x80a\x12YV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15\x18W`\0\x80\xFD[\x04\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14\x04W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x14\x04W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15UW`\0\x80\xFD[\x825a\x15`\x81a\x15\x1FV[\x91P` \x83\x015a\x15p\x81a\x154V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x8EW`\0\x80\xFD[\x825a\x15\x99\x81a\x15\x1FV[\x94` \x93\x90\x93\x015\x93PPPV[`\xFF\x81\x16\x81\x14a\x14\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\xC8W`\0\x80\xFD[\x815a\x06\xE1\x81a\x15\xA7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x15\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\x15Wa\x16\x15a\x15\xD3V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x16=Wa\x16=a\x15\xD3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x16VW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x8BW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xAAW`\0\x80\xFD[a\x16\xB6\x87\x83\x88\x01a\x15\xE9V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x16\xCCW`\0\x80\xFD[Pa\x16\xD9\x86\x82\x87\x01a\x15\xE9V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\xF5W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x0CW`\0\x80\xFD[a\x17\x18\x84\x82\x85\x01a\x15\xE9V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17aW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17<V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\x83W`\0\x80\xFD[\x845\x93P` \x85\x015a\x17\x95\x81a\x15\x1FV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xB2W`\0\x80\xFD[a\x17\xBE\x88\x83\x89\x01a\x15\xE9V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x17\xD4W`\0\x80\xFD[Pa\x17\xE1\x87\x82\x88\x01a\x15\xE9V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\0W`\0\x80\xFD[\x825a\x18\x0B\x81a\x15\x1FV[\x91P` \x83\x015a\x15p\x81a\x15\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x18.W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18FW`\0\x80\xFD[a\x18R\x86\x83\x87\x01a\x15\xE9V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x18hW`\0\x80\xFD[Pa\x18u\x85\x82\x86\x01a\x15\xE9V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18\x98W`\0\x80\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x18\xDBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xF4W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x15\x1FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12YWa\x12Ya\x19\x15V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x19XWa\x19Xa\x19\x15V[P\x02\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12YWa\x12Ya\x19\x15V[`\0` \x82\x84\x03\x12\x15a\x19\x82W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x15\xA7V[`\x01\x81\x81[\x80\x85\x11\x15a\x19\xC8W\x81`\0\x19\x04\x82\x11\x15a\x19\xAEWa\x19\xAEa\x19\x15V[\x80\x85\x16\x15a\x19\xBBW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x19\x92V[P\x92P\x92\x90PV[`\0\x82a\x19\xDFWP`\x01a\x12YV[\x81a\x19\xECWP`\0a\x12YV[\x81`\x01\x81\x14a\x1A\x02W`\x02\x81\x14a\x1A\x0CWa\x1A(V[`\x01\x91PPa\x12YV[`\xFF\x84\x11\x15a\x1A\x1DWa\x1A\x1Da\x19\x15V[PP`\x01\x82\x1Ba\x12YV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1AKWP\x81\x81\na\x12YV[a\x1AU\x83\x83a\x19\x8DV[\x80`\0\x19\x04\x82\x11\x15a\x1AiWa\x1Aia\x19\x15V[\x02\x93\x92PPPV[`\0a\x06\xE1`\xFF\x84\x16\x83a\x19\xD0V[`\0` \x82\x84\x03\x12\x15a\x1A\x92W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x154V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\x87\x87\x0B\xCA??\xD63\\?L\xE89-i5\x0BO\xA4\xE2\xA2dipfsX\"\x12 }\xB3\x15\xBD5)\xE7`\x98\x16\x94{\x0C\xBD^pPk\x16Z\x91+\xBDMD2\x94\xF2\x18\xFDX\xBDdsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static AAVEV3ATOKENADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\xBDW\x80c\xE1p\xA9\xBF\x14a\x01\xD0W\x80c\xEE\xB1I\xE7\x14a\x01\xFBW\x80c\xFAP\xE5\xD2\x14a\x02\x0EW`\0\x80\xFD[\x80c\x895:\t\x14a\x01{W\x80c\xAE\xFF\xDD\xDE\x14a\x01\x8AW\x80c\xC9\x11\x1B\xD7\x14a\x01\xAAW`\0\x80\xFD[\x80cJ\xC89\xF7\x11a\0\xC8W\x80cJ\xC89\xF7\x14a\x01,W\x80ciD\\1\x14a\x01?W\x80cxASe\x14a\x01RW\x80cy\x98\xA1\xC4\x14a\x01sW`\0\x80\xFD[\x80c#\xBF)\x8F\x14a\0\xEFW\x80c>\x03*;\x14a\x01\x04W\x80cH~\xDE\x04\x14a\x01\x19W[`\0\x80\xFD[a\x01\x02a\0\xFD6`\x04a\x15BV[a\x02!V[\0[`@Qa#(\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x02a\x01'6`\x04a\x15{V[a\x03QV[a\x01\x02a\x01:6`\x04a\x15\xB6V[a\x03\xF2V[a\x01\x02a\x01M6`\x04a\x16vV[a\x05\x1AV[a\x01ea\x01`6`\x04a\x16\xE3V[a\x06_V[`@Q\x90\x81R` \x01a\x01\x10V[a\x01ea\x06\xE8V[`@Q`\0\x81R` \x01a\x01\x10V[a\x01\x9Da\x01\x986`\x04a\x16\xE3V[a\x07FV[`@Qa\x01\x10\x91\x90a\x17 V[a\x01\x02a\x01\xB86`\x04a\x17mV[a\x08\xCCV[a\x01\x02a\x01\xCB6`\x04a\x17\xEDV[a\x0B\xFFV[a\x01\xE3a\x01\xDE6`\x04a\x16\xE3V[a\x0C\x14V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x10V[a\x01\x02a\x02\t6`\x04a\x15{V[a\x0C\x8FV[a\x01ea\x02\x1C6`\x04a\x18\x1BV[a\rVV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@QcZ;t\xB9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x15\x15`$\x83\x01R\x91\x90\x91\x16\x90cZ;t\xB9\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x91W=`\0\x80>=`\0\xFD[PPPP`\0a\x02\xAC`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x16\x91\x90a\x18\x7FV[\x95PPPPPPa\x03,g\x0E\x92Yo\xD6)\0\0\x90V[\x81\x10\x15a\x03LW`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\x1AL\xA3{`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R\x91\x90\x91\x16\x90ci2\x8D\xEC\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x18\xC9V[P`\0`\0\x80Q` a\x1A\x9E\x839\x81Q\x91Ra\x02\xACV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc(S\nG`\xE0\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c(S\nG\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04GW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04[W=`\0\x80>=`\0\xFD[PPPP`\0a\x04v`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xBCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE0\x91\x90a\x18\x7FV[\x95PPPPPPa\x04\xF6g\x0E\x92Yo\xD6)\0\0\x90V[\x81\x10\x15a\x05\x16W`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x050\x91\x90a\x18\xE2V[\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\x18\xE2V[\x90Pa\x05\xBA`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x87a\x12_V[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x88\x90R0`D\x83\x01R`\0`d\x83\x01R\x91\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x06\"W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x066W=`\0\x80>=`\0\xFD[PPPPa\x06X\x81a\x06S`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[a\x12\xE1V[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x06v\x91\x90a\x18\xE2V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xBDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a\x18\xC9V[\x93\x92PPPV[`\0`@Q` \x01a\x07+\x90` \x80\x82R`\x1C\x90\x82\x01R\x7FAave V3 aToken Adaptor V 1.0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x07q\x82a\x0C\x14V[\x81`\0\x81Q\x81\x10a\x07\x84Wa\x07\x84a\x18\xFFV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\0a\x07\xB9sTXk\xE6.<5\x807Z\xE3r<\x14RS\x06\x0C\xA0\xC2\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\xE1\x9FG\0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xF6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x1A\x91\x90a\x18\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80a\x08\xA9WPsTXk\xE6.<5\x807Z\xE3r<\x14RS\x06\x0C\xA0\xC2`\x01`\x01`\xA0\x1B\x03\x16c\x8C\x89\xB6O`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA1\x91\x90a\x18\xC9V[c\x05\xF5\xE1\0\x14\x15[\x15a\x08\xC7W`@Qcixg5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x08\xD5\x83a\x13pV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xEB\x91\x90a\x18\xE2V[\x90P`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`\x01`\x01`\xA0\x1B\x03\x16ci2\x8D\xEC\x82`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tHW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tl\x91\x90a\x18\xE2V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x88\x90R0`D\x82\x01R`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t\xBFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE3\x91\x90a\x18\xC9V[P`\0\x80`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc/\xE4\xA1_`\xE2\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n<W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n`\x91\x90a\x18\x7FV[\x95PPPP\x92PP`\0\x82\x11\x15a\x0B\x7FW`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\xED\xDF\x1By`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xED\xDF\x1By\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEA\x91\x90a\x18\xC9V[\x15a\x0B\x08W`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x80` \x01\x90Q\x81\x01\x90a\x0B\x1E\x91\x90a\x18\xC9V[\x90P\x80`\0\x03a\x0BAW`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\x0B\\WPg\x0E\x92Yo\xD6)\0\0[\x80\x82\x10\x15a\x0B}W`@Qc\x10l-/`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P[a\x0B\xF6\x86\x88\x85`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE6\x91\x90a\x18\xE2V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a\x14\x07V[PPPPPPPV[a\x05\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x12_V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0C+\x91\x90a\x18\xE2V[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CkW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xE1\x91\x90a\x18\xE2V[a\x0C\x99\x82\x82a\x14\x7FV[\x90Pa\x0C\xBD`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x83a\x12_V[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qca{\xA07`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R0`D\x83\x01R`\0`d\x83\x01R\x91\x90\x91\x16\x90ca{\xA07\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r%W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r9W=`\0\x80>=`\0\xFD[PPPPa\x05\x16\x82a\x06S`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R\x90V[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\rm\x91\x90a\x18\xE2V[\x90P`\0\x80\x80\x80`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc/\xE4\xA1_`\xE2\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xBF\x92\x85|\x90`$\x01`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a\x18\x7FV[\x95PP\x94PP\x93P\x93P\x82`\0\x03a\x0EsW`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0ECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Eg\x91\x90a\x18\xC9V[\x95PPPPPPa\x12YV[`\0\x80Q` a\x1A\x9E\x839\x81Q\x91R`@Qc\xED\xDF\x1By`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xED\xDF\x1By\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xEC\x91\x90a\x18\xC9V[\x15a\x0E\xFFW`\0\x95PPPPPPa\x12YV[`\0\x87\x80` \x01\x90Q\x81\x01\x90a\x0F\x15\x91\x90a\x18\xC9V[\x90P\x80`\0\x03a\x0F.W`\0\x96PPPPPPPa\x12YV[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\x0FIWPg\x0E\x92Yo\xD6)\0\0[`\0f#\x86\xF2o\xC1\0\0a\x0F]\x81\x84a\x19+V[\x92Pa\x0Fi\x81\x84a\x19+V[\x84\x10\x15a\x0F\x81W`\0\x98PPPPPPPPPa\x12YV[a\x0F\x9D\x86a\x0F\x95\x87eZ\xF3\x10z@\0a\x19>V[\x85\x91\x90a\x15\0V[a\x0F\xA7\x90\x88a\x19]V[\x91P`\0\x88`\x01`\x01`\xA0\x1B\x03\x16c\xB1j\x19\xDE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xE9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\r\x91\x90a\x18\xE2V[\x90P`\x003`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10OW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10s\x91\x90a\x18\xE2V[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xDE\x91\x90a\x18\xE2V[`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x91\x92P`\0\x91\x83\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11N\x91\x90a\x18\xC9V[\x90P`\0a\x11\xCA\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x93W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xB7\x91\x90a\x19pV[a\x11\xC2\x90`\na\x1AqV[\x87\x90\x84a\x15\0V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x8E\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x128\x91\x90a\x18\xC9V[\x90P\x80\x82\x11a\x12GW\x81a\x12IV[\x80[\x9DPPPPPPPPPPPPPP[\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x131W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13U\x91\x90a\x18\xC9V[\x11\x15a\x05\x16Wa\x05\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x12_V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a\x13\xE6WP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xE6\x91\x90a\x1A\x80V[\x15a\x14\x04W`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x12\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x12\xD2V[`\0`\0\x19\x82\x03a\x14\xF9W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xCEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xF2\x91\x90a\x18\xC9V[\x90Pa\x12YV[P\x80a\x12YV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15\x18W`\0\x80\xFD[\x04\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14\x04W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x14\x04W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x15UW`\0\x80\xFD[\x825a\x15`\x81a\x15\x1FV[\x91P` \x83\x015a\x15p\x81a\x154V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x15\x8EW`\0\x80\xFD[\x825a\x15\x99\x81a\x15\x1FV[\x94` \x93\x90\x93\x015\x93PPPV[`\xFF\x81\x16\x81\x14a\x14\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\xC8W`\0\x80\xFD[\x815a\x06\xE1\x81a\x15\xA7V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x15\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\x15Wa\x16\x15a\x15\xD3V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x16=Wa\x16=a\x15\xD3V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x16VW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x16\x8BW`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x16\xAAW`\0\x80\xFD[a\x16\xB6\x87\x83\x88\x01a\x15\xE9V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x16\xCCW`\0\x80\xFD[Pa\x16\xD9\x86\x82\x87\x01a\x15\xE9V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x16\xF5W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17\x0CW`\0\x80\xFD[a\x17\x18\x84\x82\x85\x01a\x15\xE9V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x17aW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x17<V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x17\x83W`\0\x80\xFD[\x845\x93P` \x85\x015a\x17\x95\x81a\x15\x1FV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x17\xB2W`\0\x80\xFD[a\x17\xBE\x88\x83\x89\x01a\x15\xE9V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x17\xD4W`\0\x80\xFD[Pa\x17\xE1\x87\x82\x88\x01a\x15\xE9V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x18\0W`\0\x80\xFD[\x825a\x18\x0B\x81a\x15\x1FV[\x91P` \x83\x015a\x15p\x81a\x15\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x18.W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18FW`\0\x80\xFD[a\x18R\x86\x83\x87\x01a\x15\xE9V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x18hW`\0\x80\xFD[Pa\x18u\x85\x82\x86\x01a\x15\xE9V[\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x18\x98W`\0\x80\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x18\xDBW`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x18\xF4W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x15\x1FV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12YWa\x12Ya\x19\x15V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a\x19XWa\x19Xa\x19\x15V[P\x02\x90V[\x81\x81\x03\x81\x81\x11\x15a\x12YWa\x12Ya\x19\x15V[`\0` \x82\x84\x03\x12\x15a\x19\x82W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x15\xA7V[`\x01\x81\x81[\x80\x85\x11\x15a\x19\xC8W\x81`\0\x19\x04\x82\x11\x15a\x19\xAEWa\x19\xAEa\x19\x15V[\x80\x85\x16\x15a\x19\xBBW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a\x19\x92V[P\x92P\x92\x90PV[`\0\x82a\x19\xDFWP`\x01a\x12YV[\x81a\x19\xECWP`\0a\x12YV[\x81`\x01\x81\x14a\x1A\x02W`\x02\x81\x14a\x1A\x0CWa\x1A(V[`\x01\x91PPa\x12YV[`\xFF\x84\x11\x15a\x1A\x1DWa\x1A\x1Da\x19\x15V[PP`\x01\x82\x1Ba\x12YV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a\x1AKWP\x81\x81\na\x12YV[a\x1AU\x83\x83a\x19\x8DV[\x80`\0\x19\x04\x82\x11\x15a\x1AiWa\x1Aia\x19\x15V[\x02\x93\x92PPPV[`\0a\x06\xE1`\xFF\x84\x16\x83a\x19\xD0V[`\0` \x82\x84\x03\x12\x15a\x1A\x92W`\0\x80\xFD[\x81Qa\x06\xE1\x81a\x154V\xFE\0\0\0\0\0\0\0\0\0\0\0\0\x87\x87\x0B\xCA??\xD63\\?L\xE89-i5\x0BO\xA4\xE2\xA2dipfsX\"\x12 }\xB3\x15\xBD5)\xE7`\x98\x16\x94{\x0C\xBD^pPk\x16Z\x91+\xBDMD2\x94\xF2\x18\xFDX\xBDdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEV3ATOKENADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveV3ATokenAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveV3ATokenAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveV3ATokenAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveV3ATokenAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveV3ATokenAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveV3ATokenAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveV3ATokenAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEV3ATOKENADAPTORV1_ABI.clone(),
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
                AAVEV3ATOKENADAPTORV1_ABI.clone(),
                AAVEV3ATOKENADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `adjustIsolationModeAssetAsCollateral` (0x23bf298f) function
        pub fn adjust_isolation_mode_asset_as_collateral(
            &self,
            asset: ::ethers::core::types::Address,
            use_as_collateral: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 191, 41, 143], (asset, use_as_collateral))
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
        ///Calls the contract's `changeEMode` (0x4ac839f7) function
        pub fn change_e_mode(
            &self,
            category_id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 200, 57, 247], category_id)
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
        ///Calls the contract's `depositToAave` (0xeeb149e7) function
        pub fn deposit_to_aave(
            &self,
            token_to_deposit: ::ethers::core::types::Address,
            amount_to_deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 177, 73, 231], (token_to_deposit, amount_to_deposit))
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
            config_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (assets, receiver, adaptor_data, config_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawFromAave` (0x487ede04) function
        pub fn withdraw_from_aave(
            &self,
            token_to_withdraw: ::ethers::core::types::Address,
            amount_to_withdraw: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 126, 222, 4], (token_to_withdraw, amount_to_withdraw))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawableFrom` (0xfa50e5d2) function
        pub fn withdrawable_from(
            &self,
            adaptor_data: ::ethers::core::types::Bytes,
            config_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 80, 229, 210], (adaptor_data, config_data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AaveV3ATokenAdaptorV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AaveV3ATokenAdaptor__HealthFactorTooLow` with signature `AaveV3ATokenAdaptor__HealthFactorTooLow()` and selector `0x83616978`
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
        name = "AaveV3ATokenAdaptor__HealthFactorTooLow",
        abi = "AaveV3ATokenAdaptor__HealthFactorTooLow()"
    )]
    pub struct AaveV3ATokenAdaptor__HealthFactorTooLow;
    ///Custom Error type `AaveV3ATokenAdaptor__OracleUsesDifferentBase` with signature `AaveV3ATokenAdaptor__OracleUsesDifferentBase()` and selector `0xd2f0ce6a`
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
        name = "AaveV3ATokenAdaptor__OracleUsesDifferentBase",
        abi = "AaveV3ATokenAdaptor__OracleUsesDifferentBase()"
    )]
    pub struct AaveV3ATokenAdaptor__OracleUsesDifferentBase;
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
    pub enum AaveV3ATokenAdaptorV1Errors {
        AaveV3ATokenAdaptor__HealthFactorTooLow(AaveV3ATokenAdaptor__HealthFactorTooLow),
        AaveV3ATokenAdaptor__OracleUsesDifferentBase(
            AaveV3ATokenAdaptor__OracleUsesDifferentBase,
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
    impl ::ethers::core::abi::AbiDecode for AaveV3ATokenAdaptorV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AaveV3ATokenAdaptor__HealthFactorTooLow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AaveV3ATokenAdaptor__HealthFactorTooLow(decoded));
            }
            if let Ok(decoded) = <AaveV3ATokenAdaptor__OracleUsesDifferentBase as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AaveV3ATokenAdaptor__OracleUsesDifferentBase(decoded));
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
    impl ::ethers::core::abi::AbiEncode for AaveV3ATokenAdaptorV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AaveV3ATokenAdaptor__HealthFactorTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AaveV3ATokenAdaptor__OracleUsesDifferentBase(element) => {
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
    impl ::ethers::contract::ContractRevert for AaveV3ATokenAdaptorV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AaveV3ATokenAdaptor__HealthFactorTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AaveV3ATokenAdaptor__OracleUsesDifferentBase as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for AaveV3ATokenAdaptorV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AaveV3ATokenAdaptor__HealthFactorTooLow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AaveV3ATokenAdaptor__OracleUsesDifferentBase(element) => {
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
    impl ::core::convert::From<::std::string::String> for AaveV3ATokenAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AaveV3ATokenAdaptor__HealthFactorTooLow>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: AaveV3ATokenAdaptor__HealthFactorTooLow) -> Self {
            Self::AaveV3ATokenAdaptor__HealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<AaveV3ATokenAdaptor__OracleUsesDifferentBase>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: AaveV3ATokenAdaptor__OracleUsesDifferentBase) -> Self {
            Self::AaveV3ATokenAdaptor__OracleUsesDifferentBase(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage> for AaveV3ATokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for AaveV3ATokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    ///Container type for all input parameters for the `adjustIsolationModeAssetAsCollateral` function with signature `adjustIsolationModeAssetAsCollateral(address,bool)` and selector `0x23bf298f`
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
        name = "adjustIsolationModeAssetAsCollateral",
        abi = "adjustIsolationModeAssetAsCollateral(address,bool)"
    )]
    pub struct AdjustIsolationModeAssetAsCollateralCall {
        pub asset: ::ethers::core::types::Address,
        pub use_as_collateral: bool,
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
    ///Container type for all input parameters for the `changeEMode` function with signature `changeEMode(uint8)` and selector `0x4ac839f7`
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
    #[ethcall(name = "changeEMode", abi = "changeEMode(uint8)")]
    pub struct ChangeEModeCall {
        pub category_id: u8,
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
        pub p2: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `depositToAave` function with signature `depositToAave(address,uint256)` and selector `0xeeb149e7`
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
    #[ethcall(name = "depositToAave", abi = "depositToAave(address,uint256)")]
    pub struct DepositToAaveCall {
        pub token_to_deposit: ::ethers::core::types::Address,
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
        pub config_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdrawFromAave` function with signature `withdrawFromAave(address,uint256)` and selector `0x487ede04`
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
    #[ethcall(name = "withdrawFromAave", abi = "withdrawFromAave(address,uint256)")]
    pub struct WithdrawFromAaveCall {
        pub token_to_withdraw: ::ethers::core::types::Address,
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
        pub config_data: ::ethers::core::types::Bytes,
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
    pub enum AaveV3ATokenAdaptorV1Calls {
        AdjustIsolationModeAssetAsCollateral(AdjustIsolationModeAssetAsCollateralCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        ChangeEMode(ChangeEModeCall),
        Deposit(DepositCall),
        DepositToAave(DepositToAaveCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawFromAave(WithdrawFromAaveCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for AaveV3ATokenAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdjustIsolationModeAssetAsCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdjustIsolationModeAssetAsCollateral(decoded));
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
            if let Ok(decoded) = <ChangeEModeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChangeEMode(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositToAaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositToAave(decoded));
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
            if let Ok(decoded) = <WithdrawFromAaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawFromAave(decoded));
            }
            if let Ok(decoded) = <WithdrawableFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawableFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AaveV3ATokenAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdjustIsolationModeAssetAsCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeEMode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositToAave(element) => {
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
                Self::WithdrawFromAave(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AaveV3ATokenAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdjustIsolationModeAssetAsCollateral(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeEMode(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositToAave(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFromAave(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdjustIsolationModeAssetAsCollateralCall>
    for AaveV3ATokenAdaptorV1Calls {
        fn from(value: AdjustIsolationModeAssetAsCollateralCall) -> Self {
            Self::AdjustIsolationModeAssetAsCollateral(value)
        }
    }
    impl ::core::convert::From<AssetOfCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ChangeEModeCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: ChangeEModeCall) -> Self {
            Self::ChangeEMode(value)
        }
    }
    impl ::core::convert::From<DepositCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositToAaveCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: DepositToAaveCall) -> Self {
            Self::DepositToAave(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawFromAaveCall> for AaveV3ATokenAdaptorV1Calls {
        fn from(value: WithdrawFromAaveCall) -> Self {
            Self::WithdrawFromAave(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for AaveV3ATokenAdaptorV1Calls {
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
