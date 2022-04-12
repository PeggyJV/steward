#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AaveV2Stablecoin {
    #[prost(
        oneof = "aave_v2_stablecoin::Function",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub function: ::core::option::Option<aave_v2_stablecoin::Function>,
}
/// Nested message and enum types in `AaveV2Stablecoin`.
pub mod aave_v2_stablecoin {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Function {
        #[prost(message, tag = "1")]
        EnterStrategy(super::EnterStrategy),
        #[prost(message, tag = "2")]
        ReinvestAmount(super::ReinvestWithAmount),
        #[prost(message, tag = "3")]
        Reinvest(super::Reinvest),
        #[prost(message, tag = "4")]
        ClaimAndUnstakeAmount(super::ClaimAndUnstakeWithAmount),
        #[prost(message, tag = "5")]
        ClaimAndUnstake(super::ClaimAndUnstake),
        #[prost(message, tag = "6")]
        Rebalance(super::Rebalance),
        #[prost(message, tag = "7")]
        AccruePlatformFees(super::AccruePlatformFees),
        #[prost(message, tag = "8")]
        TransferFees(super::TransferFees),
        #[prost(message, tag = "9")]
        SetInputToken(super::SetInputToken),
        #[prost(message, tag = "10")]
        RemoveLiquidityRestriction(super::RemoveLiquidityRestriction),
        #[prost(message, tag = "11")]
        Sweep(super::Sweep),
    }
}
/// function enterStrategy() external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnterStrategy {}
/// function reinvest(uint256 amount, uint256 minAssetsOut) external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReinvestWithAmount {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
    #[prost(uint64, tag = "2")]
    pub min_assets_out: u64,
}
/// function reinvest(uint256 minAssetsOut) external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reinvest {
    #[prost(uint64, tag = "1")]
    pub min_assets_out: u64,
}
/// function claimAndUnstake(uint256 amount) external returns (uint256 claimed);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimAndUnstakeWithAmount {
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// function claimAndUnstake() external returns (uint256);
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimAndUnstake {}
/// function rebalance(address newLendingToken, uint256 minNewLendingTokenAmount) external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rebalance {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub min_new_lending_token_amount: u64,
}
/// function accruePlatformFees() external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccruePlatformFees {}
/// function transferFees() external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferFees {}
/// function setInputToken(address token, bool isApproved) external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInputToken {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_approved: bool,
}
/// function removeLiquidityRestriction() external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLiquidityRestriction {}
/// function sweep(address token) external;
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sweep {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitRequest {
    #[prost(string, tag = "1")]
    pub cellar_id: ::prost::alloc::string::String,
    #[prost(oneof = "submit_request::CallData", tags = "2")]
    pub call_data: ::core::option::Option<submit_request::CallData>,
}
/// Nested message and enum types in `SubmitRequest`.
pub mod submit_request {
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
            let path = http::uri::PathAndQuery::from_static("/steward.v1.ContractCall/Submit");
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
        async fn submit(
            &self,
            request: tonic::Request<super::SubmitRequest>,
        ) -> Result<tonic::Response<super::SubmitResponse>, tonic::Status>;
    }
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
                "/steward.v1.ContractCall/Submit" => {
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
        const NAME: &'static str = "steward.v1.ContractCall";
    }
}
