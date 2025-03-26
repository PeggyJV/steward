pub use cellar_v1::*;
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
pub mod cellar_v1 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_registry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract Registry"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_asset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_positions"),
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
                        name: ::std::borrow::ToOwned::to_owned("_positionTypes"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "enum Cellar.PositionType[]",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_holdingPosition"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_withdrawType"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("enum Cellar.WithdrawType"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_symbol"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategistPayout"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAXIMUM_SHARE_LOCK_PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAXIMUM_SHARE_LOCK_PERIOD",
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
                    ::std::borrow::ToOwned::to_owned("MAX_FEE_CUT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_FEE_CUT"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_PERFORMANCE_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_PERFORMANCE_FEE",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_PLATFORM_FEE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_PLATFORM_FEE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_POSITIONS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_POSITIONS"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_REBALANCE_DEVIATION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MAX_REBALANCE_DEVIATION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MINIMUM_SHARE_LOCK_PERIOD"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MINIMUM_SHARE_LOCK_PERIOD",
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
                    ::std::borrow::ToOwned::to_owned("PRICE_ROUTER_REGISTRY_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PRICE_ROUTER_REGISTRY_SLOT",
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
                    ::std::borrow::ToOwned::to_owned("addPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("allowedRebalanceDeviation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "allowedRebalanceDeviation",
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("convertToAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToAssets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("convertToShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("convertToShares"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("decreaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("subtractedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositLimit"),
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
                    ::std::borrow::ToOwned::to_owned("feeData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("highWatermark"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "strategistPerformanceCut",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "strategistPlatformCut",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("platformFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("performanceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feesDistributor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "strategistPayoutAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositionType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositionType"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Cellar.PositionType"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositions"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("holdingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("holdingPosition"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("increaseAllowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addedValue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initiateShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initiateShutdown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPositionUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPositionUsed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("isShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isShutdown"),
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
                    ::std::borrow::ToOwned::to_owned("isTrusted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isTrusted"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("lastAccrual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lastAccrual"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liftShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liftShutdown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityLimit"),
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
                    ::std::borrow::ToOwned::to_owned("maxDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxDeposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("maxMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("maxRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxWithdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("positions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("positions"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("previewDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewDeposit"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("previewMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewMint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("previewRedeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewRedeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("previewWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("previewWithdraw"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                    ::std::borrow::ToOwned::to_owned("pushPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pushPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("rebalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rebalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fromPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetsFrom"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchange"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum SwapRouter.Exchange"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetsTo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("redeem"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("redeem"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("registry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract Registry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
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
                    ::std::borrow::ToOwned::to_owned("resetHighWatermark"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resetHighWatermark"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sendFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendFees"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setDepositLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setDepositLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
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
                    ::std::borrow::ToOwned::to_owned("setFeesDistributor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setFeesDistributor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFeesDistributor",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("setHoldingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setHoldingPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newHoldingPosition",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("setLiquidityLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setLiquidityLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
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
                    ::std::borrow::ToOwned::to_owned("setOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setOwner"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("setPerformanceFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPerformanceFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPerformanceFee"),
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
                    ::std::borrow::ToOwned::to_owned("setPlatformFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPlatformFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPlatformFee"),
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
                    ::std::borrow::ToOwned::to_owned("setRebalanceDeviation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setRebalanceDeviation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDeviation"),
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
                    ::std::borrow::ToOwned::to_owned("setShareLockPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setShareLockPeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newLock"),
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
                    ::std::borrow::ToOwned::to_owned("setStrategistPayoutAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStrategistPayoutAddress",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payout"),
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
                    ::std::borrow::ToOwned::to_owned("setStrategistPerformanceCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStrategistPerformanceCut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cut"),
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
                    ::std::borrow::ToOwned::to_owned("setStrategistPlatformCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStrategistPlatformCut",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cut"),
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
                    ::std::borrow::ToOwned::to_owned("setWithdrawType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setWithdrawType"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newWithdrawType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Cellar.WithdrawType"),
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
                    ::std::borrow::ToOwned::to_owned("shareLockPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shareLockPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("swapPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapPositions"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index2"),
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
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalAssets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("totalAssetsWithdrawable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "totalAssetsWithdrawable",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trustPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trustPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Cellar.PositionType"),
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
                    ::std::borrow::ToOwned::to_owned("userShareLockStartBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userShareLockStartBlock",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawType"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawType"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum Cellar.WithdrawType"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositLimitChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DepositLimitChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesDistributorChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FeesDistributorChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "oldFeesDistributor",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newFeesDistributor",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HighWatermarkReset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("HighWatermarkReset"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newHighWatermark"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HoldingPositionChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HoldingPositionChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidityLimitChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LiquidityLimitChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newLimit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("OwnerUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PerformanceFeeChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PerformanceFeeChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPerformanceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPerformanceFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PlatformFeeChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PlatformFeeChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPlatformFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPlatformFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionRemoved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionRemoved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PositionSwapped"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionSwapped"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPosition1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPosition2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("index2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PulledFromPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PulledFromPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Rebalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Rebalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("fromPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("toPosition"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetsFrom"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assetsTo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RebalanceDeviationChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RebalanceDeviationChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldDeviation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newDeviation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SendFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SendFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "feesInSharesRedeemed",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("feesInAssetsSent"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShareLockingPeriodChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ShareLockingPeriodChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShutdownChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ShutdownChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isShutdown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategistPayoutAddressChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategistPayoutAddressChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPayoutAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPayoutAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategistPerformanceCutChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategistPerformanceCutChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPerformanceCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPerformanceCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StrategistPlatformCutChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StrategistPlatformCutChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPlatformCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPlatformCut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TrustChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TrustChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isTrusted"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("caller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("shares"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WithdrawTypeChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawTypeChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newType"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__AssetMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__AssetMismatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("asset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedAsset"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__ContractNotShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__ContractNotShutdown",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__ContractShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__ContractShutdown",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__DepositRestricted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__DepositRestricted",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("maxDeposit"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__IlliquidWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__IlliquidWithdraw",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("illiquidPosition"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__IncompleteWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__IncompleteWithdraw",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetsOwed"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidCosmosAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidCosmosAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Cellar__InvalidFee"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidFeeCut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidFeeCut",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                        "Cellar__InvalidRebalanceDeviation",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidRebalanceDeviation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requested"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidShareLockPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidShareLockPeriod",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__NotApprovedToDepositOnBehalf",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__NotApprovedToDepositOnBehalf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("depositor"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__PayoutNotSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PayoutNotSet",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionAlreadyUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionAlreadyUsed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionArrayFull"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionArrayFull",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxPositions"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionNotEmpty"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionNotEmpty",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sharesRemaining"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionPricingNotSetUp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionPricingNotSetUp",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__RemoveHoldingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__RemoveHoldingPosition",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__SharesAreLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__SharesAreLocked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "blockSharesAreUnlocked",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentBlock"),
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
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__TotalAssetDeviatedOutsideRange",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__TotalAssetDeviatedOutsideRange",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("min"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("max"),
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
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__TotalSharesMustRemainConstant",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__TotalSharesMustRemainConstant",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("current"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expected"),
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
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__UntrustedPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__UntrustedPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__WrongSwapParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__WrongSwapParams",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__ZeroAssets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Cellar__ZeroAssets"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__ZeroShares"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Cellar__ZeroShares"),
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
    pub static CELLARV1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x01`\x08Ua\x02``@R`\0a\x01\x80\x81\x90Rg\nh\x89\x06\xBD\x8B\0\0a\x01\xA0\x81\x90Ra\x01\xC0RfG\r\xE4\xDF\x82\0\0a\x01\xE0Rg\x01cEx]\x8A\0\0a\x02\0Rs\xB8\x13UKB2f\xBB\xD4\xC1l2\xFA83\x94\x86\x8C\x1FUa\x02 \x81\x90Ra\x02@\x82\x90R`\x0E\x91\x90\x91U\x7F\x01cEx]\x8A\0\0\0G\r\xE4\xDF\x82\0\0\nh\x89\x06\xBD\x8B\0\0\nh\x89\x06\xBD\x8B\0\0`\x0FU`\x10U`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U`\0\x19`\x12\x81\x90U`\x13Ua\x1C `\x15Uf\n\xA8{\xEES\x80\0`\x17U4\x80\x15b\0\0\xC5W`\0\x80\xFD[P`@Qb\0m\x838\x03\x80b\0m\x83\x839\x81\x01`@\x81\x90Rb\0\0\xE8\x91b\0\t\xA4V[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01T\x91\x90b\0\n\xBCV[\x88\x84\x84\x81\x80`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01`1`\xF8\x1B\x81RP\x84\x84\x81`\x03\x90\x81b\0\x01\x86\x91\x90b\0\x0BeV[P`\x04b\0\x01\x95\x82\x82b\0\x0BeV[PP\x82Q` \x80\x85\x01\x91\x90\x91 \x83Q\x84\x83\x01 `\xE0\x82\x90Ra\x01\0\x81\x90RF`\xA0\x81\x81R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81\x88\x01\x81\x90R\x81\x83\x01\x87\x90R``\x82\x01\x86\x90R`\x80\x82\x01\x94\x90\x94R0\x81\x84\x01R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xC0\x01\x90R\x80Q\x94\x01\x93\x90\x93 \x91\x93P\x91\x90`\x80R0`\xC0Ra\x01 RPPPP`\x01`\x01`\xA0\x1B\x03\x93\x84\x16a\x01@RPP`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x84\x16\x92\x83\x17\x90UP`@Q`\0\x90\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x90\x82\x90\xA3P`\x01`\x01`\xA0\x1B\x03\x89\x16a\x01`R\x86Qb\0\x02\xA0\x90`\t\x90` \x8A\x01\x90b\0\x06\xFDV[P`\0\x80[\x88Q\x81\x10\x15b\0\x04\xE6W`\0\x89\x82\x81Q\x81\x10b\0\x02\xC6Wb\0\x02\xC6b\0\x0C1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16\x15b\0\x03!W`@Qc\n\xF6}\x91`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\n\x90\x93R\x92 \x80T\x90\x91\x16\x90\x91\x17\x90U\x88Q\x89\x90\x83\x90\x81\x10b\0\x03sWb\0\x03sb\0\x0C1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0B\x90\x92R`@\x90\x91 \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15b\0\x03\xB5Wb\0\x03\xB5b\0\x0CGV[\x02\x17\x90UPb\0\x03\xC5\x81b\0\x06*V[a\x01`Q`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R\x91\x94P`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x047\x91\x90b\0\n\xBCV[`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\xA6\x91\x90b\0\x0C]V[b\0\x04\xD0W`@Qc~\xB5n\xDB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01b\0\x03\x18V[P\x80b\0\x04\xDD\x81b\0\x0C\x81V[\x91PPb\0\x02\xA5V[P`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16b\0\x05-W`@Qc\x1E\xC7\xBC\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`$\x01b\0\x03\x18V[`\0b\0\x05:\x87b\0\x06*V[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x05\x83W`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x8B\x16`$\x82\x01R`D\x01b\0\x03\x18V[`\r\x80Ta\x01\0`\x01`\xA8\x1B\x03\x19\x81\x16a\x01\0`\x01`\x01`\xA0\x1B\x03\x8B\x16\x02\x90\x81\x17\x83U\x88\x92\x91`\x01`\x01`\xA8\x1B\x03\x19\x16`\xFF\x19\x90\x91\x16\x17`\x01\x83\x81\x81\x11\x15b\0\x05\xD0Wb\0\x05\xD0b\0\x0CGV[\x02\x17\x90UPP`\r\x80T`\x01`\xA8\x1B`\x01`\xE8\x1B\x03\x19\x16`\x01`\xA8\x1BB`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPb\0\x0C\xA9\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15b\0\x06]Wb\0\x06]b\0\x0CGV[\x14\x80b\0\x06~WP`\x02\x81`\x02\x81\x11\x15b\0\x06|Wb\0\x06|b\0\x0CGV[\x14[\x15b\0\x06\xF0W\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x06\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\xE9\x91\x90b\0\n\xBCV[\x93\x92PPPV[P\x90\x91\x90PV[P\x91\x90PV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15b\0\x07UW\x91` \x02\x82\x01[\x82\x81\x11\x15b\0\x07UW\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90b\0\x07\x1EV[Pb\0\x07c\x92\x91Pb\0\x07gV[P\x90V[[\x80\x82\x11\x15b\0\x07cW`\0\x81U`\x01\x01b\0\x07hV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x07\x94W`\0\x80\xFD[PV[\x80Qb\0\x07\xA4\x81b\0\x07~V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x07\xEAWb\0\x07\xEAb\0\x07\xA9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15b\0\x08\x0EWb\0\x08\x0Eb\0\x07\xA9V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x08*W`\0\x80\xFD[\x81Q` b\0\x08Cb\0\x08=\x83b\0\x07\xF2V[b\0\x07\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0\x08cW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0\x08\x8BW\x80Qb\0\x08}\x81b\0\x07~V[\x83R\x91\x83\x01\x91\x83\x01b\0\x08gV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12b\0\x08\xA8W`\0\x80\xFD[\x81Q` b\0\x08\xBBb\0\x08=\x83b\0\x07\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0\x08\xDBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0\x08\x8BW\x80Q`\x03\x81\x10b\0\x08\xF9W`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01b\0\x08\xDFV[\x80Q`\x02\x81\x10b\0\x07\xA4W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12b\0\t)W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\tEWb\0\tEb\0\x07\xA9V[` b\0\t[`\x1F\x83\x01`\x1F\x19\x16\x82\x01b\0\x07\xBFV[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15b\0\tpW`\0\x80\xFD[`\0[\x83\x81\x10\x15b\0\t\x90W\x85\x81\x01\x83\x01Q\x82\x82\x01\x84\x01R\x82\x01b\0\tsV[P`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0a\x01 \x8A\x8C\x03\x12\x15b\0\t\xC4W`\0\x80\xFD[b\0\t\xCF\x8Ab\0\x07\x97V[\x98Pb\0\t\xDF` \x8B\x01b\0\x07\x97V[`@\x8B\x01Q\x90\x98P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\t\xFDW`\0\x80\xFD[b\0\n\x0B\x8D\x83\x8E\x01b\0\x08\x18V[\x98P``\x8C\x01Q\x91P\x80\x82\x11\x15b\0\n\"W`\0\x80\xFD[b\0\n0\x8D\x83\x8E\x01b\0\x08\x96V[\x97Pb\0\n@`\x80\x8D\x01b\0\x07\x97V[\x96Pb\0\nP`\xA0\x8D\x01b\0\t\x07V[\x95P`\xC0\x8C\x01Q\x91P\x80\x82\x11\x15b\0\ngW`\0\x80\xFD[b\0\nu\x8D\x83\x8E\x01b\0\t\x17V[\x94P`\xE0\x8C\x01Q\x91P\x80\x82\x11\x15b\0\n\x8CW`\0\x80\xFD[Pb\0\n\x9B\x8C\x82\x8D\x01b\0\t\x17V[\x92PPb\0\n\xADa\x01\0\x8B\x01b\0\x07\x97V[\x90P\x92\x95\x98P\x92\x95\x98P\x92\x95\x98V[`\0` \x82\x84\x03\x12\x15b\0\n\xCFW`\0\x80\xFD[\x81Qb\0\x06\xE9\x81b\0\x07~V[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\n\xF1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x06\xF7WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\x1F\x82\x11\x15b\0\x0B`W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x0B;WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x0B\\W\x82\x81U`\x01\x01b\0\x0BGV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x0B\x81Wb\0\x0B\x81b\0\x07\xA9V[b\0\x0B\x99\x81b\0\x0B\x92\x84Tb\0\n\xDCV[\x84b\0\x0B\x12V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x0B\xD1W`\0\x84\x15b\0\x0B\xB8WP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x0B\\V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x0C\x02W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x0B\xE1V[P\x85\x82\x10\x15b\0\x0C!W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15b\0\x0CpW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14b\0\x06\xE9W`\0\x80\xFD[`\0`\x01\x82\x01b\0\x0C\xA2WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`\x01\x01\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa_\xF6b\0\r\x8D`\09`\0\x81\x81a\x07\x1B\x01R\x81\x81a\x0CZ\x01R\x81\x81a+L\x01R\x81\x81a-\x93\x01R\x81\x81a:{\x01R\x81\x81a?\xE0\x01R\x81\x81aE\x81\x01RaIh\x01R`\0\x81\x81a\x05\x83\x01R\x81\x81a\x0C\xF8\x01R\x81\x81a\x18\xD5\x01R\x81\x81a\x1B\xCA\x01R\x81\x81a\x1C \x01R\x81\x81a\x1D8\x01R\x81\x81a+\xCE\x01R\x81\x81a,\x0F\x01R\x81\x81a2\xB0\x01R\x81\x81a4\xE6\x01R\x81\x81aF\x1F\x01RaJ\xC2\x01R`\0a8L\x01R`\0a8\x9B\x01R`\0a8v\x01R`\0a7\xCF\x01R`\0a7\xF9\x01R`\0a8#\x01Ra_\xF6`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04tW`\x005`\xE0\x1C\x80c\x94\xBF\x80M\x11a\x02WW\x80c\xC0Ft\"\x11a\x01FW\x80c\xDF\xF9\x0B[\x11a\0\xC3W\x80c\xEF\x8B0\xF7\x11a\0\x87W\x80c\xEF\x8B0\xF7\x14a\n\xA3W\x80c\xF7\xB2N\x08\x14a\n\xB6W\x80c\xFCDE\x91\x14a\n\xBEW\x80c\xFCMC\xBE\x14a\n\xDEW\x80c\xFD\xD20\xB9\x14a\n\xF1W`\0\x80\xFD[\x80c\xDF\xF9\x0B[\x14a\t\xD2W\x80c\xE3\x94H\xE0\x14a\t\xDAW\x80c\xE7S\xE6\0\x14a\t\xF4W\x80c\xEC\xF7\x08X\x14a\n\x8BW\x80c\xEE\xF3>\xCA\x14a\n\x94W`\0\x80\xFD[\x80c\xCE\x96\xCBw\x11a\x01\nW\x80c\xCE\x96\xCBw\x14a\tsW\x80c\xD5\x05\xAC\xCF\x14a\t\x86W\x80c\xD9\x05w~\x14a\t\x99W\x80c\xDDb\xED>\x14a\t\xACW\x80c\xDF\x05\xA5*\x14a\t\xBFW`\0\x80\xFD[\x80c\xC0Ft\"\x14a\t)W\x80c\xC2D$Z\x14a\t<W\x80c\xC6=u\xB6\x14a\tEW\x80c\xC6\xE6\xF5\x92\x14a\tXW\x80c\xC8^^\x13\x14a\tkW`\0\x80\xFD[\x80c\xA8\x14NH\x11a\x01\xD4W\x80c\xB5)*\x99\x11a\x01\x98W\x80c\xB5)*\x99\x14a\x08\xD4W\x80c\xBA\x08vR\x14a\x08\xE7W\x80c\xBD\xC8\x14K\x14a\x08\xFAW\x80c\xBD\xCA\x91e\x14a\t\rW\x80c\xBF\x86\xD6\x90\x14a\t\x1CW`\0\x80\xFD[\x80c\xA8\x14NH\x14a\x08\x80W\x80c\xA9\x05\x9C\xBB\x14a\x08\x88W\x80c\xB0\xA7]6\x14a\x08\x9BW\x80c\xB3\xD7\xF6\xB9\x14a\x08\xAEW\x80c\xB4`\xAF\x94\x14a\x08\xC1W`\0\x80\xFD[\x80c\x9CU,\xA8\x11a\x02\x1BW\x80c\x9CU,\xA8\x14a\x08\tW\x80c\x9C_\0\xC2\x14a\x08\x1CW\x80c\x9E5\xC6[\x14a\x084W\x80c\x9F\xDB\x11\xB6\x14a\x08dW\x80c\xA4W\xC2\xD7\x14a\x08mW`\0\x80\xFD[\x80c\x94\xBF\x80M\x14a\x07\xA5W\x80c\x95\xD8\x9BA\x14a\x07\xB8W\x80c\x96\xD6Hy\x14a\x07\xC0W\x80c\x99\xFB\xAB\x88\x14a\x07\xE3W\x80c\x9Bo\xD1\x8E\x14a\x07\xF6W`\0\x80\xFD[\x80c@-&}\x11a\x03sW\x80co\xF1\xC0*\x11a\x02\xF0W\x80c{;\xAA\xB4\x11a\x02\xB4W\x80c{;\xAA\xB4\x14a\x07=W\x80c~\xCE\xBE\0\x14a\x07WW\x80c\x80'X`\x14a\x07jW\x80c\x8B\x0C\xEB\xF7\x14a\x07\x7FW\x80c\x8D\xA5\xCB[\x14a\x07\x92W`\0\x80\xFD[\x80co\xF1\xC0*\x14a\x06\xC2W\x80cp\xA0\x821\x14a\x06\xD1W\x80cp\xAF}\xF6\x14a\x06\xFAW\x80cr\x167\x15\x14a\x07\rW\x80c{\x109\x99\x14a\x07\x16W`\0\x80\xFD[\x80cX8Es\x11a\x037W\x80cX8Es\x14a\x06yW\x80cZ@\r%\x14a\x06\x8CW\x80c^,Wn\x14a\x06\x94W\x80cnU?e\x14a\x06\x9CW\x80cn\x85\xF1\x83\x14a\x06\xAFW`\0\x80\xFD[\x80c@-&}\x14a\x06\nW\x80cG \x90\xFE\x14a\x06\x1DW\x80cL\xDA\xD5\x06\x14a\x06@W\x80cN\xCA\x8A\x83\x14a\x06SW\x80cS\n7\x14\x14a\x06fW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x04\x01W\x80c8\x9Ar\x94\x11a\x03\xC5W\x80c8\x9Ar\x94\x14a\x05kW\x80c8\xD5.\x0F\x14a\x05~W\x80c9P\x93Q\x14a\x05\xBDW\x80c9\x98\xA6\x81\x14a\x05\xD0W\x80c<\xF9\x9AF\x14a\x05\xF7W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x14a\x05 W\x80c#\xB8r\xDD\x14a\x05(W\x80c/;Z\x13\x14a\x05;W\x80c1<\xE5g\x14a\x05NW\x80c6D\xE5\x15\x14a\x05cW`\0\x80\xFD[\x80c\x07\xA2\xD1:\x11a\x04HW\x80c\x07\xA2\xD1:\x14a\x04\xBAW\x80c\t^\xA7\xB3\x14a\x04\xCDW\x80c\n(\xA4w\x14a\x04\xF0W\x80c\nh\x0E\x18\x14a\x05\x03W\x80c\x13\xAF@5\x14a\x05\rW`\0\x80\xFD[\x80bQ\xA3\xB7\x14a\x04yW\x80c\x01\xE1\xD1\x14\x14a\x04\x94W\x80c\x04\x02\xABc\x14a\x04\x9CW\x80c\x06\xFD\xDE\x03\x14a\x04\xA5W[`\0\x80\xFD[a\x04\x81`\x08\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x81a\x0B\x04V[a\x04\x81a\x1C \x81V[a\x04\xADa\rjV[`@Qa\x04\x8B\x91\x90aW\xA3V[a\x04\x81a\x04\xC86`\x04aW\xD6V[a\r\xFCV[a\x04\xE0a\x04\xDB6`\x04aX\x04V[a\x0E\x15V[`@Q\x90\x15\x15\x81R` \x01a\x04\x8BV[a\x04\x81a\x04\xFE6`\x04aW\xD6V[a\x0E-V[a\x05\x0Ba\x0EbV[\0[a\x05\x0Ba\x05\x1B6`\x04aX0V[a\x0E\xFFV[`\x02Ta\x04\x81V[a\x04\xE0a\x0566`\x04aXMV[a\x0FuV[a\x05\x0Ba\x05I6`\x04aX\x9BV[a\x0F\x9BV[`\x12[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x04\x81a\x10,V[a\x04\x81a\x05y6`\x04aX\xB8V[a\x10;V[a\x05\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x04\xE0a\x05\xCB6`\x04aX\x04V[a\x12\xABV[a\x05\xDFg\x02\xC6\x8A\xF0\xBB\x14\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x05\x0Ba\x06\x056`\x04aYjV[a\x12\xCDV[a\x04\x81a\x06\x186`\x04aX0V[a\x13\x9FV[a\x04\xE0a\x06+6`\x04aX0V[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\x81a\x06N6`\x04aW\xD6V[a\x14GV[a\x05\x0Ba\x06a6`\x04aY\x93V[a\x14oV[a\x05\x0Ba\x06t6`\x04aW\xD6V[a\x15\xD9V[a\x05\x0Ba\x06\x876`\x04aY\xC3V[a\x16\x83V[a\x04\x81`\x02\x81V[a\x05\x0Ba\x17\xCAV[a\x04\x81a\x06\xAA6`\x04aY\x93V[a\x18RV[a\x05\x0Ba\x06\xBD6`\x04aW\xD6V[a\x19dV[a\x05\xDFg\x01cEx]\x8A\0\0\x81V[a\x04\x81a\x06\xDF6`\x04aX0V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x05\x0Ba\x07\x086`\x04aYjV[a\x19\xF7V[a\x04\x81`\x12T\x81V[a\x05\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\rTa\x05\xDF\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\x81a\x07e6`\x04aX0V[a\x1A\xCEV[a\x07ra\x1A\xECV[`@Qa\x04\x8B\x91\x90aY\xE5V[a\x05\x0Ba\x07\x8D6`\x04aX0V[a\x1BMV[`\x07Ta\x05\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x81a\x07\xB36`\x04aY\x93V[a\x1C\xB5V[a\x04\xADa\x1D\xBBV[a\x04\xE0a\x07\xCE6`\x04aX0V[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\xA5a\x07\xF16`\x04aW\xD6V[a\x1D\xCAV[a\x05\x0Ba\x08\x046`\x04aYjV[a\x1D\xF4V[a\x05\x0Ba\x08\x176`\x04aW\xD6V[a\x1E\xBAV[`\rTa\x05\xA5\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08Wa\x08B6`\x04aX0V[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x04\x8B\x91\x90aZHV[a\x04\x81`\x15T\x81V[a\x04\xE0a\x08{6`\x04aX\x04V[a\x1FPV[a\x04\x81a\x1F\xD6V[a\x04\xE0a\x08\x966`\x04aX\x04V[a!\x12V[a\x05\x0Ba\x08\xA96`\x04aX0V[a! V[a\x04\x81a\x08\xBC6`\x04aW\xD6V[a!\xB3V[a\x04\x81a\x08\xCF6`\x04aZbV[a!\xE0V[a\x05\x0Ba\x08\xE26`\x04aYjV[a#\0V[a\x04\x81a\x08\xF56`\x04aZbV[a#\xDCV[a\x05\x0Ba\t\x086`\x04aW\xD6V[a%\x05V[a\x05\xDFg\x06\xF0[Y\xD3\xB2\0\0\x81V[`\x14Ta\x04\xE0\x90`\xFF\x16\x81V[a\x05\x0Ba\t76`\x04aW\xD6V[a%pV[a\x04\x81`\x17T\x81V[a\x04\x81a\tS6`\x04aX0V[a&\x9FV[a\x04\x81a\tf6`\x04aW\xD6V[a&\xC4V[a\x05\x0Ba&\xD7V[a\x04\x81a\t\x816`\x04aX0V[a'IV[a\x05\x0Ba\t\x946`\x04aZ\xB3V[a'VV[a\x04\x81a\t\xA76`\x04aX0V[a(\xBAV[a\x04\x81a\t\xBA6`\x04a[$V[a(\xC7V[a\x05\x0Ba\t\xCD6`\x04aW\xD6V[a(\xF2V[a\x05\x0Ba)]V[`\rTa\t\xE7\x90`\xFF\x16\x81V[`@Qa\x04\x8B\x91\x90a[bV[`\x0ET`\x0FT`\x10T`\x11Ta\n<\x93\x92`\x01`\x01`@\x1B\x03\x80\x82\x16\x93`\x01`@\x1B\x83\x04\x82\x16\x93`\x01`\x80\x1B\x84\x04\x83\x16\x93`\x01`\xC0\x1B\x90\x04\x90\x92\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x87V[`@\x80Q\x97\x88R`\x01`\x01`@\x1B\x03\x96\x87\x16` \x89\x01R\x94\x86\x16\x94\x87\x01\x94\x90\x94R\x91\x84\x16``\x86\x01R\x90\x92\x16`\x80\x84\x01R`\xA0\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01a\x04\x8BV[a\x04\x81`\x13T\x81V[a\x05\xDFg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x04\x81a\n\xB16`\x04aW\xD6V[a,\xCDV[a\x05Q` \x81V[a\x04\x81a\n\xCC6`\x04aX0V[`\x16` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x0Ba\n\xEC6`\x04a[oV[a,\xF5V[a\x05\x0Ba\n\xFF6`\x04aX0V[a.\xD6V[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B#Wa\x0B#a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BLW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BiWa\x0Bia[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x92W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0C@W`\0`\t\x82\x81T\x81\x10a\x0B\xB5Wa\x0B\xB5a[\xB7V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\x0B\xD5\x81a0wV[\x84\x83\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x0C\x10\x81a18V[\x83\x83\x81Q\x81\x10a\x0C\"Wa\x0C\"a[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x0C8\x81a[\xE3V[\x91PPa\x0B\x98V[P`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCD\x91\x90a[\xFCV[`@Qc\xB33\xA1u`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB33\xA1u\x90a\r \x90\x86\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a\\\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ra\x91\x90a\\\xABV[\x94PPPPP\x90V[```\x03\x80Ta\ry\x90a\\\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xA5\x90a\\\xC4V[\x80\x15a\r\xF2W\x80`\x1F\x10a\r\xC7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x0E\x0F\x82a\x0E\na\x0B\x04V[a2\x85V[\x92\x91PPV[`\x003a\x0E#\x81\x85\x85a38V[P`\x01\x93\x92PPPV[`\0\x80a\x0E8a\x0B\x04V[\x90P`\0a\x0EE\x82a4\\V[\x90Pa\x0EZ\x84a\x0EU\x83\x85a\\\xF8V[a4\xC2V[\x94\x93PPPPV[`\x14T`\xFF\x16\x15a\x0E\x86W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x14\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01[`@Q\x80\x91\x03\x90\xA1V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x90`\0\x90\xA3PV[`\x003a\x0F\x83\x85\x82\x85a5oV[a\x0F\x8E\x85\x85\x85a5\xE9V[`\x01\x91PP[\x93\x92PPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\rT`@Q\x7F\x196Y'\xBFR\xF7\xE9V\xD3v\xB3Td\x02\xDD\xA3\x82|\x06,ziBC\x1A%]!)G\xE8\x91a\x0F\xFE\x91`\xFF\x90\x91\x16\x90\x84\x90a]1V[`@Q\x80\x91\x03\x90\xA1`\r\x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x10$Wa\x10$aZ2V[\x02\x17\x90UPPV[`\0a\x106a7\xC2V[\x90P\x90V[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a\x10\x8CW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\x01\x14a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16a\x10\xF7W`@Qc\x1E\xC7\xBC\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a\x11\x01a\x0B\x04V[\x90P`\0a\x11\x0E`\x02T\x90V[\x90Pa\x11\x1B\x89\x880a8\xE9V[`\0a\x11&\x8Aa0wV[\x90P`\0a\x113\x8Aa0wV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x11TW\x88a\x11cV[a\x11c\x82\x82\x8B\x8B\x8B\x8B0a9\xDEV[\x94Pa\x11o\x8A\x86a<\rV[`\0a\x11\x9A`\x17Tg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x8A\x91\x90a\\\xF8V[\x86\x90g\r\xE0\xB6\xB3\xA7d\0\0a=LV[\x90P`\0a\x11\xC7`\x17Tg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB7\x91\x90a]{V[\x87\x90g\r\xE0\xB6\xB3\xA7d\0\0a=zV[\x90Pa\x11\xD1a\x0B\x04V[\x95P\x80\x86\x11\x80a\x11\xE0WP\x81\x86\x10[\x15a\x12\x0FW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x0E\xB0V[`\x02T\x85\x14a\x12?W`\x02T`@Qc+@\x14Y`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x01a\x0E\xB0V[\x8B`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xA7\xA9\xA6\xE2\\\xD0\xBB\xBF\xA8\x0C\xE0\xC7dna\xEE^\x05Q\xB3\xFD\xAA\xFF\x06B\xE6\xF6\xAD\xCCr\xE2\x8D\x8A`@Qa\x12\x8D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PP`\x01`\x08UP\x92\x99\x98PPPPPPPPPV[`\x003a\x0E#\x81\x85\x85a\x12\xBE\x83\x83a(\xC7V[a\x12\xC8\x91\x90a]{V[a38V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x06\xF0[Y\xD3\xB2\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x13)W`@Qc.\x15(m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\xC0\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7F'\xDD:\xE8\xFF\x7F^=w\xAEh\xED6\xDCx\r*\xB4\nO\xFD\xDA\x18\xD8\x05\xCBA\xEE\xA3\x9C(\x94\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x14T`\0\x90`\xFF\x16\x15a\x13\xB5WP`\0\x91\x90PV[`\x13T`\x12T`\0\x19\x82\x14\x80\x15a\x13\xCDWP`\0\x19\x81\x14[\x15a\x13\xDDWP`\0\x19\x93\x92PPPV[`\0a\x13\xE7a\x0B\x04V[\x90P`\0a\x14\x13a\x14\r\x87`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[\x83a2\x85V[\x90P`\0a\x14!\x85\x83a=\x99V[\x90P`\0a\x14/\x85\x85a=\x99V[\x90Pa\x14;\x82\x82a=\xB3V[\x98\x97PPPPPPPPV[`\0\x80a\x14Ra\x0B\x04V[\x90P`\0a\x14_\x82a4\\V[\x90Pa\x0EZ\x84a\x0E\n\x83\x85a\\\xF8V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a\x14\xBDW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT` \x11a\x14\xE3W`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x15'W`@Qci\x9Ff\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x15a\x15lW`@Qc\n\xF6}\x91`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[a\x15x`\t\x83\x83a=\xC2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\n` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xCF\xF5\xC8\xA0\x88\x84\xD2\xFA\xD99\xA9\xE0\xF0\xBFr\x9AO\x9A\xF6\x11\x1AW;M\x1F\x119l\x87\x11dm\x90a\x15\xCD\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x01cEx]\x8A\0\0\x81\x11\x15a\x16=W`@Qc\x02\xD2\xA9\x0F`\xE5\x1B\x81R`\x04\x81\x01\x82\x90Rg\x01cEx]\x8A\0\0`$\x82\x01R`D\x01a\x0E\xB0V[`\x17\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xDFK\xE3;.\x9E=\xD4\xD9\xE0\xE8VE\xAE\xA4(IJ\x06D\xA7,Q\xD6\xA1Z\xED\xAEkf\xA3\xFF\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0`\t\x82\x81T\x81\x10a\x16\xC2Wa\x16\xC2a[\xB7V[`\0\x91\x82R` \x82 \x01T`\t\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x85\x90\x81\x10a\x16\xEFWa\x16\xEFa[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81\x81`\t\x86\x81T\x81\x10a\x17$Wa\x17$a[\xB7V[\x90`\0R` `\0 \x01`\0`\t\x87\x81T\x81\x10a\x17CWa\x17Ca[\xB7V[`\0\x91\x82R` \x91\x82\x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U\x82T\x94\x84\x16a\x01\0\x92\x90\x92\n\x91\x82\x02\x91\x84\x02\x19\x90\x94\x16\x17\x90U`@\x80Q\x87\x81R\x92\x83\x01\x86\x90R\x83\x82\x16\x92\x91\x85\x16\x91\x7F<\xAEOW\x96\xF9\xD1\x9F\xA1\xDC\xFA\xAC\xAEZ\x98\x11\xC0\x08\xF6\xF5\x17\xA3\xECh\x9D\x90=oi\x9EIU\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16a\x18\x17W`@Qc\xECqe\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x80T`\xFF\x19\x16\x90U`@Q`\0\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01a\x0E\xF5V[`\0`\x08T`\x01\x14a\x18vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0a\x18\x85a\x0B\x04V[\x90Pa\x18\x90\x81a?;V[a\x18\x9A\x84\x82a?xV[\x91P\x81`\0\x03a\x18\xBDW`@QcBo\x157`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC8\x84\x83\x85a?\x97V[a\x18\xFD`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x87a@\xCAV[a\x19\x07\x83\x83aA5V[`@\x80Q\x85\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a\x19X\x84\x83\x85aB V[P`\x01`\x08U\x92\x91PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x11\x15a\x19\xB6W`@Qc\x01\x9B^1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FQ:\xC1\x9C\xBB\xAA\xADNE\x0Cs.\xD3v5\x17\x8B}\x83\xBF\x8E\x84\xA9@\xFF\xE7\xE0R\xC9\xC7\xCA\xA2\x91\x01`@Q\x80\x91\x03\x90\xA1`\x10UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x02\xC6\x8A\xF0\xBB\x14\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x1ASW`@Qc.\x15(m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x80\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7FD\xAD\xA2a\xFF\\\x9A\xAC\xBF\x8D\x06\x87)G\x99\xC8\xE3\xE0\x81\x0E\xCC\x1E\xAF\xD9t\x19\xDF\xC3\x1D\xB5\xD5#\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x81 Ta\x0E\x0FV[```\t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\r\xF2W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B&WPPPPP\x90P\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16a\x1B\xBBW`@Qc\x1E\xC7\xBC\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a\x1B\xC6\x82a0wV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CMW`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`$\x82\x01R`D\x01a\x0E\xB0V[`\rT`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92a\x01\0\x90\x04\x16\x90\x7F\x7F\x83_\xBFC\xC4\xD0Z\x9CN\xA5\xCA\xFA\t\xFB\xB3\xB00}\x12\x95l\xEE\xDC\xE4?\xD7\xA39\xE5\x04n\x90`\0\x90\xA3P`\r\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`\x08T`\x01\x14a\x1C\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0a\x1C\xE8a\x0B\x04V[\x90Pa\x1C\xF3\x81a?;V[a\x1C\xFD\x84\x82aBYV[\x91P\x81`\0\x03a\x1D W`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D+\x82\x85\x85a?\x97V[a\x1D``\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x85a@\xCAV[a\x1Dj\x83\x85aA5V[`@\x80Q\x83\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a\x19X\x82\x85\x85aB V[```\x04\x80Ta\ry\x90a\\\xC4V[`\t\x81\x81T\x81\x10a\x1D\xDAW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x1EPW`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x01`@\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xC7\x93\x99\t?s\xF1\xFF\x8C/\x82y\xAA\x9E\x88\x03\xB6\x94\xC6\0\x16\xC3D\xEC\xFC*\x89\xFDw\xC0\x04\xDB\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x08\x81\x10\x80a\x1E\xF4WPa\x1C \x81\x11[\x15a\x1F\x12W`@Qc:`#?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x15\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\"\x7F\xF5\xC6\xB5\xFF\xB3\x95#k\t\xFD\x1BG+\xB1(\xB3n\xAA\x17Uf3\xFE\xEF\xE2\x8E\x94A\x1F$\x91\x01a\x16wV[`\x003\x81a\x1F^\x82\x86a(\xC7V[\x90P\x83\x81\x10\x15a\x1F\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[a\x1F\xCB\x82\x86\x86\x84\x03a38V[P`\x01\x94\x93PPPPV[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xF5Wa\x1F\xF5a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a ;Wa ;a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0C@W`\0`\t\x82\x81T\x81\x10a \x87Wa \x87a[\xB7V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90Pa \xA7\x81a0wV[\x84\x83\x81Q\x81\x10a \xB9Wa \xB9a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa \xE2\x81aBxV[\x83\x83\x81Q\x81\x10a \xF4Wa \xF4a[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a!\n\x81a[\xE3V[\x91PPa jV[`\x003a\x0E#\x81\x85\x85a5\xE9V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a!JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x11T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FQ\xDB\xB5\xA6[\xB2'7\x86\x1Ac\xEC\x12\xBAl\xE7\x8A\x98c\x1E\x94\x04\xB0Vz.\xAFz\x06\xFCTM\x91\x01`@Q\x80\x91\x03\x90\xA1`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80a!\xBEa\x0B\x04V[\x90P`\0a!\xCB\x82a4\\V[\x90Pa\x0EZ\x84a!\xDB\x83\x85a\\\xF8V[aBYV[`\0`\x08T`\x01\x14a\"\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0\x80\x80\x80\x80a\"\x17aB\xF6V[\x94P\x94P\x94P\x94P\x94Pa\"*\x85a?;V[a\"4\x89\x86a4\xC2V[\x95Pa\"B\x89\x87\x8A\x8AaF\x93V[a\"L\x87\x87aF\xC2V[`\0a\"W`\x02T\x90V[\x90Pa\"c\x88\x88aF\xDDV[`@\x80Q\x8B\x81R` \x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x92\x90\x8C\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\0`\rT`\xFF\x16`\x01\x81\x11\x15a\"\xC7Wa\"\xC7aZ2V[\x14a\"\xDFWa\"\xDA\x87\x82\x8B\x88\x87\x87aH7V[a\"\xEDV[a\"\xED\x8A\x8A\x87\x87\x87\x87aIOV[PP`\x01`\x08UP\x92\x96\x95PPPPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a#*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a#\\W`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`@\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7F\xB5\xCC\x99J&\n\x85\xA4-e\x88f\x82!W\x1A\xE0\xA1O\n(\xF9\xE4\x81zQ\x95&!\x02\xC8h\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`\x08T`\x01\x14a$\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0\x80\x80\x80\x80a$\x13aB\xF6V[\x94P\x94P\x94P\x94P\x94Pa$&\x85a?;V[a$0\x87\x8AaF\xC2V[a$:\x89\x86a2\x85V[\x95P\x85`\0\x03a$]W`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$i\x86\x8A\x8A\x8AaF\x93V[`\0a$t`\x02T\x90V[\x90Pa$\x80\x88\x8BaF\xDDV[`@\x80Q\x88\x81R` \x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x92\x90\x8C\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\0`\rT`\xFF\x16`\x01\x81\x11\x15a$\xE4Wa$\xE4aZ2V[\x14a$\xF7Wa\"\xDA\x8A\x82\x8B\x88\x87\x87aH7V[a\"\xDA\x87\x8A\x87\x87\x87\x87aIOV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x13T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCF\xB5\xA4T\xB8\xAA}\xC0N\xCB[\xC1A\x0B*W\x96\x9C\xA1\xD6\x7Ff\xD5e\x19o`\xC6\xF9\x97T\x04\x91\x01`@Q\x80\x91\x03\x90\xA1`\x13UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0`\t\x82\x81T\x81\x10a%\xAFWa%\xAFa[\xB7V[`\0\x91\x82R` \x82 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91Pa%\xCE\x82a18V[\x90P\x80\x15a&\x01W`@Qc\x84Th\x9F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x0E\xB0V[`\rT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x90\x83\x16\x03a&5W`@Qctw\x95\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&@`\t\x84aL\x9EV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\n` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x03\xC7\x8EN \xA7+\x1FWzc\xB5E$\xC6\x84\xCEs\xD2\xE4\x14\x97\x0E\xE6\x06\xBB\x0Bz\x90\x04\xAA\x9C\x90a&\x92\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`\0\x80a&\xAB\x83a\x13\x9FV[\x90P`\0\x19\x81\x14a\x0E\x0FWa&\xBF\x81a&\xC4V[a\x0F\x94V[`\0a\x0E\x0F\x82a&\xD2a\x0B\x04V[a?xV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0a'\x0Ba\x0B\x04V[`\x0E\x81\x90U`@Q\x81\x81R\x90\x91P\x7F\x87Ua\xDD\xC3B\xB1)\x81\x10=\x85\xBE\x8D\xB67Z\x88\x98-L\xF5\x8DA\x1E8qZ\xE4h30\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0a\x0E\x0F\x82`\0aM\xB8V[\x83B\x11\x15a'\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a'\xD5\x8CaP\x01V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a(0\x82aP'V[\x90P`\0a(@\x82\x87\x87\x87aPuV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x0E\xB0V[a(\xAE\x8A\x8A\x8Aa38V[PPPPPPPPPPV[`\0a\x0E\x0F\x82`\x01aM\xB8V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x12T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x1F!C-\xD7\xB8\xEA\xD6M.|\x06\xA7K\xAF\x13x;-/qS\xF0\x99\xE2\xC4\xCA\xBC<]\xBE\xC6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x12UV[`\x08T`\x01\x14a)\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\x11T`\x01`\x01`\xA0\x1B\x03\x16\x80a)\xAEW`@Qc\xDC\x13a\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a)\xB8a\x0B\x04V[\x90Pa)\xC3\x81a?;V[0`\0\x90\x81R` \x81\x90R`@\x81 T`\x0FT\x90\x91\x90a)\xED\x90\x83\x90`\x01`\x01`@\x1B\x03\x16aP\x9DV[`\rT\x90\x91P`\0\x90a*\x10\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16Ba\\\xF8V[`\x0FT\x90\x91P`\0\x90c\x01\xE13\x80\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a*C\x85\x89a]\x8EV[a*M\x91\x90a]\x8EV[a*W\x91\x90a]\xADV[a*a\x91\x90a]\xADV[\x90P`\0a*wa*r\x83\x88a?xV[aP\xB2V[\x90Pa*\x830\x82aA5V[a*\x8D\x81\x86a]{V[`\x0FT\x90\x95Pa*\xAE\x90\x82\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aP\x9DV[a*\xB8\x90\x85a]{V[\x93P\x83\x15a*\xD8Wa*\xCB0\x88\x86a5\xE9V[a*\xD5\x84\x86a\\\xF8V[\x94P[`\r\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16Bc\xFF\xFF\xFF\xFF\x16`\x01`\xA8\x1B\x02\x17\x90U`\0a+\x06\x86\x88a2\x85V[\x90P\x80\x15a,\x85W\x80`\x0E`\0\x01`\0\x82\x82Ta+#\x91\x90a\\\xF8V[\x90\x91UPa+3\x90P0\x87aF\xDDV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xBF\x91\x90a[\xFCV[\x90Pa+\xF5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82\x84aP\xE1V[`\x10T`@Qc\x1F\xFB\xE7\xF9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R`D\x81\x01\x84\x90R\x90\x82\x16\x90c\x1F\xFB\xE7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x7FW=`\0\x80>=`\0\xFD[PPPPP[`@\x80Q\x87\x81R` \x81\x01\x83\x90R\x7F\x15\xE3\xE2\xA7jh9\xC2D\xC1\xED\n\x82\x1C#<\xE8\xAFU-\xFF\xCB\x85`\x89\xEA\xE6\xCB\xBB\xB7\x1E\xA6\x91\x01`@Q\x80\x91\x03\x90\xA1PP`\x01`\x08UPPPPPPV[`\0\x80a,\xD8a\x0B\x04V[\x90P`\0a,\xE5\x82a4\\V[\x90Pa\x0EZ\x84a&\xD2\x83\x85a\\\xF8V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a-\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x90\x93R\x92 \x80T\x84\x93\x91\x92\x16\x90\x83`\x02\x81\x11\x15a-jWa-jaZ2V[\x02\x17\x90UP`\0a-z\x83a0wV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x06\x91\x90a[\xFCV[`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.r\x91\x90a]\xCFV[a.\x9AW`@Qc~\xB5n\xDB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`@Q`\x01\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xD6\0\xB94\x86\x03\xC6\xDE\xFF4\xB4\xE0\xB2\x8B`\xE1\xC8\x03l\x80gA\xB9\xE6\xD9\x002\xE7\xF3}\xD2\x7F\x90` \x01a&\x92V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a/\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a/$W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT` \x11a/JW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a/\x8EW`@Qci\x9Ff\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x15a/\xD3W`@Qc\n\xF6}\x91`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\t\x80T`\x01\x80\x82\x01\x83U\x7Fn\x15@\x17\x1Bl\x0C\x96\x0Bq\xA7\x02\r\x9F`\x07\x7Fj\xF91\xA8\xBB\xF5\x90\xDA\x02#\xDA\xCFu\xC7\xAF\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x81\x81R`\n` R`@\x90 \x80T`\xFF\x19\x16\x83\x17\x90U\x91T\x7F\xCF\xF5\xC8\xA0\x88\x84\xD2\xFA\xD99\xA9\xE0\xF0\xBFr\x9AO\x9A\xF6\x11\x1AW;M\x1F\x119l\x87\x11dm\x91a0c\x91a\\\xF8V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a0\xA7Wa0\xA7aZ2V[\x14\x80a0\xC4WP`\x02\x81`\x02\x81\x11\x15a0\xC2Wa0\xC2aZ2V[\x14[\x15a1+W\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a[\xFCV[P\x90\x91\x90PV[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a1hWa1haZ2V[\x14\x80a1\x85WP`\x02\x81`\x02\x81\x11\x15a1\x83Wa1\x83aZ2V[\x14[\x15a2YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cL\xDA\xD5\x06\x90\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xFA\x91\x90a\\\xABV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\x18\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a25W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a\\\xABV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01a2\x18V[`\0\x80a2\x91`\x02T\x90V[\x90P\x80\x15a2\xA9Wa2\xA4\x84\x84\x83a=zV[a\x0EZV[a\x0EZ`\x12\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a30\x91\x90a]\xF1V[\x86\x91\x90aQ\xF6V[`\x01`\x01`\xA0\x1B\x03\x83\x16a3\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a3\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x0FT`\0\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80\x15\x80a4}WP\x82\x15[\x15a4\x8BWP`\0\x92\x91PPV[`\x0ET\x80\x84\x11\x15a4\xBBW`\0a4\xA2\x82\x86a\\\xF8V[\x90Pa4\xB7\x81`\x01`\x01`@\x1B\x03\x85\x16aP\x9DV[\x93PP[PP\x91\x90PV[`\0\x80a4\xCE`\x02T\x90V[\x90P\x80\x15a4\xE1Wa2\xA4\x84\x82\x85a=LV[a\x0EZ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5f\x91\x90a]\xF1V[\x85\x90`\x12aQ\xF6V[`\0a5{\x84\x84a(\xC7V[\x90P`\0\x19\x81\x14a5\xE3W\x81\x81\x10\x15a5\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[a5\xE3\x84\x84\x84\x84\x03a38V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a6MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a6\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[a6\xBA\x83\x83\x83aR_V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a72W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a7i\x90\x84\x90a]{V[\x92PP\x81\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa7\xB5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a5\xE3V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a8\x1BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a8EWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a9\x19Wa9\x19aZ2V[\x14\x80a96WP`\x02\x81`\x02\x81\x11\x15a94Wa94aZ2V[\x14[\x15a9\xBAW`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R0`D\x83\x01R\x85\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a9\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xB4\x91\x90a\\\xABV[Pa5\xE3V[`\x01`\x01`\xA0\x1B\x03\x82\x160\x14a5\xE3Wa5\xE3`\x01`\x01`\xA0\x1B\x03\x85\x16\x83\x85aRhV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x81\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:M\x91\x90a\\\xABV[a:W\x91\x90a\\\xF8V[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x01`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xE6\x91\x90a[\xFCV[\x90Pa:\xFC`\x01`\x01`\xA0\x1B\x03\x8B\x16\x82\x8AaP\xE1V[\x80`\x01`\x01`\xA0\x1B\x03\x16c-\xFE\x16\x90\x88\x88\x88\x88\x8F\x8F`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;2\x96\x95\x94\x93\x92\x91\x90a^\x0EV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a;QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;u\x91\x90a\\\xABV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x93P\x82\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xE2\x91\x90a\\\xABV[\x14a<\0W`@Qc\n\x92\x13\xB5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a<=Wa<=aZ2V[\x14\x80a<ZWP`\x02\x81`\x02\x81\x11\x15a<XWa<XaZ2V[\x14[\x15a=GWa<\xD6\x83\x83\x85`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xC6\x91\x90a[\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90aP\xE1V[`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a=#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE3\x91\x90a\\\xABV[PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a=dW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a=\x92W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x81\x83\x11a=\xA9W`\0a\x0F\x94V[a\x0F\x94\x82\x84a\\\xF8V[`\0\x81\x83\x10a1+W\x81a\x0F\x94V[\x82T\x80\x15a?\x08W\x83\x80a=\xD7`\x01\x84a\\\xF8V[\x81T\x81\x10a=\xE7Wa=\xE7a[\xB7V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x83T`\x01\x81\x81\x01\x86U\x94\x84R\x91\x83 \x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x90a>/\x90\x83a\\\xF8V[\x90P[\x83\x81\x11\x15a>\xC1W\x84a>F`\x01\x83a\\\xF8V[\x81T\x81\x10a>VWa>Va[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81T\x81\x10a>\x86Wa>\x86a[\xB7V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a>\xB9\x81a^kV[\x91PPa>2V[P\x81\x84\x84\x81T\x81\x10a>\xD5Wa>\xD5a[\xB7V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa5\xE3V[\x83T`\x01\x81\x01\x85U`\0\x85\x81R` \x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90UPPPPV[`\0a?F\x82a4\\V[\x90P\x80\x15a?tW`\0a?]a*r\x83\x85a?xV[\x90P\x80\x15a=GW`\x0E\x83\x90Ua=G0\x82aA5V[PPV[`\0\x80a?\x84`\x02T\x90V[\x90P\x80\x15a4\xE1Wa2\xA4\x84\x82\x85a=zV[`\x14T`\xFF\x16\x15a?\xBBW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a@rW`@QcUQ\xE1\xB5`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cUQ\xE1\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@S\x91\x90a]\xCFV[a@rW`@Qc4\x87\x1F%`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a@}\x82a\x13\x9FV[\x90P\x80\x84\x11\x15a@\xAAW`@Qc-!\xEB\x87`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x0E\xB0V[\x83`\x0E`\0\x01`\0\x82\x82Ta@\xBF\x91\x90a]{V[\x90\x91UPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra5\xE3\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaR\x98V[`\x01`\x01`\xA0\x1B\x03\x82\x16aA\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x0E\xB0V[aA\x97`\0\x83\x83aR_V[\x80`\x02`\0\x82\x82TaA\xA9\x91\x90a]{V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90aA\xD6\x90\x84\x90a]{V[\x90\x91UPP`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[`\rTaB;\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a<\rV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x16` R`@\x90 C\x90UPPV[`\0\x80aBe`\x02T\x90V[\x90P\x80\x15a2\xA9Wa2\xA4\x84\x84\x83a=LV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15aB\xA8WaB\xA8aZ2V[\x14\x80aB\xC5WP`\x02\x81`\x02\x81\x11\x15aB\xC3WaB\xC3aZ2V[\x14[\x15a2YW`@Qc\xCE\x96\xCBw`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xCE\x96\xCBw\x90`$\x01a2\x18V[`\tT`\0\x90``\x90\x81\x90\x81\x90\x81\x90\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x1DWaC\x1Da[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aCaWaCaa[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xA5WaC\xA5a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xE9WaC\xE9a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aD-WaD-a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aDVW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81\x81\x10\x15aEgW`\0`\t\x82\x81T\x81\x10aDyWaDya[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80\x87\x83\x81Q\x81\x10aD\xACWaD\xACa[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPaD\xD5\x81a0wV[\x86\x83\x81Q\x81\x10aD\xE7WaD\xE7a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPaE\x10\x81a18V[\x85\x83\x81Q\x81\x10aE\"WaE\"a[\xB7V[` \x02` \x01\x01\x81\x81RPPaE7\x81aBxV[\x84\x83\x81Q\x81\x10aEIWaEIa[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80aE_\x81a[\xE3V[\x91PPaD\\V[P`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xF4\x91\x90a[\xFCV[`@Qc\xB33\xA1u`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB33\xA1u\x90aFG\x90\x88\x90\x88\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a\\\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x88\x91\x90a\\\xABV[\x96PPP\x90\x91\x92\x93\x94V[aF\x9C\x81aSjV[`\x0ET\x80\x85\x11aF\xB5WaF\xB0\x85\x82a\\\xF8V[aF\xB8V[`\0[`\x0EUPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a?tWa?t\x823\x83a5oV[`\x01`\x01`\xA0\x1B\x03\x82\x16aG=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[aGI\x82`\0\x83aR_V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15aG\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90aG\xEC\x90\x84\x90a\\\xF8V[\x90\x91UPP`@Q\x82\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0[\x83Q\x81\x10\x15aIFW`\0\x84\x82\x81Q\x81\x10aHWWaHWa[\xB7V[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10aHuWaHua[\xB7V[` \x02` \x01\x01Q\x90P\x80`\0\x03aH\x8EWPPaI4V[`\0aH\x9B\x82\x8B\x8Ba=zV[\x90P\x84\x84\x81Q\x81\x10aH\xAFWaH\xAFa[\xB7V[` \x02` \x01\x01Q\x81\x11\x15aH\xE2W`@Qc\xF2\x01\x8Fo`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[aH\xED\x83\x82\x8Aa8\xE9V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x94\xAD7\xA0\xF8\x93\\\n\xCD\xD1\xCC\xFF\xE9\xAA\xD2\x96\xBF\xC8\xD0\xA1\xDD-\x0C\xFB\xACy@\\_\x86\xED\x82\x82`@QaI(\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPP[\x80aI>\x81a[\xE3V[\x91PPaH:V[PPPPPPPV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xDB\x91\x90a[\xFCV[\x90P`\0[\x85Q\x81\x10\x15aL{W\x83\x81\x81Q\x81\x10aI\xFBWaI\xFBa[\xB7V[` \x02` \x01\x01Q`\0\x03\x15aLiW`\0\x85\x82\x81Q\x81\x10aJ\x1FWaJ\x1Fa[\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x88\x91\x90a]\xF1V[aJ\x93\x90`\na_fV[\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c\xBA\xAAa\xBE\x88\x85\x81Q\x81\x10aJ\xB8WaJ\xB8a[\xB7V[` \x02` \x01\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x13\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKT\x91\x90a\\\xABV[\x90P`\0aK\x86\x82\x84\x88\x87\x81Q\x81\x10aKoWaKoa[\xB7V[` \x02` \x01\x01Qa=z\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x8B\x82\x11\x15aK\xA8WaK\x9D\x8C\x85\x85a=zV[\x90P`\0\x9BPaK\xD3V[\x86\x85\x81Q\x81\x10aK\xBAWaK\xBAa[\xB7V[` \x02` \x01\x01Q\x90P\x81\x8CaK\xD0\x91\x90a\\\xF8V[\x9BP[aK\xF7\x8A\x86\x81Q\x81\x10aK\xE8WaK\xE8a[\xB7V[` \x02` \x01\x01Q\x82\x8Da8\xE9V[\x89\x85\x81Q\x81\x10aL\tWaL\ta[\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\x94\xAD7\xA0\xF8\x93\\\n\xCD\xD1\xCC\xFF\xE9\xAA\xD2\x96\xBF\xC8\xD0\xA1\xDD-\x0C\xFB\xACy@\\_\x86\xED\x82\x82`@QaLK\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x8B`\0\x03aLdWPPPPaL{V[PPPP[\x80aLs\x81a[\xE3V[\x91PPaI\xE0V[P\x86\x15aIFW`@Qc\xCC^\xA3\x9B`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x0E\xB0V[\x81T\x80\x82\x10aL\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x0E\xB0V[\x81[aL\xF2`\x01\x83a\\\xF8V[\x81\x10\x15aM\x80W\x83aM\x05\x82`\x01a]{V[\x81T\x81\x10aM\x15WaM\x15a[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81T\x81\x10aMEWaMEa[\xB7V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aMx\x81a[\xE3V[\x91PPaL\xE7V[P\x82\x80T\x80aM\x91WaM\x91a_uV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x16` R`@\x81 T\x80\x15aM\xFEW`\0`\x15T\x82aM\xE7\x91\x90a]{V[\x90PC\x81\x11\x15aM\xFCW`\0\x92PPPa\x0E\x0FV[P[`\0aN\x08a\x0B\x04V[\x90P`\0aN\x15\x82a4\\V[\x90P`\0aNEaN;\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x0E\n\x84\x86a\\\xF8V[\x90P`\0`\rT`\xFF\x16`\x01\x81\x11\x15aN`WaN`aZ2V[\x03aN\x89W`\0aNoa\x1F\xD6V[\x90P\x80\x82\x11\x15aN\x7FW\x80aN\x81V[\x81[\x95PPaO\xE0V[`\0\x80aN\x94aB\xF6V[\x94P\x94PPPP`\0aN\xA6`\x02T\x90V[\x90P`\0aN\xC9\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0[\x84Q\x81\x10\x15aO\x92W\x85\x81\x81Q\x81\x10aN\xF2WaN\xF2a[\xB7V[` \x02` \x01\x01Q`\0\x03\x15aO\x80W\x84\x81\x81Q\x81\x10aO\x14WaO\x14a[\xB7V[` \x02` \x01\x01Q`\0\x03aO6W`\0\x9APPPPPPPPPPPa\x0E\x0FV[`\0aOpg\r\xE0\xB6\xB3\xA7d\0\0\x88\x84\x81Q\x81\x10aOVWaOVa[\xB7V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10aKoWaKoa[\xB7V[\x90P\x82\x81\x10\x15aO~W\x80\x92P[P[\x80aO\x8A\x81a[\xE3V[\x91PPaN\xD7V[P`\0aO\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x86a=zV[\x90P\x81\x81\x11\x15aO\xD5WaO\xD0\x82g\r\xE0\xB6\xB3\xA7d\0\0aO\xC9\x8B\x8Da\\\xF8V[\x91\x90a=zV[aO\xD7V[\x86[\x9APPPPPPP[\x85\x15aO\xF7WaO\xF4\x85a&\xD2\x84\x86a\\\xF8V[\x94P[PPPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01\x81\x01\x82U\x90a12V[`\0a\x0E\x0FaP4a7\xC2V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0aP\x86\x87\x87\x87\x87aS\xC5V[\x91P\x91PaP\x93\x81aT\xB2V[P\x95\x94PPPPPV[`\0a\x0F\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a=zV[`\0\x80aP\xBE`\x02T\x90V[\x90P\x82\x81\x11\x15a12W`\0aP\xD4\x84\x83a\\\xF8V[\x90Pa\x0EZ\x84\x83\x83a=LV[\x80\x15\x80aQ[WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQY\x91\x90a\\\xABV[\x15[aQ\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra=G\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a@\xFEV[`\0\x81`\xFF\x16\x83`\xFF\x16\x03aR\x0CWP\x82a\x0F\x94V[\x81`\xFF\x16\x83`\xFF\x16\x10\x15aR@WaR$\x83\x83a_\x8BV[aR/\x90`\na_fV[aR9\x90\x85a]\x8EV[\x90Pa\x0F\x94V[aRJ\x82\x84a_\x8BV[aRU\x90`\na_fV[aR9\x90\x85a]\xADV[a=G\x83aSjV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra=G\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a@\xFEV[`\0aR\xED\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aVk\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a=GW\x80\x80` \x01\x90Q\x81\x01\x90aS\x0B\x91\x90a]\xCFV[a=GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x16` R`@\x90 T\x80\x15a?tW`\0`\x15T\x82aS\x99\x91\x90a]{V[\x90PC\x81\x11\x15a=GW`@Qc\x06\xF8\xEE?`\xE2\x1B\x81R`\x04\x81\x01\x82\x90RC`$\x82\x01R`D\x01a\x0E\xB0V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aS\xFCWP`\0\x90P`\x03aT\xA9V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aT\x14WP\x84`\xFF\x16`\x1C\x14\x15[\x15aT%WP`\0\x90P`\x04aT\xA9V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aTyW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aT\xA2W`\0`\x01\x92P\x92PPaT\xA9V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15aT\xC6WaT\xC6aZ2V[\x03aT\xCEWPV[`\x01\x81`\x04\x81\x11\x15aT\xE2WaT\xE2aZ2V[\x03aU/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\x02\x81`\x04\x81\x11\x15aUCWaUCaZ2V[\x03aU\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0E\xB0V[`\x03\x81`\x04\x81\x11\x15aU\xA4WaU\xA4aZ2V[\x03aU\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x04\x81`\x04\x81\x11\x15aV\x10WaV\x10aZ2V[\x03aVhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[PV[``a\x0EZ\x84\x84`\0\x85\x85`\x01`\x01`\xA0\x1B\x03\x85\x16;aV\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaV\xE9\x91\x90a_\xA4V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aW&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aW+V[``\x91P[P\x91P\x91PaW;\x82\x82\x86aWFV[\x97\x96PPPPPPPV[``\x83\x15aWUWP\x81a\x0F\x94V[\x82Q\x15aWeW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x91\x90aW\xA3V[`\0[\x83\x81\x10\x15aW\x9AW\x81\x81\x01Q\x83\x82\x01R` \x01aW\x82V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaW\xC2\x81`@\x85\x01` \x87\x01aW\x7FV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aW\xE8W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aVhW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aX\x17W`\0\x80\xFD[\x825aX\"\x81aW\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15aXBW`\0\x80\xFD[\x815a\x0F\x94\x81aW\xEFV[`\0\x80`\0``\x84\x86\x03\x12\x15aXbW`\0\x80\xFD[\x835aXm\x81aW\xEFV[\x92P` \x84\x015aX}\x81aW\xEFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x02\x81\x10aVhW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\xADW`\0\x80\xFD[\x815a\x0F\x94\x81aX\x8EV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15aX\xD1W`\0\x80\xFD[\x865aX\xDC\x81aW\xEFV[\x95P` \x87\x015aX\xEC\x81aW\xEFV[\x94P`@\x87\x015\x93P``\x87\x015aY\x03\x81aX\x8EV[\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aY\x1FW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aY3W`\0\x80\xFD[\x815\x81\x81\x11\x15aYBW`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15aYTW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15aY|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aY\xA6W`\0\x80\xFD[\x825\x91P` \x83\x015aY\xB8\x81aW\xEFV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aY\xD6W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aZ&W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aZ\x01V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10aZ\\WaZ\\aZ2V[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aZwW`\0\x80\xFD[\x835\x92P` \x84\x015aZ\x89\x81aW\xEFV[\x91P`@\x84\x015aZ\x99\x81aW\xEFV[\x80\x91PP\x92P\x92P\x92V[`\xFF\x81\x16\x81\x14aVhW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aZ\xCEW`\0\x80\xFD[\x875aZ\xD9\x81aW\xEFV[\x96P` \x88\x015aZ\xE9\x81aW\xEFV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a[\x07\x81aZ\xA4V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a[7W`\0\x80\xFD[\x825a[B\x81aW\xEFV[\x91P` \x83\x015aY\xB8\x81aW\xEFV[`\x02\x81\x10aVhWaVhaZ2V[` \x81\x01aZ\\\x83a[RV[`\0\x80`@\x83\x85\x03\x12\x15a[\x82W`\0\x80\xFD[\x825a[\x8D\x81aW\xEFV[\x91P` \x83\x015`\x03\x81\x10aY\xB8W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a[\xF5Wa[\xF5a[\xCDV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\\\x0EW`\0\x80\xFD[\x81Qa\x0F\x94\x81aW\xEFV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\\[W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\\6V[PPP\x83\x81\x03\x82\x85\x01R\x85Q\x80\x82R\x86\x83\x01\x91\x83\x01\x90`\0[\x81\x81\x10\x15a\\\x90W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\\tV[PP`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x86\x01R\x92Pa\x0EZ\x91PPV[`\0` \x82\x84\x03\x12\x15a\\\xBDW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\\\xD8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a12WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`@\x81\x01a]>\x84a[RV[\x83\x82Ra]J\x83a[RV[\x82` \x83\x01R\x93\x92PPPV[` \x80\x82R`\n\x90\x82\x01RiREENTRANCY`\xB0\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a]\xA8Wa]\xA8a[\xCDV[P\x02\x90V[`\0\x82a]\xCAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a]\xE1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a^\x03W`\0\x80\xFD[\x81Qa\x0F\x94\x81aZ\xA4V[a^\x17\x87a[RV[\x86\x81R`\xA0` \x82\x01R\x84`\xA0\x82\x01R\x84\x86`\xC0\x83\x017`\0`\xC0\x86\x83\x01\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`@\x83\x01R\x92\x84\x16``\x82\x01R\x92\x16`\x80\x83\x01R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0\x81a^zWa^za[\xCDV[P`\0\x19\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a^\xBDW\x81`\0\x19\x04\x82\x11\x15a^\xA3Wa^\xA3a[\xCDV[\x80\x85\x16\x15a^\xB0W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a^\x87V[P\x92P\x92\x90PV[`\0\x82a^\xD4WP`\x01a\x0E\x0FV[\x81a^\xE1WP`\0a\x0E\x0FV[\x81`\x01\x81\x14a^\xF7W`\x02\x81\x14a_\x01Wa_\x1DV[`\x01\x91PPa\x0E\x0FV[`\xFF\x84\x11\x15a_\x12Wa_\x12a[\xCDV[PP`\x01\x82\x1Ba\x0E\x0FV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a_@WP\x81\x81\na\x0E\x0FV[a_J\x83\x83a^\x82V[\x80`\0\x19\x04\x82\x11\x15a_^Wa_^a[\xCDV[\x02\x93\x92PPPV[`\0a\x0F\x94`\xFF\x84\x16\x83a^\xC5V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[`\0\x82Qa_\xB6\x81\x84` \x87\x01aW\x7FV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD6\xFD\xB4[\x05\xC0\x87\xF2\xCCR:\n&!E\xEB\xFA\x92\xF9\xAF\x0F\xC3a\x1F\xEF\xE8\x81\xAF\x05\x8B\xF5=dsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static CELLARV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04tW`\x005`\xE0\x1C\x80c\x94\xBF\x80M\x11a\x02WW\x80c\xC0Ft\"\x11a\x01FW\x80c\xDF\xF9\x0B[\x11a\0\xC3W\x80c\xEF\x8B0\xF7\x11a\0\x87W\x80c\xEF\x8B0\xF7\x14a\n\xA3W\x80c\xF7\xB2N\x08\x14a\n\xB6W\x80c\xFCDE\x91\x14a\n\xBEW\x80c\xFCMC\xBE\x14a\n\xDEW\x80c\xFD\xD20\xB9\x14a\n\xF1W`\0\x80\xFD[\x80c\xDF\xF9\x0B[\x14a\t\xD2W\x80c\xE3\x94H\xE0\x14a\t\xDAW\x80c\xE7S\xE6\0\x14a\t\xF4W\x80c\xEC\xF7\x08X\x14a\n\x8BW\x80c\xEE\xF3>\xCA\x14a\n\x94W`\0\x80\xFD[\x80c\xCE\x96\xCBw\x11a\x01\nW\x80c\xCE\x96\xCBw\x14a\tsW\x80c\xD5\x05\xAC\xCF\x14a\t\x86W\x80c\xD9\x05w~\x14a\t\x99W\x80c\xDDb\xED>\x14a\t\xACW\x80c\xDF\x05\xA5*\x14a\t\xBFW`\0\x80\xFD[\x80c\xC0Ft\"\x14a\t)W\x80c\xC2D$Z\x14a\t<W\x80c\xC6=u\xB6\x14a\tEW\x80c\xC6\xE6\xF5\x92\x14a\tXW\x80c\xC8^^\x13\x14a\tkW`\0\x80\xFD[\x80c\xA8\x14NH\x11a\x01\xD4W\x80c\xB5)*\x99\x11a\x01\x98W\x80c\xB5)*\x99\x14a\x08\xD4W\x80c\xBA\x08vR\x14a\x08\xE7W\x80c\xBD\xC8\x14K\x14a\x08\xFAW\x80c\xBD\xCA\x91e\x14a\t\rW\x80c\xBF\x86\xD6\x90\x14a\t\x1CW`\0\x80\xFD[\x80c\xA8\x14NH\x14a\x08\x80W\x80c\xA9\x05\x9C\xBB\x14a\x08\x88W\x80c\xB0\xA7]6\x14a\x08\x9BW\x80c\xB3\xD7\xF6\xB9\x14a\x08\xAEW\x80c\xB4`\xAF\x94\x14a\x08\xC1W`\0\x80\xFD[\x80c\x9CU,\xA8\x11a\x02\x1BW\x80c\x9CU,\xA8\x14a\x08\tW\x80c\x9C_\0\xC2\x14a\x08\x1CW\x80c\x9E5\xC6[\x14a\x084W\x80c\x9F\xDB\x11\xB6\x14a\x08dW\x80c\xA4W\xC2\xD7\x14a\x08mW`\0\x80\xFD[\x80c\x94\xBF\x80M\x14a\x07\xA5W\x80c\x95\xD8\x9BA\x14a\x07\xB8W\x80c\x96\xD6Hy\x14a\x07\xC0W\x80c\x99\xFB\xAB\x88\x14a\x07\xE3W\x80c\x9Bo\xD1\x8E\x14a\x07\xF6W`\0\x80\xFD[\x80c@-&}\x11a\x03sW\x80co\xF1\xC0*\x11a\x02\xF0W\x80c{;\xAA\xB4\x11a\x02\xB4W\x80c{;\xAA\xB4\x14a\x07=W\x80c~\xCE\xBE\0\x14a\x07WW\x80c\x80'X`\x14a\x07jW\x80c\x8B\x0C\xEB\xF7\x14a\x07\x7FW\x80c\x8D\xA5\xCB[\x14a\x07\x92W`\0\x80\xFD[\x80co\xF1\xC0*\x14a\x06\xC2W\x80cp\xA0\x821\x14a\x06\xD1W\x80cp\xAF}\xF6\x14a\x06\xFAW\x80cr\x167\x15\x14a\x07\rW\x80c{\x109\x99\x14a\x07\x16W`\0\x80\xFD[\x80cX8Es\x11a\x037W\x80cX8Es\x14a\x06yW\x80cZ@\r%\x14a\x06\x8CW\x80c^,Wn\x14a\x06\x94W\x80cnU?e\x14a\x06\x9CW\x80cn\x85\xF1\x83\x14a\x06\xAFW`\0\x80\xFD[\x80c@-&}\x14a\x06\nW\x80cG \x90\xFE\x14a\x06\x1DW\x80cL\xDA\xD5\x06\x14a\x06@W\x80cN\xCA\x8A\x83\x14a\x06SW\x80cS\n7\x14\x14a\x06fW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x04\x01W\x80c8\x9Ar\x94\x11a\x03\xC5W\x80c8\x9Ar\x94\x14a\x05kW\x80c8\xD5.\x0F\x14a\x05~W\x80c9P\x93Q\x14a\x05\xBDW\x80c9\x98\xA6\x81\x14a\x05\xD0W\x80c<\xF9\x9AF\x14a\x05\xF7W`\0\x80\xFD[\x80c\x18\x16\r\xDD\x14a\x05 W\x80c#\xB8r\xDD\x14a\x05(W\x80c/;Z\x13\x14a\x05;W\x80c1<\xE5g\x14a\x05NW\x80c6D\xE5\x15\x14a\x05cW`\0\x80\xFD[\x80c\x07\xA2\xD1:\x11a\x04HW\x80c\x07\xA2\xD1:\x14a\x04\xBAW\x80c\t^\xA7\xB3\x14a\x04\xCDW\x80c\n(\xA4w\x14a\x04\xF0W\x80c\nh\x0E\x18\x14a\x05\x03W\x80c\x13\xAF@5\x14a\x05\rW`\0\x80\xFD[\x80bQ\xA3\xB7\x14a\x04yW\x80c\x01\xE1\xD1\x14\x14a\x04\x94W\x80c\x04\x02\xABc\x14a\x04\x9CW\x80c\x06\xFD\xDE\x03\x14a\x04\xA5W[`\0\x80\xFD[a\x04\x81`\x08\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\x81a\x0B\x04V[a\x04\x81a\x1C \x81V[a\x04\xADa\rjV[`@Qa\x04\x8B\x91\x90aW\xA3V[a\x04\x81a\x04\xC86`\x04aW\xD6V[a\r\xFCV[a\x04\xE0a\x04\xDB6`\x04aX\x04V[a\x0E\x15V[`@Q\x90\x15\x15\x81R` \x01a\x04\x8BV[a\x04\x81a\x04\xFE6`\x04aW\xD6V[a\x0E-V[a\x05\x0Ba\x0EbV[\0[a\x05\x0Ba\x05\x1B6`\x04aX0V[a\x0E\xFFV[`\x02Ta\x04\x81V[a\x04\xE0a\x0566`\x04aXMV[a\x0FuV[a\x05\x0Ba\x05I6`\x04aX\x9BV[a\x0F\x9BV[`\x12[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x04\x81a\x10,V[a\x04\x81a\x05y6`\x04aX\xB8V[a\x10;V[a\x05\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x04\xE0a\x05\xCB6`\x04aX\x04V[a\x12\xABV[a\x05\xDFg\x02\xC6\x8A\xF0\xBB\x14\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04\x8BV[a\x05\x0Ba\x06\x056`\x04aYjV[a\x12\xCDV[a\x04\x81a\x06\x186`\x04aX0V[a\x13\x9FV[a\x04\xE0a\x06+6`\x04aX0V[`\n` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\x81a\x06N6`\x04aW\xD6V[a\x14GV[a\x05\x0Ba\x06a6`\x04aY\x93V[a\x14oV[a\x05\x0Ba\x06t6`\x04aW\xD6V[a\x15\xD9V[a\x05\x0Ba\x06\x876`\x04aY\xC3V[a\x16\x83V[a\x04\x81`\x02\x81V[a\x05\x0Ba\x17\xCAV[a\x04\x81a\x06\xAA6`\x04aY\x93V[a\x18RV[a\x05\x0Ba\x06\xBD6`\x04aW\xD6V[a\x19dV[a\x05\xDFg\x01cEx]\x8A\0\0\x81V[a\x04\x81a\x06\xDF6`\x04aX0V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x05\x0Ba\x07\x086`\x04aYjV[a\x19\xF7V[a\x04\x81`\x12T\x81V[a\x05\xA5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\rTa\x05\xDF\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\x81a\x07e6`\x04aX0V[a\x1A\xCEV[a\x07ra\x1A\xECV[`@Qa\x04\x8B\x91\x90aY\xE5V[a\x05\x0Ba\x07\x8D6`\x04aX0V[a\x1BMV[`\x07Ta\x05\xA5\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\x81a\x07\xB36`\x04aY\x93V[a\x1C\xB5V[a\x04\xADa\x1D\xBBV[a\x04\xE0a\x07\xCE6`\x04aX0V[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x05\xA5a\x07\xF16`\x04aW\xD6V[a\x1D\xCAV[a\x05\x0Ba\x08\x046`\x04aYjV[a\x1D\xF4V[a\x05\x0Ba\x08\x176`\x04aW\xD6V[a\x1E\xBAV[`\rTa\x05\xA5\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x08Wa\x08B6`\x04aX0V[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x04\x8B\x91\x90aZHV[a\x04\x81`\x15T\x81V[a\x04\xE0a\x08{6`\x04aX\x04V[a\x1FPV[a\x04\x81a\x1F\xD6V[a\x04\xE0a\x08\x966`\x04aX\x04V[a!\x12V[a\x05\x0Ba\x08\xA96`\x04aX0V[a! V[a\x04\x81a\x08\xBC6`\x04aW\xD6V[a!\xB3V[a\x04\x81a\x08\xCF6`\x04aZbV[a!\xE0V[a\x05\x0Ba\x08\xE26`\x04aYjV[a#\0V[a\x04\x81a\x08\xF56`\x04aZbV[a#\xDCV[a\x05\x0Ba\t\x086`\x04aW\xD6V[a%\x05V[a\x05\xDFg\x06\xF0[Y\xD3\xB2\0\0\x81V[`\x14Ta\x04\xE0\x90`\xFF\x16\x81V[a\x05\x0Ba\t76`\x04aW\xD6V[a%pV[a\x04\x81`\x17T\x81V[a\x04\x81a\tS6`\x04aX0V[a&\x9FV[a\x04\x81a\tf6`\x04aW\xD6V[a&\xC4V[a\x05\x0Ba&\xD7V[a\x04\x81a\t\x816`\x04aX0V[a'IV[a\x05\x0Ba\t\x946`\x04aZ\xB3V[a'VV[a\x04\x81a\t\xA76`\x04aX0V[a(\xBAV[a\x04\x81a\t\xBA6`\x04a[$V[a(\xC7V[a\x05\x0Ba\t\xCD6`\x04aW\xD6V[a(\xF2V[a\x05\x0Ba)]V[`\rTa\t\xE7\x90`\xFF\x16\x81V[`@Qa\x04\x8B\x91\x90a[bV[`\x0ET`\x0FT`\x10T`\x11Ta\n<\x93\x92`\x01`\x01`@\x1B\x03\x80\x82\x16\x93`\x01`@\x1B\x83\x04\x82\x16\x93`\x01`\x80\x1B\x84\x04\x83\x16\x93`\x01`\xC0\x1B\x90\x04\x90\x92\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x87V[`@\x80Q\x97\x88R`\x01`\x01`@\x1B\x03\x96\x87\x16` \x89\x01R\x94\x86\x16\x94\x87\x01\x94\x90\x94R\x91\x84\x16``\x86\x01R\x90\x92\x16`\x80\x84\x01R`\xA0\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01a\x04\x8BV[a\x04\x81`\x13T\x81V[a\x05\xDFg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x04\x81a\n\xB16`\x04aW\xD6V[a,\xCDV[a\x05Q` \x81V[a\x04\x81a\n\xCC6`\x04aX0V[`\x16` R`\0\x90\x81R`@\x90 T\x81V[a\x05\x0Ba\n\xEC6`\x04a[oV[a,\xF5V[a\x05\x0Ba\n\xFF6`\x04aX0V[a.\xD6V[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B#Wa\x0B#a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BLW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0BiWa\x0Bia[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\x92W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0C@W`\0`\t\x82\x81T\x81\x10a\x0B\xB5Wa\x0B\xB5a[\xB7V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90Pa\x0B\xD5\x81a0wV[\x84\x83\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x0C\x10\x81a18V[\x83\x83\x81Q\x81\x10a\x0C\"Wa\x0C\"a[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a\x0C8\x81a[\xE3V[\x91PPa\x0B\x98V[P`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xCD\x91\x90a[\xFCV[`@Qc\xB33\xA1u`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB33\xA1u\x90a\r \x90\x86\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a\\\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r=W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\ra\x91\x90a\\\xABV[\x94PPPPP\x90V[```\x03\x80Ta\ry\x90a\\\xC4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\r\xA5\x90a\\\xC4V[\x80\x15a\r\xF2W\x80`\x1F\x10a\r\xC7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\r\xF2V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\r\xD5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x0E\x0F\x82a\x0E\na\x0B\x04V[a2\x85V[\x92\x91PPV[`\x003a\x0E#\x81\x85\x85a38V[P`\x01\x93\x92PPPV[`\0\x80a\x0E8a\x0B\x04V[\x90P`\0a\x0EE\x82a4\\V[\x90Pa\x0EZ\x84a\x0EU\x83\x85a\\\xF8V[a4\xC2V[\x94\x93PPPPV[`\x14T`\xFF\x16\x15a\x0E\x86W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`@Q\x80\x91\x03\x90\xFD[`\x14\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01[`@Q\x80\x91\x03\x90\xA1V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F)W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x82\x92\xFC\xE1\x8F\xA6\x9E\xDFM\xB7\xB9N\xA2\xE5\x82A\xDF\n\xE5\x7F\x97\xE0\xA6\xC9\xB2\x90g\x02\x8B\xF9-v\x90`\0\x90\xA3PV[`\x003a\x0F\x83\x85\x82\x85a5oV[a\x0F\x8E\x85\x85\x85a5\xE9V[`\x01\x91PP[\x93\x92PPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\rT`@Q\x7F\x196Y'\xBFR\xF7\xE9V\xD3v\xB3Td\x02\xDD\xA3\x82|\x06,ziBC\x1A%]!)G\xE8\x91a\x0F\xFE\x91`\xFF\x90\x91\x16\x90\x84\x90a]1V[`@Q\x80\x91\x03\x90\xA1`\r\x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x10$Wa\x10$aZ2V[\x02\x17\x90UPPV[`\0a\x106a7\xC2V[\x90P\x90V[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a\x10\x8CW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\x01\x14a\x10\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16a\x10\xF7W`@Qc\x1E\xC7\xBC\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a\x11\x01a\x0B\x04V[\x90P`\0a\x11\x0E`\x02T\x90V[\x90Pa\x11\x1B\x89\x880a8\xE9V[`\0a\x11&\x8Aa0wV[\x90P`\0a\x113\x8Aa0wV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x11TW\x88a\x11cV[a\x11c\x82\x82\x8B\x8B\x8B\x8B0a9\xDEV[\x94Pa\x11o\x8A\x86a<\rV[`\0a\x11\x9A`\x17Tg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x8A\x91\x90a\\\xF8V[\x86\x90g\r\xE0\xB6\xB3\xA7d\0\0a=LV[\x90P`\0a\x11\xC7`\x17Tg\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB7\x91\x90a]{V[\x87\x90g\r\xE0\xB6\xB3\xA7d\0\0a=zV[\x90Pa\x11\xD1a\x0B\x04V[\x95P\x80\x86\x11\x80a\x11\xE0WP\x81\x86\x10[\x15a\x12\x0FW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x0E\xB0V[`\x02T\x85\x14a\x12?W`\x02T`@Qc+@\x14Y`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x86\x90R`D\x01a\x0E\xB0V[\x8B`\x01`\x01`\xA0\x1B\x03\x16\x8D`\x01`\x01`\xA0\x1B\x03\x16\x7F\xFE\xA7\xA9\xA6\xE2\\\xD0\xBB\xBF\xA8\x0C\xE0\xC7dna\xEE^\x05Q\xB3\xFD\xAA\xFF\x06B\xE6\xF6\xAD\xCCr\xE2\x8D\x8A`@Qa\x12\x8D\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA3PP`\x01`\x08UP\x92\x99\x98PPPPPPPPPV[`\x003a\x0E#\x81\x85\x85a\x12\xBE\x83\x83a(\xC7V[a\x12\xC8\x91\x90a]{V[a38V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x06\xF0[Y\xD3\xB2\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x13)W`@Qc.\x15(m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\xC0\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7F'\xDD:\xE8\xFF\x7F^=w\xAEh\xED6\xDCx\r*\xB4\nO\xFD\xDA\x18\xD8\x05\xCBA\xEE\xA3\x9C(\x94\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xC0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x14T`\0\x90`\xFF\x16\x15a\x13\xB5WP`\0\x91\x90PV[`\x13T`\x12T`\0\x19\x82\x14\x80\x15a\x13\xCDWP`\0\x19\x81\x14[\x15a\x13\xDDWP`\0\x19\x93\x92PPPV[`\0a\x13\xE7a\x0B\x04V[\x90P`\0a\x14\x13a\x14\r\x87`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[\x83a2\x85V[\x90P`\0a\x14!\x85\x83a=\x99V[\x90P`\0a\x14/\x85\x85a=\x99V[\x90Pa\x14;\x82\x82a=\xB3V[\x98\x97PPPPPPPPV[`\0\x80a\x14Ra\x0B\x04V[\x90P`\0a\x14_\x82a4\\V[\x90Pa\x0EZ\x84a\x0E\n\x83\x85a\\\xF8V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a\x14\xBDW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT` \x11a\x14\xE3W`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x15'W`@Qci\x9Ff\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x15a\x15lW`@Qc\n\xF6}\x91`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[a\x15x`\t\x83\x83a=\xC2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\n` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xCF\xF5\xC8\xA0\x88\x84\xD2\xFA\xD99\xA9\xE0\xF0\xBFr\x9AO\x9A\xF6\x11\x1AW;M\x1F\x119l\x87\x11dm\x90a\x15\xCD\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x01cEx]\x8A\0\0\x81\x11\x15a\x16=W`@Qc\x02\xD2\xA9\x0F`\xE5\x1B\x81R`\x04\x81\x01\x82\x90Rg\x01cEx]\x8A\0\0`$\x82\x01R`D\x01a\x0E\xB0V[`\x17\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xDFK\xE3;.\x9E=\xD4\xD9\xE0\xE8VE\xAE\xA4(IJ\x06D\xA7,Q\xD6\xA1Z\xED\xAEkf\xA3\xFF\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0`\t\x82\x81T\x81\x10a\x16\xC2Wa\x16\xC2a[\xB7V[`\0\x91\x82R` \x82 \x01T`\t\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x90\x85\x90\x81\x10a\x16\xEFWa\x16\xEFa[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81\x81`\t\x86\x81T\x81\x10a\x17$Wa\x17$a[\xB7V[\x90`\0R` `\0 \x01`\0`\t\x87\x81T\x81\x10a\x17CWa\x17Ca[\xB7V[`\0\x91\x82R` \x91\x82\x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90U\x82T\x94\x84\x16a\x01\0\x92\x90\x92\n\x91\x82\x02\x91\x84\x02\x19\x90\x94\x16\x17\x90U`@\x80Q\x87\x81R\x92\x83\x01\x86\x90R\x83\x82\x16\x92\x91\x85\x16\x91\x7F<\xAEOW\x96\xF9\xD1\x9F\xA1\xDC\xFA\xAC\xAEZ\x98\x11\xC0\x08\xF6\xF5\x17\xA3\xECh\x9D\x90=oi\x9EIU\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16a\x18\x17W`@Qc\xECqe\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x14\x80T`\xFF\x19\x16\x90U`@Q`\0\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01a\x0E\xF5V[`\0`\x08T`\x01\x14a\x18vW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0a\x18\x85a\x0B\x04V[\x90Pa\x18\x90\x81a?;V[a\x18\x9A\x84\x82a?xV[\x91P\x81`\0\x03a\x18\xBDW`@QcBo\x157`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC8\x84\x83\x85a?\x97V[a\x18\xFD`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x87a@\xCAV[a\x19\x07\x83\x83aA5V[`@\x80Q\x85\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a\x19X\x84\x83\x85aB V[P`\x01`\x08U\x92\x91PPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x11\x15a\x19\xB6W`@Qc\x01\x9B^1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x10T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FQ:\xC1\x9C\xBB\xAA\xADNE\x0Cs.\xD3v5\x17\x8B}\x83\xBF\x8E\x84\xA9@\xFF\xE7\xE0R\xC9\xC7\xCA\xA2\x91\x01`@Q\x80\x91\x03\x90\xA1`\x10UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A!W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\x02\xC6\x8A\xF0\xBB\x14\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x1ASW`@Qc.\x15(m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x80\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7FD\xAD\xA2a\xFF\\\x9A\xAC\xBF\x8D\x06\x87)G\x99\xC8\xE3\xE0\x81\x0E\xCC\x1E\xAF\xD9t\x19\xDF\xC3\x1D\xB5\xD5#\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x81 Ta\x0E\x0FV[```\t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\r\xF2W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1B&WPPPPP\x90P\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BwW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16a\x1B\xBBW`@Qc\x1E\xC7\xBC\xD3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a\x1B\xC6\x82a0wV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1CMW`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16`$\x82\x01R`D\x01a\x0E\xB0V[`\rT`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92a\x01\0\x90\x04\x16\x90\x7F\x7F\x83_\xBFC\xC4\xD0Z\x9CN\xA5\xCA\xFA\t\xFB\xB3\xB00}\x12\x95l\xEE\xDC\xE4?\xD7\xA39\xE5\x04n\x90`\0\x90\xA3P`\r\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16a\x01\0\x02a\x01\0`\x01`\xA8\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`\x08T`\x01\x14a\x1C\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0a\x1C\xE8a\x0B\x04V[\x90Pa\x1C\xF3\x81a?;V[a\x1C\xFD\x84\x82aBYV[\x91P\x81`\0\x03a\x1D W`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D+\x82\x85\x85a?\x97V[a\x1D``\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x85a@\xCAV[a\x1Dj\x83\x85aA5V[`@\x80Q\x83\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x85\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a\x19X\x82\x85\x85aB V[```\x04\x80Ta\ry\x90a\\\xC4V[`\t\x81\x81T\x81\x10a\x1D\xDAW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x81V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a\x1EPW`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x01`@\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xC7\x93\x99\t?s\xF1\xFF\x8C/\x82y\xAA\x9E\x88\x03\xB6\x94\xC6\0\x16\xC3D\xEC\xFC*\x89\xFDw\xC0\x04\xDB\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x08\x81\x10\x80a\x1E\xF4WPa\x1C \x81\x11[\x15a\x1F\x12W`@Qc:`#?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x15\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\"\x7F\xF5\xC6\xB5\xFF\xB3\x95#k\t\xFD\x1BG+\xB1(\xB3n\xAA\x17Uf3\xFE\xEF\xE2\x8E\x94A\x1F$\x91\x01a\x16wV[`\x003\x81a\x1F^\x82\x86a(\xC7V[\x90P\x83\x81\x10\x15a\x1F\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: decreased allowance below`D\x82\x01Rd zero`\xD8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[a\x1F\xCB\x82\x86\x86\x84\x03a38V[P`\x01\x94\x93PPPPV[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xF5Wa\x1F\xF5a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a \x1EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a ;Wa ;a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a dW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0C@W`\0`\t\x82\x81T\x81\x10a \x87Wa \x87a[\xB7V[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90Pa \xA7\x81a0wV[\x84\x83\x81Q\x81\x10a \xB9Wa \xB9a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa \xE2\x81aBxV[\x83\x83\x81Q\x81\x10a \xF4Wa \xF4a[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80a!\n\x81a[\xE3V[\x91PPa jV[`\x003a\x0E#\x81\x85\x85a5\xE9V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a!JW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x11T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FQ\xDB\xB5\xA6[\xB2'7\x86\x1Ac\xEC\x12\xBAl\xE7\x8A\x98c\x1E\x94\x04\xB0Vz.\xAFz\x06\xFCTM\x91\x01`@Q\x80\x91\x03\x90\xA1`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80a!\xBEa\x0B\x04V[\x90P`\0a!\xCB\x82a4\\V[\x90Pa\x0EZ\x84a!\xDB\x83\x85a\\\xF8V[aBYV[`\0`\x08T`\x01\x14a\"\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0\x80\x80\x80\x80a\"\x17aB\xF6V[\x94P\x94P\x94P\x94P\x94Pa\"*\x85a?;V[a\"4\x89\x86a4\xC2V[\x95Pa\"B\x89\x87\x8A\x8AaF\x93V[a\"L\x87\x87aF\xC2V[`\0a\"W`\x02T\x90V[\x90Pa\"c\x88\x88aF\xDDV[`@\x80Q\x8B\x81R` \x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x92\x90\x8C\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\0`\rT`\xFF\x16`\x01\x81\x11\x15a\"\xC7Wa\"\xC7aZ2V[\x14a\"\xDFWa\"\xDA\x87\x82\x8B\x88\x87\x87aH7V[a\"\xEDV[a\"\xED\x8A\x8A\x87\x87\x87\x87aIOV[PP`\x01`\x08UP\x92\x96\x95PPPPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a#*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a#\\W`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`@\x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x83R\x83\x16` \x83\x01R\x7F\xB5\xCC\x99J&\n\x85\xA4-e\x88f\x82!W\x1A\xE0\xA1O\n(\xF9\xE4\x81zQ\x95&!\x02\xC8h\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`\x08T`\x01\x14a$\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\0\x80\x80\x80\x80a$\x13aB\xF6V[\x94P\x94P\x94P\x94P\x94Pa$&\x85a?;V[a$0\x87\x8AaF\xC2V[a$:\x89\x86a2\x85V[\x95P\x85`\0\x03a$]W`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$i\x86\x8A\x8A\x8AaF\x93V[`\0a$t`\x02T\x90V[\x90Pa$\x80\x88\x8BaF\xDDV[`@\x80Q\x88\x81R` \x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x80\x8B\x16\x92\x90\x8C\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\0`\rT`\xFF\x16`\x01\x81\x11\x15a$\xE4Wa$\xE4aZ2V[\x14a$\xF7Wa\"\xDA\x8A\x82\x8B\x88\x87\x87aH7V[a\"\xDA\x87\x8A\x87\x87\x87\x87aIOV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x13T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCF\xB5\xA4T\xB8\xAA}\xC0N\xCB[\xC1A\x0B*W\x96\x9C\xA1\xD6\x7Ff\xD5e\x19o`\xC6\xF9\x97T\x04\x91\x01`@Q\x80\x91\x03\x90\xA1`\x13UV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0`\t\x82\x81T\x81\x10a%\xAFWa%\xAFa[\xB7V[`\0\x91\x82R` \x82 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91Pa%\xCE\x82a18V[\x90P\x80\x15a&\x01W`@Qc\x84Th\x9F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x0E\xB0V[`\rT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x90\x83\x16\x03a&5W`@Qctw\x95\xFF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&@`\t\x84aL\x9EV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\n` R`@\x90\x81\x90 \x80T`\xFF\x19\x16\x90UQ\x7F\x03\xC7\x8EN \xA7+\x1FWzc\xB5E$\xC6\x84\xCEs\xD2\xE4\x14\x97\x0E\xE6\x06\xBB\x0Bz\x90\x04\xAA\x9C\x90a&\x92\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`\0\x80a&\xAB\x83a\x13\x9FV[\x90P`\0\x19\x81\x14a\x0E\x0FWa&\xBF\x81a&\xC4V[a\x0F\x94V[`\0a\x0E\x0F\x82a&\xD2a\x0B\x04V[a?xV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\0a'\x0Ba\x0B\x04V[`\x0E\x81\x90U`@Q\x81\x81R\x90\x91P\x7F\x87Ua\xDD\xC3B\xB1)\x81\x10=\x85\xBE\x8D\xB67Z\x88\x98-L\xF5\x8DA\x1E8qZ\xE4h30\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\0a\x0E\x0F\x82`\0aM\xB8V[\x83B\x11\x15a'\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20Permit: expired deadline\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\0\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x88\x88\x88a'\xD5\x8CaP\x01V[`@\x80Q` \x81\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x86\x01R\x92\x90\x91\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x86\x90R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a(0\x82aP'V[\x90P`\0a(@\x82\x87\x87\x87aPuV[\x90P\x89`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a(\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1E`$\x82\x01R\x7FERC20Permit: invalid signature\0\0`D\x82\x01R`d\x01a\x0E\xB0V[a(\xAE\x8A\x8A\x8Aa38V[PPPPPPPPPPV[`\0a\x0E\x0F\x82`\x01aM\xB8V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x12T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x1F!C-\xD7\xB8\xEA\xD6M.|\x06\xA7K\xAF\x13x;-/qS\xF0\x99\xE2\xC4\xCA\xBC<]\xBE\xC6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x12UV[`\x08T`\x01\x14a)\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]WV[`\x02`\x08U`\x11T`\x01`\x01`\xA0\x1B\x03\x16\x80a)\xAEW`@Qc\xDC\x13a\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a)\xB8a\x0B\x04V[\x90Pa)\xC3\x81a?;V[0`\0\x90\x81R` \x81\x90R`@\x81 T`\x0FT\x90\x91\x90a)\xED\x90\x83\x90`\x01`\x01`@\x1B\x03\x16aP\x9DV[`\rT\x90\x91P`\0\x90a*\x10\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16Ba\\\xF8V[`\x0FT\x90\x91P`\0\x90c\x01\xE13\x80\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a*C\x85\x89a]\x8EV[a*M\x91\x90a]\x8EV[a*W\x91\x90a]\xADV[a*a\x91\x90a]\xADV[\x90P`\0a*wa*r\x83\x88a?xV[aP\xB2V[\x90Pa*\x830\x82aA5V[a*\x8D\x81\x86a]{V[`\x0FT\x90\x95Pa*\xAE\x90\x82\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aP\x9DV[a*\xB8\x90\x85a]{V[\x93P\x83\x15a*\xD8Wa*\xCB0\x88\x86a5\xE9V[a*\xD5\x84\x86a\\\xF8V[\x94P[`\r\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA8\x1B\x19\x16Bc\xFF\xFF\xFF\xFF\x16`\x01`\xA8\x1B\x02\x17\x90U`\0a+\x06\x86\x88a2\x85V[\x90P\x80\x15a,\x85W\x80`\x0E`\0\x01`\0\x82\x82Ta+#\x91\x90a\\\xF8V[\x90\x91UPa+3\x90P0\x87aF\xDDV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\0`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\xBF\x91\x90a[\xFCV[\x90Pa+\xF5`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82\x84aP\xE1V[`\x10T`@Qc\x1F\xFB\xE7\xF9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R`D\x81\x01\x84\x90R\x90\x82\x16\x90c\x1F\xFB\xE7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a,kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a,\x7FW=`\0\x80>=`\0\xFD[PPPPP[`@\x80Q\x87\x81R` \x81\x01\x83\x90R\x7F\x15\xE3\xE2\xA7jh9\xC2D\xC1\xED\n\x82\x1C#<\xE8\xAFU-\xFF\xCB\x85`\x89\xEA\xE6\xCB\xBB\xB7\x1E\xA6\x91\x01`@Q\x80\x91\x03\x90\xA1PP`\x01`\x08UPPPPPPV[`\0\x80a,\xD8a\x0B\x04V[\x90P`\0a,\xE5\x82a4\\V[\x90Pa\x0EZ\x84a&\xD2\x83\x85a\\\xF8V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a-\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0C` \x90\x81R`@\x80\x83 \x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x0B\x90\x93R\x92 \x80T\x84\x93\x91\x92\x16\x90\x83`\x02\x81\x11\x15a-jWa-jaZ2V[\x02\x17\x90UP`\0a-z\x83a0wV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\xE2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\x06\x91\x90a[\xFCV[`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x91\x90\x91\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.r\x91\x90a]\xCFV[a.\x9AW`@Qc~\xB5n\xDB`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`@Q`\x01\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\xD6\0\xB94\x86\x03\xC6\xDE\xFF4\xB4\xE0\xB2\x8B`\xE1\xC8\x03l\x80gA\xB9\xE6\xD9\x002\xE7\xF3}\xD2\x7F\x90` \x01a&\x92V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a/\0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x90a]\x0BV[`\x14T`\xFF\x16\x15a/$W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\tT` \x11a/JW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a/\x8EW`@Qci\x9Ff\xB1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\n` R`@\x90 T`\xFF\x16\x15a/\xD3W`@Qc\n\xF6}\x91`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[`\t\x80T`\x01\x80\x82\x01\x83U\x7Fn\x15@\x17\x1Bl\x0C\x96\x0Bq\xA7\x02\r\x9F`\x07\x7Fj\xF91\xA8\xBB\xF5\x90\xDA\x02#\xDA\xCFu\xC7\xAF\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x81\x17\x90\x91U`\0\x81\x81R`\n` R`@\x90 \x80T`\xFF\x19\x16\x83\x17\x90U\x91T\x7F\xCF\xF5\xC8\xA0\x88\x84\xD2\xFA\xD99\xA9\xE0\xF0\xBFr\x9AO\x9A\xF6\x11\x1AW;M\x1F\x119l\x87\x11dm\x91a0c\x91a\\\xF8V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a0\xA7Wa0\xA7aZ2V[\x14\x80a0\xC4WP`\x02\x81`\x02\x81\x11\x15a0\xC2Wa0\xC2aZ2V[\x14[\x15a1+W\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a[\xFCV[P\x90\x91\x90PV[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a1hWa1haZ2V[\x14\x80a1\x85WP`\x02\x81`\x02\x81\x11\x15a1\x83Wa1\x83aZ2V[\x14[\x15a2YW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cL\xDA\xD5\x06\x90\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xD6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xFA\x91\x90a\\\xABV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a2\x18\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a25W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x94\x91\x90a\\\xABV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01a2\x18V[`\0\x80a2\x91`\x02T\x90V[\x90P\x80\x15a2\xA9Wa2\xA4\x84\x84\x83a=zV[a\x0EZV[a\x0EZ`\x12\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a30\x91\x90a]\xF1V[\x86\x91\x90aQ\xF6V[`\x01`\x01`\xA0\x1B\x03\x83\x16a3\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC20: approve from the zero add`D\x82\x01Rcress`\xE0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a3\xFBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: approve to the zero addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x85\x90U\x90Q\x84\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x0FT`\0\x90`\x01`\xC0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80\x15\x80a4}WP\x82\x15[\x15a4\x8BWP`\0\x92\x91PPV[`\x0ET\x80\x84\x11\x15a4\xBBW`\0a4\xA2\x82\x86a\\\xF8V[\x90Pa4\xB7\x81`\x01`\x01`@\x1B\x03\x85\x16aP\x9DV[\x93PP[PP\x91\x90PV[`\0\x80a4\xCE`\x02T\x90V[\x90P\x80\x15a4\xE1Wa2\xA4\x84\x82\x85a=LV[a\x0EZ\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5f\x91\x90a]\xF1V[\x85\x90`\x12aQ\xF6V[`\0a5{\x84\x84a(\xC7V[\x90P`\0\x19\x81\x14a5\xE3W\x81\x81\x10\x15a5\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC20: insufficient allowance\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[a5\xE3\x84\x84\x84\x84\x03a38V[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a6MW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC20: transfer from the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x82\x16a6\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC20: transfer to the zero addr`D\x82\x01Rbess`\xE8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[a6\xBA\x83\x83\x83aR_V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a72W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FERC20: transfer amount exceeds b`D\x82\x01Realance`\xD0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R` \x81\x90R`@\x80\x82 \x85\x85\x03\x90U\x91\x85\x16\x81R\x90\x81 \x80T\x84\x92\x90a7i\x90\x84\x90a]{V[\x92PP\x81\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x84`@Qa7\xB5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3a5\xE3V[`\x000`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a8\x1BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a8EWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[P`@\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x83\x01RF`\x80\x83\x01R0`\xA0\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xC0\x90\x92\x01\x90\x92R\x80Q\x91\x01 \x90V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a9\x19Wa9\x19aZ2V[\x14\x80a96WP`\x02\x81`\x02\x81\x11\x15a94Wa94aZ2V[\x14[\x15a9\xBAW`@Qc-\x18+\xE5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R0`D\x83\x01R\x85\x16\x90c\xB4`\xAF\x94\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a9\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\xB4\x91\x90a\\\xABV[Pa5\xE3V[`\x01`\x01`\xA0\x1B\x03\x82\x160\x14a5\xE3Wa5\xE3`\x01`\x01`\xA0\x1B\x03\x85\x16\x83\x85aRhV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90\x81\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x8B\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:M\x91\x90a\\\xABV[a:W\x91\x90a\\\xF8V[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x01`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a:\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a:\xE6\x91\x90a[\xFCV[\x90Pa:\xFC`\x01`\x01`\xA0\x1B\x03\x8B\x16\x82\x8AaP\xE1V[\x80`\x01`\x01`\xA0\x1B\x03\x16c-\xFE\x16\x90\x88\x88\x88\x88\x8F\x8F`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a;2\x96\x95\x94\x93\x92\x91\x90a^\x0EV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a;QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;u\x91\x90a\\\xABV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x93P\x82\x90`\x01`\x01`\xA0\x1B\x03\x8C\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\xBEW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;\xE2\x91\x90a\\\xABV[\x14a<\0W`@Qc\n\x92\x13\xB5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15a<=Wa<=aZ2V[\x14\x80a<ZWP`\x02\x81`\x02\x81\x11\x15a<XWa<XaZ2V[\x14[\x15a=GWa<\xD6\x83\x83\x85`\x01`\x01`\xA0\x1B\x03\x16c8\xD5.\x0F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\xC6\x91\x90a[\xFCV[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90aP\xE1V[`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cnU?e\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a=#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE3\x91\x90a\\\xABV[PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a=dW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a=\x92W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x81\x83\x11a=\xA9W`\0a\x0F\x94V[a\x0F\x94\x82\x84a\\\xF8V[`\0\x81\x83\x10a1+W\x81a\x0F\x94V[\x82T\x80\x15a?\x08W\x83\x80a=\xD7`\x01\x84a\\\xF8V[\x81T\x81\x10a=\xE7Wa=\xE7a[\xB7V[`\0\x91\x82R` \x80\x83 \x90\x91\x01T\x83T`\x01\x81\x81\x01\x86U\x94\x84R\x91\x83 \x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x90a>/\x90\x83a\\\xF8V[\x90P[\x83\x81\x11\x15a>\xC1W\x84a>F`\x01\x83a\\\xF8V[\x81T\x81\x10a>VWa>Va[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81T\x81\x10a>\x86Wa>\x86a[\xB7V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a>\xB9\x81a^kV[\x91PPa>2V[P\x81\x84\x84\x81T\x81\x10a>\xD5Wa>\xD5a[\xB7V[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPa5\xE3V[\x83T`\x01\x81\x01\x85U`\0\x85\x81R` \x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90UPPPPV[`\0a?F\x82a4\\V[\x90P\x80\x15a?tW`\0a?]a*r\x83\x85a?xV[\x90P\x80\x15a=GW`\x0E\x83\x90Ua=G0\x82aA5V[PPV[`\0\x80a?\x84`\x02T\x90V[\x90P\x80\x15a4\xE1Wa2\xA4\x84\x82\x85a=zV[`\x14T`\xFF\x16\x15a?\xBBW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a@rW`@QcUQ\xE1\xB5`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cUQ\xE1\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@/W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@S\x91\x90a]\xCFV[a@rW`@Qc4\x87\x1F%`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x0E\xB0V[`\0a@}\x82a\x13\x9FV[\x90P\x80\x84\x11\x15a@\xAAW`@Qc-!\xEB\x87`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x0E\xB0V[\x83`\x0E`\0\x01`\0\x82\x82Ta@\xBF\x91\x90a]{V[\x90\x91UPPPPPPV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra5\xE3\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91RaR\x98V[`\x01`\x01`\xA0\x1B\x03\x82\x16aA\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC20: mint to the zero address\0`D\x82\x01R`d\x01a\x0E\xB0V[aA\x97`\0\x83\x83aR_V[\x80`\x02`\0\x82\x82TaA\xA9\x91\x90a]{V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x81 \x80T\x83\x92\x90aA\xD6\x90\x84\x90a]{V[\x90\x91UPP`@Q\x81\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90`\0\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[`\rTaB;\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a<\rV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x16` R`@\x90 C\x90UPPV[`\0\x80aBe`\x02T\x90V[\x90P\x80\x15a2\xA9Wa2\xA4\x84\x84\x83a=LV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0B` R`@\x81 T`\xFF\x16`\x01\x81`\x02\x81\x11\x15aB\xA8WaB\xA8aZ2V[\x14\x80aB\xC5WP`\x02\x81`\x02\x81\x11\x15aB\xC3WaB\xC3aZ2V[\x14[\x15a2YW`@Qc\xCE\x96\xCBw`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\xCE\x96\xCBw\x90`$\x01a2\x18V[`\tT`\0\x90``\x90\x81\x90\x81\x90\x81\x90\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x1DWaC\x1Da[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aCFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aCaWaCaa[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xA5WaC\xA5a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aC\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xE9WaC\xE9a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aD\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15aD-WaD-a[\xA1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aDVW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P`\0[\x81\x81\x10\x15aEgW`\0`\t\x82\x81T\x81\x10aDyWaDya[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80\x87\x83\x81Q\x81\x10aD\xACWaD\xACa[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPaD\xD5\x81a0wV[\x86\x83\x81Q\x81\x10aD\xE7WaD\xE7a[\xB7V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPaE\x10\x81a18V[\x85\x83\x81Q\x81\x10aE\"WaE\"a[\xB7V[` \x02` \x01\x01\x81\x81RPPaE7\x81aBxV[\x84\x83\x81Q\x81\x10aEIWaEIa[\xB7V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x80aE_\x81a[\xE3V[\x91PPaD\\V[P`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aE\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aE\xF4\x91\x90a[\xFCV[`@Qc\xB33\xA1u`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\xB33\xA1u\x90aFG\x90\x88\x90\x88\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a\\\x19V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aFdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\x88\x91\x90a\\\xABV[\x96PPP\x90\x91\x92\x93\x94V[aF\x9C\x81aSjV[`\x0ET\x80\x85\x11aF\xB5WaF\xB0\x85\x82a\\\xF8V[aF\xB8V[`\0[`\x0EUPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a?tWa?t\x823\x83a5oV[`\x01`\x01`\xA0\x1B\x03\x82\x16aG=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC20: burn from the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[aGI\x82`\0\x83aR_V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15aG\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC20: burn amount exceeds balan`D\x82\x01Race`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R` \x81\x90R`@\x81 \x83\x83\x03\x90U`\x02\x80T\x84\x92\x90aG\xEC\x90\x84\x90a\\\xF8V[\x90\x91UPP`@Q\x82\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0[\x83Q\x81\x10\x15aIFW`\0\x84\x82\x81Q\x81\x10aHWWaHWa[\xB7V[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10aHuWaHua[\xB7V[` \x02` \x01\x01Q\x90P\x80`\0\x03aH\x8EWPPaI4V[`\0aH\x9B\x82\x8B\x8Ba=zV[\x90P\x84\x84\x81Q\x81\x10aH\xAFWaH\xAFa[\xB7V[` \x02` \x01\x01Q\x81\x11\x15aH\xE2W`@Qc\xF2\x01\x8Fo`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x0E\xB0V[aH\xED\x83\x82\x8Aa8\xE9V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x94\xAD7\xA0\xF8\x93\\\n\xCD\xD1\xCC\xFF\xE9\xAA\xD2\x96\xBF\xC8\xD0\xA1\xDD-\x0C\xFB\xACy@\\_\x86\xED\x82\x82`@QaI(\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPP[\x80aI>\x81a[\xE3V[\x91PPaH:V[PPPPPPPV[`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aI\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aI\xDB\x91\x90a[\xFCV[\x90P`\0[\x85Q\x81\x10\x15aL{W\x83\x81\x81Q\x81\x10aI\xFBWaI\xFBa[\xB7V[` \x02` \x01\x01Q`\0\x03\x15aLiW`\0\x85\x82\x81Q\x81\x10aJ\x1FWaJ\x1Fa[\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJdW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\x88\x91\x90a]\xF1V[aJ\x93\x90`\na_fV[\x90P`\0\x83`\x01`\x01`\xA0\x1B\x03\x16c\xBA\xAAa\xBE\x88\x85\x81Q\x81\x10aJ\xB8WaJ\xB8a[\xB7V[` \x02` \x01\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aK\x13\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aKT\x91\x90a\\\xABV[\x90P`\0aK\x86\x82\x84\x88\x87\x81Q\x81\x10aKoWaKoa[\xB7V[` \x02` \x01\x01Qa=z\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x8B\x82\x11\x15aK\xA8WaK\x9D\x8C\x85\x85a=zV[\x90P`\0\x9BPaK\xD3V[\x86\x85\x81Q\x81\x10aK\xBAWaK\xBAa[\xB7V[` \x02` \x01\x01Q\x90P\x81\x8CaK\xD0\x91\x90a\\\xF8V[\x9BP[aK\xF7\x8A\x86\x81Q\x81\x10aK\xE8WaK\xE8a[\xB7V[` \x02` \x01\x01Q\x82\x8Da8\xE9V[\x89\x85\x81Q\x81\x10aL\tWaL\ta[\xB7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\x94\xAD7\xA0\xF8\x93\\\n\xCD\xD1\xCC\xFF\xE9\xAA\xD2\x96\xBF\xC8\xD0\xA1\xDD-\x0C\xFB\xACy@\\_\x86\xED\x82\x82`@QaLK\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x8B`\0\x03aLdWPPPPaL{V[PPPP[\x80aLs\x81a[\xE3V[\x91PPaI\xE0V[P\x86\x15aIFW`@Qc\xCC^\xA3\x9B`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x0E\xB0V[\x81T\x80\x82\x10aL\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x0E\xB0V[\x81[aL\xF2`\x01\x83a\\\xF8V[\x81\x10\x15aM\x80W\x83aM\x05\x82`\x01a]{V[\x81T\x81\x10aM\x15WaM\x15a[\xB7V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81T\x81\x10aMEWaMEa[\xB7V[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80aMx\x81a[\xE3V[\x91PPaL\xE7V[P\x82\x80T\x80aM\x91WaM\x91a_uV[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90UPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x16` R`@\x81 T\x80\x15aM\xFEW`\0`\x15T\x82aM\xE7\x91\x90a]{V[\x90PC\x81\x11\x15aM\xFCW`\0\x92PPPa\x0E\x0FV[P[`\0aN\x08a\x0B\x04V[\x90P`\0aN\x15\x82a4\\V[\x90P`\0aNEaN;\x88`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x0E\n\x84\x86a\\\xF8V[\x90P`\0`\rT`\xFF\x16`\x01\x81\x11\x15aN`WaN`aZ2V[\x03aN\x89W`\0aNoa\x1F\xD6V[\x90P\x80\x82\x11\x15aN\x7FW\x80aN\x81V[\x81[\x95PPaO\xE0V[`\0\x80aN\x94aB\xF6V[\x94P\x94PPPP`\0aN\xA6`\x02T\x90V[\x90P`\0aN\xC9\x8B`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0[\x84Q\x81\x10\x15aO\x92W\x85\x81\x81Q\x81\x10aN\xF2WaN\xF2a[\xB7V[` \x02` \x01\x01Q`\0\x03\x15aO\x80W\x84\x81\x81Q\x81\x10aO\x14WaO\x14a[\xB7V[` \x02` \x01\x01Q`\0\x03aO6W`\0\x9APPPPPPPPPPPa\x0E\x0FV[`\0aOpg\r\xE0\xB6\xB3\xA7d\0\0\x88\x84\x81Q\x81\x10aOVWaOVa[\xB7V[` \x02` \x01\x01Q\x88\x85\x81Q\x81\x10aKoWaKoa[\xB7V[\x90P\x82\x81\x10\x15aO~W\x80\x92P[P[\x80aO\x8A\x81a[\xE3V[\x91PPaN\xD7V[P`\0aO\xA8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x86a=zV[\x90P\x81\x81\x11\x15aO\xD5WaO\xD0\x82g\r\xE0\xB6\xB3\xA7d\0\0aO\xC9\x8B\x8Da\\\xF8V[\x91\x90a=zV[aO\xD7V[\x86[\x9APPPPPPP[\x85\x15aO\xF7WaO\xF4\x85a&\xD2\x84\x86a\\\xF8V[\x94P[PPPP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x05` R`@\x90 \x80T`\x01\x81\x01\x82U\x90a12V[`\0a\x0E\x0FaP4a7\xC2V[\x83`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x83\x90R`B\x81\x01\x82\x90R`\0\x90`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`\0\x80`\0aP\x86\x87\x87\x87\x87aS\xC5V[\x91P\x91PaP\x93\x81aT\xB2V[P\x95\x94PPPPPV[`\0a\x0F\x94\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a=zV[`\0\x80aP\xBE`\x02T\x90V[\x90P\x82\x81\x11\x15a12W`\0aP\xD4\x84\x83a\\\xF8V[\x90Pa\x0EZ\x84\x83\x83a=LV[\x80\x15\x80aQ[WP`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aQ5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aQY\x91\x90a\\\xABV[\x15[aQ\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FSafeERC20: approve from non-zero`D\x82\x01Ru to non-zero allowance`P\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra=G\x90\x84\x90c\t^\xA7\xB3`\xE0\x1B\x90`d\x01a@\xFEV[`\0\x81`\xFF\x16\x83`\xFF\x16\x03aR\x0CWP\x82a\x0F\x94V[\x81`\xFF\x16\x83`\xFF\x16\x10\x15aR@WaR$\x83\x83a_\x8BV[aR/\x90`\na_fV[aR9\x90\x85a]\x8EV[\x90Pa\x0F\x94V[aRJ\x82\x84a_\x8BV[aRU\x90`\na_fV[aR9\x90\x85a]\xADV[a=G\x83aSjV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra=G\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a@\xFEV[`\0aR\xED\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aVk\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a=GW\x80\x80` \x01\x90Q\x81\x01\x90aS\x0B\x91\x90a]\xCFV[a=GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x16` R`@\x90 T\x80\x15a?tW`\0`\x15T\x82aS\x99\x91\x90a]{V[\x90PC\x81\x11\x15a=GW`@Qc\x06\xF8\xEE?`\xE2\x1B\x81R`\x04\x81\x01\x82\x90RC`$\x82\x01R`D\x01a\x0E\xB0V[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15aS\xFCWP`\0\x90P`\x03aT\xA9V[\x84`\xFF\x16`\x1B\x14\x15\x80\x15aT\x14WP\x84`\xFF\x16`\x1C\x14\x15[\x15aT%WP`\0\x90P`\x04aT\xA9V[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aTyW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aT\xA2W`\0`\x01\x92P\x92PPaT\xA9V[\x91P`\0\x90P[\x94P\x94\x92PPPV[`\0\x81`\x04\x81\x11\x15aT\xC6WaT\xC6aZ2V[\x03aT\xCEWPV[`\x01\x81`\x04\x81\x11\x15aT\xE2WaT\xE2aZ2V[\x03aU/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\x02\x81`\x04\x81\x11\x15aUCWaUCaZ2V[\x03aU\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FECDSA: invalid signature length\0`D\x82\x01R`d\x01a\x0E\xB0V[`\x03\x81`\x04\x81\x11\x15aU\xA4WaU\xA4aZ2V[\x03aU\xFCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 's' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[`\x04\x81`\x04\x81\x11\x15aV\x10WaV\x10aZ2V[\x03aVhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FECDSA: invalid signature 'v' val`D\x82\x01Raue`\xF0\x1B`d\x82\x01R`\x84\x01a\x0E\xB0V[PV[``a\x0EZ\x84\x84`\0\x85\x85`\x01`\x01`\xA0\x1B\x03\x85\x16;aV\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0E\xB0V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaV\xE9\x91\x90a_\xA4V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aW&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aW+V[``\x91P[P\x91P\x91PaW;\x82\x82\x86aWFV[\x97\x96PPPPPPPV[``\x83\x15aWUWP\x81a\x0F\x94V[\x82Q\x15aWeW\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0E\xB0\x91\x90aW\xA3V[`\0[\x83\x81\x10\x15aW\x9AW\x81\x81\x01Q\x83\x82\x01R` \x01aW\x82V[PP`\0\x91\x01RV[` \x81R`\0\x82Q\x80` \x84\x01RaW\xC2\x81`@\x85\x01` \x87\x01aW\x7FV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aW\xE8W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aVhW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aX\x17W`\0\x80\xFD[\x825aX\"\x81aW\xEFV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15aXBW`\0\x80\xFD[\x815a\x0F\x94\x81aW\xEFV[`\0\x80`\0``\x84\x86\x03\x12\x15aXbW`\0\x80\xFD[\x835aXm\x81aW\xEFV[\x92P` \x84\x015aX}\x81aW\xEFV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\x02\x81\x10aVhW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aX\xADW`\0\x80\xFD[\x815a\x0F\x94\x81aX\x8EV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15aX\xD1W`\0\x80\xFD[\x865aX\xDC\x81aW\xEFV[\x95P` \x87\x015aX\xEC\x81aW\xEFV[\x94P`@\x87\x015\x93P``\x87\x015aY\x03\x81aX\x8EV[\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aY\x1FW`\0\x80\xFD[\x81\x89\x01\x91P\x89`\x1F\x83\x01\x12aY3W`\0\x80\xFD[\x815\x81\x81\x11\x15aYBW`\0\x80\xFD[\x8A` \x82\x85\x01\x01\x11\x15aYTW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15aY|W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F\x94W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aY\xA6W`\0\x80\xFD[\x825\x91P` \x83\x015aY\xB8\x81aW\xEFV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15aY\xD6W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aZ&W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aZ\x01V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10aZ\\WaZ\\aZ2V[\x91\x90R\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aZwW`\0\x80\xFD[\x835\x92P` \x84\x015aZ\x89\x81aW\xEFV[\x91P`@\x84\x015aZ\x99\x81aW\xEFV[\x80\x91PP\x92P\x92P\x92V[`\xFF\x81\x16\x81\x14aVhW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aZ\xCEW`\0\x80\xFD[\x875aZ\xD9\x81aW\xEFV[\x96P` \x88\x015aZ\xE9\x81aW\xEFV[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a[\x07\x81aZ\xA4V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a[7W`\0\x80\xFD[\x825a[B\x81aW\xEFV[\x91P` \x83\x015aY\xB8\x81aW\xEFV[`\x02\x81\x10aVhWaVhaZ2V[` \x81\x01aZ\\\x83a[RV[`\0\x80`@\x83\x85\x03\x12\x15a[\x82W`\0\x80\xFD[\x825a[\x8D\x81aW\xEFV[\x91P` \x83\x015`\x03\x81\x10aY\xB8W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a[\xF5Wa[\xF5a[\xCDV[P`\x01\x01\x90V[`\0` \x82\x84\x03\x12\x15a\\\x0EW`\0\x80\xFD[\x81Qa\x0F\x94\x81aW\xEFV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\\[W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\\6V[PPP\x83\x81\x03\x82\x85\x01R\x85Q\x80\x82R\x86\x83\x01\x91\x83\x01\x90`\0[\x81\x81\x10\x15a\\\x90W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\\tV[PP`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x86\x01R\x92Pa\x0EZ\x91PPV[`\0` \x82\x84\x03\x12\x15a\\\xBDW`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\\\xD8W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a12WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[`@\x81\x01a]>\x84a[RV[\x83\x82Ra]J\x83a[RV[\x82` \x83\x01R\x93\x92PPPV[` \x80\x82R`\n\x90\x82\x01RiREENTRANCY`\xB0\x1B`@\x82\x01R``\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a]\xA8Wa]\xA8a[\xCDV[P\x02\x90V[`\0\x82a]\xCAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0` \x82\x84\x03\x12\x15a]\xE1W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F\x94W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a^\x03W`\0\x80\xFD[\x81Qa\x0F\x94\x81aZ\xA4V[a^\x17\x87a[RV[\x86\x81R`\xA0` \x82\x01R\x84`\xA0\x82\x01R\x84\x86`\xC0\x83\x017`\0`\xC0\x86\x83\x01\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`@\x83\x01R\x92\x84\x16``\x82\x01R\x92\x16`\x80\x83\x01R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x92\x91PPV[`\0\x81a^zWa^za[\xCDV[P`\0\x19\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15a^\xBDW\x81`\0\x19\x04\x82\x11\x15a^\xA3Wa^\xA3a[\xCDV[\x80\x85\x16\x15a^\xB0W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a^\x87V[P\x92P\x92\x90PV[`\0\x82a^\xD4WP`\x01a\x0E\x0FV[\x81a^\xE1WP`\0a\x0E\x0FV[\x81`\x01\x81\x14a^\xF7W`\x02\x81\x14a_\x01Wa_\x1DV[`\x01\x91PPa\x0E\x0FV[`\xFF\x84\x11\x15a_\x12Wa_\x12a[\xCDV[PP`\x01\x82\x1Ba\x0E\x0FV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a_@WP\x81\x81\na\x0E\x0FV[a_J\x83\x83a^\x82V[\x80`\0\x19\x04\x82\x11\x15a_^Wa_^a[\xCDV[\x02\x93\x92PPPV[`\0a\x0F\x94`\xFF\x84\x16\x83a^\xC5V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0E\x0FWa\x0E\x0Fa[\xCDV[`\0\x82Qa_\xB6\x81\x84` \x87\x01aW\x7FV[\x91\x90\x91\x01\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xD6\xFD\xB4[\x05\xC0\x87\xF2\xCCR:\n&!E\xEB\xFA\x92\xF9\xAF\x0F\xC3a\x1F\xEF\xE8\x81\xAF\x05\x8B\xF5=dsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static CELLARV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CellarV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CellarV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CellarV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CellarV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CellarV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CellarV1)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CellarV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CELLARV1_ABI.clone(),
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
                CELLARV1_ABI.clone(),
                CELLARV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAXIMUM_SHARE_LOCK_PERIOD` (0x0402ab63) function
        pub fn maximum_share_lock_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 2, 171, 99], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_FEE_CUT` (0xeef33eca) function
        pub fn max_fee_cut(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([238, 243, 62, 202], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_PERFORMANCE_FEE` (0xbdca9165) function
        pub fn max_performance_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([189, 202, 145, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_PLATFORM_FEE` (0x3998a681) function
        pub fn max_platform_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([57, 152, 166, 129], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_POSITIONS` (0xf7b24e08) function
        pub fn max_positions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([247, 178, 78, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_REBALANCE_DEVIATION` (0x6ff1c02a) function
        pub fn max_rebalance_deviation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([111, 241, 192, 42], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MINIMUM_SHARE_LOCK_PERIOD` (0x0051a3b7) function
        pub fn minimum_share_lock_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 81, 163, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PRICE_ROUTER_REGISTRY_SLOT` (0x5a400d25) function
        pub fn price_router_registry_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 64, 13, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPosition` (0x4eca8a83) function
        pub fn add_position(
            &self,
            index: ::ethers::core::types::U256,
            position: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 202, 138, 131], (index, position))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowedRebalanceDeviation` (0xc244245a) function
        pub fn allowed_rebalance_deviation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 68, 36, 90], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToAssets` (0x07a2d13a) function
        pub fn convert_to_assets(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToShares` (0xc6e6f592) function
        pub fn convert_to_shares(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseAllowance` (0xa457c2d7) function
        pub fn decrease_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            subtracted_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x6e553f65) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositLimit` (0xecf70858) function
        pub fn deposit_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 247, 8, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feeData` (0xe753e600) function
        pub fn fee_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                u64,
                u64,
                u64,
                u64,
                [u8; 32],
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([231, 83, 230, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositionType` (0x9e35c65b) function
        pub fn get_position_type(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([158, 53, 198, 91], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositions` (0x80275860) function
        pub fn get_positions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([128, 39, 88, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `holdingPosition` (0x9c5f00c2) function
        pub fn holding_position(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([156, 95, 0, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseAllowance` (0x39509351) function
        pub fn increase_allowance(
            &self,
            spender: ::ethers::core::types::Address,
            added_value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initiateShutdown` (0x0a680e18) function
        pub fn initiate_shutdown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 104, 14, 24], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPositionUsed` (0x472090fe) function
        pub fn is_position_used(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([71, 32, 144, 254], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isShutdown` (0xbf86d690) function
        pub fn is_shutdown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([191, 134, 214, 144], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isTrusted` (0x96d64879) function
        pub fn is_trusted(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([150, 214, 72, 121], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastAccrual` (0x7b3baab4) function
        pub fn last_accrual(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([123, 59, 170, 180], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liftShutdown` (0x5e2c576e) function
        pub fn lift_shutdown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 44, 87, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityLimit` (0x72163715) function
        pub fn liquidity_limit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 22, 55, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxDeposit` (0x402d267d) function
        pub fn max_deposit(
            &self,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], receiver)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxMint` (0xc63d75b6) function
        pub fn max_mint(
            &self,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], receiver)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxRedeem` (0xd905777e) function
        pub fn max_redeem(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([217, 5, 119, 126], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxWithdraw` (0xce96cb77) function
        pub fn max_withdraw(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 150, 203, 119], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x94bf804d) function
        pub fn mint(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([148, 191, 128, 77], (shares, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `positions` (0x99fbab88) function
        pub fn positions(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([153, 251, 171, 136], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewDeposit` (0xef8b30f7) function
        pub fn preview_deposit(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewMint` (0xb3d7f6b9) function
        pub fn preview_mint(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 215, 246, 185], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewRedeem` (0x4cdad506) function
        pub fn preview_redeem(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 218, 213, 6], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewWithdraw` (0x0a28a477) function
        pub fn preview_withdraw(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 40, 164, 119], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pushPosition` (0xfdd230b9) function
        pub fn push_position(
            &self,
            position: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 210, 48, 185], position)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rebalance` (0x389a7294) function
        pub fn rebalance(
            &self,
            from_position: ::ethers::core::types::Address,
            to_position: ::ethers::core::types::Address,
            assets_from: ::ethers::core::types::U256,
            exchange: u8,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [56, 154, 114, 148],
                    (from_position, to_position, assets_from, exchange, params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xba087652) function
        pub fn redeem(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registry` (0x7b103999) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([123, 16, 57, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePosition` (0xc0467422) function
        pub fn remove_position(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 70, 116, 34], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetHighWatermark` (0xc85e5e13) function
        pub fn reset_high_watermark(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 94, 94, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendFees` (0xdff90b5b) function
        pub fn send_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setDepositLimit` (0xbdc8144b) function
        pub fn set_deposit_limit(
            &self,
            new_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([189, 200, 20, 75], new_limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFeesDistributor` (0x6e85f183) function
        pub fn set_fees_distributor(
            &self,
            new_fees_distributor: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 133, 241, 131], new_fees_distributor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHoldingPosition` (0x8b0cebf7) function
        pub fn set_holding_position(
            &self,
            new_holding_position: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 12, 235, 247], new_holding_position)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setLiquidityLimit` (0xdf05a52a) function
        pub fn set_liquidity_limit(
            &self,
            new_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 5, 165, 42], new_limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPerformanceFee` (0x3cf99a46) function
        pub fn set_performance_fee(
            &self,
            new_performance_fee: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 249, 154, 70], new_performance_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPlatformFee` (0x70af7df6) function
        pub fn set_platform_fee(
            &self,
            new_platform_fee: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 175, 125, 246], new_platform_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRebalanceDeviation` (0x530a3714) function
        pub fn set_rebalance_deviation(
            &self,
            new_deviation: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 10, 55, 20], new_deviation)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setShareLockPeriod` (0x9c552ca8) function
        pub fn set_share_lock_period(
            &self,
            new_lock: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 85, 44, 168], new_lock)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrategistPayoutAddress` (0xb0a75d36) function
        pub fn set_strategist_payout_address(
            &self,
            payout: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 167, 93, 54], payout)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrategistPerformanceCut` (0x9b6fd18e) function
        pub fn set_strategist_performance_cut(
            &self,
            cut: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 111, 209, 142], cut)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setStrategistPlatformCut` (0xb5292a99) function
        pub fn set_strategist_platform_cut(
            &self,
            cut: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 41, 42, 153], cut)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setWithdrawType` (0x2f3b5a13) function
        pub fn set_withdraw_type(
            &self,
            new_withdraw_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 59, 90, 19], new_withdraw_type)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shareLockPeriod` (0x9fdb11b6) function
        pub fn share_lock_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([159, 219, 17, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapPositions` (0x58384573) function
        pub fn swap_positions(
            &self,
            index_1: ::ethers::core::types::U256,
            index_2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 56, 69, 115], (index_1, index_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalAssets` (0x01e1d114) function
        pub fn total_assets(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 225, 209, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalAssetsWithdrawable` (0xa8144e48) function
        pub fn total_assets_withdrawable(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([168, 20, 78, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trustPosition` (0xfc4d43be) function
        pub fn trust_position(
            &self,
            position: ::ethers::core::types::Address,
            position_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 77, 67, 190], (position, position_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userShareLockStartBlock` (0xfc444591) function
        pub fn user_share_lock_start_block(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([252, 68, 69, 145], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xb460af94) function
        pub fn withdraw(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 96, 175, 148], (assets, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawType` (0xe39448e0) function
        pub fn withdraw_type(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([227, 148, 72, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositLimitChanged` event
        pub fn deposit_limit_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositLimitChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeesDistributorChanged` event
        pub fn fees_distributor_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeesDistributorChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `HighWatermarkReset` event
        pub fn high_watermark_reset_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HighWatermarkResetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `HoldingPositionChanged` event
        pub fn holding_position_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            HoldingPositionChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `LiquidityLimitChanged` event
        pub fn liquidity_limit_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LiquidityLimitChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnerUpdated` event
        pub fn owner_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnerUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PerformanceFeeChanged` event
        pub fn performance_fee_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PerformanceFeeChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PlatformFeeChanged` event
        pub fn platform_fee_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PlatformFeeChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionAdded` event
        pub fn position_added_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionAddedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionRemoved` event
        pub fn position_removed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionRemovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PositionSwapped` event
        pub fn position_swapped_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionSwappedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `PulledFromPosition` event
        pub fn pulled_from_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PulledFromPositionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Rebalance` event
        pub fn rebalance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RebalanceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RebalanceDeviationChanged` event
        pub fn rebalance_deviation_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RebalanceDeviationChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SendFees` event
        pub fn send_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SendFeesFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ShareLockingPeriodChanged` event
        pub fn share_locking_period_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ShareLockingPeriodChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ShutdownChanged` event
        pub fn shutdown_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ShutdownChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategistPayoutAddressChanged` event
        pub fn strategist_payout_address_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategistPayoutAddressChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategistPerformanceCutChanged` event
        pub fn strategist_performance_cut_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategistPerformanceCutChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StrategistPlatformCutChanged` event
        pub fn strategist_platform_cut_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StrategistPlatformCutChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TrustChanged` event
        pub fn trust_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TrustChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `WithdrawTypeChanged` event
        pub fn withdraw_type_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawTypeChangedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CellarV1Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CellarV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `Cellar__AssetMismatch` with signature `Cellar__AssetMismatch(address,address)` and selector `0x5308e78e`
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
        name = "Cellar__AssetMismatch",
        abi = "Cellar__AssetMismatch(address,address)"
    )]
    pub struct Cellar__AssetMismatch {
        pub asset: ::ethers::core::types::Address,
        pub expected_asset: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__ContractNotShutdown` with signature `Cellar__ContractNotShutdown()` and selector `0xec7165bf`
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
        name = "Cellar__ContractNotShutdown",
        abi = "Cellar__ContractNotShutdown()"
    )]
    pub struct Cellar__ContractNotShutdown;
    ///Custom Error type `Cellar__ContractShutdown` with signature `Cellar__ContractShutdown()` and selector `0x6f4a665a`
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
    #[etherror(name = "Cellar__ContractShutdown", abi = "Cellar__ContractShutdown()")]
    pub struct Cellar__ContractShutdown;
    ///Custom Error type `Cellar__DepositRestricted` with signature `Cellar__DepositRestricted(uint256,uint256)` and selector `0xb487ae1c`
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
        name = "Cellar__DepositRestricted",
        abi = "Cellar__DepositRestricted(uint256,uint256)"
    )]
    pub struct Cellar__DepositRestricted {
        pub assets: ::ethers::core::types::U256,
        pub max_deposit: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__IlliquidWithdraw` with signature `Cellar__IlliquidWithdraw(address)` and selector `0xf2018f6f`
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
        name = "Cellar__IlliquidWithdraw",
        abi = "Cellar__IlliquidWithdraw(address)"
    )]
    pub struct Cellar__IlliquidWithdraw {
        pub illiquid_position: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__IncompleteWithdraw` with signature `Cellar__IncompleteWithdraw(uint256)` and selector `0xcc5ea39b`
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
        name = "Cellar__IncompleteWithdraw",
        abi = "Cellar__IncompleteWithdraw(uint256)"
    )]
    pub struct Cellar__IncompleteWithdraw {
        pub assets_owed: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__InvalidCosmosAddress` with signature `Cellar__InvalidCosmosAddress()` and selector `0x066d78c4`
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
        name = "Cellar__InvalidCosmosAddress",
        abi = "Cellar__InvalidCosmosAddress()"
    )]
    pub struct Cellar__InvalidCosmosAddress;
    ///Custom Error type `Cellar__InvalidFee` with signature `Cellar__InvalidFee()` and selector `0x5c2a50da`
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
    #[etherror(name = "Cellar__InvalidFee", abi = "Cellar__InvalidFee()")]
    pub struct Cellar__InvalidFee;
    ///Custom Error type `Cellar__InvalidFeeCut` with signature `Cellar__InvalidFeeCut()` and selector `0x3d0203e5`
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
    #[etherror(name = "Cellar__InvalidFeeCut", abi = "Cellar__InvalidFeeCut()")]
    pub struct Cellar__InvalidFeeCut;
    ///Custom Error type `Cellar__InvalidPosition` with signature `Cellar__InvalidPosition(address)` and selector `0x1ec7bcd3`
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
        name = "Cellar__InvalidPosition",
        abi = "Cellar__InvalidPosition(address)"
    )]
    pub struct Cellar__InvalidPosition {
        pub position: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__InvalidRebalanceDeviation` with signature `Cellar__InvalidRebalanceDeviation(uint256,uint256)` and selector `0x5a5521e0`
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
        name = "Cellar__InvalidRebalanceDeviation",
        abi = "Cellar__InvalidRebalanceDeviation(uint256,uint256)"
    )]
    pub struct Cellar__InvalidRebalanceDeviation {
        pub requested: ::ethers::core::types::U256,
        pub max: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__InvalidShareLockPeriod` with signature `Cellar__InvalidShareLockPeriod()` and selector `0xe9808cfc`
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
        name = "Cellar__InvalidShareLockPeriod",
        abi = "Cellar__InvalidShareLockPeriod()"
    )]
    pub struct Cellar__InvalidShareLockPeriod;
    ///Custom Error type `Cellar__NotApprovedToDepositOnBehalf` with signature `Cellar__NotApprovedToDepositOnBehalf(address)` and selector `0xd21c7c94`
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
        name = "Cellar__NotApprovedToDepositOnBehalf",
        abi = "Cellar__NotApprovedToDepositOnBehalf(address)"
    )]
    pub struct Cellar__NotApprovedToDepositOnBehalf {
        pub depositor: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__PayoutNotSet` with signature `Cellar__PayoutNotSet()` and selector `0xdc136113`
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
    #[etherror(name = "Cellar__PayoutNotSet", abi = "Cellar__PayoutNotSet()")]
    pub struct Cellar__PayoutNotSet;
    ///Custom Error type `Cellar__PositionAlreadyUsed` with signature `Cellar__PositionAlreadyUsed(address)` and selector `0x57b3ec88`
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
        name = "Cellar__PositionAlreadyUsed",
        abi = "Cellar__PositionAlreadyUsed(address)"
    )]
    pub struct Cellar__PositionAlreadyUsed {
        pub position: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__PositionArrayFull` with signature `Cellar__PositionArrayFull(uint256)` and selector `0xf025236d`
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
        name = "Cellar__PositionArrayFull",
        abi = "Cellar__PositionArrayFull(uint256)"
    )]
    pub struct Cellar__PositionArrayFull {
        pub max_positions: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__PositionNotEmpty` with signature `Cellar__PositionNotEmpty(address,uint256)` and selector `0x8454689f`
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
        name = "Cellar__PositionNotEmpty",
        abi = "Cellar__PositionNotEmpty(address,uint256)"
    )]
    pub struct Cellar__PositionNotEmpty {
        pub position: ::ethers::core::types::Address,
        pub shares_remaining: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__PositionPricingNotSetUp` with signature `Cellar__PositionPricingNotSetUp(address)` and selector `0xfd6addb6`
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
        name = "Cellar__PositionPricingNotSetUp",
        abi = "Cellar__PositionPricingNotSetUp(address)"
    )]
    pub struct Cellar__PositionPricingNotSetUp {
        pub position: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__RemoveHoldingPosition` with signature `Cellar__RemoveHoldingPosition()` and selector `0xe8ef2bfe`
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
        name = "Cellar__RemoveHoldingPosition",
        abi = "Cellar__RemoveHoldingPosition()"
    )]
    pub struct Cellar__RemoveHoldingPosition;
    ///Custom Error type `Cellar__SharesAreLocked` with signature `Cellar__SharesAreLocked(uint256,uint256)` and selector `0x1be3b8fc`
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
        name = "Cellar__SharesAreLocked",
        abi = "Cellar__SharesAreLocked(uint256,uint256)"
    )]
    pub struct Cellar__SharesAreLocked {
        pub block_shares_are_unlocked: ::ethers::core::types::U256,
        pub current_block: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__TotalAssetDeviatedOutsideRange` with signature `Cellar__TotalAssetDeviatedOutsideRange(uint256,uint256,uint256)` and selector `0xc51988ea`
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
        name = "Cellar__TotalAssetDeviatedOutsideRange",
        abi = "Cellar__TotalAssetDeviatedOutsideRange(uint256,uint256,uint256)"
    )]
    pub struct Cellar__TotalAssetDeviatedOutsideRange {
        pub assets: ::ethers::core::types::U256,
        pub min: ::ethers::core::types::U256,
        pub max: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__TotalSharesMustRemainConstant` with signature `Cellar__TotalSharesMustRemainConstant(uint256,uint256)` and selector `0xad005164`
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
        name = "Cellar__TotalSharesMustRemainConstant",
        abi = "Cellar__TotalSharesMustRemainConstant(uint256,uint256)"
    )]
    pub struct Cellar__TotalSharesMustRemainConstant {
        pub current: ::ethers::core::types::U256,
        pub expected: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__UntrustedPosition` with signature `Cellar__UntrustedPosition(address)` and selector `0xd33ecd62`
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
        name = "Cellar__UntrustedPosition",
        abi = "Cellar__UntrustedPosition(address)"
    )]
    pub struct Cellar__UntrustedPosition {
        pub position: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__WrongSwapParams` with signature `Cellar__WrongSwapParams()` and selector `0x2a484ed4`
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
    #[etherror(name = "Cellar__WrongSwapParams", abi = "Cellar__WrongSwapParams()")]
    pub struct Cellar__WrongSwapParams;
    ///Custom Error type `Cellar__ZeroAssets` with signature `Cellar__ZeroAssets()` and selector `0x97683005`
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
    #[etherror(name = "Cellar__ZeroAssets", abi = "Cellar__ZeroAssets()")]
    pub struct Cellar__ZeroAssets;
    ///Custom Error type `Cellar__ZeroShares` with signature `Cellar__ZeroShares()` and selector `0x84de2a6e`
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
    #[etherror(name = "Cellar__ZeroShares", abi = "Cellar__ZeroShares()")]
    pub struct Cellar__ZeroShares;
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
    pub enum CellarV1Errors {
        Cellar__AssetMismatch(Cellar__AssetMismatch),
        Cellar__ContractNotShutdown(Cellar__ContractNotShutdown),
        Cellar__ContractShutdown(Cellar__ContractShutdown),
        Cellar__DepositRestricted(Cellar__DepositRestricted),
        Cellar__IlliquidWithdraw(Cellar__IlliquidWithdraw),
        Cellar__IncompleteWithdraw(Cellar__IncompleteWithdraw),
        Cellar__InvalidCosmosAddress(Cellar__InvalidCosmosAddress),
        Cellar__InvalidFee(Cellar__InvalidFee),
        Cellar__InvalidFeeCut(Cellar__InvalidFeeCut),
        Cellar__InvalidPosition(Cellar__InvalidPosition),
        Cellar__InvalidRebalanceDeviation(Cellar__InvalidRebalanceDeviation),
        Cellar__InvalidShareLockPeriod(Cellar__InvalidShareLockPeriod),
        Cellar__NotApprovedToDepositOnBehalf(Cellar__NotApprovedToDepositOnBehalf),
        Cellar__PayoutNotSet(Cellar__PayoutNotSet),
        Cellar__PositionAlreadyUsed(Cellar__PositionAlreadyUsed),
        Cellar__PositionArrayFull(Cellar__PositionArrayFull),
        Cellar__PositionNotEmpty(Cellar__PositionNotEmpty),
        Cellar__PositionPricingNotSetUp(Cellar__PositionPricingNotSetUp),
        Cellar__RemoveHoldingPosition(Cellar__RemoveHoldingPosition),
        Cellar__SharesAreLocked(Cellar__SharesAreLocked),
        Cellar__TotalAssetDeviatedOutsideRange(Cellar__TotalAssetDeviatedOutsideRange),
        Cellar__TotalSharesMustRemainConstant(Cellar__TotalSharesMustRemainConstant),
        Cellar__UntrustedPosition(Cellar__UntrustedPosition),
        Cellar__WrongSwapParams(Cellar__WrongSwapParams),
        Cellar__ZeroAssets(Cellar__ZeroAssets),
        Cellar__ZeroShares(Cellar__ZeroShares),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CellarV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Cellar__AssetMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__AssetMismatch(decoded));
            }
            if let Ok(decoded) = <Cellar__ContractNotShutdown as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ContractNotShutdown(decoded));
            }
            if let Ok(decoded) = <Cellar__ContractShutdown as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ContractShutdown(decoded));
            }
            if let Ok(decoded) = <Cellar__DepositRestricted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__DepositRestricted(decoded));
            }
            if let Ok(decoded) = <Cellar__IlliquidWithdraw as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__IlliquidWithdraw(decoded));
            }
            if let Ok(decoded) = <Cellar__IncompleteWithdraw as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__IncompleteWithdraw(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidCosmosAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidCosmosAddress(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidFee as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidFee(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidFeeCut as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidFeeCut(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidPosition(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidRebalanceDeviation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidRebalanceDeviation(decoded));
            }
            if let Ok(decoded) = <Cellar__InvalidShareLockPeriod as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <Cellar__NotApprovedToDepositOnBehalf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__NotApprovedToDepositOnBehalf(decoded));
            }
            if let Ok(decoded) = <Cellar__PayoutNotSet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PayoutNotSet(decoded));
            }
            if let Ok(decoded) = <Cellar__PositionAlreadyUsed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionAlreadyUsed(decoded));
            }
            if let Ok(decoded) = <Cellar__PositionArrayFull as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionArrayFull(decoded));
            }
            if let Ok(decoded) = <Cellar__PositionNotEmpty as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionNotEmpty(decoded));
            }
            if let Ok(decoded) = <Cellar__PositionPricingNotSetUp as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionPricingNotSetUp(decoded));
            }
            if let Ok(decoded) = <Cellar__RemoveHoldingPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__RemoveHoldingPosition(decoded));
            }
            if let Ok(decoded) = <Cellar__SharesAreLocked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__SharesAreLocked(decoded));
            }
            if let Ok(decoded) = <Cellar__TotalAssetDeviatedOutsideRange as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__TotalAssetDeviatedOutsideRange(decoded));
            }
            if let Ok(decoded) = <Cellar__TotalSharesMustRemainConstant as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__TotalSharesMustRemainConstant(decoded));
            }
            if let Ok(decoded) = <Cellar__UntrustedPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__UntrustedPosition(decoded));
            }
            if let Ok(decoded) = <Cellar__WrongSwapParams as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__WrongSwapParams(decoded));
            }
            if let Ok(decoded) = <Cellar__ZeroAssets as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ZeroAssets(decoded));
            }
            if let Ok(decoded) = <Cellar__ZeroShares as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ZeroShares(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CellarV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Cellar__AssetMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ContractNotShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ContractShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__DepositRestricted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__IlliquidWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__IncompleteWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidCosmosAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidFeeCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidRebalanceDeviation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__NotApprovedToDepositOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PayoutNotSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PositionAlreadyUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PositionArrayFull(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PositionNotEmpty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PositionPricingNotSetUp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__RemoveHoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__SharesAreLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__TotalAssetDeviatedOutsideRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__TotalSharesMustRemainConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__UntrustedPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__WrongSwapParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ZeroAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ZeroShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CellarV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Cellar__AssetMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ContractNotShutdown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ContractShutdown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__DepositRestricted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__IlliquidWithdraw as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__IncompleteWithdraw as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidCosmosAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidFeeCut as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidRebalanceDeviation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidShareLockPeriod as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__NotApprovedToDepositOnBehalf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PayoutNotSet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PositionAlreadyUsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PositionArrayFull as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PositionNotEmpty as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PositionPricingNotSetUp as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__RemoveHoldingPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__SharesAreLocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__TotalAssetDeviatedOutsideRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__TotalSharesMustRemainConstant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__UntrustedPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__WrongSwapParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ZeroAssets as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ZeroShares as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CellarV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cellar__AssetMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ContractNotShutdown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ContractShutdown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__DepositRestricted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__IlliquidWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__IncompleteWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidCosmosAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidFeeCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__NotApprovedToDepositOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PayoutNotSet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionAlreadyUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionArrayFull(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionNotEmpty(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionPricingNotSetUp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__RemoveHoldingPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__SharesAreLocked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__TotalAssetDeviatedOutsideRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__TotalSharesMustRemainConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__UntrustedPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__WrongSwapParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ZeroAssets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ZeroShares(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CellarV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Cellar__AssetMismatch> for CellarV1Errors {
        fn from(value: Cellar__AssetMismatch) -> Self {
            Self::Cellar__AssetMismatch(value)
        }
    }
    impl ::core::convert::From<Cellar__ContractNotShutdown> for CellarV1Errors {
        fn from(value: Cellar__ContractNotShutdown) -> Self {
            Self::Cellar__ContractNotShutdown(value)
        }
    }
    impl ::core::convert::From<Cellar__ContractShutdown> for CellarV1Errors {
        fn from(value: Cellar__ContractShutdown) -> Self {
            Self::Cellar__ContractShutdown(value)
        }
    }
    impl ::core::convert::From<Cellar__DepositRestricted> for CellarV1Errors {
        fn from(value: Cellar__DepositRestricted) -> Self {
            Self::Cellar__DepositRestricted(value)
        }
    }
    impl ::core::convert::From<Cellar__IlliquidWithdraw> for CellarV1Errors {
        fn from(value: Cellar__IlliquidWithdraw) -> Self {
            Self::Cellar__IlliquidWithdraw(value)
        }
    }
    impl ::core::convert::From<Cellar__IncompleteWithdraw> for CellarV1Errors {
        fn from(value: Cellar__IncompleteWithdraw) -> Self {
            Self::Cellar__IncompleteWithdraw(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidCosmosAddress> for CellarV1Errors {
        fn from(value: Cellar__InvalidCosmosAddress) -> Self {
            Self::Cellar__InvalidCosmosAddress(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidFee> for CellarV1Errors {
        fn from(value: Cellar__InvalidFee) -> Self {
            Self::Cellar__InvalidFee(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidFeeCut> for CellarV1Errors {
        fn from(value: Cellar__InvalidFeeCut) -> Self {
            Self::Cellar__InvalidFeeCut(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidPosition> for CellarV1Errors {
        fn from(value: Cellar__InvalidPosition) -> Self {
            Self::Cellar__InvalidPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidRebalanceDeviation> for CellarV1Errors {
        fn from(value: Cellar__InvalidRebalanceDeviation) -> Self {
            Self::Cellar__InvalidRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidShareLockPeriod> for CellarV1Errors {
        fn from(value: Cellar__InvalidShareLockPeriod) -> Self {
            Self::Cellar__InvalidShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<Cellar__NotApprovedToDepositOnBehalf> for CellarV1Errors {
        fn from(value: Cellar__NotApprovedToDepositOnBehalf) -> Self {
            Self::Cellar__NotApprovedToDepositOnBehalf(value)
        }
    }
    impl ::core::convert::From<Cellar__PayoutNotSet> for CellarV1Errors {
        fn from(value: Cellar__PayoutNotSet) -> Self {
            Self::Cellar__PayoutNotSet(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionAlreadyUsed> for CellarV1Errors {
        fn from(value: Cellar__PositionAlreadyUsed) -> Self {
            Self::Cellar__PositionAlreadyUsed(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionArrayFull> for CellarV1Errors {
        fn from(value: Cellar__PositionArrayFull) -> Self {
            Self::Cellar__PositionArrayFull(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionNotEmpty> for CellarV1Errors {
        fn from(value: Cellar__PositionNotEmpty) -> Self {
            Self::Cellar__PositionNotEmpty(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionPricingNotSetUp> for CellarV1Errors {
        fn from(value: Cellar__PositionPricingNotSetUp) -> Self {
            Self::Cellar__PositionPricingNotSetUp(value)
        }
    }
    impl ::core::convert::From<Cellar__RemoveHoldingPosition> for CellarV1Errors {
        fn from(value: Cellar__RemoveHoldingPosition) -> Self {
            Self::Cellar__RemoveHoldingPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__SharesAreLocked> for CellarV1Errors {
        fn from(value: Cellar__SharesAreLocked) -> Self {
            Self::Cellar__SharesAreLocked(value)
        }
    }
    impl ::core::convert::From<Cellar__TotalAssetDeviatedOutsideRange>
    for CellarV1Errors {
        fn from(value: Cellar__TotalAssetDeviatedOutsideRange) -> Self {
            Self::Cellar__TotalAssetDeviatedOutsideRange(value)
        }
    }
    impl ::core::convert::From<Cellar__TotalSharesMustRemainConstant>
    for CellarV1Errors {
        fn from(value: Cellar__TotalSharesMustRemainConstant) -> Self {
            Self::Cellar__TotalSharesMustRemainConstant(value)
        }
    }
    impl ::core::convert::From<Cellar__UntrustedPosition> for CellarV1Errors {
        fn from(value: Cellar__UntrustedPosition) -> Self {
            Self::Cellar__UntrustedPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__WrongSwapParams> for CellarV1Errors {
        fn from(value: Cellar__WrongSwapParams) -> Self {
            Self::Cellar__WrongSwapParams(value)
        }
    }
    impl ::core::convert::From<Cellar__ZeroAssets> for CellarV1Errors {
        fn from(value: Cellar__ZeroAssets) -> Self {
            Self::Cellar__ZeroAssets(value)
        }
    }
    impl ::core::convert::From<Cellar__ZeroShares> for CellarV1Errors {
        fn from(value: Cellar__ZeroShares) -> Self {
            Self::Cellar__ZeroShares(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DepositLimitChanged",
        abi = "DepositLimitChanged(uint256,uint256)"
    )]
    pub struct DepositLimitChangedFilter {
        pub old_limit: ::ethers::core::types::U256,
        pub new_limit: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FeesDistributorChanged",
        abi = "FeesDistributorChanged(bytes32,bytes32)"
    )]
    pub struct FeesDistributorChangedFilter {
        pub old_fees_distributor: [u8; 32],
        pub new_fees_distributor: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "HighWatermarkReset", abi = "HighWatermarkReset(uint256)")]
    pub struct HighWatermarkResetFilter {
        pub new_high_watermark: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "HoldingPositionChanged",
        abi = "HoldingPositionChanged(address,address)"
    )]
    pub struct HoldingPositionChangedFilter {
        #[ethevent(indexed)]
        pub old_position: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_position: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "LiquidityLimitChanged",
        abi = "LiquidityLimitChanged(uint256,uint256)"
    )]
    pub struct LiquidityLimitChangedFilter {
        pub old_limit: ::ethers::core::types::U256,
        pub new_limit: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OwnerUpdated", abi = "OwnerUpdated(address,address)")]
    pub struct OwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PerformanceFeeChanged",
        abi = "PerformanceFeeChanged(uint64,uint64)"
    )]
    pub struct PerformanceFeeChangedFilter {
        pub old_performance_fee: u64,
        pub new_performance_fee: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PlatformFeeChanged", abi = "PlatformFeeChanged(uint64,uint64)")]
    pub struct PlatformFeeChangedFilter {
        pub old_platform_fee: u64,
        pub new_platform_fee: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PositionAdded", abi = "PositionAdded(address,uint256)")]
    pub struct PositionAddedFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PositionRemoved", abi = "PositionRemoved(address,uint256)")]
    pub struct PositionRemovedFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "PositionSwapped",
        abi = "PositionSwapped(address,address,uint256,uint256)"
    )]
    pub struct PositionSwappedFilter {
        #[ethevent(indexed)]
        pub new_position_1: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_position_2: ::ethers::core::types::Address,
        pub index_1: ::ethers::core::types::U256,
        pub index_2: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PulledFromPosition", abi = "PulledFromPosition(address,uint256)")]
    pub struct PulledFromPositionFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Rebalance", abi = "Rebalance(address,address,uint256,uint256)")]
    pub struct RebalanceFilter {
        #[ethevent(indexed)]
        pub from_position: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to_position: ::ethers::core::types::Address,
        pub assets_from: ::ethers::core::types::U256,
        pub assets_to: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RebalanceDeviationChanged",
        abi = "RebalanceDeviationChanged(uint256,uint256)"
    )]
    pub struct RebalanceDeviationChangedFilter {
        pub old_deviation: ::ethers::core::types::U256,
        pub new_deviation: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SendFees", abi = "SendFees(uint256,uint256)")]
    pub struct SendFeesFilter {
        pub fees_in_shares_redeemed: ::ethers::core::types::U256,
        pub fees_in_assets_sent: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ShareLockingPeriodChanged",
        abi = "ShareLockingPeriodChanged(uint256,uint256)"
    )]
    pub struct ShareLockingPeriodChangedFilter {
        pub old_period: ::ethers::core::types::U256,
        pub new_period: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ShutdownChanged", abi = "ShutdownChanged(bool)")]
    pub struct ShutdownChangedFilter {
        pub is_shutdown: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "StrategistPayoutAddressChanged",
        abi = "StrategistPayoutAddressChanged(address,address)"
    )]
    pub struct StrategistPayoutAddressChangedFilter {
        pub old_payout_address: ::ethers::core::types::Address,
        pub new_payout_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "StrategistPerformanceCutChanged",
        abi = "StrategistPerformanceCutChanged(uint64,uint64)"
    )]
    pub struct StrategistPerformanceCutChangedFilter {
        pub old_performance_cut: u64,
        pub new_performance_cut: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "StrategistPlatformCutChanged",
        abi = "StrategistPlatformCutChanged(uint64,uint64)"
    )]
    pub struct StrategistPlatformCutChangedFilter {
        pub old_platform_cut: u64,
        pub new_platform_cut: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "TrustChanged", abi = "TrustChanged(address,bool)")]
    pub struct TrustChangedFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub is_trusted: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "WithdrawTypeChanged", abi = "WithdrawTypeChanged(uint8,uint8)")]
    pub struct WithdrawTypeChangedFilter {
        pub old_type: u8,
        pub new_type: u8,
    }
    ///Container type for all of the contract's events
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
    pub enum CellarV1Events {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        DepositLimitChangedFilter(DepositLimitChangedFilter),
        FeesDistributorChangedFilter(FeesDistributorChangedFilter),
        HighWatermarkResetFilter(HighWatermarkResetFilter),
        HoldingPositionChangedFilter(HoldingPositionChangedFilter),
        LiquidityLimitChangedFilter(LiquidityLimitChangedFilter),
        OwnerUpdatedFilter(OwnerUpdatedFilter),
        PerformanceFeeChangedFilter(PerformanceFeeChangedFilter),
        PlatformFeeChangedFilter(PlatformFeeChangedFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        PulledFromPositionFilter(PulledFromPositionFilter),
        RebalanceFilter(RebalanceFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        SendFeesFilter(SendFeesFilter),
        ShareLockingPeriodChangedFilter(ShareLockingPeriodChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPerformanceCutChangedFilter(StrategistPerformanceCutChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        TrustChangedFilter(TrustChangedFilter),
        WithdrawFilter(WithdrawFilter),
        WithdrawTypeChangedFilter(WithdrawTypeChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for CellarV1Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarV1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositLimitChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::DepositLimitChangedFilter(decoded));
            }
            if let Ok(decoded) = FeesDistributorChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::FeesDistributorChangedFilter(decoded));
            }
            if let Ok(decoded) = HighWatermarkResetFilter::decode_log(log) {
                return Ok(CellarV1Events::HighWatermarkResetFilter(decoded));
            }
            if let Ok(decoded) = HoldingPositionChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::HoldingPositionChangedFilter(decoded));
            }
            if let Ok(decoded) = LiquidityLimitChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::LiquidityLimitChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnerUpdatedFilter::decode_log(log) {
                return Ok(CellarV1Events::OwnerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PerformanceFeeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::PerformanceFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = PlatformFeeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::PlatformFeeChangedFilter(decoded));
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionAddedFilter(decoded));
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionRemovedFilter(decoded));
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(CellarV1Events::PositionSwappedFilter(decoded));
            }
            if let Ok(decoded) = PulledFromPositionFilter::decode_log(log) {
                return Ok(CellarV1Events::PulledFromPositionFilter(decoded));
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(CellarV1Events::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::RebalanceDeviationChangedFilter(decoded));
            }
            if let Ok(decoded) = SendFeesFilter::decode_log(log) {
                return Ok(CellarV1Events::SendFeesFilter(decoded));
            }
            if let Ok(decoded) = ShareLockingPeriodChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::ShareLockingPeriodChangedFilter(decoded));
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::ShutdownChangedFilter(decoded));
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::StrategistPayoutAddressChangedFilter(decoded));
            }
            if let Ok(decoded) = StrategistPerformanceCutChangedFilter::decode_log(log) {
                return Ok(
                    CellarV1Events::StrategistPerformanceCutChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::StrategistPlatformCutChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarV1Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = TrustChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::TrustChangedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarV1Events::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawTypeChangedFilter::decode_log(log) {
                return Ok(CellarV1Events::WithdrawTypeChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CellarV1Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositLimitChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesDistributorChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HighWatermarkResetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::HoldingPositionChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidityLimitChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PerformanceFeeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PlatformFeeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionSwappedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PulledFromPositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RebalanceDeviationChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShareLockingPeriodChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShutdownChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategistPayoutAddressChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategistPerformanceCutChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategistPlatformCutChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawTypeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for CellarV1Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for CellarV1Events {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<DepositLimitChangedFilter> for CellarV1Events {
        fn from(value: DepositLimitChangedFilter) -> Self {
            Self::DepositLimitChangedFilter(value)
        }
    }
    impl ::core::convert::From<FeesDistributorChangedFilter> for CellarV1Events {
        fn from(value: FeesDistributorChangedFilter) -> Self {
            Self::FeesDistributorChangedFilter(value)
        }
    }
    impl ::core::convert::From<HighWatermarkResetFilter> for CellarV1Events {
        fn from(value: HighWatermarkResetFilter) -> Self {
            Self::HighWatermarkResetFilter(value)
        }
    }
    impl ::core::convert::From<HoldingPositionChangedFilter> for CellarV1Events {
        fn from(value: HoldingPositionChangedFilter) -> Self {
            Self::HoldingPositionChangedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidityLimitChangedFilter> for CellarV1Events {
        fn from(value: LiquidityLimitChangedFilter) -> Self {
            Self::LiquidityLimitChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnerUpdatedFilter> for CellarV1Events {
        fn from(value: OwnerUpdatedFilter) -> Self {
            Self::OwnerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PerformanceFeeChangedFilter> for CellarV1Events {
        fn from(value: PerformanceFeeChangedFilter) -> Self {
            Self::PerformanceFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<PlatformFeeChangedFilter> for CellarV1Events {
        fn from(value: PlatformFeeChangedFilter) -> Self {
            Self::PlatformFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<PositionAddedFilter> for CellarV1Events {
        fn from(value: PositionAddedFilter) -> Self {
            Self::PositionAddedFilter(value)
        }
    }
    impl ::core::convert::From<PositionRemovedFilter> for CellarV1Events {
        fn from(value: PositionRemovedFilter) -> Self {
            Self::PositionRemovedFilter(value)
        }
    }
    impl ::core::convert::From<PositionSwappedFilter> for CellarV1Events {
        fn from(value: PositionSwappedFilter) -> Self {
            Self::PositionSwappedFilter(value)
        }
    }
    impl ::core::convert::From<PulledFromPositionFilter> for CellarV1Events {
        fn from(value: PulledFromPositionFilter) -> Self {
            Self::PulledFromPositionFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceFilter> for CellarV1Events {
        fn from(value: RebalanceFilter) -> Self {
            Self::RebalanceFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceDeviationChangedFilter> for CellarV1Events {
        fn from(value: RebalanceDeviationChangedFilter) -> Self {
            Self::RebalanceDeviationChangedFilter(value)
        }
    }
    impl ::core::convert::From<SendFeesFilter> for CellarV1Events {
        fn from(value: SendFeesFilter) -> Self {
            Self::SendFeesFilter(value)
        }
    }
    impl ::core::convert::From<ShareLockingPeriodChangedFilter> for CellarV1Events {
        fn from(value: ShareLockingPeriodChangedFilter) -> Self {
            Self::ShareLockingPeriodChangedFilter(value)
        }
    }
    impl ::core::convert::From<ShutdownChangedFilter> for CellarV1Events {
        fn from(value: ShutdownChangedFilter) -> Self {
            Self::ShutdownChangedFilter(value)
        }
    }
    impl ::core::convert::From<StrategistPayoutAddressChangedFilter> for CellarV1Events {
        fn from(value: StrategistPayoutAddressChangedFilter) -> Self {
            Self::StrategistPayoutAddressChangedFilter(value)
        }
    }
    impl ::core::convert::From<StrategistPerformanceCutChangedFilter>
    for CellarV1Events {
        fn from(value: StrategistPerformanceCutChangedFilter) -> Self {
            Self::StrategistPerformanceCutChangedFilter(value)
        }
    }
    impl ::core::convert::From<StrategistPlatformCutChangedFilter> for CellarV1Events {
        fn from(value: StrategistPlatformCutChangedFilter) -> Self {
            Self::StrategistPlatformCutChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for CellarV1Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TrustChangedFilter> for CellarV1Events {
        fn from(value: TrustChangedFilter) -> Self {
            Self::TrustChangedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for CellarV1Events {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawTypeChangedFilter> for CellarV1Events {
        fn from(value: WithdrawTypeChangedFilter) -> Self {
            Self::WithdrawTypeChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `MAXIMUM_SHARE_LOCK_PERIOD` function with signature `MAXIMUM_SHARE_LOCK_PERIOD()` and selector `0x0402ab63`
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
    #[ethcall(name = "MAXIMUM_SHARE_LOCK_PERIOD", abi = "MAXIMUM_SHARE_LOCK_PERIOD()")]
    pub struct MaximumShareLockPeriodCall;
    ///Container type for all input parameters for the `MAX_FEE_CUT` function with signature `MAX_FEE_CUT()` and selector `0xeef33eca`
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
    #[ethcall(name = "MAX_FEE_CUT", abi = "MAX_FEE_CUT()")]
    pub struct MaxFeeCutCall;
    ///Container type for all input parameters for the `MAX_PERFORMANCE_FEE` function with signature `MAX_PERFORMANCE_FEE()` and selector `0xbdca9165`
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
    #[ethcall(name = "MAX_PERFORMANCE_FEE", abi = "MAX_PERFORMANCE_FEE()")]
    pub struct MaxPerformanceFeeCall;
    ///Container type for all input parameters for the `MAX_PLATFORM_FEE` function with signature `MAX_PLATFORM_FEE()` and selector `0x3998a681`
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
    #[ethcall(name = "MAX_PLATFORM_FEE", abi = "MAX_PLATFORM_FEE()")]
    pub struct MaxPlatformFeeCall;
    ///Container type for all input parameters for the `MAX_POSITIONS` function with signature `MAX_POSITIONS()` and selector `0xf7b24e08`
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
    #[ethcall(name = "MAX_POSITIONS", abi = "MAX_POSITIONS()")]
    pub struct MaxPositionsCall;
    ///Container type for all input parameters for the `MAX_REBALANCE_DEVIATION` function with signature `MAX_REBALANCE_DEVIATION()` and selector `0x6ff1c02a`
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
    #[ethcall(name = "MAX_REBALANCE_DEVIATION", abi = "MAX_REBALANCE_DEVIATION()")]
    pub struct MaxRebalanceDeviationCall;
    ///Container type for all input parameters for the `MINIMUM_SHARE_LOCK_PERIOD` function with signature `MINIMUM_SHARE_LOCK_PERIOD()` and selector `0x0051a3b7`
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
    #[ethcall(name = "MINIMUM_SHARE_LOCK_PERIOD", abi = "MINIMUM_SHARE_LOCK_PERIOD()")]
    pub struct MinimumShareLockPeriodCall;
    ///Container type for all input parameters for the `PRICE_ROUTER_REGISTRY_SLOT` function with signature `PRICE_ROUTER_REGISTRY_SLOT()` and selector `0x5a400d25`
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
    #[ethcall(name = "PRICE_ROUTER_REGISTRY_SLOT", abi = "PRICE_ROUTER_REGISTRY_SLOT()")]
    pub struct PriceRouterRegistrySlotCall;
    ///Container type for all input parameters for the `addPosition` function with signature `addPosition(uint256,address)` and selector `0x4eca8a83`
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
    #[ethcall(name = "addPosition", abi = "addPosition(uint256,address)")]
    pub struct AddPositionCall {
        pub index: ::ethers::core::types::U256,
        pub position: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allowedRebalanceDeviation` function with signature `allowedRebalanceDeviation()` and selector `0xc244245a`
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
    #[ethcall(name = "allowedRebalanceDeviation", abi = "allowedRebalanceDeviation()")]
    pub struct AllowedRebalanceDeviationCall;
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    #[ethcall(name = "decreaseAllowance", abi = "decreaseAllowance(address,uint256)")]
    pub struct DecreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub subtracted_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `depositLimit` function with signature `depositLimit()` and selector `0xecf70858`
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
    #[ethcall(name = "depositLimit", abi = "depositLimit()")]
    pub struct DepositLimitCall;
    ///Container type for all input parameters for the `feeData` function with signature `feeData()` and selector `0xe753e600`
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
    #[ethcall(name = "feeData", abi = "feeData()")]
    pub struct FeeDataCall;
    ///Container type for all input parameters for the `getPositionType` function with signature `getPositionType(address)` and selector `0x9e35c65b`
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
    #[ethcall(name = "getPositionType", abi = "getPositionType(address)")]
    pub struct GetPositionTypeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `getPositions` function with signature `getPositions()` and selector `0x80275860`
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
    #[ethcall(name = "getPositions", abi = "getPositions()")]
    pub struct GetPositionsCall;
    ///Container type for all input parameters for the `holdingPosition` function with signature `holdingPosition()` and selector `0x9c5f00c2`
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
    #[ethcall(name = "holdingPosition", abi = "holdingPosition()")]
    pub struct HoldingPositionCall;
    ///Container type for all input parameters for the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    #[ethcall(name = "increaseAllowance", abi = "increaseAllowance(address,uint256)")]
    pub struct IncreaseAllowanceCall {
        pub spender: ::ethers::core::types::Address,
        pub added_value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initiateShutdown` function with signature `initiateShutdown()` and selector `0x0a680e18`
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
    #[ethcall(name = "initiateShutdown", abi = "initiateShutdown()")]
    pub struct InitiateShutdownCall;
    ///Container type for all input parameters for the `isPositionUsed` function with signature `isPositionUsed(address)` and selector `0x472090fe`
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
    #[ethcall(name = "isPositionUsed", abi = "isPositionUsed(address)")]
    pub struct IsPositionUsedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `isShutdown` function with signature `isShutdown()` and selector `0xbf86d690`
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
    #[ethcall(name = "isShutdown", abi = "isShutdown()")]
    pub struct IsShutdownCall;
    ///Container type for all input parameters for the `isTrusted` function with signature `isTrusted(address)` and selector `0x96d64879`
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
    #[ethcall(name = "isTrusted", abi = "isTrusted(address)")]
    pub struct IsTrustedCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `lastAccrual` function with signature `lastAccrual()` and selector `0x7b3baab4`
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
    #[ethcall(name = "lastAccrual", abi = "lastAccrual()")]
    pub struct LastAccrualCall;
    ///Container type for all input parameters for the `liftShutdown` function with signature `liftShutdown()` and selector `0x5e2c576e`
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
    #[ethcall(name = "liftShutdown", abi = "liftShutdown()")]
    pub struct LiftShutdownCall;
    ///Container type for all input parameters for the `liquidityLimit` function with signature `liquidityLimit()` and selector `0x72163715`
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
    #[ethcall(name = "liquidityLimit", abi = "liquidityLimit()")]
    pub struct LiquidityLimitCall;
    ///Container type for all input parameters for the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit(address)")]
    pub struct MaxDepositCall {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    #[ethcall(name = "maxMint", abi = "maxMint(address)")]
    pub struct MaxMintCall {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    #[ethcall(name = "maxRedeem", abi = "maxRedeem(address)")]
    pub struct MaxRedeemCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    #[ethcall(name = "maxWithdraw", abi = "maxWithdraw(address)")]
    pub struct MaxWithdrawCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    #[ethcall(name = "mint", abi = "mint(uint256,address)")]
    pub struct MintCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
    #[ethcall(name = "positions", abi = "positions(uint256)")]
    pub struct PositionsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    #[ethcall(name = "previewMint", abi = "previewMint(uint256)")]
    pub struct PreviewMintCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    #[ethcall(name = "previewRedeem", abi = "previewRedeem(uint256)")]
    pub struct PreviewRedeemCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    #[ethcall(name = "previewWithdraw", abi = "previewWithdraw(uint256)")]
    pub struct PreviewWithdrawCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pushPosition` function with signature `pushPosition(address)` and selector `0xfdd230b9`
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
    #[ethcall(name = "pushPosition", abi = "pushPosition(address)")]
    pub struct PushPositionCall {
        pub position: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `rebalance` function with signature `rebalance(address,address,uint256,uint8,bytes)` and selector `0x389a7294`
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
        name = "rebalance",
        abi = "rebalance(address,address,uint256,uint8,bytes)"
    )]
    pub struct RebalanceCall {
        pub from_position: ::ethers::core::types::Address,
        pub to_position: ::ethers::core::types::Address,
        pub assets_from: ::ethers::core::types::U256,
        pub exchange: u8,
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `registry` function with signature `registry()` and selector `0x7b103999`
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
    #[ethcall(name = "registry", abi = "registry()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `removePosition` function with signature `removePosition(uint256)` and selector `0xc0467422`
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
    #[ethcall(name = "removePosition", abi = "removePosition(uint256)")]
    pub struct RemovePositionCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `resetHighWatermark` function with signature `resetHighWatermark()` and selector `0xc85e5e13`
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
    #[ethcall(name = "resetHighWatermark", abi = "resetHighWatermark()")]
    pub struct ResetHighWatermarkCall;
    ///Container type for all input parameters for the `sendFees` function with signature `sendFees()` and selector `0xdff90b5b`
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
    #[ethcall(name = "sendFees", abi = "sendFees()")]
    pub struct SendFeesCall;
    ///Container type for all input parameters for the `setDepositLimit` function with signature `setDepositLimit(uint256)` and selector `0xbdc8144b`
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
    #[ethcall(name = "setDepositLimit", abi = "setDepositLimit(uint256)")]
    pub struct SetDepositLimitCall {
        pub new_limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setFeesDistributor` function with signature `setFeesDistributor(bytes32)` and selector `0x6e85f183`
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
    #[ethcall(name = "setFeesDistributor", abi = "setFeesDistributor(bytes32)")]
    pub struct SetFeesDistributorCall {
        pub new_fees_distributor: [u8; 32],
    }
    ///Container type for all input parameters for the `setHoldingPosition` function with signature `setHoldingPosition(address)` and selector `0x8b0cebf7`
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
    #[ethcall(name = "setHoldingPosition", abi = "setHoldingPosition(address)")]
    pub struct SetHoldingPositionCall {
        pub new_holding_position: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setLiquidityLimit` function with signature `setLiquidityLimit(uint256)` and selector `0xdf05a52a`
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
    #[ethcall(name = "setLiquidityLimit", abi = "setLiquidityLimit(uint256)")]
    pub struct SetLiquidityLimitCall {
        pub new_limit: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setOwner` function with signature `setOwner(address)` and selector `0x13af4035`
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
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setPerformanceFee` function with signature `setPerformanceFee(uint64)` and selector `0x3cf99a46`
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
    #[ethcall(name = "setPerformanceFee", abi = "setPerformanceFee(uint64)")]
    pub struct SetPerformanceFeeCall {
        pub new_performance_fee: u64,
    }
    ///Container type for all input parameters for the `setPlatformFee` function with signature `setPlatformFee(uint64)` and selector `0x70af7df6`
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
    #[ethcall(name = "setPlatformFee", abi = "setPlatformFee(uint64)")]
    pub struct SetPlatformFeeCall {
        pub new_platform_fee: u64,
    }
    ///Container type for all input parameters for the `setRebalanceDeviation` function with signature `setRebalanceDeviation(uint256)` and selector `0x530a3714`
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
    #[ethcall(name = "setRebalanceDeviation", abi = "setRebalanceDeviation(uint256)")]
    pub struct SetRebalanceDeviationCall {
        pub new_deviation: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setShareLockPeriod` function with signature `setShareLockPeriod(uint256)` and selector `0x9c552ca8`
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
    #[ethcall(name = "setShareLockPeriod", abi = "setShareLockPeriod(uint256)")]
    pub struct SetShareLockPeriodCall {
        pub new_lock: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setStrategistPayoutAddress` function with signature `setStrategistPayoutAddress(address)` and selector `0xb0a75d36`
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
        name = "setStrategistPayoutAddress",
        abi = "setStrategistPayoutAddress(address)"
    )]
    pub struct SetStrategistPayoutAddressCall {
        pub payout: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setStrategistPerformanceCut` function with signature `setStrategistPerformanceCut(uint64)` and selector `0x9b6fd18e`
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
        name = "setStrategistPerformanceCut",
        abi = "setStrategistPerformanceCut(uint64)"
    )]
    pub struct SetStrategistPerformanceCutCall {
        pub cut: u64,
    }
    ///Container type for all input parameters for the `setStrategistPlatformCut` function with signature `setStrategistPlatformCut(uint64)` and selector `0xb5292a99`
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
        name = "setStrategistPlatformCut",
        abi = "setStrategistPlatformCut(uint64)"
    )]
    pub struct SetStrategistPlatformCutCall {
        pub cut: u64,
    }
    ///Container type for all input parameters for the `setWithdrawType` function with signature `setWithdrawType(uint8)` and selector `0x2f3b5a13`
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
    #[ethcall(name = "setWithdrawType", abi = "setWithdrawType(uint8)")]
    pub struct SetWithdrawTypeCall {
        pub new_withdraw_type: u8,
    }
    ///Container type for all input parameters for the `shareLockPeriod` function with signature `shareLockPeriod()` and selector `0x9fdb11b6`
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
    #[ethcall(name = "shareLockPeriod", abi = "shareLockPeriod()")]
    pub struct ShareLockPeriodCall;
    ///Container type for all input parameters for the `swapPositions` function with signature `swapPositions(uint256,uint256)` and selector `0x58384573`
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
    #[ethcall(name = "swapPositions", abi = "swapPositions(uint256,uint256)")]
    pub struct SwapPositionsCall {
        pub index_1: ::ethers::core::types::U256,
        pub index_2: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    #[ethcall(name = "totalAssets", abi = "totalAssets()")]
    pub struct TotalAssetsCall;
    ///Container type for all input parameters for the `totalAssetsWithdrawable` function with signature `totalAssetsWithdrawable()` and selector `0xa8144e48`
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
    #[ethcall(name = "totalAssetsWithdrawable", abi = "totalAssetsWithdrawable()")]
    pub struct TotalAssetsWithdrawableCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `trustPosition` function with signature `trustPosition(address,uint8)` and selector `0xfc4d43be`
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
    #[ethcall(name = "trustPosition", abi = "trustPosition(address,uint8)")]
    pub struct TrustPositionCall {
        pub position: ::ethers::core::types::Address,
        pub position_type: u8,
    }
    ///Container type for all input parameters for the `userShareLockStartBlock` function with signature `userShareLockStartBlock(address)` and selector `0xfc444591`
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
        name = "userShareLockStartBlock",
        abi = "userShareLockStartBlock(address)"
    )]
    pub struct UserShareLockStartBlockCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256,address,address)")]
    pub struct WithdrawCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawType` function with signature `withdrawType()` and selector `0xe39448e0`
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
    #[ethcall(name = "withdrawType", abi = "withdrawType()")]
    pub struct WithdrawTypeCall;
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
    pub enum CellarV1Calls {
        DomainSeparator(DomainSeparatorCall),
        MaximumShareLockPeriod(MaximumShareLockPeriodCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPerformanceFee(MaxPerformanceFeeCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        MinimumShareLockPeriod(MinimumShareLockPeriodCall),
        PriceRouterRegistrySlot(PriceRouterRegistrySlotCall),
        AddPosition(AddPositionCall),
        Allowance(AllowanceCall),
        AllowedRebalanceDeviation(AllowedRebalanceDeviationCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        BalanceOf(BalanceOfCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        Decimals(DecimalsCall),
        DecreaseAllowance(DecreaseAllowanceCall),
        Deposit(DepositCall),
        DepositLimit(DepositLimitCall),
        FeeData(FeeDataCall),
        GetPositionType(GetPositionTypeCall),
        GetPositions(GetPositionsCall),
        HoldingPosition(HoldingPositionCall),
        IncreaseAllowance(IncreaseAllowanceCall),
        InitiateShutdown(InitiateShutdownCall),
        IsPositionUsed(IsPositionUsedCall),
        IsShutdown(IsShutdownCall),
        IsTrusted(IsTrustedCall),
        LastAccrual(LastAccrualCall),
        LiftShutdown(LiftShutdownCall),
        LiquidityLimit(LiquidityLimitCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        Positions(PositionsCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PushPosition(PushPositionCall),
        Rebalance(RebalanceCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemovePosition(RemovePositionCall),
        ResetHighWatermark(ResetHighWatermarkCall),
        SendFees(SendFeesCall),
        SetDepositLimit(SetDepositLimitCall),
        SetFeesDistributor(SetFeesDistributorCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetLiquidityLimit(SetLiquidityLimitCall),
        SetOwner(SetOwnerCall),
        SetPerformanceFee(SetPerformanceFeeCall),
        SetPlatformFee(SetPlatformFeeCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetShareLockPeriod(SetShareLockPeriodCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPerformanceCut(SetStrategistPerformanceCutCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        SetWithdrawType(SetWithdrawTypeCall),
        ShareLockPeriod(ShareLockPeriodCall),
        SwapPositions(SwapPositionsCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithdrawable(TotalAssetsWithdrawableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TrustPosition(TrustPositionCall),
        UserShareLockStartBlock(UserShareLockStartBlockCall),
        Withdraw(WithdrawCall),
        WithdrawType(WithdrawTypeCall),
    }
    impl ::ethers::core::abi::AbiDecode for CellarV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <MaximumShareLockPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaximumShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <MaxFeeCutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxFeeCut(decoded));
            }
            if let Ok(decoded) = <MaxPerformanceFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPerformanceFee(decoded));
            }
            if let Ok(decoded) = <MaxPlatformFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPlatformFee(decoded));
            }
            if let Ok(decoded) = <MaxPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxPositions(decoded));
            }
            if let Ok(decoded) = <MaxRebalanceDeviationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxRebalanceDeviation(decoded));
            }
            if let Ok(decoded) = <MinimumShareLockPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MinimumShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <PriceRouterRegistrySlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PriceRouterRegistrySlot(decoded));
            }
            if let Ok(decoded) = <AddPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddPosition(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <AllowedRebalanceDeviationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowedRebalanceDeviation(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ConvertToAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToAssets(decoded));
            }
            if let Ok(decoded) = <ConvertToSharesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConvertToShares(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseAllowance(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <DepositLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DepositLimit(decoded));
            }
            if let Ok(decoded) = <FeeDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeData(decoded));
            }
            if let Ok(decoded) = <GetPositionTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositionType(decoded));
            }
            if let Ok(decoded) = <GetPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositions(decoded));
            }
            if let Ok(decoded) = <HoldingPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HoldingPosition(decoded));
            }
            if let Ok(decoded) = <IncreaseAllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseAllowance(decoded));
            }
            if let Ok(decoded) = <InitiateShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitiateShutdown(decoded));
            }
            if let Ok(decoded) = <IsPositionUsedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPositionUsed(decoded));
            }
            if let Ok(decoded) = <IsShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsShutdown(decoded));
            }
            if let Ok(decoded) = <IsTrustedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsTrusted(decoded));
            }
            if let Ok(decoded) = <LastAccrualCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastAccrual(decoded));
            }
            if let Ok(decoded) = <LiftShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LiquidityLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityLimit(decoded));
            }
            if let Ok(decoded) = <MaxDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxDeposit(decoded));
            }
            if let Ok(decoded) = <MaxMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxMint(decoded));
            }
            if let Ok(decoded) = <MaxRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxRedeem(decoded));
            }
            if let Ok(decoded) = <MaxWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxWithdraw(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <PositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Positions(decoded));
            }
            if let Ok(decoded) = <PreviewDepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewDeposit(decoded));
            }
            if let Ok(decoded) = <PreviewMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewMint(decoded));
            }
            if let Ok(decoded) = <PreviewRedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewRedeem(decoded));
            }
            if let Ok(decoded) = <PreviewWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PreviewWithdraw(decoded));
            }
            if let Ok(decoded) = <PushPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PushPosition(decoded));
            }
            if let Ok(decoded) = <RebalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rebalance(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded) = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded) = <RemovePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePosition(decoded));
            }
            if let Ok(decoded) = <ResetHighWatermarkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResetHighWatermark(decoded));
            }
            if let Ok(decoded) = <SendFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendFees(decoded));
            }
            if let Ok(decoded) = <SetDepositLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetDepositLimit(decoded));
            }
            if let Ok(decoded) = <SetFeesDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetFeesDistributor(decoded));
            }
            if let Ok(decoded) = <SetHoldingPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetHoldingPosition(decoded));
            }
            if let Ok(decoded) = <SetLiquidityLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetLiquidityLimit(decoded));
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetOwner(decoded));
            }
            if let Ok(decoded) = <SetPerformanceFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPerformanceFee(decoded));
            }
            if let Ok(decoded) = <SetPlatformFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPlatformFee(decoded));
            }
            if let Ok(decoded) = <SetRebalanceDeviationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRebalanceDeviation(decoded));
            }
            if let Ok(decoded) = <SetShareLockPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <SetStrategistPayoutAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategistPayoutAddress(decoded));
            }
            if let Ok(decoded) = <SetStrategistPerformanceCutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategistPerformanceCut(decoded));
            }
            if let Ok(decoded) = <SetStrategistPlatformCutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) = <SetWithdrawTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetWithdrawType(decoded));
            }
            if let Ok(decoded) = <ShareLockPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <SwapPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapPositions(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalAssets(decoded));
            }
            if let Ok(decoded) = <TotalAssetsWithdrawableCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalAssetsWithdrawable(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TrustPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrustPosition(decoded));
            }
            if let Ok(decoded) = <UserShareLockStartBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserShareLockStartBlock(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawTypeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawType(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CellarV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFeeCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPerformanceFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPlatformFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxRebalanceDeviation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinimumShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceRouterRegistrySlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllowedRebalanceDeviation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPositionType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitiateShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPositionUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsTrusted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastAccrual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiftShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidityLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxMint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Positions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewDeposit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewRedeem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PreviewWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PushPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rebalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Registry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetHighWatermark(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeesDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidityLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPerformanceFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPlatformFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRebalanceDeviation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrategistPayoutAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrategistPerformanceCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStrategistPlatformCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetWithdrawType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalAssetsWithdrawable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrustPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserShareLockStartBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CellarV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaximumShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFeeCut(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPerformanceFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPlatformFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MinimumShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PriceRouterRegistrySlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowedRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionType(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::HoldingPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseAllowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitiateShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPositionUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTrusted(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastAccrual(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiftShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Positions(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::PushPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rebalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetHighWatermark(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SendFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHoldingPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLiquidityLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPerformanceFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPlatformFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStrategistPayoutAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStrategistPerformanceCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStrategistPlatformCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetWithdrawType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShareLockPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssetsWithdrawable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserShareLockStartBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawType(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for CellarV1Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<MaximumShareLockPeriodCall> for CellarV1Calls {
        fn from(value: MaximumShareLockPeriodCall) -> Self {
            Self::MaximumShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<MaxFeeCutCall> for CellarV1Calls {
        fn from(value: MaxFeeCutCall) -> Self {
            Self::MaxFeeCut(value)
        }
    }
    impl ::core::convert::From<MaxPerformanceFeeCall> for CellarV1Calls {
        fn from(value: MaxPerformanceFeeCall) -> Self {
            Self::MaxPerformanceFee(value)
        }
    }
    impl ::core::convert::From<MaxPlatformFeeCall> for CellarV1Calls {
        fn from(value: MaxPlatformFeeCall) -> Self {
            Self::MaxPlatformFee(value)
        }
    }
    impl ::core::convert::From<MaxPositionsCall> for CellarV1Calls {
        fn from(value: MaxPositionsCall) -> Self {
            Self::MaxPositions(value)
        }
    }
    impl ::core::convert::From<MaxRebalanceDeviationCall> for CellarV1Calls {
        fn from(value: MaxRebalanceDeviationCall) -> Self {
            Self::MaxRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<MinimumShareLockPeriodCall> for CellarV1Calls {
        fn from(value: MinimumShareLockPeriodCall) -> Self {
            Self::MinimumShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<PriceRouterRegistrySlotCall> for CellarV1Calls {
        fn from(value: PriceRouterRegistrySlotCall) -> Self {
            Self::PriceRouterRegistrySlot(value)
        }
    }
    impl ::core::convert::From<AddPositionCall> for CellarV1Calls {
        fn from(value: AddPositionCall) -> Self {
            Self::AddPosition(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for CellarV1Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<AllowedRebalanceDeviationCall> for CellarV1Calls {
        fn from(value: AllowedRebalanceDeviationCall) -> Self {
            Self::AllowedRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for CellarV1Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for CellarV1Calls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CellarV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for CellarV1Calls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for CellarV1Calls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for CellarV1Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseAllowanceCall> for CellarV1Calls {
        fn from(value: DecreaseAllowanceCall) -> Self {
            Self::DecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<DepositCall> for CellarV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositLimitCall> for CellarV1Calls {
        fn from(value: DepositLimitCall) -> Self {
            Self::DepositLimit(value)
        }
    }
    impl ::core::convert::From<FeeDataCall> for CellarV1Calls {
        fn from(value: FeeDataCall) -> Self {
            Self::FeeData(value)
        }
    }
    impl ::core::convert::From<GetPositionTypeCall> for CellarV1Calls {
        fn from(value: GetPositionTypeCall) -> Self {
            Self::GetPositionType(value)
        }
    }
    impl ::core::convert::From<GetPositionsCall> for CellarV1Calls {
        fn from(value: GetPositionsCall) -> Self {
            Self::GetPositions(value)
        }
    }
    impl ::core::convert::From<HoldingPositionCall> for CellarV1Calls {
        fn from(value: HoldingPositionCall) -> Self {
            Self::HoldingPosition(value)
        }
    }
    impl ::core::convert::From<IncreaseAllowanceCall> for CellarV1Calls {
        fn from(value: IncreaseAllowanceCall) -> Self {
            Self::IncreaseAllowance(value)
        }
    }
    impl ::core::convert::From<InitiateShutdownCall> for CellarV1Calls {
        fn from(value: InitiateShutdownCall) -> Self {
            Self::InitiateShutdown(value)
        }
    }
    impl ::core::convert::From<IsPositionUsedCall> for CellarV1Calls {
        fn from(value: IsPositionUsedCall) -> Self {
            Self::IsPositionUsed(value)
        }
    }
    impl ::core::convert::From<IsShutdownCall> for CellarV1Calls {
        fn from(value: IsShutdownCall) -> Self {
            Self::IsShutdown(value)
        }
    }
    impl ::core::convert::From<IsTrustedCall> for CellarV1Calls {
        fn from(value: IsTrustedCall) -> Self {
            Self::IsTrusted(value)
        }
    }
    impl ::core::convert::From<LastAccrualCall> for CellarV1Calls {
        fn from(value: LastAccrualCall) -> Self {
            Self::LastAccrual(value)
        }
    }
    impl ::core::convert::From<LiftShutdownCall> for CellarV1Calls {
        fn from(value: LiftShutdownCall) -> Self {
            Self::LiftShutdown(value)
        }
    }
    impl ::core::convert::From<LiquidityLimitCall> for CellarV1Calls {
        fn from(value: LiquidityLimitCall) -> Self {
            Self::LiquidityLimit(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for CellarV1Calls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for CellarV1Calls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for CellarV1Calls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for CellarV1Calls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintCall> for CellarV1Calls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for CellarV1Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for CellarV1Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for CellarV1Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for CellarV1Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PositionsCall> for CellarV1Calls {
        fn from(value: PositionsCall) -> Self {
            Self::Positions(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for CellarV1Calls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for CellarV1Calls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for CellarV1Calls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall> for CellarV1Calls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<PushPositionCall> for CellarV1Calls {
        fn from(value: PushPositionCall) -> Self {
            Self::PushPosition(value)
        }
    }
    impl ::core::convert::From<RebalanceCall> for CellarV1Calls {
        fn from(value: RebalanceCall) -> Self {
            Self::Rebalance(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CellarV1Calls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for CellarV1Calls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<RemovePositionCall> for CellarV1Calls {
        fn from(value: RemovePositionCall) -> Self {
            Self::RemovePosition(value)
        }
    }
    impl ::core::convert::From<ResetHighWatermarkCall> for CellarV1Calls {
        fn from(value: ResetHighWatermarkCall) -> Self {
            Self::ResetHighWatermark(value)
        }
    }
    impl ::core::convert::From<SendFeesCall> for CellarV1Calls {
        fn from(value: SendFeesCall) -> Self {
            Self::SendFees(value)
        }
    }
    impl ::core::convert::From<SetDepositLimitCall> for CellarV1Calls {
        fn from(value: SetDepositLimitCall) -> Self {
            Self::SetDepositLimit(value)
        }
    }
    impl ::core::convert::From<SetFeesDistributorCall> for CellarV1Calls {
        fn from(value: SetFeesDistributorCall) -> Self {
            Self::SetFeesDistributor(value)
        }
    }
    impl ::core::convert::From<SetHoldingPositionCall> for CellarV1Calls {
        fn from(value: SetHoldingPositionCall) -> Self {
            Self::SetHoldingPosition(value)
        }
    }
    impl ::core::convert::From<SetLiquidityLimitCall> for CellarV1Calls {
        fn from(value: SetLiquidityLimitCall) -> Self {
            Self::SetLiquidityLimit(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for CellarV1Calls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<SetPerformanceFeeCall> for CellarV1Calls {
        fn from(value: SetPerformanceFeeCall) -> Self {
            Self::SetPerformanceFee(value)
        }
    }
    impl ::core::convert::From<SetPlatformFeeCall> for CellarV1Calls {
        fn from(value: SetPlatformFeeCall) -> Self {
            Self::SetPlatformFee(value)
        }
    }
    impl ::core::convert::From<SetRebalanceDeviationCall> for CellarV1Calls {
        fn from(value: SetRebalanceDeviationCall) -> Self {
            Self::SetRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<SetShareLockPeriodCall> for CellarV1Calls {
        fn from(value: SetShareLockPeriodCall) -> Self {
            Self::SetShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<SetStrategistPayoutAddressCall> for CellarV1Calls {
        fn from(value: SetStrategistPayoutAddressCall) -> Self {
            Self::SetStrategistPayoutAddress(value)
        }
    }
    impl ::core::convert::From<SetStrategistPerformanceCutCall> for CellarV1Calls {
        fn from(value: SetStrategistPerformanceCutCall) -> Self {
            Self::SetStrategistPerformanceCut(value)
        }
    }
    impl ::core::convert::From<SetStrategistPlatformCutCall> for CellarV1Calls {
        fn from(value: SetStrategistPlatformCutCall) -> Self {
            Self::SetStrategistPlatformCut(value)
        }
    }
    impl ::core::convert::From<SetWithdrawTypeCall> for CellarV1Calls {
        fn from(value: SetWithdrawTypeCall) -> Self {
            Self::SetWithdrawType(value)
        }
    }
    impl ::core::convert::From<ShareLockPeriodCall> for CellarV1Calls {
        fn from(value: ShareLockPeriodCall) -> Self {
            Self::ShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<SwapPositionsCall> for CellarV1Calls {
        fn from(value: SwapPositionsCall) -> Self {
            Self::SwapPositions(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for CellarV1Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for CellarV1Calls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalAssetsWithdrawableCall> for CellarV1Calls {
        fn from(value: TotalAssetsWithdrawableCall) -> Self {
            Self::TotalAssetsWithdrawable(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for CellarV1Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for CellarV1Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for CellarV1Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TrustPositionCall> for CellarV1Calls {
        fn from(value: TrustPositionCall) -> Self {
            Self::TrustPosition(value)
        }
    }
    impl ::core::convert::From<UserShareLockStartBlockCall> for CellarV1Calls {
        fn from(value: UserShareLockStartBlockCall) -> Self {
            Self::UserShareLockStartBlock(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CellarV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawTypeCall> for CellarV1Calls {
        fn from(value: WithdrawTypeCall) -> Self {
            Self::WithdrawType(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `MAXIMUM_SHARE_LOCK_PERIOD` function with signature `MAXIMUM_SHARE_LOCK_PERIOD()` and selector `0x0402ab63`
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
    pub struct MaximumShareLockPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_FEE_CUT` function with signature `MAX_FEE_CUT()` and selector `0xeef33eca`
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
    pub struct MaxFeeCutReturn(pub u64);
    ///Container type for all return fields from the `MAX_PERFORMANCE_FEE` function with signature `MAX_PERFORMANCE_FEE()` and selector `0xbdca9165`
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
    pub struct MaxPerformanceFeeReturn(pub u64);
    ///Container type for all return fields from the `MAX_PLATFORM_FEE` function with signature `MAX_PLATFORM_FEE()` and selector `0x3998a681`
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
    pub struct MaxPlatformFeeReturn(pub u64);
    ///Container type for all return fields from the `MAX_POSITIONS` function with signature `MAX_POSITIONS()` and selector `0xf7b24e08`
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
    pub struct MaxPositionsReturn(pub u8);
    ///Container type for all return fields from the `MAX_REBALANCE_DEVIATION` function with signature `MAX_REBALANCE_DEVIATION()` and selector `0x6ff1c02a`
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
    pub struct MaxRebalanceDeviationReturn(pub u64);
    ///Container type for all return fields from the `MINIMUM_SHARE_LOCK_PERIOD` function with signature `MINIMUM_SHARE_LOCK_PERIOD()` and selector `0x0051a3b7`
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
    pub struct MinimumShareLockPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PRICE_ROUTER_REGISTRY_SLOT` function with signature `PRICE_ROUTER_REGISTRY_SLOT()` and selector `0x5a400d25`
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
    pub struct PriceRouterRegistrySlotReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `allowedRebalanceDeviation` function with signature `allowedRebalanceDeviation()` and selector `0xc244245a`
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
    pub struct AllowedRebalanceDeviationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    ///Container type for all return fields from the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    pub struct ConvertToAssetsReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    pub struct ConvertToSharesReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `decreaseAllowance` function with signature `decreaseAllowance(address,uint256)` and selector `0xa457c2d7`
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
    pub struct DecreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    pub struct DepositReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `depositLimit` function with signature `depositLimit()` and selector `0xecf70858`
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
    pub struct DepositLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `feeData` function with signature `feeData()` and selector `0xe753e600`
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
    pub struct FeeDataReturn {
        pub high_watermark: ::ethers::core::types::U256,
        pub strategist_performance_cut: u64,
        pub strategist_platform_cut: u64,
        pub platform_fee: u64,
        pub performance_fee: u64,
        pub fees_distributor: [u8; 32],
        pub strategist_payout_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getPositionType` function with signature `getPositionType(address)` and selector `0x9e35c65b`
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
    pub struct GetPositionTypeReturn(pub u8);
    ///Container type for all return fields from the `getPositions` function with signature `getPositions()` and selector `0x80275860`
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
    pub struct GetPositionsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `holdingPosition` function with signature `holdingPosition()` and selector `0x9c5f00c2`
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
    pub struct HoldingPositionReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `increaseAllowance` function with signature `increaseAllowance(address,uint256)` and selector `0x39509351`
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
    pub struct IncreaseAllowanceReturn(pub bool);
    ///Container type for all return fields from the `isPositionUsed` function with signature `isPositionUsed(address)` and selector `0x472090fe`
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
    pub struct IsPositionUsedReturn(pub bool);
    ///Container type for all return fields from the `isShutdown` function with signature `isShutdown()` and selector `0xbf86d690`
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
    pub struct IsShutdownReturn(pub bool);
    ///Container type for all return fields from the `isTrusted` function with signature `isTrusted(address)` and selector `0x96d64879`
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
    pub struct IsTrustedReturn(pub bool);
    ///Container type for all return fields from the `lastAccrual` function with signature `lastAccrual()` and selector `0x7b3baab4`
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
    pub struct LastAccrualReturn(pub u64);
    ///Container type for all return fields from the `liquidityLimit` function with signature `liquidityLimit()` and selector `0x72163715`
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
    pub struct LiquidityLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxDeposit` function with signature `maxDeposit(address)` and selector `0x402d267d`
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
    pub struct MaxDepositReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `maxMint` function with signature `maxMint(address)` and selector `0xc63d75b6`
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
    pub struct MaxMintReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxRedeem` function with signature `maxRedeem(address)` and selector `0xd905777e`
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
    pub struct MaxRedeemReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `maxWithdraw` function with signature `maxWithdraw(address)` and selector `0xce96cb77`
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
    pub struct MaxWithdrawReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mint` function with signature `mint(uint256,address)` and selector `0x94bf804d`
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
    pub struct MintReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `positions` function with signature `positions(uint256)` and selector `0x99fbab88`
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
    pub struct PositionsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    pub struct PreviewDepositReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `previewMint` function with signature `previewMint(uint256)` and selector `0xb3d7f6b9`
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
    pub struct PreviewMintReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `previewRedeem` function with signature `previewRedeem(uint256)` and selector `0x4cdad506`
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
    pub struct PreviewRedeemReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `previewWithdraw` function with signature `previewWithdraw(uint256)` and selector `0x0a28a477`
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
    pub struct PreviewWithdrawReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `rebalance` function with signature `rebalance(address,address,uint256,uint8,bytes)` and selector `0x389a7294`
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
    pub struct RebalanceReturn {
        pub assets_to: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    pub struct RedeemReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `registry` function with signature `registry()` and selector `0x7b103999`
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
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `shareLockPeriod` function with signature `shareLockPeriod()` and selector `0x9fdb11b6`
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
    pub struct ShareLockPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalAssets` function with signature `totalAssets()` and selector `0x01e1d114`
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
    pub struct TotalAssetsReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `totalAssetsWithdrawable` function with signature `totalAssetsWithdrawable()` and selector `0xa8144e48`
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
    pub struct TotalAssetsWithdrawableReturn {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `userShareLockStartBlock` function with signature `userShareLockStartBlock(address)` and selector `0xfc444591`
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
    pub struct UserShareLockStartBlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdraw` function with signature `withdraw(uint256,address,address)` and selector `0xb460af94`
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
    pub struct WithdrawReturn {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `withdrawType` function with signature `withdrawType()` and selector `0xe39448e0`
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
    pub struct WithdrawTypeReturn(pub u8);
}
