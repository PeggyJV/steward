///
/// Represents a function call to the Aave V2 Stablecoin cellar
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV2Stablecoin {
    /// The function you wish to execute on the target cellar
    #[prost(
        oneof = "aave_v2_stablecoin::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub function: ::core::option::Option<aave_v2_stablecoin::Function>,
}
/// Nested message and enum types in `AaveV2Stablecoin`.
pub mod aave_v2_stablecoin {
    ///
    /// Accrue yield, platform fees, and performance fees..
    ///
    /// Represents function `accrue()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Accrue {}
    ///
    /// Claim rewards from Aave and begin cooldown period to unstake them.
    ///
    /// Represents function `claimAndUnstake()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimAndUnstake {}
    ///
    /// Pushes total assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct EnterPosition {}
    ///
    /// Pushes assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition(uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct EnterPositionWithAssets {
        /// amount of assets to enter into the current position
        #[prost(string, tag = "1")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Pulls total assets from the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPosition {}
    ///
    /// Pulls assets from the current Aave lending position.
    ///
    /// Represents function `exitPosition(uint256 assets)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ExitPositionWithAssets {
        /// amount of assets to exit from the current position
        #[prost(string, tag = "1")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Rebalances current assets into a new asset position.
    ///
    /// Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`
    ///
    /// This function is based on the Curve Pool Registry exchange_multiple() function:
    /// https://github.com/curvefi/curve-pool-registry/blob/16a8664952cf61d7fed06acca79ad5ac696f4b20/contracts/Swaps.vy#L461-L489
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Rebalance {
        /// array of [initial token, pool, token, pool, token, ...] that specifies the swap route on Curve.
        #[prost(string, repeated, tag = "1")]
        pub route: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// An array of up to 4 swap params. Attempting more than four swaps will fail.
        #[prost(message, repeated, tag = "2")]
        pub swap_params: ::prost::alloc::vec::Vec<rebalance::SwapParams>,
        /// Minimum acceptable assets to be received from the swap (slippage parameter).  Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "3")]
        pub min_assets_out: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Rebalance`.
    pub mod rebalance {
        ///
        /// Represents parameters for a single swap. Each swap needs the indeces in Rebalance.route of the in/out token addresses and the swap type. See the Curve contract linked above for more detail.
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
        pub struct SwapParams {
            /// Index in the `route` array of the swap's input token address
            #[prost(uint64, tag = "1")]
            pub in_index: u64,
            /// Index in the `route` array of the swap's output token address
            #[prost(uint64, tag = "2")]
            pub out_index: u64,
            /// 1 - stableswap `exchange`
            /// 2 - stableswap `exchange_underlying`
            /// 3 - cryptoswap `exchange`
            /// 4 - cryptoswap `exchange_underlying`
            /// 5 - Polygon factory metapools `exchange_underlying`
            /// See the Curve Pool Registry exchange_multiple() function for more information.
            #[prost(uint64, tag = "3")]
            pub swap_type: u64,
        }
    }
    ///
    /// Reinvest rewards back into cellar's current position. Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.
    ///
    /// Represents function `reinvest(uint256 minAssetsOut)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Reinvest {
        /// Minimum acceptable assets to be received from the swap (slippage parameter).  Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub min_assets_out: ::prost::alloc::string::String,
    }
    ///
    /// Set the accrual period over which yield is distributed.
    ///
    /// Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetAccrualPeriod {
        #[prost(uint32, tag = "1")]
        pub new_accrual_period: u32,
    }
    ///
    /// Set the per-wallet deposit limit. Uses the same decimals as the current asset.
    ///
    /// Represents function `setDepositLimit(uint256 limit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetDepositLimit {
        /// Amount of assets to set as the new limit. Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.
    ///
    /// Represents function `setLiquidityLimit(uint256 limit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetLiquidityLimit {
        /// Amount of assets to set as the new limit
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Transfer accrued fees to the Sommelier Chain to distribute.
    ///
    /// Represents function `sendFees()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SendFees {}
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `accruePlatformFees()`
        #[prost(message, tag = "1")]
        Accrue(Accrue),
        /// Represents function `claimAndUnstake()`
        #[prost(message, tag = "2")]
        ClaimAndUnstake(ClaimAndUnstake),
        /// Represents function `enterPosition()`
        #[prost(message, tag = "3")]
        EnterPosition(EnterPosition),
        /// Represents function `enterPosition(uint256 assets)`
        #[prost(message, tag = "4")]
        EnterPositionWithAssets(EnterPositionWithAssets),
        /// Represents function `exitPosition()`
        #[prost(message, tag = "5")]
        ExitPosition(ExitPosition),
        /// Represents function `exitPosition(uint256 assets)`
        #[prost(message, tag = "6")]
        ExitPositionWithAssets(ExitPositionWithAssets),
        /// Represents function `rebalance(address newLendingToken, uint256 minNewLendingTokenAmount)`
        #[prost(message, tag = "7")]
        Rebalance(Rebalance),
        /// Represents function `reinvest(uint256 minAssetsOut)`
        #[prost(message, tag = "8")]
        Reinvest(Reinvest),
        /// Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`
        #[prost(message, tag = "9")]
        SetAccrualPeriod(SetAccrualPeriod),
        /// Represents function `setDepositLimit(uint256 limit)`
        #[prost(message, tag = "10")]
        SetDepositLimit(SetDepositLimit),
        /// Represents function `setLiquidityLimit(uint256 limit)`
        #[prost(message, tag = "11")]
        SetLiquidityLimit(SetLiquidityLimit),
        /// Represents function `transferFees()`
        #[prost(message, tag = "12")]
        SendFees(SendFees),
    }
}
///
/// Represents a function call initiated by governance
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveV2StablecoinGovernance {
    /// The function to call on the target cellar
    #[prost(
        oneof = "aave_v2_stablecoin_governance::Function",
        tags = "1, 2, 3, 4, 5"
    )]
    pub function: ::core::option::Option<aave_v2_stablecoin_governance::Function>,
}
/// Nested message and enum types in `AaveV2StablecoinGovernance`.
pub mod aave_v2_stablecoin_governance {
    /// Represents function `setFeesDistributor(bytes32)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetFeesDistributor {
        /// The new fees distributor
        #[prost(string, tag = "1")]
        pub new_fees_distributor: ::prost::alloc::string::String,
    }
    /// Represents function `initiateShutdown(bool)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {
        /// Whether to empty the position
        #[prost(bool, tag = "1")]
        pub empty_position: bool,
    }
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    /// Represents function `setTrust(address, bool)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetTrust {
        /// The position to set trust for
        #[prost(string, tag = "1")]
        pub position: ::prost::alloc::string::String,
        /// Whether to trust the address
        #[prost(bool, tag = "2")]
        pub trust: bool,
    }
    /// Represents function `sweep(address, address)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Sweep {
        /// The address of the ERC20 token to sweep
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The recipient of the sweep
        #[prost(string, tag = "2")]
        pub recipient: ::prost::alloc::string::String,
    }
    /// The function to call on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `setFeesDistributor(bytes32)`
        #[prost(message, tag = "1")]
        SetFeesDistributor(SetFeesDistributor),
        /// Represents function `initiateShutdown(bool)`
        #[prost(message, tag = "2")]
        InitiateShutdown(InitiateShutdown),
        /// Represents function `liftShutdown()`
        #[prost(message, tag = "3")]
        LiftShutdown(LiftShutdown),
        /// Represents function `setTrust(address, bool)`
        #[prost(message, tag = "4")]
        SetTrust(SetTrust),
        /// Represents function `sweep(address, address)`
        #[prost(message, tag = "5")]
        Sweep(Sweep),
    }
}
///
/// Represents a function call to a cellar that implements Cellar.sol
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV1 {
    /// The function you wish to execute on the target cellar
    #[prost(
        oneof = "cellar_v1::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12"
    )]
    pub function: ::core::option::Option<cellar_v1::Function>,
}
/// Nested message and enum types in `CellarV1`.
pub mod cellar_v1 {
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint256 index, address position)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(string, tag = "1")]
        pub index: ::prost::alloc::string::String,
        /// Address of the position to add
        #[prost(string, tag = "2")]
        pub position: ::prost::alloc::string::String,
    }
    ///
    /// Push a trusted position to the end of the list of positions used by the cellar. If you
    ///know you are going to add a position to the end of the array, this is more efficient then
    ///`addPosition`.
    ///
    /// Represents function `pushPosition(address position)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct PushPosition {
        /// Address of the position to push
        #[prost(string, tag = "1")]
        pub position: ::prost::alloc::string::String,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint256 index)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(string, tag = "1")]
        pub index: ::prost::alloc::string::String,
    }
    ///
    /// Set the holding position used by the cellar.
    ///
    /// Represents function `setHoldingPosition(address newHoldingPosition)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// Address of the new holding position to use
        #[prost(string, tag = "1")]
        pub new_holding_position: ::prost::alloc::string::String,
    }
    ///
    /// Represents swap parameters for UniswapV2
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UniV2SwapParams {
        /// Array of addresses dictating what swap path to follow
        #[prost(string, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Amount of the first asset in the path to swap
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
        /// The minimum amount of the last asset in the path to receive
        #[prost(string, tag = "3")]
        pub amount_out_min: ::prost::alloc::string::String,
    }
    ///
    /// Represents swap parameters for UniswapV3
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct UniV3SwapParams {
        /// Array of addresses dictating what swap path to follow
        #[prost(string, repeated, tag = "1")]
        pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Array of pool fees dictating what swap pools to use
        #[prost(uint32, repeated, tag = "2")]
        pub pool_fees: ::prost::alloc::vec::Vec<u32>,
        /// Amount of the first asset in the path to swap
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
        /// The minimum amount of the last asset in the path to receive
        #[prost(string, tag = "4")]
        pub amount_out_min: ::prost::alloc::string::String,
    }
    ///
    /// Represents swap parameters for an exchange
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapParams {
        #[prost(oneof = "swap_params::Params", tags = "1, 2")]
        pub params: ::core::option::Option<swap_params::Params>,
    }
    /// Nested message and enum types in `SwapParams`.
    pub mod swap_params {
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum Params {
            /// Params for a Uniswap V2 swap
            #[prost(message, tag = "1")]
            Univ2Params(super::UniV2SwapParams),
            /// Params for a Uniswap V3 swap
            #[prost(message, tag = "2")]
            Univ3Params(super::UniV3SwapParams),
        }
    }
    ///
    /// Move assets between positions. To move assets from/to this cellar's holdings, specify
    ///the address of this cellar as the `fromPosition`/`toPosition`.
    ///
    /// Represents function `rebalance(address fromPosition, address toPosition,
    ///  uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct Rebalance {
        #[prost(string, tag = "1")]
        pub from_position: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub to_position: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub assets_from: ::prost::alloc::string::String,
        #[prost(enumeration = "Exchange", tag = "4")]
        pub exchange: i32,
        #[prost(message, optional, tag = "5")]
        pub params: ::core::option::Option<SwapParams>,
    }
    ///
    /// Sets the Strategists payout address.
    ///
    /// Represents function `setStrategistPayoutAddress(address payout)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPayoutAddress {
        #[prost(string, tag = "1")]
        pub payout: ::prost::alloc::string::String,
    }
    ///
    /// Set the withdraw type used by the cellar.
    ///
    /// Represents function `setWithdrawType(WithdrawType newWithdrawType)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetWithdrawType {
        /// The withdraw type to use for the cellar
        #[prost(enumeration = "WithdrawType", tag = "1")]
        pub new_withdraw_type: i32,
    }
    ///
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint256 index1, uint256 index2)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(string, tag = "1")]
        pub index_1: ::prost::alloc::string::String,
        /// Index of the second position
        #[prost(string, tag = "2")]
        pub index_2: ::prost::alloc::string::String,
    }
    ///
    /// Set the per-wallet deposit limit. Uses the same decimals as the current asset.
    ///
    /// Represents function `setDepositLimit()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetDepositLimit {
        #[prost(string, tag = "1")]
        pub new_limit: ::prost::alloc::string::String,
    }
    ///
    /// Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.
    ///
    /// Represents function `setLiquidityLimit()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetLiquidityLimit {
        #[prost(string, tag = "1")]
        pub new_limit: ::prost::alloc::string::String,
    }
    ///
    /// Allows share lock period to be updated.
    ///
    /// Represents function `setShareLockPeriod()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetShareLockPeriod {
        #[prost(string, tag = "1")]
        pub new_lock: ::prost::alloc::string::String,
    }
    ///
    ///
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    ///
    /// Exchange selector
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum Exchange {
        Unspecified = 0,
        /// Represents Uniswap V2
        Univ2 = 1,
        /// Represents Uniswap V3
        Univ3 = 2,
    }
    ///
    /// Represents the withdraw type to use for the cellar
    #[derive(
        serde::Deserialize,
        serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[repr(i32)]
    pub enum WithdrawType {
        Unspecified = 0,
        Orderly = 1,
        Proportional = 2,
    }
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `addPosition(uint256 index, address position)`
        #[prost(message, tag = "1")]
        AddPosition(AddPosition),
        /// Represents function `pushPosition(address position)`
        #[prost(message, tag = "2")]
        PushPosition(PushPosition),
        /// Represents function `removePosition(uint256 index)`
        #[prost(message, tag = "3")]
        RemovePosition(RemovePosition),
        /// Represents function `setHoldingPosition(address newHoldingPosition)`
        #[prost(message, tag = "4")]
        SetHoldingPosition(SetHoldingPosition),
        ///
        /// Represents function `rebalance(address fromPosition, address toPosition,
        ///uint256 assetsFrom, SwapRouter.Exchange exchange, bytes calldata params)`
        #[prost(message, tag = "5")]
        Rebalance(Rebalance),
        /// Represents function `setStrategistPayoutAddress(address payout)`
        #[prost(message, tag = "6")]
        SetStrategistPayoutAddress(SetStrategistPayoutAddress),
        /// Represents function `setWithdrawType(WithdrawType newWithdrawType)`
        #[prost(message, tag = "7")]
        SetWithdrawType(SetWithdrawType),
        /// Represents function `swapPositions(uint256 index1, uint256 index2)`
        #[prost(message, tag = "8")]
        SwapPositions(SwapPositions),
        /// Represents function `setDepositLimit()`
        #[prost(message, tag = "9")]
        SetDepositLimit(SetDepositLimit),
        /// Represents function `setLiquidityLimit()`
        #[prost(message, tag = "10")]
        SetLiquidityLimit(SetLiquidityLimit),
        /// Represents function `setShareLockPeriod()`
        #[prost(message, tag = "11")]
        SetShareLockPeriod(SetShareLockPeriod),
        /// Represents function `setRebalanceDeviation(uint265)`
        #[prost(message, tag = "12")]
        SetRebalanceDeviation(SetRebalanceDeviation),
    }
}
///
/// Represent a function call initiated through a governance proposal
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV1Governance {
    /// The function to call on the target cellar
    #[prost(
        oneof = "cellar_v1_governance::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"
    )]
    pub function: ::core::option::Option<cellar_v1_governance::Function>,
}
/// Nested message and enum types in `CellarV1Governance`.
pub mod cellar_v1_governance {
    /// Represents function `initiateShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {}
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    /// Represents function `resetHighWatermark()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ResetHighWatermark {}
    /// Represents function `setFeesDistributor(bytes32)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetFeesDistributor {
        /// Cosmos address of the new fees distributor
        #[prost(string, tag = "1")]
        pub new_fees_distributor: ::prost::alloc::string::String,
    }
    /// Represents function `setOwner(address)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetOwner {
        /// Address of the new owner
        #[prost(string, tag = "1")]
        pub new_owner: ::prost::alloc::string::String,
    }
    /// Represents function `setPerformanceFee(uint64)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetPerformanceFee {
        /// New performance fee
        #[prost(uint64, tag = "1")]
        pub amount: u64,
    }
    /// Represents function `setPlatformFee(uint64)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetPlatformFee {
        /// New platform fee
        #[prost(uint64, tag = "1")]
        pub amount: u64,
    }
    /// Represents function `setStrategistPerformanceCut(uint64)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPerformanceCut {
        /// New strategist performance cut
        #[prost(uint64, tag = "1")]
        pub amount: u64,
    }
    /// Represents function `setStrategistPlatformCut(uint64)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetStrategistPlatformCut {
        /// New strategist platform cut
        #[prost(uint64, tag = "1")]
        pub amount: u64,
    }
    /// Represents function `trustPosition(address)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct TrustPosition {
        #[prost(oneof = "trust_position::Position", tags = "1, 2, 3")]
        pub position: ::core::option::Option<trust_position::Position>,
    }
    /// Nested message and enum types in `TrustPosition`.
    pub mod trust_position {
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum Position {
            #[prost(string, tag = "1")]
            Erc20Address(::prost::alloc::string::String),
            #[prost(string, tag = "2")]
            Erc4626Address(::prost::alloc::string::String),
            #[prost(string, tag = "3")]
            CellarAddress(::prost::alloc::string::String),
        }
    }
    /// The function to call on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `initiateShutdown()`
        #[prost(message, tag = "1")]
        InitiateShutdown(InitiateShutdown),
        /// Represents function `liftShutdown()`
        #[prost(message, tag = "2")]
        LiftShutdown(LiftShutdown),
        /// Represents function `resetHighWatermark()`
        #[prost(message, tag = "3")]
        ResetHighWatermark(ResetHighWatermark),
        /// Represents function `setFeesDistributor(address)`
        #[prost(message, tag = "4")]
        SetFeesDistributor(SetFeesDistributor),
        /// Represents function `setOwner(address)`
        #[prost(message, tag = "5")]
        SetOwner(SetOwner),
        /// Represents function `setPerformanceFee(uint256)`
        #[prost(message, tag = "6")]
        SetPerformanceFee(SetPerformanceFee),
        /// Represents function `setPlatformFee(uint256)`
        #[prost(message, tag = "7")]
        SetPlatformFee(SetPlatformFee),
        /// Represents function `setStrategistPerformanceCut(uint256)`
        #[prost(message, tag = "8")]
        SetStrategistPerformanceCut(SetStrategistPerformanceCut),
        /// Represents function `setStrategistPlatformCut(address)`
        #[prost(message, tag = "9")]
        SetStrategistPlatformCut(SetStrategistPlatformCut),
        /// Represents function `trustPosition(address)`
        #[prost(message, tag = "10")]
        TrustPosition(TrustPosition),
    }
}
///
/// Represents a governance-executed cellar function call. Used for Scheduled Cork Proposals in Sommelier.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct GovernanceCall {
    /// The type of Cellar to call
    #[prost(oneof = "governance_call::Call", tags = "2, 3")]
    pub call: ::core::option::Option<governance_call::Call>,
}
/// Nested message and enum types in `GovernanceCall`.
pub mod governance_call {
    /// The type of Cellar to call
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Call {
        /// Governance function calls to the AaveV2Stablecoin cellar
        #[prost(message, tag = "2")]
        AaveV2Stablecoin(super::AaveV2StablecoinGovernance),
        /// Governance function calls to V1 cellars
        #[prost(message, tag = "3")]
        CellarV1(super::CellarV1Governance),
    }
}
///
/// Represents a single, scheduled function call to a particular Cellar
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduleRequest {
    /// The ID (currently simply an Ethereum address) of the target Cellar
    #[prost(string, tag = "1")]
    pub cellar_id: ::prost::alloc::string::String,
    /// The block height at which to schedule the contract call
    #[prost(uint64, tag = "4")]
    pub block_height: u64,
    /// The data from which the desired contract function will be encoded
    #[prost(oneof = "schedule_request::CallData", tags = "2, 3")]
    pub call_data: ::core::option::Option<schedule_request::CallData>,
}
/// Nested message and enum types in `ScheduleRequest`.
pub mod schedule_request {
    /// The data from which the desired contract function will be encoded
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallData {
        #[prost(message, tag = "2")]
        AaveV2Stablecoin(super::AaveV2Stablecoin),
        #[prost(message, tag = "3")]
        CellarV1(super::CellarV1),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduleResponse {}
#[doc = r" Generated client implementations."]
pub mod contract_call_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = ""]
    #[doc = " Service for handling Cellar contract calls"]
    pub struct ContractCallClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContractCallClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ContractCallClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Handles scheduled contract call submission"]
        pub async fn schedule(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleRequest>,
        ) -> Result<tonic::Response<super::ScheduleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/steward.v2.ContractCall/Schedule");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ContractCallClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ContractCallClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ContractCallClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod contract_call_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ContractCallServer."]
    #[async_trait]
    pub trait ContractCall: Send + Sync + 'static {
        #[doc = " Handles scheduled contract call submission"]
        async fn schedule(
            &self,
            request: tonic::Request<super::ScheduleRequest>,
        ) -> Result<tonic::Response<super::ScheduleResponse>, tonic::Status>;
    }
    #[doc = ""]
    #[doc = " Service for handling Cellar contract calls"]
    #[derive(Debug)]
    pub struct ContractCallServer<T: ContractCall> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ContractCall> ContractCallServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ContractCallServer<T>
    where
        T: ContractCall,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/steward.v2.ContractCall/Schedule" => {
                    #[allow(non_camel_case_types)]
                    struct ScheduleSvc<T: ContractCall>(pub Arc<T>);
                    impl<T: ContractCall> tonic::server::UnaryService<super::ScheduleRequest> for ScheduleSvc<T> {
                        type Response = super::ScheduleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScheduleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).schedule(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ScheduleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ContractCall> Clone for ContractCallServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ContractCall> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ContractCall> tonic::transport::NamedService for ContractCallServer<T> {
        const NAME: &'static str = "steward.v2.ContractCall";
    }
}
