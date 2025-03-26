pub use cellar_with_share_lock_period_v1::*;
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
pub mod cellar_with_share_lock_period_v1 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_owner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
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
                        name: ::std::borrow::ToOwned::to_owned("_holdingPosition"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_holdingPositionConfig"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_initialDeposit"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategistPlatformCut"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint64"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_shareSupplyCap"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(192usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint192"),
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
                    ::std::borrow::ToOwned::to_owned("GRAVITY_BRIDGE_REGISTRY_SLOT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GRAVITY_BRIDGE_REGISTRY_SLOT",
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
                    ::std::borrow::ToOwned::to_owned("adaptorCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adaptorCatalogue"),
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
                    ::std::borrow::ToOwned::to_owned("addAdaptorToCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addAdaptorToCatalogue",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
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
                    ::std::borrow::ToOwned::to_owned("addPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("configurationData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inDebtArray"),
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
                    ::std::borrow::ToOwned::to_owned("addPositionToCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "addPositionToCatalogue",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
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
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("automationActions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("automationActions"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
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
                    ::std::borrow::ToOwned::to_owned("blockExternalReceiver"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "blockExternalReceiver",
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
                    ::std::borrow::ToOwned::to_owned("cachePriceRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cachePriceRouter"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("checkTotalAssets"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("allowableRange"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint16"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "expectedPriceRouter",
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
                    ::std::borrow::ToOwned::to_owned("callOnAdaptor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callOnAdaptor"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                        ),
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Cellar.AdaptorCall[]",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("creditPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("creditPositions"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("debtPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("debtPositions"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("decreaseShareSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "decreaseShareSupplyCap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newShareSupplyCap",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
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
                    ::std::borrow::ToOwned::to_owned("feeData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feeData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("lastAccrual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("forcePositionOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("forcePositionOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inDebtArray"),
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
                    ::std::borrow::ToOwned::to_owned("getCreditPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getCreditPositions"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getDebtPositions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getDebtPositions"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPositionData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPositionData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("isDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ignorePause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ignorePause"),
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
                    ::std::borrow::ToOwned::to_owned("increaseShareSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "increaseShareSupplyCap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_newShareSupplyCap",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
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
                    ::std::borrow::ToOwned::to_owned("isPaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPaused"),
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
                    ::std::borrow::ToOwned::to_owned("isPositionUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPositionUsed"),
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
                    ::std::borrow::ToOwned::to_owned("locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("locked"),
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
                    ::std::borrow::ToOwned::to_owned("maxDeposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxDeposit"),
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
                    ::std::borrow::ToOwned::to_owned("maxMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxMint"),
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
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("multicall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
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
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("positionCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("positionCatalogue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("priceRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceRouter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract PriceRouter"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("removeAdaptorFromCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removeAdaptorFromCatalogue",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
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
                    ::std::borrow::ToOwned::to_owned("removePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("removePosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inDebtArray"),
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
                    ::std::borrow::ToOwned::to_owned("removePositionFromCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "removePositionFromCatalogue",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
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
                    ::std::borrow::ToOwned::to_owned("setAutomationActions"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setAutomationActions",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_registryId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_expectedAutomationActions",
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
                    ::std::borrow::ToOwned::to_owned("setHoldingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setHoldingPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
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
                    ::std::borrow::ToOwned::to_owned("shareSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("shareSupplyCap"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        192usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint192"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inDebtArray"),
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
                    ::std::borrow::ToOwned::to_owned("toggleIgnorePause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("toggleIgnorePause"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
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
                    ::std::borrow::ToOwned::to_owned("userShareLockStartTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "userShareLockStartTime",
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
                    ::std::borrow::ToOwned::to_owned("viewPositionBalances"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "viewPositionBalances",
                            ),
                            inputs: ::std::vec![],
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balances"),
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
                                    name: ::std::borrow::ToOwned::to_owned("isDebt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool[]"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdaptorCalled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdaptorCalled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AdaptorCatalogueAltered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AdaptorCatalogueAltered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("inCatalogue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__AutomationActionsUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__AutomationActionsUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newAutomationActions",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("PositionAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PositionAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("PositionCatalogueAltered"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "PositionCatalogueAltered",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("inCatalogue"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPosition2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
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
                    ::std::borrow::ToOwned::to_owned("Cellar__CallToAdaptorNotAllowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__CallToAdaptorNotAllowed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("adaptor"),
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
                        "Cellar__CallerNotApprovedToRebalance",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__CallerNotApprovedToRebalance",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Cellar__DebtMismatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__DebtMismatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__ExpectedAddressDoesNotMatchActual",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__ExpectedAddressDoesNotMatchActual",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__FailedToForceOutPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__FailedToForceOutPosition",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidHoldingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidHoldingPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("positionId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__InvalidShareSupplyCap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__InvalidShareSupplyCap",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__MinimumConstructorMintNotMet",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__MinimumConstructorMintNotMet",
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
                    ::std::borrow::ToOwned::to_owned("Cellar__Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Cellar__Paused"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionNotInCatalogue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionNotInCatalogue",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__PositionNotUsed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__PositionNotUsed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__RemovingHoldingPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__RemovingHoldingPosition",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "Cellar__SettingValueToRegistryIdZeroIsProhibited",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__SettingValueToRegistryIdZeroIsProhibited",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Cellar__ShareSupplyCapExceeded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "Cellar__ShareSupplyCapExceeded",
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
                                        "timeSharesAreUnlocked",
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
    pub static CELLARWITHSHARELOCKPERIODV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xA0`@Rg\nh\x89\x06\xBD\x8B\0\0a\x01 Rf#\x86\xF2o\xC1\0\0a\x01@R`\0a\x01`\x81\x90Ra\x01\x80R`\x0F\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16n#\x86\xF2o\xC1\0\0\nh\x89\x06\xBD\x8B\0\0\x17\x90U`\x10\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90Uf\x01\x10\xD91n\xC0\0`\x12Ub\x02\xA3\0`\x13U4\x80\x15b\0\0zW`\0\x80\xFD[P`@Qb\0r\xD08\x03\x80b\0r\xD0\x839\x81\x01`@\x81\x90Rb\0\0\x9D\x91b\0\x10]V[\x89\x89\x89\x89\x89\x89\x89\x89\x89\x893\x88\x88\x88\x81\x81\x84`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\0\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x12\x91\x90b\0\x11kV[`\0b\0\x01 \x84\x82b\0\x12\x1FV[P`\x01b\0\x01/\x83\x82b\0\x12\x1FV[P`\xFF\x81\x16`\x80RF`\xA0Rb\0\x01Eb\0\x02\xF0V[`\xC0RPPPP`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xE0RP`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x83\x16\x91\x82\x17\x90U`@Q`\0\x90`\0\x80Q` b\0r\x89\x839\x81Q\x91R\x90\x82\x90\xA3P`\x01`\x01`\xA0\x1B\x03\x89\x16a\x01\0\x81\x90R`@Qc\\\x9F\xCD\x85`\xE1\x1B\x81R`\x02`\x04\x82\x01Rc\xB9?\x9B\n\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x01\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xFD\x91\x90b\0\x12\xEBV[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\x02(\x85b\0\x03\x8CV[b\0\x027`\0\x86\x86\x82b\0\x04\x9DV[b\0\x02B\x85b\0\x07\xACV[`\x07\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\x01`\xC0\x1B\x03\x83\x16\x17\x90Ua'\x10\x83\x10\x15b\0\x02\x81W`@Qc-\r%\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\0\x02\x98`\x01`\x01`\xA0\x1B\x03\x89\x16\x8B0\x86b\0\t\x06V[b\0\x02\xA43\x84b\0\t\x9BV[b\0\x02\xB0\x85\x84b\0\n\x08V[`\x0F\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x84\x16\x17\x90Ub\0\x02\xD6\x8Ab\0\n\x9FV[PPPPPPPPPPPPPPPPPPPPb\0\x15\x86V[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qb\0\x03$\x91\x90b\0\x13\x0BV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x03\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\0Q`@QcQY\xD8\x7F`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA2\xB3\xB0\xFE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x04&W`\0\x80\xFD[PZ\xFA\x15\x80\x15b\0\x04;W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\r` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F\xEA\x05-\x1F\xB1\xEC\xBAj\xAFk\xD3.\x92\xF2\x0E{j\tN\xAAG\x82H2,\xC8\xFF\x02J\x90\x97\x8F\x91\x01`@Q\x80\x91\x03\x90\xA1PV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x04\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`D\x82\x01R`d\x01b\0\x03\xD2V[b\0\x04\xF2b\0\x0B%V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16\x15b\0\x053W`@Qc3X\x94\xFB`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01b\0\x03\xD2V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16b\0\x05sW`@Qc\x1F\x9D\xB0\x1D`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01b\0\x03\xD2V[a\x01\0Q`@Qc\x85\xAE]W`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01R`\0\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85\xAE]W\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x05\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Rb\0\x05\xF2\x91\x90\x81\x01\x90b\0\x13\x89V[\x92P\x92P\x92P\x83\x15\x15\x82\x15\x15\x14b\0\x06&W`@Qc+\x1D\x0B\xD3`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x01b\0\x03\xD2V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x15\x15` \x80\x84\x01\x91\x82R\x83\x85\x01\x86\x81R``\x85\x01\x8B\x90Rc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`\x0C\x90\x92R\x94\x90 \x83Q\x81T\x92Q\x15\x15`\x01`\xA0\x1B\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90b\0\x06\xA2\x90\x82b\0\x12\x1FV[P``\x82\x01Q`\x02\x82\x01\x90b\0\x06\xB9\x90\x82b\0\x12\x1FV[P\x90PP\x81\x15b\0\x07\0W`\nT` \x11b\0\x06\xECW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01b\0\x03\xD2V[b\0\x06\xFA`\n\x88\x88b\0\x0BSV[b\0\x076V[`\tT` \x11b\0\x07(W`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01b\0\x03\xD2V[b\0\x076`\t\x88\x88b\0\x0BSV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x0B` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xC4\xF8\xCBW\xC0\x16\xF0\xB2\x94\xFF\xF2fo\x86\xFAl\xFE\xE9\xB0:\xED\x19\xF8\x16\xAEK\xF4K~\x83{\xBB\x90b\0\x07\x9B\x90\x88\x90\x8A\x90c\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x07\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`D\x82\x01R`d\x01b\0\x03\xD2V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16b\0\x087W`@Qcp\xAB\xE8Y`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01b\0\x03\xD2V[`\xE0Q`\x01`\x01`\xA0\x1B\x03\x16b\0\x08N\x82b\0\r0V[`\x01`\x01`\xA0\x1B\x03\x16\x14b\0\x08\x98W`\xE0Qb\0\x08k\x82b\0\r0V[`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01b\0\x03\xD2V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15b\0\x08\xE0W`@Qc\nB\xC0\xF9`\xE4\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01b\0\x03\xD2V[`\x06\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80b\0\t\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FTRANSFER_FROM_FAILED\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01b\0\x03\xD2V[PPPPPV[\x80`\x02`\0\x82\x82Tb\0\t\xAF\x91\x90b\0\x14\x0EV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\x01`@Q\x80\x91\x03\x90\xA3PPV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91b\0\n\x99\x91ciD\\1`\xE0\x1B\x91b\0\nV\x91\x86\x91`\x01\x82\x01\x91`\x02\x01\x90`$\x01b\0\x14\xA9V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x19\x93\x90\x93\x16`\x01`\x01`\xE0\x1B\x03\x93\x84\x16\x17\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x91b\0\r\xC5\x16V[PPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\n\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`D\x82\x01R`d\x01b\0\x03\xD2V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90`\0\x80Q` b\0r\x89\x839\x81Q\x91R\x90`\0\x90\xA3PV[`\x06T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15b\0\x0BQW`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[\x82T\x80\x15b\0\x0C\xEFW\x83\x80b\0\x0Bk`\x01\x84b\0\x14\xD8V[\x81T\x81\x10b\0\x0B~Wb\0\x0B~b\0\x14\xEEV[`\0\x91\x82R` \x80\x83 `\x08\x80\x84\x04\x90\x91\x01T\x85T`\x01\x80\x82\x01\x88U\x96\x86R\x92\x85 \x91\x83\x04\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x95\x86\x16\x81\x02a\x01\0\x90\x81\n\x83\x81\x02\x19\x90\x94\x16\x96\x90\x97\x16\x02\x90\x95\n\x90\x92\x04\x90\x93\x16\x02\x17\x90U\x90b\0\x0B\xE2\x90\x83b\0\x14\xD8V[\x90P[\x83c\xFF\xFF\xFF\xFF\x16\x81\x11\x15b\0\x0C\x99W\x84b\0\x0C\x02`\x01\x83b\0\x14\xD8V[\x81T\x81\x10b\0\x0C\x15Wb\0\x0C\x15b\0\x14\xEEV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x85\x82\x81T\x81\x10b\0\x0CPWb\0\x0CPb\0\x14\xEEV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80b\0\x0C\x90\x90b\0\x15\x04V[\x91PPb\0\x0B\xE5V[P\x81\x84\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10b\0\x0C\xB6Wb\0\x0C\xB6b\0\x14\xEEV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPb\0\n\x99V[P\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x90\x93 `\x08\x84\x04\x01\x80T`\x07\x90\x94\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x95\x16\x92\x90\x94\x16\x93\x90\x93\x02\x17\x90\x91UPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x80T\x91Qc\xE1p\xA9\xBF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x91c\xE1p\xA9\xBF\x91b\0\rz\x91`\x01\x01\x90`\x04\x01b\0\x15\x1EV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\r\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\r\xBE\x91\x90b\0\x12\xEBV[\x93\x92PPPV[``b\0\r\xED\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01b\0r\xA9`'\x919b\0\r\xF6V[\x90P[\x92\x91PPV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@Qb\0\x0E\x15\x91\x90b\0\x153V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14b\0\x0ERW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>b\0\x0EWV[``\x91P[P\x90\x92P\x90Pb\0\x0Ek\x86\x83\x83\x87b\0\x0EuV[\x96\x95PPPPPPV[``\x83\x15b\0\x0E\xE9W\x82Q`\0\x03b\0\x0E\xE1W`\x01`\x01`\xA0\x1B\x03\x85\x16;b\0\x0E\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01b\0\x03\xD2V[P\x81b\0\x0E\xF5V[b\0\x0E\xF5\x83\x83b\0\x0E\xFDV[\x94\x93PPPPV[\x81Q\x15b\0\x0F\x0EW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01b\0\x03\xD2\x91\x90b\0\x15QV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x0F@W`\0\x80\xFD[PV[\x80Qb\0\x0FP\x81b\0\x0F*V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x0F\x88W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x0FnV[PP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\x0F\xA3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x0F\xC0Wb\0\x0F\xC0b\0\x0FUV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x0F\xEBWb\0\x0F\xEBb\0\x0FUV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x10\x05W`\0\x80\xFD[b\0\x0Ek\x84` \x83\x01` \x89\x01b\0\x0FkV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x0FPW`\0\x80\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x0FPW`\0\x80\xFD[\x80Q`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14b\0\x0FPW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15b\0\x10~W`\0\x80\xFD[b\0\x10\x89\x8Bb\0\x0FCV[\x99Pb\0\x10\x99` \x8C\x01b\0\x0FCV[\x98Pb\0\x10\xA9`@\x8C\x01b\0\x0FCV[``\x8C\x01Q\x90\x98P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x10\xC7W`\0\x80\xFD[b\0\x10\xD5\x8E\x83\x8F\x01b\0\x0F\x91V[\x98P`\x80\x8D\x01Q\x91P\x80\x82\x11\x15b\0\x10\xECW`\0\x80\xFD[b\0\x10\xFA\x8E\x83\x8F\x01b\0\x0F\x91V[\x97Pb\0\x11\n`\xA0\x8E\x01b\0\x10\x18V[\x96P`\xC0\x8D\x01Q\x91P\x80\x82\x11\x15b\0\x11!W`\0\x80\xFD[Pb\0\x110\x8D\x82\x8E\x01b\0\x0F\x91V[\x94PP`\xE0\x8B\x01Q\x92Pb\0\x11Ia\x01\0\x8C\x01b\0\x10-V[\x91Pb\0\x11Za\x01 \x8C\x01b\0\x10EV[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15b\0\x11~W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14b\0\r\xBEW`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x11\xA5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x11\xC6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\x12\x1AW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x11\xF5WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x12\x16W\x82\x81U`\x01\x01b\0\x12\x01V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x12;Wb\0\x12;b\0\x0FUV[b\0\x12S\x81b\0\x12L\x84Tb\0\x11\x90V[\x84b\0\x11\xCCV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x12\x8BW`\0\x84\x15b\0\x12rWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x12\x16V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x12\xBCW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x12\x9BV[P\x85\x82\x10\x15b\0\x12\xDBW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15b\0\x12\xFEW`\0\x80\xFD[\x81Qb\0\r\xBE\x81b\0\x0F*V[`\0\x80\x83Tb\0\x13\x1B\x81b\0\x11\x90V[`\x01\x82\x81\x16\x80\x15b\0\x136W`\x01\x81\x14b\0\x13LWb\0\x13}V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pb\0\x13}V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15b\0\x13tW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01b\0\x13YV[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x13\x9FW`\0\x80\xFD[\x83Qb\0\x13\xAC\x81b\0\x0F*V[` \x85\x01Q\x90\x93P\x80\x15\x15\x81\x14b\0\x13\xC3W`\0\x80\xFD[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x13\xE0W`\0\x80\xFD[b\0\x13\xEE\x86\x82\x87\x01b\0\x0F\x91V[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15b\0\r\xF0Wb\0\r\xF0b\0\x13\xF8V[`\0\x81Tb\0\x143\x81b\0\x11\x90V[\x80\x85R` `\x01\x83\x81\x16\x80\x15b\0\x14SW`\x01\x81\x14b\0\x14nWb\0\x14\x9EV[`\xFF\x19\x85\x16\x88\x84\x01R\x83\x15\x15`\x05\x1B\x88\x01\x83\x01\x95Pb\0\x14\x9EV[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15b\0\x14\x96W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01b\0\x14yV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[\x83\x81R``` \x82\x01R`\0b\0\x14\xC4``\x83\x01\x85b\0\x14$V[\x82\x81\x03`@\x84\x01Rb\0\x0Ek\x81\x85b\0\x14$V[\x81\x81\x03\x81\x81\x11\x15b\0\r\xF0Wb\0\r\xF0b\0\x13\xF8V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81b\0\x15\x16Wb\0\x15\x16b\0\x13\xF8V[P`\0\x19\x01\x90V[` \x81R`\0b\0\r\xED` \x83\x01\x84b\0\x14$V[`\0\x82Qb\0\x15G\x81\x84` \x87\x01b\0\x0FkV[\x91\x90\x91\x01\x92\x91PPV[` \x81R`\0\x82Q\x80` \x84\x01Rb\0\x15r\x81`@\x85\x01` \x87\x01b\0\x0FkV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\\Ub\0\x164`\09`\0\x81\x81a\x086\x01R\x81\x81a\x14\n\x01R\x81\x81a\x17\xD6\x01R\x81\x81a\"\x13\x01R\x81\x81a%\xAB\x01R\x81\x81a(\xA1\x01R\x81\x81a0\xEA\x01R\x81\x81a@Q\x01RaC\x90\x01R`\0\x81\x81a\x06F\x01R\x81\x81a\rL\x01R\x81\x81a\r\x8D\x01R\x81\x81a3\t\x01R\x81\x81a5\xCA\x01R\x81\x81a<L\x01RaFV\x01R`\0a\x10\xC7\x01R`\0a\x10\x97\x01R`\0\x81\x81a\x05\xDF\x01RaF\xD4\x01Ra\\U`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04\xB6W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x02xW\x80c\xBF\x86\xD6\x90\x11a\x01\\W\x80c\xD4F\xBB\xCC\x11a\0\xCEW\x80c\xE1\xB1\xAC\xB7\x11a\0\x92W\x80c\xE1\xB1\xAC\xB7\x14a\x0BVW\x80c\xE7S\xE6\0\x14a\x0BiW\x80c\xEE\xF3>\xCA\x14a\x0B\xDEW\x80c\xEF\x8B0\xF7\x14a\n<W\x80c\xF2\xFD\xE3\x8B\x14a\x0B\xEDW\x80c\xF7\xB2N\x08\x14a\x0C\0W`\0\x80\xFD[\x80c\xD4F\xBB\xCC\x14a\n\xC7W\x80c\xD5\x05\xAC\xCF\x14a\n\xF2W\x80c\xD7\xD4\xBFE\x14a\x0B\x05W\x80c\xD9\x05w~\x14a\x0B\x18W\x80c\xDDb\xED>\x14a\x0B+W`\0\x80\xFD[\x80c\xC8\xE8\x19P\x11a\x01 W\x80c\xC8\xE8\x19P\x14a\nOW\x80c\xCB\xDF3\xD0\x14a\nbW\x80c\xCD\x82\xF8\xB1\x14a\n\x85W\x80c\xCE\x96\xCBw\x14a\n\x8DW\x80c\xCF0\x90\x12\x14a\n\xA0W\x80c\xD1\xE8\x84\x04\x14a\n\xB4W`\0\x80\xFD[\x80c\xBF\x86\xD6\x90\x14a\t\xF9W\x80c\xC2D$Z\x14a\n\rW\x80c\xC5\x88\xD8\xD6\x14a\n\x16W\x80c\xC6=u\xB6\x14a\n)W\x80c\xC6\xE6\xF5\x92\x14a\n<W`\0\x80\xFD[\x80c\xA3s\xE3\xFF\x11a\x01\xF5W\x80c\xB0\xA7]6\x11a\x01\xB9W\x80c\xB0\xA7]6\x14a\t\x92W\x80c\xB1\x87\xBD&\x14a\t\xA5W\x80c\xB3\xD7\xF6\xB9\x14a\t\xADW\x80c\xB4`\xAF\x94\x14a\t\xC0W\x80c\xB5)*\x99\x14a\t\xD3W\x80c\xBA\x08vR\x14a\t\xE6W`\0\x80\xFD[\x80c\xA3s\xE3\xFF\x14a\tIW\x80c\xA8\x14NH\x14a\tQW\x80c\xA9\x05\x9C\xBB\x14a\tYW\x80c\xAC\x96P\xD8\x14a\tlW\x80c\xB0dn'\x14a\t\x7FW`\0\x80\xFD[\x80c\x99Y\xAF\x94\x11a\x02<W\x80c\x99Y\xAF\x94\x14a\x08\xEFW\x80c\x9CU,\xA8\x14a\t\x03W\x80c\x9C_\0\xC2\x14a\t\x16W\x80c\x9F\xDB\x11\xB6\x14a\t-W\x80c\xA0{\xEE\x0B\x14a\t6W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x08\x8BW\x80c\x93\xBB\xEA\xC0\x14a\x08\x9EW\x80c\x94\xBF\x80M\x14a\x08\xC1W\x80c\x95\xD8\x9BA\x14a\x08\xD4W\x80c\x99U\xA9\xD4\x14a\x08\xDCW`\0\x80\xFD[\x80c@-&}\x11a\x03\x9FW\x80c_k\x88\xA0\x11a\x03\x1CW\x80cq\xE9\x9D\xC2\x11a\x02\xE0W\x80cq\xE9\x9D\xC2\x14a\x07\xEFW\x80cs\x84PO\x14a\x07\xF7W\x80cx\xE0#>\x14a\x08\x1AW\x80c{\x109\x99\x14a\x081W\x80c~\xCE\xBE\0\x14a\x08XW\x80c\x88\xC4\xCA\xBA\x14a\x08xW`\0\x80\xFD[\x80c_k\x88\xA0\x14a\x07zW\x80ch|+P\x14a\x07\x8DW\x80cnU?e\x14a\x07\xADW\x80co\xF1\xC0*\x14a\x07\xC0W\x80cp\xA0\x821\x14a\x07\xCFW`\0\x80\xFD[\x80cS\n7\x14\x11a\x03cW\x80cS\n7\x14\x14a\x07\x1CW\x80cW[\xBC\xE6\x14a\x07/W\x80cY\xD2\x0BN\x14a\x07BW\x80cZ@\r%\x14a\x07jW\x80c^,Wn\x14a\x07rW`\0\x80\xFD[\x80c@-&}\x14a\x06\xCFW\x80cLF\x02\xDA\x14a\x06\xE2W\x80cL\xDA\xD5\x06\x14a\x05\x13W\x80cN\x84\xBE\xFE\x14a\x06\xF6W\x80cP\x1E\xB4\xFE\x14a\x07\tW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x048W\x80c6D\xE5\x15\x11a\x03\xFCW\x80c6D\xE5\x15\x14a\x06&W\x80c7\x9E\x0B\x13\x14a\x06.W\x80c8\xD5.\x0F\x14a\x06AW\x80c9\x98\xA6\x81\x14a\x06\x80W\x80c=\x8A\xB1\xE5\x14a\x06\xA7W\x80c>3\x82\xBA\x14a\x06\xBAW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x14a\x05\x9BW\x80c\x18\xD4\xC1C\x14a\x05\xA4W\x80c#\xB8r\xDD\x14a\x05\xC7W\x80c1<\xE5g\x14a\x05\xDAW\x80c3\xE1[\xE2\x14a\x06\x13W`\0\x80\xFD[\x80c\x07\xA2\xD1:\x11a\x04\x7FW\x80c\x07\xA2\xD1:\x14a\x05\x13W\x80c\t^\xA7\xB3\x14a\x05&W\x80c\n(\xA4w\x14a\x05IW\x80c\nh\x0E\x18\x14a\x05\\W\x80c\x15\x0Bz\x02\x14a\x05dW`\0\x80\xFD[\x80bQ\xA3\xB7\x14a\x04\xBBW\x80c\x01\xE1\xD1\x14\x14a\x04\xD7W\x80c\x04\x02\xABc\x14a\x04\xDFW\x80c\x06\xFD\xDE\x03\x14a\x04\xE9W\x80c\x07\x80\xFD:\x14a\x04\xFEW[`\0\x80\xFD[a\x04\xC4a\x01,\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\xC4a\x0C\x08V[a\x04\xC4b\x02\xA3\0\x81V[a\x04\xF1a\x0CTV[`@Qa\x04\xCE\x91\x90aLTV[a\x05\x11a\x05\x0C6`\x04aL\x80V[a\x0C\xE2V[\0[a\x04\xC4a\x05!6`\x04aL\x9BV[a\x0EMV[a\x059a\x0546`\x04aL\xC9V[a\x0EsV[`@Q\x90\x15\x15\x81R` \x01a\x04\xCEV[a\x04\xC4a\x05W6`\x04aL\x9BV[a\x0E\xE0V[a\x05\x11a\x0E\xFEV[a\x05\x82a\x05r6`\x04aM\xE0V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x04\xC4`\x02T\x81V[a\x059a\x05\xB26`\x04aNKV[`\x0E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x059a\x05\xD56`\x04aNhV[a\x0FzV[a\x06\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x06!6`\x04aN\xB7V[a\x0F\x90V[a\x04\xC4a\x10\x93V[a\x05\x11a\x06<6`\x04aN\xEEV[a\x10\xE9V[a\x06h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x06\x8Fg\x02\xC6\x8A\xF0\xBB\x14\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x06\xB56`\x04aNKV[a\x13\xC1V[a\x06\xC2a\x14\xC6V[`@Qa\x04\xCE\x91\x90aO5V[a\x04\xC4a\x06\xDD6`\x04aNKV[a\x15JV[`\x06Ta\x059\x90`\x01`\xB8\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\x07\x046`\x04aO\xCAV[a\x15\xECV[a\x05\x11a\x07\x176`\x04aL\x80V[a\x17\x91V[a\x05\x11a\x07*6`\x04aL\x9BV[a\x18\x90V[a\x05\x11a\x07=6`\x04aP\x0BV[a\x19:V[a\x07Ua\x07P6`\x04aL\x9BV[a\x19\xB6V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x04\xC4`\x02\x81V[a\x05\x11a\x19\xF0V[a\x05\x11a\x07\x886`\x04aNKV[a\x1A\x82V[a\x04\xC4a\x07\x9B6`\x04aNKV[`\x14` R`\0\x90\x81R`@\x90 T\x81V[a\x04\xC4a\x07\xBB6`\x04aP4V[a\x1B\x03V[a\x06\x8Fg\x01cEx]\x8A\0\0\x81V[a\x04\xC4a\x07\xDD6`\x04aNKV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x06\xC2a\x1B\xD7V[a\x08\na\x08\x056`\x04aL\x80V[a\x1C4V[`@Qa\x04\xCE\x94\x93\x92\x91\x90aPYV[a\x08\"a\x1D}V[`@Qa\x04\xCE\x93\x92\x91\x90aQ\x16V[a\x06h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xC4a\x08f6`\x04aNKV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x11Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x06Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x059a\x08\xAC6`\x04aL\x9BV[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\xC4a\x08\xCF6`\x04aP4V[a nV[a\x04\xF1a!-V[a\x05\x11a\x08\xEA6`\x04aQ\x81V[a!:V[`\x06Ta\x059\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\t\x116`\x04aL\x9BV[a$+V[`\x06Ta\x07U\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\xC4`\x13T\x81V[a\x05\x11a\tD6`\x04aN\xEEV[a$\xC3V[a\x05\x11a&GV[a\x04\xC4a&\xAAV[a\x059a\tg6`\x04aL\xC9V[a&\xE8V[a\x05\x11a\tz6`\x04aO\xCAV[a'\x04V[a\x05\x11a\t\x8D6`\x04aP\x0BV[a'\x8BV[a\x05\x11a\t\xA06`\x04aNKV[a'\xE5V[a\x059a(xV[a\x04\xC4a\t\xBB6`\x04aL\x9BV[a)\x1AV[a\x04\xC4a\t\xCE6`\x04aQ\xF1V[a)8V[a\x05\x11a\t\xE16`\x04aR(V[a)\xB7V[a\x04\xC4a\t\xF46`\x04aQ\xF1V[a*}V[`\x06Ta\x059\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[a\x04\xC4`\x12T\x81V[a\x05\x11a\n$6`\x04aRQV[a+\x07V[a\x04\xC4a\n76`\x04aNKV[a+\xF4V[a\x04\xC4a\nJ6`\x04aL\x9BV[a,lV[a\x05\x11a\n]6`\x04aP4V[a,\x8AV[a\x059a\np6`\x04aL\x80V[`\r` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\xC4`\0\x81V[a\x04\xC4a\n\x9B6`\x04aNKV[a-\x0CV[`\x06Ta\x059\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\n\xC26`\x04aL\x80V[a-DV[`\x07Ta\n\xDA\x90`\x01`\x01`\xC0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x0B\x006`\x04aR\x97V[a-\xC2V[`\x08Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xC4a\x0B&6`\x04aNKV[a0\x06V[a\x04\xC4a\x0B96`\x04aS\x08V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x07Ua\x0Bd6`\x04aL\x9BV[a0>V[`\x0FT`\x10Ta\x0B\xA5\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x83\x04\x82\x16\x92`\x01`\x80\x1B\x90\x04\x90\x91\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q`\x01`\x01`@\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16``\x82\x01R`\x80\x01a\x04\xCEV[a\x06\x8Fg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x05\x11a\x0B\xFB6`\x04aNKV[a0NV[a\x04\xC4` \x81V[`\0a\x0C\x12a0\xC4V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0CEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`@Q\x80\x91\x03\x90\xFD[a\x0CO`\0a1}V[\x90P\x90V[`\0\x80Ta\x0Ca\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x8D\x90aSZV[\x80\x15a\x0C\xDAW\x80`\x1F\x10a\x0C\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xDAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16a\rJW`@Qcp\xAB\xE8Y`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\r}\x82a6?V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xE1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xB5\x82a6?V[`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0E'W`@Qc\nB\xC0\xF9`\xE4\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[`\x06\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80`\0a\x0E\\`\0a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a6\xE1V[\x94\x93PPPPV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0E\xCE\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\0\x80`\0a\x0E\xEF`\0a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a6\xEEV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a\x0F0a6\xFBV[`\x06\x80T`\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B\x17\x90U`@Q`\x01\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01[`@Q\x80\x91\x03\x90\xA1V[`\0a\x0F\x85\x84a7&V[a\x0Ek\x84\x84\x84a7\x85V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x81a\x10\x05W`\t\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F\xDAWa\x0F\xDAaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\x10EV[`\n\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x10\x1EWa\x10\x1EaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16[\x90P`\0a\x10R\x82a8eV[\x90P\x80\x15a\x10\x82W`@Qc\x1C{\x94m`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x0C<V[a\x10\x8D\x84\x83\x85a8\xEEV[PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x10\xC4Wa\x0COa9\xDAV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x80\x82\x15a\x12EW`\n\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x115Wa\x115aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\n\x85c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11vWa\x11vaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81\x81`\n\x87c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11\xB9Wa\x11\xB9aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02`\n\x88c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11\xE9Wa\x11\xE9aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x84\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa\x13jV[`\t\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12^Wa\x12^aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\t\x85c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\x9FWa\x12\x9FaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81\x81`\t\x87c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\xE2Wa\x12\xE2aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02`\t\x88c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x13\x12Wa\x13\x12aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x84\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x81\x16\x82R\x83\x81\x16` \x83\x01R\x87\x81\x16\x82\x84\x01R\x86\x16``\x82\x01R\x90Q\x7F\xB7\xC5\xDF\x04t\x9A:\x06\xA9\xA7\xBF\x1A\x81B\xCC\xF2\xA4\xEEl\xBFG\tH\x9E\x87jnN\xB30\x1E\x8A\x91\x81\x90\x03`\x80\x01\x90\xA1PPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`@Qcgw\x14\x05`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCE\xEE(\n\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14LW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14`W=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7FW%p\xE8\xA47\x82\xD3i\x8A?\xED%\x8Cr\xF9\xC2\x01\xC1\x9B\xE1\xE4vN5\x9D\x1A\xDC\x8F\0\xAFz\x91\x01[`@Q\x80\x91\x03\x90\xA1PV[```\n\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15@W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15\x03W\x90P[PPPPP\x90P\x90V[`\x06T`\0\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x15gWP`\0\x91\x90PV[`\x07T`\x01`\x01`\xC0\x1B\x03\x16`\x02`\x01`\xC0\x1B\x03\x19\x81\x01a\x15\x8CWP`\0\x19\x92\x91PPV[`\0\x80a\x15\x99`\x01a6\xC8V[\x91P\x91P\x82`\x01`\x01`\xC0\x1B\x03\x16\x81\x10a\x15\xB8WP`\0\x94\x93PPPPV[`\0a\x15\xCD\x82`\x01`\x01`\xC0\x1B\x03\x86\x16aS\xE6V[\x90Pa\x15\xDA\x81\x84\x84a6\xE1V[\x96\x95PPPPPPV[PPP\x91\x90PV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x16\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\x01`\xA0\x1B`\xFF`\xA0\x1B\x19\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x16NWP`\x11T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x16lW`@Qc?\xD2\x925`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16ta6\xFBV[a\x16|a0\xC4V[`\x06\x80T`\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x17\x90U`\0\x80\x80\x80a\x16\x9D\x81a1}V[\x90Pa\x16\xC8`\x12Tg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xB8\x91\x90aS\xE6V[\x82\x90g\r\xE0\xB6\xB3\xA7d\0\0a:tV[\x93Pa\x16\xE3`\x12Tg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xB8\x91\x90aS\xF9V[`\x02T\x90\x93P\x91Pa\x16\xFF\x90Pa\x16\xFA\x85\x87aT/V[a:\xA2V[`\0a\x17\x0B`\0a1}V[\x90P\x83\x81\x10\x80a\x17\x1AWP\x82\x81\x11[\x15a\x17IW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R`d\x01a\x0C<V[`\x02T\x82\x14a\x17yW`\x02T`@Qc+@\x14Y`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R`D\x01a\x0C<V[PP`\x06\x80Tc\xFF\0\0\xFF`\xA0\x1B\x19\x16\x90UPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`@QcQY\xD8\x7F`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA2\xB3\xB0\xFE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18 W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x184W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\r` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F\xEA\x05-\x1F\xB1\xEC\xBAj\xAFk\xD3.\x92\xF2\x0E{j\tN\xAAG\x82H2,\xC8\xFF\x02J\x90\x97\x8F\x91\x01a\x14\xBBV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[g\x01cEx]\x8A\0\0\x81\x11\x15a\x18\xF4W`@Qc\x02\xD2\xA9\x0F`\xE5\x1B\x81R`\x04\x81\x01\x82\x90Rg\x01cEx]\x8A\0\0`$\x82\x01R`D\x01a\x0C<V[`\x12\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xDFK\xE3;.\x9E=\xD4\xD9\xE0\xE8VE\xAE\xA4(IJ\x06D\xA7,Q\xD6\xA1Z\xED\xAEkf\xA3\xFF\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x07T`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x19\x94W`@Qc4\xF1\xEC\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\x01`\xC0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\t\x81\x81T\x81\x10a\x19\xC6W`\0\x80\xFD[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06T`\x01`\xA8\x1B\x90\x04`\xFF\x16a\x1ADW`@Qc\xECqe\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\xFF`\xA8\x1B\x19\x16\x90U`@Q`\0\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01a\x0FpV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U\x80Q\x93\x84R\x90\x83\x01\x91\x90\x91R\x7FW%p\xE8\xA47\x82\xD3i\x8A?\xED%\x8Cr\xF9\xC2\x01\xC1\x9B\xE1\xE4vN5\x9D\x1A\xDC\x8F\0\xAFz\x91\x01a\x14\xBBV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x1B0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a\x1BP`\x01a6\xC8V[\x91P\x91Pa\x1B_\x85\x83\x83a<'V[\x92P\x82`\0\x03a\x1B\x82W`@QcBo\x157`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xC0\x1B\x03\x16a\x1B\x98\x84\x83aS\xF9V[\x11\x15a\x1B\xB7W`@Qc\xAD\xEA=\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC2\x85\x84\x86a<4V[PP`\x06\x80T`\xFF`\xA0\x1B\x19\x16\x90U\x92\x91PPV[```\t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15@W`\0\x91\x82R` \x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x16\x84R\x90\x82\x02\x83\x01\x92\x90\x91`\x04\x91\x01\x80\x84\x11a\x15\x03W\x90PPPPPP\x90P\x90V[`\x0C` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x93`\x01`\xA0\x1B\x90\x93\x04`\xFF\x16\x92\x91\x90a\x1Cl\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\x98\x90aSZV[\x80\x15a\x1C\xE5W\x80`\x1F\x10a\x1C\xBAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xE5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xC8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x1C\xFA\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D&\x90aSZV[\x80\x15a\x1DsW\x80`\x1F\x10a\x1DHWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1DsV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1DVW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x84V[`\tT`\nT``\x91\x82\x91\x82\x91\x90a\x1D\x95\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xACWa\x1D\xACaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94Pa\x1D\xE2\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xF9Wa\x1D\xF9aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93Pa\x1E/\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFWa\x1EFaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0[\x82\x81\x10\x15a\x1F\x83Wa\x1E\xBE`\t\x82\x81T\x81\x10a\x1E\x93Wa\x1E\x93aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a6?V[\x86\x82\x81Q\x81\x10a\x1E\xD0Wa\x1E\xD0aS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x1F1`\t\x82\x81T\x81\x10a\x1F\x06Wa\x1F\x06aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a8eV[\x85\x82\x81Q\x81\x10a\x1FCWa\x1FCaS\xBAV[` \x02` \x01\x01\x81\x81RPP`\0\x84\x82\x81Q\x81\x10a\x1FcWa\x1FcaS\xBAV[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x1F|\x81aUUV[\x90Pa\x1EuV[P`\0[\x81\x81\x10\x15a fWa\x1F\xA5`\n\x82\x81T\x81\x10a\x1E\x93Wa\x1E\x93aS\xBAV[`\tT\x87\x90a\x1F\xB4\x90\x84aS\xF9V[\x81Q\x81\x10a\x1F\xC4Wa\x1F\xC4aS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x1F\xFA`\n\x82\x81T\x81\x10a\x1F\x06Wa\x1F\x06aS\xBAV[`\tT\x86\x90a \t\x90\x84aS\xF9V[\x81Q\x81\x10a \x19Wa \x19aS\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\tT`\x01\x90\x85\x90a 6\x90\x84aS\xF9V[\x81Q\x81\x10a FWa FaS\xBAV[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra _\x81aUUV[\x90Pa\x1F\x87V[PPP\x90\x91\x92V[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a \x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a \xBB`\x01a6\xC8V[\x91P\x91Pa \xCA\x85\x83\x83a<\xCFV[\x92P\x82`\0\x03a \xEDW`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xC0\x1B\x03\x16a!\x03\x86\x83aS\xF9V[\x11\x15a!\"W`@Qc\xAD\xEA=\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC2\x83\x86\x86a<4V[`\x01\x80Ta\x0Ca\x90aSZV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a!dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a!la6\xFBV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16\x15a!\xABW`@Qc3X\x94\xFB`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a!\xE9W`@Qc\x1F\x9D\xB0\x1D`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x0C<V[`@Qc\x85\xAE]W`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x85\xAE]W\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x82\x91\x90\x81\x01\x90aUnV[\x92P\x92P\x92P\x83\x15\x15\x82\x15\x15\x14a\"\xB4W`@Qc+\x1D\x0B\xD3`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x01a\x0C<V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x15\x15` \x80\x84\x01\x91\x82R\x83\x85\x01\x86\x81R``\x85\x01\x8B\x90Rc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`\x0C\x90\x92R\x94\x90 \x83Q\x81T\x92Q\x15\x15`\x01`\xA0\x1B\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a#.\x90\x82aVSV[P``\x82\x01Q`\x02\x82\x01\x90a#C\x90\x82aVSV[P\x90PP\x81\x15a#\x84W`\nT` \x11a#sW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0C<V[a#\x7F`\n\x88\x88a<\xDCV[a#\xB6V[`\tT` \x11a#\xAAW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0C<V[a#\xB6`\t\x88\x88a<\xDCV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x0B` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xC4\xF8\xCBW\xC0\x16\xF0\xB2\x94\xFF\xF2fo\x86\xFAl\xFE\xE9\xB0:\xED\x19\xF8\x16\xAEK\xF4K~\x83{\xBB\x90a$\x1A\x90\x88\x90\x8A\x90c\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a$UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a\x01,\x81\x10\x80a$gWPb\x02\xA3\0\x81\x11[\x15a$\x85W`@Qc:`#?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\"\x7F\xF5\xC6\xB5\xFF\xB3\x95#k\t\xFD\x1BG+\xB1(\xB3n\xAA\x17Uf3\xFE\xEF\xE2\x8E\x94A\x1F$\x91\x01a\x19.V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a$\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x81a%8W`\t\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%\rWa%\raS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a%xV[`\n\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%QWa%QaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16[\x90P\x80c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x15\x80a&\x1EWP`@Qc!\xA0\xF7S`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c!\xA0\xF7S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1E\x91\x90aW\x12V[\x15a&<W`@Qc\xD4\xDB\x0By`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x8D\x84\x84\x84a8\xEEV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a&qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06T`\x01`\xB0\x1B\x90\x04`\xFF\x16a&\x89W`\x01a&\x8CV[`\0[`\x06\x80T\x91\x15\x15`\x01`\xB0\x1B\x02`\xFF`\xB0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a&\xB4a0\xC4V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a&\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0CO`\x01a1}V[`\0a&\xF33a7&V[a&\xFD\x83\x83a>\xA1V[\x93\x92PPPV[`\0[\x81\x81\x10\x15a'\x86Wa's\x83\x83\x83\x81\x81\x10a'$Wa'$aS\xBAV[\x90P` \x02\x81\x01\x90a'6\x91\x90aW/V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x93\x92PPa?\x07\x90PV[P\x80a'~\x81aUUV[\x91PPa'\x07V[PPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x07T`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x90\x82\x16\x10\x15a\x19\x94W`@Qc4\xF1\xEC\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a(\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x10T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FQ\xDB\xB5\xA6[\xB2'7\x86\x1Ac\xEC\x12\xBAl\xE7\x8A\x98c\x1E\x94\x04\xB0Vz.\xAFz\x06\xFCTM\x91\x01`@Q\x80\x91\x03\x90\xA1`\x10\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x06T`\0\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16a)\x14W`@Qc\n\xD8]\xFF`\xE4\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\x85\xDF\xF0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CO\x91\x90aW\x12V[P`\0\x90V[`\0\x80`\0a))`\x01a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a<\xCFV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a)eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a)\x84\x81a6\xC8V[\x91P\x91Pa)\x93\x86\x83\x83a6\xEEV[\x92Pa)\xA1\x86\x84\x87\x87a?,V[PP`\x06\x80T`\xFF`\xA0\x1B\x19\x16\x90U\x93\x92PPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a*\x13W`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x01`@\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xB5\xCC\x99J&\n\x85\xA4-e\x88f\x82!W\x1A\xE0\xA1O\n(\xF9\xE4\x81zQ\x95&!\x02\xC8h\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a*\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a*\xC9\x81a6\xC8V[\x91P\x91Pa*\xD8\x86\x83\x83a6\xE1V[\x92P\x82`\0\x03a*\xFBW`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\xA1\x83\x87\x87\x87a?,V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a+1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x80\x84\x15a+xW`\0a+Da\x0C\x08V[\x90Pa+ca+U\x86a'\x10aWuV[\x82\x90a\xFF\xFF\x16a'\x10a@\x05V[\x92Pa+ta+U\x86a'\x10aW\x90V[\x91PP[a+\x83`\x02\x84a@$V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90U`\0a+\xA8a\x0C\x08V[\x90P\x85\x15a+\xECW\x82\x81\x10\x80a+\xBDWP\x81\x81\x11[\x15a+\xECW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`d\x01a\x0C<V[PPPPPPV[`\x06T`\0\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a,\x11WP`\0\x91\x90PV[`\x07T`\x01`\x01`\xC0\x1B\x03\x16`\x02`\x01`\xC0\x1B\x03\x19\x81\x01a,6WP`\0\x19\x92\x91PPV[`\x02T`\x01`\x01`\xC0\x1B\x03\x82\x16\x81\x10\x15a,bWa,]\x81`\x01`\x01`\xC0\x1B\x03\x84\x16aS\xE6V[a\x0EkV[`\0\x94\x93PPPPV[`\0\x80`\0a,{`\x01a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a<'V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a,\xBE\x82\x82a@$V[`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F<\xED\x9F\r\n\xC3\x7F3p\xE1\xE0\x05\x15\x18*7Wsi\x8BP\xF5\xA4e#\xE2\xCB76\x01U\x83\x90` \x01a\x19.V[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a-9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0E\xDA\x82`\0aA\x05V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a-nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\r` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U\x80Q\x93\x84R\x90\x83\x01\x91\x90\x91R\x7F\xEA\x05-\x1F\xB1\xEC\xBAj\xAFk\xD3.\x92\xF2\x0E{j\tN\xAAG\x82H2,\xC8\xFF\x02J\x90\x97\x8F\x91\x01a\x14\xBBV[B\x84\x10\x15a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0C<V[`\0`\x01a.\x1Ea\x10\x93V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a/*W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a/`WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a/\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x0C<V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0E\xDA\x82`\x01aA\x05V[`\n\x81\x81T\x81\x10a\x19\xC6W`\0\x80\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a0xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\x06T`\x01`\xB0\x1B\x90\x04`\xFF\x16a1{W`@Qc\n\xD8]\xFF`\xE4\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\x85\xDF\xF0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1]\x91\x90aW\x12V[\x15a1{W`@Qc\x0F0\x1F\x8F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x9CWa1\x9CaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE2Wa1\xE2aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84\x15a3yW`\0[\x83\x81\x10\x15a2\xDCW`\0`\t\x82\x81T\x81\x10a24Wa24aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa2e\x81aAaV[\x83\x83\x81Q\x81\x10a2wWa2waS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a2\x8FWPa2\xCCV[a2\x98\x81a6?V[\x84\x83\x81Q\x81\x10a2\xAAWa2\xAAaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a2\xD5\x81aUUV[\x90Pa2\x17V[P`\x08T`@Qc\xB33\xA1u`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB33\xA1u\x90a31\x90\x85\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01aW\xABV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3r\x91\x90aW\xE9V[\x93Pa\x15\xE4V[`\nT`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x96Wa3\x96aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xBFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xDCWa3\xDCaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x86\x81\x10\x15a4\xD0W`\0`\t\x82\x81T\x81\x10a4(Wa4(aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa4Y\x81a8eV[\x86\x83\x81Q\x81\x10a4kWa4kaS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a4\x83WPa4\xC0V[a4\x8C\x81a6?V[\x87\x83\x81Q\x81\x10a4\x9EWa4\x9EaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a4\xC9\x81aUUV[\x90Pa4\x0BV[P`\0[\x83\x81\x10\x15a5\x99W`\0`\n\x82\x81T\x81\x10a4\xF1Wa4\xF1aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa5\"\x81a8eV[\x83\x83\x81Q\x81\x10a54Wa54aS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a5LWPa5\x89V[a5U\x81a6?V[\x84\x83\x81Q\x81\x10a5gWa5gaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a5\x92\x81aUUV[\x90Pa4\xD4V[P`\x08T`@Qcucs\x8B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEA\xC6\xE7\x16\x90a5\xF2\x90\x88\x90\x88\x90\x87\x90\x87\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01aX\x02V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a63\x91\x90aW\xE9V[\x98\x97PPPPPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x80T\x91Qc\xE1p\xA9\xBF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x91c\xE1p\xA9\xBF\x91a6\x87\x91`\x01\x01\x90`\x04\x01aX\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFD\x91\x90aX\xFAV[`\0\x80a6\xD5`\0a1}V[\x91P`\x02T\x90P\x91P\x91V[`\0a\x0Ek\x84\x84\x84a@\x05V[`\0a\x0Ek\x84\x83\x85a:tV[`\x06T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a1{W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x14` R`@\x90 T\x80\x15a7\x81W`\0`\x13T\x82a7U\x91\x90aS\xF9V[\x90PB\x81\x11\x15a'\x86W`@Qc\x06\xF8\xEE?`\xE2\x1B\x81R`\x04\x81\x01\x82\x90RB`$\x82\x01R`D\x01a\x0C<V[PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a7\xE1Wa7\xBC\x83\x82aS\xE6V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a8\t\x90\x84\x90aS\xE6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90a8R\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x80T\x91QcxASe`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x91cxASe\x91a8\xAD\x91`\x01\x01\x90`\x04\x01aX\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFD\x91\x90aW\xE9V[`\x06Tc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x90\x83\x16\x03a9!W`@Qc\x19\xDE\xD71`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a97Wa92`\n\x84aB\x1BV[a9BV[a9B`\t\x84aB\x1BV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x0C\x90\x91R\x81 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U\x90a9\x85`\x01\x83\x01\x82aK\xAEV[a9\x93`\x02\x83\x01`\0aK\xAEV[PP`@\x80Qc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x85\x16` \x82\x01R\x7F\xA5\xCD\0\x99\xB7\x8B'\x9C\x04\x98z\xA8\x0F\xFF\xFA\xF8\xFC\x8C\x8A\xF4\xE7\xC7\xBC\xE2hn\x8D\x01\xE2\xE1\xBDQ\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa:\x0C\x91\x90aY\x17V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a:\x8CW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0[\x81Q\x81\x10\x15a7\x81W`\0\x82\x82\x81Q\x81\x10a:\xC2Wa:\xC2aS\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0E\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16a;\x17W`@Qc]\xF6\xB6\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[`\0[\x83\x83\x81Q\x81\x10a;,Wa;,aS\xBAV[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a<\x14Wa;\x92\x84\x84\x81Q\x81\x10a;UWa;UaS\xBAV[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a;rWa;raS\xBAV[` \x02` \x01\x01Q\x83`\x01`\x01`\xA0\x1B\x03\x16a?\x07\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x7FtE\xC6Y\x8E\x1BU?\x07mPv\x92\xEA\xB3\xDC\xEE\xF0\xD6\x08uqA\xB5>\x9EV\xAA\x8B\xBA\xF4\x83\x82\x85\x85\x81Q\x81\x10a;\xC7Wa;\xC7aS\xBAV[` \x02` \x01\x01Q` \x01Q\x83\x81Q\x81\x10a;\xE4Wa;\xE4aS\xBAV[` \x02` \x01\x01Q`@Qa;\xFA\x92\x91\x90aY\x8DV[`@Q\x80\x91\x03\x90\xA1\x80a<\x0C\x81aUUV[\x91PPa;\x1AV[PP\x80a< \x90aUUV[\x90Pa:\xA5V[`\0a\x0Ek\x84\x83\x85a@\x05V[a<?\x83\x83\x83aC`V[a<t`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x86aD\"V[a<~\x81\x83aD\xACV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a'\x86\x83\x83\x83aE\x06V[`\0a\x0Ek\x84\x84\x84a:tV[\x82T\x80\x15a>`W\x83\x80a<\xF1`\x01\x84aS\xE6V[\x81T\x81\x10a=\x01Wa=\x01aS\xBAV[`\0\x91\x82R` \x80\x83 `\x08\x80\x84\x04\x90\x91\x01T\x85T`\x01\x80\x82\x01\x88U\x96\x86R\x92\x85 \x91\x83\x04\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x95\x86\x16\x81\x02a\x01\0\x90\x81\n\x83\x81\x02\x19\x90\x94\x16\x96\x90\x97\x16\x02\x90\x95\n\x90\x92\x04\x90\x93\x16\x02\x17\x90U\x90a=c\x90\x83aS\xE6V[\x90P[\x83c\xFF\xFF\xFF\xFF\x16\x81\x11\x15a>\x0EW\x84a=\x80`\x01\x83aS\xE6V[\x81T\x81\x10a=\x90Wa=\x90aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x85\x82\x81T\x81\x10a=\xC8Wa=\xC8aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80a>\x06\x90aY\xB1V[\x91PPa=fV[P\x81\x84\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a>(Wa>(aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x10\x8DV[P\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x90\x93 `\x08\x84\x04\x01\x80T`\x07\x90\x94\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x95\x16\x92\x90\x94\x16\x93\x90\x93\x02\x17\x90\x91UPV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a>\xC2\x90\x84\x90aS\xE6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90a\x0E\xCE\x90\x86\x81R` \x01\x90V[``a&\xFD\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a[\xD9`'\x919aE,V[a?8\x84\x84\x84\x84aE\x9AV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a?\xA6W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a?\xA4Wa?\x7F\x84\x82aS\xE6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a?\xB0\x81\x84aE\xAFV[`@\x80Q\x85\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x90\x85\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\x10\x8D\x84\x83aF\x11V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a@\x1DW`\0\x80\xFD[\x04\x92\x91PPV[\x81`\0\x03a@EW`@Qc-\xB3\x8D\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB9?\x9B\n\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x9D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xDE\x91\x90aX\xFAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a7\x81W`@QcN\xE2\x04\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aA\x11\x83\x83aI{V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x14` R`@\x90 T\x90\x91P\x80\x15aAZW`\0`\x13T\x82aAC\x91\x90aS\xF9V[\x90PB\x81\x11\x15aAXW`\0\x92PPPa\x0E\xDAV[P[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x81 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15aA\x8EWP`\0\x91\x90PV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Qc}(r\xE9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xFAP\xE5\xD2\x91aA\xDA\x91`\x01\x82\x01\x91`\x02\x01\x90`\x04\x01aY\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xDA\x91\x90aW\xE9V[\x81Tc\xFF\xFF\xFF\xFF\x82\x16\x81\x11aBhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x82\x16[aB{`\x01\x83aS\xE6V[\x81\x10\x15aC\x1CW\x83aB\x8E\x82`\x01aS\xF9V[\x81T\x81\x10aB\x9EWaB\x9EaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x84\x82\x81T\x81\x10aB\xD6WaB\xD6aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80aC\x14\x90aUUV[\x91PPaBpV[P\x82\x80T\x80aC-WaC-aY\xF6V[`\0\x82\x81R` \x90 `\x08`\0\x19\x90\x92\x01\x91\x82\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x85\x16\x02a\x01\0\n\x02\x19\x16\x90U\x90UPPPV[aCk\x83\x83\x83aI\xFDV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a'\x86W`@QcUQ\xE1\xB5`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cUQ\xE1\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x03\x91\x90aW\x12V[a'\x86W`@Qc4\x87\x1F%`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x0C<V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80aD\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0C<V[PPPPPV[\x80`\x02`\0\x82\x82TaD\xBE\x91\x90aS\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\\\0\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x14` R`@\x90 B\x90Ua'\x86\x83\x83\x83aJ\rV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaEI\x91\x90aZ\x0CV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aE\x84W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aE\x89V[``\x91P[P\x91P\x91Pa\x15\xDA\x86\x83\x83\x87aJ'V[aE\xA3\x81a7&V[a\x10\x8D\x84\x84\x84\x84aJ\xA0V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90aE\xD7\x90\x84\x90aS\xE6V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90` \x01aD\xFAV[aF<`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\x08T`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xCA\x91\x90aW\xE9V[`@\x82\x01RaF\xFA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\na[\x0CV[``\x82\x01R`\tT`\0[\x81\x81\x10\x15aIXW`\0`\t\x82\x81T\x81\x10aG\"WaG\"aS\xBAV[`\0\x91\x82R` \x82 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91PaGP\x82aAaV[\x90P\x80`\0\x03aGaWPPaIHV[`\0aGl\x83a6?V[`\x08T`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xDC\x91\x90aW\xE9V[\x86`\0\x01\x81\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHG\x91\x90a[\x1BV[aHR\x90`\na[\x0CV[` \x87\x01\x81\x90R\x86Q`\0\x91\x82\x91aH}\x91aHv\x87g\r\xE0\xB6\xB3\xA7d\0\0a[8V[\x91\x90a@\x05V[\x90PaH\x9C\x88``\x01Q\x89`@\x01Q\x83a@\x05\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PaH\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x83a[OV[\x91PP`\0\x89\x82\x11\x15aI\x16W`\0aH\xE0\x89`@\x01Q\x8A``\x01Q\x8Dg\r\xE0\xB6\xB3\xA7d\0\0aHv\x91\x90a[8V[` \x8A\x01Q\x8AQ\x91\x92PaH\xF6\x91\x83\x91\x90a@\x05V[\x91PaI\ng\r\xE0\xB6\xB3\xA7d\0\0\x83a[OV[\x91P`\0\x9APPaI%V[P\x82aI\"\x82\x8BaS\xE6V[\x99P[aI0\x85\x82\x8BaJ\xA8V[\x89`\0\x03aIBWPPPPPaIXV[PPPPP[aIQ\x81aUUV[\x90PaG\x05V[P\x83\x15a\x10\x8DW`@Qc\xCC^\xA3\x9B`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x0C<V[`\0aI\x85a0\xC4V[`\0\x80aI\x92`\0a6\xC8V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x03` R`@\x81 T\x92\x94P\x90\x92P\x90aI\xBD\x90\x84\x84a6\xE1V[\x90P`\0aI\xCB`\x01a1}V[\x90P\x80\x82\x11\x15aI\xDBW\x80aI\xDDV[\x81[\x94P\x85\x15aI\xF3WaI\xF0\x85\x85\x85a<'V[\x94P[PPPP\x92\x91PPV[aJ\x05a6\xFBV[a'\x86a0\xC4V[`\x06Ta'\x86\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x84aK9V[``\x83\x15aJ\x96W\x82Q`\0\x03aJ\x8FW`\x01`\x01`\xA0\x1B\x03\x85\x16;aJ\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0C<V[P\x81a\x0EkV[a\x0Ek\x83\x83aK\x84V[a\x10\x8Da0\xC4V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91aD\xA5\x91c\xC9\x11\x1B\xD7`\xE0\x1B\x91aJ\xF7\x91\x87\x91\x87\x91`\x01\x81\x01\x91`\x02\x90\x91\x01\x90`$\x01a[qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90a?\x07V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a\x10\x8D\x91ciD\\1`\xE0\x1B\x91aJ\xF7\x91\x86\x91`\x01\x82\x01\x91`\x02\x01\x90`$\x01a[\xADV[\x81Q\x15aK\x94W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x91\x90aLTV[P\x80TaK\xBA\x90aSZV[`\0\x82U\x80`\x1F\x10aK\xCAWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90aK\xE8\x91\x90aK\xEBV[PV[[\x80\x82\x11\x15aL\0W`\0\x81U`\x01\x01aK\xECV[P\x90V[`\0[\x83\x81\x10\x15aL\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01aL\x07V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaL@\x81` \x86\x01` \x86\x01aL\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a&\xFD` \x83\x01\x84aL(V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aL{W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aL\x92W`\0\x80\xFD[a&\xFD\x82aLgV[`\0` \x82\x84\x03\x12\x15aL\xADW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aL\xDCW`\0\x80\xFD[\x825aL\xE7\x81aL\xB4V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM-WaM-aL\xF5V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aL\xF5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aM|WaM|aL\xF5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aM\x9BW`\0\x80\xFD[\x815aM\xAEaM\xA9\x82aMcV[aM3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aM\xC3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aM\xF6W`\0\x80\xFD[\x845aN\x01\x81aL\xB4V[\x93P` \x85\x015aN\x11\x81aL\xB4V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN3W`\0\x80\xFD[aN?\x87\x82\x88\x01aM\x8AV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15aN]W`\0\x80\xFD[\x815a&\xFD\x81aL\xB4V[`\0\x80`\0``\x84\x86\x03\x12\x15aN}W`\0\x80\xFD[\x835aN\x88\x81aL\xB4V[\x92P` \x84\x015aN\x98\x81aL\xB4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x80\x15\x15\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aN\xCAW`\0\x80\xFD[aN\xD3\x83aLgV[\x91P` \x83\x015aN\xE3\x81aN\xA9V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aO\x03W`\0\x80\xFD[aO\x0C\x84aLgV[\x92PaO\x1A` \x85\x01aLgV[\x91P`@\x84\x015aO*\x81aN\xA9V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aOsW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aOQV[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aO\x91W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xA8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aO\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aO\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF3W`\0\x80\xFD[aO\xFF\x85\x82\x86\x01aO\x7FV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aP\x1DW`\0\x80\xFD[\x815`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a&\xFDW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aPGW`\0\x80\xFD[\x825\x91P` \x83\x015aN\xE3\x81aL\xB4V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x83\x15\x15` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90aP\x85\x90\x83\x01\x85aL(V[\x82\x81\x03``\x84\x01RaP\x97\x81\x85aL(V[\x97\x96PPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aP\xDBW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xB6V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aP\xDBW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xFAV[``\x81R`\0aQ)``\x83\x01\x86aP\xA2V[` \x83\x82\x03\x81\x85\x01RaQ<\x82\x87aP\xE6V[\x84\x81\x03`@\x86\x01R\x85Q\x80\x82R\x82\x87\x01\x93P\x90\x82\x01\x90`\0[\x81\x81\x10\x15aQsW\x84Q\x15\x15\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aQUV[P\x90\x98\x97PPPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aQ\x97W`\0\x80\xFD[aQ\xA0\x85aLgV[\x93PaQ\xAE` \x86\x01aLgV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC9W`\0\x80\xFD[aQ\xD5\x87\x82\x88\x01aM\x8AV[\x92PP``\x85\x015aQ\xE6\x81aN\xA9V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\x06W`\0\x80\xFD[\x835\x92P` \x84\x015aR\x18\x81aL\xB4V[\x91P`@\x84\x015aO*\x81aL\xB4V[`\0` \x82\x84\x03\x12\x15aR:W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a&\xFDW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aRfW`\0\x80\xFD[\x835aRq\x81aN\xA9V[\x92P` \x84\x015a\xFF\xFF\x81\x16\x81\x14aR\x18W`\0\x80\xFD[`\xFF\x81\x16\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aR\xB2W`\0\x80\xFD[\x875aR\xBD\x81aL\xB4V[\x96P` \x88\x015aR\xCD\x81aL\xB4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015aR\xEB\x81aR\x88V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aS\x1BW`\0\x80\xFD[\x825aS&\x81aL\xB4V[\x91P` \x83\x015aN\xE3\x81aL\xB4V[` \x80\x82R`\n\x90\x82\x01RiREENTRANCY`\xB0\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aSnW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS\x8EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xDAWa\x0E\xDAaS\xD0V[\x80\x82\x01\x80\x82\x11\x15a\x0E\xDAWa\x0E\xDAaS\xD0V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aT%WaT%aL\xF5V[P`\x05\x1B` \x01\x90V[`\0aT=aM\xA9\x84aT\x0CV[\x83\x81R` \x80\x82\x01\x91\x90`\x05\x86\x81\x1B\x86\x016\x81\x11\x15aT[W`\0\x80\xFD[\x86[\x81\x81\x10\x15aUHW\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT}W`\0\x80\x81\xFD[\x81\x8A\x01\x91P`@\x826\x03\x12\x15aT\x93W`\0\x80\x81\xFD[aT\x9BaM\x0BV[\x825aT\xA6\x81aL\xB4V[\x81R\x82\x87\x015\x82\x81\x11\x15aT\xBAW`\0\x80\x81\xFD[\x92\x90\x92\x01\x916`\x1F\x84\x01\x12aT\xCFW`\0\x80\x81\xFD[\x825aT\xDDaM\xA9\x82aT\x0CV[\x81\x81R\x90\x87\x1B\x84\x01\x88\x01\x90\x88\x81\x01\x906\x83\x11\x15aT\xFAW`\0\x80\x81\xFD[\x89\x86\x01[\x83\x81\x10\x15aU2W\x805\x86\x81\x11\x15aU\x16W`\0\x80\x81\xFD[aU$6\x8D\x83\x8B\x01\x01aM\x8AV[\x84RP\x91\x8A\x01\x91\x8A\x01aT\xFEV[P\x83\x8A\x01RPP\x88RPP\x94\x83\x01\x94\x83\x01aT]V[P\x92\x97\x96PPPPPPPV[`\0`\x01\x82\x01aUgWaUgaS\xD0V[P`\x01\x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aU\x83W`\0\x80\xFD[\x83QaU\x8E\x81aL\xB4V[` \x85\x01Q\x90\x93PaU\x9F\x81aN\xA9V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xBBW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aU\xCCW`\0\x80\xFD[\x80QaU\xDAaM\xA9\x82aMcV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15aU\xEFW`\0\x80\xFD[aV\0\x82` \x83\x01` \x86\x01aL\x04V[\x80\x93PPPP\x92P\x92P\x92V[`\x1F\x82\x11\x15a'\x86W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aV4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\xECW\x82\x81U`\x01\x01aV@V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVlWaVlaL\xF5V[aV\x80\x81aVz\x84TaSZV[\x84aV\rV[` \x80`\x1F\x83\x11`\x01\x81\x14aV\xB5W`\0\x84\x15aV\x9DWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua+\xECV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aV\xE4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aV\xC5V[P\x85\x82\x10\x15aW\x02W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15aW$W`\0\x80\xFD[\x81Qa&\xFD\x81aN\xA9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aWFW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aW`W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aO\xC3W`\0\x80\xFD[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15aAZWaAZaS\xD0V[a\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15aAZWaAZaS\xD0V[``\x81R`\0aW\xBE``\x83\x01\x86aP\xA2V[\x82\x81\x03` \x84\x01RaW\xD0\x81\x86aP\xE6V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aW\xFBW`\0\x80\xFD[PQ\x91\x90PV[`\xA0\x81R`\0aX\x15`\xA0\x83\x01\x88aP\xA2V[\x82\x81\x03` \x84\x01RaX'\x81\x88aP\xE6V[\x90P\x82\x81\x03`@\x84\x01RaX;\x81\x87aP\xA2V[\x90P\x82\x81\x03``\x84\x01RaXO\x81\x86aP\xE6V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[`\0\x81TaXw\x81aSZV[\x80\x85R` `\x01\x83\x81\x16\x80\x15aX\x94W`\x01\x81\x14aX\xAEWaX\xDCV[`\xFF\x19\x85\x16\x88\x84\x01R\x83\x15\x15`\x05\x1B\x88\x01\x83\x01\x95PaX\xDCV[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15aX\xD4W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01aX\xB9V[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[` \x81R`\0a&\xFD` \x83\x01\x84aXjV[`\0` \x82\x84\x03\x12\x15aY\x0CW`\0\x80\xFD[\x81Qa&\xFD\x81aL\xB4V[`\0\x80\x83TaY%\x81aSZV[`\x01\x82\x81\x16\x80\x15aY=W`\x01\x81\x14aYRWaY\x81V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94PaY\x81V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15aYxW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01aY_V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0Ek\x90\x83\x01\x84aL(V[`\0\x81aY\xC0WaY\xC0aS\xD0V[P`\0\x19\x01\x90V[`@\x81R`\0aY\xDB`@\x83\x01\x85aXjV[\x82\x81\x03` \x84\x01RaY\xED\x81\x85aXjV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82QaZ\x1E\x81\x84` \x87\x01aL\x04V[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81[\x80\x85\x11\x15aZcW\x81`\0\x19\x04\x82\x11\x15aZIWaZIaS\xD0V[\x80\x85\x16\x15aZVW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aZ-V[P\x92P\x92\x90PV[`\0\x82aZzWP`\x01a\x0E\xDAV[\x81aZ\x87WP`\0a\x0E\xDAV[\x81`\x01\x81\x14aZ\x9DW`\x02\x81\x14aZ\xA7WaZ\xC3V[`\x01\x91PPa\x0E\xDAV[`\xFF\x84\x11\x15aZ\xB8WaZ\xB8aS\xD0V[PP`\x01\x82\x1Ba\x0E\xDAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aZ\xE6WP\x81\x81\na\x0E\xDAV[aZ\xF0\x83\x83aZ(V[\x80`\0\x19\x04\x82\x11\x15a[\x04Wa[\x04aS\xD0V[\x02\x93\x92PPPV[`\0a&\xFD`\xFF\x84\x16\x83aZkV[`\0` \x82\x84\x03\x12\x15a[-W`\0\x80\xFD[\x81Qa&\xFD\x81aR\x88V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xDAWa\x0E\xDAaS\xD0V[`\0\x82a[lWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a[\x9B\x90\x83\x01\x85aXjV[\x82\x81\x03``\x84\x01RaP\x97\x81\x85aXjV[\x83\x81R``` \x82\x01R`\0a[\xC6``\x83\x01\x85aXjV[\x82\x81\x03`@\x84\x01Ra\x15\xDA\x81\x85aXjV\xFEAddress: low-level delegate call failed\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 3\xBC\xFE\\\x80p\xDD\t\xD9#<\xEC\x0B\xD6+\xCA\x9E\xCD\xFF\x13\x1E\xEB}\xBE*I`\xA3+\xFC\xE8\xF0dsolcC\0\x08\x15\x003\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0Address: low-level delegate call failed";
    /// The bytecode of the contract.
    pub static CELLARWITHSHARELOCKPERIODV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x04\xB6W`\x005`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\x02xW\x80c\xBF\x86\xD6\x90\x11a\x01\\W\x80c\xD4F\xBB\xCC\x11a\0\xCEW\x80c\xE1\xB1\xAC\xB7\x11a\0\x92W\x80c\xE1\xB1\xAC\xB7\x14a\x0BVW\x80c\xE7S\xE6\0\x14a\x0BiW\x80c\xEE\xF3>\xCA\x14a\x0B\xDEW\x80c\xEF\x8B0\xF7\x14a\n<W\x80c\xF2\xFD\xE3\x8B\x14a\x0B\xEDW\x80c\xF7\xB2N\x08\x14a\x0C\0W`\0\x80\xFD[\x80c\xD4F\xBB\xCC\x14a\n\xC7W\x80c\xD5\x05\xAC\xCF\x14a\n\xF2W\x80c\xD7\xD4\xBFE\x14a\x0B\x05W\x80c\xD9\x05w~\x14a\x0B\x18W\x80c\xDDb\xED>\x14a\x0B+W`\0\x80\xFD[\x80c\xC8\xE8\x19P\x11a\x01 W\x80c\xC8\xE8\x19P\x14a\nOW\x80c\xCB\xDF3\xD0\x14a\nbW\x80c\xCD\x82\xF8\xB1\x14a\n\x85W\x80c\xCE\x96\xCBw\x14a\n\x8DW\x80c\xCF0\x90\x12\x14a\n\xA0W\x80c\xD1\xE8\x84\x04\x14a\n\xB4W`\0\x80\xFD[\x80c\xBF\x86\xD6\x90\x14a\t\xF9W\x80c\xC2D$Z\x14a\n\rW\x80c\xC5\x88\xD8\xD6\x14a\n\x16W\x80c\xC6=u\xB6\x14a\n)W\x80c\xC6\xE6\xF5\x92\x14a\n<W`\0\x80\xFD[\x80c\xA3s\xE3\xFF\x11a\x01\xF5W\x80c\xB0\xA7]6\x11a\x01\xB9W\x80c\xB0\xA7]6\x14a\t\x92W\x80c\xB1\x87\xBD&\x14a\t\xA5W\x80c\xB3\xD7\xF6\xB9\x14a\t\xADW\x80c\xB4`\xAF\x94\x14a\t\xC0W\x80c\xB5)*\x99\x14a\t\xD3W\x80c\xBA\x08vR\x14a\t\xE6W`\0\x80\xFD[\x80c\xA3s\xE3\xFF\x14a\tIW\x80c\xA8\x14NH\x14a\tQW\x80c\xA9\x05\x9C\xBB\x14a\tYW\x80c\xAC\x96P\xD8\x14a\tlW\x80c\xB0dn'\x14a\t\x7FW`\0\x80\xFD[\x80c\x99Y\xAF\x94\x11a\x02<W\x80c\x99Y\xAF\x94\x14a\x08\xEFW\x80c\x9CU,\xA8\x14a\t\x03W\x80c\x9C_\0\xC2\x14a\t\x16W\x80c\x9F\xDB\x11\xB6\x14a\t-W\x80c\xA0{\xEE\x0B\x14a\t6W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x08\x8BW\x80c\x93\xBB\xEA\xC0\x14a\x08\x9EW\x80c\x94\xBF\x80M\x14a\x08\xC1W\x80c\x95\xD8\x9BA\x14a\x08\xD4W\x80c\x99U\xA9\xD4\x14a\x08\xDCW`\0\x80\xFD[\x80c@-&}\x11a\x03\x9FW\x80c_k\x88\xA0\x11a\x03\x1CW\x80cq\xE9\x9D\xC2\x11a\x02\xE0W\x80cq\xE9\x9D\xC2\x14a\x07\xEFW\x80cs\x84PO\x14a\x07\xF7W\x80cx\xE0#>\x14a\x08\x1AW\x80c{\x109\x99\x14a\x081W\x80c~\xCE\xBE\0\x14a\x08XW\x80c\x88\xC4\xCA\xBA\x14a\x08xW`\0\x80\xFD[\x80c_k\x88\xA0\x14a\x07zW\x80ch|+P\x14a\x07\x8DW\x80cnU?e\x14a\x07\xADW\x80co\xF1\xC0*\x14a\x07\xC0W\x80cp\xA0\x821\x14a\x07\xCFW`\0\x80\xFD[\x80cS\n7\x14\x11a\x03cW\x80cS\n7\x14\x14a\x07\x1CW\x80cW[\xBC\xE6\x14a\x07/W\x80cY\xD2\x0BN\x14a\x07BW\x80cZ@\r%\x14a\x07jW\x80c^,Wn\x14a\x07rW`\0\x80\xFD[\x80c@-&}\x14a\x06\xCFW\x80cLF\x02\xDA\x14a\x06\xE2W\x80cL\xDA\xD5\x06\x14a\x05\x13W\x80cN\x84\xBE\xFE\x14a\x06\xF6W\x80cP\x1E\xB4\xFE\x14a\x07\tW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x11a\x048W\x80c6D\xE5\x15\x11a\x03\xFCW\x80c6D\xE5\x15\x14a\x06&W\x80c7\x9E\x0B\x13\x14a\x06.W\x80c8\xD5.\x0F\x14a\x06AW\x80c9\x98\xA6\x81\x14a\x06\x80W\x80c=\x8A\xB1\xE5\x14a\x06\xA7W\x80c>3\x82\xBA\x14a\x06\xBAW`\0\x80\xFD[\x80c\x18\x16\r\xDD\x14a\x05\x9BW\x80c\x18\xD4\xC1C\x14a\x05\xA4W\x80c#\xB8r\xDD\x14a\x05\xC7W\x80c1<\xE5g\x14a\x05\xDAW\x80c3\xE1[\xE2\x14a\x06\x13W`\0\x80\xFD[\x80c\x07\xA2\xD1:\x11a\x04\x7FW\x80c\x07\xA2\xD1:\x14a\x05\x13W\x80c\t^\xA7\xB3\x14a\x05&W\x80c\n(\xA4w\x14a\x05IW\x80c\nh\x0E\x18\x14a\x05\\W\x80c\x15\x0Bz\x02\x14a\x05dW`\0\x80\xFD[\x80bQ\xA3\xB7\x14a\x04\xBBW\x80c\x01\xE1\xD1\x14\x14a\x04\xD7W\x80c\x04\x02\xABc\x14a\x04\xDFW\x80c\x06\xFD\xDE\x03\x14a\x04\xE9W\x80c\x07\x80\xFD:\x14a\x04\xFEW[`\0\x80\xFD[a\x04\xC4a\x01,\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x04\xC4a\x0C\x08V[a\x04\xC4b\x02\xA3\0\x81V[a\x04\xF1a\x0CTV[`@Qa\x04\xCE\x91\x90aLTV[a\x05\x11a\x05\x0C6`\x04aL\x80V[a\x0C\xE2V[\0[a\x04\xC4a\x05!6`\x04aL\x9BV[a\x0EMV[a\x059a\x0546`\x04aL\xC9V[a\x0EsV[`@Q\x90\x15\x15\x81R` \x01a\x04\xCEV[a\x04\xC4a\x05W6`\x04aL\x9BV[a\x0E\xE0V[a\x05\x11a\x0E\xFEV[a\x05\x82a\x05r6`\x04aM\xE0V[c\n\x85\xBD\x01`\xE1\x1B\x94\x93PPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x04\xC4`\x02T\x81V[a\x059a\x05\xB26`\x04aNKV[`\x0E` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x059a\x05\xD56`\x04aNhV[a\x0FzV[a\x06\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x06!6`\x04aN\xB7V[a\x0F\x90V[a\x04\xC4a\x10\x93V[a\x05\x11a\x06<6`\x04aN\xEEV[a\x10\xE9V[a\x06h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x06\x8Fg\x02\xC6\x8A\xF0\xBB\x14\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x06\xB56`\x04aNKV[a\x13\xC1V[a\x06\xC2a\x14\xC6V[`@Qa\x04\xCE\x91\x90aO5V[a\x04\xC4a\x06\xDD6`\x04aNKV[a\x15JV[`\x06Ta\x059\x90`\x01`\xB8\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\x07\x046`\x04aO\xCAV[a\x15\xECV[a\x05\x11a\x07\x176`\x04aL\x80V[a\x17\x91V[a\x05\x11a\x07*6`\x04aL\x9BV[a\x18\x90V[a\x05\x11a\x07=6`\x04aP\x0BV[a\x19:V[a\x07Ua\x07P6`\x04aL\x9BV[a\x19\xB6V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x04\xC4`\x02\x81V[a\x05\x11a\x19\xF0V[a\x05\x11a\x07\x886`\x04aNKV[a\x1A\x82V[a\x04\xC4a\x07\x9B6`\x04aNKV[`\x14` R`\0\x90\x81R`@\x90 T\x81V[a\x04\xC4a\x07\xBB6`\x04aP4V[a\x1B\x03V[a\x06\x8Fg\x01cEx]\x8A\0\0\x81V[a\x04\xC4a\x07\xDD6`\x04aNKV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x06\xC2a\x1B\xD7V[a\x08\na\x08\x056`\x04aL\x80V[a\x1C4V[`@Qa\x04\xCE\x94\x93\x92\x91\x90aPYV[a\x08\"a\x1D}V[`@Qa\x04\xCE\x93\x92\x91\x90aQ\x16V[a\x06h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xC4a\x08f6`\x04aNKV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[`\x11Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x06Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x059a\x08\xAC6`\x04aL\x9BV[`\x0B` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\xC4a\x08\xCF6`\x04aP4V[a nV[a\x04\xF1a!-V[a\x05\x11a\x08\xEA6`\x04aQ\x81V[a!:V[`\x06Ta\x059\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\t\x116`\x04aL\x9BV[a$+V[`\x06Ta\x07U\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04\xC4`\x13T\x81V[a\x05\x11a\tD6`\x04aN\xEEV[a$\xC3V[a\x05\x11a&GV[a\x04\xC4a&\xAAV[a\x059a\tg6`\x04aL\xC9V[a&\xE8V[a\x05\x11a\tz6`\x04aO\xCAV[a'\x04V[a\x05\x11a\t\x8D6`\x04aP\x0BV[a'\x8BV[a\x05\x11a\t\xA06`\x04aNKV[a'\xE5V[a\x059a(xV[a\x04\xC4a\t\xBB6`\x04aL\x9BV[a)\x1AV[a\x04\xC4a\t\xCE6`\x04aQ\xF1V[a)8V[a\x05\x11a\t\xE16`\x04aR(V[a)\xB7V[a\x04\xC4a\t\xF46`\x04aQ\xF1V[a*}V[`\x06Ta\x059\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[a\x04\xC4`\x12T\x81V[a\x05\x11a\n$6`\x04aRQV[a+\x07V[a\x04\xC4a\n76`\x04aNKV[a+\xF4V[a\x04\xC4a\nJ6`\x04aL\x9BV[a,lV[a\x05\x11a\n]6`\x04aP4V[a,\x8AV[a\x059a\np6`\x04aL\x80V[`\r` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x04\xC4`\0\x81V[a\x04\xC4a\n\x9B6`\x04aNKV[a-\x0CV[`\x06Ta\x059\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x05\x11a\n\xC26`\x04aL\x80V[a-DV[`\x07Ta\n\xDA\x90`\x01`\x01`\xC0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x04\xCEV[a\x05\x11a\x0B\x006`\x04aR\x97V[a-\xC2V[`\x08Ta\x06h\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04\xC4a\x0B&6`\x04aNKV[a0\x06V[a\x04\xC4a\x0B96`\x04aS\x08V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[a\x07Ua\x0Bd6`\x04aL\x9BV[a0>V[`\x0FT`\x10Ta\x0B\xA5\x91`\x01`\x01`@\x1B\x03\x80\x82\x16\x92h\x01\0\0\0\0\0\0\0\0\x83\x04\x82\x16\x92`\x01`\x80\x1B\x90\x04\x90\x91\x16\x90`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q`\x01`\x01`@\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16``\x82\x01R`\x80\x01a\x04\xCEV[a\x06\x8Fg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x05\x11a\x0B\xFB6`\x04aNKV[a0NV[a\x04\xC4` \x81V[`\0a\x0C\x12a0\xC4V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0CEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`@Q\x80\x91\x03\x90\xFD[a\x0CO`\0a1}V[\x90P\x90V[`\0\x80Ta\x0Ca\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\x8D\x90aSZV[\x80\x15a\x0C\xDAW\x80`\x1F\x10a\x0C\xAFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xDAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16a\rJW`@Qcp\xAB\xE8Y`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\r}\x82a6?V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\xE1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xB5\x82a6?V[`@Qc)\x84s\xC7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`D\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x0E'W`@Qc\nB\xC0\xF9`\xE4\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[`\x06\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0\x80`\0a\x0E\\`\0a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a6\xE1V[\x94\x93PPPPV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0E\xCE\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\0\x80`\0a\x0E\xEF`\0a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a6\xEEV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F(W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a\x0F0a6\xFBV[`\x06\x80T`\xFF`\xA8\x1B\x19\x16`\x01`\xA8\x1B\x17\x90U`@Q`\x01\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01[`@Q\x80\x91\x03\x90\xA1V[`\0a\x0F\x85\x84a7&V[a\x0Ek\x84\x84\x84a7\x85V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x81a\x10\x05W`\t\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F\xDAWa\x0F\xDAaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a\x10EV[`\n\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x10\x1EWa\x10\x1EaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16[\x90P`\0a\x10R\x82a8eV[\x90P\x80\x15a\x10\x82W`@Qc\x1C{\x94m`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x0C<V[a\x10\x8D\x84\x83\x85a8\xEEV[PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x10\xC4Wa\x0COa9\xDAV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x11\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x80\x82\x15a\x12EW`\n\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x115Wa\x115aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\n\x85c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11vWa\x11vaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81\x81`\n\x87c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11\xB9Wa\x11\xB9aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02`\n\x88c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x11\xE9Wa\x11\xE9aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x84\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa\x13jV[`\t\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12^Wa\x12^aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91P`\t\x85c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\x9FWa\x12\x9FaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81\x81`\t\x87c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\xE2Wa\x12\xE2aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02`\t\x88c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x13\x12Wa\x13\x12aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x84\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x83\x91\x90a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x81\x16\x82R\x83\x81\x16` \x83\x01R\x87\x81\x16\x82\x84\x01R\x86\x16``\x82\x01R\x90Q\x7F\xB7\xC5\xDF\x04t\x9A:\x06\xA9\xA7\xBF\x1A\x81B\xCC\xF2\xA4\xEEl\xBFG\tH\x9E\x87jnN\xB30\x1E\x8A\x91\x81\x90\x03`\x80\x01\x90\xA1PPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x13\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`@Qcgw\x14\x05`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xCE\xEE(\n\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x14LW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x14`W=`\0\x80>=`\0\xFD[PPPP`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7FW%p\xE8\xA47\x82\xD3i\x8A?\xED%\x8Cr\xF9\xC2\x01\xC1\x9B\xE1\xE4vN5\x9D\x1A\xDC\x8F\0\xAFz\x91\x01[`@Q\x80\x91\x03\x90\xA1PV[```\n\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15@W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15\x03W\x90P[PPPPP\x90P\x90V[`\x06T`\0\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x15gWP`\0\x91\x90PV[`\x07T`\x01`\x01`\xC0\x1B\x03\x16`\x02`\x01`\xC0\x1B\x03\x19\x81\x01a\x15\x8CWP`\0\x19\x92\x91PPV[`\0\x80a\x15\x99`\x01a6\xC8V[\x91P\x91P\x82`\x01`\x01`\xC0\x1B\x03\x16\x81\x10a\x15\xB8WP`\0\x94\x93PPPPV[`\0a\x15\xCD\x82`\x01`\x01`\xC0\x1B\x03\x86\x16aS\xE6V[\x90Pa\x15\xDA\x81\x84\x84a6\xE1V[\x96\x95PPPPPPV[PPP\x91\x90PV[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x16\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\x01`\xA0\x1B`\xFF`\xA0\x1B\x19\x82\x16\x17\x90\x91U`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x16NWP`\x11T`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x16lW`@Qc?\xD2\x925`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16ta6\xFBV[a\x16|a0\xC4V[`\x06\x80T`\xFF`\xB8\x1B\x19\x16`\x01`\xB8\x1B\x17\x90U`\0\x80\x80\x80a\x16\x9D\x81a1}V[\x90Pa\x16\xC8`\x12Tg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xB8\x91\x90aS\xE6V[\x82\x90g\r\xE0\xB6\xB3\xA7d\0\0a:tV[\x93Pa\x16\xE3`\x12Tg\r\xE0\xB6\xB3\xA7d\0\0a\x16\xB8\x91\x90aS\xF9V[`\x02T\x90\x93P\x91Pa\x16\xFF\x90Pa\x16\xFA\x85\x87aT/V[a:\xA2V[`\0a\x17\x0B`\0a1}V[\x90P\x83\x81\x10\x80a\x17\x1AWP\x82\x81\x11[\x15a\x17IW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R`d\x01a\x0C<V[`\x02T\x82\x14a\x17yW`\x02T`@Qc+@\x14Y`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x83\x90R`D\x01a\x0C<V[PP`\x06\x80Tc\xFF\0\0\xFF`\xA0\x1B\x19\x16\x90UPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`@QcQY\xD8\x7F`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA2\xB3\xB0\xFE\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18 W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x184W=`\0\x80>=`\0\xFD[PPPPc\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\r` \x90\x81R`@\x91\x82\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x82Q\x93\x84R\x90\x83\x01R\x7F\xEA\x05-\x1F\xB1\xEC\xBAj\xAFk\xD3.\x92\xF2\x0E{j\tN\xAAG\x82H2,\xC8\xFF\x02J\x90\x97\x8F\x91\x01a\x14\xBBV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[g\x01cEx]\x8A\0\0\x81\x11\x15a\x18\xF4W`@Qc\x02\xD2\xA9\x0F`\xE5\x1B\x81R`\x04\x81\x01\x82\x90Rg\x01cEx]\x8A\0\0`$\x82\x01R`D\x01a\x0C<V[`\x12\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\xDFK\xE3;.\x9E=\xD4\xD9\xE0\xE8VE\xAE\xA4(IJ\x06D\xA7,Q\xD6\xA1Z\xED\xAEkf\xA3\xFF\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x07T`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x19\x94W`@Qc4\xF1\xEC\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80T`\x01`\x01`\xC0\x1B\x03\x19\x16`\x01`\x01`\xC0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\t\x81\x81T\x81\x10a\x19\xC6W`\0\x80\xFD[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06T`\x01`\xA8\x1B\x90\x04`\xFF\x16a\x1ADW`@Qc\xECqe\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06\x80T`\xFF`\xA8\x1B\x19\x16\x90U`@Q`\0\x81R\x7F\xB8R{\x93\xC3m\xAB\xDF\xE0x\xAFA\xBEx\x9B\xA9F\xA4\xAD\xCF\xEA\xFC\xF9\xD8\xDE!\xD5\x16)\x85\x9E<\x90` \x01a\x0FpV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x0E` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U\x80Q\x93\x84R\x90\x83\x01\x91\x90\x91R\x7FW%p\xE8\xA47\x82\xD3i\x8A?\xED%\x8Cr\xF9\xC2\x01\xC1\x9B\xE1\xE4vN5\x9D\x1A\xDC\x8F\0\xAFz\x91\x01a\x14\xBBV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x1B0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a\x1BP`\x01a6\xC8V[\x91P\x91Pa\x1B_\x85\x83\x83a<'V[\x92P\x82`\0\x03a\x1B\x82W`@QcBo\x157`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xC0\x1B\x03\x16a\x1B\x98\x84\x83aS\xF9V[\x11\x15a\x1B\xB7W`@Qc\xAD\xEA=\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC2\x85\x84\x86a<4V[PP`\x06\x80T`\xFF`\xA0\x1B\x19\x16\x90U\x92\x91PPV[```\t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x15@W`\0\x91\x82R` \x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x16\x84R\x90\x82\x02\x83\x01\x92\x90\x91`\x04\x91\x01\x80\x84\x11a\x15\x03W\x90PPPPPP\x90P\x90V[`\x0C` R`\0\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x83\x16\x93`\x01`\xA0\x1B\x90\x93\x04`\xFF\x16\x92\x91\x90a\x1Cl\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\x98\x90aSZV[\x80\x15a\x1C\xE5W\x80`\x1F\x10a\x1C\xBAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xE5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xC8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x1C\xFA\x90aSZV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D&\x90aSZV[\x80\x15a\x1DsW\x80`\x1F\x10a\x1DHWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1DsV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1DVW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x84V[`\tT`\nT``\x91\x82\x91\x82\x91\x90a\x1D\x95\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xACWa\x1D\xACaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\xD5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x94Pa\x1D\xE2\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xF9Wa\x1D\xF9aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E\"W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x93Pa\x1E/\x81\x83aS\xF9V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1EFWa\x1EFaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EoW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0[\x82\x81\x10\x15a\x1F\x83Wa\x1E\xBE`\t\x82\x81T\x81\x10a\x1E\x93Wa\x1E\x93aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a6?V[\x86\x82\x81Q\x81\x10a\x1E\xD0Wa\x1E\xD0aS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x1F1`\t\x82\x81T\x81\x10a\x1F\x06Wa\x1F\x06aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a8eV[\x85\x82\x81Q\x81\x10a\x1FCWa\x1FCaS\xBAV[` \x02` \x01\x01\x81\x81RPP`\0\x84\x82\x81Q\x81\x10a\x1FcWa\x1FcaS\xBAV[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x1F|\x81aUUV[\x90Pa\x1EuV[P`\0[\x81\x81\x10\x15a fWa\x1F\xA5`\n\x82\x81T\x81\x10a\x1E\x93Wa\x1E\x93aS\xBAV[`\tT\x87\x90a\x1F\xB4\x90\x84aS\xF9V[\x81Q\x81\x10a\x1F\xC4Wa\x1F\xC4aS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPa\x1F\xFA`\n\x82\x81T\x81\x10a\x1F\x06Wa\x1F\x06aS\xBAV[`\tT\x86\x90a \t\x90\x84aS\xF9V[\x81Q\x81\x10a \x19Wa \x19aS\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\tT`\x01\x90\x85\x90a 6\x90\x84aS\xF9V[\x81Q\x81\x10a FWa FaS\xBAV[\x91\x15\x15` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra _\x81aUUV[\x90Pa\x1F\x87V[PPP\x90\x91\x92V[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a \x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a \xBB`\x01a6\xC8V[\x91P\x91Pa \xCA\x85\x83\x83a<\xCFV[\x92P\x82`\0\x03a \xEDW`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xC0\x1B\x03\x16a!\x03\x86\x83aS\xF9V[\x11\x15a!\"W`@Qc\xAD\xEA=\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC2\x83\x86\x86a<4V[`\x01\x80Ta\x0Ca\x90aSZV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a!dW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a!la6\xFBV[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x0B` R`@\x90 T`\xFF\x16\x15a!\xABW`@Qc3X\x94\xFB`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\r` R`@\x90 T`\xFF\x16a!\xE9W`@Qc\x1F\x9D\xB0\x1D`\xE3\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\x0C<V[`@Qc\x85\xAE]W`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`\0\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x85\xAE]W\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\"\x82\x91\x90\x81\x01\x90aUnV[\x92P\x92P\x92P\x83\x15\x15\x82\x15\x15\x14a\"\xB4W`@Qc+\x1D\x0B\xD3`\xE1\x1B\x81Rc\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R`$\x01a\x0C<V[`@\x80Q`\x80\x81\x01\x82R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x15\x15` \x80\x84\x01\x91\x82R\x83\x85\x01\x86\x81R``\x85\x01\x8B\x90Rc\xFF\xFF\xFF\xFF\x8C\x16`\0\x90\x81R`\x0C\x90\x92R\x94\x90 \x83Q\x81T\x92Q\x15\x15`\x01`\xA0\x1B\x02`\x01`\x01`\xA8\x1B\x03\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a#.\x90\x82aVSV[P``\x82\x01Q`\x02\x82\x01\x90a#C\x90\x82aVSV[P\x90PP\x81\x15a#\x84W`\nT` \x11a#sW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0C<V[a#\x7F`\n\x88\x88a<\xDCV[a#\xB6V[`\tT` \x11a#\xAAW`@Qc\xF0%#m`\xE0\x1B\x81R` `\x04\x82\x01R`$\x01a\x0C<V[a#\xB6`\t\x88\x88a<\xDCV[c\xFF\xFF\xFF\xFF\x86\x16`\0\x90\x81R`\x0B` R`@\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7F\xC4\xF8\xCBW\xC0\x16\xF0\xB2\x94\xFF\xF2fo\x86\xFAl\xFE\xE9\xB0:\xED\x19\xF8\x16\xAEK\xF4K~\x83{\xBB\x90a$\x1A\x90\x88\x90\x8A\x90c\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a$UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a\x01,\x81\x10\x80a$gWPb\x02\xA3\0\x81\x11[\x15a$\x85W`@Qc:`#?`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x13\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7F\"\x7F\xF5\xC6\xB5\xFF\xB3\x95#k\t\xFD\x1BG+\xB1(\xB3n\xAA\x17Uf3\xFE\xEF\xE2\x8E\x94A\x1F$\x91\x01a\x19.V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a$\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x81a%8W`\t\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%\rWa%\raS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16a%xV[`\n\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%QWa%QaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16[\x90P\x80c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x14\x15\x80a&\x1EWP`@Qc!\xA0\xF7S`\xE0\x1B\x81Rc\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c!\xA0\xF7S\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xFAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\x1E\x91\x90aW\x12V[\x15a&<W`@Qc\xD4\xDB\x0By`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\x8D\x84\x84\x84a8\xEEV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a&qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06T`\x01`\xB0\x1B\x90\x04`\xFF\x16a&\x89W`\x01a&\x8CV[`\0[`\x06\x80T\x91\x15\x15`\x01`\xB0\x1B\x02`\xFF`\xB0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a&\xB4a0\xC4V[`\x06T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a&\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0CO`\x01a1}V[`\0a&\xF33a7&V[a&\xFD\x83\x83a>\xA1V[\x93\x92PPPV[`\0[\x81\x81\x10\x15a'\x86Wa's\x83\x83\x83\x81\x81\x10a'$Wa'$aS\xBAV[\x90P` \x02\x81\x01\x90a'6\x91\x90aW/V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP0\x93\x92PPa?\x07\x90PV[P\x80a'~\x81aUUV[\x91PPa'\x07V[PPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a'\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x07T`\x01`\x01`\xC0\x1B\x03\x90\x81\x16\x90\x82\x16\x10\x15a\x19\x94W`@Qc4\xF1\xEC\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a(\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x10T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7FQ\xDB\xB5\xA6[\xB2'7\x86\x1Ac\xEC\x12\xBAl\xE7\x8A\x98c\x1E\x94\x04\xB0Vz.\xAFz\x06\xFCTM\x91\x01`@Q\x80\x91\x03\x90\xA1`\x10\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x06T`\0\x90`\x01`\xB0\x1B\x90\x04`\xFF\x16a)\x14W`@Qc\n\xD8]\xFF`\xE4\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\x85\xDF\xF0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CO\x91\x90aW\x12V[P`\0\x90V[`\0\x80`\0a))`\x01a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a<\xCFV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a)eW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a)\x84\x81a6\xC8V[\x91P\x91Pa)\x93\x86\x83\x83a6\xEEV[\x92Pa)\xA1\x86\x84\x87\x87a?,V[PP`\x06\x80T`\xFF`\xA0\x1B\x19\x16\x90U\x93\x92PPPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[g\r\xE0\xB6\xB3\xA7d\0\0`\x01`\x01`@\x1B\x03\x82\x16\x11\x15a*\x13W`@Qc=\x02\x03\xE5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0FT`@\x80Q`\x01`\x01`@\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xB5\xCC\x99J&\n\x85\xA4-e\x88f\x82!W\x1A\xE0\xA1O\n(\xF9\xE4\x81zQ\x95&!\x02\xC8h\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0F\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a*\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[`\x06\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U`\0\x80a*\xC9\x81a6\xC8V[\x91P\x91Pa*\xD8\x86\x83\x83a6\xE1V[\x92P\x82`\0\x03a*\xFBW`@Qc\x97h0\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\xA1\x83\x87\x87\x87a?,V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a+1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\0\x80\x84\x15a+xW`\0a+Da\x0C\x08V[\x90Pa+ca+U\x86a'\x10aWuV[\x82\x90a\xFF\xFF\x16a'\x10a@\x05V[\x92Pa+ta+U\x86a'\x10aW\x90V[\x91PP[a+\x83`\x02\x84a@$V[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90U`\0a+\xA8a\x0C\x08V[\x90P\x85\x15a+\xECW\x82\x81\x10\x80a+\xBDWP\x81\x81\x11[\x15a+\xECW`@Qcb\x8C\xC4u`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R`d\x01a\x0C<V[PPPPPPV[`\x06T`\0\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a,\x11WP`\0\x91\x90PV[`\x07T`\x01`\x01`\xC0\x1B\x03\x16`\x02`\x01`\xC0\x1B\x03\x19\x81\x01a,6WP`\0\x19\x92\x91PPV[`\x02T`\x01`\x01`\xC0\x1B\x03\x82\x16\x81\x10\x15a,bWa,]\x81`\x01`\x01`\xC0\x1B\x03\x84\x16aS\xE6V[a\x0EkV[`\0\x94\x93PPPPV[`\0\x80`\0a,{`\x01a6\xC8V[\x91P\x91Pa\x0Ek\x84\x83\x83a<'V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[a,\xBE\x82\x82a@$V[`\x11\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F<\xED\x9F\r\n\xC3\x7F3p\xE1\xE0\x05\x15\x18*7Wsi\x8BP\xF5\xA4e#\xE2\xCB76\x01U\x83\x90` \x01a\x19.V[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a-9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0E\xDA\x82`\0aA\x05V[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a-nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[c\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R`\r` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U\x80Q\x93\x84R\x90\x83\x01\x91\x90\x91R\x7F\xEA\x05-\x1F\xB1\xEC\xBAj\xAFk\xD3.\x92\xF2\x0E{j\tN\xAAG\x82H2,\xC8\xFF\x02J\x90\x97\x8F\x91\x01a\x14\xBBV[B\x84\x10\x15a.\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0C<V[`\0`\x01a.\x1Ea\x10\x93V[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a/*W=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a/`WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a/\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x0C<V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\x06T`\0\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a03W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS6V[a\x0E\xDA\x82`\x01aA\x05V[`\n\x81\x81T\x81\x10a\x19\xC6W`\0\x80\xFD[`\x06T`\x01`\x01`\xA0\x1B\x03\x163\x14a0xW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x90aS\x94V[`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q3\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PV[`\x06T`\x01`\xB0\x1B\x90\x04`\xFF\x16a1{W`@Qc\n\xD8]\xFF`\xE4\x1B\x81R0`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xAD\x85\xDF\xF0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1]\x91\x90aW\x12V[\x15a1{W`@Qc\x0F0\x1F\x8F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\tT`\0\x90\x81\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x9CWa1\x9CaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a1\xC5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE2Wa1\xE2aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\x0BW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x84\x15a3yW`\0[\x83\x81\x10\x15a2\xDCW`\0`\t\x82\x81T\x81\x10a24Wa24aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa2e\x81aAaV[\x83\x83\x81Q\x81\x10a2wWa2waS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a2\x8FWPa2\xCCV[a2\x98\x81a6?V[\x84\x83\x81Q\x81\x10a2\xAAWa2\xAAaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a2\xD5\x81aUUV[\x90Pa2\x17V[P`\x08T`@Qc\xB33\xA1u`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xB33\xA1u\x90a31\x90\x85\x90\x85\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01aW\xABV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3r\x91\x90aW\xE9V[\x93Pa\x15\xE4V[`\nT`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x96Wa3\x96aL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a3\xBFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xDCWa3\xDCaL\xF5V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a4\x05W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x86\x81\x10\x15a4\xD0W`\0`\t\x82\x81T\x81\x10a4(Wa4(aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa4Y\x81a8eV[\x86\x83\x81Q\x81\x10a4kWa4kaS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a4\x83WPa4\xC0V[a4\x8C\x81a6?V[\x87\x83\x81Q\x81\x10a4\x9EWa4\x9EaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a4\xC9\x81aUUV[\x90Pa4\x0BV[P`\0[\x83\x81\x10\x15a5\x99W`\0`\n\x82\x81T\x81\x10a4\xF1Wa4\xF1aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90Pa5\"\x81a8eV[\x83\x83\x81Q\x81\x10a54Wa54aS\xBAV[` \x02` \x01\x01\x81\x81RP`\0\x03a5LWPa5\x89V[a5U\x81a6?V[\x84\x83\x81Q\x81\x10a5gWa5gaS\xBAV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPP[a5\x92\x81aUUV[\x90Pa4\xD4V[P`\x08T`@Qcucs\x8B`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xEA\xC6\xE7\x16\x90a5\xF2\x90\x88\x90\x88\x90\x87\x90\x87\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01aX\x02V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a63\x91\x90aW\xE9V[\x98\x97PPPPPPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x80T\x91Qc\xE1p\xA9\xBF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x91c\xE1p\xA9\xBF\x91a6\x87\x91`\x01\x01\x90`\x04\x01aX\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFD\x91\x90aX\xFAV[`\0\x80a6\xD5`\0a1}V[\x91P`\x02T\x90P\x91P\x91V[`\0a\x0Ek\x84\x84\x84a@\x05V[`\0a\x0Ek\x84\x83\x85a:tV[`\x06T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a1{W`@Qc7\xA53-`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x14` R`@\x90 T\x80\x15a7\x81W`\0`\x13T\x82a7U\x91\x90aS\xF9V[\x90PB\x81\x11\x15a'\x86W`@Qc\x06\xF8\xEE?`\xE2\x1B\x81R`\x04\x81\x01\x82\x90RB`$\x82\x01R`D\x01a\x0C<V[PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a7\xE1Wa7\xBC\x83\x82aS\xE6V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a8\t\x90\x84\x90aS\xE6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90a8R\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x80\x82 \x80T\x91QcxASe`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x91cxASe\x91a8\xAD\x91`\x01\x01\x90`\x04\x01aX\xE7V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8\xCAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xFD\x91\x90aW\xE9V[`\x06Tc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x90\x83\x16\x03a9!W`@Qc\x19\xDE\xD71`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a97Wa92`\n\x84aB\x1BV[a9BV[a9B`\t\x84aB\x1BV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0B` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90U`\x0C\x90\x91R\x81 \x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x81U\x90a9\x85`\x01\x83\x01\x82aK\xAEV[a9\x93`\x02\x83\x01`\0aK\xAEV[PP`@\x80Qc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R\x85\x16` \x82\x01R\x7F\xA5\xCD\0\x99\xB7\x8B'\x9C\x04\x98z\xA8\x0F\xFF\xFA\xF8\xFC\x8C\x8A\xF4\xE7\xC7\xBC\xE2hn\x8D\x01\xE2\xE1\xBDQ\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa:\x0C\x91\x90aY\x17V[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a:\x8CW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0[\x81Q\x81\x10\x15a7\x81W`\0\x82\x82\x81Q\x81\x10a:\xC2Wa:\xC2aS\xBAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0E\x90\x92R`@\x90\x91 T\x90\x91P`\xFF\x16a;\x17W`@Qc]\xF6\xB6\x17`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x0C<V[`\0[\x83\x83\x81Q\x81\x10a;,Wa;,aS\xBAV[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a<\x14Wa;\x92\x84\x84\x81Q\x81\x10a;UWa;UaS\xBAV[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a;rWa;raS\xBAV[` \x02` \x01\x01Q\x83`\x01`\x01`\xA0\x1B\x03\x16a?\x07\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x7FtE\xC6Y\x8E\x1BU?\x07mPv\x92\xEA\xB3\xDC\xEE\xF0\xD6\x08uqA\xB5>\x9EV\xAA\x8B\xBA\xF4\x83\x82\x85\x85\x81Q\x81\x10a;\xC7Wa;\xC7aS\xBAV[` \x02` \x01\x01Q` \x01Q\x83\x81Q\x81\x10a;\xE4Wa;\xE4aS\xBAV[` \x02` \x01\x01Q`@Qa;\xFA\x92\x91\x90aY\x8DV[`@Q\x80\x91\x03\x90\xA1\x80a<\x0C\x81aUUV[\x91PPa;\x1AV[PP\x80a< \x90aUUV[\x90Pa:\xA5V[`\0a\x0Ek\x84\x83\x85a@\x05V[a<?\x83\x83\x83aC`V[a<t`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x1630\x86aD\"V[a<~\x81\x83aD\xACV[`@\x80Q\x84\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01`@Q\x80\x91\x03\x90\xA3a'\x86\x83\x83\x83aE\x06V[`\0a\x0Ek\x84\x84\x84a:tV[\x82T\x80\x15a>`W\x83\x80a<\xF1`\x01\x84aS\xE6V[\x81T\x81\x10a=\x01Wa=\x01aS\xBAV[`\0\x91\x82R` \x80\x83 `\x08\x80\x84\x04\x90\x91\x01T\x85T`\x01\x80\x82\x01\x88U\x96\x86R\x92\x85 \x91\x83\x04\x90\x91\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x95\x86\x16\x81\x02a\x01\0\x90\x81\n\x83\x81\x02\x19\x90\x94\x16\x96\x90\x97\x16\x02\x90\x95\n\x90\x92\x04\x90\x93\x16\x02\x17\x90U\x90a=c\x90\x83aS\xE6V[\x90P[\x83c\xFF\xFF\xFF\xFF\x16\x81\x11\x15a>\x0EW\x84a=\x80`\x01\x83aS\xE6V[\x81T\x81\x10a=\x90Wa=\x90aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x85\x82\x81T\x81\x10a=\xC8Wa=\xC8aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80a>\x06\x90aY\xB1V[\x91PPa=fV[P\x81\x84\x84c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a>(Wa>(aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa\x10\x8DV[P\x82T`\x01\x81\x01\x84U`\0\x93\x84R` \x90\x93 `\x08\x84\x04\x01\x80T`\x07\x90\x94\x16`\x04\x02a\x01\0\nc\xFF\xFF\xFF\xFF\x81\x81\x02\x19\x90\x95\x16\x92\x90\x94\x16\x93\x90\x93\x02\x17\x90\x91UPV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a>\xC2\x90\x84\x90aS\xE6V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90a\x0E\xCE\x90\x86\x81R` \x01\x90V[``a&\xFD\x83\x83`@Q\x80``\x01`@R\x80`'\x81R` \x01a[\xD9`'\x919aE,V[a?8\x84\x84\x84\x84aE\x9AV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a?\xA6W`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a?\xA4Wa?\x7F\x84\x82aS\xE6V[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a?\xB0\x81\x84aE\xAFV[`@\x80Q\x85\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x90\x85\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4a\x10\x8D\x84\x83aF\x11V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a@\x1DW`\0\x80\xFD[\x04\x92\x91PPV[\x81`\0\x03a@EW`@Qc-\xB3\x8D\x05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB9?\x9B\n\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a@\x9D\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\xDE\x91\x90aX\xFAV[`\x01`\x01`\xA0\x1B\x03\x16\x14a7\x81W`@QcN\xE2\x04\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0aA\x11\x83\x83aI{V[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x14` R`@\x90 T\x90\x91P\x80\x15aAZW`\0`\x13T\x82aAC\x91\x90aS\xF9V[\x90PB\x81\x11\x15aAXW`\0\x92PPPa\x0E\xDAV[P[P\x92\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x0C` R`@\x81 T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15aA\x8EWP`\0\x91\x90PV[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Qc}(r\xE9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91c\xFAP\xE5\xD2\x91aA\xDA\x91`\x01\x82\x01\x91`\x02\x01\x90`\x04\x01aY\xC8V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xDA\x91\x90aW\xE9V[\x81Tc\xFF\xFF\xFF\xFF\x82\x16\x81\x11aBhW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x0C<V[c\xFF\xFF\xFF\xFF\x82\x16[aB{`\x01\x83aS\xE6V[\x81\x10\x15aC\x1CW\x83aB\x8E\x82`\x01aS\xF9V[\x81T\x81\x10aB\x9EWaB\x9EaS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x84\x82\x81T\x81\x10aB\xD6WaB\xD6aS\xBAV[\x90`\0R` `\0 \x90`\x08\x91\x82\x82\x04\x01\x91\x90\x06`\x04\x02a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80aC\x14\x90aUUV[\x91PPaBpV[P\x82\x80T\x80aC-WaC-aY\xF6V[`\0\x82\x81R` \x90 `\x08`\0\x19\x90\x92\x01\x91\x82\x04\x01\x80Tc\xFF\xFF\xFF\xFF`\x04`\x07\x85\x16\x02a\x01\0\n\x02\x19\x16\x90U\x90UPPPV[aCk\x83\x83\x83aI\xFDV[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a'\x86W`@QcUQ\xE1\xB5`\xE0\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cUQ\xE1\xB5\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aC\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aD\x03\x91\x90aW\x12V[a'\x86W`@Qc4\x87\x1F%`\xE2\x1B\x81R3`\x04\x82\x01R`$\x01a\x0C<V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80aD\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x0C<V[PPPPPV[\x80`\x02`\0\x82\x82TaD\xBE\x91\x90aS\xF9V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\\\0\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x14` R`@\x90 B\x90Ua'\x86\x83\x83\x83aJ\rV[```\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`@QaEI\x91\x90aZ\x0CV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14aE\x84W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aE\x89V[``\x91P[P\x91P\x91Pa\x15\xDA\x86\x83\x83\x87aJ'V[aE\xA3\x81a7&V[a\x10\x8D\x84\x84\x84\x84aJ\xA0V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90aE\xD7\x90\x84\x90aS\xE6V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\\\0\x839\x81Q\x91R\x90` \x01aD\xFAV[aF<`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\x08T`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R\x90\x91\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aF\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aF\xCA\x91\x90aW\xE9V[`@\x82\x01RaF\xFA\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\na[\x0CV[``\x82\x01R`\tT`\0[\x81\x81\x10\x15aIXW`\0`\t\x82\x81T\x81\x10aG\"WaG\"aS\xBAV[`\0\x91\x82R` \x82 `\x08\x82\x04\x01T`\x07\x90\x91\x16`\x04\x02a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x91PaGP\x82aAaV[\x90P\x80`\0\x03aGaWPPaIHV[`\0aGl\x83a6?V[`\x08T`@Qc\x02&aG`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x92\x93P\x91\x16\x90c\x02&aG\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aG\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aG\xDC\x91\x90aW\xE9V[\x86`\0\x01\x81\x81RPP\x80`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aH#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aHG\x91\x90a[\x1BV[aHR\x90`\na[\x0CV[` \x87\x01\x81\x90R\x86Q`\0\x91\x82\x91aH}\x91aHv\x87g\r\xE0\xB6\xB3\xA7d\0\0a[8V[\x91\x90a@\x05V[\x90PaH\x9C\x88``\x01Q\x89`@\x01Q\x83a@\x05\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x91PaH\xB0g\r\xE0\xB6\xB3\xA7d\0\0\x83a[OV[\x91PP`\0\x89\x82\x11\x15aI\x16W`\0aH\xE0\x89`@\x01Q\x8A``\x01Q\x8Dg\r\xE0\xB6\xB3\xA7d\0\0aHv\x91\x90a[8V[` \x8A\x01Q\x8AQ\x91\x92PaH\xF6\x91\x83\x91\x90a@\x05V[\x91PaI\ng\r\xE0\xB6\xB3\xA7d\0\0\x83a[OV[\x91P`\0\x9APPaI%V[P\x82aI\"\x82\x8BaS\xE6V[\x99P[aI0\x85\x82\x8BaJ\xA8V[\x89`\0\x03aIBWPPPPPaIXV[PPPPP[aIQ\x81aUUV[\x90PaG\x05V[P\x83\x15a\x10\x8DW`@Qc\xCC^\xA3\x9B`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x0C<V[`\0aI\x85a0\xC4V[`\0\x80aI\x92`\0a6\xC8V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x03` R`@\x81 T\x92\x94P\x90\x92P\x90aI\xBD\x90\x84\x84a6\xE1V[\x90P`\0aI\xCB`\x01a1}V[\x90P\x80\x82\x11\x15aI\xDBW\x80aI\xDDV[\x81[\x94P\x85\x15aI\xF3WaI\xF0\x85\x85\x85a<'V[\x94P[PPPP\x92\x91PPV[aJ\x05a6\xFBV[a'\x86a0\xC4V[`\x06Ta'\x86\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x84aK9V[``\x83\x15aJ\x96W\x82Q`\0\x03aJ\x8FW`\x01`\x01`\xA0\x1B\x03\x85\x16;aJ\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0C<V[P\x81a\x0EkV[a\x0Ek\x83\x83aK\x84V[a\x10\x8Da0\xC4V[c\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91aD\xA5\x91c\xC9\x11\x1B\xD7`\xE0\x1B\x91aJ\xF7\x91\x87\x91\x87\x91`\x01\x81\x01\x91`\x02\x90\x91\x01\x90`$\x01a[qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90a?\x07V[c\xFF\xFF\xFF\xFF\x82\x16`\0\x90\x81R`\x0C` R`@\x90\x81\x90 \x80T\x91Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91a\x10\x8D\x91ciD\\1`\xE0\x1B\x91aJ\xF7\x91\x86\x91`\x01\x82\x01\x91`\x02\x01\x90`$\x01a[\xADV[\x81Q\x15aK\x94W\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0C<\x91\x90aLTV[P\x80TaK\xBA\x90aSZV[`\0\x82U\x80`\x1F\x10aK\xCAWPPV[`\x1F\x01` \x90\x04\x90`\0R` `\0 \x90\x81\x01\x90aK\xE8\x91\x90aK\xEBV[PV[[\x80\x82\x11\x15aL\0W`\0\x81U`\x01\x01aK\xECV[P\x90V[`\0[\x83\x81\x10\x15aL\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01aL\x07V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaL@\x81` \x86\x01` \x86\x01aL\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a&\xFD` \x83\x01\x84aL(V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aL{W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15aL\x92W`\0\x80\xFD[a&\xFD\x82aLgV[`\0` \x82\x84\x03\x12\x15aL\xADW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aL\xDCW`\0\x80\xFD[\x825aL\xE7\x81aL\xB4V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM-WaM-aL\xF5V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aM[WaM[aL\xF5V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aM|WaM|aL\xF5V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aM\x9BW`\0\x80\xFD[\x815aM\xAEaM\xA9\x82aMcV[aM3V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aM\xC3W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aM\xF6W`\0\x80\xFD[\x845aN\x01\x81aL\xB4V[\x93P` \x85\x015aN\x11\x81aL\xB4V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN3W`\0\x80\xFD[aN?\x87\x82\x88\x01aM\x8AV[\x91PP\x92\x95\x91\x94P\x92PV[`\0` \x82\x84\x03\x12\x15aN]W`\0\x80\xFD[\x815a&\xFD\x81aL\xB4V[`\0\x80`\0``\x84\x86\x03\x12\x15aN}W`\0\x80\xFD[\x835aN\x88\x81aL\xB4V[\x92P` \x84\x015aN\x98\x81aL\xB4V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x80\x15\x15\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aN\xCAW`\0\x80\xFD[aN\xD3\x83aLgV[\x91P` \x83\x015aN\xE3\x81aN\xA9V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15aO\x03W`\0\x80\xFD[aO\x0C\x84aLgV[\x92PaO\x1A` \x85\x01aLgV[\x91P`@\x84\x015aO*\x81aN\xA9V[\x80\x91PP\x92P\x92P\x92V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aOsW\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aOQV[P\x90\x96\x95PPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12aO\x91W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xA8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aO\xC3W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aO\xDDW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xF3W`\0\x80\xFD[aO\xFF\x85\x82\x86\x01aO\x7FV[\x90\x96\x90\x95P\x93PPPPV[`\0` \x82\x84\x03\x12\x15aP\x1DW`\0\x80\xFD[\x815`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x14a&\xFDW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aPGW`\0\x80\xFD[\x825\x91P` \x83\x015aN\xE3\x81aL\xB4V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x81R\x83\x15\x15` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90aP\x85\x90\x83\x01\x85aL(V[\x82\x81\x03``\x84\x01RaP\x97\x81\x85aL(V[\x97\x96PPPPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aP\xDBW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xB6V[P\x94\x95\x94PPPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aP\xDBW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aP\xFAV[``\x81R`\0aQ)``\x83\x01\x86aP\xA2V[` \x83\x82\x03\x81\x85\x01RaQ<\x82\x87aP\xE6V[\x84\x81\x03`@\x86\x01R\x85Q\x80\x82R\x82\x87\x01\x93P\x90\x82\x01\x90`\0[\x81\x81\x10\x15aQsW\x84Q\x15\x15\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01aQUV[P\x90\x98\x97PPPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15aQ\x97W`\0\x80\xFD[aQ\xA0\x85aLgV[\x93PaQ\xAE` \x86\x01aLgV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\xC9W`\0\x80\xFD[aQ\xD5\x87\x82\x88\x01aM\x8AV[\x92PP``\x85\x015aQ\xE6\x81aN\xA9V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0``\x84\x86\x03\x12\x15aR\x06W`\0\x80\xFD[\x835\x92P` \x84\x015aR\x18\x81aL\xB4V[\x91P`@\x84\x015aO*\x81aL\xB4V[`\0` \x82\x84\x03\x12\x15aR:W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a&\xFDW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15aRfW`\0\x80\xFD[\x835aRq\x81aN\xA9V[\x92P` \x84\x015a\xFF\xFF\x81\x16\x81\x14aR\x18W`\0\x80\xFD[`\xFF\x81\x16\x81\x14aK\xE8W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15aR\xB2W`\0\x80\xFD[\x875aR\xBD\x81aL\xB4V[\x96P` \x88\x015aR\xCD\x81aL\xB4V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015aR\xEB\x81aR\x88V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aS\x1BW`\0\x80\xFD[\x825aS&\x81aL\xB4V[\x91P` \x83\x015aN\xE3\x81aL\xB4V[` \x80\x82R`\n\x90\x82\x01RiREENTRANCY`\xB0\x1B`@\x82\x01R``\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80aSnW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aS\x8EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R`\x0C\x90\x82\x01Rk\x15S\x90UU\x12\x13\xD4\x92V\x91Q`\xA2\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xDAWa\x0E\xDAaS\xD0V[\x80\x82\x01\x80\x82\x11\x15a\x0E\xDAWa\x0E\xDAaS\xD0V[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aT%WaT%aL\xF5V[P`\x05\x1B` \x01\x90V[`\0aT=aM\xA9\x84aT\x0CV[\x83\x81R` \x80\x82\x01\x91\x90`\x05\x86\x81\x1B\x86\x016\x81\x11\x15aT[W`\0\x80\xFD[\x86[\x81\x81\x10\x15aUHW\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aT}W`\0\x80\x81\xFD[\x81\x8A\x01\x91P`@\x826\x03\x12\x15aT\x93W`\0\x80\x81\xFD[aT\x9BaM\x0BV[\x825aT\xA6\x81aL\xB4V[\x81R\x82\x87\x015\x82\x81\x11\x15aT\xBAW`\0\x80\x81\xFD[\x92\x90\x92\x01\x916`\x1F\x84\x01\x12aT\xCFW`\0\x80\x81\xFD[\x825aT\xDDaM\xA9\x82aT\x0CV[\x81\x81R\x90\x87\x1B\x84\x01\x88\x01\x90\x88\x81\x01\x906\x83\x11\x15aT\xFAW`\0\x80\x81\xFD[\x89\x86\x01[\x83\x81\x10\x15aU2W\x805\x86\x81\x11\x15aU\x16W`\0\x80\x81\xFD[aU$6\x8D\x83\x8B\x01\x01aM\x8AV[\x84RP\x91\x8A\x01\x91\x8A\x01aT\xFEV[P\x83\x8A\x01RPP\x88RPP\x94\x83\x01\x94\x83\x01aT]V[P\x92\x97\x96PPPPPPPV[`\0`\x01\x82\x01aUgWaUgaS\xD0V[P`\x01\x01\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15aU\x83W`\0\x80\xFD[\x83QaU\x8E\x81aL\xB4V[` \x85\x01Q\x90\x93PaU\x9F\x81aN\xA9V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15aU\xBBW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13aU\xCCW`\0\x80\xFD[\x80QaU\xDAaM\xA9\x82aMcV[\x81\x81R\x87` \x83\x85\x01\x01\x11\x15aU\xEFW`\0\x80\xFD[aV\0\x82` \x83\x01` \x86\x01aL\x04V[\x80\x93PPPP\x92P\x92P\x92V[`\x1F\x82\x11\x15a'\x86W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15aV4WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a+\xECW\x82\x81U`\x01\x01aV@V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aVlWaVlaL\xF5V[aV\x80\x81aVz\x84TaSZV[\x84aV\rV[` \x80`\x1F\x83\x11`\x01\x81\x14aV\xB5W`\0\x84\x15aV\x9DWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua+\xECV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aV\xE4W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aV\xC5V[P\x85\x82\x10\x15aW\x02W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15aW$W`\0\x80\xFD[\x81Qa&\xFD\x81aN\xA9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aWFW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aW`W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aO\xC3W`\0\x80\xFD[a\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x80\x82\x11\x15aAZWaAZaS\xD0V[a\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x80\x82\x11\x15aAZWaAZaS\xD0V[``\x81R`\0aW\xBE``\x83\x01\x86aP\xA2V[\x82\x81\x03` \x84\x01RaW\xD0\x81\x86aP\xE6V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15aW\xFBW`\0\x80\xFD[PQ\x91\x90PV[`\xA0\x81R`\0aX\x15`\xA0\x83\x01\x88aP\xA2V[\x82\x81\x03` \x84\x01RaX'\x81\x88aP\xE6V[\x90P\x82\x81\x03`@\x84\x01RaX;\x81\x87aP\xA2V[\x90P\x82\x81\x03``\x84\x01RaXO\x81\x86aP\xE6V[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`\x80\x83\x01R\x96\x95PPPPPPV[`\0\x81TaXw\x81aSZV[\x80\x85R` `\x01\x83\x81\x16\x80\x15aX\x94W`\x01\x81\x14aX\xAEWaX\xDCV[`\xFF\x19\x85\x16\x88\x84\x01R\x83\x15\x15`\x05\x1B\x88\x01\x83\x01\x95PaX\xDCV[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15aX\xD4W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01aX\xB9V[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[` \x81R`\0a&\xFD` \x83\x01\x84aXjV[`\0` \x82\x84\x03\x12\x15aY\x0CW`\0\x80\xFD[\x81Qa&\xFD\x81aL\xB4V[`\0\x80\x83TaY%\x81aSZV[`\x01\x82\x81\x16\x80\x15aY=W`\x01\x81\x14aYRWaY\x81V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94PaY\x81V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15aYxW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01aY_V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R`\0\x90a\x0Ek\x90\x83\x01\x84aL(V[`\0\x81aY\xC0WaY\xC0aS\xD0V[P`\0\x19\x01\x90V[`@\x81R`\0aY\xDB`@\x83\x01\x85aXjV[\x82\x81\x03` \x84\x01RaY\xED\x81\x85aXjV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0\x82QaZ\x1E\x81\x84` \x87\x01aL\x04V[\x91\x90\x91\x01\x92\x91PPV[`\x01\x81\x81[\x80\x85\x11\x15aZcW\x81`\0\x19\x04\x82\x11\x15aZIWaZIaS\xD0V[\x80\x85\x16\x15aZVW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aZ-V[P\x92P\x92\x90PV[`\0\x82aZzWP`\x01a\x0E\xDAV[\x81aZ\x87WP`\0a\x0E\xDAV[\x81`\x01\x81\x14aZ\x9DW`\x02\x81\x14aZ\xA7WaZ\xC3V[`\x01\x91PPa\x0E\xDAV[`\xFF\x84\x11\x15aZ\xB8WaZ\xB8aS\xD0V[PP`\x01\x82\x1Ba\x0E\xDAV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aZ\xE6WP\x81\x81\na\x0E\xDAV[aZ\xF0\x83\x83aZ(V[\x80`\0\x19\x04\x82\x11\x15a[\x04Wa[\x04aS\xD0V[\x02\x93\x92PPPV[`\0a&\xFD`\xFF\x84\x16\x83aZkV[`\0` \x82\x84\x03\x12\x15a[-W`\0\x80\xFD[\x81Qa&\xFD\x81aR\x88V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xDAWa\x0E\xDAaS\xD0V[`\0\x82a[lWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R`\0\x90a[\x9B\x90\x83\x01\x85aXjV[\x82\x81\x03``\x84\x01RaP\x97\x81\x85aXjV[\x83\x81R``` \x82\x01R`\0a[\xC6``\x83\x01\x85aXjV[\x82\x81\x03`@\x84\x01Ra\x15\xDA\x81\x85aXjV\xFEAddress: low-level delegate call failed\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 3\xBC\xFE\\\x80p\xDD\t\xD9#<\xEC\x0B\xD6+\xCA\x9E\xCD\xFF\x13\x1E\xEB}\xBE*I`\xA3+\xFC\xE8\xF0dsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static CELLARWITHSHARELOCKPERIODV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CellarWithShareLockPeriodV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CellarWithShareLockPeriodV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CellarWithShareLockPeriodV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CellarWithShareLockPeriodV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CellarWithShareLockPeriodV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CellarWithShareLockPeriodV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CellarWithShareLockPeriodV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CELLARWITHSHARELOCKPERIODV1_ABI.clone(),
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
                CELLARWITHSHARELOCKPERIODV1_ABI.clone(),
                CELLARWITHSHARELOCKPERIODV1_BYTECODE.clone().into(),
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
        ///Calls the contract's `GRAVITY_BRIDGE_REGISTRY_SLOT` (0xcd82f8b1) function
        pub fn gravity_bridge_registry_slot(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([205, 130, 248, 177], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
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
        ///Calls the contract's `adaptorCatalogue` (0x18d4c143) function
        pub fn adaptor_catalogue(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 212, 193, 67], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addAdaptorToCatalogue` (0x3d8ab1e5) function
        pub fn add_adaptor_to_catalogue(
            &self,
            adaptor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 138, 177, 229], adaptor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPosition` (0x9955a9d4) function
        pub fn add_position(
            &self,
            index: u32,
            position_id: u32,
            configuration_data: ::ethers::core::types::Bytes,
            in_debt_array: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [153, 85, 169, 212],
                    (index, position_id, configuration_data, in_debt_array),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addPositionToCatalogue` (0x501eb4fe) function
        pub fn add_position_to_catalogue(
            &self,
            position_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 30, 180, 254], position_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
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
        ///Calls the contract's `automationActions` (0x88c4caba) function
        pub fn automation_actions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 196, 202, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `blockExternalReceiver` (0x4c4602da) function
        pub fn block_external_receiver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([76, 70, 2, 218], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cachePriceRouter` (0xc588d8d6) function
        pub fn cache_price_router(
            &self,
            check_total_assets: bool,
            allowable_range: u16,
            expected_price_router: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [197, 136, 216, 214],
                    (check_total_assets, allowable_range, expected_price_router),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callOnAdaptor` (0x4e84befe) function
        pub fn call_on_adaptor(
            &self,
            data: ::std::vec::Vec<AdaptorCall>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 132, 190, 254], data)
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
        ///Calls the contract's `creditPositions` (0x59d20b4e) function
        pub fn credit_positions(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([89, 210, 11, 78], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debtPositions` (0xe1b1acb7) function
        pub fn debt_positions(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([225, 177, 172, 183], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decreaseShareSupplyCap` (0x575bbce6) function
        pub fn decrease_share_supply_cap(
            &self,
            new_share_supply_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([87, 91, 188, 230], new_share_supply_cap)
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
        ///Calls the contract's `feeData` (0xe753e600) function
        pub fn fee_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u64, u64, u64, ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([231, 83, 230, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `forcePositionOut` (0xa07bee0b) function
        pub fn force_position_out(
            &self,
            index: u32,
            position_id: u32,
            in_debt_array: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 123, 238, 11], (index, position_id, in_debt_array))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCreditPositions` (0x71e99dc2) function
        pub fn get_credit_positions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([113, 233, 157, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDebtPositions` (0x3e3382ba) function
        pub fn get_debt_positions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<u32>> {
            self.0
                .method_hash([62, 51, 130, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPositionData` (0x7384504f) function
        pub fn get_position_data(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                bool,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([115, 132, 80, 79], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `holdingPosition` (0x9c5f00c2) function
        pub fn holding_position(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([156, 95, 0, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ignorePause` (0x9959af94) function
        pub fn ignore_pause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([153, 89, 175, 148], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `increaseShareSupplyCap` (0xb0646e27) function
        pub fn increase_share_supply_cap(
            &self,
            new_share_supply_cap: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 100, 110, 39], new_share_supply_cap)
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
        ///Calls the contract's `isPaused` (0xb187bd26) function
        pub fn is_paused(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([177, 135, 189, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPositionUsed` (0x93bbeac0) function
        pub fn is_position_used(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([147, 187, 234, 192], p0)
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
        ///Calls the contract's `liftShutdown` (0x5e2c576e) function
        pub fn lift_shutdown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 44, 87, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `locked` (0xcf309012) function
        pub fn locked(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([207, 48, 144, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxDeposit` (0x402d267d) function
        pub fn max_deposit(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 45, 38, 125], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxMint` (0xc63d75b6) function
        pub fn max_mint(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 61, 117, 182], p0)
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
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 150, 80, 216], data)
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
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
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
        ///Calls the contract's `positionCatalogue` (0xcbdf33d0) function
        pub fn position_catalogue(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([203, 223, 51, 208], p0)
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
        ///Calls the contract's `priceRouter` (0xd7d4bf45) function
        pub fn price_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([215, 212, 191, 69], ())
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
        ///Calls the contract's `removeAdaptorFromCatalogue` (0x5f6b88a0) function
        pub fn remove_adaptor_from_catalogue(
            &self,
            adaptor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 107, 136, 160], adaptor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePosition` (0x33e15be2) function
        pub fn remove_position(
            &self,
            index: u32,
            in_debt_array: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 225, 91, 226], (index, in_debt_array))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removePositionFromCatalogue` (0xd1e88404) function
        pub fn remove_position_from_catalogue(
            &self,
            position_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 232, 132, 4], position_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAutomationActions` (0xc8e81950) function
        pub fn set_automation_actions(
            &self,
            registry_id: ::ethers::core::types::U256,
            expected_automation_actions: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [200, 232, 25, 80],
                    (registry_id, expected_automation_actions),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setHoldingPosition` (0x0780fd3a) function
        pub fn set_holding_position(
            &self,
            position_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 128, 253, 58], position_id)
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
        ///Calls the contract's `setStrategistPlatformCut` (0xb5292a99) function
        pub fn set_strategist_platform_cut(
            &self,
            cut: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 41, 42, 153], cut)
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
        ///Calls the contract's `shareSupplyCap` (0xd446bbcc) function
        pub fn share_supply_cap(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([212, 70, 187, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapPositions` (0x379e0b13) function
        pub fn swap_positions(
            &self,
            index_1: u32,
            index_2: u32,
            in_debt_array: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 158, 11, 19], (index_1, index_2, in_debt_array))
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
        ///Calls the contract's `toggleIgnorePause` (0xa373e3ff) function
        pub fn toggle_ignore_pause(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 115, 227, 255], ())
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
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `userShareLockStartTime` (0x687c2b50) function
        pub fn user_share_lock_start_time(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 124, 43, 80], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `viewPositionBalances` (0x78e0233e) function
        pub fn view_position_balances(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::Address>,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::std::vec::Vec<bool>,
            ),
        > {
            self.0
                .method_hash([120, 224, 35, 62], ())
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
        ///Gets the contract's `AdaptorCalled` event
        pub fn adaptor_called_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdaptorCalledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `AdaptorCatalogueAltered` event
        pub fn adaptor_catalogue_altered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdaptorCatalogueAlteredFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `Cellar__AutomationActionsUpdated` event
        pub fn cellar_automation_actions_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CellarAutomationActionsUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
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
        ///Gets the contract's `PositionCatalogueAltered` event
        pub fn position_catalogue_altered_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PositionCatalogueAlteredFilter,
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CellarWithShareLockPeriodV1Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CellarWithShareLockPeriodV1<M> {
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
    ///Custom Error type `Cellar__CallToAdaptorNotAllowed` with signature `Cellar__CallToAdaptorNotAllowed(address)` and selector `0xbbed6c2e`
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
        name = "Cellar__CallToAdaptorNotAllowed",
        abi = "Cellar__CallToAdaptorNotAllowed(address)"
    )]
    pub struct Cellar__CallToAdaptorNotAllowed {
        pub adaptor: ::ethers::core::types::Address,
    }
    ///Custom Error type `Cellar__CallerNotApprovedToRebalance` with signature `Cellar__CallerNotApprovedToRebalance()` and selector `0x3fd29235`
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
        name = "Cellar__CallerNotApprovedToRebalance",
        abi = "Cellar__CallerNotApprovedToRebalance()"
    )]
    pub struct Cellar__CallerNotApprovedToRebalance;
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
    ///Custom Error type `Cellar__DebtMismatch` with signature `Cellar__DebtMismatch(uint32)` and selector `0x563a17a6`
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
    #[etherror(name = "Cellar__DebtMismatch", abi = "Cellar__DebtMismatch(uint32)")]
    pub struct Cellar__DebtMismatch {
        pub position: u32,
    }
    ///Custom Error type `Cellar__ExpectedAddressDoesNotMatchActual` with signature `Cellar__ExpectedAddressDoesNotMatchActual()` and selector `0x4ee204d7`
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
        name = "Cellar__ExpectedAddressDoesNotMatchActual",
        abi = "Cellar__ExpectedAddressDoesNotMatchActual()"
    )]
    pub struct Cellar__ExpectedAddressDoesNotMatchActual;
    ///Custom Error type `Cellar__FailedToForceOutPosition` with signature `Cellar__FailedToForceOutPosition()` and selector `0xd4db0b79`
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
        name = "Cellar__FailedToForceOutPosition",
        abi = "Cellar__FailedToForceOutPosition()"
    )]
    pub struct Cellar__FailedToForceOutPosition;
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
    ///Custom Error type `Cellar__InvalidHoldingPosition` with signature `Cellar__InvalidHoldingPosition(uint32)` and selector `0xa42c0f90`
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
        name = "Cellar__InvalidHoldingPosition",
        abi = "Cellar__InvalidHoldingPosition(uint32)"
    )]
    pub struct Cellar__InvalidHoldingPosition {
        pub position_id: u32,
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
    ///Custom Error type `Cellar__InvalidShareSupplyCap` with signature `Cellar__InvalidShareSupplyCap()` and selector `0x34f1ec1b`
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
        name = "Cellar__InvalidShareSupplyCap",
        abi = "Cellar__InvalidShareSupplyCap()"
    )]
    pub struct Cellar__InvalidShareSupplyCap;
    ///Custom Error type `Cellar__MinimumConstructorMintNotMet` with signature `Cellar__MinimumConstructorMintNotMet()` and selector `0x5a1a4a32`
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
        name = "Cellar__MinimumConstructorMintNotMet",
        abi = "Cellar__MinimumConstructorMintNotMet()"
    )]
    pub struct Cellar__MinimumConstructorMintNotMet;
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
    ///Custom Error type `Cellar__Paused` with signature `Cellar__Paused()` and selector `0xf301f8f0`
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
    #[etherror(name = "Cellar__Paused", abi = "Cellar__Paused()")]
    pub struct Cellar__Paused;
    ///Custom Error type `Cellar__PositionAlreadyUsed` with signature `Cellar__PositionAlreadyUsed(uint32)` and selector `0x66b129f6`
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
        abi = "Cellar__PositionAlreadyUsed(uint32)"
    )]
    pub struct Cellar__PositionAlreadyUsed {
        pub position: u32,
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
    ///Custom Error type `Cellar__PositionNotEmpty` with signature `Cellar__PositionNotEmpty(uint32,uint256)` and selector `0xe3dca368`
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
        abi = "Cellar__PositionNotEmpty(uint32,uint256)"
    )]
    pub struct Cellar__PositionNotEmpty {
        pub position: u32,
        pub shares_remaining: ::ethers::core::types::U256,
    }
    ///Custom Error type `Cellar__PositionNotInCatalogue` with signature `Cellar__PositionNotInCatalogue(uint32)` and selector `0xfced80e8`
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
        name = "Cellar__PositionNotInCatalogue",
        abi = "Cellar__PositionNotInCatalogue(uint32)"
    )]
    pub struct Cellar__PositionNotInCatalogue {
        pub position: u32,
    }
    ///Custom Error type `Cellar__PositionNotUsed` with signature `Cellar__PositionNotUsed(uint32)` and selector `0x70abe859`
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
        name = "Cellar__PositionNotUsed",
        abi = "Cellar__PositionNotUsed(uint32)"
    )]
    pub struct Cellar__PositionNotUsed {
        pub position: u32,
    }
    ///Custom Error type `Cellar__RemovingHoldingPosition` with signature `Cellar__RemovingHoldingPosition()` and selector `0x677b5cc4`
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
        name = "Cellar__RemovingHoldingPosition",
        abi = "Cellar__RemovingHoldingPosition()"
    )]
    pub struct Cellar__RemovingHoldingPosition;
    ///Custom Error type `Cellar__SettingValueToRegistryIdZeroIsProhibited` with signature `Cellar__SettingValueToRegistryIdZeroIsProhibited()` and selector `0x2db38d05`
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
        name = "Cellar__SettingValueToRegistryIdZeroIsProhibited",
        abi = "Cellar__SettingValueToRegistryIdZeroIsProhibited()"
    )]
    pub struct Cellar__SettingValueToRegistryIdZeroIsProhibited;
    ///Custom Error type `Cellar__ShareSupplyCapExceeded` with signature `Cellar__ShareSupplyCapExceeded()` and selector `0xadea3dfd`
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
        name = "Cellar__ShareSupplyCapExceeded",
        abi = "Cellar__ShareSupplyCapExceeded()"
    )]
    pub struct Cellar__ShareSupplyCapExceeded;
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
        pub time_shares_are_unlocked: ::ethers::core::types::U256,
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
    pub enum CellarWithShareLockPeriodV1Errors {
        Cellar__AssetMismatch(Cellar__AssetMismatch),
        Cellar__CallToAdaptorNotAllowed(Cellar__CallToAdaptorNotAllowed),
        Cellar__CallerNotApprovedToRebalance(Cellar__CallerNotApprovedToRebalance),
        Cellar__ContractNotShutdown(Cellar__ContractNotShutdown),
        Cellar__ContractShutdown(Cellar__ContractShutdown),
        Cellar__DebtMismatch(Cellar__DebtMismatch),
        Cellar__ExpectedAddressDoesNotMatchActual(
            Cellar__ExpectedAddressDoesNotMatchActual,
        ),
        Cellar__FailedToForceOutPosition(Cellar__FailedToForceOutPosition),
        Cellar__IlliquidWithdraw(Cellar__IlliquidWithdraw),
        Cellar__IncompleteWithdraw(Cellar__IncompleteWithdraw),
        Cellar__InvalidFee(Cellar__InvalidFee),
        Cellar__InvalidFeeCut(Cellar__InvalidFeeCut),
        Cellar__InvalidHoldingPosition(Cellar__InvalidHoldingPosition),
        Cellar__InvalidRebalanceDeviation(Cellar__InvalidRebalanceDeviation),
        Cellar__InvalidShareLockPeriod(Cellar__InvalidShareLockPeriod),
        Cellar__InvalidShareSupplyCap(Cellar__InvalidShareSupplyCap),
        Cellar__MinimumConstructorMintNotMet(Cellar__MinimumConstructorMintNotMet),
        Cellar__NotApprovedToDepositOnBehalf(Cellar__NotApprovedToDepositOnBehalf),
        Cellar__Paused(Cellar__Paused),
        Cellar__PositionAlreadyUsed(Cellar__PositionAlreadyUsed),
        Cellar__PositionArrayFull(Cellar__PositionArrayFull),
        Cellar__PositionNotEmpty(Cellar__PositionNotEmpty),
        Cellar__PositionNotInCatalogue(Cellar__PositionNotInCatalogue),
        Cellar__PositionNotUsed(Cellar__PositionNotUsed),
        Cellar__RemovingHoldingPosition(Cellar__RemovingHoldingPosition),
        Cellar__SettingValueToRegistryIdZeroIsProhibited(
            Cellar__SettingValueToRegistryIdZeroIsProhibited,
        ),
        Cellar__ShareSupplyCapExceeded(Cellar__ShareSupplyCapExceeded),
        Cellar__SharesAreLocked(Cellar__SharesAreLocked),
        Cellar__TotalAssetDeviatedOutsideRange(Cellar__TotalAssetDeviatedOutsideRange),
        Cellar__TotalSharesMustRemainConstant(Cellar__TotalSharesMustRemainConstant),
        Cellar__ZeroAssets(Cellar__ZeroAssets),
        Cellar__ZeroShares(Cellar__ZeroShares),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CellarWithShareLockPeriodV1Errors {
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
            if let Ok(decoded) = <Cellar__CallToAdaptorNotAllowed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__CallToAdaptorNotAllowed(decoded));
            }
            if let Ok(decoded) = <Cellar__CallerNotApprovedToRebalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__CallerNotApprovedToRebalance(decoded));
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
            if let Ok(decoded) = <Cellar__DebtMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__DebtMismatch(decoded));
            }
            if let Ok(decoded) = <Cellar__ExpectedAddressDoesNotMatchActual as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ExpectedAddressDoesNotMatchActual(decoded));
            }
            if let Ok(decoded) = <Cellar__FailedToForceOutPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__FailedToForceOutPosition(decoded));
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
            if let Ok(decoded) = <Cellar__InvalidHoldingPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidHoldingPosition(decoded));
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
            if let Ok(decoded) = <Cellar__InvalidShareSupplyCap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__InvalidShareSupplyCap(decoded));
            }
            if let Ok(decoded) = <Cellar__MinimumConstructorMintNotMet as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__MinimumConstructorMintNotMet(decoded));
            }
            if let Ok(decoded) = <Cellar__NotApprovedToDepositOnBehalf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__NotApprovedToDepositOnBehalf(decoded));
            }
            if let Ok(decoded) = <Cellar__Paused as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__Paused(decoded));
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
            if let Ok(decoded) = <Cellar__PositionNotInCatalogue as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionNotInCatalogue(decoded));
            }
            if let Ok(decoded) = <Cellar__PositionNotUsed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__PositionNotUsed(decoded));
            }
            if let Ok(decoded) = <Cellar__RemovingHoldingPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__RemovingHoldingPosition(decoded));
            }
            if let Ok(decoded) = <Cellar__SettingValueToRegistryIdZeroIsProhibited as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::Cellar__SettingValueToRegistryIdZeroIsProhibited(decoded),
                );
            }
            if let Ok(decoded) = <Cellar__ShareSupplyCapExceeded as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cellar__ShareSupplyCapExceeded(decoded));
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
    impl ::ethers::core::abi::AbiEncode for CellarWithShareLockPeriodV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Cellar__AssetMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__CallToAdaptorNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__CallerNotApprovedToRebalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ContractNotShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ContractShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__DebtMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ExpectedAddressDoesNotMatchActual(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__FailedToForceOutPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__IlliquidWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__IncompleteWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidFeeCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidHoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidRebalanceDeviation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__InvalidShareSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__MinimumConstructorMintNotMet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__NotApprovedToDepositOnBehalf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__Paused(element) => {
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
                Self::Cellar__PositionNotInCatalogue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__PositionNotUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__RemovingHoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__SettingValueToRegistryIdZeroIsProhibited(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Cellar__ShareSupplyCapExceeded(element) => {
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
    impl ::ethers::contract::ContractRevert for CellarWithShareLockPeriodV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <Cellar__AssetMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__CallToAdaptorNotAllowed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__CallerNotApprovedToRebalance as ::ethers::contract::EthError>::selector() => {
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
                    == <Cellar__DebtMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ExpectedAddressDoesNotMatchActual as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__FailedToForceOutPosition as ::ethers::contract::EthError>::selector() => {
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
                    == <Cellar__InvalidFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidFeeCut as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__InvalidHoldingPosition as ::ethers::contract::EthError>::selector() => {
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
                    == <Cellar__InvalidShareSupplyCap as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__MinimumConstructorMintNotMet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__NotApprovedToDepositOnBehalf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__Paused as ::ethers::contract::EthError>::selector() => {
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
                    == <Cellar__PositionNotInCatalogue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__PositionNotUsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__RemovingHoldingPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__SettingValueToRegistryIdZeroIsProhibited as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Cellar__ShareSupplyCapExceeded as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for CellarWithShareLockPeriodV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cellar__AssetMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__CallToAdaptorNotAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__CallerNotApprovedToRebalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ContractNotShutdown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ContractShutdown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__DebtMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ExpectedAddressDoesNotMatchActual(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__FailedToForceOutPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__IlliquidWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__IncompleteWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidFeeCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidHoldingPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__InvalidShareSupplyCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__MinimumConstructorMintNotMet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__NotApprovedToDepositOnBehalf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::Cellar__PositionAlreadyUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionArrayFull(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionNotEmpty(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionNotInCatalogue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__PositionNotUsed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__RemovingHoldingPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__SettingValueToRegistryIdZeroIsProhibited(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Cellar__ShareSupplyCapExceeded(element) => {
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
    impl ::core::convert::From<::std::string::String>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Cellar__AssetMismatch>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__AssetMismatch) -> Self {
            Self::Cellar__AssetMismatch(value)
        }
    }
    impl ::core::convert::From<Cellar__CallToAdaptorNotAllowed>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__CallToAdaptorNotAllowed) -> Self {
            Self::Cellar__CallToAdaptorNotAllowed(value)
        }
    }
    impl ::core::convert::From<Cellar__CallerNotApprovedToRebalance>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__CallerNotApprovedToRebalance) -> Self {
            Self::Cellar__CallerNotApprovedToRebalance(value)
        }
    }
    impl ::core::convert::From<Cellar__ContractNotShutdown>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__ContractNotShutdown) -> Self {
            Self::Cellar__ContractNotShutdown(value)
        }
    }
    impl ::core::convert::From<Cellar__ContractShutdown>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__ContractShutdown) -> Self {
            Self::Cellar__ContractShutdown(value)
        }
    }
    impl ::core::convert::From<Cellar__DebtMismatch>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__DebtMismatch) -> Self {
            Self::Cellar__DebtMismatch(value)
        }
    }
    impl ::core::convert::From<Cellar__ExpectedAddressDoesNotMatchActual>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__ExpectedAddressDoesNotMatchActual) -> Self {
            Self::Cellar__ExpectedAddressDoesNotMatchActual(value)
        }
    }
    impl ::core::convert::From<Cellar__FailedToForceOutPosition>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__FailedToForceOutPosition) -> Self {
            Self::Cellar__FailedToForceOutPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__IlliquidWithdraw>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__IlliquidWithdraw) -> Self {
            Self::Cellar__IlliquidWithdraw(value)
        }
    }
    impl ::core::convert::From<Cellar__IncompleteWithdraw>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__IncompleteWithdraw) -> Self {
            Self::Cellar__IncompleteWithdraw(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidFee>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidFee) -> Self {
            Self::Cellar__InvalidFee(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidFeeCut>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidFeeCut) -> Self {
            Self::Cellar__InvalidFeeCut(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidHoldingPosition>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidHoldingPosition) -> Self {
            Self::Cellar__InvalidHoldingPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidRebalanceDeviation>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidRebalanceDeviation) -> Self {
            Self::Cellar__InvalidRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidShareLockPeriod>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidShareLockPeriod) -> Self {
            Self::Cellar__InvalidShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<Cellar__InvalidShareSupplyCap>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__InvalidShareSupplyCap) -> Self {
            Self::Cellar__InvalidShareSupplyCap(value)
        }
    }
    impl ::core::convert::From<Cellar__MinimumConstructorMintNotMet>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__MinimumConstructorMintNotMet) -> Self {
            Self::Cellar__MinimumConstructorMintNotMet(value)
        }
    }
    impl ::core::convert::From<Cellar__NotApprovedToDepositOnBehalf>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__NotApprovedToDepositOnBehalf) -> Self {
            Self::Cellar__NotApprovedToDepositOnBehalf(value)
        }
    }
    impl ::core::convert::From<Cellar__Paused> for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__Paused) -> Self {
            Self::Cellar__Paused(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionAlreadyUsed>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__PositionAlreadyUsed) -> Self {
            Self::Cellar__PositionAlreadyUsed(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionArrayFull>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__PositionArrayFull) -> Self {
            Self::Cellar__PositionArrayFull(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionNotEmpty>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__PositionNotEmpty) -> Self {
            Self::Cellar__PositionNotEmpty(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionNotInCatalogue>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__PositionNotInCatalogue) -> Self {
            Self::Cellar__PositionNotInCatalogue(value)
        }
    }
    impl ::core::convert::From<Cellar__PositionNotUsed>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__PositionNotUsed) -> Self {
            Self::Cellar__PositionNotUsed(value)
        }
    }
    impl ::core::convert::From<Cellar__RemovingHoldingPosition>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__RemovingHoldingPosition) -> Self {
            Self::Cellar__RemovingHoldingPosition(value)
        }
    }
    impl ::core::convert::From<Cellar__SettingValueToRegistryIdZeroIsProhibited>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__SettingValueToRegistryIdZeroIsProhibited) -> Self {
            Self::Cellar__SettingValueToRegistryIdZeroIsProhibited(value)
        }
    }
    impl ::core::convert::From<Cellar__ShareSupplyCapExceeded>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__ShareSupplyCapExceeded) -> Self {
            Self::Cellar__ShareSupplyCapExceeded(value)
        }
    }
    impl ::core::convert::From<Cellar__SharesAreLocked>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__SharesAreLocked) -> Self {
            Self::Cellar__SharesAreLocked(value)
        }
    }
    impl ::core::convert::From<Cellar__TotalAssetDeviatedOutsideRange>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__TotalAssetDeviatedOutsideRange) -> Self {
            Self::Cellar__TotalAssetDeviatedOutsideRange(value)
        }
    }
    impl ::core::convert::From<Cellar__TotalSharesMustRemainConstant>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__TotalSharesMustRemainConstant) -> Self {
            Self::Cellar__TotalSharesMustRemainConstant(value)
        }
    }
    impl ::core::convert::From<Cellar__ZeroAssets>
    for CellarWithShareLockPeriodV1Errors {
        fn from(value: Cellar__ZeroAssets) -> Self {
            Self::Cellar__ZeroAssets(value)
        }
    }
    impl ::core::convert::From<Cellar__ZeroShares>
    for CellarWithShareLockPeriodV1Errors {
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
    #[ethevent(name = "AdaptorCalled", abi = "AdaptorCalled(address,bytes)")]
    pub struct AdaptorCalledFilter {
        pub adaptor: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
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
        name = "AdaptorCatalogueAltered",
        abi = "AdaptorCatalogueAltered(address,bool)"
    )]
    pub struct AdaptorCatalogueAlteredFilter {
        pub adaptor: ::ethers::core::types::Address,
        pub in_catalogue: bool,
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
    #[ethevent(
        name = "Cellar__AutomationActionsUpdated",
        abi = "Cellar__AutomationActionsUpdated(address)"
    )]
    pub struct CellarAutomationActionsUpdatedFilter {
        pub new_automation_actions: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
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
    #[ethevent(name = "PositionAdded", abi = "PositionAdded(uint32,uint256)")]
    pub struct PositionAddedFilter {
        pub position: u32,
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
        name = "PositionCatalogueAltered",
        abi = "PositionCatalogueAltered(uint32,bool)"
    )]
    pub struct PositionCatalogueAlteredFilter {
        pub position_id: u32,
        pub in_catalogue: bool,
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
    #[ethevent(name = "PositionRemoved", abi = "PositionRemoved(uint32,uint256)")]
    pub struct PositionRemovedFilter {
        pub position: u32,
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
        abi = "PositionSwapped(uint32,uint32,uint256,uint256)"
    )]
    pub struct PositionSwappedFilter {
        pub new_position_1: u32,
        pub new_position_2: u32,
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
    pub enum CellarWithShareLockPeriodV1Events {
        AdaptorCalledFilter(AdaptorCalledFilter),
        AdaptorCatalogueAlteredFilter(AdaptorCatalogueAlteredFilter),
        ApprovalFilter(ApprovalFilter),
        CellarAutomationActionsUpdatedFilter(CellarAutomationActionsUpdatedFilter),
        DepositFilter(DepositFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PositionAddedFilter(PositionAddedFilter),
        PositionCatalogueAlteredFilter(PositionCatalogueAlteredFilter),
        PositionRemovedFilter(PositionRemovedFilter),
        PositionSwappedFilter(PositionSwappedFilter),
        RebalanceDeviationChangedFilter(RebalanceDeviationChangedFilter),
        ShareLockingPeriodChangedFilter(ShareLockingPeriodChangedFilter),
        ShutdownChangedFilter(ShutdownChangedFilter),
        StrategistPayoutAddressChangedFilter(StrategistPayoutAddressChangedFilter),
        StrategistPlatformCutChangedFilter(StrategistPlatformCutChangedFilter),
        TransferFilter(TransferFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for CellarWithShareLockPeriodV1Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdaptorCalledFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::AdaptorCalledFilter(decoded),
                );
            }
            if let Ok(decoded) = AdaptorCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::AdaptorCatalogueAlteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = CellarAutomationActionsUpdatedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::CellarAutomationActionsUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::DepositFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::OwnershipTransferredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PositionAddedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::PositionAddedFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionCatalogueAlteredFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::PositionCatalogueAlteredFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = PositionRemovedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::PositionRemovedFilter(decoded),
                );
            }
            if let Ok(decoded) = PositionSwappedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::PositionSwappedFilter(decoded),
                );
            }
            if let Ok(decoded) = RebalanceDeviationChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::RebalanceDeviationChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ShareLockingPeriodChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::ShareLockingPeriodChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = ShutdownChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::ShutdownChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = StrategistPayoutAddressChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::StrategistPayoutAddressChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = StrategistPlatformCutChangedFilter::decode_log(log) {
                return Ok(
                    CellarWithShareLockPeriodV1Events::StrategistPlatformCutChangedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(CellarWithShareLockPeriodV1Events::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CellarWithShareLockPeriodV1Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdaptorCalledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AdaptorCatalogueAlteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CellarAutomationActionsUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionCatalogueAlteredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionRemovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PositionSwappedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceDeviationChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShareLockingPeriodChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShutdownChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategistPayoutAddressChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StrategistPlatformCutChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdaptorCalledFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: AdaptorCalledFilter) -> Self {
            Self::AdaptorCalledFilter(value)
        }
    }
    impl ::core::convert::From<AdaptorCatalogueAlteredFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: AdaptorCatalogueAlteredFilter) -> Self {
            Self::AdaptorCatalogueAlteredFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for CellarWithShareLockPeriodV1Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<CellarAutomationActionsUpdatedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: CellarAutomationActionsUpdatedFilter) -> Self {
            Self::CellarAutomationActionsUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for CellarWithShareLockPeriodV1Events {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PositionAddedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: PositionAddedFilter) -> Self {
            Self::PositionAddedFilter(value)
        }
    }
    impl ::core::convert::From<PositionCatalogueAlteredFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: PositionCatalogueAlteredFilter) -> Self {
            Self::PositionCatalogueAlteredFilter(value)
        }
    }
    impl ::core::convert::From<PositionRemovedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: PositionRemovedFilter) -> Self {
            Self::PositionRemovedFilter(value)
        }
    }
    impl ::core::convert::From<PositionSwappedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: PositionSwappedFilter) -> Self {
            Self::PositionSwappedFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceDeviationChangedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: RebalanceDeviationChangedFilter) -> Self {
            Self::RebalanceDeviationChangedFilter(value)
        }
    }
    impl ::core::convert::From<ShareLockingPeriodChangedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: ShareLockingPeriodChangedFilter) -> Self {
            Self::ShareLockingPeriodChangedFilter(value)
        }
    }
    impl ::core::convert::From<ShutdownChangedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: ShutdownChangedFilter) -> Self {
            Self::ShutdownChangedFilter(value)
        }
    }
    impl ::core::convert::From<StrategistPayoutAddressChangedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: StrategistPayoutAddressChangedFilter) -> Self {
            Self::StrategistPayoutAddressChangedFilter(value)
        }
    }
    impl ::core::convert::From<StrategistPlatformCutChangedFilter>
    for CellarWithShareLockPeriodV1Events {
        fn from(value: StrategistPlatformCutChangedFilter) -> Self {
            Self::StrategistPlatformCutChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for CellarWithShareLockPeriodV1Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for CellarWithShareLockPeriodV1Events {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
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
    ///Container type for all input parameters for the `GRAVITY_BRIDGE_REGISTRY_SLOT` function with signature `GRAVITY_BRIDGE_REGISTRY_SLOT()` and selector `0xcd82f8b1`
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
        name = "GRAVITY_BRIDGE_REGISTRY_SLOT",
        abi = "GRAVITY_BRIDGE_REGISTRY_SLOT()"
    )]
    pub struct GravityBridgeRegistrySlotCall;
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
    ///Container type for all input parameters for the `adaptorCatalogue` function with signature `adaptorCatalogue(address)` and selector `0x18d4c143`
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
    #[ethcall(name = "adaptorCatalogue", abi = "adaptorCatalogue(address)")]
    pub struct AdaptorCatalogueCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `addAdaptorToCatalogue` function with signature `addAdaptorToCatalogue(address)` and selector `0x3d8ab1e5`
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
    #[ethcall(name = "addAdaptorToCatalogue", abi = "addAdaptorToCatalogue(address)")]
    pub struct AddAdaptorToCatalogueCall {
        pub adaptor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `addPosition` function with signature `addPosition(uint32,uint32,bytes,bool)` and selector `0x9955a9d4`
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
    #[ethcall(name = "addPosition", abi = "addPosition(uint32,uint32,bytes,bool)")]
    pub struct AddPositionCall {
        pub index: u32,
        pub position_id: u32,
        pub configuration_data: ::ethers::core::types::Bytes,
        pub in_debt_array: bool,
    }
    ///Container type for all input parameters for the `addPositionToCatalogue` function with signature `addPositionToCatalogue(uint32)` and selector `0x501eb4fe`
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
    #[ethcall(name = "addPositionToCatalogue", abi = "addPositionToCatalogue(uint32)")]
    pub struct AddPositionToCatalogueCall {
        pub position_id: u32,
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
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    ///Container type for all input parameters for the `automationActions` function with signature `automationActions()` and selector `0x88c4caba`
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
    #[ethcall(name = "automationActions", abi = "automationActions()")]
    pub struct AutomationActionsCall;
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
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `blockExternalReceiver` function with signature `blockExternalReceiver()` and selector `0x4c4602da`
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
    #[ethcall(name = "blockExternalReceiver", abi = "blockExternalReceiver()")]
    pub struct BlockExternalReceiverCall;
    ///Container type for all input parameters for the `cachePriceRouter` function with signature `cachePriceRouter(bool,uint16,address)` and selector `0xc588d8d6`
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
    #[ethcall(name = "cachePriceRouter", abi = "cachePriceRouter(bool,uint16,address)")]
    pub struct CachePriceRouterCall {
        pub check_total_assets: bool,
        pub allowable_range: u16,
        pub expected_price_router: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `callOnAdaptor` function with signature `callOnAdaptor((address,bytes[])[])` and selector `0x4e84befe`
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
    #[ethcall(name = "callOnAdaptor", abi = "callOnAdaptor((address,bytes[])[])")]
    pub struct CallOnAdaptorCall {
        pub data: ::std::vec::Vec<AdaptorCall>,
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
    ///Container type for all input parameters for the `creditPositions` function with signature `creditPositions(uint256)` and selector `0x59d20b4e`
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
    #[ethcall(name = "creditPositions", abi = "creditPositions(uint256)")]
    pub struct CreditPositionsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `debtPositions` function with signature `debtPositions(uint256)` and selector `0xe1b1acb7`
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
    #[ethcall(name = "debtPositions", abi = "debtPositions(uint256)")]
    pub struct DebtPositionsCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `decreaseShareSupplyCap` function with signature `decreaseShareSupplyCap(uint192)` and selector `0x575bbce6`
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
    #[ethcall(name = "decreaseShareSupplyCap", abi = "decreaseShareSupplyCap(uint192)")]
    pub struct DecreaseShareSupplyCapCall {
        pub new_share_supply_cap: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `forcePositionOut` function with signature `forcePositionOut(uint32,uint32,bool)` and selector `0xa07bee0b`
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
    #[ethcall(name = "forcePositionOut", abi = "forcePositionOut(uint32,uint32,bool)")]
    pub struct ForcePositionOutCall {
        pub index: u32,
        pub position_id: u32,
        pub in_debt_array: bool,
    }
    ///Container type for all input parameters for the `getCreditPositions` function with signature `getCreditPositions()` and selector `0x71e99dc2`
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
    #[ethcall(name = "getCreditPositions", abi = "getCreditPositions()")]
    pub struct GetCreditPositionsCall;
    ///Container type for all input parameters for the `getDebtPositions` function with signature `getDebtPositions()` and selector `0x3e3382ba`
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
    #[ethcall(name = "getDebtPositions", abi = "getDebtPositions()")]
    pub struct GetDebtPositionsCall;
    ///Container type for all input parameters for the `getPositionData` function with signature `getPositionData(uint32)` and selector `0x7384504f`
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
    #[ethcall(name = "getPositionData", abi = "getPositionData(uint32)")]
    pub struct GetPositionDataCall(pub u32);
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
    ///Container type for all input parameters for the `ignorePause` function with signature `ignorePause()` and selector `0x9959af94`
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
    #[ethcall(name = "ignorePause", abi = "ignorePause()")]
    pub struct IgnorePauseCall;
    ///Container type for all input parameters for the `increaseShareSupplyCap` function with signature `increaseShareSupplyCap(uint192)` and selector `0xb0646e27`
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
    #[ethcall(name = "increaseShareSupplyCap", abi = "increaseShareSupplyCap(uint192)")]
    pub struct IncreaseShareSupplyCapCall {
        pub new_share_supply_cap: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
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
    #[ethcall(name = "isPaused", abi = "isPaused()")]
    pub struct IsPausedCall;
    ///Container type for all input parameters for the `isPositionUsed` function with signature `isPositionUsed(uint256)` and selector `0x93bbeac0`
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
    #[ethcall(name = "isPositionUsed", abi = "isPositionUsed(uint256)")]
    pub struct IsPositionUsedCall(pub ::ethers::core::types::U256);
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
    ///Container type for all input parameters for the `locked` function with signature `locked()` and selector `0xcf309012`
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
    #[ethcall(name = "locked", abi = "locked()")]
    pub struct LockedCall;
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
    pub struct MaxDepositCall(pub ::ethers::core::types::Address);
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
    pub struct MaxMintCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
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
    ///Container type for all input parameters for the `positionCatalogue` function with signature `positionCatalogue(uint32)` and selector `0xcbdf33d0`
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
    #[ethcall(name = "positionCatalogue", abi = "positionCatalogue(uint32)")]
    pub struct PositionCatalogueCall(pub u32);
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
    ///Container type for all input parameters for the `priceRouter` function with signature `priceRouter()` and selector `0xd7d4bf45`
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
    #[ethcall(name = "priceRouter", abi = "priceRouter()")]
    pub struct PriceRouterCall;
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
    ///Container type for all input parameters for the `removeAdaptorFromCatalogue` function with signature `removeAdaptorFromCatalogue(address)` and selector `0x5f6b88a0`
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
        name = "removeAdaptorFromCatalogue",
        abi = "removeAdaptorFromCatalogue(address)"
    )]
    pub struct RemoveAdaptorFromCatalogueCall {
        pub adaptor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `removePosition` function with signature `removePosition(uint32,bool)` and selector `0x33e15be2`
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
    #[ethcall(name = "removePosition", abi = "removePosition(uint32,bool)")]
    pub struct RemovePositionCall {
        pub index: u32,
        pub in_debt_array: bool,
    }
    ///Container type for all input parameters for the `removePositionFromCatalogue` function with signature `removePositionFromCatalogue(uint32)` and selector `0xd1e88404`
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
        name = "removePositionFromCatalogue",
        abi = "removePositionFromCatalogue(uint32)"
    )]
    pub struct RemovePositionFromCatalogueCall {
        pub position_id: u32,
    }
    ///Container type for all input parameters for the `setAutomationActions` function with signature `setAutomationActions(uint256,address)` and selector `0xc8e81950`
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
        name = "setAutomationActions",
        abi = "setAutomationActions(uint256,address)"
    )]
    pub struct SetAutomationActionsCall {
        pub registry_id: ::ethers::core::types::U256,
        pub expected_automation_actions: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setHoldingPosition` function with signature `setHoldingPosition(uint32)` and selector `0x0780fd3a`
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
    #[ethcall(name = "setHoldingPosition", abi = "setHoldingPosition(uint32)")]
    pub struct SetHoldingPositionCall {
        pub position_id: u32,
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
    ///Container type for all input parameters for the `shareSupplyCap` function with signature `shareSupplyCap()` and selector `0xd446bbcc`
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
    #[ethcall(name = "shareSupplyCap", abi = "shareSupplyCap()")]
    pub struct ShareSupplyCapCall;
    ///Container type for all input parameters for the `swapPositions` function with signature `swapPositions(uint32,uint32,bool)` and selector `0x379e0b13`
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
    #[ethcall(name = "swapPositions", abi = "swapPositions(uint32,uint32,bool)")]
    pub struct SwapPositionsCall {
        pub index_1: u32,
        pub index_2: u32,
        pub in_debt_array: bool,
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
    ///Container type for all input parameters for the `toggleIgnorePause` function with signature `toggleIgnorePause()` and selector `0xa373e3ff`
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
    #[ethcall(name = "toggleIgnorePause", abi = "toggleIgnorePause()")]
    pub struct ToggleIgnorePauseCall;
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
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `userShareLockStartTime` function with signature `userShareLockStartTime(address)` and selector `0x687c2b50`
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
    #[ethcall(name = "userShareLockStartTime", abi = "userShareLockStartTime(address)")]
    pub struct UserShareLockStartTimeCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `viewPositionBalances` function with signature `viewPositionBalances()` and selector `0x78e0233e`
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
    #[ethcall(name = "viewPositionBalances", abi = "viewPositionBalances()")]
    pub struct ViewPositionBalancesCall;
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
    pub enum CellarWithShareLockPeriodV1Calls {
        DomainSeparator(DomainSeparatorCall),
        GravityBridgeRegistrySlot(GravityBridgeRegistrySlotCall),
        MaximumShareLockPeriod(MaximumShareLockPeriodCall),
        MaxFeeCut(MaxFeeCutCall),
        MaxPlatformFee(MaxPlatformFeeCall),
        MaxPositions(MaxPositionsCall),
        MaxRebalanceDeviation(MaxRebalanceDeviationCall),
        MinimumShareLockPeriod(MinimumShareLockPeriodCall),
        PriceRouterRegistrySlot(PriceRouterRegistrySlotCall),
        AdaptorCatalogue(AdaptorCatalogueCall),
        AddAdaptorToCatalogue(AddAdaptorToCatalogueCall),
        AddPosition(AddPositionCall),
        AddPositionToCatalogue(AddPositionToCatalogueCall),
        Allowance(AllowanceCall),
        AllowedRebalanceDeviation(AllowedRebalanceDeviationCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        AutomationActions(AutomationActionsCall),
        BalanceOf(BalanceOfCall),
        BlockExternalReceiver(BlockExternalReceiverCall),
        CachePriceRouter(CachePriceRouterCall),
        CallOnAdaptor(CallOnAdaptorCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CreditPositions(CreditPositionsCall),
        DebtPositions(DebtPositionsCall),
        Decimals(DecimalsCall),
        DecreaseShareSupplyCap(DecreaseShareSupplyCapCall),
        Deposit(DepositCall),
        FeeData(FeeDataCall),
        ForcePositionOut(ForcePositionOutCall),
        GetCreditPositions(GetCreditPositionsCall),
        GetDebtPositions(GetDebtPositionsCall),
        GetPositionData(GetPositionDataCall),
        HoldingPosition(HoldingPositionCall),
        IgnorePause(IgnorePauseCall),
        IncreaseShareSupplyCap(IncreaseShareSupplyCapCall),
        InitiateShutdown(InitiateShutdownCall),
        IsPaused(IsPausedCall),
        IsPositionUsed(IsPositionUsedCall),
        IsShutdown(IsShutdownCall),
        LiftShutdown(LiftShutdownCall),
        Locked(LockedCall),
        MaxDeposit(MaxDepositCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        Nonces(NoncesCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        Permit(PermitCall),
        PositionCatalogue(PositionCatalogueCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        PriceRouter(PriceRouterCall),
        Redeem(RedeemCall),
        Registry(RegistryCall),
        RemoveAdaptorFromCatalogue(RemoveAdaptorFromCatalogueCall),
        RemovePosition(RemovePositionCall),
        RemovePositionFromCatalogue(RemovePositionFromCatalogueCall),
        SetAutomationActions(SetAutomationActionsCall),
        SetHoldingPosition(SetHoldingPositionCall),
        SetRebalanceDeviation(SetRebalanceDeviationCall),
        SetShareLockPeriod(SetShareLockPeriodCall),
        SetStrategistPayoutAddress(SetStrategistPayoutAddressCall),
        SetStrategistPlatformCut(SetStrategistPlatformCutCall),
        ShareLockPeriod(ShareLockPeriodCall),
        ShareSupplyCap(ShareSupplyCapCall),
        SwapPositions(SwapPositionsCall),
        Symbol(SymbolCall),
        ToggleIgnorePause(ToggleIgnorePauseCall),
        TotalAssets(TotalAssetsCall),
        TotalAssetsWithdrawable(TotalAssetsWithdrawableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        UserShareLockStartTime(UserShareLockStartTimeCall),
        ViewPositionBalances(ViewPositionBalancesCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for CellarWithShareLockPeriodV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <GravityBridgeRegistrySlotCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GravityBridgeRegistrySlot(decoded));
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
            if let Ok(decoded) = <AdaptorCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdaptorCatalogue(decoded));
            }
            if let Ok(decoded) = <AddAdaptorToCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddAdaptorToCatalogue(decoded));
            }
            if let Ok(decoded) = <AddPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddPosition(decoded));
            }
            if let Ok(decoded) = <AddPositionToCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddPositionToCatalogue(decoded));
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
            if let Ok(decoded) = <AutomationActionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AutomationActions(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BlockExternalReceiverCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BlockExternalReceiver(decoded));
            }
            if let Ok(decoded) = <CachePriceRouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CachePriceRouter(decoded));
            }
            if let Ok(decoded) = <CallOnAdaptorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallOnAdaptor(decoded));
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
            if let Ok(decoded) = <CreditPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreditPositions(decoded));
            }
            if let Ok(decoded) = <DebtPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DebtPositions(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DecreaseShareSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DecreaseShareSupplyCap(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <FeeDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeeData(decoded));
            }
            if let Ok(decoded) = <ForcePositionOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ForcePositionOut(decoded));
            }
            if let Ok(decoded) = <GetCreditPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCreditPositions(decoded));
            }
            if let Ok(decoded) = <GetDebtPositionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDebtPositions(decoded));
            }
            if let Ok(decoded) = <GetPositionDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPositionData(decoded));
            }
            if let Ok(decoded) = <HoldingPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HoldingPosition(decoded));
            }
            if let Ok(decoded) = <IgnorePauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IgnorePause(decoded));
            }
            if let Ok(decoded) = <IncreaseShareSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncreaseShareSupplyCap(decoded));
            }
            if let Ok(decoded) = <InitiateShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitiateShutdown(decoded));
            }
            if let Ok(decoded) = <IsPausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPaused(decoded));
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
            if let Ok(decoded) = <LiftShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiftShutdown(decoded));
            }
            if let Ok(decoded) = <LockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
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
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Multicall(decoded));
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
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
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
            if let Ok(decoded) = <PositionCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PositionCatalogue(decoded));
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
            if let Ok(decoded) = <PriceRouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PriceRouter(decoded));
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
            if let Ok(decoded) = <RemoveAdaptorFromCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveAdaptorFromCatalogue(decoded));
            }
            if let Ok(decoded) = <RemovePositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePosition(decoded));
            }
            if let Ok(decoded) = <RemovePositionFromCatalogueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemovePositionFromCatalogue(decoded));
            }
            if let Ok(decoded) = <SetAutomationActionsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAutomationActions(decoded));
            }
            if let Ok(decoded) = <SetHoldingPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetHoldingPosition(decoded));
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
            if let Ok(decoded) = <SetStrategistPlatformCutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStrategistPlatformCut(decoded));
            }
            if let Ok(decoded) = <ShareLockPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShareLockPeriod(decoded));
            }
            if let Ok(decoded) = <ShareSupplyCapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ShareSupplyCap(decoded));
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
            if let Ok(decoded) = <ToggleIgnorePauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToggleIgnorePause(decoded));
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
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UserShareLockStartTimeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UserShareLockStartTime(decoded));
            }
            if let Ok(decoded) = <ViewPositionBalancesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ViewPositionBalances(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CellarWithShareLockPeriodV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GravityBridgeRegistrySlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaximumShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MaxFeeCut(element) => {
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
                Self::AdaptorCatalogue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddAdaptorToCatalogue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddPositionToCatalogue(element) => {
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
                Self::AutomationActions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BlockExternalReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CachePriceRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallOnAdaptor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CreditPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DebtPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DecreaseShareSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FeeData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ForcePositionOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCreditPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDebtPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPositionData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HoldingPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IgnorePause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncreaseShareSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitiateShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPaused(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPositionUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiftShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PositionCatalogue(element) => {
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
                Self::PriceRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Registry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveAdaptorFromCatalogue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovePositionFromCatalogue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAutomationActions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetHoldingPosition(element) => {
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
                Self::SetStrategistPlatformCut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShareLockPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ShareSupplyCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapPositions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ToggleIgnorePause(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UserShareLockStartTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ViewPositionBalances(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for CellarWithShareLockPeriodV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::GravityBridgeRegistrySlot(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaximumShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MaxFeeCut(element) => ::core::fmt::Display::fmt(element, f),
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
                Self::AdaptorCatalogue(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddAdaptorToCatalogue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddPositionToCatalogue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowedRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::AutomationActions(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BlockExternalReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CachePriceRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallOnAdaptor(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreditPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::DebtPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseShareSupplyCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeData(element) => ::core::fmt::Display::fmt(element, f),
                Self::ForcePositionOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCreditPositions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDebtPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPositionData(element) => ::core::fmt::Display::fmt(element, f),
                Self::HoldingPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::IgnorePause(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseShareSupplyCap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitiateShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPaused(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPositionUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiftShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionCatalogue(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveAdaptorFromCatalogue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RemovePosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovePositionFromCatalogue(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAutomationActions(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetHoldingPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetRebalanceDeviation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetShareLockPeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStrategistPayoutAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStrategistPlatformCut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShareLockPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShareSupplyCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapPositions(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToggleIgnorePause(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssetsWithdrawable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserShareLockStartTime(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ViewPositionBalances(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<GravityBridgeRegistrySlotCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: GravityBridgeRegistrySlotCall) -> Self {
            Self::GravityBridgeRegistrySlot(value)
        }
    }
    impl ::core::convert::From<MaximumShareLockPeriodCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaximumShareLockPeriodCall) -> Self {
            Self::MaximumShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<MaxFeeCutCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxFeeCutCall) -> Self {
            Self::MaxFeeCut(value)
        }
    }
    impl ::core::convert::From<MaxPlatformFeeCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxPlatformFeeCall) -> Self {
            Self::MaxPlatformFee(value)
        }
    }
    impl ::core::convert::From<MaxPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxPositionsCall) -> Self {
            Self::MaxPositions(value)
        }
    }
    impl ::core::convert::From<MaxRebalanceDeviationCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxRebalanceDeviationCall) -> Self {
            Self::MaxRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<MinimumShareLockPeriodCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: MinimumShareLockPeriodCall) -> Self {
            Self::MinimumShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<PriceRouterRegistrySlotCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: PriceRouterRegistrySlotCall) -> Self {
            Self::PriceRouterRegistrySlot(value)
        }
    }
    impl ::core::convert::From<AdaptorCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: AdaptorCatalogueCall) -> Self {
            Self::AdaptorCatalogue(value)
        }
    }
    impl ::core::convert::From<AddAdaptorToCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: AddAdaptorToCatalogueCall) -> Self {
            Self::AddAdaptorToCatalogue(value)
        }
    }
    impl ::core::convert::From<AddPositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: AddPositionCall) -> Self {
            Self::AddPosition(value)
        }
    }
    impl ::core::convert::From<AddPositionToCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: AddPositionToCatalogueCall) -> Self {
            Self::AddPositionToCatalogue(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<AllowedRebalanceDeviationCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: AllowedRebalanceDeviationCall) -> Self {
            Self::AllowedRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<AutomationActionsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: AutomationActionsCall) -> Self {
            Self::AutomationActions(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BlockExternalReceiverCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: BlockExternalReceiverCall) -> Self {
            Self::BlockExternalReceiver(value)
        }
    }
    impl ::core::convert::From<CachePriceRouterCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: CachePriceRouterCall) -> Self {
            Self::CachePriceRouter(value)
        }
    }
    impl ::core::convert::From<CallOnAdaptorCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: CallOnAdaptorCall) -> Self {
            Self::CallOnAdaptor(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<CreditPositionsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: CreditPositionsCall) -> Self {
            Self::CreditPositions(value)
        }
    }
    impl ::core::convert::From<DebtPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: DebtPositionsCall) -> Self {
            Self::DebtPositions(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DecreaseShareSupplyCapCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: DecreaseShareSupplyCapCall) -> Self {
            Self::DecreaseShareSupplyCap(value)
        }
    }
    impl ::core::convert::From<DepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<FeeDataCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: FeeDataCall) -> Self {
            Self::FeeData(value)
        }
    }
    impl ::core::convert::From<ForcePositionOutCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ForcePositionOutCall) -> Self {
            Self::ForcePositionOut(value)
        }
    }
    impl ::core::convert::From<GetCreditPositionsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: GetCreditPositionsCall) -> Self {
            Self::GetCreditPositions(value)
        }
    }
    impl ::core::convert::From<GetDebtPositionsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: GetDebtPositionsCall) -> Self {
            Self::GetDebtPositions(value)
        }
    }
    impl ::core::convert::From<GetPositionDataCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: GetPositionDataCall) -> Self {
            Self::GetPositionData(value)
        }
    }
    impl ::core::convert::From<HoldingPositionCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: HoldingPositionCall) -> Self {
            Self::HoldingPosition(value)
        }
    }
    impl ::core::convert::From<IgnorePauseCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: IgnorePauseCall) -> Self {
            Self::IgnorePause(value)
        }
    }
    impl ::core::convert::From<IncreaseShareSupplyCapCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: IncreaseShareSupplyCapCall) -> Self {
            Self::IncreaseShareSupplyCap(value)
        }
    }
    impl ::core::convert::From<InitiateShutdownCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: InitiateShutdownCall) -> Self {
            Self::InitiateShutdown(value)
        }
    }
    impl ::core::convert::From<IsPausedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: IsPausedCall) -> Self {
            Self::IsPaused(value)
        }
    }
    impl ::core::convert::From<IsPositionUsedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: IsPositionUsedCall) -> Self {
            Self::IsPositionUsed(value)
        }
    }
    impl ::core::convert::From<IsShutdownCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: IsShutdownCall) -> Self {
            Self::IsShutdown(value)
        }
    }
    impl ::core::convert::From<LiftShutdownCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: LiftShutdownCall) -> Self {
            Self::LiftShutdown(value)
        }
    }
    impl ::core::convert::From<LockedCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: LockedCall) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<NameCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PositionCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: PositionCatalogueCall) -> Self {
            Self::PositionCatalogue(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<PriceRouterCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: PriceRouterCall) -> Self {
            Self::PriceRouter(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<RemoveAdaptorFromCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: RemoveAdaptorFromCatalogueCall) -> Self {
            Self::RemoveAdaptorFromCatalogue(value)
        }
    }
    impl ::core::convert::From<RemovePositionCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: RemovePositionCall) -> Self {
            Self::RemovePosition(value)
        }
    }
    impl ::core::convert::From<RemovePositionFromCatalogueCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: RemovePositionFromCatalogueCall) -> Self {
            Self::RemovePositionFromCatalogue(value)
        }
    }
    impl ::core::convert::From<SetAutomationActionsCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetAutomationActionsCall) -> Self {
            Self::SetAutomationActions(value)
        }
    }
    impl ::core::convert::From<SetHoldingPositionCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetHoldingPositionCall) -> Self {
            Self::SetHoldingPosition(value)
        }
    }
    impl ::core::convert::From<SetRebalanceDeviationCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetRebalanceDeviationCall) -> Self {
            Self::SetRebalanceDeviation(value)
        }
    }
    impl ::core::convert::From<SetShareLockPeriodCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetShareLockPeriodCall) -> Self {
            Self::SetShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<SetStrategistPayoutAddressCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetStrategistPayoutAddressCall) -> Self {
            Self::SetStrategistPayoutAddress(value)
        }
    }
    impl ::core::convert::From<SetStrategistPlatformCutCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: SetStrategistPlatformCutCall) -> Self {
            Self::SetStrategistPlatformCut(value)
        }
    }
    impl ::core::convert::From<ShareLockPeriodCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ShareLockPeriodCall) -> Self {
            Self::ShareLockPeriod(value)
        }
    }
    impl ::core::convert::From<ShareSupplyCapCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: ShareSupplyCapCall) -> Self {
            Self::ShareSupplyCap(value)
        }
    }
    impl ::core::convert::From<SwapPositionsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: SwapPositionsCall) -> Self {
            Self::SwapPositions(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<ToggleIgnorePauseCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ToggleIgnorePauseCall) -> Self {
            Self::ToggleIgnorePause(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalAssetsWithdrawableCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: TotalAssetsWithdrawableCall) -> Self {
            Self::TotalAssetsWithdrawable(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UserShareLockStartTimeCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: UserShareLockStartTimeCall) -> Self {
            Self::UserShareLockStartTime(value)
        }
    }
    impl ::core::convert::From<ViewPositionBalancesCall>
    for CellarWithShareLockPeriodV1Calls {
        fn from(value: ViewPositionBalancesCall) -> Self {
            Self::ViewPositionBalances(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for CellarWithShareLockPeriodV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
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
    ///Container type for all return fields from the `GRAVITY_BRIDGE_REGISTRY_SLOT` function with signature `GRAVITY_BRIDGE_REGISTRY_SLOT()` and selector `0xcd82f8b1`
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
    pub struct GravityBridgeRegistrySlotReturn(pub ::ethers::core::types::U256);
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
    pub struct MaxPositionsReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `adaptorCatalogue` function with signature `adaptorCatalogue(address)` and selector `0x18d4c143`
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
    pub struct AdaptorCatalogueReturn(pub bool);
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
    ///Container type for all return fields from the `automationActions` function with signature `automationActions()` and selector `0x88c4caba`
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
    pub struct AutomationActionsReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `blockExternalReceiver` function with signature `blockExternalReceiver()` and selector `0x4c4602da`
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
    pub struct BlockExternalReceiverReturn(pub bool);
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
    ///Container type for all return fields from the `creditPositions` function with signature `creditPositions(uint256)` and selector `0x59d20b4e`
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
    pub struct CreditPositionsReturn(pub u32);
    ///Container type for all return fields from the `debtPositions` function with signature `debtPositions(uint256)` and selector `0xe1b1acb7`
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
    pub struct DebtPositionsReturn(pub u32);
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
        pub strategist_platform_cut: u64,
        pub platform_fee: u64,
        pub last_accrual: u64,
        pub strategist_payout_address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getCreditPositions` function with signature `getCreditPositions()` and selector `0x71e99dc2`
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
    pub struct GetCreditPositionsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getDebtPositions` function with signature `getDebtPositions()` and selector `0x3e3382ba`
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
    pub struct GetDebtPositionsReturn(pub ::std::vec::Vec<u32>);
    ///Container type for all return fields from the `getPositionData` function with signature `getPositionData(uint32)` and selector `0x7384504f`
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
    pub struct GetPositionDataReturn {
        pub adaptor: ::ethers::core::types::Address,
        pub is_debt: bool,
        pub adaptor_data: ::ethers::core::types::Bytes,
        pub configuration_data: ::ethers::core::types::Bytes,
    }
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
    pub struct HoldingPositionReturn(pub u32);
    ///Container type for all return fields from the `ignorePause` function with signature `ignorePause()` and selector `0x9959af94`
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
    pub struct IgnorePauseReturn(pub bool);
    ///Container type for all return fields from the `isPaused` function with signature `isPaused()` and selector `0xb187bd26`
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
    pub struct IsPausedReturn(pub bool);
    ///Container type for all return fields from the `isPositionUsed` function with signature `isPositionUsed(uint256)` and selector `0x93bbeac0`
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
    ///Container type for all return fields from the `locked` function with signature `locked()` and selector `0xcf309012`
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
    pub struct LockedReturn(pub bool);
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
    pub struct MaxDepositReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
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
    ///Container type for all return fields from the `positionCatalogue` function with signature `positionCatalogue(uint32)` and selector `0xcbdf33d0`
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
    pub struct PositionCatalogueReturn(pub bool);
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
    ///Container type for all return fields from the `priceRouter` function with signature `priceRouter()` and selector `0xd7d4bf45`
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
    pub struct PriceRouterReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `shareSupplyCap` function with signature `shareSupplyCap()` and selector `0xd446bbcc`
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
    pub struct ShareSupplyCapReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `userShareLockStartTime` function with signature `userShareLockStartTime(address)` and selector `0x687c2b50`
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
    pub struct UserShareLockStartTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `viewPositionBalances` function with signature `viewPositionBalances()` and selector `0x78e0233e`
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
    pub struct ViewPositionBalancesReturn {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
        pub is_debt: ::std::vec::Vec<bool>,
    }
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
    ///`AdaptorCall(address,bytes[])`
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
    pub struct AdaptorCall {
        pub adaptor: ::ethers::core::types::Address,
        pub call_data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
}
