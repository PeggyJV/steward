pub use morpho_reward_handler::*;
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
pub mod morpho_reward_handler {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_morphoRewardsDistributor",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("claim"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claim"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("claimable"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
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
                    ::std::borrow::ToOwned::to_owned("morphoRewardsDistributor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "morphoRewardsDistributor",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract RewardsDistributor",
                                        ),
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
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MORPHOREWARDHANDLER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x03\x1C8\x03\x80a\x03\x1C\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x02\x8Ca\0\x90`\09`\0\x81\x81`U\x01R`\xAA\x01Ra\x02\x8C`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c/R\xEB\xB7\x14a\0;W\x80c[]Mx\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01/V[a\0\x93V[\0[a\0w\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`@Qc\x0FD\xFE\x1D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\x13\xF8t\x90a\0\xE3\x900\x90\x86\x90\x86\x90`\x04\x01a\x01\xF9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x11W=`\0\x80>=`\0\xFD[PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01BW`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01bW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01vW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\x88Wa\x01\x88a\x01\x19V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x01\xADWa\x01\xADa\x01\x19V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x89\x83\x11\x15a\x01\xCBW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x01\xE9W\x845\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x01\xD0V[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x80\x82\x01\x84\x90R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R`\0\x91\x84\x81\x01\x91`\x80\x85\x01\x90\x84[\x81\x81\x10\x15a\x02HW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x02,V[P\x90\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \x95\x17,\x1FCz\xBBI\xF5J\x90\x16\xE5\x13/\xF70\xF3\xCEa\xEA\xAEQ\x94\x03Y\x81y\xB1\x16\xC3odsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static MORPHOREWARDHANDLER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80c/R\xEB\xB7\x14a\0;W\x80c[]Mx\x14a\0PW[`\0\x80\xFD[a\0Na\0I6`\x04a\x01/V[a\0\x93V[\0[a\0w\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[`@Qc\x0FD\xFE\x1D`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\x13\xF8t\x90a\0\xE3\x900\x90\x86\x90\x86\x90`\x04\x01a\x01\xF9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\0\xFDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x11W=`\0\x80>=`\0\xFD[PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x01BW`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x01bW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x01vW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x01\x88Wa\x01\x88a\x01\x19V[\x80`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x85\x82\x11\x17\x15a\x01\xADWa\x01\xADa\x01\x19V[`@R\x91\x82R\x84\x82\x01\x92P\x83\x81\x01\x85\x01\x91\x89\x83\x11\x15a\x01\xCBW`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a\x01\xE9W\x845\x84R\x93\x85\x01\x93\x92\x85\x01\x92a\x01\xD0V[\x80\x96PPPPPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R` \x80\x82\x01\x84\x90R```@\x83\x01\x81\x90R\x83Q\x90\x83\x01\x81\x90R`\0\x91\x84\x81\x01\x91`\x80\x85\x01\x90\x84[\x81\x81\x10\x15a\x02HW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x02,V[P\x90\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \x95\x17,\x1FCz\xBBI\xF5J\x90\x16\xE5\x13/\xF70\xF3\xCEa\xEA\xAEQ\x94\x03Y\x81y\xB1\x16\xC3odsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static MORPHOREWARDHANDLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MorphoRewardHandler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MorphoRewardHandler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MorphoRewardHandler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MorphoRewardHandler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MorphoRewardHandler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MorphoRewardHandler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MorphoRewardHandler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MORPHOREWARDHANDLER_ABI.clone(),
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
                MORPHOREWARDHANDLER_ABI.clone(),
                MORPHOREWARDHANDLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `claim` (0x2f52ebb7) function
        pub fn claim(
            &self,
            claimable: ::ethers::core::types::U256,
            proof: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 82, 235, 183], (claimable, proof))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `morphoRewardsDistributor` (0x5b5d4d78) function
        pub fn morpho_rewards_distributor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([91, 93, 77, 120], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MorphoRewardHandler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `claim` function with signature `claim(uint256,bytes32[])` and selector `0x2f52ebb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "claim", abi = "claim(uint256,bytes32[])")]
    pub struct ClaimCall {
        pub claimable: ::ethers::core::types::U256,
        pub proof: ::std::vec::Vec<[u8; 32]>,
    }
    ///Container type for all input parameters for the `morphoRewardsDistributor` function with signature `morphoRewardsDistributor()` and selector `0x5b5d4d78`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Deserialize,
        serde::Serialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "morphoRewardsDistributor", abi = "morphoRewardsDistributor()")]
    pub struct MorphoRewardsDistributorCall;
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
    pub enum MorphoRewardHandlerCalls {
        Claim(ClaimCall),
        MorphoRewardsDistributor(MorphoRewardsDistributorCall),
    }
    impl ::ethers::core::abi::AbiDecode for MorphoRewardHandlerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded) = <MorphoRewardsDistributorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MorphoRewardsDistributor(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MorphoRewardHandlerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MorphoRewardsDistributor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MorphoRewardHandlerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::MorphoRewardsDistributor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ClaimCall> for MorphoRewardHandlerCalls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<MorphoRewardsDistributorCall>
    for MorphoRewardHandlerCalls {
        fn from(value: MorphoRewardsDistributorCall) -> Self {
            Self::MorphoRewardsDistributor(value)
        }
    }
    ///Container type for all return fields from the `morphoRewardsDistributor` function with signature `morphoRewardsDistributor()` and selector `0x5b5d4d78`
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
    pub struct MorphoRewardsDistributorReturn(pub ::ethers::core::types::Address);
}
