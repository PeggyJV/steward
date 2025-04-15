pub use swap_with_uniswap_adaptor_v1::*;
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
pub mod swap_with_uniswap_adaptor_v1 {
    const _: () = {
        ::core::include_bytes!(
            "../abi/SwapWithUniswapAdaptorV1.json",
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
                    ::std::borrow::ToOwned::to_owned("swapWithUniV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapWithUniV2"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
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
                    ::std::borrow::ToOwned::to_owned("swapWithUniV3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapWithUniV3"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
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
                                    name: ::std::borrow::ToOwned::to_owned("poolFees"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24[]"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
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
                    ::std::borrow::ToOwned::to_owned("uniswapV2Router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uniswapV2Router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IUniswapV2Router02",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3Router"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uniswapV3Router"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IUniswapV3Router",
                                        ),
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
    pub static SWAPWITHUNISWAPADAPTORV1_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x17K\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x8CW\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x01\xE0W\x80c\xD3\xBF\xE7j\x14a\x01\xF3W\x80c\xE1p\xA9\xBF\x14a\x02\x06W\x80c\xFAP\xE5\xD2\x14a\x02\x14W`\0\x80\xFD[\x80c\x895:\t\x14a\x01\x9EW\x80c\xAE\xFF\xDD\xDE\x14a\x01\xADW\x80c\xC4Tzs\x14a\x01\xCDW`\0\x80\xFD[\x80c>\x03*;\x11a\0\xC8W\x80c>\x03*;\x14a\x01QW\x80ciD\\1\x14a\x01aW\x80cxASe\x14a\x01tW\x80cy\x98\xA1\xC4\x14a\x01\x96W`\0\x80\xFD[\x80c\x16\x94P^\x14a\0\xEFW\x80c\x172\xC1\xDC\x14a\x01\"W\x80c,v\xD7\xA6\x14a\x017W[`\0\x80\xFD[sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x015a\x0106`\x04a\x10\xBDV[a\x02'V[\0[s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x01\x05V[`@Qa#(\x81R` \x01a\x01\x19V[a\x015a\x01o6`\x04a\x11{V[a\x07;V[a\x01\x88a\x01\x826`\x04a\x11\xE8V[P`\0\x90V[`@Q\x90\x81R` \x01a\x01\x19V[a\x01\x88a\x07TV[`@Q`\0\x81R` \x01a\x01\x19V[a\x01\xC0a\x01\xBB6`\x04a\x11\xE8V[a\x07\xB2V[`@Qa\x01\x19\x91\x90a\x12%V[a\x015a\x01\xDB6`\x04a\x12rV[a\x08\x0EV[a\x015a\x01\xEE6`\x04a\x13MV[a\r\xDFV[a\x015a\x02\x016`\x04a\x13\xCDV[a\r\xF8V[a\x01\x05a\x01\x826`\x04a\x11\xE8V[a\x01\x88a\x02\"6`\x04a\x14\x06V[a\x0E\x11V[`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8B\x91\x90a\x14jV[\x90P`\0\x84`\0\x81Q\x81\x10a\x02\xA2Wa\x02\xA2a\x14\x8EV[` \x02` \x01\x01Q\x90P`\0\x85`\x01\x87Qa\x02\xBD\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x02\xCDWa\x02\xCDa\x14\x8EV[` \x02` \x01\x01Q\x90Pa\x02\xE1\x82\x86a\x0E\x1AV[\x94Pa\x03\rsz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D[`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x87a\x0E\x9BV[`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03w\x91\x90a\x14\xCDV[\x15a\x06\x83W`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x84\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE6\x91\x90a\x14\xCDV[a\x04\x13W`@Qcz\xB28I`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04~\x91\x90a\x14\xEFV[\x90P`\0sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Dc8\xED\x179\x88\x88\x8B0a\x04\xABB`<a\x15\x08V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xCB\x95\x94\x93\x92\x91\x90a\x15\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x12\x91\x90\x81\x01\x90a\x15\x8CV[\x90P`\0\x81`\x01\x83Qa\x05%\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x055Wa\x055a\x14\x8EV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\x14\xEFV[a\x05\xB8\x90\x85a\x14\xBAV[`@Qc\x15\x1DJY`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x81\x16`D\x83\x01R\x91\x92P`\0\x91\x89\x16\x90c\xA8\xEAR\xC8\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x067\x91\x90a\x14\xEFV[\x90Pa\x06Ya#([c\xFF\xFF\xFF\xFF\x16a'\x10\x84a\x0F\x18\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81\x10\x15a\x06yW`@Qc\x1E\xA0\xEB\x01`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPa\x07\x15V[sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Dc8\xED\x179\x86\x86\x890a\x06\xACB`<a\x15\x08V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xCC\x95\x94\x93\x92\x91\x90a\x15\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x13\x91\x90\x81\x01\x90a\x15\x8CV[P[a\x073\x82sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Da\x0F7V[PPPPPPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q` \x01a\x07\x97\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FSwap With Uniswap Adaptor V 0.0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x07\xE9Wa\x07\xE9a\x14\x8EV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08r\x91\x90a\x14jV[\x90P`\0\x85`\0\x81Q\x81\x10a\x08\x89Wa\x08\x89a\x14\x8EV[` \x02` \x01\x01Q\x90P`\0\x86`\x01\x88Qa\x08\xA4\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x08\xB4Wa\x08\xB4a\x14\x8EV[` \x02` \x01\x01Q\x90Pa\x08\xC8\x82\x86a\x0E\x1AV[\x94Pa\x08\xE7s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x02\xFCV[`\0\x87`\0\x81Q\x81\x10a\x08\xFCWa\x08\xFCa\x14\x8EV[` \x02` \x01\x01Q`@Q` \x01a\t,\x91\x90``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01[\x88Q\x81\x10\x15a\t\xBFW\x81\x88a\tW`\x01\x84a\x14\xBAV[\x81Q\x81\x10a\tgWa\tga\x14\x8EV[` \x02` \x01\x01Q\x8A\x83\x81Q\x81\x10a\t\x81Wa\t\x81a\x14\x8EV[` \x02` \x01\x01Q`@Q` \x01a\t\x9B\x93\x92\x91\x90a\x16AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x80\x80a\t\xB7\x90a\x16\x8DV[\x91PPa\tAV[P`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x85\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n*\x91\x90a\x14\xCDV[\x15a\r\nW`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x85\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x99\x91\x90a\x14\xCDV[a\n\xC1W`@Qcz\xB28I`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\nV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a\x14\xEFV[`@\x80Q`\xA0\x81\x01\x82R\x84\x81R0` \x82\x01R\x91\x92P`\0\x91s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15d\x91c\xC0K\x8DY\x91\x90\x81\x01a\x0BoB`<a\x15\x08V[\x81R` \x01\x8B\x81R` \x01\x8A\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x9A\x91\x90a\x16\xA6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xDD\x91\x90a\x14\xEFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CK\x91\x90a\x14\xEFV[a\x0CU\x90\x84a\x14\xBAV[`@Qc\x15\x1DJY`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x81\x16`D\x83\x01R\x91\x92P`\0\x91\x89\x16\x90c\xA8\xEAR\xC8\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD4\x91\x90a\x14\xEFV[\x90Pa\x0C\xE1a#(a\x06@V[\x81\x10\x15a\r\x01W`@Qc\x1E\xA0\xEB\x01`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPa\r\xB7V[`@\x80Q`\xA0\x81\x01\x82R\x82\x81R0` \x82\x01Rs\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15d\x91c\xC0K\x8DY\x91\x90\x81\x01a\rGB`<a\x15\x08V[\x81R` \x01\x89\x81R` \x01\x88\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rr\x91\x90a\x16\xA6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB5\x91\x90a\x14\xEFV[P[a\r\xD5\x83s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x0F7V[PPPPPPPPV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\r`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0E\x9BV[PPV[`\0[\x92\x91PPV[`\0`\0\x19\x82\x03a\x0E\x94W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8D\x91\x90a\x14\xEFV[\x90Pa\x0E\x14V[P\x80a\x0E\x14V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x04\nV[PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0F0W`\0\x80\xFD[\x04\x92\x91PPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAB\x91\x90a\x14\xEFV[\x11\x15a\x0E\rWa\x0E\r`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0E\x9BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x05Wa\x10\x05a\x0F\xC6V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10'Wa\x10'a\x0F\xC6V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10FW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x10ZW`\0\x80\xFD[\x815` a\x10oa\x10j\x83a\x10\rV[a\x0F\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\x8EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x10\xB2W\x805a\x10\xA5\x81a\x101V[\x83R\x91\x83\x01\x91\x83\x01a\x10\x92V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xD2W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xE9W`\0\x80\xFD[a\x10\xF5\x86\x82\x87\x01a\x10IV[\x96` \x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x11\x1CW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x116Wa\x116a\x0F\xC6V[a\x11I`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0F\xDCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11^W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x90W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xAFW`\0\x80\xFD[a\x11\xBB\x87\x83\x88\x01a\x11\x0BV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x11\xD1W`\0\x80\xFD[Pa\x11\xDE\x86\x82\x87\x01a\x11\x0BV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x11\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x11W`\0\x80\xFD[a\x12\x1D\x84\x82\x85\x01a\x11\x0BV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x12fW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x12AV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\x88W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xA0W`\0\x80\xFD[a\x12\xAC\x88\x83\x89\x01a\x10IV[\x95P` \x91P\x81\x87\x015\x81\x81\x11\x15a\x12\xC3W`\0\x80\xFD[\x87\x01\x90P`\x1F\x81\x01\x88\x13a\x12\xD6W`\0\x80\xFD[\x805a\x12\xE4a\x10j\x82a\x10\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15a\x13\x03W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x133W\x835b\xFF\xFF\xFF\x81\x16\x81\x14a\x13$W`\0\x80\x81\xFD[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x13\x08V[\x97\x9A\x97\x99PPPP`@\x86\x015\x95``\x015\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13cW`\0\x80\xFD[\x845\x93P` \x85\x015a\x13u\x81a\x101V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\x92W`\0\x80\xFD[a\x13\x9E\x88\x83\x89\x01a\x11\x0BV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x13\xB4W`\0\x80\xFD[Pa\x13\xC1\x87\x82\x88\x01a\x11\x0BV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xE0W`\0\x80\xFD[\x825a\x13\xEB\x81a\x101V[\x91P` \x83\x015a\x13\xFB\x81a\x101V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x19W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x141W`\0\x80\xFD[a\x14=\x86\x83\x87\x01a\x11\x0BV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x14SW`\0\x80\xFD[Pa\x14`\x85\x82\x86\x01a\x11\x0BV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14|W`\0\x80\xFD[\x81Qa\x14\x87\x81a\x101V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\x14Wa\x0E\x14a\x14\xA4V[`\0` \x82\x84\x03\x12\x15a\x14\xDFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\x87W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\x01W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x14Wa\x0E\x14a\x14\xA4V[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a\x15kW\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x15FV[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x15\x9FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15\xC7W`\0\x80\xFD[\x80Qa\x15\xD5a\x10j\x82a\x10\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x15\xF4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x16\x12W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\xF9V[\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\x168W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16 V[PP`\0\x91\x01RV[`\0\x84Qa\x16S\x81\x84` \x89\x01a\x16\x1DV[`\xE8\x94\x90\x94\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16\x91\x90\x93\x01\x90\x81R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x03\x82\x01R`\x17\x01\x92\x91PPV[`\0`\x01\x82\x01a\x16\x9FWa\x16\x9Fa\x14\xA4V[P`\x01\x01\x90V[` \x81R`\0\x82Q`\xA0` \x84\x01R\x80Q\x80`\xC0\x85\x01Ra\x16\xCE\x81`\xE0\x86\x01` \x85\x01a\x16\x1DV[`\x01\x80`\xA0\x1B\x03` \x86\x01Q\x16`@\x85\x01R`@\x85\x01Q``\x85\x01R``\x85\x01Q`\x80\x85\x01R`\x80\x85\x01Q`\xA0\x85\x01R`\xE0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 x\x8E\xE9\xCF\xB5\x18\x7F\x9F\xBD8\xC6\x18\x88,\xC9&t\x99\x9DC\x90\x97\xCBV9b\xFD\xEE\xE9#w\xD2dsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static SWAPWITHUNISWAPADAPTORV1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xEAW`\x005`\xE0\x1C\x80c\x895:\t\x11a\0\x8CW\x80c\xC9\x11\x1B\xD7\x11a\0fW\x80c\xC9\x11\x1B\xD7\x14a\x01\xE0W\x80c\xD3\xBF\xE7j\x14a\x01\xF3W\x80c\xE1p\xA9\xBF\x14a\x02\x06W\x80c\xFAP\xE5\xD2\x14a\x02\x14W`\0\x80\xFD[\x80c\x895:\t\x14a\x01\x9EW\x80c\xAE\xFF\xDD\xDE\x14a\x01\xADW\x80c\xC4Tzs\x14a\x01\xCDW`\0\x80\xFD[\x80c>\x03*;\x11a\0\xC8W\x80c>\x03*;\x14a\x01QW\x80ciD\\1\x14a\x01aW\x80cxASe\x14a\x01tW\x80cy\x98\xA1\xC4\x14a\x01\x96W`\0\x80\xFD[\x80c\x16\x94P^\x14a\0\xEFW\x80c\x172\xC1\xDC\x14a\x01\"W\x80c,v\xD7\xA6\x14a\x017W[`\0\x80\xFD[sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x015a\x0106`\x04a\x10\xBDV[a\x02'V[\0[s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x01\x05V[`@Qa#(\x81R` \x01a\x01\x19V[a\x015a\x01o6`\x04a\x11{V[a\x07;V[a\x01\x88a\x01\x826`\x04a\x11\xE8V[P`\0\x90V[`@Q\x90\x81R` \x01a\x01\x19V[a\x01\x88a\x07TV[`@Q`\0\x81R` \x01a\x01\x19V[a\x01\xC0a\x01\xBB6`\x04a\x11\xE8V[a\x07\xB2V[`@Qa\x01\x19\x91\x90a\x12%V[a\x015a\x01\xDB6`\x04a\x12rV[a\x08\x0EV[a\x015a\x01\xEE6`\x04a\x13MV[a\r\xDFV[a\x015a\x02\x016`\x04a\x13\xCDV[a\r\xF8V[a\x01\x05a\x01\x826`\x04a\x11\xE8V[a\x01\x88a\x02\"6`\x04a\x14\x06V[a\x0E\x11V[`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02gW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\x8B\x91\x90a\x14jV[\x90P`\0\x84`\0\x81Q\x81\x10a\x02\xA2Wa\x02\xA2a\x14\x8EV[` \x02` \x01\x01Q\x90P`\0\x85`\x01\x87Qa\x02\xBD\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x02\xCDWa\x02\xCDa\x14\x8EV[` \x02` \x01\x01Q\x90Pa\x02\xE1\x82\x86a\x0E\x1AV[\x94Pa\x03\rsz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8D[`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x87a\x0E\x9BV[`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x84\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03SW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03w\x91\x90a\x14\xCDV[\x15a\x06\x83W`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R\x84\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xE6\x91\x90a\x14\xCDV[a\x04\x13W`@Qcz\xB28I`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04~\x91\x90a\x14\xEFV[\x90P`\0sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Dc8\xED\x179\x88\x88\x8B0a\x04\xABB`<a\x15\x08V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xCB\x95\x94\x93\x92\x91\x90a\x15\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x04\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x12\x91\x90\x81\x01\x90a\x15\x8CV[\x90P`\0\x81`\x01\x83Qa\x05%\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x055Wa\x055a\x14\x8EV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\x14\xEFV[a\x05\xB8\x90\x85a\x14\xBAV[`@Qc\x15\x1DJY`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x81\x16`D\x83\x01R\x91\x92P`\0\x91\x89\x16\x90c\xA8\xEAR\xC8\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x13W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x067\x91\x90a\x14\xEFV[\x90Pa\x06Ya#([c\xFF\xFF\xFF\xFF\x16a'\x10\x84a\x0F\x18\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81\x10\x15a\x06yW`@Qc\x1E\xA0\xEB\x01`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPa\x07\x15V[sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Dc8\xED\x179\x86\x86\x890a\x06\xACB`<a\x15\x08V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xCC\x95\x94\x93\x92\x91\x90a\x15\x1BV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\xEBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\x13\x91\x90\x81\x01\x90a\x15\x8CV[P[a\x073\x82sz%\rV0\xB4\xCFS\x979\xDF,]\xAC\xB4\xC6Y\xF2H\x8Da\x0F7V[PPPPPPV[`@Qc2\x04\xED[`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q` \x01a\x07\x97\x90` \x80\x82R`\x1F\x90\x82\x01R\x7FSwap With Uniswap Adaptor V 0.0\0`@\x82\x01R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P`\0\x81`\0\x81Q\x81\x10a\x07\xE9Wa\x07\xE9a\x14\x8EV[` \x02` \x01\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x91\x90PV[`\x000`\x01`\x01`\xA0\x1B\x03\x16c\xD7\xD4\xBFE`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08NW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08r\x91\x90a\x14jV[\x90P`\0\x85`\0\x81Q\x81\x10a\x08\x89Wa\x08\x89a\x14\x8EV[` \x02` \x01\x01Q\x90P`\0\x86`\x01\x88Qa\x08\xA4\x91\x90a\x14\xBAV[\x81Q\x81\x10a\x08\xB4Wa\x08\xB4a\x14\x8EV[` \x02` \x01\x01Q\x90Pa\x08\xC8\x82\x86a\x0E\x1AV[\x94Pa\x08\xE7s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x02\xFCV[`\0\x87`\0\x81Q\x81\x10a\x08\xFCWa\x08\xFCa\x14\x8EV[` \x02` \x01\x01Q`@Q` \x01a\t,\x91\x90``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x81R`\x14\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x90P`\x01[\x88Q\x81\x10\x15a\t\xBFW\x81\x88a\tW`\x01\x84a\x14\xBAV[\x81Q\x81\x10a\tgWa\tga\x14\x8EV[` \x02` \x01\x01Q\x8A\x83\x81Q\x81\x10a\t\x81Wa\t\x81a\x14\x8EV[` \x02` \x01\x01Q`@Q` \x01a\t\x9B\x93\x92\x91\x90a\x16AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P\x80\x80a\t\xB7\x90a\x16\x8DV[\x91PPa\tAV[P`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x85\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n*\x91\x90a\x14\xCDV[\x15a\r\nW`@QcO\x12\x9CS`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x85\x16\x90cO\x12\x9CS\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nuW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x99\x91\x90a\x14\xCDV[a\n\xC1W`@Qcz\xB28I`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\nV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x08W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B,\x91\x90a\x14\xEFV[`@\x80Q`\xA0\x81\x01\x82R\x84\x81R0` \x82\x01R\x91\x92P`\0\x91s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15d\x91c\xC0K\x8DY\x91\x90\x81\x01a\x0BoB`<a\x15\x08V[\x81R` \x01\x8B\x81R` \x01\x8A\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x9A\x91\x90a\x16\xA6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x0B\xB9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xDD\x91\x90a\x14\xEFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CK\x91\x90a\x14\xEFV[a\x0CU\x90\x84a\x14\xBAV[`@Qc\x15\x1DJY`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x85\x90R\x88\x81\x16`D\x83\x01R\x91\x92P`\0\x91\x89\x16\x90c\xA8\xEAR\xC8\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xB0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xD4\x91\x90a\x14\xEFV[\x90Pa\x0C\xE1a#(a\x06@V[\x81\x10\x15a\r\x01W`@Qc\x1E\xA0\xEB\x01`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPa\r\xB7V[`@\x80Q`\xA0\x81\x01\x82R\x82\x81R0` \x82\x01Rs\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15d\x91c\xC0K\x8DY\x91\x90\x81\x01a\rGB`<a\x15\x08V[\x81R` \x01\x89\x81R` \x01\x88\x81RP`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\rr\x91\x90a\x16\xA6V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\r\x91W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xB5\x91\x90a\x14\xEFV[P[a\r\xD5\x83s\xE5\x92Bz\n\xEC\xE9-\xE3\xED\xEE\x1F\x18\xE0\x15|\x05\x86\x15da\x0F7V[PPPPPPPPV[`@Qc_P\x03\xC5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E\r`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0E\x9BV[PPV[`\0[\x92\x91PPV[`\0`\0\x19\x82\x03a\x0E\x94W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EiW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x8D\x91\x90a\x14\xEFV[\x90Pa\x0E\x14V[P\x80a\x0E\x14V[`\0`@Qc\t^\xA7\xB3`\xE0\x1B\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x0F\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x10T\x14\x14\x93\xD5\x91W\xD1\x90RS\x11Q`\x92\x1B`D\x82\x01R`d\x01a\x04\nV[PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0F0W`\0\x80\xFD[\x04\x92\x91PPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R`\0\x91\x90\x84\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x87W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xAB\x91\x90a\x14\xEFV[\x11\x15a\x0E\rWa\x0E\r`\x01`\x01`\xA0\x1B\x03\x83\x16\x82`\0a\x0E\x9BV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x10\x05Wa\x10\x05a\x0F\xC6V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10'Wa\x10'a\x0F\xC6V[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10FW`\0\x80\xFD[PV[`\0\x82`\x1F\x83\x01\x12a\x10ZW`\0\x80\xFD[\x815` a\x10oa\x10j\x83a\x10\rV[a\x0F\xDCV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\x8EW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x10\xB2W\x805a\x10\xA5\x81a\x101V[\x83R\x91\x83\x01\x91\x83\x01a\x10\x92V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x10\xD2W`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\xE9W`\0\x80\xFD[a\x10\xF5\x86\x82\x87\x01a\x10IV[\x96` \x86\x015\x96P`@\x90\x95\x015\x94\x93PPPPV[`\0\x82`\x1F\x83\x01\x12a\x11\x1CW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x116Wa\x116a\x0F\xC6V[a\x11I`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0F\xDCV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x11^W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\x90W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\xAFW`\0\x80\xFD[a\x11\xBB\x87\x83\x88\x01a\x11\x0BV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x11\xD1W`\0\x80\xFD[Pa\x11\xDE\x86\x82\x87\x01a\x11\x0BV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x11\xFAW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\x11W`\0\x80\xFD[a\x12\x1D\x84\x82\x85\x01a\x11\x0BV[\x94\x93PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x12fW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x12AV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\x88W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x12\xA0W`\0\x80\xFD[a\x12\xAC\x88\x83\x89\x01a\x10IV[\x95P` \x91P\x81\x87\x015\x81\x81\x11\x15a\x12\xC3W`\0\x80\xFD[\x87\x01\x90P`\x1F\x81\x01\x88\x13a\x12\xD6W`\0\x80\xFD[\x805a\x12\xE4a\x10j\x82a\x10\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15a\x13\x03W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x133W\x835b\xFF\xFF\xFF\x81\x16\x81\x14a\x13$W`\0\x80\x81\xFD[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x13\x08V[\x97\x9A\x97\x99PPPP`@\x86\x015\x95``\x015\x94\x93PPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x13cW`\0\x80\xFD[\x845\x93P` \x85\x015a\x13u\x81a\x101V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x13\x92W`\0\x80\xFD[a\x13\x9E\x88\x83\x89\x01a\x11\x0BV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x13\xB4W`\0\x80\xFD[Pa\x13\xC1\x87\x82\x88\x01a\x11\x0BV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xE0W`\0\x80\xFD[\x825a\x13\xEB\x81a\x101V[\x91P` \x83\x015a\x13\xFB\x81a\x101V[\x80\x91PP\x92P\x92\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x19W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x141W`\0\x80\xFD[a\x14=\x86\x83\x87\x01a\x11\x0BV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x14SW`\0\x80\xFD[Pa\x14`\x85\x82\x86\x01a\x11\x0BV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x14|W`\0\x80\xFD[\x81Qa\x14\x87\x81a\x101V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\x14Wa\x0E\x14a\x14\xA4V[`\0` \x82\x84\x03\x12\x15a\x14\xDFW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x14\x87W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x15\x01W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x14Wa\x0E\x14a\x14\xA4V[`\0`\xA0\x82\x01\x87\x83R` \x87\x81\x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P\x82\x89\x01\x93P`\0[\x81\x81\x10\x15a\x15kW\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x15FV[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x15\x9FW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xB6W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15\xC7W`\0\x80\xFD[\x80Qa\x15\xD5a\x10j\x82a\x10\rV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a\x15\xF4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x16\x12W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\xF9V[\x97\x96PPPPPPPV[`\0[\x83\x81\x10\x15a\x168W\x81\x81\x01Q\x83\x82\x01R` \x01a\x16 V[PP`\0\x91\x01RV[`\0\x84Qa\x16S\x81\x84` \x89\x01a\x16\x1DV[`\xE8\x94\x90\x94\x1B`\x01`\x01`\xE8\x1B\x03\x19\x16\x91\x90\x93\x01\x90\x81R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x03\x82\x01R`\x17\x01\x92\x91PPV[`\0`\x01\x82\x01a\x16\x9FWa\x16\x9Fa\x14\xA4V[P`\x01\x01\x90V[` \x81R`\0\x82Q`\xA0` \x84\x01R\x80Q\x80`\xC0\x85\x01Ra\x16\xCE\x81`\xE0\x86\x01` \x85\x01a\x16\x1DV[`\x01\x80`\xA0\x1B\x03` \x86\x01Q\x16`@\x85\x01R`@\x85\x01Q``\x85\x01R``\x85\x01Q`\x80\x85\x01R`\x80\x85\x01Q`\xA0\x85\x01R`\xE0`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV\xFE\xA2dipfsX\"\x12 x\x8E\xE9\xCF\xB5\x18\x7F\x9F\xBD8\xC6\x18\x88,\xC9&t\x99\x9DC\x90\x97\xCBV9b\xFD\xEE\xE9#w\xD2dsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static SWAPWITHUNISWAPADAPTORV1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SwapWithUniswapAdaptorV1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SwapWithUniswapAdaptorV1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SwapWithUniswapAdaptorV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SwapWithUniswapAdaptorV1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SwapWithUniswapAdaptorV1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SwapWithUniswapAdaptorV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SwapWithUniswapAdaptorV1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SWAPWITHUNISWAPADAPTORV1_ABI.clone(),
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
                SWAPWITHUNISWAPADAPTORV1_ABI.clone(),
                SWAPWITHUNISWAPADAPTORV1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `swapWithUniV2` (0x1732c1dc) function
        pub fn swap_with_uni_v2(
            &self,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            amount: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 50, 193, 220], (path, amount, amount_out_min))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapWithUniV3` (0xc4547a73) function
        pub fn swap_with_uni_v3(
            &self,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            pool_fees: ::std::vec::Vec<u32>,
            amount: ::ethers::core::types::U256,
            amount_out_min: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [196, 84, 122, 115],
                    (path, pool_fees, amount, amount_out_min),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV2Router` (0x1694505e) function
        pub fn uniswap_v2_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([22, 148, 80, 94], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3Router` (0x2c76d7a6) function
        pub fn uniswap_v3_router(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([44, 118, 215, 166], ())
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
    for SwapWithUniswapAdaptorV1<M> {
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
    pub enum SwapWithUniswapAdaptorV1Errors {
        BaseAdaptor__ExternalReceiverBlocked(BaseAdaptor__ExternalReceiverBlocked),
        BaseAdaptor__PricingNotSupported(BaseAdaptor__PricingNotSupported),
        BaseAdaptor__Slippage(BaseAdaptor__Slippage),
        BaseAdaptor__UserDepositsNotAllowed(BaseAdaptor__UserDepositsNotAllowed),
        BaseAdaptor__UserWithdrawsNotAllowed(BaseAdaptor__UserWithdrawsNotAllowed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SwapWithUniswapAdaptorV1Errors {
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
    impl ::ethers::core::abi::AbiEncode for SwapWithUniswapAdaptorV1Errors {
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
    impl ::ethers::contract::ContractRevert for SwapWithUniswapAdaptorV1Errors {
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
    impl ::core::fmt::Display for SwapWithUniswapAdaptorV1Errors {
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
    for SwapWithUniswapAdaptorV1Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__ExternalReceiverBlocked>
    for SwapWithUniswapAdaptorV1Errors {
        fn from(value: BaseAdaptor__ExternalReceiverBlocked) -> Self {
            Self::BaseAdaptor__ExternalReceiverBlocked(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__PricingNotSupported>
    for SwapWithUniswapAdaptorV1Errors {
        fn from(value: BaseAdaptor__PricingNotSupported) -> Self {
            Self::BaseAdaptor__PricingNotSupported(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__Slippage>
    for SwapWithUniswapAdaptorV1Errors {
        fn from(value: BaseAdaptor__Slippage) -> Self {
            Self::BaseAdaptor__Slippage(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserDepositsNotAllowed>
    for SwapWithUniswapAdaptorV1Errors {
        fn from(value: BaseAdaptor__UserDepositsNotAllowed) -> Self {
            Self::BaseAdaptor__UserDepositsNotAllowed(value)
        }
    }
    impl ::core::convert::From<BaseAdaptor__UserWithdrawsNotAllowed>
    for SwapWithUniswapAdaptorV1Errors {
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
    ///Container type for all input parameters for the `swapWithUniV2` function with signature `swapWithUniV2(address[],uint256,uint256)` and selector `0x1732c1dc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "swapWithUniV2", abi = "swapWithUniV2(address[],uint256,uint256)")]
    pub struct SwapWithUniV2Call {
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amount: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapWithUniV3` function with signature `swapWithUniV3(address[],uint24[],uint256,uint256)` and selector `0xc4547a73`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
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
        name = "swapWithUniV3",
        abi = "swapWithUniV3(address[],uint24[],uint256,uint256)"
    )]
    pub struct SwapWithUniV3Call {
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub pool_fees: ::std::vec::Vec<u32>,
        pub amount: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uniswapV2Router` function with signature `uniswapV2Router()` and selector `0x1694505e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "uniswapV2Router", abi = "uniswapV2Router()")]
    pub struct UniswapV2RouterCall;
    ///Container type for all input parameters for the `uniswapV3Router` function with signature `uniswapV3Router()` and selector `0x2c76d7a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "uniswapV3Router", abi = "uniswapV3Router()")]
    pub struct UniswapV3RouterCall;
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
    pub enum SwapWithUniswapAdaptorV1Calls {
        AssetOf(AssetOfCall),
        AssetsUsed(AssetsUsedCall),
        BalanceOf(BalanceOfCall),
        Deposit(DepositCall),
        Identifier(IdentifierCall),
        IsDebt(IsDebtCall),
        RevokeApproval(RevokeApprovalCall),
        Slippage(SlippageCall),
        SwapWithUniV2(SwapWithUniV2Call),
        SwapWithUniV3(SwapWithUniV3Call),
        UniswapV2Router(UniswapV2RouterCall),
        UniswapV3Router(UniswapV3RouterCall),
        Withdraw(WithdrawCall),
        WithdrawableFrom(WithdrawableFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for SwapWithUniswapAdaptorV1Calls {
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
            if let Ok(decoded) = <SwapWithUniV2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapWithUniV2(decoded));
            }
            if let Ok(decoded) = <SwapWithUniV3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapWithUniV3(decoded));
            }
            if let Ok(decoded) = <UniswapV2RouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV2Router(decoded));
            }
            if let Ok(decoded) = <UniswapV3RouterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3Router(decoded));
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
    impl ::ethers::core::abi::AbiEncode for SwapWithUniswapAdaptorV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::RevokeApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Slippage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapWithUniV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapWithUniV3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV2Router(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3Router(element) => {
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
    impl ::core::fmt::Display for SwapWithUniswapAdaptorV1Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::AssetsUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Identifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDebt(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokeApproval(element) => ::core::fmt::Display::fmt(element, f),
                Self::Slippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapWithUniV2(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapWithUniV3(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV2Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3Router(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawableFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetOfCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: AssetOfCall) -> Self {
            Self::AssetOf(value)
        }
    }
    impl ::core::convert::From<AssetsUsedCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: AssetsUsedCall) -> Self {
            Self::AssetsUsed(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DepositCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<IdentifierCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: IdentifierCall) -> Self {
            Self::Identifier(value)
        }
    }
    impl ::core::convert::From<IsDebtCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: IsDebtCall) -> Self {
            Self::IsDebt(value)
        }
    }
    impl ::core::convert::From<RevokeApprovalCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: RevokeApprovalCall) -> Self {
            Self::RevokeApproval(value)
        }
    }
    impl ::core::convert::From<SlippageCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: SlippageCall) -> Self {
            Self::Slippage(value)
        }
    }
    impl ::core::convert::From<SwapWithUniV2Call> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: SwapWithUniV2Call) -> Self {
            Self::SwapWithUniV2(value)
        }
    }
    impl ::core::convert::From<SwapWithUniV3Call> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: SwapWithUniV3Call) -> Self {
            Self::SwapWithUniV3(value)
        }
    }
    impl ::core::convert::From<UniswapV2RouterCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: UniswapV2RouterCall) -> Self {
            Self::UniswapV2Router(value)
        }
    }
    impl ::core::convert::From<UniswapV3RouterCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: UniswapV3RouterCall) -> Self {
            Self::UniswapV3Router(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for SwapWithUniswapAdaptorV1Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawableFromCall> for SwapWithUniswapAdaptorV1Calls {
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
    ///Container type for all return fields from the `uniswapV2Router` function with signature `uniswapV2Router()` and selector `0x1694505e`
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
    pub struct UniswapV2RouterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `uniswapV3Router` function with signature `uniswapV3Router()` and selector `0x2c76d7a6`
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
    pub struct UniswapV3RouterReturn(pub ::ethers::core::types::Address);
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
