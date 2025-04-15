pub use aave_v2_stablecoin_cellar::*;
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
pub mod aave_v2_stablecoin_cellar {
    const _: () = {
        ::core::include_bytes!(
            "../abi/AaveV2StablecoinCellar.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_asset"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_approvedPositions"),
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
                        name: ::std::borrow::ToOwned::to_owned("_curveRegistryExchange"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ICurveSwaps"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sushiswapRouter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ISushiSwapRouter"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_lendingPool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ILendingPool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_incentivesController"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract IAaveIncentivesController",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gravityBridge"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IGravity"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_stkAAVE"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IStakedTokenV2"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_AAVE"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_WETH"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract ERC20"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AAVE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("AAVE"),
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
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("WETH"),
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
                    ::std::borrow::ToOwned::to_owned("accrualPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accrualPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("accrue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("accrue"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("assetAToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetAToken"),
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
                    ::std::borrow::ToOwned::to_owned("assetDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("assetDecimals"),
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
                    ::std::borrow::ToOwned::to_owned("claimAndUnstake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimAndUnstake"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("curveRegistryExchange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "curveRegistryExchange",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ICurveSwaps"),
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
                    ::std::borrow::ToOwned::to_owned("enterPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enterPosition"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("enterPosition"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exitPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exitPosition"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exitPosition"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("feesDistributor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("feesDistributor"),
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
                    ::std::borrow::ToOwned::to_owned("gravityBridge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gravityBridge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IGravity"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("incentivesController"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "incentivesController",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IAaveIncentivesController",
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
                    ::std::borrow::ToOwned::to_owned("initiateShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initiateShutdown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("emptyPosition"),
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
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
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
                    ::std::borrow::ToOwned::to_owned("lendingPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lendingPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ILendingPool"),
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
                    ::std::borrow::ToOwned::to_owned("maxLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxLocked"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("results"),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("performanceFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("performanceFee"),
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
                    ::std::borrow::ToOwned::to_owned("platformFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("platformFee"),
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
                    ::std::borrow::ToOwned::to_owned("rebalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rebalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("route"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                        9usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[9]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                3usize,
                                            ),
                                        ),
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[3][4]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minAssetsOut"),
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
                    ::std::borrow::ToOwned::to_owned("reinvest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("reinvest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minAssetsOut"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
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
                    ::std::borrow::ToOwned::to_owned("setAccrualPeriod"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAccrualPeriod"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newAccrualPeriod"),
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
                    ::std::borrow::ToOwned::to_owned("setTrust"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTrust"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("trust"),
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
                    ::std::borrow::ToOwned::to_owned("stkAAVE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stkAAVE"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IStakedTokenV2"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sushiswapRouter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sushiswapRouter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract ISushiSwapRouter",
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
                    ::std::borrow::ToOwned::to_owned("sweep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweep"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
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
                    ::std::borrow::ToOwned::to_owned("totalBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalBalance"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        240usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint240"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalHoldings"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalHoldings"),
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
                    ::std::borrow::ToOwned::to_owned("totalLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalLocked"),
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
                    ::std::borrow::ToOwned::to_owned("Accrual"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Accrual"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("platformFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("performanceFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("yield"),
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
                    ::std::borrow::ToOwned::to_owned("AccrualPeriodChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AccrualPeriodChanged",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPeriod"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                    ::std::borrow::ToOwned::to_owned("ClaimAndUnstake"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ClaimAndUnstake"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
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
                    ::std::borrow::ToOwned::to_owned("DepositIntoPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "DepositIntoPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("EnterPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("EnterPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExitPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ExitPosition"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
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
                    ::std::borrow::ToOwned::to_owned("Rebalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Rebalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldAsset"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAsset"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Reinvest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Reinvest"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rewards"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("assets"),
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
                    ::std::borrow::ToOwned::to_owned("ShutdownInitiated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ShutdownInitiated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("emptyPositions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ShutdownLifted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ShutdownLifted"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Sweep"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Sweep"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                                    name: ::std::borrow::ToOwned::to_owned("trusted"),
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
                    ::std::borrow::ToOwned::to_owned("WithdrawFromPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "WithdrawFromPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("position"),
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
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("STATE_AccrualOngoing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STATE_AccrualOngoing",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("STATE_ContractShutdown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "STATE_ContractShutdown",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("USR_DepositRestricted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USR_DepositRestricted",
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
                    ::std::borrow::ToOwned::to_owned("USR_ProtectedAsset"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("USR_ProtectedAsset"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
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
                    ::std::borrow::ToOwned::to_owned("USR_SamePosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("USR_SamePosition"),
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
                    ::std::borrow::ToOwned::to_owned("USR_TooManyDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USR_TooManyDecimals",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("maxDecimals"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("USR_UnsupportedPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USR_UnsupportedPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "unsupportedPosition",
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
                    ::std::borrow::ToOwned::to_owned("USR_UntrustedPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "USR_UntrustedPosition",
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static AAVEV2STABLECOINCELLAR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\xE0`@R`\n\x80Tc\xFF\xFF\xFF\xFF\x19\x16b\t:\x80\x17\x90Us\xB8\x13UKB2f\xBB\xD4\xC1l2\xFA83\x94\x86\x8C\x1FU`\x0BU4\x80\x15b\0\0<W`\0\x80\xFD[P`@Qb\0UB8\x03\x80b\0UB\x839\x81\x01`@\x81\x90Rb\0\0_\x91b\0\x07\x0CV[\x89`@Q\x80``\x01`@R\x80`,\x81R` \x01b\0U\x16`,\x919`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rjaave2-CLR-S`\xA8\x1B` \x82\x01R`\x12\x82\x82\x82`\0b\0\0\xAF\x84\x82b\0\t#V[P`\x01b\0\0\xBE\x83\x82b\0\t#V[P`\xFF\x81\x16`\x80RF`\xA0Rb\0\0\xD4b\0\x02\\V[`\xC0RPP`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16\x95\x90\x95\x17\x90\x94UPb\0\x01\x12\x92P3\x91Pb\0\x01\x0C\x90PV[b\0\x02\xF8V[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16`\xE0R\x87\x81\x16a\x01\0R\x86\x81\x16a\x01 R\x85\x81\x16a\x01@R\x84\x81\x16a\x01`R\x83\x81\x16a\x01\x80R\x82\x81\x16a\x01\xA0R\x81\x81\x16a\x01\xC0R\x8A\x16`\0\x90\x81R`\x0C` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ub\0\x01w\x8Bb\0\x03JV[\x90P`\0b\0\x01\x88\x82`\nb\0\x0B\x04V[\x90Pb\0\x01\x99\x81bLK@b\0\x0B\x15V[`\rUb\0\x01\xAA\x81a\xC3Pb\0\x0B\x15V[`\x0EU`\0[\x8BQ\x81\x10\x15b\0\x02\x1CW`\x01`\x0C`\0\x8E\x84\x81Q\x81\x10b\0\x01\xD5Wb\0\x01\xD5b\0\x0B7V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82R\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90U\x80b\0\x02\x13\x81b\0\x0BMV[\x91PPb\0\x01\xB0V[P`\n\x80T`\x01` \x1B`\x01``\x1B\x03\x19\x16Bc\xFF\xFF\xFF\xFF\x16d\x01\0\0\0\0\x02\x17\x90Ub\0\x02J\x86b\0\x05}V[PPPPPPPPPPPPb\0\roV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qb\0\x02\x90\x91\x90b\0\x0BiV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[a\x01 Q`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0\x92\x83\x92\x91\x16\x90c5\xEAju\x90`$\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x03\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03\xC1\x91\x90b\0\x0C\x11V[P\x92\x9APP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x98Pb\0\x04\t\x97PPPPPPPPW`@Qc\n\\^}`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x08`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15b\0\x04\\W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x04\x82\x91\x90b\0\r\x08V[\x92P`\x12\x83`\xFF\x16\x11\x15b\0\x04\xB7W`@Qc\x06Q\x98/`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`\x12`$\x82\x01R`D\x01b\0\x04\0V[`\xFF\x81\x16\x15\x80\x15\x90b\0\x04\xD0WP\x82`\xFF\x16\x81`\xFF\x16\x14\x15[\x15b\0\x053W`\x0ET`\rT`\0\x19\x82\x14b\0\x05\x06Wb\0\x05\x02\x83\x86\x84b\0\x06N` \x1Bb\x000W\x17\x90\x92\x91\x90` \x1CV[`\x0EU[`\0\x19\x81\x14b\0\x050Wb\0\x05,\x83\x86\x83b\0\x06N` \x1Bb\x000W\x17\x90\x92\x91\x90` \x1CV[`\rU[PP[P`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x17\x90\x91U`\x08\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\xA0\x1B`\xFF\x86\x16\x02\x90\x92\x16\x91\x90\x91\x17\x91\x90\x93\x16\x17\x90\x91U\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14b\0\x05\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01b\0\x04\0V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x06@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01b\0\x04\0V[b\0\x06K\x81b\0\x02\xF8V[PV[`\0\x81`\xFF\x16\x83`\xFF\x16\x03b\0\x06fWP\x82b\0\x06\xC7V[\x81`\xFF\x16\x83`\xFF\x16\x10\x15b\0\x06\xA2Wb\0\x06\x81\x83\x83b\0\r&V[b\0\x06\x8E\x90`\nb\0\x0B\x04V[b\0\x06\x9A\x90\x85b\0\x0B\x15V[\x90Pb\0\x06\xC7V[b\0\x06\xAE\x82\x84b\0\r&V[b\0\x06\xBB\x90`\nb\0\x0B\x04V[b\0\x06\x9A\x90\x85b\0\rLV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x06KW`\0\x80\xFD[\x80Qb\0\x06\xF1\x81b\0\x06\xCEV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01@\x8B\x8D\x03\x12\x15b\0\x07-W`\0\x80\xFD[\x8AQb\0\x07:\x81b\0\x06\xCEV[` \x8C\x01Q\x90\x9AP`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x07XW`\0\x80\xFD[\x81\x8D\x01\x91P\x8D`\x1F\x83\x01\x12b\0\x07mW`\0\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x07\x82Wb\0\x07\x82b\0\x06\xF6V[`@Q`\x1F\x19`?\x83`\x05\x1B\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15b\0\x07\xA9Wb\0\x07\xA9b\0\x06\xF6V[`@R\x81\x81R` \x80\x82\x01\x93P`\x05\x92\x90\x92\x1B\x84\x01\x90\x91\x01\x90\x8F\x82\x11\x15b\0\x07\xD0W`\0\x80\xFD[` \x84\x01\x93P[\x81\x84\x10\x15b\0\x07\xFDWb\0\x07\xEB\x84b\0\x06\xE4V[\x83R` \x93\x84\x01\x93\x90\x92\x01\x91b\0\x07\xD7V[\x9BPb\0\x08\x11\x92PPP`@\x8C\x01b\0\x06\xE4V[\x97Pb\0\x08!``\x8C\x01b\0\x06\xE4V[\x96Pb\0\x081`\x80\x8C\x01b\0\x06\xE4V[\x95Pb\0\x08A`\xA0\x8C\x01b\0\x06\xE4V[\x94Pb\0\x08Q`\xC0\x8C\x01b\0\x06\xE4V[\x93Pb\0\x08a`\xE0\x8C\x01b\0\x06\xE4V[\x92Pb\0\x08ra\x01\0\x8C\x01b\0\x06\xE4V[\x91Pb\0\x08\x83a\x01 \x8C\x01b\0\x06\xE4V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x08\xA9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x08\xCAWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\t\x1EW`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\x08\xF9WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\t\x1AW\x82\x81U`\x01\x01b\0\t\x05V[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\t?Wb\0\t?b\0\x06\xF6V[b\0\tW\x81b\0\tP\x84Tb\0\x08\x94V[\x84b\0\x08\xD0V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\t\x8FW`\0\x84\x15b\0\tvWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\t\x1AV[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\t\xC0W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\t\x9FV[P\x85\x82\x10\x15b\0\t\xDFW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01\x81\x81[\x80\x85\x11\x15b\0\nFW\x81`\0\x19\x04\x82\x11\x15b\0\n*Wb\0\n*b\0\t\xEFV[\x80\x85\x16\x15b\0\n8W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\0\n\nV[P\x92P\x92\x90PV[`\0\x82b\0\n_WP`\x01b\0\n\xFEV[\x81b\0\nnWP`\0b\0\n\xFEV[\x81`\x01\x81\x14b\0\n\x87W`\x02\x81\x14b\0\n\x92Wb\0\n\xB2V[`\x01\x91PPb\0\n\xFEV[`\xFF\x84\x11\x15b\0\n\xA6Wb\0\n\xA6b\0\t\xEFV[PP`\x01\x82\x1Bb\0\n\xFEV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0\n\xD7WP\x81\x81\nb\0\n\xFEV[b\0\n\xE3\x83\x83b\0\n\x05V[\x80`\0\x19\x04\x82\x11\x15b\0\n\xFAWb\0\n\xFAb\0\t\xEFV[\x02\x90P[\x92\x91PPV[`\0b\0\x06\xC7`\xFF\x84\x16\x83b\0\nNV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0\x0B2Wb\0\x0B2b\0\t\xEFV[P\x02\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\x01\x82\x01b\0\x0BbWb\0\x0Bbb\0\t\xEFV[P`\x01\x01\x90V[`\0\x80\x83Tb\0\x0By\x81b\0\x08\x94V[`\x01\x82\x81\x16\x80\x15b\0\x0B\x94W`\x01\x81\x14b\0\x0B\xAAWb\0\x0B\xDBV[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pb\0\x0B\xDBV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15b\0\x0B\xD2W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01b\0\x0B\xB7V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x80Q`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x14b\0\x06\xF1W`\0\x80\xFD[\x80Q`\xFF\x81\x16\x81\x14b\0\x06\xF1W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\x80\x8D\x8F\x03\x12\x15b\0\x0C5W`\0\x80\xFD[\x8CQ\x9BPb\0\x0CG` \x8E\x01b\0\x0B\xE7V[\x9APb\0\x0CW`@\x8E\x01b\0\x0B\xE7V[\x99Pb\0\x0Cg``\x8E\x01b\0\x0B\xE7V[\x98Pb\0\x0Cw`\x80\x8E\x01b\0\x0B\xE7V[\x97Pb\0\x0C\x87`\xA0\x8E\x01b\0\x0B\xE7V[\x96P`\xC0\x8D\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x0C\xA2W`\0\x80\xFD[\x95Pb\0\x0C\xB2`\xE0\x8E\x01b\0\x06\xE4V[\x94Pb\0\x0C\xC3a\x01\0\x8E\x01b\0\x06\xE4V[\x93Pb\0\x0C\xD4a\x01 \x8E\x01b\0\x06\xE4V[\x92Pb\0\x0C\xE5a\x01@\x8E\x01b\0\x06\xE4V[\x91Pb\0\x0C\xF6a\x01`\x8E\x01b\0\x0B\xFFV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\0` \x82\x84\x03\x12\x15b\0\r\x1BW`\0\x80\xFD[b\0\x06\xC7\x82b\0\x0B\xFFV[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15b\0\rCWb\0\rCb\0\t\xEFV[\x90\x03\x93\x92PPPV[`\0\x82b\0\rjWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x01\x80Qa\x01\xA0Qa\x01\xC0QaF\xA2b\0\x0Et`\09`\0\x81\x81a\n \x01Ra\x1B\x1E\x01R`\0\x81\x81a\x06s\x01R\x81\x81a\x1A\x1F\x01R\x81\x81a\x1A\xCA\x01Ra\x1B\xA0\x01R`\0\x81\x81a\x05$\x01R\x81\x81a\x19\xA4\x01R\x81\x81a \x93\x01Ra\"\xC1\x01R`\0\x81\x81a\t/\x01R\x81\x81a*c\x01Ra*\xB9\x01R`\0\x81\x81a\n\x8C\x01Ra \x0F\x01R`\0\x81\x81a\tc\x01R\x81\x81a1<\x01R\x81\x81a2\x97\x01R\x81\x81a4\x9D\x01Ra4\xF5\x01R`\0\x81\x81a\x0C\xFE\x01R\x81\x81a\x1B\xC2\x01Ra\x1B\xF3\x01R`\0\x81\x81a\t\xB7\x01R\x81\x81a\x11\xD9\x01Ra\x12\x18\x01R`\0a\x14\xA0\x01R`\0a\x14p\x01R`\0a\x05\xC3\x01RaF\xA2`\0\xF3\xFE`\x80`@R`\x046\x10a\x04\x1BW`\x005`\xE0\x1C\x80c\x99r\x92\x16\x11a\x02\x1EW\x80c\xC6=u\xB6\x11a\x01#W\x80c\xE9$\x0C-\x11a\0\xABW\x80c\xEFz\xC8\x83\x11a\0zW\x80c\xEFz\xC8\x83\x14a\rkW\x80c\xEF\x8B0\xF7\x14a\r\x8BW\x80c\xF2\xFD\xE3\x8B\x14a\r\xABW\x80c\xF6fAU\x14a\r\xCBW\x80c\xF8\xBAL\xFF\x14a\r\xFDW`\0\x80\xFD[\x80c\xE9$\x0C-\x14a\x0C\xECW\x80c\xE9\xEC.\x99\x14a\r W\x80c\xEC\xF7\x08X\x14a\r5W\x80c\xEFF]\x92\x14a\rKW`\0\x80\xFD[\x80c\xD5\x05\xAC\xCF\x11a\0\xF2W\x80c\xD5\x05\xAC\xCF\x14a\x0C)W\x80c\xD9\x05w~\x14a\x0CIW\x80c\xDDb\xED>\x14a\x0C\x7FW\x80c\xDF\x05\xA5*\x14a\x0C\xB7W\x80c\xDF\xF9\x0B[\x14a\x0C\xD7W`\0\x80\xFD[\x80c\xC6=u\xB6\x14a\x0B\xA9W\x80c\xC6\xE6\xF5\x92\x14a\x0B\xC9W\x80c\xCA\xB5\x928\x14a\x0B\xE9W\x80c\xCE\x96\xCBw\x14a\x0C\tW`\0\x80\xFD[\x80c\xAF\x1D\xF2U\x11a\x01\xA6W\x80c\xBA\x08vR\x11a\x01uW\x80c\xBA\x08vR\x14a\x0B\x0EW\x80c\xBD\xC8\x14K\x14a\x0B.W\x80c\xBF\x86\xD6\x90\x14a\x0BNW\x80c\xC1\x7Fg@\x14a\x0BhW\x80c\xC2\xD4\x16\x01\x14a\x0B\x88W`\0\x80\xFD[\x80c\xAF\x1D\xF2U\x14a\nzW\x80c\xB3\xD7\xF6\xB9\x14a\n\xAEW\x80c\xB4`\xAF\x94\x14a\n\xCEW\x80c\xB8\xDCI\x1B\x14a\n\xEEW`\0\x80\xFD[\x80c\xAC55\x10\x11a\x01\xEDW\x80c\xAC55\x10\x14a\t\xA5W\x80c\xAC\x96P\xD8\x14a\t\xD9W\x80c\xAD\0N \x14a\t\xF9W\x80c\xAD\\FH\x14a\n\x0EW\x80c\xADzg/\x14a\nBW`\0\x80\xFD[\x80c\x99r\x92\x16\x14a\t\x08W\x80c\xA4\xDA-\x02\x14a\t\x1DW\x80c\xA5\x9A\x99s\x14a\tQW\x80c\xA9\x05\x9C\xBB\x14a\t\x85W`\0\x80\xFD[\x80c^,Wn\x11a\x03$W\x80c~\xCE\xBE\0\x11a\x02\xACW\x80c\x8E\x0B\xAE\x7F\x11a\x02{W\x80c\x8E\x0B\xAE\x7F\x14a\x08fW\x80c\x8F\xDC\x9D\xFA\x14a\x08|W\x80c\x94\xBF\x80M\x14a\x08\xA3W\x80c\x95\xD8\x9BA\x14a\x08\xC3W\x80c\x96\xD6Hy\x14a\x08\xD8W`\0\x80\xFD[\x80c~\xCE\xBE\0\x14a\x07\xDFW\x80c\x83\xB4\x91\x8B\x14a\x08\x0CW\x80c\x87x\x87\x82\x14a\x08,W\x80c\x8D\xA5\xCB[\x14a\x08HW`\0\x80\xFD[\x80cp\xA0\x821\x11a\x02\xF3W\x80cp\xA0\x821\x14a\x07?W\x80cqP\x18\xA6\x14a\x07lW\x80cr\x167\x15\x14a\x07\x81W\x80cx\xDC\x90Y\x14a\x07\x97W\x80c{;\xAA\xB4\x14a\x07\xB7W`\0\x80\xFD[\x80c^,Wn\x14a\x06\xCAW\x80cn\x08@k\x14a\x06\xDFW\x80cnU?e\x14a\x06\xFFW\x80cn\x85\xF1\x83\x14a\x07\x1FW`\0\x80\xFD[\x80c&#*.\x11a\x03\xA7W\x80c=\xC6\xEA\xBF\x11a\x03vW\x80c=\xC6\xEA\xBF\x14a\x06,W\x80c@-&}\x14a\x06AW\x80cH\xCC\xDA<\x14a\x06aW\x80cL\xDA\xD5\x06\x14a\x06\x95W\x80cV\x89\x14\x12\x14a\x06\xB5W`\0\x80\xFD[\x80c&#*.\x14a\x05~W\x80c1<\xE5g\x14a\x05\xB1W\x80c6D\xE5\x15\x14a\x05\xF7W\x80c8\xD5.\x0F\x14a\x06\x0CW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x03\xEEW\x80c\n(\xA4w\x14a\x04\xBAW\x80c\x15\xF4\xC6\x11\x14a\x04\xDAW\x80c\x18\x16\r\xDD\x14a\x04\xFCW\x80c\x1F\xC2\x9C\x01\x14a\x05\x12W\x80c#\xB8r\xDD\x14a\x05^W`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x04 W\x80c\x06\xFD\xDE\x03\x14a\x04HW\x80c\x07\xA2\xD1:\x14a\x04jW\x80c\t^\xA7\xB3\x14a\x04\x8AW[`\0\x80\xFD[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x045a\x0E\x12V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04TW`\0\x80\xFD[Pa\x04]a\x0EIV[`@Qa\x04?\x91\x90a9\xE6V[4\x80\x15a\x04vW`\0\x80\xFD[Pa\x045a\x04\x856`\x04a9\xF9V[a\x0E\xD7V[4\x80\x15a\x04\x96W`\0\x80\xFD[Pa\x04\xAAa\x04\xA56`\x04a:'V[a\x0F\x1DV[`@Q\x90\x15\x15\x81R` \x01a\x04?V[4\x80\x15a\x04\xC6W`\0\x80\xFD[Pa\x045a\x04\xD56`\x04a9\xF9V[a\x0F\x8AV[4\x80\x15a\x04\xE6W`\0\x80\xFD[Pa\x04\xFAa\x04\xF56`\x04a;\x06V[a\x0F\xC4V[\0[4\x80\x15a\x05\x08W`\0\x80\xFD[Pa\x045`\x02T\x81V[4\x80\x15a\x05\x1EW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x04\xAAa\x05y6`\x04a<\x10V[a\x13\x8CV[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x05\x99f\x08\xE1\xBC\x9B\xF0@\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x05\xBDW`\0\x80\xFD[Pa\x05\xE5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x06\x03W`\0\x80\xFD[Pa\x045a\x14lV[4\x80\x15a\x06\x18W`\0\x80\xFD[P`\x06Ta\x05F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x068W`\0\x80\xFD[Pa\x04\xFAa\x14\xC2V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x045a\x06\\6`\x04a<QV[a\x14\xCFV[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xA1W`\0\x80\xFD[Pa\x045a\x06\xB06`\x04a9\xF9V[a\x15GV[4\x80\x15a\x06\xC1W`\0\x80\xFD[Pa\x045a\x15RV[4\x80\x15a\x06\xD6W`\0\x80\xFD[Pa\x04\xFAa\x15\xCDV[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x04\xFAa\x06\xFA6`\x04a9\xF9V[a\x16,V[4\x80\x15a\x07\x0BW`\0\x80\xFD[Pa\x045a\x07\x1A6`\x04a<nV[a\x17\x1CV[4\x80\x15a\x07+W`\0\x80\xFD[Pa\x04\xFAa\x07:6`\x04a9\xF9V[a\x17\xE0V[4\x80\x15a\x07KW`\0\x80\xFD[Pa\x045a\x07Z6`\x04a<QV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x07xW`\0\x80\xFD[Pa\x04\xFAa\x18KV[4\x80\x15a\x07\x8DW`\0\x80\xFD[Pa\x045`\rT\x81V[4\x80\x15a\x07\xA3W`\0\x80\xFD[Pa\x04\xFAa\x07\xB26`\x04a9\xF9V[a\x18\x7FV[4\x80\x15a\x07\xC3W`\0\x80\xFD[P`\nTa\x05\x99\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x045a\x07\xFA6`\x04a<QV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\x18W`\0\x80\xFD[Pa\x04\xFAa\x08'6`\x04a9\xF9V[a\x19]V[4\x80\x15a\x088W`\0\x80\xFD[Pa\x05\x99g\x01cEx]\x8A\0\0\x81V[4\x80\x15a\x08TW`\0\x80\xFD[P`\x07T`\x01`\x01`\xA0\x1B\x03\x16a\x05FV[4\x80\x15a\x08rW`\0\x80\xFD[Pa\x045`\x0BT\x81V[4\x80\x15a\x08\x88W`\0\x80\xFD[P`\nTa\x05F\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08\xAFW`\0\x80\xFD[Pa\x045a\x08\xBE6`\x04a<nV[a\x1D\x1FV[4\x80\x15a\x08\xCFW`\0\x80\xFD[Pa\x04]a\x1D\x9CV[4\x80\x15a\x08\xE4W`\0\x80\xFD[Pa\x04\xAAa\x08\xF36`\x04a<QV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\t\x14W`\0\x80\xFD[Pa\x04\xFAa\x1D\xA9V[4\x80\x15a\t)W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t]W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x91W`\0\x80\xFD[Pa\x04\xAAa\t\xA06`\x04a:'V[a\x1D\xBEV[4\x80\x15a\t\xB1W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\t\xECa\t\xE76`\x04a<\x9EV[a\x1E$V[`@Qa\x04?\x91\x90a=\x12V[4\x80\x15a\n\x05W`\0\x80\xFD[Pa\x045a\x1F{V[4\x80\x15a\n\x1AW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\nNW`\0\x80\xFD[P`\tTa\nb\x90`\x01`\x01`\xF0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\n\x86W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\n\xBAW`\0\x80\xFD[Pa\x045a\n\xC96`\x04a9\xF9V[a!AV[4\x80\x15a\n\xDAW`\0\x80\xFD[Pa\x045a\n\xE96`\x04a=tV[a!`V[4\x80\x15a\n\xFAW`\0\x80\xFD[Pa\x04\xFAa\x0B\t6`\x04a=\xB6V[a\"SV[4\x80\x15a\x0B\x1AW`\0\x80\xFD[Pa\x045a\x0B)6`\x04a=tV[a#\xF1V[4\x80\x15a\x0B:W`\0\x80\xFD[Pa\x04\xFAa\x0BI6`\x04a9\xF9V[a%\"V[4\x80\x15a\x0BZW`\0\x80\xFD[P`\x0FTa\x04\xAA\x90`\xFF\x16\x81V[4\x80\x15a\x0BtW`\0\x80\xFD[P`\x08Ta\x05F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B\x94W`\0\x80\xFD[P`\x08Ta\x05\xE5\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x0B\xB5W`\0\x80\xFD[Pa\x045a\x0B\xC46`\x04a<QV[a%\x8DV[4\x80\x15a\x0B\xD5W`\0\x80\xFD[Pa\x045a\x0B\xE46`\x04a9\xF9V[a%\xF7V[4\x80\x15a\x0B\xF5W`\0\x80\xFD[Pa\x04\xFAa\x0C\x046`\x04a=\xF9V[a&\x17V[4\x80\x15a\x0C\x15W`\0\x80\xFD[Pa\x045a\x0C$6`\x04a<QV[a&\xE4V[4\x80\x15a\x0C5W`\0\x80\xFD[Pa\x04\xFAa\x0CD6`\x04a>=V[a'\x06V[4\x80\x15a\x0CUW`\0\x80\xFD[Pa\x045a\x0Cd6`\x04a<QV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[4\x80\x15a\x0C\x8BW`\0\x80\xFD[Pa\x045a\x0C\x9A6`\x04a=\xB6V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x0C\xC3W`\0\x80\xFD[Pa\x04\xFAa\x0C\xD26`\x04a9\xF9V[a)JV[4\x80\x15a\x0C\xE3W`\0\x80\xFD[Pa\x04\xFAa)\xB5V[4\x80\x15a\x0C\xF8W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\r,W`\0\x80\xFD[Pa\x045a+VV[4\x80\x15a\rAW`\0\x80\xFD[Pa\x045`\x0ET\x81V[4\x80\x15a\rWW`\0\x80\xFD[Pa\x04\xFAa\rf6`\x04a>\xAEV[a+\xC3V[4\x80\x15a\rwW`\0\x80\xFD[Pa\x04\xFAa\r\x866`\x04a>\xC9V[a,xV[4\x80\x15a\r\x97W`\0\x80\xFD[Pa\x045a\r\xA66`\x04a9\xF9V[a-+V[4\x80\x15a\r\xB7W`\0\x80\xFD[Pa\x04\xFAa\r\xC66`\x04a<QV[a-6V[4\x80\x15a\r\xD7W`\0\x80\xFD[P`\nTa\r\xE8\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x0E\tW`\0\x80\xFD[Pa\x04\xFAa-\xD1V[`\0a\x0E\x1Ca\x15RV[a\x0E$a+VV[`\tTa\x0E:\x91\x90`\x01`\x01`\xF0\x1B\x03\x16a?\x05V[a\x0ED\x91\x90a?\x1DV[\x90P\x90V[`\0\x80Ta\x0EV\x90a?4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x82\x90a?4V[\x80\x15a\x0E\xCFW\x80`\x1F\x10a\x0E\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\0\x90\x80\x15a\x0E\xFBWa\x0E\xF6a\x0E\xEEa\x0E\x12V[\x84\x90\x83a0\xC0V[a\x0F\x16V[`\x08Ta\x0F\x16\x90\x84\x90`\x12\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a0WV[\x93\x92PPPV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0Fx\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x02T`\0\x90\x80\x15a\x0F\xAAWa\x0E\xF6\x81a\x0F\xA2a\x0E\x12V[\x85\x91\x90a0\xDFV[`\x08Ta\x0F\x16\x90\x84\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x12a0WV[`\x0FT`\xFF\x16\x15a\x0F\xE8W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x80`\x08\x14\x80a\x10WWP`\0\x85a\x107\x83`\x01a?\x05V[`\t\x81\x10a\x10GWa\x10Ga?\xA3V[` \x02\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x10zW\x84\x81`\t\x81\x10a\x10nWa\x10na?\xA3V[` \x02\x01Q\x91Pa\x10\x8CV[a\x10\x85`\x02\x82a?\x05V[\x90Pa\x10\x1FV[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x10\xD1W`@Qc\x86C?+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a\x11\rW`@Qc\x06\x13\xAE\xCF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\0a\x11\x17a+VV[`\tT\x90\x91P`\0\x90a\x114\x90\x83\x90`\x01`\x01`\xF0\x1B\x03\x16a?\x05V[`\x08T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA6\x91\x90a?\xB9V[\x11a\x11\xB1W\x82a\x11\xC8V[\x82a\x11\xBE\x85`\0\x19a1\rV[a\x11\xC8\x91\x90a?\x05V[\x90Pa\x11\xFE`\x01`\x01`\xA0\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\xF6V[`@Qc\rO)\t`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5<\xA4$\x90a\x12S\x90\x8C\x90\x8C\x90\x87\x90\x8D\x90`\x04\x01a?\xD2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x96\x91\x90a?\xB9V[`\x08T\x90\x91P`\x01`\xA0\x1B\x90\x04`\xFF\x16`\0a\x12\xB1\x88a2sV[\x90Pa\x12\xBD\x88\x84a4\x8EV[`\nTa\x12\xDB\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a0WV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01``\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\0a\x13\x1Da\x13\x17\x87\x85\x85a0WV[\x85a5\x8CV[`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01`\x01`\xF0\x1B\x03\x83\x16\x17\x90U`@Q\x81\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x8A\x16\x90\x7F\xB0\x85\x0B\x8E\x0F\x9E\x83\x15\xDD\xE3\xC9\xF9\xF3\x118(>k\xBE\x16\xCD)\xE8U.\xB1\xDC\xDF\x9F\xAC\x9E;\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x13\xE8Wa\x13\xC3\x83\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x14\x10\x90\x84\x90a?\x1DV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90a\x14Y\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x14\x9DWa\x0EDa5\xA2V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x14\xCDa\x06\xFAa+VV[V[`\x0FT`\0\x90`\xFF\x16\x15a\x14\xE5WP`\0\x91\x90PV[`\x0ET`\rT`\0\x19\x82\x14\x80\x15a\x14\xFDWP`\0\x19\x81\x14[\x15a\x15\rWP`\0\x19\x93\x92PPPV[`\0a\x15\"a\x15\x1B\x86a&\xE4V[\x84\x90a6<V[\x90P`\0a\x151a\x15\x1Ba\x0E\x12V[\x90Pa\x15=\x82\x82a5\x8CV[\x96\x95PPPPPPV[`\0a\x0F\x84\x82a\x0E\xD7V[`\nT`\0\x90`\x01`\x01`@\x1B\x03d\x01\0\0\0\0\x82\x04\x16\x90c\xFF\xFF\xFF\xFF\x16a\x15z\x81\x83a?\x05V[B\x10a\x15\x89W`\0\x92PPP\x90V[`\nT`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81a\x15\xA7\x84Ba?\x1DV[a\x15\xB1\x90\x83a@nV[a\x15\xBB\x91\x90a@\x8DV[a\x15\xC5\x90\x82a?\x1DV[\x93PPPP\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0F\x80T`\xFF\x19\x16\x90U`@Q\x7F\t\xBE\xC6\x19\x9BW\x12\xAB\xE9\xCB\xB7\x19\x97\xB0oaI\xA4S\xEC\xA5\xAB\xEC\x15\xD5(\xE1Ne\xE1`^\x90`\0\x90\xA1V[`\x0FT`\xFF\x16\x15a\x16PW`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\t\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x91\x90`\0\x90a\x16\xA7\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xAFV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UPa\x16\xD5\x81\x83a4\x8EV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB6\xF4\xB9%^\xE9\x89\xB1\x84J\x8Ek}\xA8\x90k\x81 \x0C8\xF7\xB3\xF4\xF1\xAC1\xE9\xA2A\xC7WP\x83`@Qa\x17\x10\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x17'\x83a-+V[\x90P\x80`\0\x03a\x17gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_SHARES`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a\x17r\x83\x82\x84a6VV[`\x06Ta\x17\x8A\x90`\x01`\x01`\xA0\x1B\x03\x1630\x86a6\xB2V[a\x17\x94\x82\x82a7<V[`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01[`@Q\x80\x91\x03\x90\xA3a\x0F\x84V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0BT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FQ:\xC1\x9C\xBB\xAA\xADNE\x0Cs.\xD3v5\x17\x8B}\x83\xBF\x8E\x84\xA9@\xFF\xE7\xE0R\xC9\xC7\xCA\xA2\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0BUV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[a\x14\xCD`\0a7\x96V[`\x0FT`\xFF\x16\x15a\x18\xA3W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x18\xE3\x81\x83a1\rV[`\t\x80T`\0\x90a\x18\xFE\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xDAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDEL\xC1\xD2\xDDA\x97\n\x82z\x8D\xF5^\xFD\x18\xC5'\xC1|&HXG\xD6\x80\xCC+Lq\xE7\xA8|\x83`@Qa\x17\x10\x91\x81R` \x01\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@Qc\x01\xE9\xA6\x95`\xE4\x1B\x81R0`\x04\x82\x01R`\0\x19`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1E\x9AiP\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x04W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x93\x91\x90a?\xB9V[`\x06T`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91`\0\x91` \x82\x01``\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x1A\xFCWa\x1A\xFCa?\xA3V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x1BPWa\x1BPa?\xA3V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\x02\x81Q\x81\x10a\x1B\x84Wa\x1B\x84a?\xA3V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01Ra\x1B\xE7\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a1\xF6V[`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c8\xED\x179\x85\x87\x850a\x1C'B`<a?\x05V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1CG\x95\x94\x93\x92\x91\x90aAFV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1CfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C\x8E\x91\x90\x81\x01\x90aA\x82V[\x90P`\0\x81`\x01\x83Qa\x1C\xA1\x91\x90a?\x1DV[\x81Q\x81\x10a\x1C\xB1Wa\x1C\xB1a?\xA3V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0FT\x90\x91P`\xFF\x16a\x1C\xD3Wa\x1C\xD3\x84\x82a4\x8EV[`@\x80Q\x86\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x7F\xC0\x03\xF4[\xC2$\xD1\x16\xB6\xD0y\x10\rJ\xB5z[\x963$LG\xA5\xA9*\x17l[y\xA8_(\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x1D*\x83a!AV[\x90Pa\x1D7\x81\x84\x84a6VV[`\x06Ta\x1DO\x90`\x01`\x01`\xA0\x1B\x03\x1630\x84a6\xB2V[a\x1DY\x82\x84a7<V[`@\x80Q\x82\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01a\x17\xD3V[`\x01\x80Ta\x0EV\x90a?4V[`\tTa\x14\xCD\x90`\x01`\x01`\xF0\x1B\x03\x16a\x18\x7FV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x1D\xDF\x90\x84\x90a?\x1DV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90a\x0Fx\x90\x86\x81R` \x01\x90V[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E>Wa\x1E>a:SV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EqW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\\W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1FtW`\0\x800\x86\x86\x85\x81\x81\x10a\x1E\x95Wa\x1E\x95a?\xA3V[\x90P` \x02\x81\x01\x90a\x1E\xA7\x91\x90aB'V[`@Qa\x1E\xB5\x92\x91\x90aBtV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1E\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E\xF5V[``\x91P[P\x91P\x91P\x81a\x1FAW`D\x81Q\x10\x15a\x1F\x0EW`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x1F(\x91\x90aB\x84V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x91\x90a9\xE6V[\x80\x84\x84\x81Q\x81\x10a\x1FTWa\x1FTa?\xA3V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1Fl\x90aC\x17V[\x91PPa\x1EwV[P\x92\x91PPV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`\x08T\x82Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91P`\0\x90a\x1F\xEAWa\x1F\xEAa?\xA3V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@Qc1\x11\xE7\xB3`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c1\x11\xE7\xB3\x90a L\x90\x84\x90`\0\x19\x900\x90`\x04\x01aC0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x8F\x91\x90a?\xB9V[\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cxz\x08\xA6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\0W=`\0\x80>=`\0\xFD[PPPP\x7F\x8C\xA0\x18\x8D\x97p\xB3\x83\xD1\xA7\xA2\xDD\xFE^\x0C\x1F\x02\x90\x84H\x1ASi}lQR\\G\xA8\xD8\x8E\x82`@Qa!5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x90V[`\x02T`\0\x90\x80\x15a\x0E\xFBWa\x0E\xF6a!Xa\x0E\x12V[\x84\x90\x83a0\xDFV[`\0a!k\x84a\x0F\x8AV[\x90P3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a!\xDBW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a!\xD9Wa!\xB4\x82\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a!\xE7\x84\x82\x85\x85a7\xE8V[a!\xF1\x82\x82a8cV[`@\x80Q\x85\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\x06Ta\x0E\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x86a8\xC5V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x80a\"\xA6WP`\x08T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14[\x80a\"\xB9WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14[\x80a\"\xF5WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a#\x1EW`@Qc9\xB8T\x91`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x10\x12V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x89\x91\x90a?\xB9V[\x90Pa#\x9F`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a8\xC5V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xEDg\x93(\xAE\xBFt\xED\xE7z\xE0\x9E\xFC\xF3n\x90$O\x83d=\xAD\xAC\x1C-\x9F\x0B!\xA4oj\xB7\x83`@Qa#\xE4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a$aW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a$_Wa$:\x85\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a$j\x84a\x15GV[\x90P\x80`\0\x03a$\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a$\xB6\x81\x85\x85\x85a7\xE8V[a$\xC0\x82\x85a8cV[`@\x80Q\x82\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\x06Ta\x0E\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x83a8\xC5V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0ET`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCF\xB5\xA4T\xB8\xAA}\xC0N\xCB[\xC1A\x0B*W\x96\x9C\xA1\xD6\x7Ff\xD5e\x19o`\xC6\xF9\x97T\x04\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0EUV[`\x0FT`\0\x90`\xFF\x16\x15a%\xA3WP`\0\x91\x90PV[`\x0ET`\rT`\0\x19\x82\x14\x80\x15a%\xBBWP`\0\x19\x81\x14[\x15a%\xCBWP`\0\x19\x93\x92PPPV[`\0a%\xD9a\x15\x1B\x86a&\xE4V[\x90P`\0a%\xE8a\x15\x1Ba\x0E\x12V[\x90Pa\x15=a\x0B\xE4\x83\x83a5\x8CV[`\x02T`\0\x90\x80\x15a\x0F\xAAWa\x0E\xF6\x81a&\x0Fa\x0E\x12V[\x85\x91\x90a0\xC0V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a&AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16\x83\x15\x80\x15\x91\x82\x17\x90\x92U`\x06T\x90\x92\x16\x91a&\x8CWP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a&\x9AWa&\x9A\x81a9=V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD6\0\xB94\x86\x03\xC6\xDE\xFF4\xB4\xE0\xB2\x8B`\xE1\xC8\x03l\x80gA\xB9\xE6\xD9\x002\xE7\xF3}\xD2\x7F\x83`@Qa&\xD7\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x81 Ta\x0F\x84\x90a\x0E\xD7V[B\x84\x10\x15a'VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x10\x12V[`\0`\x01a'ba\x14lV[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a(nW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a(\xA4WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a(\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x10\x12V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\rT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x1F!C-\xD7\xB8\xEA\xD6M.|\x06\xA7K\xAF\x13x;-/qS\xF0\x99\xE2\xC4\xCA\xBC<]\xBE\xC6\x91\x01`@Q\x80\x91\x03\x90\xA1`\rUV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[0`\0\x90\x81R`\x03` R`@\x81 T\x90a)\xF9\x82a\x15GV[\x90P\x80`\0\x03a*9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a*G\x81`\0\x80`\0a7\xE8V[a*Q0\x83a8cV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a*\x88\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a1\xF6V[`\x0BT`@Qc\x1F\xFB\xE7\xF9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R`D\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1F\xFB\xE7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+\x13W=`\0\x80>=`\0\xFD[PP`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\x15\xE3\xE2\xA7jh9\xC2D\xC1\xED\n\x82\x1C#<\xE8\xAFU-\xFF\xCB\x85`\x89\xEA\xE6\xCB\xBB\xB7\x1E\xA6\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1PPPV[`\x06T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ED\x91\x90a?\xB9V[`\x0FT`\xFF\x16\x15a+\xE7W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[\x80\x15a,,W`\x06Ta,,\x90`\x01`\x01`\xA0\x1B\x03\x16a9=V[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7Fn|\xABj\xCC\xF9\xB0\x93\xA6\xB8\0\xED\x92\r\xF6\x10\xDBM\xBF\xD8\x80t\x17\xF5\xF2\xC4\x8D\xD6l\x03\xBA\xBB\x90a,m\x90\x83\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\0a,\xACa\x15RV[\x11\x15a,\xCBW`@Qck\x86c\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@\x80Qc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F<9+D\xAD\x99\xB1\xFB|\x87\xAE{\x91L\xBD\x1D\xE1\xAE\xED>\x93i\xA2\r0p\xCCw\x16i\x89\x8F\x91\x01`@Q\x80\x91\x03\x90\xA1`\n\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0F\x84\x82a%\xF7V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a-`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x01`\x01`\xA0\x1B\x03\x81\x16a-\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x10\x12V[a-\xCE\x81a7\x96V[PV[`\0a-\xDBa\x15RV[\x90Pa-\xEF`\x07T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a.\x10WP`\0\x81\x11[\x15a..W`@Qck\x86c\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90a.I\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\naDGV[\x90P`\0a.V\x82a%\xF7V[`\tT`\x08T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x93P`\x01`\x01`\xF0\x1B\x03\x90\x91\x16\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xD5\x91\x90a?\xB9V[`\nT\x90\x91P`\0\x90a.\xF9\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`@\x1B\x03\x16Ba?\x1DV[\x90P`\0c\x01\xE13\x80g\r\xE0\xB6\xB3\xA7d\0\0f\x08\xE1\xBC\x9B\xF0@\0a/\x1D\x85\x87a@nV[a/'\x91\x90a@nV[a/1\x91\x90a@\x8DV[a/;\x91\x90a@\x8DV[\x90P`\0a/J\x82\x87\x89a0\xC0V[\x90P`\0a/X\x85\x87a6<V[\x90P`\0a/n\x82g\x01cEx]\x8A\0\0a9yV[\x90P`\0a/}\x82\x8A\x8Ca0\xC0V[\x90Pa/\x920a/\x8D\x83\x87a?\x05V[a7<V[a/\x9Fa\x15\x1B\x83\x87a?\x05V[a/\xA9\x90\x8Ca?\x05V[`\n\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x92\x90\x92\x17B\x90\x92\x16d\x01\0\0\0\0\x02\x91\x90\x91\x17\x90U`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01`\x01`\xF0\x1B\x03\x89\x16\x17\x90U`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R\x7F\xFD#\xCE\xFBI\x92\xBC\x1B\x95\xDF\x1FTN\xFD\xB9\x90\x8D\x90\x12\x885D!'\x0Fz\x8F\x8A\r\xFE\xC2\n\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[`\0\x81`\xFF\x16\x83`\xFF\x16\x03a0mWP\x82a\x0F\x16V[\x81`\xFF\x16\x83`\xFF\x16\x10\x15a0\xA1Wa0\x85\x83\x83aDVV[a0\x90\x90`\naDGV[a0\x9A\x90\x85a@nV[\x90Pa\x0F\x16V[a0\xAB\x82\x84aDVV[a0\xB6\x90`\naDGV[a0\x9A\x90\x85a@\x8DV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a0\xD8W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a0\xF7W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@Qc\x1AL\xA3{`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ci2\x8D\xEC\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a1\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xAB\x91\x90a?\xB9V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x844<\xC9v!\xDB\xC5\x1B\xCE\x19\x8A%\x82\x18\xA2\x06<\x16\x0EMG?\xF5\x10\x07\xC7\xA6\x0E\xEC_\xA1\x82`@Qa1\xE8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a2mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x10\x12V[PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x03\x91\x90aD\xAFV[P\x92\x9APP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x98Pa3E\x97PPPPPPPPW`@Qc\n\\^}`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\0`\x08`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xBB\x91\x90aE\x90V[\x92P`\x12\x83`\xFF\x16\x11\x15a3\xEEW`@Qc\x06Q\x98/`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`\x12`$\x82\x01R`D\x01a\x10\x12V[`\xFF\x81\x16\x15\x80\x15\x90a4\x06WP\x82`\xFF\x16\x81`\xFF\x16\x14\x15[\x15a4DW`\x0ET`\rT`\0\x19\x82\x14a4)Wa4%\x82\x84\x87a0WV[`\x0EU[`\0\x19\x81\x14a4AWa4=\x81\x84\x87a0WV[`\rU[PP[P`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x17\x90\x91U`\x08\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\xA0\x1B`\xFF\x86\x16\x02\x90\x92\x16\x91\x90\x91\x17\x91\x90\x93\x16\x17\x90\x91U\x90V[a4\xC2`\x01`\x01`\xA0\x1B\x03\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\xF6V[`@Qc\xE8\xED\xA9\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE8\xED\xA9\xDF\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a59W`\0\x80\xFD[PZ\xF1\x15\x80\x15a5MW=`\0\x80>=`\0\xFD[PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\x99\xEF\xD5m\x0Cd\xF9\xA1\xAA\x13y\xA4p\xD8q9+g\xEAvx\xEDVY\xADK\xFE}\xD7eu\x82`@Qa\x17\x10\x91\x81R` \x01\x90V[`\0\x81\x83\x10a5\x9BW\x81a\x0F\x16V[P\x90\x91\x90PV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa5\xD4\x91\x90aE\xADV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x11a6LW`\0a\x0F\x16V[a\x0F\x16\x82\x84a?\x1DV[`\x0FT`\xFF\x16\x15a6zW`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a6\x85\x82a\x14\xCFV[\x90P\x80\x84\x11\x15a2mW`@Qc#\xDC)\x05`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x10\x12V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a75W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x10\x12V[PPPPPV[\x80`\x02`\0\x82\x82Ta7N\x91\x90a?\x05V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` aFM\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16`\0a7\xFEa+VV[\x90P\x80\x86\x11\x15a8[Wa8\x1B\x82a8\x16\x83\x89a?\x1DV[a1\rV[`\t\x80T`\0\x90a86\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xDAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UP[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a8\x8B\x90\x84\x90a?\x1DV[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90` \x01a7\x8AV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a2mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x10\x12V[`\tT`\x01`\x01`\xF0\x1B\x03\x16\x80\x15a9uWa9Wa-\xD1V[a9c\x82`\0\x19a1\rV[P`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16\x90U[PPV[`\0a\x0F\x16\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\xC0V[`\0[\x83\x81\x10\x15a9\xA9W\x81\x81\x01Q\x83\x82\x01R` \x01a9\x91V[\x83\x81\x11\x15a2mWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra9\xD2\x81` \x86\x01` \x86\x01a9\x8EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0F\x16` \x83\x01\x84a9\xBAV[`\0` \x82\x84\x03\x12\x15a:\x0BW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\xCEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a::W`\0\x80\xFD[\x825a:E\x81a:\x12V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01 \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\xFEWa:\xFEa:SV[`@R\x91\x90PV[`\0\x80`\0a\x02\xC0\x84\x86\x03\x12\x15a;\x1CW`\0\x80\xFD[`\x1F\x85\x81\x86\x01\x12a;,W`\0\x80\xFD[a;4a:iV[\x80a\x01 \x87\x01\x88\x81\x11\x15a;GW`\0\x80\xFD[\x87[\x81\x81\x10\x15a;jW\x805a;\\\x81a:\x12V[\x84R` \x93\x84\x01\x93\x01a;IV[P\x81\x96P\x88a\x01?\x89\x01\x12a;~W`\0\x80\xFD[a;\x86a:\x92V[\x92P\x82\x91Pa\x02\xA0\x88\x01\x89\x81\x11\x15a;\x9DW`\0\x80\xFD[\x80\x82\x10\x15a<\0W\x89\x85\x83\x01\x12a;\xB4W`\0\x80\x81\xFD[a;\xBCa:\xB4V[\x80``\x84\x01\x8C\x81\x11\x15a;\xCFW`\0\x80\x81\xFD[\x84[\x81\x81\x10\x15a;\xE9W\x805\x84R` \x93\x84\x01\x93\x01a;\xD1V[PP\x85RP` \x90\x93\x01\x92``\x91\x90\x91\x01\x90a;\x9DV[\x96\x99\x91\x98PP\x945\x95PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a<%W`\0\x80\xFD[\x835a<0\x81a:\x12V[\x92P` \x84\x015a<@\x81a:\x12V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a<cW`\0\x80\xFD[\x815a\x0F\x16\x81a:\x12V[`\0\x80`@\x83\x85\x03\x12\x15a<\x81W`\0\x80\xFD[\x825\x91P` \x83\x015a<\x93\x81a:\x12V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a<\xB1W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xC8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a<\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a<\xEBW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a=\0W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a=gW`?\x19\x88\x86\x03\x01\x84Ra=U\x85\x83Qa9\xBAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a=9V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a=\x89W`\0\x80\xFD[\x835\x92P` \x84\x015a=\x9B\x81a:\x12V[\x91P`@\x84\x015a=\xAB\x81a:\x12V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a=\xC9W`\0\x80\xFD[\x825a=\xD4\x81a:\x12V[\x91P` \x83\x015a<\x93\x81a:\x12V[\x805\x80\x15\x15\x81\x14a=\xF4W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a>\x0CW`\0\x80\xFD[\x825a>\x17\x81a:\x12V[\x91Pa>%` \x84\x01a=\xE4V[\x90P\x92P\x92\x90PV[`\xFF\x81\x16\x81\x14a-\xCEW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a>XW`\0\x80\xFD[\x875a>c\x81a:\x12V[\x96P` \x88\x015a>s\x81a:\x12V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a>\x91\x81a>.V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a>\xC0W`\0\x80\xFD[a\x0F\x16\x82a=\xE4V[`\0` \x82\x84\x03\x12\x15a>\xDBW`\0\x80\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x16W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a?\x18Wa?\x18a>\xEFV[P\x01\x90V[`\0\x82\x82\x10\x15a?/Wa?/a>\xEFV[P\x03\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a?HW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?hWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a?\xCBW`\0\x80\xFD[PQ\x91\x90PV[a\x02\xE0\x81\x01\x81\x86`\0[`\t\x81\x10\x15a@\x04W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a?\xDCV[PPPa\x01 \x82\x01\x85`\0[`\x04\x81\x10\x15a@WW\x81Q\x83`\0[`\x03\x81\x10\x15a@>W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a@\x1FV[PPP``\x92\x90\x92\x01\x91` \x91\x90\x91\x01\x90`\x01\x01a@\x10V[PPPa\x02\xA0\x82\x01\x93\x90\x93Ra\x02\xC0\x01R\x92\x91PPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a@\x88Wa@\x88a>\xEFV[P\x02\x90V[`\0\x82a@\xAAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01`\xF0\x1B\x03\x82\x81\x16\x84\x82\x16\x80\x83\x03\x82\x11\x15a@\xD1Wa@\xD1a>\xEFV[\x01\x94\x93PPPPV[`\0`\x01`\x01`\xF0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a@\xFAWa@\xFAa>\xEFV[\x03\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aA;W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aA\x16V[P\x94\x95\x94PPPPPV[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0aAe`\xA0\x83\x01\x86aA\x02V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aA\x95W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xACW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aA\xC0W`\0\x80\xFD[\x81Q\x81\x81\x11\x15aA\xD2WaA\xD2a:SV[\x80`\x05\x1B\x91PaA\xE3\x84\x83\x01a:\xD6V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15aA\xFDW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aB\x1BW\x84Q\x82R\x93\x85\x01\x93\x90\x85\x01\x90aB\x02V[\x98\x97PPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB>W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aBXW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aBmW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15aB\x96W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xADW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aB\xC1W`\0\x80\xFD[\x81Q\x81\x81\x11\x15aB\xD3WaB\xD3a:SV[aB\xE6`\x1F\x82\x01`\x1F\x19\x16` \x01a:\xD6V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15aB\xFDW`\0\x80\xFD[aC\x0E\x81` \x84\x01` \x86\x01a9\x8EV[P\x94\x93PPPPV[`\0`\x01\x82\x01aC)WaC)a>\xEFV[P`\x01\x01\x90V[``\x81R`\0aCC``\x83\x01\x86aA\x02V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\x01\x81\x81[\x80\x85\x11\x15aC\x9EW\x81`\0\x19\x04\x82\x11\x15aC\x84WaC\x84a>\xEFV[\x80\x85\x16\x15aC\x91W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aChV[P\x92P\x92\x90PV[`\0\x82aC\xB5WP`\x01a\x0F\x84V[\x81aC\xC2WP`\0a\x0F\x84V[\x81`\x01\x81\x14aC\xD8W`\x02\x81\x14aC\xE2WaC\xFEV[`\x01\x91PPa\x0F\x84V[`\xFF\x84\x11\x15aC\xF3WaC\xF3a>\xEFV[PP`\x01\x82\x1Ba\x0F\x84V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aD!WP\x81\x81\na\x0F\x84V[aD+\x83\x83aCcV[\x80`\0\x19\x04\x82\x11\x15aD?WaD?a>\xEFV[\x02\x93\x92PPPV[`\0a\x0F\x16`\xFF\x84\x16\x83aC\xA6V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aDpWaDpa>\xEFV[\x90\x03\x93\x92PPPV[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a=\xF4W`\0\x80\xFD[\x80Qa=\xF4\x81a:\x12V[\x80Qa=\xF4\x81a>.V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\x80\x8D\x8F\x03\x12\x15aD\xD2W`\0\x80\xFD[\x8CQ\x9BPaD\xE2` \x8E\x01aDyV[\x9APaD\xF0`@\x8E\x01aDyV[\x99PaD\xFE``\x8E\x01aDyV[\x98PaE\x0C`\x80\x8E\x01aDyV[\x97PaE\x1A`\xA0\x8E\x01aDyV[\x96P`\xC0\x8D\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aE4W`\0\x80\xFD[\x95PaEB`\xE0\x8E\x01aD\x99V[\x94PaEQa\x01\0\x8E\x01aD\x99V[\x93PaE`a\x01 \x8E\x01aD\x99V[\x92PaEoa\x01@\x8E\x01aD\x99V[\x91PaE~a\x01`\x8E\x01aD\xA4V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\0` \x82\x84\x03\x12\x15aE\xA2W`\0\x80\xFD[\x81Qa\x0F\x16\x81a>.V[`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80aE\xC9W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03aE\xE8WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15aE\xFCW`\x01\x81\x14aF\x11WaF>V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96PaF>V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15aF6W\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01aF\x1DV[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xEAd\x98\xAB\x16\xF3\xF2\xED\xC70Wz\x96S\x18kZdae\\\x1AUN\xD9\xF17\te4\xCB\xD1dsolcC\0\x08\x0F\x003Sommelier Aave V2 Stablecoin Cellar LP Token";
    /// The bytecode of the contract.
    pub static AAVEV2STABLECOINCELLAR_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x04\x1BW`\x005`\xE0\x1C\x80c\x99r\x92\x16\x11a\x02\x1EW\x80c\xC6=u\xB6\x11a\x01#W\x80c\xE9$\x0C-\x11a\0\xABW\x80c\xEFz\xC8\x83\x11a\0zW\x80c\xEFz\xC8\x83\x14a\rkW\x80c\xEF\x8B0\xF7\x14a\r\x8BW\x80c\xF2\xFD\xE3\x8B\x14a\r\xABW\x80c\xF6fAU\x14a\r\xCBW\x80c\xF8\xBAL\xFF\x14a\r\xFDW`\0\x80\xFD[\x80c\xE9$\x0C-\x14a\x0C\xECW\x80c\xE9\xEC.\x99\x14a\r W\x80c\xEC\xF7\x08X\x14a\r5W\x80c\xEFF]\x92\x14a\rKW`\0\x80\xFD[\x80c\xD5\x05\xAC\xCF\x11a\0\xF2W\x80c\xD5\x05\xAC\xCF\x14a\x0C)W\x80c\xD9\x05w~\x14a\x0CIW\x80c\xDDb\xED>\x14a\x0C\x7FW\x80c\xDF\x05\xA5*\x14a\x0C\xB7W\x80c\xDF\xF9\x0B[\x14a\x0C\xD7W`\0\x80\xFD[\x80c\xC6=u\xB6\x14a\x0B\xA9W\x80c\xC6\xE6\xF5\x92\x14a\x0B\xC9W\x80c\xCA\xB5\x928\x14a\x0B\xE9W\x80c\xCE\x96\xCBw\x14a\x0C\tW`\0\x80\xFD[\x80c\xAF\x1D\xF2U\x11a\x01\xA6W\x80c\xBA\x08vR\x11a\x01uW\x80c\xBA\x08vR\x14a\x0B\x0EW\x80c\xBD\xC8\x14K\x14a\x0B.W\x80c\xBF\x86\xD6\x90\x14a\x0BNW\x80c\xC1\x7Fg@\x14a\x0BhW\x80c\xC2\xD4\x16\x01\x14a\x0B\x88W`\0\x80\xFD[\x80c\xAF\x1D\xF2U\x14a\nzW\x80c\xB3\xD7\xF6\xB9\x14a\n\xAEW\x80c\xB4`\xAF\x94\x14a\n\xCEW\x80c\xB8\xDCI\x1B\x14a\n\xEEW`\0\x80\xFD[\x80c\xAC55\x10\x11a\x01\xEDW\x80c\xAC55\x10\x14a\t\xA5W\x80c\xAC\x96P\xD8\x14a\t\xD9W\x80c\xAD\0N \x14a\t\xF9W\x80c\xAD\\FH\x14a\n\x0EW\x80c\xADzg/\x14a\nBW`\0\x80\xFD[\x80c\x99r\x92\x16\x14a\t\x08W\x80c\xA4\xDA-\x02\x14a\t\x1DW\x80c\xA5\x9A\x99s\x14a\tQW\x80c\xA9\x05\x9C\xBB\x14a\t\x85W`\0\x80\xFD[\x80c^,Wn\x11a\x03$W\x80c~\xCE\xBE\0\x11a\x02\xACW\x80c\x8E\x0B\xAE\x7F\x11a\x02{W\x80c\x8E\x0B\xAE\x7F\x14a\x08fW\x80c\x8F\xDC\x9D\xFA\x14a\x08|W\x80c\x94\xBF\x80M\x14a\x08\xA3W\x80c\x95\xD8\x9BA\x14a\x08\xC3W\x80c\x96\xD6Hy\x14a\x08\xD8W`\0\x80\xFD[\x80c~\xCE\xBE\0\x14a\x07\xDFW\x80c\x83\xB4\x91\x8B\x14a\x08\x0CW\x80c\x87x\x87\x82\x14a\x08,W\x80c\x8D\xA5\xCB[\x14a\x08HW`\0\x80\xFD[\x80cp\xA0\x821\x11a\x02\xF3W\x80cp\xA0\x821\x14a\x07?W\x80cqP\x18\xA6\x14a\x07lW\x80cr\x167\x15\x14a\x07\x81W\x80cx\xDC\x90Y\x14a\x07\x97W\x80c{;\xAA\xB4\x14a\x07\xB7W`\0\x80\xFD[\x80c^,Wn\x14a\x06\xCAW\x80cn\x08@k\x14a\x06\xDFW\x80cnU?e\x14a\x06\xFFW\x80cn\x85\xF1\x83\x14a\x07\x1FW`\0\x80\xFD[\x80c&#*.\x11a\x03\xA7W\x80c=\xC6\xEA\xBF\x11a\x03vW\x80c=\xC6\xEA\xBF\x14a\x06,W\x80c@-&}\x14a\x06AW\x80cH\xCC\xDA<\x14a\x06aW\x80cL\xDA\xD5\x06\x14a\x06\x95W\x80cV\x89\x14\x12\x14a\x06\xB5W`\0\x80\xFD[\x80c&#*.\x14a\x05~W\x80c1<\xE5g\x14a\x05\xB1W\x80c6D\xE5\x15\x14a\x05\xF7W\x80c8\xD5.\x0F\x14a\x06\x0CW`\0\x80\xFD[\x80c\n(\xA4w\x11a\x03\xEEW\x80c\n(\xA4w\x14a\x04\xBAW\x80c\x15\xF4\xC6\x11\x14a\x04\xDAW\x80c\x18\x16\r\xDD\x14a\x04\xFCW\x80c\x1F\xC2\x9C\x01\x14a\x05\x12W\x80c#\xB8r\xDD\x14a\x05^W`\0\x80\xFD[\x80c\x01\xE1\xD1\x14\x14a\x04 W\x80c\x06\xFD\xDE\x03\x14a\x04HW\x80c\x07\xA2\xD1:\x14a\x04jW\x80c\t^\xA7\xB3\x14a\x04\x8AW[`\0\x80\xFD[4\x80\x15a\x04,W`\0\x80\xFD[Pa\x045a\x0E\x12V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04TW`\0\x80\xFD[Pa\x04]a\x0EIV[`@Qa\x04?\x91\x90a9\xE6V[4\x80\x15a\x04vW`\0\x80\xFD[Pa\x045a\x04\x856`\x04a9\xF9V[a\x0E\xD7V[4\x80\x15a\x04\x96W`\0\x80\xFD[Pa\x04\xAAa\x04\xA56`\x04a:'V[a\x0F\x1DV[`@Q\x90\x15\x15\x81R` \x01a\x04?V[4\x80\x15a\x04\xC6W`\0\x80\xFD[Pa\x045a\x04\xD56`\x04a9\xF9V[a\x0F\x8AV[4\x80\x15a\x04\xE6W`\0\x80\xFD[Pa\x04\xFAa\x04\xF56`\x04a;\x06V[a\x0F\xC4V[\0[4\x80\x15a\x05\x08W`\0\x80\xFD[Pa\x045`\x02T\x81V[4\x80\x15a\x05\x1EW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x05jW`\0\x80\xFD[Pa\x04\xAAa\x05y6`\x04a<\x10V[a\x13\x8CV[4\x80\x15a\x05\x8AW`\0\x80\xFD[Pa\x05\x99f\x08\xE1\xBC\x9B\xF0@\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x05\xBDW`\0\x80\xFD[Pa\x05\xE5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x06\x03W`\0\x80\xFD[Pa\x045a\x14lV[4\x80\x15a\x06\x18W`\0\x80\xFD[P`\x06Ta\x05F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x068W`\0\x80\xFD[Pa\x04\xFAa\x14\xC2V[4\x80\x15a\x06MW`\0\x80\xFD[Pa\x045a\x06\\6`\x04a<QV[a\x14\xCFV[4\x80\x15a\x06mW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xA1W`\0\x80\xFD[Pa\x045a\x06\xB06`\x04a9\xF9V[a\x15GV[4\x80\x15a\x06\xC1W`\0\x80\xFD[Pa\x045a\x15RV[4\x80\x15a\x06\xD6W`\0\x80\xFD[Pa\x04\xFAa\x15\xCDV[4\x80\x15a\x06\xEBW`\0\x80\xFD[Pa\x04\xFAa\x06\xFA6`\x04a9\xF9V[a\x16,V[4\x80\x15a\x07\x0BW`\0\x80\xFD[Pa\x045a\x07\x1A6`\x04a<nV[a\x17\x1CV[4\x80\x15a\x07+W`\0\x80\xFD[Pa\x04\xFAa\x07:6`\x04a9\xF9V[a\x17\xE0V[4\x80\x15a\x07KW`\0\x80\xFD[Pa\x045a\x07Z6`\x04a<QV[`\x03` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x07xW`\0\x80\xFD[Pa\x04\xFAa\x18KV[4\x80\x15a\x07\x8DW`\0\x80\xFD[Pa\x045`\rT\x81V[4\x80\x15a\x07\xA3W`\0\x80\xFD[Pa\x04\xFAa\x07\xB26`\x04a9\xF9V[a\x18\x7FV[4\x80\x15a\x07\xC3W`\0\x80\xFD[P`\nTa\x05\x99\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x07\xEBW`\0\x80\xFD[Pa\x045a\x07\xFA6`\x04a<QV[`\x05` R`\0\x90\x81R`@\x90 T\x81V[4\x80\x15a\x08\x18W`\0\x80\xFD[Pa\x04\xFAa\x08'6`\x04a9\xF9V[a\x19]V[4\x80\x15a\x088W`\0\x80\xFD[Pa\x05\x99g\x01cEx]\x8A\0\0\x81V[4\x80\x15a\x08TW`\0\x80\xFD[P`\x07T`\x01`\x01`\xA0\x1B\x03\x16a\x05FV[4\x80\x15a\x08rW`\0\x80\xFD[Pa\x045`\x0BT\x81V[4\x80\x15a\x08\x88W`\0\x80\xFD[P`\nTa\x05F\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x08\xAFW`\0\x80\xFD[Pa\x045a\x08\xBE6`\x04a<nV[a\x1D\x1FV[4\x80\x15a\x08\xCFW`\0\x80\xFD[Pa\x04]a\x1D\x9CV[4\x80\x15a\x08\xE4W`\0\x80\xFD[Pa\x04\xAAa\x08\xF36`\x04a<QV[`\x0C` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[4\x80\x15a\t\x14W`\0\x80\xFD[Pa\x04\xFAa\x1D\xA9V[4\x80\x15a\t)W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t]W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x91W`\0\x80\xFD[Pa\x04\xAAa\t\xA06`\x04a:'V[a\x1D\xBEV[4\x80\x15a\t\xB1W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\t\xECa\t\xE76`\x04a<\x9EV[a\x1E$V[`@Qa\x04?\x91\x90a=\x12V[4\x80\x15a\n\x05W`\0\x80\xFD[Pa\x045a\x1F{V[4\x80\x15a\n\x1AW`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\nNW`\0\x80\xFD[P`\tTa\nb\x90`\x01`\x01`\xF0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xF0\x1B\x03\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\n\x86W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\n\xBAW`\0\x80\xFD[Pa\x045a\n\xC96`\x04a9\xF9V[a!AV[4\x80\x15a\n\xDAW`\0\x80\xFD[Pa\x045a\n\xE96`\x04a=tV[a!`V[4\x80\x15a\n\xFAW`\0\x80\xFD[Pa\x04\xFAa\x0B\t6`\x04a=\xB6V[a\"SV[4\x80\x15a\x0B\x1AW`\0\x80\xFD[Pa\x045a\x0B)6`\x04a=tV[a#\xF1V[4\x80\x15a\x0B:W`\0\x80\xFD[Pa\x04\xFAa\x0BI6`\x04a9\xF9V[a%\"V[4\x80\x15a\x0BZW`\0\x80\xFD[P`\x0FTa\x04\xAA\x90`\xFF\x16\x81V[4\x80\x15a\x0BtW`\0\x80\xFD[P`\x08Ta\x05F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x0B\x94W`\0\x80\xFD[P`\x08Ta\x05\xE5\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x0B\xB5W`\0\x80\xFD[Pa\x045a\x0B\xC46`\x04a<QV[a%\x8DV[4\x80\x15a\x0B\xD5W`\0\x80\xFD[Pa\x045a\x0B\xE46`\x04a9\xF9V[a%\xF7V[4\x80\x15a\x0B\xF5W`\0\x80\xFD[Pa\x04\xFAa\x0C\x046`\x04a=\xF9V[a&\x17V[4\x80\x15a\x0C\x15W`\0\x80\xFD[Pa\x045a\x0C$6`\x04a<QV[a&\xE4V[4\x80\x15a\x0C5W`\0\x80\xFD[Pa\x04\xFAa\x0CD6`\x04a>=V[a'\x06V[4\x80\x15a\x0CUW`\0\x80\xFD[Pa\x045a\x0Cd6`\x04a<QV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[4\x80\x15a\x0C\x8BW`\0\x80\xFD[Pa\x045a\x0C\x9A6`\x04a=\xB6V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x0C\xC3W`\0\x80\xFD[Pa\x04\xFAa\x0C\xD26`\x04a9\xF9V[a)JV[4\x80\x15a\x0C\xE3W`\0\x80\xFD[Pa\x04\xFAa)\xB5V[4\x80\x15a\x0C\xF8W`\0\x80\xFD[Pa\x05F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\r,W`\0\x80\xFD[Pa\x045a+VV[4\x80\x15a\rAW`\0\x80\xFD[Pa\x045`\x0ET\x81V[4\x80\x15a\rWW`\0\x80\xFD[Pa\x04\xFAa\rf6`\x04a>\xAEV[a+\xC3V[4\x80\x15a\rwW`\0\x80\xFD[Pa\x04\xFAa\r\x866`\x04a>\xC9V[a,xV[4\x80\x15a\r\x97W`\0\x80\xFD[Pa\x045a\r\xA66`\x04a9\xF9V[a-+V[4\x80\x15a\r\xB7W`\0\x80\xFD[Pa\x04\xFAa\r\xC66`\x04a<QV[a-6V[4\x80\x15a\r\xD7W`\0\x80\xFD[P`\nTa\r\xE8\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x04?V[4\x80\x15a\x0E\tW`\0\x80\xFD[Pa\x04\xFAa-\xD1V[`\0a\x0E\x1Ca\x15RV[a\x0E$a+VV[`\tTa\x0E:\x91\x90`\x01`\x01`\xF0\x1B\x03\x16a?\x05V[a\x0ED\x91\x90a?\x1DV[\x90P\x90V[`\0\x80Ta\x0EV\x90a?4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0E\x82\x90a?4V[\x80\x15a\x0E\xCFW\x80`\x1F\x10a\x0E\xA4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0E\xCFV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0E\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\x02T`\0\x90\x80\x15a\x0E\xFBWa\x0E\xF6a\x0E\xEEa\x0E\x12V[\x84\x90\x83a0\xC0V[a\x0F\x16V[`\x08Ta\x0F\x16\x90\x84\x90`\x12\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16a0WV[\x93\x92PPPV[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x0Fx\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x02T`\0\x90\x80\x15a\x0F\xAAWa\x0E\xF6\x81a\x0F\xA2a\x0E\x12V[\x85\x91\x90a0\xDFV[`\x08Ta\x0F\x16\x90\x84\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\x12a0WV[`\x0FT`\xFF\x16\x15a\x0F\xE8W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x10\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x80`\x08\x14\x80a\x10WWP`\0\x85a\x107\x83`\x01a?\x05V[`\t\x81\x10a\x10GWa\x10Ga?\xA3V[` \x02\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x10zW\x84\x81`\t\x81\x10a\x10nWa\x10na?\xA3V[` \x02\x01Q\x91Pa\x10\x8CV[a\x10\x85`\x02\x82a?\x05V[\x90Pa\x10\x1FV[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x0C` R`@\x90 T`\xFF\x16a\x10\xD1W`@Qc\x86C?+`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x81\x90\x03a\x11\rW`@Qc\x06\x13\xAE\xCF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\0a\x11\x17a+VV[`\tT\x90\x91P`\0\x90a\x114\x90\x83\x90`\x01`\x01`\xF0\x1B\x03\x16a?\x05V[`\x08T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x92P`\0\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xA6\x91\x90a?\xB9V[\x11a\x11\xB1W\x82a\x11\xC8V[\x82a\x11\xBE\x85`\0\x19a1\rV[a\x11\xC8\x91\x90a?\x05V[\x90Pa\x11\xFE`\x01`\x01`\xA0\x1B\x03\x85\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\xF6V[`@Qc\rO)\t`\xE2\x1B\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5<\xA4$\x90a\x12S\x90\x8C\x90\x8C\x90\x87\x90\x8D\x90`\x04\x01a?\xD2V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x96\x91\x90a?\xB9V[`\x08T\x90\x91P`\x01`\xA0\x1B\x90\x04`\xFF\x16`\0a\x12\xB1\x88a2sV[\x90Pa\x12\xBD\x88\x84a4\x8EV[`\nTa\x12\xDB\x90`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x83\x83a0WV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x01``\x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U`\0a\x13\x1Da\x13\x17\x87\x85\x85a0WV[\x85a5\x8CV[`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01`\x01`\xF0\x1B\x03\x83\x16\x17\x90U`@Q\x81\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x8A\x16\x90\x7F\xB0\x85\x0B\x8E\x0F\x9E\x83\x15\xDD\xE3\xC9\xF9\xF3\x118(>k\xBE\x16\xCD)\xE8U.\xB1\xDC\xDF\x9F\xAC\x9E;\x90` \x01`@Q\x80\x91\x03\x90\xA3PPPPPPPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x13\xE8Wa\x13\xC3\x83\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x14\x10\x90\x84\x90a?\x1DV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90a\x14Y\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14a\x14\x9DWa\x0EDa5\xA2V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x14\xCDa\x06\xFAa+VV[V[`\x0FT`\0\x90`\xFF\x16\x15a\x14\xE5WP`\0\x91\x90PV[`\x0ET`\rT`\0\x19\x82\x14\x80\x15a\x14\xFDWP`\0\x19\x81\x14[\x15a\x15\rWP`\0\x19\x93\x92PPPV[`\0a\x15\"a\x15\x1B\x86a&\xE4V[\x84\x90a6<V[\x90P`\0a\x151a\x15\x1Ba\x0E\x12V[\x90Pa\x15=\x82\x82a5\x8CV[\x96\x95PPPPPPV[`\0a\x0F\x84\x82a\x0E\xD7V[`\nT`\0\x90`\x01`\x01`@\x1B\x03d\x01\0\0\0\0\x82\x04\x16\x90c\xFF\xFF\xFF\xFF\x16a\x15z\x81\x83a?\x05V[B\x10a\x15\x89W`\0\x92PPP\x90V[`\nT`\x01``\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81a\x15\xA7\x84Ba?\x1DV[a\x15\xB1\x90\x83a@nV[a\x15\xBB\x91\x90a@\x8DV[a\x15\xC5\x90\x82a?\x1DV[\x93PPPP\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0F\x80T`\xFF\x19\x16\x90U`@Q\x7F\t\xBE\xC6\x19\x9BW\x12\xAB\xE9\xCB\xB7\x19\x97\xB0oaI\xA4S\xEC\xA5\xAB\xEC\x15\xD5(\xE1Ne\xE1`^\x90`\0\x90\xA1V[`\x0FT`\xFF\x16\x15a\x16PW`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16zW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\t\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x83\x91\x90`\0\x90a\x16\xA7\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xAFV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UPa\x16\xD5\x81\x83a4\x8EV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\xB6\xF4\xB9%^\xE9\x89\xB1\x84J\x8Ek}\xA8\x90k\x81 \x0C8\xF7\xB3\xF4\xF1\xAC1\xE9\xA2A\xC7WP\x83`@Qa\x17\x10\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPV[`\0a\x17'\x83a-+V[\x90P\x80`\0\x03a\x17gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_SHARES`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a\x17r\x83\x82\x84a6VV[`\x06Ta\x17\x8A\x90`\x01`\x01`\xA0\x1B\x03\x1630\x86a6\xB2V[a\x17\x94\x82\x82a7<V[`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01[`@Q\x80\x91\x03\x90\xA3a\x0F\x84V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0BT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FQ:\xC1\x9C\xBB\xAA\xADNE\x0Cs.\xD3v5\x17\x8B}\x83\xBF\x8E\x84\xA9@\xFF\xE7\xE0R\xC9\xC7\xCA\xA2\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0BUV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18uW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[a\x14\xCD`\0a7\x96V[`\x0FT`\xFF\x16\x15a\x18\xA3W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a\x18\xE3\x81\x83a1\rV[`\t\x80T`\0\x90a\x18\xFE\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xDAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UP\x80`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDEL\xC1\xD2\xDDA\x97\n\x82z\x8D\xF5^\xFD\x18\xC5'\xC1|&HXG\xD6\x80\xCC+Lq\xE7\xA8|\x83`@Qa\x17\x10\x91\x81R` \x01\x90V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@Qc\x01\xE9\xA6\x95`\xE4\x1B\x81R0`\x04\x82\x01R`\0\x19`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x1E\x9AiP\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xF0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\x04W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1AoW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x93\x91\x90a?\xB9V[`\x06T`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R\x92\x93P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91`\0\x91` \x82\x01``\x806\x837\x01\x90PP\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\0\x81Q\x81\x10a\x1A\xFCWa\x1A\xFCa?\xA3V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01\x81Q\x81\x10a\x1BPWa\x1BPa?\xA3V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x81\x81`\x02\x81Q\x81\x10a\x1B\x84Wa\x1B\x84a?\xA3V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01Ra\x1B\xE7\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a1\xF6V[`\0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c8\xED\x179\x85\x87\x850a\x1C'B`<a?\x05V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1CG\x95\x94\x93\x92\x91\x90aAFV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1CfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C\x8E\x91\x90\x81\x01\x90aA\x82V[\x90P`\0\x81`\x01\x83Qa\x1C\xA1\x91\x90a?\x1DV[\x81Q\x81\x10a\x1C\xB1Wa\x1C\xB1a?\xA3V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x0FT\x90\x91P`\xFF\x16a\x1C\xD3Wa\x1C\xD3\x84\x82a4\x8EV[`@\x80Q\x86\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x91\x7F\xC0\x03\xF4[\xC2$\xD1\x16\xB6\xD0y\x10\rJ\xB5z[\x963$LG\xA5\xA9*\x17l[y\xA8_(\x91\x01`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0a\x1D*\x83a!AV[\x90Pa\x1D7\x81\x84\x84a6VV[`\x06Ta\x1DO\x90`\x01`\x01`\xA0\x1B\x03\x1630\x84a6\xB2V[a\x1DY\x82\x84a7<V[`@\x80Q\x82\x81R` \x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16\x913\x91\x7F\xDC\xBC\x1C\x05$\x0F1\xFF:\xD0g\xEF\x1E\xE3\\\xE4\x99wbu.:\tR\x84uED\xF4\xC7\t\xD7\x91\x01a\x17\xD3V[`\x01\x80Ta\x0EV\x90a?4V[`\tTa\x14\xCD\x90`\x01`\x01`\xF0\x1B\x03\x16a\x18\x7FV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x1D\xDF\x90\x84\x90a?\x1DV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90a\x0Fx\x90\x86\x81R` \x01\x90V[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E>Wa\x1E>a:SV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1EqW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1E\\W\x90P[P\x90P`\0[\x82\x81\x10\x15a\x1FtW`\0\x800\x86\x86\x85\x81\x81\x10a\x1E\x95Wa\x1E\x95a?\xA3V[\x90P` \x02\x81\x01\x90a\x1E\xA7\x91\x90aB'V[`@Qa\x1E\xB5\x92\x91\x90aBtV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1E\xF0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E\xF5V[``\x91P[P\x91P\x91P\x81a\x1FAW`D\x81Q\x10\x15a\x1F\x0EW`\0\x80\xFD[`\x04\x81\x01\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x1F(\x91\x90aB\x84V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x91\x90a9\xE6V[\x80\x84\x84\x81Q\x81\x10a\x1FTWa\x1FTa?\xA3V[` \x02` \x01\x01\x81\x90RPPP\x80\x80a\x1Fl\x90aC\x17V[\x91PPa\x1EwV[P\x92\x91PPV[`\x07T`\0\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1F\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91` \x80\x83\x01\x90\x806\x837PP`\x08T\x82Q\x92\x93P`\x01`\x01`\xA0\x1B\x03\x16\x91\x83\x91P`\0\x90a\x1F\xEAWa\x1F\xEAa?\xA3V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x01R`@Qc1\x11\xE7\xB3`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c1\x11\xE7\xB3\x90a L\x90\x84\x90`\0\x19\x900\x90`\x04\x01aC0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x8F\x91\x90a?\xB9V[\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cxz\x08\xA6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a \xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a!\0W=`\0\x80>=`\0\xFD[PPPP\x7F\x8C\xA0\x18\x8D\x97p\xB3\x83\xD1\xA7\xA2\xDD\xFE^\x0C\x1F\x02\x90\x84H\x1ASi}lQR\\G\xA8\xD8\x8E\x82`@Qa!5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1P\x90V[`\x02T`\0\x90\x80\x15a\x0E\xFBWa\x0E\xF6a!Xa\x0E\x12V[\x84\x90\x83a0\xDFV[`\0a!k\x84a\x0F\x8AV[\x90P3`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a!\xDBW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a!\xD9Wa!\xB4\x82\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a!\xE7\x84\x82\x85\x85a7\xE8V[a!\xF1\x82\x82a8cV[`@\x80Q\x85\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\x06Ta\x0E\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x86a8\xC5V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"}W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x06T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x80a\"\xA6WP`\x08T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14[\x80a\"\xB9WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14[\x80a\"\xF5WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a#\x1EW`@Qc9\xB8T\x91`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x10\x12V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#eW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x89\x91\x90a?\xB9V[\x90Pa#\x9F`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a8\xC5V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xEDg\x93(\xAE\xBFt\xED\xE7z\xE0\x9E\xFC\xF3n\x90$O\x83d=\xAD\xAC\x1C-\x9F\x0B!\xA4oj\xB7\x83`@Qa#\xE4\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x83\x16\x14a$aW`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\0\x19\x81\x14a$_Wa$:\x85\x82a?\x1DV[`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[P[a$j\x84a\x15GV[\x90P\x80`\0\x03a$\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a$\xB6\x81\x85\x85\x85a7\xE8V[a$\xC0\x82\x85a8cV[`@\x80Q\x82\x81R` \x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16\x92\x90\x86\x16\x913\x91\x7F\xFB\xDEy} \x1Ch\x1B\x91\x05e)\x11\x9E\x0B\x02@|{\xB9jJ,u\xC0\x1F\xC9fr2\xC8\xDB\x91\x01`@Q\x80\x91\x03\x90\xA4`\x06Ta\x0E\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x84\x83a8\xC5V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a%LW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x0ET`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\xCF\xB5\xA4T\xB8\xAA}\xC0N\xCB[\xC1A\x0B*W\x96\x9C\xA1\xD6\x7Ff\xD5e\x19o`\xC6\xF9\x97T\x04\x91\x01`@Q\x80\x91\x03\x90\xA1`\x0EUV[`\x0FT`\0\x90`\xFF\x16\x15a%\xA3WP`\0\x91\x90PV[`\x0ET`\rT`\0\x19\x82\x14\x80\x15a%\xBBWP`\0\x19\x81\x14[\x15a%\xCBWP`\0\x19\x93\x92PPPV[`\0a%\xD9a\x15\x1B\x86a&\xE4V[\x90P`\0a%\xE8a\x15\x1Ba\x0E\x12V[\x90Pa\x15=a\x0B\xE4\x83\x83a5\x8CV[`\x02T`\0\x90\x80\x15a\x0F\xAAWa\x0E\xF6\x81a&\x0Fa\x0E\x12V[\x85\x91\x90a0\xC0V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a&AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\x0C` R`@\x90 \x80T`\xFF\x19\x16\x83\x15\x80\x15\x91\x82\x17\x90\x92U`\x06T\x90\x92\x16\x91a&\x8CWP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a&\x9AWa&\x9A\x81a9=V[\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD6\0\xB94\x86\x03\xC6\xDE\xFF4\xB4\xE0\xB2\x8B`\xE1\xC8\x03l\x80gA\xB9\xE6\xD9\x002\xE7\xF3}\xD2\x7F\x83`@Qa&\xD7\x91\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` R`@\x81 Ta\x0F\x84\x90a\x0E\xD7V[B\x84\x10\x15a'VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x10\x12V[`\0`\x01a'ba\x14lV[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x05` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a(nW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a(\xA4WP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a(\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x10\x12V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\rT`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7F\x1F!C-\xD7\xB8\xEA\xD6M.|\x06\xA7K\xAF\x13x;-/qS\xF0\x99\xE2\xC4\xCA\xBC<]\xBE\xC6\x91\x01`@Q\x80\x91\x03\x90\xA1`\rUV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a)\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[0`\0\x90\x81R`\x03` R`@\x81 T\x90a)\xF9\x82a\x15GV[\x90P\x80`\0\x03a*9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0B`$\x82\x01RjZERO_ASSETS`\xA8\x1B`D\x82\x01R`d\x01a\x10\x12V[a*G\x81`\0\x80`\0a7\xE8V[a*Q0\x83a8cV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16a*\x88\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a1\xF6V[`\x0BT`@Qc\x1F\xFB\xE7\xF9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R`D\x81\x01\x84\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x1F\xFB\xE7\xF9\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a*\xFFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a+\x13W=`\0\x80>=`\0\xFD[PP`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\x15\xE3\xE2\xA7jh9\xC2D\xC1\xED\n\x82\x1C#<\xE8\xAFU-\xFF\xCB\x85`\x89\xEA\xE6\xCB\xBB\xB7\x1E\xA6\x93P\x01\x90P`@Q\x80\x91\x03\x90\xA1PPPV[`\x06T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0ED\x91\x90a?\xB9V[`\x0FT`\xFF\x16\x15a+\xE7W`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[\x80\x15a,,W`\x06Ta,,\x90`\x01`\x01`\xA0\x1B\x03\x16a9=V[`\x0F\x80T`\xFF\x19\x16`\x01\x17\x90U`@Q\x7Fn|\xABj\xCC\xF9\xB0\x93\xA6\xB8\0\xED\x92\r\xF6\x10\xDBM\xBF\xD8\x80t\x17\xF5\xF2\xC4\x8D\xD6l\x03\xBA\xBB\x90a,m\x90\x83\x15\x15\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PV[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a,\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\0a,\xACa\x15RV[\x11\x15a,\xCBW`@Qck\x86c\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\nT`@\x80Qc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F<9+D\xAD\x99\xB1\xFB|\x87\xAE{\x91L\xBD\x1D\xE1\xAE\xED>\x93i\xA2\r0p\xCCw\x16i\x89\x8F\x91\x01`@Q\x80\x91\x03\x90\xA1`\n\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a\x0F\x84\x82a%\xF7V[`\x07T`\x01`\x01`\xA0\x1B\x03\x163\x14a-`W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x10\x12\x90a?nV[`\x01`\x01`\xA0\x1B\x03\x81\x16a-\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x10\x12V[a-\xCE\x81a7\x96V[PV[`\0a-\xDBa\x15RV[\x90Pa-\xEF`\x07T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a.\x10WP`\0\x81\x11[\x15a..W`@Qck\x86c\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08T`\0\x90a.I\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16`\naDGV[\x90P`\0a.V\x82a%\xF7V[`\tT`\x08T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x92\x93P`\x01`\x01`\xF0\x1B\x03\x90\x91\x16\x91`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xD5\x91\x90a?\xB9V[`\nT\x90\x91P`\0\x90a.\xF9\x90d\x01\0\0\0\0\x90\x04`\x01`\x01`@\x1B\x03\x16Ba?\x1DV[\x90P`\0c\x01\xE13\x80g\r\xE0\xB6\xB3\xA7d\0\0f\x08\xE1\xBC\x9B\xF0@\0a/\x1D\x85\x87a@nV[a/'\x91\x90a@nV[a/1\x91\x90a@\x8DV[a/;\x91\x90a@\x8DV[\x90P`\0a/J\x82\x87\x89a0\xC0V[\x90P`\0a/X\x85\x87a6<V[\x90P`\0a/n\x82g\x01cEx]\x8A\0\0a9yV[\x90P`\0a/}\x82\x8A\x8Ca0\xC0V[\x90Pa/\x920a/\x8D\x83\x87a?\x05V[a7<V[a/\x9Fa\x15\x1B\x83\x87a?\x05V[a/\xA9\x90\x8Ca?\x05V[`\n\x80Tc\xFF\xFF\xFF\xFF\x90\x81\x16`\x01``\x1B`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x92\x90\x92\x17B\x90\x92\x16d\x01\0\0\0\0\x02\x91\x90\x91\x17\x90U`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16`\x01`\x01`\xF0\x1B\x03\x89\x16\x17\x90U`@\x80Q\x85\x81R` \x81\x01\x83\x90R\x90\x81\x01\x84\x90R\x7F\xFD#\xCE\xFBI\x92\xBC\x1B\x95\xDF\x1FTN\xFD\xB9\x90\x8D\x90\x12\x885D!'\x0Fz\x8F\x8A\r\xFE\xC2\n\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPV[`\0\x81`\xFF\x16\x83`\xFF\x16\x03a0mWP\x82a\x0F\x16V[\x81`\xFF\x16\x83`\xFF\x16\x10\x15a0\xA1Wa0\x85\x83\x83aDVV[a0\x90\x90`\naDGV[a0\x9A\x90\x85a@nV[\x90Pa\x0F\x16V[a0\xAB\x82\x84aDVV[a0\xB6\x90`\naDGV[a0\x9A\x90\x85a@\x8DV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a0\xD8W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a0\xF7W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`@Qc\x1AL\xA3{`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ci2\x8D\xEC\x90`d\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a1\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xAB\x91\x90a?\xB9V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x844<\xC9v!\xDB\xC5\x1B\xCE\x19\x8A%\x82\x18\xA2\x06<\x16\x0EMG?\xF5\x10\x07\xC7\xA6\x0E\xEC_\xA1\x82`@Qa1\xE8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x92\x91PPV[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a2mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x10\x12V[PPPPV[`@Qc5\xEAju`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R`\0\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c5\xEAju\x90`$\x01a\x01\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x03\x91\x90aD\xAFV[P\x92\x9APP`\x01`\x01`\xA0\x1B\x03\x8A\x16\x98Pa3E\x97PPPPPPPPW`@Qc\n\\^}`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x10\x12V[`\0`\x08`\x14\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xBB\x91\x90aE\x90V[\x92P`\x12\x83`\xFF\x16\x11\x15a3\xEEW`@Qc\x06Q\x98/`\xE1\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R`\x12`$\x82\x01R`D\x01a\x10\x12V[`\xFF\x81\x16\x15\x80\x15\x90a4\x06WP\x82`\xFF\x16\x81`\xFF\x16\x14\x15[\x15a4DW`\x0ET`\rT`\0\x19\x82\x14a4)Wa4%\x82\x84\x87a0WV[`\x0EU[`\0\x19\x81\x14a4AWa4=\x81\x84\x87a0WV[`\rU[PP[P`\x06\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x17\x90\x91U`\x08\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\xA0\x1B`\xFF\x86\x16\x02\x90\x92\x16\x91\x90\x91\x17\x91\x90\x93\x16\x17\x90\x91U\x90V[a4\xC2`\x01`\x01`\xA0\x1B\x03\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a1\xF6V[`@Qc\xE8\xED\xA9\xDF`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R0`D\x83\x01R`\0`d\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xE8\xED\xA9\xDF\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a59W`\0\x80\xFD[PZ\xF1\x15\x80\x15a5MW=`\0\x80>=`\0\xFD[PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\xF0\x99\xEF\xD5m\x0Cd\xF9\xA1\xAA\x13y\xA4p\xD8q9+g\xEAvx\xEDVY\xADK\xFE}\xD7eu\x82`@Qa\x17\x10\x91\x81R` \x01\x90V[`\0\x81\x83\x10a5\x9BW\x81a\x0F\x16V[P\x90\x91\x90PV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa5\xD4\x91\x90aE\xADV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x81\x83\x11a6LW`\0a\x0F\x16V[a\x0F\x16\x82\x84a?\x1DV[`\x0FT`\xFF\x16\x15a6zW`@Qc/\"\x81\x97`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a6\x85\x82a\x14\xCFV[\x90P\x80\x84\x11\x15a2mW`@Qc#\xDC)\x05`\xE2\x1B\x81R`\x04\x81\x01\x85\x90R`$\x81\x01\x82\x90R`D\x01a\x10\x12V[`\0`@Qc#\xB8r\xDD`\xE0\x1B\x81R\x84`\x04\x82\x01R\x83`$\x82\x01R\x82`D\x82\x01R` `\0`d\x83`\0\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a75W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x10\x12V[PPPPPV[\x80`\x02`\0\x82\x82Ta7N\x91\x90a?\x05V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` aFM\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x07\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x06T`\x01`\x01`\xA0\x1B\x03\x16`\0a7\xFEa+VV[\x90P\x80\x86\x11\x15a8[Wa8\x1B\x82a8\x16\x83\x89a?\x1DV[a1\rV[`\t\x80T`\0\x90a86\x90\x84\x90`\x01`\x01`\xF0\x1B\x03\x16a@\xDAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`\xF0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xF0\x1B\x03\x16\x02\x17\x90UP[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a8\x8B\x90\x84\x90a?\x1DV[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` aFM\x839\x81Q\x91R\x90` \x01a7\x8AV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a2mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x10\x12V[`\tT`\x01`\x01`\xF0\x1B\x03\x16\x80\x15a9uWa9Wa-\xD1V[a9c\x82`\0\x19a1\rV[P`\t\x80T`\x01`\x01`\xF0\x1B\x03\x19\x16\x90U[PPV[`\0a\x0F\x16\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\xC0V[`\0[\x83\x81\x10\x15a9\xA9W\x81\x81\x01Q\x83\x82\x01R` \x01a9\x91V[\x83\x81\x11\x15a2mWPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra9\xD2\x81` \x86\x01` \x86\x01a9\x8EV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x0F\x16` \x83\x01\x84a9\xBAV[`\0` \x82\x84\x03\x12\x15a:\x0BW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-\xCEW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a::W`\0\x80\xFD[\x825a:E\x81a:\x12V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Qa\x01 \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\x8CWa:\x8Ca:SV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a:\xFEWa:\xFEa:SV[`@R\x91\x90PV[`\0\x80`\0a\x02\xC0\x84\x86\x03\x12\x15a;\x1CW`\0\x80\xFD[`\x1F\x85\x81\x86\x01\x12a;,W`\0\x80\xFD[a;4a:iV[\x80a\x01 \x87\x01\x88\x81\x11\x15a;GW`\0\x80\xFD[\x87[\x81\x81\x10\x15a;jW\x805a;\\\x81a:\x12V[\x84R` \x93\x84\x01\x93\x01a;IV[P\x81\x96P\x88a\x01?\x89\x01\x12a;~W`\0\x80\xFD[a;\x86a:\x92V[\x92P\x82\x91Pa\x02\xA0\x88\x01\x89\x81\x11\x15a;\x9DW`\0\x80\xFD[\x80\x82\x10\x15a<\0W\x89\x85\x83\x01\x12a;\xB4W`\0\x80\x81\xFD[a;\xBCa:\xB4V[\x80``\x84\x01\x8C\x81\x11\x15a;\xCFW`\0\x80\x81\xFD[\x84[\x81\x81\x10\x15a;\xE9W\x805\x84R` \x93\x84\x01\x93\x01a;\xD1V[PP\x85RP` \x90\x93\x01\x92``\x91\x90\x91\x01\x90a;\x9DV[\x96\x99\x91\x98PP\x945\x95PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a<%W`\0\x80\xFD[\x835a<0\x81a:\x12V[\x92P` \x84\x015a<@\x81a:\x12V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0` \x82\x84\x03\x12\x15a<cW`\0\x80\xFD[\x815a\x0F\x16\x81a:\x12V[`\0\x80`@\x83\x85\x03\x12\x15a<\x81W`\0\x80\xFD[\x825\x91P` \x83\x015a<\x93\x81a:\x12V[\x80\x91PP\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a<\xB1W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xC8W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a<\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a<\xEBW`\0\x80\xFD[\x86` \x82`\x05\x1B\x85\x01\x01\x11\x15a=\0W`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a=gW`?\x19\x88\x86\x03\x01\x84Ra=U\x85\x83Qa9\xBAV[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a=9V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a=\x89W`\0\x80\xFD[\x835\x92P` \x84\x015a=\x9B\x81a:\x12V[\x91P`@\x84\x015a=\xAB\x81a:\x12V[\x80\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a=\xC9W`\0\x80\xFD[\x825a=\xD4\x81a:\x12V[\x91P` \x83\x015a<\x93\x81a:\x12V[\x805\x80\x15\x15\x81\x14a=\xF4W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a>\x0CW`\0\x80\xFD[\x825a>\x17\x81a:\x12V[\x91Pa>%` \x84\x01a=\xE4V[\x90P\x92P\x92\x90PV[`\xFF\x81\x16\x81\x14a-\xCEW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a>XW`\0\x80\xFD[\x875a>c\x81a:\x12V[\x96P` \x88\x015a>s\x81a:\x12V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015a>\x91\x81a>.V[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a>\xC0W`\0\x80\xFD[a\x0F\x16\x82a=\xE4V[`\0` \x82\x84\x03\x12\x15a>\xDBW`\0\x80\xFD[\x815c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\x16W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15a?\x18Wa?\x18a>\xEFV[P\x01\x90V[`\0\x82\x82\x10\x15a?/Wa?/a>\xEFV[P\x03\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a?HW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a?hWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[` \x80\x82R\x81\x81\x01R\x7FOwnable: caller is not the owner`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a?\xCBW`\0\x80\xFD[PQ\x91\x90PV[a\x02\xE0\x81\x01\x81\x86`\0[`\t\x81\x10\x15a@\x04W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a?\xDCV[PPPa\x01 \x82\x01\x85`\0[`\x04\x81\x10\x15a@WW\x81Q\x83`\0[`\x03\x81\x10\x15a@>W\x82Q\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a@\x1FV[PPP``\x92\x90\x92\x01\x91` \x91\x90\x91\x01\x90`\x01\x01a@\x10V[PPPa\x02\xA0\x82\x01\x93\x90\x93Ra\x02\xC0\x01R\x92\x91PPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a@\x88Wa@\x88a>\xEFV[P\x02\x90V[`\0\x82a@\xAAWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01`\xF0\x1B\x03\x82\x81\x16\x84\x82\x16\x80\x83\x03\x82\x11\x15a@\xD1Wa@\xD1a>\xEFV[\x01\x94\x93PPPPV[`\0`\x01`\x01`\xF0\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a@\xFAWa@\xFAa>\xEFV[\x03\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aA;W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aA\x16V[P\x94\x95\x94PPPPPV[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0aAe`\xA0\x83\x01\x86aA\x02V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15aA\x95W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\xACW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aA\xC0W`\0\x80\xFD[\x81Q\x81\x81\x11\x15aA\xD2WaA\xD2a:SV[\x80`\x05\x1B\x91PaA\xE3\x84\x83\x01a:\xD6V[\x81\x81R\x91\x83\x01\x84\x01\x91\x84\x81\x01\x90\x88\x84\x11\x15aA\xFDW`\0\x80\xFD[\x93\x85\x01\x93[\x83\x85\x10\x15aB\x1BW\x84Q\x82R\x93\x85\x01\x93\x90\x85\x01\x90aB\x02V[\x98\x97PPPPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB>W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aBXW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aBmW`\0\x80\xFD[\x92P\x92\x90PV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[`\0` \x82\x84\x03\x12\x15aB\x96W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xADW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12aB\xC1W`\0\x80\xFD[\x81Q\x81\x81\x11\x15aB\xD3WaB\xD3a:SV[aB\xE6`\x1F\x82\x01`\x1F\x19\x16` \x01a:\xD6V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15aB\xFDW`\0\x80\xFD[aC\x0E\x81` \x84\x01` \x86\x01a9\x8EV[P\x94\x93PPPPV[`\0`\x01\x82\x01aC)WaC)a>\xEFV[P`\x01\x01\x90V[``\x81R`\0aCC``\x83\x01\x86aA\x02V[` \x83\x01\x94\x90\x94RP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`@\x90\x91\x01R\x91\x90PV[`\x01\x81\x81[\x80\x85\x11\x15aC\x9EW\x81`\0\x19\x04\x82\x11\x15aC\x84WaC\x84a>\xEFV[\x80\x85\x16\x15aC\x91W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90aChV[P\x92P\x92\x90PV[`\0\x82aC\xB5WP`\x01a\x0F\x84V[\x81aC\xC2WP`\0a\x0F\x84V[\x81`\x01\x81\x14aC\xD8W`\x02\x81\x14aC\xE2WaC\xFEV[`\x01\x91PPa\x0F\x84V[`\xFF\x84\x11\x15aC\xF3WaC\xF3a>\xEFV[PP`\x01\x82\x1Ba\x0F\x84V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15aD!WP\x81\x81\na\x0F\x84V[aD+\x83\x83aCcV[\x80`\0\x19\x04\x82\x11\x15aD?WaD?a>\xEFV[\x02\x93\x92PPPV[`\0a\x0F\x16`\xFF\x84\x16\x83aC\xA6V[`\0`\xFF\x82\x16`\xFF\x84\x16\x80\x82\x10\x15aDpWaDpa>\xEFV[\x90\x03\x93\x92PPPV[\x80Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a=\xF4W`\0\x80\xFD[\x80Qa=\xF4\x81a:\x12V[\x80Qa=\xF4\x81a>.V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80a\x01\x80\x8D\x8F\x03\x12\x15aD\xD2W`\0\x80\xFD[\x8CQ\x9BPaD\xE2` \x8E\x01aDyV[\x9APaD\xF0`@\x8E\x01aDyV[\x99PaD\xFE``\x8E\x01aDyV[\x98PaE\x0C`\x80\x8E\x01aDyV[\x97PaE\x1A`\xA0\x8E\x01aDyV[\x96P`\xC0\x8D\x01Qd\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aE4W`\0\x80\xFD[\x95PaEB`\xE0\x8E\x01aD\x99V[\x94PaEQa\x01\0\x8E\x01aD\x99V[\x93PaE`a\x01 \x8E\x01aD\x99V[\x92PaEoa\x01@\x8E\x01aD\x99V[\x91PaE~a\x01`\x8E\x01aD\xA4V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[`\0` \x82\x84\x03\x12\x15aE\xA2W`\0\x80\xFD[\x81Qa\x0F\x16\x81a>.V[`\0\x80\x83T\x81`\x01\x82\x81\x1C\x91P\x80\x83\x16\x80aE\xC9W`\x7F\x83\x16\x92P[` \x80\x84\x10\x82\x03aE\xE8WcNH{q`\xE0\x1B\x86R`\"`\x04R`$\x86\xFD[\x81\x80\x15aE\xFCW`\x01\x81\x14aF\x11WaF>V[`\xFF\x19\x86\x16\x89R\x84\x15\x15\x85\x02\x89\x01\x96PaF>V[`\0\x8A\x81R` \x90 `\0[\x86\x81\x10\x15aF6W\x81T\x8B\x82\x01R\x90\x85\x01\x90\x83\x01aF\x1DV[PP\x84\x89\x01\x96P[P\x94\x98\x97PPPPPPPPV\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xEAd\x98\xAB\x16\xF3\xF2\xED\xC70Wz\x96S\x18kZdae\\\x1AUN\xD9\xF17\te4\xCB\xD1dsolcC\0\x08\x0F\x003";
    /// The deployed bytecode of the contract.
    pub static AAVEV2STABLECOINCELLAR_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AaveV2StablecoinCellar<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AaveV2StablecoinCellar<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AaveV2StablecoinCellar<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AaveV2StablecoinCellar<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AaveV2StablecoinCellar<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AaveV2StablecoinCellar))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AaveV2StablecoinCellar<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AAVEV2STABLECOINCELLAR_ABI.clone(),
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
                AAVEV2STABLECOINCELLAR_ABI.clone(),
                AAVEV2STABLECOINCELLAR_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `AAVE` (0x48ccda3c) function
        pub fn aave(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 204, 218, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accrualPeriod` (0xf6664155) function
        pub fn accrual_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([246, 102, 65, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accrue` (0xf8ba4cff) function
        pub fn accrue(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 186, 76, 255], ())
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
        ///Calls the contract's `assetAToken` (0xc17f6740) function
        pub fn asset_a_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([193, 127, 103, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assetDecimals` (0xc2d41601) function
        pub fn asset_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([194, 212, 22, 1], ())
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
        ///Calls the contract's `claimAndUnstake` (0xad004e20) function
        pub fn claim_and_unstake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 0, 78, 32], ())
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
        ///Calls the contract's `curveRegistryExchange` (0xac353510) function
        pub fn curve_registry_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([172, 53, 53, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
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
        ///Calls the contract's `enterPosition` (0x3dc6eabf) function
        pub fn enter_position(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([61, 198, 234, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `enterPosition` (0x6e08406b) function
        pub fn enter_position_with_assets(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 8, 64, 107], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitPosition` (0x78dc9059) function
        pub fn exit_position_with_assets(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 220, 144, 89], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `exitPosition` (0x99729216) function
        pub fn exit_position(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 114, 146, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `feesDistributor` (0x8e0bae7f) function
        pub fn fees_distributor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([142, 11, 174, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gravityBridge` (0xa4da2d02) function
        pub fn gravity_bridge(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([164, 218, 45, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incentivesController` (0xaf1df255) function
        pub fn incentives_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([175, 29, 242, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initiateShutdown` (0xef465d92) function
        pub fn initiate_shutdown(
            &self,
            empty_position: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 70, 93, 146], empty_position)
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
        ///Calls the contract's `lendingPool` (0xa59a9973) function
        pub fn lending_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([165, 154, 153, 115], ())
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
        ///Calls the contract's `maxLocked` (0x8fdc9dfa) function
        pub fn max_locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([143, 220, 157, 250], ())
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
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
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
        ///Calls the contract's `performanceFee` (0x87788782) function
        pub fn performance_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([135, 120, 135, 130], ())
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
        ///Calls the contract's `platformFee` (0x26232a2e) function
        pub fn platform_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([38, 35, 42, 46], ())
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
        ///Calls the contract's `rebalance` (0x15f4c611) function
        pub fn rebalance(
            &self,
            route: [::ethers::core::types::Address; 9],
            swap_params: [[::ethers::core::types::U256; 3]; 4],
            min_assets_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 244, 198, 17], (route, swap_params, min_assets_out))
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
        ///Calls the contract's `reinvest` (0x83b4918b) function
        pub fn reinvest(
            &self,
            min_assets_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 180, 145, 139], min_assets_out)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendFees` (0xdff90b5b) function
        pub fn send_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 249, 11, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAccrualPeriod` (0xef7ac883) function
        pub fn set_accrual_period(
            &self,
            new_accrual_period: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 122, 200, 131], new_accrual_period)
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
        ///Calls the contract's `setLiquidityLimit` (0xdf05a52a) function
        pub fn set_liquidity_limit(
            &self,
            new_limit: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 5, 165, 42], new_limit)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTrust` (0xcab59238) function
        pub fn set_trust(
            &self,
            position: ::ethers::core::types::Address,
            trust: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 181, 146, 56], (position, trust))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stkAAVE` (0x1fc29c01) function
        pub fn stk_aave(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([31, 194, 156, 1], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sushiswapRouter` (0xe9240c2d) function
        pub fn sushiswap_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([233, 36, 12, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweep` (0xb8dc491b) function
        pub fn sweep(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 220, 73, 27], (token, to))
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
        ///Calls the contract's `totalBalance` (0xad7a672f) function
        pub fn total_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([173, 122, 103, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalHoldings` (0xe9ec2e99) function
        pub fn total_holdings(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([233, 236, 46, 153], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLocked` (0x56891412) function
        pub fn total_locked(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([86, 137, 20, 18], ())
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
        ///Gets the contract's `Accrual` event
        pub fn accrual_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AccrualFilter> {
            self.0.event()
        }
        ///Gets the contract's `AccrualPeriodChanged` event
        pub fn accrual_period_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AccrualPeriodChangedFilter,
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
        ///Gets the contract's `ClaimAndUnstake` event
        pub fn claim_and_unstake_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ClaimAndUnstakeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `DepositIntoPosition` event
        pub fn deposit_into_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositIntoPositionFilter,
        > {
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
        ///Gets the contract's `EnterPosition` event
        pub fn enter_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EnterPositionFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ExitPosition` event
        pub fn exit_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExitPositionFilter,
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
        ///Gets the contract's `Reinvest` event
        pub fn reinvest_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ReinvestFilter,
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
        ///Gets the contract's `ShutdownInitiated` event
        pub fn shutdown_initiated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ShutdownInitiatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ShutdownLifted` event
        pub fn shutdown_lifted_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ShutdownLiftedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Sweep` event
        pub fn sweep_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SweepFilter> {
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
        ///Gets the contract's `WithdrawFromPosition` event
        pub fn withdraw_from_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFromPositionFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AaveV2StablecoinCellarEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AaveV2StablecoinCellar<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `STATE_AccrualOngoing` with signature `STATE_AccrualOngoing()` and selector `0xd70cc726`
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
    #[etherror(name = "STATE_AccrualOngoing", abi = "STATE_AccrualOngoing()")]
    pub struct STATE_AccrualOngoing;
    ///Custom Error type `STATE_ContractShutdown` with signature `STATE_ContractShutdown()` and selector `0x5e45032e`
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
    #[etherror(name = "STATE_ContractShutdown", abi = "STATE_ContractShutdown()")]
    pub struct STATE_ContractShutdown;
    ///Custom Error type `USR_DepositRestricted` with signature `USR_DepositRestricted(uint256,uint256)` and selector `0x8f70a414`
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
        name = "USR_DepositRestricted",
        abi = "USR_DepositRestricted(uint256,uint256)"
    )]
    pub struct USR_DepositRestricted {
        pub assets: ::ethers::core::types::U256,
        pub max_deposit: ::ethers::core::types::U256,
    }
    ///Custom Error type `USR_ProtectedAsset` with signature `USR_ProtectedAsset(address)` and selector `0x39b85491`
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
    #[etherror(name = "USR_ProtectedAsset", abi = "USR_ProtectedAsset(address)")]
    pub struct USR_ProtectedAsset {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `USR_SamePosition` with signature `USR_SamePosition(address)` and selector `0x0c275d9e`
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
    #[etherror(name = "USR_SamePosition", abi = "USR_SamePosition(address)")]
    pub struct USR_SamePosition {
        pub position: ::ethers::core::types::Address,
    }
    ///Custom Error type `USR_TooManyDecimals` with signature `USR_TooManyDecimals(uint8,uint8)` and selector `0x0ca3305e`
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
    #[etherror(name = "USR_TooManyDecimals", abi = "USR_TooManyDecimals(uint8,uint8)")]
    pub struct USR_TooManyDecimals {
        pub new_decimals: u8,
        pub max_decimals: u8,
    }
    ///Custom Error type `USR_UnsupportedPosition` with signature `USR_UnsupportedPosition(address)` and selector `0x14b8bcfa`
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
        name = "USR_UnsupportedPosition",
        abi = "USR_UnsupportedPosition(address)"
    )]
    pub struct USR_UnsupportedPosition {
        pub unsupported_position: ::ethers::core::types::Address,
    }
    ///Custom Error type `USR_UntrustedPosition` with signature `USR_UntrustedPosition(address)` and selector `0x86433f2b`
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
    #[etherror(name = "USR_UntrustedPosition", abi = "USR_UntrustedPosition(address)")]
    pub struct USR_UntrustedPosition {
        pub position: ::ethers::core::types::Address,
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
    pub enum AaveV2StablecoinCellarErrors {
        STATE_AccrualOngoing(STATE_AccrualOngoing),
        STATE_ContractShutdown(STATE_ContractShutdown),
        USR_DepositRestricted(USR_DepositRestricted),
        USR_ProtectedAsset(USR_ProtectedAsset),
        USR_SamePosition(USR_SamePosition),
        USR_TooManyDecimals(USR_TooManyDecimals),
        USR_UnsupportedPosition(USR_UnsupportedPosition),
        USR_UntrustedPosition(USR_UntrustedPosition),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AaveV2StablecoinCellarErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <STATE_AccrualOngoing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::STATE_AccrualOngoing(decoded));
            }
            if let Ok(decoded) = <STATE_ContractShutdown as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::STATE_ContractShutdown(decoded));
            }
            if let Ok(decoded) = <USR_DepositRestricted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_DepositRestricted(decoded));
            }
            if let Ok(decoded) = <USR_ProtectedAsset as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_ProtectedAsset(decoded));
            }
            if let Ok(decoded) = <USR_SamePosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_SamePosition(decoded));
            }
            if let Ok(decoded) = <USR_TooManyDecimals as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_TooManyDecimals(decoded));
            }
            if let Ok(decoded) = <USR_UnsupportedPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_UnsupportedPosition(decoded));
            }
            if let Ok(decoded) = <USR_UntrustedPosition as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::USR_UntrustedPosition(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AaveV2StablecoinCellarErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::STATE_AccrualOngoing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::STATE_ContractShutdown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_DepositRestricted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_ProtectedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_SamePosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_TooManyDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_UnsupportedPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::USR_UntrustedPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AaveV2StablecoinCellarErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <STATE_AccrualOngoing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <STATE_ContractShutdown as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_DepositRestricted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_ProtectedAsset as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_SamePosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_TooManyDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_UnsupportedPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <USR_UntrustedPosition as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AaveV2StablecoinCellarErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::STATE_AccrualOngoing(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::STATE_ContractShutdown(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::USR_DepositRestricted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::USR_ProtectedAsset(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::USR_SamePosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::USR_TooManyDecimals(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::USR_UnsupportedPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::USR_UntrustedPosition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AaveV2StablecoinCellarErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<STATE_AccrualOngoing> for AaveV2StablecoinCellarErrors {
        fn from(value: STATE_AccrualOngoing) -> Self {
            Self::STATE_AccrualOngoing(value)
        }
    }
    impl ::core::convert::From<STATE_ContractShutdown> for AaveV2StablecoinCellarErrors {
        fn from(value: STATE_ContractShutdown) -> Self {
            Self::STATE_ContractShutdown(value)
        }
    }
    impl ::core::convert::From<USR_DepositRestricted> for AaveV2StablecoinCellarErrors {
        fn from(value: USR_DepositRestricted) -> Self {
            Self::USR_DepositRestricted(value)
        }
    }
    impl ::core::convert::From<USR_ProtectedAsset> for AaveV2StablecoinCellarErrors {
        fn from(value: USR_ProtectedAsset) -> Self {
            Self::USR_ProtectedAsset(value)
        }
    }
    impl ::core::convert::From<USR_SamePosition> for AaveV2StablecoinCellarErrors {
        fn from(value: USR_SamePosition) -> Self {
            Self::USR_SamePosition(value)
        }
    }
    impl ::core::convert::From<USR_TooManyDecimals> for AaveV2StablecoinCellarErrors {
        fn from(value: USR_TooManyDecimals) -> Self {
            Self::USR_TooManyDecimals(value)
        }
    }
    impl ::core::convert::From<USR_UnsupportedPosition>
    for AaveV2StablecoinCellarErrors {
        fn from(value: USR_UnsupportedPosition) -> Self {
            Self::USR_UnsupportedPosition(value)
        }
    }
    impl ::core::convert::From<USR_UntrustedPosition> for AaveV2StablecoinCellarErrors {
        fn from(value: USR_UntrustedPosition) -> Self {
            Self::USR_UntrustedPosition(value)
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
    #[ethevent(name = "Accrual", abi = "Accrual(uint256,uint256,uint256)")]
    pub struct AccrualFilter {
        pub platform_fees: ::ethers::core::types::U256,
        pub performance_fees: ::ethers::core::types::U256,
        pub yield_: ::ethers::core::types::U256,
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
        name = "AccrualPeriodChanged",
        abi = "AccrualPeriodChanged(uint32,uint32)"
    )]
    pub struct AccrualPeriodChangedFilter {
        pub old_period: u32,
        pub new_period: u32,
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
    #[ethevent(name = "ClaimAndUnstake", abi = "ClaimAndUnstake(uint256)")]
    pub struct ClaimAndUnstakeFilter {
        pub rewards: ::ethers::core::types::U256,
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
        name = "DepositIntoPosition",
        abi = "DepositIntoPosition(address,uint256)"
    )]
    pub struct DepositIntoPositionFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
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
    #[ethevent(name = "EnterPosition", abi = "EnterPosition(address,uint256)")]
    pub struct EnterPositionFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
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
    #[ethevent(name = "ExitPosition", abi = "ExitPosition(address,uint256)")]
    pub struct ExitPositionFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "Rebalance", abi = "Rebalance(address,address,uint256)")]
    pub struct RebalanceFilter {
        #[ethevent(indexed)]
        pub old_asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_asset: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
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
    #[ethevent(name = "Reinvest", abi = "Reinvest(address,uint256,uint256)")]
    pub struct ReinvestFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub rewards: ::ethers::core::types::U256,
        pub assets: ::ethers::core::types::U256,
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
    #[ethevent(name = "ShutdownInitiated", abi = "ShutdownInitiated(bool)")]
    pub struct ShutdownInitiatedFilter {
        pub empty_positions: bool,
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
    #[ethevent(name = "ShutdownLifted", abi = "ShutdownLifted()")]
    pub struct ShutdownLiftedFilter;
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
    #[ethevent(name = "Sweep", abi = "Sweep(address,address,uint256)")]
    pub struct SweepFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
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
    #[ethevent(name = "TrustChanged", abi = "TrustChanged(address,bool)")]
    pub struct TrustChangedFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub trusted: bool,
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
    #[ethevent(
        name = "WithdrawFromPosition",
        abi = "WithdrawFromPosition(address,uint256)"
    )]
    pub struct WithdrawFromPositionFilter {
        #[ethevent(indexed)]
        pub position: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
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
    pub enum AaveV2StablecoinCellarEvents {
        AccrualFilter(AccrualFilter),
        AccrualPeriodChangedFilter(AccrualPeriodChangedFilter),
        ApprovalFilter(ApprovalFilter),
        ClaimAndUnstakeFilter(ClaimAndUnstakeFilter),
        DepositFilter(DepositFilter),
        DepositIntoPositionFilter(DepositIntoPositionFilter),
        DepositLimitChangedFilter(DepositLimitChangedFilter),
        EnterPositionFilter(EnterPositionFilter),
        ExitPositionFilter(ExitPositionFilter),
        FeesDistributorChangedFilter(FeesDistributorChangedFilter),
        LiquidityLimitChangedFilter(LiquidityLimitChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PerformanceFeeChangedFilter(PerformanceFeeChangedFilter),
        PlatformFeeChangedFilter(PlatformFeeChangedFilter),
        RebalanceFilter(RebalanceFilter),
        ReinvestFilter(ReinvestFilter),
        SendFeesFilter(SendFeesFilter),
        ShutdownInitiatedFilter(ShutdownInitiatedFilter),
        ShutdownLiftedFilter(ShutdownLiftedFilter),
        SweepFilter(SweepFilter),
        TransferFilter(TransferFilter),
        TrustChangedFilter(TrustChangedFilter),
        WithdrawFilter(WithdrawFilter),
        WithdrawFromPositionFilter(WithdrawFromPositionFilter),
    }
    impl ::ethers::contract::EthLogDecode for AaveV2StablecoinCellarEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AccrualFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::AccrualFilter(decoded));
            }
            if let Ok(decoded) = AccrualPeriodChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::AccrualPeriodChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ClaimAndUnstakeFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ClaimAndUnstakeFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = DepositIntoPositionFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::DepositIntoPositionFilter(decoded),
                );
            }
            if let Ok(decoded) = DepositLimitChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::DepositLimitChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = EnterPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::EnterPositionFilter(decoded));
            }
            if let Ok(decoded) = ExitPositionFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ExitPositionFilter(decoded));
            }
            if let Ok(decoded) = FeesDistributorChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::FeesDistributorChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = LiquidityLimitChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::LiquidityLimitChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PerformanceFeeChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::PerformanceFeeChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = PlatformFeeChangedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::PlatformFeeChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = RebalanceFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::RebalanceFilter(decoded));
            }
            if let Ok(decoded) = ReinvestFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ReinvestFilter(decoded));
            }
            if let Ok(decoded) = SendFeesFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SendFeesFilter(decoded));
            }
            if let Ok(decoded) = ShutdownInitiatedFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::ShutdownInitiatedFilter(decoded),
                );
            }
            if let Ok(decoded) = ShutdownLiftedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::ShutdownLiftedFilter(decoded));
            }
            if let Ok(decoded) = SweepFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::SweepFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = TrustChangedFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::TrustChangedFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(AaveV2StablecoinCellarEvents::WithdrawFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFromPositionFilter::decode_log(log) {
                return Ok(
                    AaveV2StablecoinCellarEvents::WithdrawFromPositionFilter(decoded),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AaveV2StablecoinCellarEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AccrualFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccrualPeriodChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAndUnstakeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositIntoPositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositLimitChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EnterPositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExitPositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesDistributorChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidityLimitChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PerformanceFeeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PlatformFeeChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RebalanceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReinvestFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShutdownInitiatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ShutdownLiftedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SweepFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrustChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawFromPositionFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AccrualFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: AccrualFilter) -> Self {
            Self::AccrualFilter(value)
        }
    }
    impl ::core::convert::From<AccrualPeriodChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: AccrualPeriodChangedFilter) -> Self {
            Self::AccrualPeriodChangedFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ClaimAndUnstakeFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: ClaimAndUnstakeFilter) -> Self {
            Self::ClaimAndUnstakeFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<DepositIntoPositionFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: DepositIntoPositionFilter) -> Self {
            Self::DepositIntoPositionFilter(value)
        }
    }
    impl ::core::convert::From<DepositLimitChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: DepositLimitChangedFilter) -> Self {
            Self::DepositLimitChangedFilter(value)
        }
    }
    impl ::core::convert::From<EnterPositionFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: EnterPositionFilter) -> Self {
            Self::EnterPositionFilter(value)
        }
    }
    impl ::core::convert::From<ExitPositionFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: ExitPositionFilter) -> Self {
            Self::ExitPositionFilter(value)
        }
    }
    impl ::core::convert::From<FeesDistributorChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: FeesDistributorChangedFilter) -> Self {
            Self::FeesDistributorChangedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidityLimitChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: LiquidityLimitChangedFilter) -> Self {
            Self::LiquidityLimitChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PerformanceFeeChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: PerformanceFeeChangedFilter) -> Self {
            Self::PerformanceFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<PlatformFeeChangedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: PlatformFeeChangedFilter) -> Self {
            Self::PlatformFeeChangedFilter(value)
        }
    }
    impl ::core::convert::From<RebalanceFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: RebalanceFilter) -> Self {
            Self::RebalanceFilter(value)
        }
    }
    impl ::core::convert::From<ReinvestFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: ReinvestFilter) -> Self {
            Self::ReinvestFilter(value)
        }
    }
    impl ::core::convert::From<SendFeesFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: SendFeesFilter) -> Self {
            Self::SendFeesFilter(value)
        }
    }
    impl ::core::convert::From<ShutdownInitiatedFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: ShutdownInitiatedFilter) -> Self {
            Self::ShutdownInitiatedFilter(value)
        }
    }
    impl ::core::convert::From<ShutdownLiftedFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: ShutdownLiftedFilter) -> Self {
            Self::ShutdownLiftedFilter(value)
        }
    }
    impl ::core::convert::From<SweepFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: SweepFilter) -> Self {
            Self::SweepFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TrustChangedFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: TrustChangedFilter) -> Self {
            Self::TrustChangedFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for AaveV2StablecoinCellarEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFromPositionFilter>
    for AaveV2StablecoinCellarEvents {
        fn from(value: WithdrawFromPositionFilter) -> Self {
            Self::WithdrawFromPositionFilter(value)
        }
    }
    ///Container type for all input parameters for the `AAVE` function with signature `AAVE()` and selector `0x48ccda3c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "AAVE", abi = "AAVE()")]
    pub struct AaveCall;
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
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `accrualPeriod` function with signature `accrualPeriod()` and selector `0xf6664155`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accrualPeriod", abi = "accrualPeriod()")]
    pub struct AccrualPeriodCall;
    ///Container type for all input parameters for the `accrue` function with signature `accrue()` and selector `0xf8ba4cff`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accrue", abi = "accrue()")]
    pub struct AccrueCall;
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
    ///Container type for all input parameters for the `assetAToken` function with signature `assetAToken()` and selector `0xc17f6740`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "assetAToken", abi = "assetAToken()")]
    pub struct AssetATokenCall;
    ///Container type for all input parameters for the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "assetDecimals", abi = "assetDecimals()")]
    pub struct AssetDecimalsCall;
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
    ///Container type for all input parameters for the `claimAndUnstake` function with signature `claimAndUnstake()` and selector `0xad004e20`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimAndUnstake", abi = "claimAndUnstake()")]
    pub struct ClaimAndUnstakeCall;
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
    ///Container type for all input parameters for the `curveRegistryExchange` function with signature `curveRegistryExchange()` and selector `0xac353510`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "curveRegistryExchange", abi = "curveRegistryExchange()")]
    pub struct CurveRegistryExchangeCall;
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
    ///Container type for all input parameters for the `enterPosition` function with signature `enterPosition()` and selector `0x3dc6eabf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "enterPosition", abi = "enterPosition()")]
    pub struct EnterPositionCall;
    ///Container type for all input parameters for the `enterPosition` function with signature `enterPosition(uint256)` and selector `0x6e08406b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "enterPosition", abi = "enterPosition(uint256)")]
    pub struct EnterPositionWithAssetsCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exitPosition` function with signature `exitPosition(uint256)` and selector `0x78dc9059`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exitPosition", abi = "exitPosition(uint256)")]
    pub struct ExitPositionWithAssetsCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `exitPosition` function with signature `exitPosition()` and selector `0x99729216`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "exitPosition", abi = "exitPosition()")]
    pub struct ExitPositionCall;
    ///Container type for all input parameters for the `feesDistributor` function with signature `feesDistributor()` and selector `0x8e0bae7f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "feesDistributor", abi = "feesDistributor()")]
    pub struct FeesDistributorCall;
    ///Container type for all input parameters for the `gravityBridge` function with signature `gravityBridge()` and selector `0xa4da2d02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "gravityBridge", abi = "gravityBridge()")]
    pub struct GravityBridgeCall;
    ///Container type for all input parameters for the `incentivesController` function with signature `incentivesController()` and selector `0xaf1df255`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "incentivesController", abi = "incentivesController()")]
    pub struct IncentivesControllerCall;
    ///Container type for all input parameters for the `initiateShutdown` function with signature `initiateShutdown(bool)` and selector `0xef465d92`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initiateShutdown", abi = "initiateShutdown(bool)")]
    pub struct InitiateShutdownCall {
        pub empty_position: bool,
    }
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
    ///Container type for all input parameters for the `lendingPool` function with signature `lendingPool()` and selector `0xa59a9973`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lendingPool", abi = "lendingPool()")]
    pub struct LendingPoolCall;
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
    ///Container type for all input parameters for the `maxLocked` function with signature `maxLocked()` and selector `0x8fdc9dfa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "maxLocked", abi = "maxLocked()")]
    pub struct MaxLockedCall;
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
    ///Container type for all input parameters for the `performanceFee` function with signature `performanceFee()` and selector `0x87788782`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "performanceFee", abi = "performanceFee()")]
    pub struct PerformanceFeeCall;
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
    ///Container type for all input parameters for the `platformFee` function with signature `platformFee()` and selector `0x26232a2e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "platformFee", abi = "platformFee()")]
    pub struct PlatformFeeCall;
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
    ///Container type for all input parameters for the `rebalance` function with signature `rebalance(address[9],uint256[3][4],uint256)` and selector `0x15f4c611`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rebalance", abi = "rebalance(address[9],uint256[3][4],uint256)")]
    pub struct RebalanceCall {
        pub route: [::ethers::core::types::Address; 9],
        pub swap_params: [[::ethers::core::types::U256; 3]; 4],
        pub min_assets_out: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `reinvest` function with signature `reinvest(uint256)` and selector `0x83b4918b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "reinvest", abi = "reinvest(uint256)")]
    pub struct ReinvestCall {
        pub min_assets_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    ///Container type for all input parameters for the `setAccrualPeriod` function with signature `setAccrualPeriod(uint32)` and selector `0xef7ac883`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAccrualPeriod", abi = "setAccrualPeriod(uint32)")]
    pub struct SetAccrualPeriodCall {
        pub new_accrual_period: u32,
    }
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
    ///Container type for all input parameters for the `setTrust` function with signature `setTrust(address,bool)` and selector `0xcab59238`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setTrust", abi = "setTrust(address,bool)")]
    pub struct SetTrustCall {
        pub position: ::ethers::core::types::Address,
        pub trust: bool,
    }
    ///Container type for all input parameters for the `stkAAVE` function with signature `stkAAVE()` and selector `0x1fc29c01`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stkAAVE", abi = "stkAAVE()")]
    pub struct StkAAVECall;
    ///Container type for all input parameters for the `sushiswapRouter` function with signature `sushiswapRouter()` and selector `0xe9240c2d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sushiswapRouter", abi = "sushiswapRouter()")]
    pub struct SushiswapRouterCall;
    ///Container type for all input parameters for the `sweep` function with signature `sweep(address,address)` and selector `0xb8dc491b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sweep", abi = "sweep(address,address)")]
    pub struct SweepCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `totalBalance` function with signature `totalBalance()` and selector `0xad7a672f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalBalance", abi = "totalBalance()")]
    pub struct TotalBalanceCall;
    ///Container type for all input parameters for the `totalHoldings` function with signature `totalHoldings()` and selector `0xe9ec2e99`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalHoldings", abi = "totalHoldings()")]
    pub struct TotalHoldingsCall;
    ///Container type for all input parameters for the `totalLocked` function with signature `totalLocked()` and selector `0x56891412`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalLocked", abi = "totalLocked()")]
    pub struct TotalLockedCall;
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
    pub enum AaveV2StablecoinCellarCalls {
        Aave(AaveCall),
        DomainSeparator(DomainSeparatorCall),
        Weth(WethCall),
        AccrualPeriod(AccrualPeriodCall),
        Accrue(AccrueCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        AssetAToken(AssetATokenCall),
        AssetDecimals(AssetDecimalsCall),
        BalanceOf(BalanceOfCall),
        ClaimAndUnstake(ClaimAndUnstakeCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToShares(ConvertToSharesCall),
        CurveRegistryExchange(CurveRegistryExchangeCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        DepositLimit(DepositLimitCall),
        EnterPosition(EnterPositionCall),
        EnterPositionWithAssets(EnterPositionWithAssetsCall),
        ExitPositionWithAssets(ExitPositionWithAssetsCall),
        ExitPosition(ExitPositionCall),
        FeesDistributor(FeesDistributorCall),
        GravityBridge(GravityBridgeCall),
        IncentivesController(IncentivesControllerCall),
        InitiateShutdown(InitiateShutdownCall),
        IsShutdown(IsShutdownCall),
        IsTrusted(IsTrustedCall),
        LastAccrual(LastAccrualCall),
        LendingPool(LendingPoolCall),
        LiftShutdown(LiftShutdownCall),
        LiquidityLimit(LiquidityLimitCall),
        MaxDeposit(MaxDepositCall),
        MaxLocked(MaxLockedCall),
        MaxMint(MaxMintCall),
        MaxRedeem(MaxRedeemCall),
        MaxWithdraw(MaxWithdrawCall),
        Mint(MintCall),
        Multicall(MulticallCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        PerformanceFee(PerformanceFeeCall),
        Permit(PermitCall),
        PlatformFee(PlatformFeeCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewMint(PreviewMintCall),
        PreviewRedeem(PreviewRedeemCall),
        PreviewWithdraw(PreviewWithdrawCall),
        Rebalance(RebalanceCall),
        Redeem(RedeemCall),
        Reinvest(ReinvestCall),
        RenounceOwnership(RenounceOwnershipCall),
        SendFees(SendFeesCall),
        SetAccrualPeriod(SetAccrualPeriodCall),
        SetDepositLimit(SetDepositLimitCall),
        SetFeesDistributor(SetFeesDistributorCall),
        SetLiquidityLimit(SetLiquidityLimitCall),
        SetTrust(SetTrustCall),
        StkAAVE(StkAAVECall),
        SushiswapRouter(SushiswapRouterCall),
        Sweep(SweepCall),
        Symbol(SymbolCall),
        TotalAssets(TotalAssetsCall),
        TotalBalance(TotalBalanceCall),
        TotalHoldings(TotalHoldingsCall),
        TotalLocked(TotalLockedCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for AaveV2StablecoinCellarCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AaveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Aave(decoded));
            }
            if let Ok(decoded) = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <AccrualPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AccrualPeriod(decoded));
            }
            if let Ok(decoded) = <AccrueCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Accrue(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
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
            if let Ok(decoded) = <AssetATokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetAToken(decoded));
            }
            if let Ok(decoded) = <AssetDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AssetDecimals(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <ClaimAndUnstakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimAndUnstake(decoded));
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
            if let Ok(decoded) = <CurveRegistryExchangeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurveRegistryExchange(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Decimals(decoded));
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
            if let Ok(decoded) = <EnterPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnterPosition(decoded));
            }
            if let Ok(decoded) = <EnterPositionWithAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnterPositionWithAssets(decoded));
            }
            if let Ok(decoded) = <ExitPositionWithAssetsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExitPositionWithAssets(decoded));
            }
            if let Ok(decoded) = <ExitPositionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExitPosition(decoded));
            }
            if let Ok(decoded) = <FeesDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FeesDistributor(decoded));
            }
            if let Ok(decoded) = <GravityBridgeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GravityBridge(decoded));
            }
            if let Ok(decoded) = <IncentivesControllerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IncentivesController(decoded));
            }
            if let Ok(decoded) = <InitiateShutdownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InitiateShutdown(decoded));
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
            if let Ok(decoded) = <LendingPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LendingPool(decoded));
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
            if let Ok(decoded) = <MaxLockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MaxLocked(decoded));
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
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PerformanceFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PerformanceFee(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <PlatformFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PlatformFee(decoded));
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
            if let Ok(decoded) = <ReinvestCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Reinvest(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <SendFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendFees(decoded));
            }
            if let Ok(decoded) = <SetAccrualPeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAccrualPeriod(decoded));
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
            if let Ok(decoded) = <SetLiquidityLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetLiquidityLimit(decoded));
            }
            if let Ok(decoded) = <SetTrustCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTrust(decoded));
            }
            if let Ok(decoded) = <StkAAVECall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StkAAVE(decoded));
            }
            if let Ok(decoded) = <SushiswapRouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SushiswapRouter(decoded));
            }
            if let Ok(decoded) = <SweepCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sweep(decoded));
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
            if let Ok(decoded) = <TotalBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalBalance(decoded));
            }
            if let Ok(decoded) = <TotalHoldingsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalHoldings(decoded));
            }
            if let Ok(decoded) = <TotalLockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalLocked(decoded));
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
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AaveV2StablecoinCellarCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Aave(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AccrualPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Accrue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetAToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimAndUnstake(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurveRegistryExchange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnterPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EnterPositionWithAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExitPositionWithAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExitPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeesDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GravityBridge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncentivesController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InitiateShutdown(element) => {
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
                Self::LendingPool(element) => {
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
                Self::MaxLocked(element) => {
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
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PerformanceFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PlatformFee(element) => {
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
                Self::Rebalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Reinvest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAccrualPeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetDepositLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetFeesDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLiquidityLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTrust(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StkAAVE(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SushiswapRouter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sweep(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalHoldings(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalLocked(element) => {
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
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AaveV2StablecoinCellarCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Aave(element) => ::core::fmt::Display::fmt(element, f),
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::AccrualPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::Accrue(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetAToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimAndUnstake(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurveRegistryExchange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnterPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnterPositionWithAssets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExitPositionWithAssets(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExitPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesDistributor(element) => ::core::fmt::Display::fmt(element, f),
                Self::GravityBridge(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncentivesController(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitiateShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsTrusted(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastAccrual(element) => ::core::fmt::Display::fmt(element, f),
                Self::LendingPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiftShutdown(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PerformanceFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PlatformFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewMint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rebalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Reinvest(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAccrualPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetDepositLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLiquidityLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTrust(element) => ::core::fmt::Display::fmt(element, f),
                Self::StkAAVE(element) => ::core::fmt::Display::fmt(element, f),
                Self::SushiswapRouter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sweep(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalHoldings(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AaveCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AaveCall) -> Self {
            Self::Aave(value)
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for AaveV2StablecoinCellarCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<WethCall> for AaveV2StablecoinCellarCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AccrualPeriodCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AccrualPeriodCall) -> Self {
            Self::AccrualPeriod(value)
        }
    }
    impl ::core::convert::From<AccrueCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AccrueCall) -> Self {
            Self::Accrue(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<AssetATokenCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AssetATokenCall) -> Self {
            Self::AssetAToken(value)
        }
    }
    impl ::core::convert::From<AssetDecimalsCall> for AaveV2StablecoinCellarCalls {
        fn from(value: AssetDecimalsCall) -> Self {
            Self::AssetDecimals(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for AaveV2StablecoinCellarCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<ClaimAndUnstakeCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ClaimAndUnstakeCall) -> Self {
            Self::ClaimAndUnstake(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<CurveRegistryExchangeCall>
    for AaveV2StablecoinCellarCalls {
        fn from(value: CurveRegistryExchangeCall) -> Self {
            Self::CurveRegistryExchange(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for AaveV2StablecoinCellarCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for AaveV2StablecoinCellarCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DepositLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(value: DepositLimitCall) -> Self {
            Self::DepositLimit(value)
        }
    }
    impl ::core::convert::From<EnterPositionCall> for AaveV2StablecoinCellarCalls {
        fn from(value: EnterPositionCall) -> Self {
            Self::EnterPosition(value)
        }
    }
    impl ::core::convert::From<EnterPositionWithAssetsCall>
    for AaveV2StablecoinCellarCalls {
        fn from(value: EnterPositionWithAssetsCall) -> Self {
            Self::EnterPositionWithAssets(value)
        }
    }
    impl ::core::convert::From<ExitPositionWithAssetsCall>
    for AaveV2StablecoinCellarCalls {
        fn from(value: ExitPositionWithAssetsCall) -> Self {
            Self::ExitPositionWithAssets(value)
        }
    }
    impl ::core::convert::From<ExitPositionCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ExitPositionCall) -> Self {
            Self::ExitPosition(value)
        }
    }
    impl ::core::convert::From<FeesDistributorCall> for AaveV2StablecoinCellarCalls {
        fn from(value: FeesDistributorCall) -> Self {
            Self::FeesDistributor(value)
        }
    }
    impl ::core::convert::From<GravityBridgeCall> for AaveV2StablecoinCellarCalls {
        fn from(value: GravityBridgeCall) -> Self {
            Self::GravityBridge(value)
        }
    }
    impl ::core::convert::From<IncentivesControllerCall>
    for AaveV2StablecoinCellarCalls {
        fn from(value: IncentivesControllerCall) -> Self {
            Self::IncentivesController(value)
        }
    }
    impl ::core::convert::From<InitiateShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(value: InitiateShutdownCall) -> Self {
            Self::InitiateShutdown(value)
        }
    }
    impl ::core::convert::From<IsShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(value: IsShutdownCall) -> Self {
            Self::IsShutdown(value)
        }
    }
    impl ::core::convert::From<IsTrustedCall> for AaveV2StablecoinCellarCalls {
        fn from(value: IsTrustedCall) -> Self {
            Self::IsTrusted(value)
        }
    }
    impl ::core::convert::From<LastAccrualCall> for AaveV2StablecoinCellarCalls {
        fn from(value: LastAccrualCall) -> Self {
            Self::LastAccrual(value)
        }
    }
    impl ::core::convert::From<LendingPoolCall> for AaveV2StablecoinCellarCalls {
        fn from(value: LendingPoolCall) -> Self {
            Self::LendingPool(value)
        }
    }
    impl ::core::convert::From<LiftShutdownCall> for AaveV2StablecoinCellarCalls {
        fn from(value: LiftShutdownCall) -> Self {
            Self::LiftShutdown(value)
        }
    }
    impl ::core::convert::From<LiquidityLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(value: LiquidityLimitCall) -> Self {
            Self::LiquidityLimit(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxLockedCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MaxLockedCall) -> Self {
            Self::MaxLocked(value)
        }
    }
    impl ::core::convert::From<MaxMintCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MaxMintCall) -> Self {
            Self::MaxMint(value)
        }
    }
    impl ::core::convert::From<MaxRedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MaxRedeemCall) -> Self {
            Self::MaxRedeem(value)
        }
    }
    impl ::core::convert::From<MaxWithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MaxWithdrawCall) -> Self {
            Self::MaxWithdraw(value)
        }
    }
    impl ::core::convert::From<MintCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for AaveV2StablecoinCellarCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<NameCall> for AaveV2StablecoinCellarCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for AaveV2StablecoinCellarCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AaveV2StablecoinCellarCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PerformanceFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PerformanceFeeCall) -> Self {
            Self::PerformanceFee(value)
        }
    }
    impl ::core::convert::From<PermitCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PlatformFeeCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PlatformFeeCall) -> Self {
            Self::PlatformFee(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewMintCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PreviewMintCall) -> Self {
            Self::PreviewMint(value)
        }
    }
    impl ::core::convert::From<PreviewRedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PreviewRedeemCall) -> Self {
            Self::PreviewRedeem(value)
        }
    }
    impl ::core::convert::From<PreviewWithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(value: PreviewWithdrawCall) -> Self {
            Self::PreviewWithdraw(value)
        }
    }
    impl ::core::convert::From<RebalanceCall> for AaveV2StablecoinCellarCalls {
        fn from(value: RebalanceCall) -> Self {
            Self::Rebalance(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for AaveV2StablecoinCellarCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<ReinvestCall> for AaveV2StablecoinCellarCalls {
        fn from(value: ReinvestCall) -> Self {
            Self::Reinvest(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AaveV2StablecoinCellarCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<SendFeesCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SendFeesCall) -> Self {
            Self::SendFees(value)
        }
    }
    impl ::core::convert::From<SetAccrualPeriodCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SetAccrualPeriodCall) -> Self {
            Self::SetAccrualPeriod(value)
        }
    }
    impl ::core::convert::From<SetDepositLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SetDepositLimitCall) -> Self {
            Self::SetDepositLimit(value)
        }
    }
    impl ::core::convert::From<SetFeesDistributorCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SetFeesDistributorCall) -> Self {
            Self::SetFeesDistributor(value)
        }
    }
    impl ::core::convert::From<SetLiquidityLimitCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SetLiquidityLimitCall) -> Self {
            Self::SetLiquidityLimit(value)
        }
    }
    impl ::core::convert::From<SetTrustCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SetTrustCall) -> Self {
            Self::SetTrust(value)
        }
    }
    impl ::core::convert::From<StkAAVECall> for AaveV2StablecoinCellarCalls {
        fn from(value: StkAAVECall) -> Self {
            Self::StkAAVE(value)
        }
    }
    impl ::core::convert::From<SushiswapRouterCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SushiswapRouterCall) -> Self {
            Self::SushiswapRouter(value)
        }
    }
    impl ::core::convert::From<SweepCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SweepCall) -> Self {
            Self::Sweep(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for AaveV2StablecoinCellarCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalAssetsCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TotalAssetsCall) -> Self {
            Self::TotalAssets(value)
        }
    }
    impl ::core::convert::From<TotalBalanceCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TotalBalanceCall) -> Self {
            Self::TotalBalance(value)
        }
    }
    impl ::core::convert::From<TotalHoldingsCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TotalHoldingsCall) -> Self {
            Self::TotalHoldings(value)
        }
    }
    impl ::core::convert::From<TotalLockedCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TotalLockedCall) -> Self {
            Self::TotalLocked(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AaveV2StablecoinCellarCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for AaveV2StablecoinCellarCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `AAVE` function with signature `AAVE()` and selector `0x48ccda3c`
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
    pub struct AaveReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `accrualPeriod` function with signature `accrualPeriod()` and selector `0xf6664155`
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
    pub struct AccrualPeriodReturn(pub u32);
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
    ///Container type for all return fields from the `assetAToken` function with signature `assetAToken()` and selector `0xc17f6740`
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
    pub struct AssetATokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `assetDecimals` function with signature `assetDecimals()` and selector `0xc2d41601`
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
    pub struct AssetDecimalsReturn(pub u8);
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
    ///Container type for all return fields from the `claimAndUnstake` function with signature `claimAndUnstake()` and selector `0xad004e20`
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
    pub struct ClaimAndUnstakeReturn {
        pub rewards: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `curveRegistryExchange` function with signature `curveRegistryExchange()` and selector `0xac353510`
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
    pub struct CurveRegistryExchangeReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `feesDistributor` function with signature `feesDistributor()` and selector `0x8e0bae7f`
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
    pub struct FeesDistributorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `gravityBridge` function with signature `gravityBridge()` and selector `0xa4da2d02`
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
    pub struct GravityBridgeReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `incentivesController` function with signature `incentivesController()` and selector `0xaf1df255`
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
    pub struct IncentivesControllerReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `lendingPool` function with signature `lendingPool()` and selector `0xa59a9973`
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
    pub struct LendingPoolReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `maxLocked` function with signature `maxLocked()` and selector `0x8fdc9dfa`
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
    pub struct MaxLockedReturn(pub ::ethers::core::types::U256);
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
    pub struct MaxMintReturn {
        pub shares: ::ethers::core::types::U256,
    }
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
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
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
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
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
    ///Container type for all return fields from the `performanceFee` function with signature `performanceFee()` and selector `0x87788782`
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
    pub struct PerformanceFeeReturn(pub u64);
    ///Container type for all return fields from the `platformFee` function with signature `platformFee()` and selector `0x26232a2e`
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
    pub struct PlatformFeeReturn(pub u64);
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
    pub struct PreviewDepositReturn(pub ::ethers::core::types::U256);
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
    pub struct PreviewRedeemReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `stkAAVE` function with signature `stkAAVE()` and selector `0x1fc29c01`
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
    pub struct StkAAVEReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `sushiswapRouter` function with signature `sushiswapRouter()` and selector `0xe9240c2d`
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
    pub struct SushiswapRouterReturn(pub ::ethers::core::types::Address);
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
    pub struct TotalAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalBalance` function with signature `totalBalance()` and selector `0xad7a672f`
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
    pub struct TotalBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalHoldings` function with signature `totalHoldings()` and selector `0xe9ec2e99`
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
    pub struct TotalHoldingsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalLocked` function with signature `totalLocked()` and selector `0x56891412`
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
    pub struct TotalLockedReturn(pub ::ethers::core::types::U256);
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
}
