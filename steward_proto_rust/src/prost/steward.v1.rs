///
/// Represents a function call to the Aave V2 Stablecoin cellar
#[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Accrue {}
    ///
    /// Claim rewards from Aave and begin cooldown period to unstake them.
    ///
    /// Represents function `claimAndUnstake()`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClaimAndUnstake {}
    ///
    /// Pushes total assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnterPosition {}
    ///
    /// Pushes assets into the current Aave lending position.
    ///
    /// Represents function `enterPosition(uint256 assets)`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnterPositionWithAssets {
        /// amount of assets to enter into the current position
        #[prost(string, tag = "1")]
        pub assets: ::prost::alloc::string::String,
    }
    ///
    /// Pulls total assets from the current Aave lending position.
    ///
    /// Represents function `enterPosition()`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExitPosition {}
    ///
    /// Pulls assets from the current Aave lending position.
    ///
    /// Represents function `exitPosition(uint256 assets)`
    #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(Clone, PartialEq, ::prost::Message)]
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
        #[derive(Clone, PartialEq, ::prost::Message)]
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
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reinvest {
        /// Minimum acceptable assets to be received from the swap (slippage parameter).  Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub min_assets_out: ::prost::alloc::string::String,
    }
    ///
    /// Set the accrual period over which yield is distributed.
    ///
    /// Represents function `setAccrualPeriod(uint32 newAccrualPeriod)`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetAccrualPeriod {
        #[prost(uint32, tag = "1")]
        pub new_accrual_period: u32,
    }
    ///
    /// Set the per-wallet deposit limit. Uses the same decimals as the current asset.
    ///
    /// Represents function `setDepositLimit(uint256 limit)`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetDepositLimit {
        /// Amount of assets to set as the new limit. Must be parsable as an unsigned 256-bit integer.
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Set the maximum liquidity that cellar can manage. Uses the same decimals as the current asset.
    ///
    /// Represents function `setLiquidityLimit(uint256 limit)`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetLiquidityLimit {
        /// Amount of assets to set as the new limit
        #[prost(string, tag = "1")]
        pub limit: ::prost::alloc::string::String,
    }
    ///
    /// Transfer accrued fees to the Sommelier Chain to distribute.
    ///
    /// Represents function `sendFees()`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendFees {}
    /// The function you wish to execute on the target cellar
    #[derive(Clone, PartialEq, ::prost::Oneof)]
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
/// Represents a single function call on a particular Cellar
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitRequest {
    /// The ID (currently simply an Ethereum address) of the target Cellar
    #[prost(string, tag = "1")]
    pub cellar_id: ::prost::alloc::string::String,
    /// The data from which the desired contract function will be encoded
    #[prost(oneof = "submit_request::CallData", tags = "2")]
    pub call_data: ::core::option::Option<submit_request::CallData>,
}
/// Nested message and enum types in `SubmitRequest`.
pub mod submit_request {
    /// The data from which the desired contract function will be encoded
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CallData {
        #[prost(message, tag = "2")]
        AaveV2Stablecoin(super::AaveV2Stablecoin),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitResponse {}
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
        #[doc = " Handles simple contract call submission"]
        pub async fn submit(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitRequest>,
        ) -> Result<tonic::Response<super::SubmitResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/steward.v2.ContractCall/Submit");
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
        #[doc = " Handles simple contract call submission"]
        async fn submit(
            &self,
            request: tonic::Request<super::SubmitRequest>,
        ) -> Result<tonic::Response<super::SubmitResponse>, tonic::Status>;
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
                "/steward.v2.ContractCall/Submit" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitSvc<T: ContractCall>(pub Arc<T>);
                    impl<T: ContractCall> tonic::server::UnaryService<super::SubmitRequest> for SubmitSvc<T> {
                        type Response = super::SubmitResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).submit(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SubmitSvc(inner);
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
