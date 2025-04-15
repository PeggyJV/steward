pub use collateral_f_token_adaptor_v1::*;
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
pub mod collateral_f_token_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/CollateralFTokenAdaptorV1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("addCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addCollateral"),
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
                                        "_collateralToDeposit",
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
                    ::std::borrow::ToOwned::to_owned("assetOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adaptorData"),
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
                    ::std::borrow::ToOwned::to_owned("removeCollateral"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removeCollateral"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_collateralAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                        "CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
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
                        "CollateralFTokenAdaptor__HealthFactorTooLow",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CollateralFTokenAdaptor__HealthFactorTooLow",
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
    pub static COLLATERALFTOKENADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x12\xD88\x03\x80a\x12\xD8\x839\x81\x01`@\x81\x90Ra\0/\x91a\0zV[a\08\x81a\0NV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80R`\xA0Ra\0\xB4V[g\x0E\x92Yo\xD6)\0\0\x81\x10\x15a\0wW`@Qc\x97\xED_A`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\0\x8DW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xA4W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\x80Q`\xA0Qa\x11\xF9a\0\xDF`\09`\0\x81\x81`\xF4\x01Ra\x04X\x01R`\0a\x01\xB0\x01Ra\x11\xF9`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xAE\xFF\xDD\xDE\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\xFDW\x80c\xD4\x1D\xDC\x96\x14a\x02\x10W\x80c\xE1p\xA9\xBF\x14a\x02#W\x80c\xFAP\xE5\xD2\x14a\x026W`\0\x80\xFD[\x80c\xAE\xFF\xDD\xDE\x14a\x01\x8BW\x80c\xB0\xE4Uo\x14a\x01\xABW\x80c\xC9\x11\x1B\xD7\x14a\x01\xEAW`\0\x80\xFD[\x80cmu\xB9\xEE\x11a\0\xC8W\x80cmu\xB9\xEE\x14a\x01NW\x80cxASe\x14a\x01aW\x80cy\x98\xA1\xC4\x14a\x01tW\x80c\x895:\t\x14a\x01|W`\0\x80\xFD[\x80c\x1C\xAF\xF8\xB1\x14a\0\xEFW\x80c>\x03*;\x14a\x01)W\x80ciD\\1\x14a\x019W[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qa#(\x81R` \x01a\x01 V[a\x01La\x01G6`\x04a\r\x0FV[a\x02IV[\0[a\x01La\x01\\6`\x04a\r\x94V[a\x02\xA8V[a\x01\x16a\x01o6`\x04a\r\xC0V[a\x02\xFCV[a\x01\x16a\x03&V[`@Q`\0\x81R` \x01a\x01 V[a\x01\x9Ea\x01\x996`\x04a\r\xC0V[a\x03\x97V[`@Qa\x01 \x91\x90a\r\xFDV[a\x01\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[a\x01La\x01\xF86`\x04a\x0EJV[a\x03\xFAV[a\x01La\x02\x0B6`\x04a\x0E\xCAV[a\x04\x13V[a\x01La\x02\x1E6`\x04a\x0F\x03V[a\x04,V[a\x01\xD2a\x0216`\x04a\r\xC0V[a\x04\xABV[a\x01\x16a\x02D6`\x04a\x0F(V[a\x04\xCDV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02_\x91\x90a\x0F\x8CV[\x90P`\0a\x02l\x82a\x04\xD6V[\x90Pa\x02w\x82a\x05:V[\x81a\x02\x8C`\x01`\x01`\xA0\x1B\x03\x83\x16\x82\x88a\x06\xF4V[a\x02\x96\x83\x87a\x07qV[a\x02\xA0\x82\x82a\x07\xCEV[PPPPPPV[a\x02\xB1\x82a\x05:V[`\0a\x02\xBC\x83a\x04\xD6V[\x90P`\0a\x02\xCA\x82\x84a\x08]V[\x90P\x83a\x02\xE1`\x01`\x01`\xA0\x1B\x03\x84\x16\x82\x84a\x06\xF4V[a\x02\xEB\x85\x83a\x07qV[a\x02\xF5\x83\x82a\x07\xCEV[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03\x13\x91\x90a\x0F\x8CV[\x90Pa\x03\x1F\x813a\x08\xDEV[\x93\x92PPPV[`\0`@Q` \x01a\x03|\x90` \x80\x82R`*\x90\x82\x01R\x7FFraxLend Collateral fTokenV2 Ada`@\x82\x01Riptor V 0.1`\xB0\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x03\xC2\x82a\x04\xABV[\x81`\0\x81Q\x81\x10a\x03\xD5Wa\x03\xD5a\x0F\xA9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04(`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x06\xF4V[PPV[a\x045\x81a\x05:V[a\x04?\x82\x82a\tMV[`\0a\x04J\x82a\t\x80V[\x90Pa\x04V\x82\x82a\t\xEEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15a\x04\xA6W`@Qc\x1E\x7F}\xAD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x04\xC2\x91\x90a\x0F\x8CV[\x90Pa\x03\x1F\x81a\x04\xD6V[`\0[\x92\x91PPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xC6\xE1\xC7\xC9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD0\x91\x90a\x0F\x8CV[`\0a\x05Da\x03&V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05z\x93\x92\x91` \x01a\x0F\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF6\x91\x90a\x0F\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06#\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06d\x91\x90a\x107V[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCC\x91\x90a\x10RV[a\x04\xA6W`@Qc\x06sS\x17`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x9DV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x04\x9DV[PPPPV[`@Qc\xCA\xDA\xC4y`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCA\xDA\xC4y\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA0W=`\0\x80>=`\0\xFD[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08B\x91\x90a\x10tV[\x11\x15a\x04(Wa\x04(`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x06\xF4V[`\0`\0\x19\x82\x03a\x08\xD7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD0\x91\x90a\x10tV[\x90Pa\x04\xD0V[P\x80a\x04\xD0V[`@QcZ\xD7\x981`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90c\xB5\xAF0b\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1F\x91\x90a\x10tV[`@Qcj\x0E\xEEK`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD4\x1D\xDC\x96\x90`D\x01a\x07\xA0V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFB\xBB\xF9L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE4\x91\x90a\x10\x8DV[`\x80\x01Q\x92\x91PPV[`\0\x80a\t\xFB\x840a\n\xBBV[\x90P`\0a\n\x0C\x85\x83`\x01\x80a\n\xEDV[\x90P\x80`\0\x03a\n(Wg\x0E\x92Yo\xD6)\0\0\x92PPPa\x04\xD0V[`\0a\n4\x860a\x08\xDEV[\x90P\x80`\0\x03a\nJW`\0\x93PPPPa\x04\xD0V[`\0\x80a\nV\x88a\x0BrV[\x90\x92P\x90P`\0\x83\x83\x83a\nj\x8B\x89a\x11\x1FV[a\nt\x91\x90a\x11DV[a\n~\x91\x90a\x11\x1FV[a\n\x88\x91\x90a\x11DV[\x90P`\0a\n\x95\x8Aa\x0B\xE9V[\x90P`\0a\n\xAC\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0CMV[\x9B\x9APPPPPPPPPPPV[`@QcO\xD4\"\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90cO\xD4\"\xDF\x90`$\x01a\t\x0CV[`@Qc~\xC4\xB5q`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c~\xC4\xB5q\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bi\x91\x90a\x10tV[\x95\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c\x9A)^s`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD8\x91\x90a\x11fV[P\x95\x99\x91\x98P\x90\x96PPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x84\xBD\x05`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD0\x91\x90a\x10tV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CeW`\0\x80\xFD[\x04\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0C\x93W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xAEWa\x0C\xAEa\x0ClV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0C\xD6Wa\x0C\xD6a\x0ClV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\xEFW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r$W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\rCW`\0\x80\xFD[a\rO\x87\x83\x88\x01a\x0C\x82V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\reW`\0\x80\xFD[Pa\rr\x86\x82\x87\x01a\x0C\x82V[\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x91W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xA7W`\0\x80\xFD[\x825a\r\xB2\x81a\r|V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\r\xD2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE9W`\0\x80\xFD[a\r\xF5\x84\x82\x85\x01a\x0C\x82V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E>W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x19V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E`W`\0\x80\xFD[\x845\x93P` \x85\x015a\x0Er\x81a\r|V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x8FW`\0\x80\xFD[a\x0E\x9B\x88\x83\x89\x01a\x0C\x82V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xB1W`\0\x80\xFD[Pa\x0E\xBE\x87\x82\x88\x01a\x0C\x82V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xDDW`\0\x80\xFD[\x825a\x0E\xE8\x81a\r|V[\x91P` \x83\x015a\x0E\xF8\x81a\r|V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x16W`\0\x80\xFD[\x825\x91P` \x83\x015a\x0E\xF8\x81a\r|V[`\0\x80`@\x83\x85\x03\x12\x15a\x0F;W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FSW`\0\x80\xFD[a\x0F_\x86\x83\x87\x01a\x0C\x82V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0FuW`\0\x80\xFD[Pa\x0F\x82\x85\x82\x86\x01a\x0C\x82V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\x9EW`\0\x80\xFD[\x81Qa\x03\x1F\x81a\r|V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`\0` \x84\x15\x15\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x0F\xFBW\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x0F\xDFV[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x102W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10IW`\0\x80\xFD[a\x03\x1F\x82a\x10\x1EV[`\0` \x82\x84\x03\x12\x15a\x10dW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x86W`\0\x80\xFD[PQ\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\x10\x9FW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\xC2Wa\x10\xC2a\x0ClV[`@R\x82Qa\x10\xD0\x81a\r|V[\x81Ra\x10\xDE` \x84\x01a\x10\x1EV[` \x82\x01R`@\x83\x01Q`\x01`\x01`\xB8\x1B\x03\x81\x16\x81\x14a\x10\xFDW`\0\x80\xFD[`@\x82\x01R``\x83\x81\x01Q\x90\x82\x01R`\x80\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xD0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x11aWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x11\x83W`\0\x80\xFD[PP\x86Q` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x8C\x01Q`\xC0\x8D\x01Q`\xE0\x90\x9D\x01Q\x95\x9E\x94\x9DP\x92\x9B\x91\x9AP\x98P\x90\x96P\x94P\x90\x92P\x90PV\xFE\xA2dipfsX\"\x12 \xA7X\xB2\x99\x0F\xB2\xDD\x13\xBC\xBDT\x8Aq\xEDt\xB1\x0C_\xDB\xF9\n6[\xA8\xE5@\xFD\xFEx\xADj\\dsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static COLLATERALFTOKENADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\xAE\xFF\xDD\xDE\x11a\0\x8CW\x80c\xD3\xBF\xE7j\x11a\0fW\x80c\xD3\xBF\xE7j\x14a\x01\xFDW\x80c\xD4\x1D\xDC\x96\x14a\x02\x10W\x80c\xE1p\xA9\xBF\x14a\x02#W\x80c\xFAP\xE5\xD2\x14a\x026W`\0\x80\xFD[\x80c\xAE\xFF\xDD\xDE\x14a\x01\x8BW\x80c\xB0\xE4Uo\x14a\x01\xABW\x80c\xC9\x11\x1B\xD7\x14a\x01\xEAW`\0\x80\xFD[\x80cmu\xB9\xEE\x11a\0\xC8W\x80cmu\xB9\xEE\x14a\x01NW\x80cxASe\x14a\x01aW\x80cy\x98\xA1\xC4\x14a\x01tW\x80c\x895:\t\x14a\x01|W`\0\x80\xFD[\x80c\x1C\xAF\xF8\xB1\x14a\0\xEFW\x80c>\x03*;\x14a\x01)W\x80ciD\\1\x14a\x019W[`\0\x80\xFD[a\x01\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`@Qa#(\x81R` \x01a\x01 V[a\x01La\x01G6`\x04a\r\x0FV[a\x02IV[\0[a\x01La\x01\\6`\x04a\r\x94V[a\x02\xA8V[a\x01\x16a\x01o6`\x04a\r\xC0V[a\x02\xFCV[a\x01\x16a\x03&V[`@Q`\0\x81R` \x01a\x01 V[a\x01\x9Ea\x01\x996`\x04a\r\xC0V[a\x03\x97V[`@Qa\x01 \x91\x90a\r\xFDV[a\x01\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01 V[a\x01La\x01\xF86`\x04a\x0EJV[a\x03\xFAV[a\x01La\x02\x0B6`\x04a\x0E\xCAV[a\x04\x13V[a\x01La\x02\x1E6`\x04a\x0F\x03V[a\x04,V[a\x01\xD2a\x0216`\x04a\r\xC0V[a\x04\xABV[a\x01\x16a\x02D6`\x04a\x0F(V[a\x04\xCDV[`\0\x82\x80` \x01\x90Q\x81\x01\x90a\x02_\x91\x90a\x0F\x8CV[\x90P`\0a\x02l\x82a\x04\xD6V[\x90Pa\x02w\x82a\x05:V[\x81a\x02\x8C`\x01`\x01`\xA0\x1B\x03\x83\x16\x82\x88a\x06\xF4V[a\x02\x96\x83\x87a\x07qV[a\x02\xA0\x82\x82a\x07\xCEV[PPPPPPV[a\x02\xB1\x82a\x05:V[`\0a\x02\xBC\x83a\x04\xD6V[\x90P`\0a\x02\xCA\x82\x84a\x08]V[\x90P\x83a\x02\xE1`\x01`\x01`\xA0\x1B\x03\x84\x16\x82\x84a\x06\xF4V[a\x02\xEB\x85\x83a\x07qV[a\x02\xF5\x83\x82a\x07\xCEV[PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x03\x13\x91\x90a\x0F\x8CV[\x90Pa\x03\x1F\x813a\x08\xDEV[\x93\x92PPPV[`\0`@Q` \x01a\x03|\x90` \x80\x82R`*\x90\x82\x01R\x7FFraxLend Collateral fTokenV2 Ada`@\x82\x01Riptor V 0.1`\xB0\x1B``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x03\xC2\x82a\x04\xABV[\x81`\0\x81Q\x81\x10a\x03\xD5Wa\x03\xD5a\x0F\xA9V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04(`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x06\xF4V[PPV[a\x045\x81a\x05:V[a\x04?\x82\x82a\tMV[`\0a\x04J\x82a\t\x80V[\x90Pa\x04V\x82\x82a\t\xEEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x11\x15a\x04\xA6W`@Qc\x1E\x7F}\xAD`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[PPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x04\xC2\x91\x90a\x0F\x8CV[\x90Pa\x03\x1F\x81a\x04\xD6V[`\0[\x92\x91PPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xC6\xE1\xC7\xC9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x16W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD0\x91\x90a\x0F\x8CV[`\0a\x05Da\x03&V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\0\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05z\x93\x92\x91` \x01a\x0F\xBFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF6\x91\x90a\x0F\x8CV[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06#\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06d\x91\x90a\x107V[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xCC\x91\x90a\x10RV[a\x04\xA6W`@Qc\x06sS\x17`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x9DV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x04\x9DV[PPPPV[`@Qc\xCA\xDA\xC4y`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xCA\xDA\xC4y\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xBAW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xA0W=`\0\x80>=`\0\xFD[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08B\x91\x90a\x10tV[\x11\x15a\x04(Wa\x04(`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x06\xF4V[`\0`\0\x19\x82\x03a\x08\xD7W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xACW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD0\x91\x90a\x10tV[\x90Pa\x04\xD0V[P\x80a\x04\xD0V[`@QcZ\xD7\x981`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90c\xB5\xAF0b\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x1F\x91\x90a\x10tV[`@Qcj\x0E\xEEK`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xD4\x1D\xDC\x96\x90`D\x01a\x07\xA0V[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xFB\xBB\xF9L`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xC0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xE4\x91\x90a\x10\x8DV[`\x80\x01Q\x92\x91PPV[`\0\x80a\t\xFB\x840a\n\xBBV[\x90P`\0a\n\x0C\x85\x83`\x01\x80a\n\xEDV[\x90P\x80`\0\x03a\n(Wg\x0E\x92Yo\xD6)\0\0\x92PPPa\x04\xD0V[`\0a\n4\x860a\x08\xDEV[\x90P\x80`\0\x03a\nJW`\0\x93PPPPa\x04\xD0V[`\0\x80a\nV\x88a\x0BrV[\x90\x92P\x90P`\0\x83\x83\x83a\nj\x8B\x89a\x11\x1FV[a\nt\x91\x90a\x11DV[a\n~\x91\x90a\x11\x1FV[a\n\x88\x91\x90a\x11DV[\x90P`\0a\n\x95\x8Aa\x0B\xE9V[\x90P`\0a\n\xAC\x82g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x0CMV[\x9B\x9APPPPPPPPPPPV[`@QcO\xD4\"\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x90\x84\x16\x90cO\xD4\"\xDF\x90`$\x01a\t\x0CV[`@Qc~\xC4\xB5q`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\x82\x15\x15`$\x82\x01R\x81\x15\x15`D\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90c~\xC4\xB5q\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bi\x91\x90a\x10tV[\x95\x94PPPPPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c\x9A)^s`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xB4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xD8\x91\x90a\x11fV[P\x95\x99\x91\x98P\x90\x96PPPPPPPV[`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xF3\x84\xBD\x05`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD0\x91\x90a\x10tV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CeW`\0\x80\xFD[\x04\x92\x91PPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\x0C\x93W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0C\xAEWa\x0C\xAEa\x0ClV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x0C\xD6Wa\x0C\xD6a\x0ClV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x0C\xEFW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r$W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\rCW`\0\x80\xFD[a\rO\x87\x83\x88\x01a\x0C\x82V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\reW`\0\x80\xFD[Pa\rr\x86\x82\x87\x01a\x0C\x82V[\x91PP\x92P\x92P\x92V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r\x91W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xA7W`\0\x80\xFD[\x825a\r\xB2\x81a\r|V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\r\xD2W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xE9W`\0\x80\xFD[a\r\xF5\x84\x82\x85\x01a\x0C\x82V[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0E>W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0E\x19V[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x0E`W`\0\x80\xFD[\x845\x93P` \x85\x015a\x0Er\x81a\r|V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\x8FW`\0\x80\xFD[a\x0E\x9B\x88\x83\x89\x01a\x0C\x82V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xB1W`\0\x80\xFD[Pa\x0E\xBE\x87\x82\x88\x01a\x0C\x82V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0E\xDDW`\0\x80\xFD[\x825a\x0E\xE8\x81a\r|V[\x91P` \x83\x015a\x0E\xF8\x81a\r|V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x0F\x16W`\0\x80\xFD[\x825\x91P` \x83\x015a\x0E\xF8\x81a\r|V[`\0\x80`@\x83\x85\x03\x12\x15a\x0F;W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0FSW`\0\x80\xFD[a\x0F_\x86\x83\x87\x01a\x0C\x82V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0FuW`\0\x80\xFD[Pa\x0F\x82\x85\x82\x86\x01a\x0C\x82V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0F\x9EW`\0\x80\xFD[\x81Qa\x03\x1F\x81a\r|V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x83\x81R`\0` \x84\x15\x15\x81\x84\x01R```@\x84\x01R\x83Q\x80``\x85\x01R`\0[\x81\x81\x10\x15a\x0F\xFBW\x85\x81\x01\x83\x01Q\x85\x82\x01`\x80\x01R\x82\x01a\x0F\xDFV[P`\0`\x80\x82\x86\x01\x01R`\x80`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x94\x93PPPPV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x102W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10IW`\0\x80\xFD[a\x03\x1F\x82a\x10\x1EV[`\0` \x82\x84\x03\x12\x15a\x10dW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\x1FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x10\x86W`\0\x80\xFD[PQ\x91\x90PV[`\0`\xA0\x82\x84\x03\x12\x15a\x10\x9FW`\0\x80\xFD[`@Q`\xA0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x10\xC2Wa\x10\xC2a\x0ClV[`@R\x82Qa\x10\xD0\x81a\r|V[\x81Ra\x10\xDE` \x84\x01a\x10\x1EV[` \x82\x01R`@\x83\x01Q`\x01`\x01`\xB8\x1B\x03\x81\x16\x81\x14a\x10\xFDW`\0\x80\xFD[`@\x82\x01R``\x83\x81\x01Q\x90\x82\x01R`\x80\x92\x83\x01Q\x92\x81\x01\x92\x90\x92RP\x91\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xD0WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82a\x11aWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a\x11\x83W`\0\x80\xFD[PP\x86Q` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x8C\x01Q`\xC0\x8D\x01Q`\xE0\x90\x9D\x01Q\x95\x9E\x94\x9DP\x92\x9B\x91\x9AP\x98P\x90\x96P\x94P\x90\x92P\x90PV\xFE\xA2dipfsX\"\x12 \xA7X\xB2\x99\x0F\xB2\xDD\x13\xBC\xBDT\x8Aq\xEDt\xB1\x0C_\xDB\xF9\n6[\xA8\xE5@\xFD\xFEx\xADj\\dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static COLLATERALFTOKENADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CollateralFTokenAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CollateralFTokenAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CollateralFTokenAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CollateralFTokenAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CollateralFTokenAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CollateralFTokenAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CollateralFTokenAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COLLATERALFTOKENADAPTORV1_ABI.clone(),
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
                COLLATERALFTOKENADAPTORV1_ABI.clone(),
                COLLATERALFTOKENADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `addCollateral` (0x6d75b9ee) function
        pub fn add_collateral(
            &self,
            fraxlend_pair: ::ethers::core::types::Address,
            collateral_to_deposit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [109, 117, 185, 238],
                    (fraxlend_pair, collateral_to_deposit),
                )
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
        ///Calls the contract's `removeCollateral` (0xd41ddc96) function
        pub fn remove_collateral(
            &self,
            collateral_amount: ::ethers::core::types::U256,
            fraxlend_pair: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([212, 29, 220, 150], (collateral_amount, fraxlend_pair))
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
    for CollateralFTokenAdaptorV1<M> {
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
    ///Custom Error type `CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked` with signature `CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(address)` and selector `0x19cd4c5c`
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
        name = "CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked",
        abi = "CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(address)"
    )]
    pub struct CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked {
        pub fraxlend_pair: ::ethers::core::types::Address,
    }
    ///Custom Error type `CollateralFTokenAdaptor__HealthFactorTooLow` with signature `CollateralFTokenAdaptor__HealthFactorTooLow(address)` and selector `0xf3fbed68`
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
        name = "CollateralFTokenAdaptor__HealthFactorTooLow",
        abi = "CollateralFTokenAdaptor__HealthFactorTooLow(address)"
    )]
    pub struct CollateralFTokenAdaptor__HealthFactorTooLow {
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
    pub enum CollateralFTokenAdaptorV1Errors {
        BaseAdaptor__ConstructorHealthFactorTooLow(
            BaseAdaptor__ConstructorHealthFactorTooLow,
        ),
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(
            CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked,
        ),
        CollateralFTokenAdaptor__HealthFactorTooLow(
            CollateralFTokenAdaptor__HealthFactorTooLow,
        ),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CollateralFTokenAdaptorV1Errors {
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
            if let Ok(decoded) = <CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <CollateralFTokenAdaptor__HealthFactorTooLow as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollateralFTokenAdaptor__HealthFactorTooLow(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CollateralFTokenAdaptorV1Errors {
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
                Self::CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollateralFTokenAdaptor__HealthFactorTooLow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CollateralFTokenAdaptorV1Errors {
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
                    == <CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CollateralFTokenAdaptor__HealthFactorTooLow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CollateralFTokenAdaptorV1Errors {
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
                Self::CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::CollateralFTokenAdaptor__HealthFactorTooLow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ConstructorHealthFactorTooLow>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__ConstructorHealthFactorTooLow) -> Self {
            Self::BaseAdaptor__ConstructorHealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    impl ::core::convert::From<
        CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked,
    > for CollateralFTokenAdaptorV1Errors {
        fn from(
            value: CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked,
        ) -> Self {
            Self::CollateralFTokenAdaptor__FraxlendPairPositionsMustBeTracked(value)
        }
    }
    impl ::core::convert::From<CollateralFTokenAdaptor__HealthFactorTooLow>
    for CollateralFTokenAdaptorV1Errors {
        fn from(value: CollateralFTokenAdaptor__HealthFactorTooLow) -> Self {
            Self::CollateralFTokenAdaptor__HealthFactorTooLow(value)
        }
    }
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
    ///Container type for all input parameters for the `addCollateral` function with signature `addCollateral(address,uint256)` and selector `0x6d75b9ee`
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
    #[ethcall(name = "addCollateral", abi = "addCollateral(address,uint256)")]
    pub struct AddCollateralCall {
        pub fraxlend_pair: ::ethers::core::types::Address,
        pub collateral_to_deposit: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `removeCollateral` function with signature `removeCollateral(uint256,address)` and selector `0xd41ddc96`
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
    #[ethcall(name = "removeCollateral", abi = "removeCollateral(uint256,address)")]
    pub struct RemoveCollateralCall {
        pub collateral_amount: ::ethers::core::types::U256,
        pub fraxlend_pair: ::ethers::core::types::Address,
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
    pub enum CollateralFTokenAdaptorV1Calls {
        Frax(FraxCall),
        AddCollateral(AddCollateralCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        MinimumHealthFactor(MinimumHealthFactorCall),
        RemoveCollateral(RemoveCollateralCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for CollateralFTokenAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <FraxCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Frax(decoded));
            }
            if let Ok(decoded) = <AddCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddCollateral(decoded));
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
            if let Ok(decoded) = <RemoveCollateralCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveCollateral(decoded));
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
    impl ::ethers::core::abi::AbiEncode for CollateralFTokenAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Frax(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddCollateral(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
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
                Self::RemoveCollateral(element) => {
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
    impl ::core::fmt::Display for CollateralFTokenAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Frax(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinimumHealthFactor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemoveCollateral(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<FraxCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: FraxCall) -> Self {
            Self::Frax(value)
        }
    }
    impl ::core::convert::From<AddCollateralCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: AddCollateralCall) -> Self {
            Self::AddCollateral(value)
        }
    }
    impl ::core::convert::From<AssetOfCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<MinimumHealthFactorCall>
    for CollateralFTokenAdaptorV1Calls {
        fn from(value: MinimumHealthFactorCall) -> Self {
            Self::MinimumHealthFactor(value)
        }
    }
    impl ::core::convert::From<RemoveCollateralCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: RemoveCollateralCall) -> Self {
            Self::RemoveCollateral(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for CollateralFTokenAdaptorV1Calls {
        fn from(value: WithdrawableFromCall) -> Self {
            Self::WithdrawableFrom(value)
        }
    }
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
