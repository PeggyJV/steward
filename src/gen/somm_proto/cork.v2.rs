#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct Cork {
    /// call body containing the ABI encoded bytes to send to the contract
    #[prost(bytes = "vec", tag = "1")]
    pub encoded_contract_call: ::prost::alloc::vec::Vec<u8>,
    /// address of the contract to send the call
    #[prost(string, tag = "2")]
    pub target_contract_address: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduledCork {
    #[prost(message, optional, tag = "1")]
    pub cork: ::core::option::Option<Cork>,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(string, tag = "3")]
    pub validator: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CorkResult {
    #[prost(message, optional, tag = "1")]
    pub cork: ::core::option::Option<Cork>,
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    #[prost(bool, tag = "3")]
    pub approved: bool,
    #[prost(string, tag = "4")]
    pub approval_percentage: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CellarIdSet {
    #[prost(string, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgScheduleCorkRequest - sdk.Msg for scheduling a cork request for on or after a specific block height
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgScheduleCorkRequest {
    /// the scheduled cork
    #[prost(message, optional, tag = "1")]
    pub cork: ::core::option::Option<Cork>,
    /// the block height that must be reached
    #[prost(uint64, tag = "2")]
    pub block_height: u64,
    /// signer account address
    #[prost(string, tag = "3")]
    pub signer: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgScheduleCorkResponse {
    /// cork ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " MsgService defines the msgs that the cork module handles"]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        pub async fn schedule_cork(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgScheduleCorkRequest>,
        ) -> Result<tonic::Response<super::MsgScheduleCorkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Msg/ScheduleCork");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MsgClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MsgClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MsgClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MsgServer."]
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        async fn schedule_cork(
            &self,
            request: tonic::Request<super::MsgScheduleCorkRequest>,
        ) -> Result<tonic::Response<super::MsgScheduleCorkResponse>, tonic::Status>;
    }
    #[doc = " MsgService defines the msgs that the cork module handles"]
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Msg> MsgServer<T> {
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
    impl<T, B> Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
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
                "/cork.v2.Msg/ScheduleCork" => {
                    #[allow(non_camel_case_types)]
                    struct ScheduleCorkSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgScheduleCorkRequest> for ScheduleCorkSvc<T> {
                        type Response = super::MsgScheduleCorkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgScheduleCorkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).schedule_cork(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ScheduleCorkSvc(inner);
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
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::transport::NamedService for MsgServer<T> {
        const NAME: &'static str = "cork.v2.Msg";
    }
}
/// GenesisState - all cork state that must be provided at genesis
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag = "2")]
    pub cellar_ids: ::core::option::Option<CellarIdSet>,
    #[prost(uint64, tag = "3")]
    pub invalidation_nonce: u64,
    #[prost(message, repeated, tag = "4")]
    pub scheduled_corks: ::prost::alloc::vec::Vec<ScheduledCork>,
    #[prost(message, repeated, tag = "5")]
    pub cork_results: ::prost::alloc::vec::Vec<CorkResult>,
}
/// Params cork parameters
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// VoteThreshold defines the percentage of bonded stake required to vote for a scheduled cork to be approved
    #[prost(string, tag = "1")]
    pub vote_threshold: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params gRPC method.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsRequest is the response type for the Query/Params gRPC method.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// allocation parameters
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCellarIDsRequest is the request type for Query/QueryCellarIDs gRPC method.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCellarIDsRequest {}
/// QueryCellarIDsResponse is the response type for Query/QueryCellars gRPC method.
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCellarIDsResponse {
    #[prost(string, repeated, tag = "1")]
    pub cellar_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryScheduledCorksRequest
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksRequest {}
/// QueryScheduledCorksResponse
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksResponse {
    #[prost(message, repeated, tag = "1")]
    pub corks: ::prost::alloc::vec::Vec<ScheduledCork>,
}
/// QueryScheduledBlockHeightsRequest
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledBlockHeightsRequest {}
/// QueryScheduledBlockHeightsResponse
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledBlockHeightsResponse {
    #[prost(uint64, repeated, tag = "1")]
    pub block_heights: ::prost::alloc::vec::Vec<u64>,
}
/// QueryScheduledCorksByBlockHeightRequest
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksByBlockHeightRequest {
    #[prost(uint64, tag = "1")]
    pub block_height: u64,
}
/// QueryScheduledCorksByBlockHeightResponse
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksByBlockHeightResponse {
    #[prost(message, repeated, tag = "1")]
    pub corks: ::prost::alloc::vec::Vec<ScheduledCork>,
}
/// QueryScheduledCorksByIDRequest
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksByIdRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryScheduledCorksByIDResponse
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryScheduledCorksByIdResponse {
    #[prost(message, repeated, tag = "1")]
    pub corks: ::prost::alloc::vec::Vec<ScheduledCork>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCorkResultRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCorkResultResponse {
    #[prost(message, optional, tag = "1")]
    pub cork_result: ::core::option::Option<CorkResult>,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCorkResultsRequest {}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct QueryCorkResultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub cork_results: ::prost::alloc::vec::Vec<CorkResult>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC query service for the cork module."]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        #[doc = " QueryParams queries the allocation module parameters."]
        pub async fn query_params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryParams");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " QueryCellarIDs returns all cellars and current tick ranges"]
        pub async fn query_cellar_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCellarIDsRequest>,
        ) -> Result<tonic::Response<super::QueryCellarIDsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryCellarIDs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " QueryScheduledCorks returns all scheduled corks"]
        pub async fn query_scheduled_corks(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScheduledCorksRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryScheduledCorks");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " QueryScheduledBlockHeights returns all scheduled block heights"]
        pub async fn query_scheduled_block_heights(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScheduledBlockHeightsRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledBlockHeightsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryScheduledBlockHeights");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " QueryScheduledCorks returns all scheduled corks at a block height"]
        pub async fn query_scheduled_corks_by_block_height(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScheduledCorksByBlockHeightRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksByBlockHeightResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cork.v2.Query/QueryScheduledCorksByBlockHeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " QueryScheduledCorks returns all scheduled corks with the specified ID"]
        pub async fn query_scheduled_corks_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScheduledCorksByIdRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksByIdResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryScheduledCorksByID");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_cork_result(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCorkResultRequest>,
        ) -> Result<tonic::Response<super::QueryCorkResultResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryCorkResult");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query_cork_results(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCorkResultsRequest>,
        ) -> Result<tonic::Response<super::QueryCorkResultsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cork.v2.Query/QueryCorkResults");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."]
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        #[doc = " QueryParams queries the allocation module parameters."]
        async fn query_params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        #[doc = " QueryCellarIDs returns all cellars and current tick ranges"]
        async fn query_cellar_i_ds(
            &self,
            request: tonic::Request<super::QueryCellarIDsRequest>,
        ) -> Result<tonic::Response<super::QueryCellarIDsResponse>, tonic::Status>;
        #[doc = " QueryScheduledCorks returns all scheduled corks"]
        async fn query_scheduled_corks(
            &self,
            request: tonic::Request<super::QueryScheduledCorksRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksResponse>, tonic::Status>;
        #[doc = " QueryScheduledBlockHeights returns all scheduled block heights"]
        async fn query_scheduled_block_heights(
            &self,
            request: tonic::Request<super::QueryScheduledBlockHeightsRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledBlockHeightsResponse>, tonic::Status>;
        #[doc = " QueryScheduledCorks returns all scheduled corks at a block height"]
        async fn query_scheduled_corks_by_block_height(
            &self,
            request: tonic::Request<super::QueryScheduledCorksByBlockHeightRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksByBlockHeightResponse>, tonic::Status>;
        #[doc = " QueryScheduledCorks returns all scheduled corks with the specified ID"]
        async fn query_scheduled_corks_by_id(
            &self,
            request: tonic::Request<super::QueryScheduledCorksByIdRequest>,
        ) -> Result<tonic::Response<super::QueryScheduledCorksByIdResponse>, tonic::Status>;
        async fn query_cork_result(
            &self,
            request: tonic::Request<super::QueryCorkResultRequest>,
        ) -> Result<tonic::Response<super::QueryCorkResultResponse>, tonic::Status>;
        async fn query_cork_results(
            &self,
            request: tonic::Request<super::QueryCorkResultsRequest>,
        ) -> Result<tonic::Response<super::QueryCorkResultsResponse>, tonic::Status>;
    }
    #[doc = " Query defines the gRPC query service for the cork module."]
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Query> QueryServer<T> {
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
    impl<T, B> Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
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
                "/cork.v2.Query/QueryParams" => {
                    #[allow(non_camel_case_types)]
                    struct QueryParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for QueryParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryParamsSvc(inner);
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
                "/cork.v2.Query/QueryCellarIDs" => {
                    #[allow(non_camel_case_types)]
                    struct QueryCellarIDsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCellarIDsRequest> for QueryCellarIDsSvc<T> {
                        type Response = super::QueryCellarIDsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCellarIDsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_cellar_i_ds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryCellarIDsSvc(inner);
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
                "/cork.v2.Query/QueryScheduledCorks" => {
                    #[allow(non_camel_case_types)]
                    struct QueryScheduledCorksSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryScheduledCorksRequest>
                        for QueryScheduledCorksSvc<T>
                    {
                        type Response = super::QueryScheduledCorksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScheduledCorksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_scheduled_corks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryScheduledCorksSvc(inner);
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
                "/cork.v2.Query/QueryScheduledBlockHeights" => {
                    #[allow(non_camel_case_types)]
                    struct QueryScheduledBlockHeightsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryScheduledBlockHeightsRequest>
                        for QueryScheduledBlockHeightsSvc<T>
                    {
                        type Response = super::QueryScheduledBlockHeightsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScheduledBlockHeightsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).query_scheduled_block_heights(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryScheduledBlockHeightsSvc(inner);
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
                "/cork.v2.Query/QueryScheduledCorksByBlockHeight" => {
                    #[allow(non_camel_case_types)]
                    struct QueryScheduledCorksByBlockHeightSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryScheduledCorksByBlockHeightRequest>
                        for QueryScheduledCorksByBlockHeightSvc<T>
                    {
                        type Response = super::QueryScheduledCorksByBlockHeightResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScheduledCorksByBlockHeightRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .query_scheduled_corks_by_block_height(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryScheduledCorksByBlockHeightSvc(inner);
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
                "/cork.v2.Query/QueryScheduledCorksByID" => {
                    #[allow(non_camel_case_types)]
                    struct QueryScheduledCorksByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryScheduledCorksByIdRequest>
                        for QueryScheduledCorksByIDSvc<T>
                    {
                        type Response = super::QueryScheduledCorksByIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScheduledCorksByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).query_scheduled_corks_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryScheduledCorksByIDSvc(inner);
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
                "/cork.v2.Query/QueryCorkResult" => {
                    #[allow(non_camel_case_types)]
                    struct QueryCorkResultSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCorkResultRequest>
                        for QueryCorkResultSvc<T>
                    {
                        type Response = super::QueryCorkResultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCorkResultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_cork_result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryCorkResultSvc(inner);
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
                "/cork.v2.Query/QueryCorkResults" => {
                    #[allow(non_camel_case_types)]
                    struct QueryCorkResultsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCorkResultsRequest>
                        for QueryCorkResultsSvc<T>
                    {
                        type Response = super::QueryCorkResultsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCorkResultsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query_cork_results(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryCorkResultsSvc(inner);
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
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::transport::NamedService for QueryServer<T> {
        const NAME: &'static str = "cork.v2.Query";
    }
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AddManagedCellarIDsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub cellar_ids: ::core::option::Option<CellarIdSet>,
}
/// AddManagedCellarIDsProposalWithDeposit is a specific definition for CLI commands
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AddManagedCellarIDsProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub cellar_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub deposit: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct RemoveManagedCellarIDsProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub cellar_ids: ::core::option::Option<CellarIdSet>,
}
/// RemoveManagedCellarIDsProposalWithDeposit is a specific definition for CLI commands
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct RemoveManagedCellarIDsProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub cellar_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub deposit: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduledCorkProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(string, tag = "4")]
    pub target_contract_address: ::prost::alloc::string::String,
    ///
    /// The JSON representation of a ScheduleRequest defined in the Steward protos
    ///
    /// Example: The following is the JSON form of a ScheduleRequest containing a steward.v2.cellar_v1.TrustPosition
    /// message, which maps to the `trustPosition(address)` function of the the V1 Cellar contract.
    ///
    /// {
    ///   "cellar_id": "0x1234567890000000000000000000000000000000",
    ///   "cellar_v1": {
    ///     "trust_position": {
    ///       "erc20_address": "0x1234567890000000000000000000000000000000"
    ///     }
    ///   },
    ///   "block_height": 1000000
    /// }
    ///
    /// You can use the Steward CLI to generate the required JSON rather than constructing it by hand https://github.com/peggyjv/steward
    #[prost(string, tag = "5")]
    pub contract_call_proto_json: ::prost::alloc::string::String,
}
/// ScheduledCorkProposalWithDeposit is a specific definition for CLI commands
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct ScheduledCorkProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub block_height: u64,
    #[prost(string, tag = "4")]
    pub target_contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub contract_call_proto_json: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub deposit: ::prost::alloc::string::String,
}
