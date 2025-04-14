pub use balancer_pool_adaptor_v1::*;
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
pub mod balancer_pool_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/BalancerPoolAdaptorV1.json",
        );
    };
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_vault"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_minter"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_balancerSlippage"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint32"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EXACT_TOKENS_IN_FOR_BPT_OUT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EXACT_TOKENS_IN_FOR_BPT_OUT",
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
                                    name: ::std::borrow::ToOwned::to_owned("_adaptorData"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("balancerSlippage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balancerSlippage"),
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
                    ::std::borrow::ToOwned::to_owned("claimRewards"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimRewards"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("gauge"),
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
                    ::std::borrow::ToOwned::to_owned("exitPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("exitPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetBpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapsAfterExit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.SingleSwap[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BalancerPoolAdaptor.SwapData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("request"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.ExitPoolRequest",
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
                    ::std::borrow::ToOwned::to_owned("getExpectedTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getExpectedTokens"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetBpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("expectedTokens"),
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
                    ::std::borrow::ToOwned::to_owned("joinPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("joinPool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetBpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapsBeforeJoin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IVault.SingleSwap[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct BalancerPoolAdaptor.SwapData",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minimumBpt"),
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
                    ::std::borrow::ToOwned::to_owned("makeFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("makeFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
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
                                    name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("minter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("minter"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IBalancerMinter"),
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
                    ::std::borrow::ToOwned::to_owned("stakeBPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stakeBPT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidityGauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountIn"),
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
                    ::std::borrow::ToOwned::to_owned("unstakeBPT"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unstakeBPT"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_bpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ERC20"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_liquidityGauge"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_amountOut"),
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
                    ::std::borrow::ToOwned::to_owned("vault"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vault"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IVault"),
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
                                    name: ::std::borrow::ToOwned::to_owned("_amountBPTToSend"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adaptorData"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawableFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawableFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_adaptorData"),
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
                        "BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bpt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityGauge"),
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
                        "BalancerPoolAdaptor___InternalBalancesNotSupported",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___InternalBalancesNotSupported",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BalancerPoolAdaptor___InvalidConstructorSlippage",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___InvalidConstructorSlippage",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BalancerPoolAdaptor___LengthMismatch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___LengthMismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BalancerPoolAdaptor___Slippage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___Slippage",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BalancerPoolAdaptor___UnsupportedTokenNotSwapped",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___UnsupportedTokenNotSwapped",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "BalancerPoolAdaptor___WrongSwapKind",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BalancerPoolAdaptor___WrongSwapKind",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
    pub static BALANCERPOOLADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x006?8\x03\x80b\x006?\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0\xB0V[a#(\x81c\xFF\xFF\xFF\xFF\x16\x10\x80b\0\0RWPa'\x10\x81c\xFF\xFF\xFF\xFF\x16\x11[\x15b\0\0qW`@Qc:\xA8\xBC\xF1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x80R\x91\x16`\xA0Rc\xFF\xFF\xFF\xFF\x16`\xC0Rb\0\x01\x06V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xABW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\0\xC6W`\0\x80\xFD[b\0\0\xD1\x84b\0\0\x93V[\x92Pb\0\0\xE1` \x85\x01b\0\0\x93V[\x91P`@\x84\x01Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\0\xFBW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa4\x99b\0\x01\xA6`\09`\0\x81\x81a\x02\xD1\x01R\x81\x81a\x0E\x9B\x01Ra\x1C\x89\x01R`\0\x81\x81a\x01A\x01Ra\x1Fc\x01R`\0\x81\x81a\x03\x0B\x01R\x81\x81a\x04I\x01R\x81\x81a\x07\x0F\x01R\x81\x81a\x08|\x01R\x81\x81a\tk\x01R\x81\x81a\x0BP\x01R\x81\x81a\r}\x01R\x81\x81a\r\xE3\x01R\x81\x81a\x11\t\x01R\x81\x81a\x13l\x01R\x81\x81a\x15\xBE\x01R\x81\x81a\x18\xBB\x01R\x81\x81a\x19\xD7\x01Ra\x1E.\x01Ra4\x99`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\x97\x19K\x13\x11a\0\xB8W\x80c\xD3\xBF\xE7j\x11a\0|W\x80c\xD3\xBF\xE7j\x14a\x02\x93W\x80c\xE1p\xA9\xBF\x14a\x02\xA6W\x80c\xEF\\\xFB\x8C\x14a\x02\xB9W\x80c\xF7\xE6\x9B\x16\x14a\x02\xCCW\x80c\xFAP\xE5\xD2\x14a\x02\xF3W\x80c\xFB\xFAw\xCF\x14a\x03\x06W`\0\x80\xFD[\x80c\x97\x19K\x13\x14a\x024W\x80c\xAE\xFF\xDD\xDE\x14a\x02GW\x80c\xC9\x11\x1B\xD7\x14a\x02ZW\x80c\xC9\xA6\x95b\x14a\x02mW\x80c\xD2\xE8X\x06\x14a\x02\x80W`\0\x80\xFD[\x80cxASe\x11a\0\xFFW\x80cxASe\x14a\x01\xD7W\x80cy\x06\xAF\xBF\x14a\x01\xEAW\x80cy\x98\xA1\xC4\x14a\x02\nW\x80c\x895:\t\x14a\x02\x12W\x80c\x8F!\nj\x14a\x02!W`\0\x80\xFD[\x80c\x07Tar\x14a\x01<W\x80c\x0E\".R\x14a\x01\x80W\x80c>\x03*;\x14a\x01\x96W\x80c\\\x14\xAC\xDB\x14a\x01\xAFW\x80ciD\\1\x14a\x01\xC4W[`\0\x80\xFD[a\x01c\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x88`\x01\x81V[`@Q\x90\x81R` \x01a\x01wV[a#([`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01wV[a\x01\xC2a\x01\xBD6`\x04a)\xA5V[a\x03-V[\0[a\x01\xC2a\x01\xD26`\x04a*\"V[PPPV[a\x01\x88a\x01\xE56`\x04a*\x8EV[a\x0E\xF5V[a\x01\xFDa\x01\xF86`\x04a*\xC2V[a\x10\x7FV[`@Qa\x01w\x91\x90a+#V[a\x01\x88a\x11\x86V[`@Q`\0\x81R` \x01a\x01wV[a\x01\xC2a\x02/6`\x04a+6V[a\x11\xE4V[a\x01\xC2a\x02B6`\x04a+\x90V[a\x12\xE4V[a\x01\xFDa\x02U6`\x04a*\x8EV[a\x1C\xE2V[a\x01\xC2a\x02h6`\x04a-\rV[a\x1DEV[a\x01\xC2a\x02{6`\x04a-\x8CV[a\x1E\x17V[a\x01\xC2a\x02\x8E6`\x04a+6V[a\x1E\x97V[a\x01\xC2a\x02\xA16`\x04a.DV[a\x1F\x0FV[a\x01ca\x02\xB46`\x04a*\x8EV[a\x1F(V[a\x01\xC2a\x02\xC76`\x04a*\xC2V[a\x1FDV[a\x01\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x88a\x03\x016`\x04a.}V[a\x1F\xBBV[a\x01c\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x91\x91\x90a.\xE0V[\x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF7\x91\x90a.\xF9V[\x90Pa\x04&`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81RP\x90V[`\0``\x82\x81\x01\x82\x90R`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x86\x90R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC0\x91\x90\x81\x01\x90a/qV[PP\x90Pa\x04\xCE\x89\x82a\x1F\xCDV[\x91P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\xE9Wa\x04\xE9a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83R`\0[\x81Q\x81\x10\x15a\x05yW\x81\x81\x81Q\x81\x10a\x053Wa\x053a0>V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x05QWa\x05Qa0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x05r\x81a0jV[\x90Pa\x05\x18V[P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\x93Wa\x05\x93a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xBCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x84\x01R`\0[\x81Q\x81\x10\x15a\x06\x02W`\0\x19\x84` \x01Q\x82\x81Q\x81\x10a\x05\xE7Wa\x05\xE7a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x05\xFB\x81a0jV[\x90Pa\x05\xC5V[PP\x80Q\x87Q\x14a\x06&W`@Qc\x96uu\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06AWa\x06Aa&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06jW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x88Q\x81\x10\x15a\n\x9EW\x88\x81\x81Q\x81\x10a\x06\x8BWa\x06\x8Ba0>V[` \x02` \x01\x01Q`\x80\x01Q`\0\x03\x15a\n\x8EW`\0\x89\x82\x81Q\x81\x10a\x06\xB3Wa\x06\xB3a0>V[` \x02` \x01\x01Q`@\x01Q\x90Pa\x06\xE8\x81\x8B\x84\x81Q\x81\x10a\x06\xD7Wa\x06\xD7a0>V[` \x02` \x01\x01Q`\x80\x01Qa!\xD2V[\x8A\x83\x81Q\x81\x10a\x06\xFAWa\x06\xFAa0>V[` \x02` \x01\x01Q`\x80\x01\x81\x81RPPa\x07e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x84\x81Q\x81\x10a\x07@Wa\x07@a0>V[` \x02` \x01\x01Q`\x80\x01Q\x83`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\x07\x82Wa\x07\x82a0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE7W\x83\x82\x81Q\x81\x10a\x07\xAEWa\x07\xAEa0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\x07\xD1Wa\x07\xD1a0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\x04W`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x83\x81Q\x81\x10a\x08\x18Wa\x08\x18a0>V[` \x02` \x01\x01Q` \x01Q`\x01\x81\x11\x15a\x085Wa\x085a0\x83V[\x14a\x08SW`@Qct\xC0r\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R0\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01R``\x81\x01\x82\x90R\x8BQ\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cR\xBB\xBE)\x90\x8E\x90\x87\x90\x81\x10a\x08\xBDWa\x08\xBDa0>V[` \x02` \x01\x01Q\x84\x8E`\0\x01Q\x88\x81Q\x81\x10a\x08\xDCWa\x08\xDCa0>V[` \x02` \x01\x01Q\x8F` \x01Q\x89\x81Q\x81\x10a\x08\xFAWa\x08\xFAa0>V[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t!\x94\x93\x92\x91\x90a0\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\td\x91\x90a.\xE0V[\x90Pa\t\xC1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8E\x87\x81Q\x81\x10a\t\x9DWa\t\x9Da0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80\x85\x85\x81Q\x81\x10a\t\xD4Wa\t\xD4a0>V[` \x02` \x01\x01\x81\x81RPPPPa\n\x8CV[\x83\x82\x81Q\x81\x10a\t\xF9Wa\t\xF9a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\n\x1CWa\n\x1Ca0>V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\nOW`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x82\x81Q\x81\x10a\naWa\naa0>V[` \x02` \x01\x01Q`\x80\x01Q\x83\x83\x81Q\x81\x10a\n\x7FWa\n\x7Fa0>V[` \x02` \x01\x01\x81\x81RPP[P[a\n\x97\x81a0jV[\x90Pa\x06pV[P`\x01\x81\x87`@Q` \x01a\n\xB5\x93\x92\x91\x90a1\xEDV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x84\x81\x01\x91\x90\x91RQcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B6\x91\x90a.\xE0V[`@Qc\x17+\x95\x85`\xE3\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB9\\\xAC(\x90a\x0B\x8B\x90\x88\x900\x90\x81\x90\x89\x90`\x04\x01a2\xADV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xB9W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x92P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C'\x91\x90a.\xE0V[a\x0C1\x91\x90a2\xDFV[\x90P`\0\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CNWa\x0CNa&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CwW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x89Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x95Wa\x0C\x95a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x8AQ\x81\x10\x15a\x0E\x1AW`\0\x8B\x82\x81Q\x81\x10a\x0C\xE1Wa\x0C\xE1a0>V[` \x02` \x01\x01Q`@\x01Q\x90P\x80\x83\x83\x81Q\x81\x10a\r\x02Wa\r\x02a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8B\x82\x81Q\x81\x10a\r4Wa\r4a0>V[` \x02` \x01\x01Q`\x80\x01Q\x84\x83\x81Q\x81\x10a\rRWa\rRa0>V[` \x02` \x01\x01\x81\x81RPPa\r\xA1\x83\x83\x81Q\x81\x10a\rsWa\rsa0>V[` \x02` \x01\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xD5V[`\0\x8C\x83\x81Q\x81\x10a\r\xB5Wa\r\xB5a0>V[` \x02` \x01\x01Q``\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x07Wa\x0E\x07\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xD5V[PP\x80a\x0E\x13\x90a0jV[\x90Pa\x0C\xC4V[P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c\xB33\xA1u\x83\x85\x8F`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EM\x93\x92\x91\x90a2\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8E\x91\x90a.\xE0V[\x90Pa\x0E\xC7\x81c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90a'\x10\x90a#d\x16V[\x84\x10\x15a\x0E\xE7W`@Qc6\xBE\xDB\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x0F\x0E\x91\x90a30V[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F\x91W`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FeW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x89\x91\x90a.\xE0V[\x94\x93PPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFE\x91\x90a.\xE0V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10k\x91\x90a.\xE0V[a\x10u\x91\x90a3_V[\x96\x95PPPPPPV[```\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE5\x91\x90a.\xE0V[`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11x\x91\x90\x81\x01\x90a/qV[PP\x90Pa\x0F\x89\x84\x82a\x1F\xCDV[`\0`@Q` \x01a\x11\xC9\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FBalancer Pool Adaptor V 1.0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x11\xEE\x83\x83a#\x83V[`\0a\x11\xFA\x84\x83a!\xD2V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x84\x91\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12r\x91\x90a3rV[P`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cnU?e\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xBBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPPa\x12\xDD\x85\x85a\"\xD5V[PPPPPV[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13H\x91\x90a.\xE0V[`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xDB\x91\x90\x81\x01\x90a/qV[PP\x90P`\0a\x13\xEB\x87\x83a\x1F\xCDV[\x90P\x80Q\x86Q\x14a\x14\x0FW`@Qc\x96uu\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83``\x01Q\x15a\x142W`@Qcm\"Q\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14MWa\x14Ma&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14vW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x158W\x82\x81\x81Q\x81\x10a\x14\x97Wa\x14\x97a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x0B\x91\x90a.\xE0V[\x82\x82\x81Q\x81\x10a\x15\x1DWa\x15\x1Da0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x151\x81a0jV[\x90Pa\x14|V[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA4\x91\x90a.\xE0V[`@Qc\x8B\xDB9\x13`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x8B\xDB9\x13\x90a\x15\xF9\x90\x88\x900\x90\x81\x90\x8C\x90`\x04\x01a2\xADV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16'W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x93\x91\x90a.\xE0V[a\x16\x9D\x90\x82a2\xDFV[\x90P`\0[\x83Q\x81\x10\x15a\x17\x82W\x82\x81\x81Q\x81\x10a\x16\xBDWa\x16\xBDa0>V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a\x16\xD7Wa\x16\xD7a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17K\x91\x90a.\xE0V[a\x17U\x91\x90a2\xDFV[\x83\x82\x81Q\x81\x10a\x17gWa\x17ga0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x17{\x81a0jV[\x90Pa\x16\xA2V[P`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE7\x91\x90a.\xF9V[\x90P`\0[\x84Q\x81\x10\x15a\x1C\x08W\x83\x81\x81Q\x81\x10a\x18\x07Wa\x18\x07a0>V[` \x02` \x01\x01Q`\0\x03\x15a\x1B\xF8W\x84\x81\x81Q\x81\x10a\x18)Wa\x18)a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x18LWa\x18La0>V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x7FW`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x18\x9CWa\x18\x9Ca0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1BFWa\x19&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x81Q\x81\x10a\x18\xECWa\x18\xECa0>V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x19\x06Wa\x19\x06a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x80Q`\x80\x81\x01\x82R0\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01R``\x81\x01\x91\x90\x91R\x84Q\x85\x90\x83\x90\x81\x10a\x19]Wa\x19]a0>V[` \x02` \x01\x01Q\x8B\x83\x81Q\x81\x10a\x19wWa\x19wa0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x80\x01R`\0\x8B\x83\x81Q\x81\x10a\x19\x9AWa\x19\x9Aa0>V[` \x02` \x01\x01Q` \x01Q`\x01\x81\x11\x15a\x19\xB7Wa\x19\xB7a0\x83V[\x14a\x19\xD5W`@Qct\xC0r\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cR\xBB\xBE)\x8C\x84\x81Q\x81\x10a\x1A\x16Wa\x1A\x16a0>V[` \x02` \x01\x01Q\x83\x8D`\0\x01Q\x86\x81Q\x81\x10a\x1A5Wa\x1A5a0>V[` \x02` \x01\x01Q\x8E` \x01Q\x87\x81Q\x81\x10a\x1ASWa\x1ASa0>V[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Az\x94\x93\x92\x91\x90a0\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xBD\x91\x90a.\xE0V[\x85\x83\x81Q\x81\x10a\x1A\xCFWa\x1A\xCFa0>V[` \x02` \x01\x01\x81\x81RPPa\x1A\xF0\x86\x83\x81Q\x81\x10a\rsWa\rsa0>V[\x8A\x82\x81Q\x81\x10a\x1B\x02Wa\x1B\x02a0>V[` \x02` \x01\x01Q``\x01Q\x86\x83\x81Q\x81\x10a\x1B Wa\x1B a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPPa\x1B\xF8V[\x81`\x01`\x01`\xA0\x1B\x03\x16cO\x12\x9CS\x86\x83\x81Q\x81\x10a\x1BgWa\x1Bga0>V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x9A\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xDB\x91\x90a3rV[a\x1B\xF8W`@Qc\x01\xDAlI`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x01\x81a0jV[\x90Pa\x17\xECV[P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB33\xA1u\x86\x86\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C;\x93\x92\x91\x90a2\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C|\x91\x90a.\xE0V[\x90Pa\x1C\xB5\x83c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90a'\x10\x90a#d\x16V[\x81\x10\x15a\x1C\xD5W`@Qc6\xBE\xDB\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x1D\r\x82a\x1F(V[\x81`\0\x81Q\x81\x10a\x1D Wa\x1D a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x1DN\x83a%IV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x1De\x91\x90a30V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x93P\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xD5\x91\x90a.\xE0V[\x90P\x80\x87\x11\x15a\x1D\xFAW`\0a\x1D\xEB\x82\x89a2\xDFV[\x90Pa\x1D\xF8\x84\x84\x83a\x1E\x97V[P[a\x1E\x0E`\x01`\x01`\xA0\x1B\x03\x84\x16\x87\x89a%\xE0V[PPPPPPPV[`@Qc.\x1C\"O`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\\8D\x9E\x90a\x1Ei\x900\x90\x87\x90\x87\x90\x87\x90`\x04\x01a3\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x0EW=`\0\x80>=`\0\xFD[a\x1E\xA1\x83\x83a#\x83V[\x81a\x1E\xAC\x81\x83a!\xD2V[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\x05W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x1F$`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\"SV[PPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x1F>\x91\x90a.\xF9V[\x92\x91PPV[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cjbxB\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xDDW=`\0\x80>=`\0\xFD[`\0a\x1F\xC6\x83a\x0E\xF5V[\x93\x92PPPV[\x80Q``\x90`\0\x80[\x82\x81\x10\x15a )W\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x1F\xFAWa\x1F\xFAa0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a \x19W`\x01\x91Pa )V[a \"\x81a0jV[\x90Pa\x1F\xD6V[P\x80\x15a!%Wa ;`\x01\x83a2\xDFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a RWa Ra&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a {W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x83\x81\x10\x15a!\x1EW\x86`\x01`\x01`\xA0\x1B\x03\x16\x86\x82\x81Q\x81\x10a \xA6Wa \xA6a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\x0EW\x85\x81\x81Q\x81\x10a \xCEWa \xCEa0>V[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xE8Wa \xE8a0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a!\n\x81a0jV[\x92PP[a!\x17\x81a0jV[\x90Pa \x82V[PPa!\xCAV[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a!=Wa!=a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0[\x82\x81\x10\x15a!\xC8W\x84\x81\x81Q\x81\x10a!\x86Wa!\x86a0>V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a!\xA0Wa!\xA0a0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra!\xC1\x81a0jV[\x90Pa!lV[P[PP\x92\x91PPV[`\0`\0\x19\x82\x03a\"LW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"E\x91\x90a.\xE0V[\x90Pa\x1F>V[P\x80a\x1F>V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#I\x91\x90a.\xE0V[\x11\x15a\x1F$Wa\x1F$`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\"SV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#|W`\0\x80\xFD[\x04\x92\x91PPV[`\0a#\x8Da\x11\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x83\x01R\x85\x16\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x90\x92Ra#\xC7\x92\x91`\0\x91`\x80\x01a4\x13V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$C\x91\x90a.\xF9V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$p\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xB1\x91\x90a4=V[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x19\x91\x90a3rV[a\"\xCFW`@Qc\x11C\"\xB9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R\x84\x16`$\x82\x01R`D\x01a\"\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a%\xBFWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xBF\x91\x90a3rV[\x15a%\xDDW`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\"\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%\xDDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xA5Wa&\xA5a&mV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xA5Wa&\xA5a&mV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xF5Wa&\xF5a&mV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a'\x16Wa'\x16a&mV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a'/W`\0\x80\xFD[\x91\x90PV[\x805a'/\x81a&XV[`\0\x82`\x1F\x83\x01\x12a'PW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'iWa'ia&mV[a'|`\x1F\x82\x01`\x1F\x19\x16` \x01a&\xCDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a'\x91W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xBFW`\0\x80\xFD[\x815` a'\xD4a'\xCF\x83a&\xFDV[a&\xCDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a'\xF3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\x17W`\0\x80\x81\xFD[\x90\x88\x01\x90`\xC0\x82\x8B\x03`\x1F\x19\x01\x81\x13\x15a(1W`\0\x80\x81\xFD[a(9a&\x83V[\x87\x84\x015\x81R`@a(L\x81\x86\x01a' V[\x89\x83\x01R``a(]\x81\x87\x01a'4V[\x82\x84\x01R`\x80\x91Pa(p\x82\x87\x01a'4V[\x90\x83\x01R`\xA0\x85\x81\x015\x82\x84\x01R\x92\x85\x015\x92\x84\x84\x11\x15a(\x93W`\0\x91P\x81\x82\xFD[a(\xA1\x8E\x8B\x86\x89\x01\x01a'?V[\x90\x83\x01RP\x86RPPP\x91\x83\x01\x91\x83\x01a'\xF7V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a(\xD2W`\0\x80\xFD[\x815` a(\xE2a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)\x01W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x805\x83R\x91\x83\x01\x91\x83\x01a)\x05V[`\0`@\x82\x84\x03\x12\x15a).W`\0\x80\xFD[`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a)QWa)Qa&mV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15a)iW`\0\x80\xFD[a)u\x86\x83\x87\x01a(\xC1V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a)\x8BW`\0\x80\xFD[Pa)\x98\x85\x82\x86\x01a(\xC1V[` \x83\x01RPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)\xBBW`\0\x80\xFD[\x845a)\xC6\x81a&XV[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\xE2W`\0\x80\xFD[a)\xEE\x88\x83\x89\x01a'\xAEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a*\x04W`\0\x80\xFD[Pa*\x11\x87\x82\x88\x01a)\x1CV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a*7W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*UW`\0\x80\xFD[a*a\x87\x83\x88\x01a'?V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a*wW`\0\x80\xFD[Pa*\x84\x86\x82\x87\x01a'?V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a*\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB6W`\0\x80\xFD[a\x0F\x89\x84\x82\x85\x01a'?V[`\0` \x82\x84\x03\x12\x15a*\xD4W`\0\x80\xFD[\x815a\x1F\xC6\x81a&XV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a+\x18W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a*\xF3V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x1F\xC6` \x83\x01\x84a*\xDFV[`\0\x80`\0``\x84\x86\x03\x12\x15a+KW`\0\x80\xFD[\x835a+V\x81a&XV[\x92P` \x84\x015a+f\x81a&XV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x80\x15\x15\x81\x14a%\xDDW`\0\x80\xFD[\x805a'/\x81a+wV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xA6W`\0\x80\xFD[\x845a+\xB1\x81a&XV[\x93P` \x85\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a+\xCEW`\0\x80\xFD[a+\xDA\x89\x83\x8A\x01a'\xAEV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a+\xF0W`\0\x80\xFD[a+\xFC\x89\x83\x8A\x01a)\x1CV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a,\x12W`\0\x80\xFD[\x90\x87\x01\x90`\x80\x82\x8A\x03\x12\x15a,&W`\0\x80\xFD[a,.a&\xABV[\x825\x82\x81\x11\x15a,=W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x8B\x13a,NW`\0\x80\xFD[\x805a,\\a'\xCF\x82a&\xFDV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x86\x01\x90\x86\x81\x01\x90\x8D\x83\x11\x15a,{W`\0\x80\xFD[\x92\x87\x01\x92[\x82\x84\x10\x15a,\xA2W\x835a,\x93\x81a&XV[\x82R\x92\x87\x01\x92\x90\x87\x01\x90a,\x80V[\x84RPPP\x82\x84\x015\x82\x81\x11\x15a,\xB8W`\0\x80\xFD[a,\xC4\x8B\x82\x86\x01a(\xC1V[\x85\x83\x01RP`@\x83\x015\x93P\x81\x84\x11\x15a,\xDDW`\0\x80\xFD[a,\xE9\x8A\x85\x85\x01a'?V[`@\x82\x01Ra,\xFA``\x84\x01a+\x85V[``\x82\x01R\x96\x99\x95\x98P\x93\x96PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a-#W`\0\x80\xFD[\x845\x93P` \x85\x015a-5\x81a&XV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-QW`\0\x80\xFD[a-]\x88\x83\x89\x01a'?V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a-sW`\0\x80\xFD[Pa-\x80\x87\x82\x88\x01a'?V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a-\xA1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\xB8W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a-\xCCW`\0\x80\xFD[\x815` a-\xDCa'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a-\xFBW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a.\"W\x855a.\x13\x81a&XV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a.\0V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a.8W`\0\x80\xFD[a*a\x87\x83\x88\x01a(\xC1V[`\0\x80`@\x83\x85\x03\x12\x15a.WW`\0\x80\xFD[\x825a.b\x81a&XV[\x91P` \x83\x015a.r\x81a&XV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\x90W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\xA7W`\0\x80\xFD[a.\xB3\x86\x83\x87\x01a'?V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a.\xC9W`\0\x80\xFD[Pa.\xD6\x85\x82\x86\x01a'?V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a.\xF2W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a/\x0BW`\0\x80\xFD[\x81Qa\x1F\xC6\x81a&XV[`\0\x82`\x1F\x83\x01\x12a/'W`\0\x80\xFD[\x81Q` a/7a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/VW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x80Q\x83R\x91\x83\x01\x91\x83\x01a/ZV[`\0\x80`\0``\x84\x86\x03\x12\x15a/\x86W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a/\x9DW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a/\xB1W`\0\x80\xFD[\x81Q` a/\xC1a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a/\xE0W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a0\x07W\x85Qa/\xF8\x81a&XV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a/\xE5V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a0 W`\0\x80\xFD[Pa0-\x86\x82\x87\x01a/\x16V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a0|Wa0|a0TV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a0\xBFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a0\xA3V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\xE0\x81R\x84Q`\xE0\x82\x01R`\0` \x86\x01Q`\x02\x81\x10a1\x0FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\x01\0\x83\x01R`@\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x83\x01R``\x86\x01Qa1Da\x01@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x86\x01Qa\x01`\x83\x01R`\xA0\x86\x01Q`\xC0a\x01\x80\x84\x01Ra1ka\x01\xA0\x84\x01\x82a0\x99V[\x91PPa1\xAB` \x83\x01\x86\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[`\xA0\x82\x01\x93\x90\x93R`\xC0\x01R\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a+\x18W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a1\xD1V[\x83\x81R``` \x82\x01R`\0a2\x06``\x83\x01\x85a1\xBDV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[\x80Q`\x80\x80\x84R\x81Q\x90\x84\x01\x81\x90R`\0\x91` \x91\x90\x82\x01\x90`\xA0\x86\x01\x90\x84[\x81\x81\x10\x15a2[W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a26V[PP\x82\x85\x01Q\x91P\x85\x81\x03\x83\x87\x01Ra2t\x81\x83a1\xBDV[\x92PPP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra2\x8F\x82\x82a0\x99V[\x91PP``\x83\x01Qa2\xA5``\x86\x01\x82\x15\x15\x90RV[P\x93\x92PPPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80``\x82\x01\x81\x90R`\0\x90a\x10u\x90\x83\x01\x84a2\x16V[\x81\x81\x03\x81\x81\x11\x15a\x1F>Wa\x1F>a0TV[``\x81R`\0a3\x05``\x83\x01\x86a*\xDFV[\x82\x81\x03` \x84\x01Ra3\x17\x81\x86a1\xBDV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3CW`\0\x80\xFD[\x82Qa3N\x81a&XV[` \x84\x01Q\x90\x92Pa.r\x81a&XV[\x80\x82\x01\x80\x82\x11\x15a\x1F>Wa\x1F>a0TV[`\0` \x82\x84\x03\x12\x15a3\x84W`\0\x80\xFD[\x81Qa\x1F\xC6\x81a+wV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a3\xDDW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a3\xBFV[PP\x85\x81\x03`@\x87\x01Ra3\xF1\x81\x89a1\xBDV[\x93PPPP\x82\x81\x03``\x84\x01Ra4\x08\x81\x85a0\x99V[\x97\x96PPPPPPPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a44``\x83\x01\x84a0\x99V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a4OW`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\xC6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE2\xD4\xD4\xF4\x05\xE2\x13\xA0\xADS.*\xBDF\xC6\xE7\tF\xED\xB7\xC43r-W\xDE\xA2\x9B\xA5\xCC#\xDEdsolcC\0\x08\x15\x003";
    /// The bytecode of the contract.
    pub static BALANCERPOOLADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x017W`\x005`\xE0\x1C\x80c\x97\x19K\x13\x11a\0\xB8W\x80c\xD3\xBF\xE7j\x11a\0|W\x80c\xD3\xBF\xE7j\x14a\x02\x93W\x80c\xE1p\xA9\xBF\x14a\x02\xA6W\x80c\xEF\\\xFB\x8C\x14a\x02\xB9W\x80c\xF7\xE6\x9B\x16\x14a\x02\xCCW\x80c\xFAP\xE5\xD2\x14a\x02\xF3W\x80c\xFB\xFAw\xCF\x14a\x03\x06W`\0\x80\xFD[\x80c\x97\x19K\x13\x14a\x024W\x80c\xAE\xFF\xDD\xDE\x14a\x02GW\x80c\xC9\x11\x1B\xD7\x14a\x02ZW\x80c\xC9\xA6\x95b\x14a\x02mW\x80c\xD2\xE8X\x06\x14a\x02\x80W`\0\x80\xFD[\x80cxASe\x11a\0\xFFW\x80cxASe\x14a\x01\xD7W\x80cy\x06\xAF\xBF\x14a\x01\xEAW\x80cy\x98\xA1\xC4\x14a\x02\nW\x80c\x895:\t\x14a\x02\x12W\x80c\x8F!\nj\x14a\x02!W`\0\x80\xFD[\x80c\x07Tar\x14a\x01<W\x80c\x0E\".R\x14a\x01\x80W\x80c>\x03*;\x14a\x01\x96W\x80c\\\x14\xAC\xDB\x14a\x01\xAFW\x80ciD\\1\x14a\x01\xC4W[`\0\x80\xFD[a\x01c\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x88`\x01\x81V[`@Q\x90\x81R` \x01a\x01wV[a#([`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01wV[a\x01\xC2a\x01\xBD6`\x04a)\xA5V[a\x03-V[\0[a\x01\xC2a\x01\xD26`\x04a*\"V[PPPV[a\x01\x88a\x01\xE56`\x04a*\x8EV[a\x0E\xF5V[a\x01\xFDa\x01\xF86`\x04a*\xC2V[a\x10\x7FV[`@Qa\x01w\x91\x90a+#V[a\x01\x88a\x11\x86V[`@Q`\0\x81R` \x01a\x01wV[a\x01\xC2a\x02/6`\x04a+6V[a\x11\xE4V[a\x01\xC2a\x02B6`\x04a+\x90V[a\x12\xE4V[a\x01\xFDa\x02U6`\x04a*\x8EV[a\x1C\xE2V[a\x01\xC2a\x02h6`\x04a-\rV[a\x1DEV[a\x01\xC2a\x02{6`\x04a-\x8CV[a\x1E\x17V[a\x01\xC2a\x02\x8E6`\x04a+6V[a\x1E\x97V[a\x01\xC2a\x02\xA16`\x04a.DV[a\x1F\x0FV[a\x01ca\x02\xB46`\x04a*\x8EV[a\x1F(V[a\x01\xC2a\x02\xC76`\x04a*\xC2V[a\x1FDV[a\x01\x9A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\x88a\x03\x016`\x04a.}V[a\x1F\xBBV[a\x01c\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03mW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x91\x91\x90a.\xE0V[\x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xF7\x91\x90a.\xF9V[\x90Pa\x04&`@Q\x80`\x80\x01`@R\x80``\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x15\x15\x81RP\x90V[`\0``\x82\x81\x01\x82\x90R`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x86\x90R\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xC0\x91\x90\x81\x01\x90a/qV[PP\x90Pa\x04\xCE\x89\x82a\x1F\xCDV[\x91P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04\xE9Wa\x04\xE9a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x12W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83R`\0[\x81Q\x81\x10\x15a\x05yW\x81\x81\x81Q\x81\x10a\x053Wa\x053a0>V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x05QWa\x05Qa0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra\x05r\x81a0jV[\x90Pa\x05\x18V[P\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05\x93Wa\x05\x93a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xBCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x84\x01R`\0[\x81Q\x81\x10\x15a\x06\x02W`\0\x19\x84` \x01Q\x82\x81Q\x81\x10a\x05\xE7Wa\x05\xE7a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x05\xFB\x81a0jV[\x90Pa\x05\xC5V[PP\x80Q\x87Q\x14a\x06&W`@Qc\x96uu\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06AWa\x06Aa&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06jW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x88Q\x81\x10\x15a\n\x9EW\x88\x81\x81Q\x81\x10a\x06\x8BWa\x06\x8Ba0>V[` \x02` \x01\x01Q`\x80\x01Q`\0\x03\x15a\n\x8EW`\0\x89\x82\x81Q\x81\x10a\x06\xB3Wa\x06\xB3a0>V[` \x02` \x01\x01Q`@\x01Q\x90Pa\x06\xE8\x81\x8B\x84\x81Q\x81\x10a\x06\xD7Wa\x06\xD7a0>V[` \x02` \x01\x01Q`\x80\x01Qa!\xD2V[\x8A\x83\x81Q\x81\x10a\x06\xFAWa\x06\xFAa0>V[` \x02` \x01\x01Q`\x80\x01\x81\x81RPPa\x07e\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8B\x84\x81Q\x81\x10a\x07@Wa\x07@a0>V[` \x02` \x01\x01Q`\x80\x01Q\x83`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\x07\x82Wa\x07\x82a0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xE7W\x83\x82\x81Q\x81\x10a\x07\xAEWa\x07\xAEa0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\x07\xD1Wa\x07\xD1a0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\x04W`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x83\x81Q\x81\x10a\x08\x18Wa\x08\x18a0>V[` \x02` \x01\x01Q` \x01Q`\x01\x81\x11\x15a\x085Wa\x085a0\x83V[\x14a\x08SW`@Qct\xC0r\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x80\x81\x01\x82R0\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01R``\x81\x01\x82\x90R\x8BQ\x90\x91\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cR\xBB\xBE)\x90\x8E\x90\x87\x90\x81\x10a\x08\xBDWa\x08\xBDa0>V[` \x02` \x01\x01Q\x84\x8E`\0\x01Q\x88\x81Q\x81\x10a\x08\xDCWa\x08\xDCa0>V[` \x02` \x01\x01Q\x8F` \x01Q\x89\x81Q\x81\x10a\x08\xFAWa\x08\xFAa0>V[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t!\x94\x93\x92\x91\x90a0\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\t@W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\td\x91\x90a.\xE0V[\x90Pa\t\xC1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8E\x87\x81Q\x81\x10a\t\x9DWa\t\x9Da0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80\x85\x85\x81Q\x81\x10a\t\xD4Wa\t\xD4a0>V[` \x02` \x01\x01\x81\x81RPPPPa\n\x8CV[\x83\x82\x81Q\x81\x10a\t\xF9Wa\t\xF9a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x83\x81Q\x81\x10a\n\x1CWa\n\x1Ca0>V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\nOW`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x89\x82\x81Q\x81\x10a\naWa\naa0>V[` \x02` \x01\x01Q`\x80\x01Q\x83\x83\x81Q\x81\x10a\n\x7FWa\n\x7Fa0>V[` \x02` \x01\x01\x81\x81RPP[P[a\n\x97\x81a0jV[\x90Pa\x06pV[P`\x01\x81\x87`@Q` \x01a\n\xB5\x93\x92\x91\x90a1\xEDV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x81R\x84\x81\x01\x91\x90\x91RQcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91P`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B6\x91\x90a.\xE0V[`@Qc\x17+\x95\x85`\xE3\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xB9\\\xAC(\x90a\x0B\x8B\x90\x88\x900\x90\x81\x90\x89\x90`\x04\x01a2\xADV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xB9W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x92P`\x01`\x01`\xA0\x1B\x03\x8C\x16\x91Pcp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C'\x91\x90a.\xE0V[a\x0C1\x91\x90a2\xDFV[\x90P`\0\x88Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CNWa\x0CNa&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CwW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x89Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x95Wa\x0C\x95a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xBEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x8AQ\x81\x10\x15a\x0E\x1AW`\0\x8B\x82\x81Q\x81\x10a\x0C\xE1Wa\x0C\xE1a0>V[` \x02` \x01\x01Q`@\x01Q\x90P\x80\x83\x83\x81Q\x81\x10a\r\x02Wa\r\x02a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8B\x82\x81Q\x81\x10a\r4Wa\r4a0>V[` \x02` \x01\x01Q`\x80\x01Q\x84\x83\x81Q\x81\x10a\rRWa\rRa0>V[` \x02` \x01\x01\x81\x81RPPa\r\xA1\x83\x83\x81Q\x81\x10a\rsWa\rsa0>V[` \x02` \x01\x01Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xD5V[`\0\x8C\x83\x81Q\x81\x10a\r\xB5Wa\r\xB5a0>V[` \x02` \x01\x01Q``\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x07Wa\x0E\x07\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\"\xD5V[PP\x80a\x0E\x13\x90a0jV[\x90Pa\x0C\xC4V[P`\0\x86`\x01`\x01`\xA0\x1B\x03\x16c\xB33\xA1u\x83\x85\x8F`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EM\x93\x92\x91\x90a2\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EjW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8E\x91\x90a.\xE0V[\x90Pa\x0E\xC7\x81c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90a'\x10\x90a#d\x16V[\x84\x10\x15a\x0E\xE7W`@Qc6\xBE\xDB\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPPV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x0F\x0E\x91\x90a30V[\x90\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0F\x91W`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FeW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x89\x91\x90a.\xE0V[\x94\x93PPPPV[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x81\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFE\x91\x90a.\xE0V[`@Qcp\xA0\x821`\xE0\x1B\x81R3`\x04\x82\x01R\x90\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10GW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10k\x91\x90a.\xE0V[a\x10u\x91\x90a3_V[\x96\x95PPPPPPV[```\0\x82`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xE5\x91\x90a.\xE0V[`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11PW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11x\x91\x90\x81\x01\x90a/qV[PP\x90Pa\x0F\x89\x84\x82a\x1F\xCDV[`\0`@Q` \x01a\x11\xC9\x90` \x80\x82R`\x1B\x90\x82\x01R\x7FBalancer Pool Adaptor V 1.0\0\0\0\0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x11\xEE\x83\x83a#\x83V[`\0a\x11\xFA\x84\x83a!\xD2V[`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x84\x91\x86\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12r\x91\x90a3rV[P`@QcnU?e`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R0`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cnU?e\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xBBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPPa\x12\xDD\x85\x85a\"\xD5V[PPPPPV[`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c8\xFF\xF2\xD0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13H\x91\x90a.\xE0V[`@Qc\x1F)\xA8\xCD`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF9MFh\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xDB\x91\x90\x81\x01\x90a/qV[PP\x90P`\0a\x13\xEB\x87\x83a\x1F\xCDV[\x90P\x80Q\x86Q\x14a\x14\x0FW`@Qc\x96uu\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83``\x01Q\x15a\x142W`@Qcm\"Q\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14MWa\x14Ma&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14vW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82Q\x81\x10\x15a\x158W\x82\x81\x81Q\x81\x10a\x14\x97Wa\x14\x97a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x0B\x91\x90a.\xE0V[\x82\x82\x81Q\x81\x10a\x15\x1DWa\x15\x1Da0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x151\x81a0jV[\x90Pa\x14|V[P`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\x80W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xA4\x91\x90a.\xE0V[`@Qc\x8B\xDB9\x13`\xE0\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x8B\xDB9\x13\x90a\x15\xF9\x90\x88\x900\x90\x81\x90\x8C\x90`\x04\x01a2\xADV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x16\x13W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x16'W=`\0\x80>=`\0\xFD[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8C\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x93\x91\x90a.\xE0V[a\x16\x9D\x90\x82a2\xDFV[\x90P`\0[\x83Q\x81\x10\x15a\x17\x82W\x82\x81\x81Q\x81\x10a\x16\xBDWa\x16\xBDa0>V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a\x16\xD7Wa\x16\xD7a0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17K\x91\x90a.\xE0V[a\x17U\x91\x90a2\xDFV[\x83\x82\x81Q\x81\x10a\x17gWa\x17ga0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Ra\x17{\x81a0jV[\x90Pa\x16\xA2V[P`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xE7\x91\x90a.\xF9V[\x90P`\0[\x84Q\x81\x10\x15a\x1C\x08W\x83\x81\x81Q\x81\x10a\x18\x07Wa\x18\x07a0>V[` \x02` \x01\x01Q`\0\x03\x15a\x1B\xF8W\x84\x81\x81Q\x81\x10a\x18)Wa\x18)a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x18LWa\x18La0>V[` \x02` \x01\x01Q`@\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x7FW`@Qc\x85\xF4\xBD;`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`\x01`\x01`\xA0\x1B\x03\x16\x8A\x82\x81Q\x81\x10a\x18\x9CWa\x18\x9Ca0>V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1BFWa\x19&\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x83\x81Q\x81\x10a\x18\xECWa\x18\xECa0>V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x19\x06Wa\x19\x06a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\"S\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@\x80Q`\x80\x81\x01\x82R0\x80\x82R`\0` \x83\x01\x81\x90R\x92\x82\x01R``\x81\x01\x91\x90\x91R\x84Q\x85\x90\x83\x90\x81\x10a\x19]Wa\x19]a0>V[` \x02` \x01\x01Q\x8B\x83\x81Q\x81\x10a\x19wWa\x19wa0>V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x80\x01R`\0\x8B\x83\x81Q\x81\x10a\x19\x9AWa\x19\x9Aa0>V[` \x02` \x01\x01Q` \x01Q`\x01\x81\x11\x15a\x19\xB7Wa\x19\xB7a0\x83V[\x14a\x19\xD5W`@Qct\xC0r\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cR\xBB\xBE)\x8C\x84\x81Q\x81\x10a\x1A\x16Wa\x1A\x16a0>V[` \x02` \x01\x01Q\x83\x8D`\0\x01Q\x86\x81Q\x81\x10a\x1A5Wa\x1A5a0>V[` \x02` \x01\x01Q\x8E` \x01Q\x87\x81Q\x81\x10a\x1ASWa\x1ASa0>V[` \x02` \x01\x01Q`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1Az\x94\x93\x92\x91\x90a0\xDFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x1A\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xBD\x91\x90a.\xE0V[\x85\x83\x81Q\x81\x10a\x1A\xCFWa\x1A\xCFa0>V[` \x02` \x01\x01\x81\x81RPPa\x1A\xF0\x86\x83\x81Q\x81\x10a\rsWa\rsa0>V[\x8A\x82\x81Q\x81\x10a\x1B\x02Wa\x1B\x02a0>V[` \x02` \x01\x01Q``\x01Q\x86\x83\x81Q\x81\x10a\x1B Wa\x1B a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPPa\x1B\xF8V[\x81`\x01`\x01`\xA0\x1B\x03\x16cO\x12\x9CS\x86\x83\x81Q\x81\x10a\x1BgWa\x1Bga0>V[` \x02` \x01\x01Q`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1B\x9A\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xDB\x91\x90a3rV[a\x1B\xF8W`@Qc\x01\xDAlI`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x01\x81a0jV[\x90Pa\x17\xECV[P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\xB33\xA1u\x86\x86\x8E`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C;\x93\x92\x91\x90a2\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C|\x91\x90a.\xE0V[\x90Pa\x1C\xB5\x83c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90a'\x10\x90a#d\x16V[\x81\x10\x15a\x1C\xD5W`@Qc6\xBE\xDB\xD9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x1D\r\x82a\x1F(V[\x81`\0\x81Q\x81\x10a\x1D Wa\x1D a0>V[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[a\x1DN\x83a%IV[`\0\x80\x83\x80` \x01\x90Q\x81\x01\x90a\x1De\x91\x90a30V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91\x93P\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xD5\x91\x90a.\xE0V[\x90P\x80\x87\x11\x15a\x1D\xFAW`\0a\x1D\xEB\x82\x89a2\xDFV[\x90Pa\x1D\xF8\x84\x84\x83a\x1E\x97V[P[a\x1E\x0E`\x01`\x01`\xA0\x1B\x03\x84\x16\x87\x89a%\xE0V[PPPPPPPV[`@Qc.\x1C\"O`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\\8D\x9E\x90a\x1Ei\x900\x90\x87\x90\x87\x90\x87\x90`\x04\x01a3\x8FV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\x83W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x0EW=`\0\x80>=`\0\xFD[a\x1E\xA1\x83\x83a#\x83V[\x81a\x1E\xAC\x81\x83a!\xD2V[`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E\xF1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1F\x05W=`\0\x80>=`\0\xFD[PPPPPPPPV[a\x1F$`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\"SV[PPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x1F>\x91\x90a.\xF9V[\x92\x91PPV[`@Qc51<!`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cjbxB\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xDDW=`\0\x80>=`\0\xFD[`\0a\x1F\xC6\x83a\x0E\xF5V[\x93\x92PPPV[\x80Q``\x90`\0\x80[\x82\x81\x10\x15a )W\x85`\x01`\x01`\xA0\x1B\x03\x16\x85\x82\x81Q\x81\x10a\x1F\xFAWa\x1F\xFAa0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a \x19W`\x01\x91Pa )V[a \"\x81a0jV[\x90Pa\x1F\xD6V[P\x80\x15a!%Wa ;`\x01\x83a2\xDFV[`\x01`\x01`@\x1B\x03\x81\x11\x15a RWa Ra&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a {W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80[\x83\x81\x10\x15a!\x1EW\x86`\x01`\x01`\xA0\x1B\x03\x16\x86\x82\x81Q\x81\x10a \xA6Wa \xA6a0>V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a!\x0EW\x85\x81\x81Q\x81\x10a \xCEWa \xCEa0>V[` \x02` \x01\x01Q\x85\x83\x81Q\x81\x10a \xE8Wa \xE8a0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a!\n\x81a0jV[\x92PP[a!\x17\x81a0jV[\x90Pa \x82V[PPa!\xCAV[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a!=Wa!=a&mV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0[\x82\x81\x10\x15a!\xC8W\x84\x81\x81Q\x81\x10a!\x86Wa!\x86a0>V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a!\xA0Wa!\xA0a0>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01Ra!\xC1\x81a0jV[\x90Pa!lV[P[PP\x92\x91PPV[`\0`\0\x19\x82\x03a\"LW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"!W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"E\x91\x90a.\xE0V[\x90Pa\x1F>V[P\x80a\x1F>V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PPPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#%W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#I\x91\x90a.\xE0V[\x11\x15a\x1F$Wa\x1F$`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\"SV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a#|W`\0\x80\xFD[\x04\x92\x91PPV[`\0a#\x8Da\x11\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x83\x01R\x85\x16\x81\x83\x01R\x81Q\x80\x82\x03\x83\x01\x81R``\x82\x01\x90\x92Ra#\xC7\x92\x91`\0\x91`\x80\x01a4\x13V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\x000`\x01`\x01`\xA0\x1B\x03\x16c{\x109\x99`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$C\x91\x90a.\xF9V[`\x01`\x01`\xA0\x1B\x03\x16c\t\xF7\xAB\xD2\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$p\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\xB1\x91\x90a4=V[`@Qc\x02N\xEF\xAB`\xE6\x1B\x81Rc\xFF\xFF\xFF\xFF\x82\x16`\x04\x82\x01R\x90\x91P0\x90c\x93\xBB\xEA\xC0\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x19\x91\x90a3rV[a\"\xCFW`@Qc\x11C\"\xB9`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R\x84\x16`$\x82\x01R`D\x01a\"\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x80\x15\x90a%\xBFWP0`\x01`\x01`\xA0\x1B\x03\x16cLF\x02\xDA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\x9BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xBF\x91\x90a3rV[\x15a%\xDDW`@Qc\x07\xDE\x9BQ`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\"\xCFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\"\xC6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%\xDDW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xC0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xA5Wa&\xA5a&mV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xA5Wa&\xA5a&mV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a&\xF5Wa&\xF5a&mV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a'\x16Wa'\x16a&mV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a'/W`\0\x80\xFD[\x91\x90PV[\x805a'/\x81a&XV[`\0\x82`\x1F\x83\x01\x12a'PW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'iWa'ia&mV[a'|`\x1F\x82\x01`\x1F\x19\x16` \x01a&\xCDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a'\x91W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a'\xBFW`\0\x80\xFD[\x815` a'\xD4a'\xCF\x83a&\xFDV[a&\xCDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a'\xF3W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x805`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a(\x17W`\0\x80\x81\xFD[\x90\x88\x01\x90`\xC0\x82\x8B\x03`\x1F\x19\x01\x81\x13\x15a(1W`\0\x80\x81\xFD[a(9a&\x83V[\x87\x84\x015\x81R`@a(L\x81\x86\x01a' V[\x89\x83\x01R``a(]\x81\x87\x01a'4V[\x82\x84\x01R`\x80\x91Pa(p\x82\x87\x01a'4V[\x90\x83\x01R`\xA0\x85\x81\x015\x82\x84\x01R\x92\x85\x015\x92\x84\x84\x11\x15a(\x93W`\0\x91P\x81\x82\xFD[a(\xA1\x8E\x8B\x86\x89\x01\x01a'?V[\x90\x83\x01RP\x86RPPP\x91\x83\x01\x91\x83\x01a'\xF7V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a(\xD2W`\0\x80\xFD[\x815` a(\xE2a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a)\x01W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x805\x83R\x91\x83\x01\x91\x83\x01a)\x05V[`\0`@\x82\x84\x03\x12\x15a).W`\0\x80\xFD[`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a)QWa)Qa&mV[\x81`@R\x82\x93P\x845\x91P\x80\x82\x11\x15a)iW`\0\x80\xFD[a)u\x86\x83\x87\x01a(\xC1V[\x83R` \x85\x015\x91P\x80\x82\x11\x15a)\x8BW`\0\x80\xFD[Pa)\x98\x85\x82\x86\x01a(\xC1V[` \x83\x01RPP\x92\x91PPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a)\xBBW`\0\x80\xFD[\x845a)\xC6\x81a&XV[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a)\xE2W`\0\x80\xFD[a)\xEE\x88\x83\x89\x01a'\xAEV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a*\x04W`\0\x80\xFD[Pa*\x11\x87\x82\x88\x01a)\x1CV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a*7W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*UW`\0\x80\xFD[a*a\x87\x83\x88\x01a'?V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a*wW`\0\x80\xFD[Pa*\x84\x86\x82\x87\x01a'?V[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a*\xA0W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xB6W`\0\x80\xFD[a\x0F\x89\x84\x82\x85\x01a'?V[`\0` \x82\x84\x03\x12\x15a*\xD4W`\0\x80\xFD[\x815a\x1F\xC6\x81a&XV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a+\x18W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a*\xF3V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x1F\xC6` \x83\x01\x84a*\xDFV[`\0\x80`\0``\x84\x86\x03\x12\x15a+KW`\0\x80\xFD[\x835a+V\x81a&XV[\x92P` \x84\x015a+f\x81a&XV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x80\x15\x15\x81\x14a%\xDDW`\0\x80\xFD[\x805a'/\x81a+wV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a+\xA6W`\0\x80\xFD[\x845a+\xB1\x81a&XV[\x93P` \x85\x81\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a+\xCEW`\0\x80\xFD[a+\xDA\x89\x83\x8A\x01a'\xAEV[\x95P`@\x88\x015\x91P\x80\x82\x11\x15a+\xF0W`\0\x80\xFD[a+\xFC\x89\x83\x8A\x01a)\x1CV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a,\x12W`\0\x80\xFD[\x90\x87\x01\x90`\x80\x82\x8A\x03\x12\x15a,&W`\0\x80\xFD[a,.a&\xABV[\x825\x82\x81\x11\x15a,=W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x8B\x13a,NW`\0\x80\xFD[\x805a,\\a'\xCF\x82a&\xFDV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x86\x01\x90\x86\x81\x01\x90\x8D\x83\x11\x15a,{W`\0\x80\xFD[\x92\x87\x01\x92[\x82\x84\x10\x15a,\xA2W\x835a,\x93\x81a&XV[\x82R\x92\x87\x01\x92\x90\x87\x01\x90a,\x80V[\x84RPPP\x82\x84\x015\x82\x81\x11\x15a,\xB8W`\0\x80\xFD[a,\xC4\x8B\x82\x86\x01a(\xC1V[\x85\x83\x01RP`@\x83\x015\x93P\x81\x84\x11\x15a,\xDDW`\0\x80\xFD[a,\xE9\x8A\x85\x85\x01a'?V[`@\x82\x01Ra,\xFA``\x84\x01a+\x85V[``\x82\x01R\x96\x99\x95\x98P\x93\x96PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a-#W`\0\x80\xFD[\x845\x93P` \x85\x015a-5\x81a&XV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-QW`\0\x80\xFD[a-]\x88\x83\x89\x01a'?V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a-sW`\0\x80\xFD[Pa-\x80\x87\x82\x88\x01a'?V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a-\xA1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-\xB8W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a-\xCCW`\0\x80\xFD[\x815` a-\xDCa'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a-\xFBW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a.\"W\x855a.\x13\x81a&XV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a.\0V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a.8W`\0\x80\xFD[a*a\x87\x83\x88\x01a(\xC1V[`\0\x80`@\x83\x85\x03\x12\x15a.WW`\0\x80\xFD[\x825a.b\x81a&XV[\x91P` \x83\x015a.r\x81a&XV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a.\x90W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\xA7W`\0\x80\xFD[a.\xB3\x86\x83\x87\x01a'?V[\x93P` \x85\x015\x91P\x80\x82\x11\x15a.\xC9W`\0\x80\xFD[Pa.\xD6\x85\x82\x86\x01a'?V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a.\xF2W`\0\x80\xFD[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15a/\x0BW`\0\x80\xFD[\x81Qa\x1F\xC6\x81a&XV[`\0\x82`\x1F\x83\x01\x12a/'W`\0\x80\xFD[\x81Q` a/7a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a/VW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a(\xB6W\x80Q\x83R\x91\x83\x01\x91\x83\x01a/ZV[`\0\x80`\0``\x84\x86\x03\x12\x15a/\x86W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a/\x9DW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a/\xB1W`\0\x80\xFD[\x81Q` a/\xC1a'\xCF\x83a&\xFDV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a/\xE0W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a0\x07W\x85Qa/\xF8\x81a&XV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a/\xE5V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a0 W`\0\x80\xFD[Pa0-\x86\x82\x87\x01a/\x16V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\x01\x82\x01a0|Wa0|a0TV[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a0\xBFW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a0\xA3V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\xE0\x81R\x84Q`\xE0\x82\x01R`\0` \x86\x01Q`\x02\x81\x10a1\x0FWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\x01\0\x83\x01R`@\x86\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x83\x01R``\x86\x01Qa1Da\x01@\x84\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\x80\x86\x01Qa\x01`\x83\x01R`\xA0\x86\x01Q`\xC0a\x01\x80\x84\x01Ra1ka\x01\xA0\x84\x01\x82a0\x99V[\x91PPa1\xAB` \x83\x01\x86\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x15\x15\x90\x84\x01R`@\x80\x83\x01Q\x90\x91\x16\x90\x83\x01R``\x90\x81\x01Q\x15\x15\x91\x01RV[`\xA0\x82\x01\x93\x90\x93R`\xC0\x01R\x92\x91PPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a+\x18W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a1\xD1V[\x83\x81R``` \x82\x01R`\0a2\x06``\x83\x01\x85a1\xBDV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[\x80Q`\x80\x80\x84R\x81Q\x90\x84\x01\x81\x90R`\0\x91` \x91\x90\x82\x01\x90`\xA0\x86\x01\x90\x84[\x81\x81\x10\x15a2[W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a26V[PP\x82\x85\x01Q\x91P\x85\x81\x03\x83\x87\x01Ra2t\x81\x83a1\xBDV[\x92PPP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra2\x8F\x82\x82a0\x99V[\x91PP``\x83\x01Qa2\xA5``\x86\x01\x82\x15\x15\x90RV[P\x93\x92PPPV[\x84\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16` \x83\x01R\x83\x16`@\x82\x01R`\x80``\x82\x01\x81\x90R`\0\x90a\x10u\x90\x83\x01\x84a2\x16V[\x81\x81\x03\x81\x81\x11\x15a\x1F>Wa\x1F>a0TV[``\x81R`\0a3\x05``\x83\x01\x86a*\xDFV[\x82\x81\x03` \x84\x01Ra3\x17\x81\x86a1\xBDV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3CW`\0\x80\xFD[\x82Qa3N\x81a&XV[` \x84\x01Q\x90\x92Pa.r\x81a&XV[\x80\x82\x01\x80\x82\x11\x15a\x1F>Wa\x1F>a0TV[`\0` \x82\x84\x03\x12\x15a3\x84W`\0\x80\xFD[\x81Qa\x1F\xC6\x81a+wV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R`\0\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a3\xDDW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a3\xBFV[PP\x85\x81\x03`@\x87\x01Ra3\xF1\x81\x89a1\xBDV[\x93PPPP\x82\x81\x03``\x84\x01Ra4\x08\x81\x85a0\x99V[\x97\x96PPPPPPPV[\x83\x81R\x82\x15\x15` \x82\x01R```@\x82\x01R`\0a44``\x83\x01\x84a0\x99V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a4OW`\0\x80\xFD[\x81Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F\xC6W`\0\x80\xFD\xFE\xA2dipfsX\"\x12 \xE2\xD4\xD4\xF4\x05\xE2\x13\xA0\xADS.*\xBDF\xC6\xE7\tF\xED\xB7\xC43r-W\xDE\xA2\x9B\xA5\xCC#\xDEdsolcC\0\x08\x15\x003";
    /// The deployed bytecode of the contract.
    pub static BALANCERPOOLADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BalancerPoolAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BalancerPoolAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BalancerPoolAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BalancerPoolAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BalancerPoolAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BalancerPoolAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BalancerPoolAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BALANCERPOOLADAPTORV1_ABI.clone(),
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
                BALANCERPOOLADAPTORV1_ABI.clone(),
                BALANCERPOOLADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `EXACT_TOKENS_IN_FOR_BPT_OUT` (0x0e222e52) function
        pub fn exact_tokens_in_for_bpt_out(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([14, 34, 46, 82], ())
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
        ///Calls the contract's `balancerSlippage` (0xf7e69b16) function
        pub fn balancer_slippage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([247, 230, 155, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimRewards` (0xef5cfb8c) function
        pub fn claim_rewards(
            &self,
            gauge: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 92, 251, 140], gauge)
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
        ///Calls the contract's `exitPool` (0x97194b13) function
        pub fn exit_pool(
            &self,
            target_bpt: ::ethers::core::types::Address,
            swaps_after_exit: ::std::vec::Vec<SingleSwap>,
            swap_data: SwapData,
            request: ExitPoolRequest,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [151, 25, 75, 19],
                    (target_bpt, swaps_after_exit, swap_data, request),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getExpectedTokens` (0x7906afbf) function
        pub fn get_expected_tokens(
            &self,
            target_bpt: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([121, 6, 175, 191], target_bpt)
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
        ///Calls the contract's `joinPool` (0x5c14acdb) function
        pub fn join_pool(
            &self,
            target_bpt: ::ethers::core::types::Address,
            swaps_before_join: ::std::vec::Vec<SingleSwap>,
            swap_data: SwapData,
            minimum_bpt: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [92, 20, 172, 219],
                    (target_bpt, swaps_before_join, swap_data, minimum_bpt),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makeFlashLoan` (0xc9a69562) function
        pub fn make_flash_loan(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 166, 149, 98], (tokens, amounts, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minter` (0x07546172) function
        pub fn minter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([7, 84, 97, 114], ())
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
        ///Calls the contract's `stakeBPT` (0x8f210a6a) function
        pub fn stake_bpt(
            &self,
            bpt: ::ethers::core::types::Address,
            liquidity_gauge: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 33, 10, 106], (bpt, liquidity_gauge, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unstakeBPT` (0xd2e85806) function
        pub fn unstake_bpt(
            &self,
            bpt: ::ethers::core::types::Address,
            liquidity_gauge: ::ethers::core::types::Address,
            amount_out: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 232, 88, 6], (bpt, liquidity_gauge, amount_out))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vault` (0xfbfa77cf) function
        pub fn vault(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([251, 250, 119, 207], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0xc9111bd7) function
        pub fn withdraw(
            &self,
            amount_bpt_to_send: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            adaptor_data: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [201, 17, 27, 215],
                    (amount_bpt_to_send, recipient, adaptor_data, p3),
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
    for BalancerPoolAdaptorV1<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked` with signature `BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(address,address)` and selector `0x8a1915c8`
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
        name = "BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked",
        abi = "BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(address,address)"
    )]
    pub struct BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked {
        pub bpt: ::ethers::core::types::Address,
        pub liquidity_gauge: ::ethers::core::types::Address,
    }
    ///Custom Error type `BalancerPoolAdaptor___InternalBalancesNotSupported` with signature `BalancerPoolAdaptor___InternalBalancesNotSupported()` and selector `0x6d225183`
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
        name = "BalancerPoolAdaptor___InternalBalancesNotSupported",
        abi = "BalancerPoolAdaptor___InternalBalancesNotSupported()"
    )]
    pub struct BalancerPoolAdaptor___InternalBalancesNotSupported;
    ///Custom Error type `BalancerPoolAdaptor___InvalidConstructorSlippage` with signature `BalancerPoolAdaptor___InvalidConstructorSlippage()` and selector `0x3aa8bcf1`
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
        name = "BalancerPoolAdaptor___InvalidConstructorSlippage",
        abi = "BalancerPoolAdaptor___InvalidConstructorSlippage()"
    )]
    pub struct BalancerPoolAdaptor___InvalidConstructorSlippage;
    ///Custom Error type `BalancerPoolAdaptor___LengthMismatch` with signature `BalancerPoolAdaptor___LengthMismatch()` and selector `0x967575bd`
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
        name = "BalancerPoolAdaptor___LengthMismatch",
        abi = "BalancerPoolAdaptor___LengthMismatch()"
    )]
    pub struct BalancerPoolAdaptor___LengthMismatch;
    ///Custom Error type `BalancerPoolAdaptor___Slippage` with signature `BalancerPoolAdaptor___Slippage()` and selector `0x6d7db7b2`
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
        name = "BalancerPoolAdaptor___Slippage",
        abi = "BalancerPoolAdaptor___Slippage()"
    )]
    pub struct BalancerPoolAdaptor___Slippage;
    ///Custom Error type `BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch` with signature `BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch()` and selector `0x85f4bd3b`
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
        name = "BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch",
        abi = "BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch()"
    )]
    pub struct BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch;
    ///Custom Error type `BalancerPoolAdaptor___UnsupportedTokenNotSwapped` with signature `BalancerPoolAdaptor___UnsupportedTokenNotSwapped()` and selector `0x03b4d892`
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
        name = "BalancerPoolAdaptor___UnsupportedTokenNotSwapped",
        abi = "BalancerPoolAdaptor___UnsupportedTokenNotSwapped()"
    )]
    pub struct BalancerPoolAdaptor___UnsupportedTokenNotSwapped;
    ///Custom Error type `BalancerPoolAdaptor___WrongSwapKind` with signature `BalancerPoolAdaptor___WrongSwapKind()` and selector `0x74c0729b`
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
        name = "BalancerPoolAdaptor___WrongSwapKind",
        abi = "BalancerPoolAdaptor___WrongSwapKind()"
    )]
    pub struct BalancerPoolAdaptor___WrongSwapKind;
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
    pub enum BalancerPoolAdaptorV1Errors {
        BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(
            BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked,
        ),
        BalancerPoolAdaptor___InternalBalancesNotSupported(
            BalancerPoolAdaptor___InternalBalancesNotSupported,
        ),
        BalancerPoolAdaptor___InvalidConstructorSlippage(
            BalancerPoolAdaptor___InvalidConstructorSlippage,
        ),
        BalancerPoolAdaptor___LengthMismatch(BalancerPoolAdaptor___LengthMismatch),
        BalancerPoolAdaptor___Slippage(BalancerPoolAdaptor___Slippage),
        BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch(
            BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch,
        ),
        BalancerPoolAdaptor___UnsupportedTokenNotSwapped(
            BalancerPoolAdaptor___UnsupportedTokenNotSwapped,
        ),
        BalancerPoolAdaptor___WrongSwapKind(BalancerPoolAdaptor___WrongSwapKind),
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
    impl ::ethers::core::abi::AbiDecode for BalancerPoolAdaptorV1Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(decoded),
                );
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___InternalBalancesNotSupported as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BalancerPoolAdaptor___InternalBalancesNotSupported(decoded),
                );
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___InvalidConstructorSlippage as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BalancerPoolAdaptor___InvalidConstructorSlippage(decoded),
                );
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___LengthMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalancerPoolAdaptor___LengthMismatch(decoded));
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___Slippage as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalancerPoolAdaptor___Slippage(decoded));
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___UnsupportedTokenNotSwapped as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(
                    Self::BalancerPoolAdaptor___UnsupportedTokenNotSwapped(decoded),
                );
            }
            if let Ok(decoded) = <BalancerPoolAdaptor___WrongSwapKind as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalancerPoolAdaptor___WrongSwapKind(decoded));
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
    impl ::ethers::core::abi::AbiEncode for BalancerPoolAdaptorV1Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___InternalBalancesNotSupported(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___InvalidConstructorSlippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___LengthMismatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalancerPoolAdaptor___UnsupportedTokenNotSwapped(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerPoolAdaptor___WrongSwapKind(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::ethers::contract::ContractRevert for BalancerPoolAdaptorV1Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___InternalBalancesNotSupported as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___InvalidConstructorSlippage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___LengthMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___Slippage as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___UnsupportedTokenNotSwapped as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BalancerPoolAdaptor___WrongSwapKind as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
    impl ::core::fmt::Display for BalancerPoolAdaptorV1Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___InternalBalancesNotSupported(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___InvalidConstructorSlippage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___LengthMismatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___Slippage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::BalancerPoolAdaptor___UnsupportedTokenNotSwapped(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalancerPoolAdaptor___WrongSwapKind(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String> for BalancerPoolAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked) -> Self {
            Self::BalancerPoolAdaptor__BptAndGaugeComboMustBeTracked(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___InternalBalancesNotSupported>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___InternalBalancesNotSupported) -> Self {
            Self::BalancerPoolAdaptor___InternalBalancesNotSupported(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___InvalidConstructorSlippage>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___InvalidConstructorSlippage) -> Self {
            Self::BalancerPoolAdaptor___InvalidConstructorSlippage(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___LengthMismatch>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___LengthMismatch) -> Self {
            Self::BalancerPoolAdaptor___LengthMismatch(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___Slippage>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___Slippage) -> Self {
            Self::BalancerPoolAdaptor___Slippage(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch) -> Self {
            Self::BalancerPoolAdaptor___SwapTokenAndExpectedTokenMismatch(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___UnsupportedTokenNotSwapped>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___UnsupportedTokenNotSwapped) -> Self {
            Self::BalancerPoolAdaptor___UnsupportedTokenNotSwapped(value)
        }
    }
    impl ::core::convert::From<BalancerPoolAdaptor___WrongSwapKind>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BalancerPoolAdaptor___WrongSwapKind) -> Self {
            Self::BalancerPoolAdaptor___WrongSwapKind(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ConstructorHealthFactorTooLow>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__ConstructorHealthFactorTooLow) -> Self {
            Self::BaseAdaptor__ConstructorHealthFactorTooLow(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage> for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for BalancerPoolAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserWithdrawsNotAllowed) -> Self {
            Self::BaseAdaptor__UserWithdrawsNotAllowed(value)
        }
    }
    ///Container type for all input parameters for the `EXACT_TOKENS_IN_FOR_BPT_OUT` function with signature `EXACT_TOKENS_IN_FOR_BPT_OUT()` and selector `0x0e222e52`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "EXACT_TOKENS_IN_FOR_BPT_OUT",
        abi = "EXACT_TOKENS_IN_FOR_BPT_OUT()"
    )]
    pub struct ExactTokensInForBptOutCall;
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
    ///Container type for all input parameters for the `balancerSlippage` function with signature `balancerSlippage()` and selector `0xf7e69b16`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balancerSlippage", abi = "balancerSlippage()")]
    pub struct BalancerSlippageCall;
    ///Container type for all input parameters for the `claimRewards` function with signature `claimRewards(address)` and selector `0xef5cfb8c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claimRewards", abi = "claimRewards(address)")]
    pub struct ClaimRewardsCall {
        pub gauge: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `exitPool` function with signature `exitPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),(address[],uint256[],bytes,bool))` and selector `0x97194b13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "exitPool",
        abi = "exitPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),(address[],uint256[],bytes,bool))"
    )]
    pub struct ExitPoolCall {
        pub target_bpt: ::ethers::core::types::Address,
        pub swaps_after_exit: ::std::vec::Vec<SingleSwap>,
        pub swap_data: SwapData,
        pub request: ExitPoolRequest,
    }
    ///Container type for all input parameters for the `getExpectedTokens` function with signature `getExpectedTokens(address)` and selector `0x7906afbf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getExpectedTokens", abi = "getExpectedTokens(address)")]
    pub struct GetExpectedTokensCall {
        pub target_bpt: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `joinPool` function with signature `joinPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),uint256)` and selector `0x5c14acdb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "joinPool",
        abi = "joinPool(address,(bytes32,uint8,address,address,uint256,bytes)[],(uint256[],uint256[]),uint256)"
    )]
    pub struct JoinPoolCall {
        pub target_bpt: ::ethers::core::types::Address,
        pub swaps_before_join: ::std::vec::Vec<SingleSwap>,
        pub swap_data: SwapData,
        pub minimum_bpt: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `makeFlashLoan` function with signature `makeFlashLoan(address[],uint256[],bytes)` and selector `0xc9a69562`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makeFlashLoan", abi = "makeFlashLoan(address[],uint256[],bytes)")]
    pub struct MakeFlashLoanCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `minter` function with signature `minter()` and selector `0x07546172`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "minter", abi = "minter()")]
    pub struct MinterCall;
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
    ///Container type for all input parameters for the `stakeBPT` function with signature `stakeBPT(address,address,uint256)` and selector `0x8f210a6a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stakeBPT", abi = "stakeBPT(address,address,uint256)")]
    pub struct StakeBPTCall {
        pub bpt: ::ethers::core::types::Address,
        pub liquidity_gauge: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unstakeBPT` function with signature `unstakeBPT(address,address,uint256)` and selector `0xd2e85806`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unstakeBPT", abi = "unstakeBPT(address,address,uint256)")]
    pub struct UnstakeBPTCall {
        pub bpt: ::ethers::core::types::Address,
        pub liquidity_gauge: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `vault` function with signature `vault()` and selector `0xfbfa77cf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "vault", abi = "vault()")]
    pub struct VaultCall;
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
        pub amount_bpt_to_send: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub adaptor_data: ::ethers::core::types::Bytes,
        pub p3: ::ethers::core::types::Bytes,
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
    pub enum BalancerPoolAdaptorV1Calls {
        ExactTokensInForBptOut(ExactTokensInForBptOutCall),
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        BalancerSlippage(BalancerSlippageCall),
        ClaimRewards(ClaimRewardsCall),
        Deposit(DepositCall),
        ExitPool(ExitPoolCall),
        GetExpectedTokens(GetExpectedTokensCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        JoinPool(JoinPoolCall),
        MakeFlashLoan(MakeFlashLoanCall),
        Minter(MinterCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        StakeBPT(StakeBPTCall),
        UnstakeBPT(UnstakeBPTCall),
        Vault(VaultCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for BalancerPoolAdaptorV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExactTokensInForBptOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExactTokensInForBptOut(decoded));
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
            if let Ok(decoded) = <BalancerSlippageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalancerSlippage(decoded));
            }
            if let Ok(decoded) = <ClaimRewardsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClaimRewards(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <ExitPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExitPool(decoded));
            }
            if let Ok(decoded) = <GetExpectedTokensCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetExpectedTokens(decoded));
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
            if let Ok(decoded) = <JoinPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::JoinPool(decoded));
            }
            if let Ok(decoded) = <MakeFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakeFlashLoan(decoded));
            }
            if let Ok(decoded) = <MinterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Minter(decoded));
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
            if let Ok(decoded) = <StakeBPTCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StakeBPT(decoded));
            }
            if let Ok(decoded) = <UnstakeBPTCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnstakeBPT(decoded));
            }
            if let Ok(decoded) = <VaultCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Vault(decoded));
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
    impl ::ethers::core::abi::AbiEncode for BalancerPoolAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExactTokensInForBptOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AssetOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AssetsUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalancerSlippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimRewards(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetExpectedTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Identifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsDebt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::JoinPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MakeFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Minter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevokeApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StakeBPT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnstakeBPT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Vault(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawableFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BalancerPoolAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExactTokensInForBptOut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalancerSlippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetExpectedTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::JoinPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakeFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::Minter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::StakeBPT(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnstakeBPT(element) => ::core::fmt::Display::fmt(element, f),
                Self::Vault(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExactTokensInForBptOutCall>
    for BalancerPoolAdaptorV1Calls {
        fn from(value: ExactTokensInForBptOutCall) -> Self {
            Self::ExactTokensInForBptOut(value)
        }
    }
    impl ::core::convert::From<AssetOfCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalancerSlippageCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: BalancerSlippageCall) -> Self {
            Self::BalancerSlippage(value)
        }
    }
    impl ::core::convert::From<ClaimRewardsCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: ClaimRewardsCall) -> Self {
            Self::ClaimRewards(value)
        }
    }
    impl ::core::convert::From<DepositCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<ExitPoolCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: ExitPoolCall) -> Self {
            Self::ExitPool(value)
        }
    }
    impl ::core::convert::From<GetExpectedTokensCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: GetExpectedTokensCall) -> Self {
            Self::GetExpectedTokens(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<JoinPoolCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: JoinPoolCall) -> Self {
            Self::JoinPool(value)
        }
    }
    impl ::core::convert::From<MakeFlashLoanCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: MakeFlashLoanCall) -> Self {
            Self::MakeFlashLoan(value)
        }
    }
    impl ::core::convert::From<MinterCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: MinterCall) -> Self {
            Self::Minter(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<StakeBPTCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: StakeBPTCall) -> Self {
            Self::StakeBPT(value)
        }
    }
    impl ::core::convert::From<UnstakeBPTCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: UnstakeBPTCall) -> Self {
            Self::UnstakeBPT(value)
        }
    }
    impl ::core::convert::From<VaultCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: VaultCall) -> Self {
            Self::Vault(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for BalancerPoolAdaptorV1Calls {
        fn from(value: WithdrawableFromCall) -> Self {
            Self::WithdrawableFrom(value)
        }
    }
    ///Container type for all return fields from the `EXACT_TOKENS_IN_FOR_BPT_OUT` function with signature `EXACT_TOKENS_IN_FOR_BPT_OUT()` and selector `0x0e222e52`
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
    pub struct ExactTokensInForBptOutReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `balancerSlippage` function with signature `balancerSlippage()` and selector `0xf7e69b16`
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
    pub struct BalancerSlippageReturn(pub u32);
    ///Container type for all return fields from the `getExpectedTokens` function with signature `getExpectedTokens(address)` and selector `0x7906afbf`
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
    pub struct GetExpectedTokensReturn {
        pub expected_tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
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
    ///Container type for all return fields from the `minter` function with signature `minter()` and selector `0x07546172`
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
    pub struct MinterReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `vault` function with signature `vault()` and selector `0xfbfa77cf`
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
    pub struct VaultReturn(pub ::ethers::core::types::Address);
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
    ///`SwapData(uint256[],uint256[])`
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
    pub struct SwapData {
        pub min_amounts_for_swaps: ::std::vec::Vec<::ethers::core::types::U256>,
        pub swap_deadlines: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///`ExitPoolRequest(address[],uint256[],bytes,bool)`
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
    pub struct ExitPoolRequest {
        pub assets: ::std::vec::Vec<::ethers::core::types::Address>,
        pub min_amounts_out: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
        pub to_internal_balance: bool,
    }
    ///`SingleSwap(bytes32,uint8,address,address,uint256,bytes)`
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
    pub struct SingleSwap {
        pub pool_id: [u8; 32],
        pub kind: u8,
        pub asset_in: ::ethers::core::types::Address,
        pub asset_out: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub user_data: ::ethers::core::types::Bytes,
    }
}
