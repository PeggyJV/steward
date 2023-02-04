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
/// Represents oracle swap parameters for UniswapV2
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV2OracleSwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///
/// Represents oracle swap parameters for UniswapV3
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniV3OracleSwapParams {
    /// Array of addresses dictating what swap path to follow
    #[prost(string, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Array of pool fees dictating what swap pools to use
    #[prost(uint32, repeated, tag = "2")]
    pub pool_fees: ::prost::alloc::vec::Vec<u32>,
}
///
/// Represents swap params for BaseAdaptor.oracleSwap()
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OracleSwapParams {
    #[prost(oneof = "oracle_swap_params::Params", tags = "1, 2")]
    pub params: ::core::option::Option<oracle_swap_params::Params>,
}
/// Nested message and enum types in `OracleSwapParams`.
pub mod oracle_swap_params {
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        #[prost(message, tag = "1")]
        Univ2Params(super::UniV2OracleSwapParams),
        #[prost(message, tag = "2")]
        Univ3Params(super::UniV3OracleSwapParams),
    }
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
/// Helper function that allows swaps using the Swap Router
///
/// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    /// Asset to swap from
    #[prost(string, tag = "1")]
    pub asset_in: ::prost::alloc::string::String,
    /// Asset to swap to
    #[prost(string, tag = "2")]
    pub asset_out: ::prost::alloc::string::String,
    /// Amount to swap
    #[prost(string, tag = "3")]
    pub amount_in: ::prost::alloc::string::String,
    /// The exchange to make the swap on
    #[prost(enumeration = "Exchange", tag = "4")]
    pub exchange: i32,
    /// The parameters for the swap
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<SwapParams>,
}
///
/// Helper function to make safe "blind" Uniswap Swaps by comparing value in vs value out of the swap.
///
/// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct OracleSwap {
    /// Asset to swap from
    #[prost(string, tag = "1")]
    pub asset_in: ::prost::alloc::string::String,
    /// Asset to swap to
    #[prost(string, tag = "2")]
    pub asset_out: ::prost::alloc::string::String,
    /// Amount to swap
    #[prost(string, tag = "3")]
    pub amount_in: ::prost::alloc::string::String,
    /// The exchange to make the swap on
    #[prost(enumeration = "Exchange", tag = "4")]
    pub exchange: i32,
    /// The parameters for the swap
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<OracleSwapParams>,
    /// The slippage allowed for the swap
    #[prost(uint64, tag = "6")]
    pub slippage: u64,
}
///
/// Allows strategists to zero out an approval for a given `asset`.
///
/// Represents function `revokeApproval(ERC20 asset, address spender)`
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct RevokeApproval {
    /// ERC20 Asset to revoke spender's approval for
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// The spender to revoke approval of asset for
    #[prost(string, tag = "2")]
    pub spender: ::prost::alloc::string::String,
}
/// Represents call data for the Uniswap V3 adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3Adaptor {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "uniswap_v3_adaptor::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8"
    )]
    pub function: ::core::option::Option<uniswap_v3_adaptor::Function>,
}
/// Nested message and enum types in `UniswapV3Adaptor`.
pub mod uniswap_v3_adaptor {
    ///
    /// Allows strategist to open up arbritray Uniswap V3 positions.
    ///
    /// Represents function openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct OpenPosition {
        #[prost(string, tag = "1")]
        pub token_0: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub token_1: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub pool_fee: u32,
        #[prost(string, tag = "4")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(int32, tag = "8")]
        pub tick_lower: i32,
        #[prost(int32, tag = "9")]
        pub tick_upper: i32,
    }
    ///
    /// Allows strategist to close Uniswap V3 positions.
    ///
    /// Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClosePosition {
        #[prost(string, tag = "1")]
        pub position_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to add to existing Uniswap V3 positions.
    ///
    /// Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddToPosition {
        #[prost(string, tag = "1")]
        pub position_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub min_1: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategist to take from existing Uniswap V3 positions.
    ///
    /// Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct TakeFromPosition {
        #[prost(string, tag = "1")]
        pub position_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub liquidity: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub min_0: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub min_1: ::prost::alloc::string::String,
        #[prost(bool, tag = "5")]
        pub collect_fees: bool,
    }
    ///
    /// Allows strategist to collect fees from existing Uniswap V3 positions.
    ///
    /// Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CollectFees {
        #[prost(string, tag = "1")]
        pub position_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_0: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount_1: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `openPosition(ERC20 token0, ERC20 token1, uint24 poolFee, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1, int24 tickLower, int24 tickUpper)`
        #[prost(message, tag = "4")]
        OpenPosition(OpenPosition),
        /// Represents function `closePosition(uint256 positionId, uint256 min0, uint256 min1)`
        #[prost(message, tag = "5")]
        ClosePosition(ClosePosition),
        /// Represents function `addToPosition(uint256 positionId, uint256 amount0, uint256 amount1, uint256 min0, uint256 min1)`
        #[prost(message, tag = "6")]
        AddToPosition(AddToPosition),
        /// Represents function `takeFromPosition(uint256 positionId, uint128 liquidity, uint256 min0, uint256 min1, bool collectFees)`
        #[prost(message, tag = "7")]
        TakeFromPosition(TakeFromPosition),
        /// Represents function `collectFees(uint256 positionId, uint128 amount0, uint128 amount1)`
        #[prost(message, tag = "8")]
        CollectFees(CollectFees),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct UniswapV3AdaptorCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<UniswapV3Adaptor>,
}
/// Represents call data for the Vesting Simple adaptor
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VestingSimpleAdaptor {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "vesting_simple_adaptor::Function",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
    pub function: ::core::option::Option<vesting_simple_adaptor::Function>,
}
/// Nested message and enum types in `VestingSimpleAdaptor`.
pub mod vesting_simple_adaptor {
    ///
    /// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
    /// deposit its entire balance (appropriate in most cases).
    ///
    /// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to deposit tokens to the vesting contract. By passing a max uint256 for amountToDeposit, the cellar will
    /// deposit its entire balance (appropriate in most cases).
    ///
    /// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub deposit_id: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Withdraw a single deposit from vesting. This will not affect the cellar's TVL because any deposit must already have vested, and
    /// will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
    ///
    /// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawAnyFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Withdraw a certain amount of tokens from vesting, from any deposit. This will not affect the cellar's TVL because any deposit must
    /// already have vested, and will be reported in balanceOf. Will revert if not enough tokens are available based on amountToWithdraw.
    ///
    /// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawAllFromVesting {
        #[prost(string, tag = "1")]
        pub vesting_contract: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToVesting(VestingSimple vestingContract, uint256 amountToDeposit)`
        #[prost(message, tag = "4")]
        DepositToVesting(DepositToVesting),
        /// Represents function `withdrawFromVesting(VestingSimple vestingContract, uint256 depositId, uint256 amountToWithdraw)`
        #[prost(message, tag = "5")]
        WithdrawFromVesting(WithdrawFromVesting),
        /// Represents function `withdrawAnyFromVesting(VestingSimple vestingContract, uint256 amountToWithdraw)`
        #[prost(message, tag = "6")]
        WithdrawAnyFromVesting(WithdrawAnyFromVesting),
        /// Represents function `withdrawAllFromVesting(VestingSimple vestingContract)`
        #[prost(message, tag = "7")]
        WithdrawAllFromVesting(WithdrawAllFromVesting),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct VestingSimpleAdaptorCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<VestingSimpleAdaptor>,
}
/// Represents call data for the Aave Debt Token adaptor, used for borrowing and repaying debt on Aave.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptor {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_debt_token_adaptor::Function", tags = "1, 2, 3, 4, 5, 6")]
    pub function: ::core::option::Option<aave_debt_token_adaptor::Function>,
}
/// Nested message and enum types in `AaveDebtTokenAdaptor`.
pub mod aave_debt_token_adaptor {
    ///
    /// Allows strategists to borrow assets from Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct BorrowFromAave {
        /// The address of the ERC20 token to borrow
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to borrow
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to repay loan debt on Aave.
    ///
    /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RepayAaveDebt {
        /// The address of the ERC20 token to repay
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to repay
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to swap assets and repay loans in one call.
    ///
    /// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapAndRepay {
        /// The address of the token to swap from
        #[prost(string, tag = "1")]
        pub token_in: ::prost::alloc::string::String,
        /// The address of the token to swap to and repay with
        #[prost(string, tag = "2")]
        pub token_to_repay: ::prost::alloc::string::String,
        /// The amount to swap
        #[prost(string, tag = "3")]
        pub amount_in: ::prost::alloc::string::String,
        /// The exchange to make the swap on
        #[prost(enumeration = "super::Exchange", tag = "4")]
        pub exchange: i32,
        /// The parameters for the swap
        #[prost(message, optional, tag = "5")]
        pub params: ::core::option::Option<super::SwapParams>,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `borrowFromAave(ERC20 debtTokenToBorrow, uint256 amountToBorrow)`
        #[prost(message, tag = "4")]
        BorrowFromAave(BorrowFromAave),
        /// Represents function `repayAaveDebt(ERC20 tokenToRepay, uint256 amountToRepay)`
        #[prost(message, tag = "5")]
        RepayAaveDebt(RepayAaveDebt),
        /// Represents function `swapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "6")]
        SwapAndRepay(SwapAndRepay),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveDebtTokenAdaptorCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveDebtTokenAdaptor>,
}
/// Represents call data for the Aave AToken adaptor, used to manage lending positions on Aave
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptor {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(oneof = "aave_a_token_adaptor::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<aave_a_token_adaptor::Function>,
}
/// Nested message and enum types in `AaveATokenAdaptor`.
pub mod aave_a_token_adaptor {
    ///
    /// Allows strategists to lend assets on Aave.
    ///
    /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToAave {
        /// The address of the ERC20 token to deposit
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to deposit
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Aave.
    ///
    /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromAave {
        /// The address of the ERC20 token to withdraw
        #[prost(string, tag = "1")]
        pub token: ::prost::alloc::string::String,
        /// The amount to withdraw
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToAave(ERC20 tokenToDeposit, uint256 amountToDeposit)`
        #[prost(message, tag = "4")]
        DepositToAave(DepositToAave),
        /// Represents function `withdrawFromAave(ERC20 tokenToWithdraw, uint256 amountToWithdraw)`
        #[prost(message, tag = "5")]
        WithdrawFromAave(WithdrawFromAave),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AaveATokenAdaptorCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<AaveATokenAdaptor>,
}
/// Represents call data for the Compound C Token adaptor, managing lending positions on Compound.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CompoundCTokenAdaptor {
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[prost(
        oneof = "compound_c_token_adaptor::Function",
        tags = "1, 2, 3, 4, 5, 6, 7"
    )]
    pub function: ::core::option::Option<compound_c_token_adaptor::Function>,
}
/// Nested message and enum types in `CompoundCTokenAdaptor`.
pub mod compound_c_token_adaptor {
    ///
    /// Allows strategists to lend assets on Compound.
    ///
    /// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct DepositToCompound {
        #[prost(string, tag = "1")]
        pub market: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_to_deposit: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to withdraw assets from Compound.
    ///
    /// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct WithdrawFromCompound {
        #[prost(string, tag = "1")]
        pub market: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount_to_withdraw: ::prost::alloc::string::String,
    }
    ///
    /// Allows strategists to claim COMP rewards.
    ///
    /// Represents function `claimComp()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimComp {}
    ///
    /// Allows strategists to claim COMP and immediately swap it using oracleSwap.
    ///
    /// Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct ClaimCompAndSwap {
        #[prost(string, tag = "1")]
        pub asset_out: ::prost::alloc::string::String,
        #[prost(enumeration = "super::Exchange", tag = "2")]
        pub exchange: i32,
        #[prost(message, optional, tag = "3")]
        pub params: ::core::option::Option<super::OracleSwapParams>,
        #[prost(uint64, tag = "4")]
        pub slippage: u64,
    }
    ///**** BASE ADAPTOR FUNCTIONS ****
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `swap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params)`
        #[prost(message, tag = "1")]
        Swap(super::Swap),
        /// Represents function `oracleSwap(ERC20 assetIn, ERC20 assetOut, uint256 amountIn, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "2")]
        OracleSwap(super::OracleSwap),
        /// Represents function `revokeApproval(ERC20 asset, address spender)`
        #[prost(message, tag = "3")]
        RevokeApproval(super::RevokeApproval),
        //**** ADAPTOR-SPECIFIC FUNCTIONS ****
        /// Represents function `depositToCompound(CErc20 market, uint256 amountToDeposit)`
        #[prost(message, tag = "4")]
        DepositToCompound(DepositToCompound),
        /// Represents function `withdrawFromCompound(CErc20 market, uint256 amountToWithdraw)`
        #[prost(message, tag = "5")]
        WithdrawFromCompound(WithdrawFromCompound),
        /// Represents function `claimComp()`
        #[prost(message, tag = "6")]
        ClaimComp(ClaimComp),
        /// Represents function `claimCompAndSwap(ERC20 assetOut, SwapRouter.Exchange exchange, bytes memory params, uint64 slippage)`
        #[prost(message, tag = "7")]
        ClaimCompAndSwap(ClaimCompAndSwap),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CompoundCTokenAdaptorCalls {
    #[prost(message, repeated, tag = "1")]
    pub calls: ::prost::alloc::vec::Vec<CompoundCTokenAdaptor>,
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
        #[prost(enumeration = "super::Exchange", tag = "4")]
        pub exchange: i32,
        #[prost(message, optional, tag = "5")]
        pub params: ::core::option::Option<super::SwapParams>,
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
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9"
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
        /// Represents function `setPerformanceFee(uint256)`
        #[prost(message, tag = "5")]
        SetPerformanceFee(SetPerformanceFee),
        /// Represents function `setPlatformFee(uint256)`
        #[prost(message, tag = "6")]
        SetPlatformFee(SetPlatformFee),
        /// Represents function `setStrategistPerformanceCut(uint256)`
        #[prost(message, tag = "7")]
        SetStrategistPerformanceCut(SetStrategistPerformanceCut),
        /// Represents function `setStrategistPlatformCut(address)`
        #[prost(message, tag = "8")]
        SetStrategistPlatformCut(SetStrategistPlatformCut),
        /// Represents function `trustPosition(address)`
        #[prost(message, tag = "9")]
        TrustPosition(TrustPosition),
    }
}
///
/// Represents a function call to a cellar that implements Cellar.sol
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV2 {
    /// The function you wish to execute on the target cellar
    #[prost(oneof = "cellar_v2::Function", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub function: ::core::option::Option<cellar_v2::Function>,
}
/// Nested message and enum types in `CellarV2`.
pub mod cellar_v2 {
    ///
    /// Insert a trusted position to the list of positions used by the cellar at a given index.
    ///
    /// Represents function `addPosition(uint32 index, uint32 positionId, bytes configurationData, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AddPosition {
        /// Index at which to add the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// The position's ID in the cellar registry
        #[prost(uint32, tag = "2")]
        pub position_id: u32,
        /// Data used to configure how the position behaves
        #[prost(bytes = "vec", tag = "3")]
        pub configuration_data: ::prost::alloc::vec::Vec<u8>,
        /// Whether to add position in the debt array, or the credit array.
        #[prost(bool, tag = "4")]
        pub in_debt_array: bool,
    }
    /// Represents a call to adaptor an. The cellar must be authorized to call the target adaptor.
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AdaptorCall {
        /// Address of the adaptor
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
        /// The function call data for the adaptor
        #[prost(oneof = "adaptor_call::CallData", tags = "2, 3, 4, 5, 6")]
        pub call_data: ::core::option::Option<adaptor_call::CallData>,
    }
    /// Nested message and enum types in `AdaptorCall`.
    pub mod adaptor_call {
        /// The function call data for the adaptor
        #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
        pub enum CallData {
            /// Represents function calls to the UniswapV3Adaptor
            #[prost(message, tag = "2")]
            UniswapV3Calls(super::super::UniswapV3AdaptorCalls),
            /// Represents function calls to the AaveATokenAdaptor
            #[prost(message, tag = "3")]
            AaveATokenCalls(super::super::AaveATokenAdaptorCalls),
            /// Represents function calls to the AavaDebtTokenAdaptor
            #[prost(message, tag = "4")]
            AaveDebtTokenCalls(super::super::AaveDebtTokenAdaptorCalls),
            /// Represents function calls to the CompoundCTokenAdaptor
            #[prost(message, tag = "5")]
            CompoundCTokenCalls(super::super::CompoundCTokenAdaptorCalls),
            /// Represents function calls to the VestingSimpleAdaptor
            #[prost(message, tag = "6")]
            VestingSimpleCalls(super::super::VestingSimpleAdaptorCalls),
        }
    }
    ///
    /// Allows strategists to manage their Cellar using arbitrary logic calls to adaptors.
    ///
    /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct CallOnAdaptor {
        #[prost(message, repeated, tag = "1")]
        pub data: ::prost::alloc::vec::Vec<AdaptorCall>,
    }
    ///
    /// Remove the position at a given index from the list of positions used by the cellar.
    ///
    /// Represents function `removePosition(uint32 index, bool inDebtArray)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct RemovePosition {
        /// Index at which to remove the position
        #[prost(uint32, tag = "1")]
        pub index: u32,
        /// Whether to remove position from the debt array, or the credit array.
        #[prost(bool, tag = "2")]
        pub in_debt_array: bool,
    }
    ///
    /// Set the holding position used of the cellar.
    ///
    /// Represents function `setHoldingIndex(uint8 index)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetHoldingPosition {
        /// ID (index) of the new holding position to use
        #[prost(uint32, tag = "1")]
        pub position_id: u32,
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
    /// Swap the positions at two given indeces.
    ///
    /// Represents function `swapPositions(uint32 index1, uint32 index2)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SwapPositions {
        /// Index of the first position
        #[prost(uint32, tag = "1")]
        pub index_1: u32,
        /// Index of the second position
        #[prost(uint32, tag = "2")]
        pub index_2: u32,
        /// Whether to switch positions in the debt array, or the credit array.
        #[prost(bool, tag = "3")]
        pub in_debt_array: bool,
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
    /// Changes the cellar's allowed rebalance deviation, which is the percent the total assets of a cellar may deviate
    /// during a `callOnAdaptor`(rebalance) call. The maximum allowed deviation is 100000000000000000 (0.1e18), or 10%.
    ///
    /// Represents function `setRebalanceDeviation(uint256)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetRebalanceDeviation {
        #[prost(string, tag = "1")]
        pub new_deviation: ::prost::alloc::string::String,
    }
    /// The function you wish to execute on the target cellar
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        /// Represents function `addPosition(uint256 index, address position)`
        #[prost(message, tag = "1")]
        AddPosition(AddPosition),
        /// Represents function `callOnAdaptor(AdaptorCall[] memory data)`
        #[prost(message, tag = "2")]
        CallOnAdaptor(CallOnAdaptor),
        /// Represents function `removePosition(uint256 index)`
        #[prost(message, tag = "3")]
        RemovePosition(RemovePosition),
        /// Represents function `setHoldingPosition(uint32 position_id)`
        #[prost(message, tag = "4")]
        SetHoldingPosition(SetHoldingPosition),
        /// Represents function `setStrategistPayoutAddress(address payout)`
        #[prost(message, tag = "5")]
        SetStrategistPayoutAddress(SetStrategistPayoutAddress),
        /// Represents function `swapPositions(uint256 index1, uint256 index2)`
        #[prost(message, tag = "6")]
        SwapPositions(SwapPositions),
        /// Represents function `setRebalanceDeviation(uint265)`
        #[prost(message, tag = "7")]
        SetRebalanceDeviation(SetRebalanceDeviation),
        /// Represents function `setShareLockPeriod(uint256 newLock)`
        #[prost(message, tag = "8")]
        SetShareLockPeriod(SetShareLockPeriod),
    }
}
///
/// Represent a function call initiated through a governance proposal
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarV2Governance {
    /// The function to call on the target cellar
    #[prost(oneof = "cellar_v2_governance::Function", tags = "1, 2, 3, 4, 5")]
    pub function: ::core::option::Option<cellar_v2_governance::Function>,
}
/// Nested message and enum types in `CellarV2Governance`.
pub mod cellar_v2_governance {
    /// Represents function `initiateShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct InitiateShutdown {}
    /// Represents function `liftShutdown()`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct LiftShutdown {}
    /// Represents function `setPlatformFee(uint64)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetPlatformFee {
        /// New platform fee
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
    ///
    /// Allows owner to add new adaptors for the cellar to use.
    ///
    /// Represents function `setupAdaptor(address adaptor)`
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct SetupAdaptor {
        /// Address of the adaptor
        #[prost(string, tag = "1")]
        pub adaptor: ::prost::alloc::string::String,
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
        /// Represents function `setPlatformFee(uint256)`
        #[prost(message, tag = "3")]
        SetPlatformFee(SetPlatformFee),
        /// Represents function `setStrategistPlatformCut(address)`
        #[prost(message, tag = "4")]
        SetStrategistPlatformCut(SetStrategistPlatformCut),
        /// Represents function `setupAdaptor(address adaptor)`
        #[prost(message, tag = "5")]
        SetupAdaptor(SetupAdaptor),
    }
}
///
/// Represents a governance-executed cellar function call. Used for Scheduled Cork Proposals in Sommelier.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct GovernanceCall {
    /// The type of Cellar to call
    #[prost(oneof = "governance_call::Call", tags = "2, 3, 4")]
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
        /// Governance function calls to V2 cellars
        #[prost(message, tag = "4")]
        CellarV2(super::CellarV2Governance),
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
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    /// The data from which the desired contract function will be encoded
    #[prost(oneof = "schedule_request::CallData", tags = "3, 4, 5")]
    pub call_data: ::core::option::Option<schedule_request::CallData>,
}
/// Nested message and enum types in `ScheduleRequest`.
pub mod schedule_request {
    /// The data from which the desired contract function will be encoded
    #[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Oneof)]
    pub enum CallData {
        #[prost(message, tag = "3")]
        AaveV2Stablecoin(super::AaveV2Stablecoin),
        #[prost(message, tag = "4")]
        CellarV1(super::CellarV1),
        #[prost(message, tag = "5")]
        CellarV2(super::CellarV2),
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduleResponse {
    /// The hex encoded ID of the scheduled cork
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
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
            let path = http::uri::PathAndQuery::from_static("/steward.v3.ContractCall/Schedule");
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
                "/steward.v3.ContractCall/Schedule" => {
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
        const NAME: &'static str = "steward.v3.ContractCall";
    }
}
