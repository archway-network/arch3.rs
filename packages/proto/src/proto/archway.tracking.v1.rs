/// TxInfo keeps a transaction gas tracking data.
/// Object is being created at the module EndBlocker.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInfo {
    /// id defines the unique transaction ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// height defines the block height of the transaction.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// total_gas defines total gas consumption by the transaction.
    /// It is the sum of gas consumed by all contract operations (VM + SDK gas).
    #[prost(uint64, tag = "3")]
    pub total_gas: u64,
}
/// ContractOperationInfo keeps a single contract operation gas consumption data.
/// Object is being created by the IngestGasRecord call from the wasmd.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractOperationInfo {
    /// id defines the unique operation ID.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// tx_id defines a transaction ID operation relates to (TxInfo.id).
    #[prost(uint64, tag = "2")]
    pub tx_id: u64,
    /// contract_address defines the contract address operation relates to.
    #[prost(string, tag = "3")]
    pub contract_address: ::prost::alloc::string::String,
    /// operation_type defines the gas consumption type.
    #[prost(enumeration = "ContractOperation", tag = "4")]
    pub operation_type: i32,
    /// vm_gas is the gas consumption reported by the WASM VM.
    /// Value is adjusted by this module (CalculateUpdatedGas func).
    #[prost(uint64, tag = "5")]
    pub vm_gas: u64,
    /// sdk_gas is the gas consumption reported by the SDK gas meter and the WASM
    /// GasRegister (cost of Execute/Query/etc). Value is adjusted by this module
    /// (CalculateUpdatedGas func).
    #[prost(uint64, tag = "6")]
    pub sdk_gas: u64,
}
/// BlockTracking is the tracking information for a block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockTracking {
    /// txs defines the list of transactions tracked in the block.
    #[prost(message, repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<TxTracking>,
}
/// TxTracking is the tracking information for a single transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxTracking {
    /// info defines the transaction details.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<TxInfo>,
    /// contract_operations defines the list of contract operations consumed by the
    /// transaction.
    #[prost(message, repeated, tag = "2")]
    pub contract_operations: ::prost::alloc::vec::Vec<ContractOperationInfo>,
}
/// ContractOperation denotes which operation consumed gas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractOperation {
    /// Invalid or unknown operation
    Unspecified = 0,
    /// Instantiate operation
    Instantiation = 1,
    /// Execute operation
    Execution = 2,
    /// Query
    Query = 3,
    /// Migrate operation
    Migrate = 4,
    /// IBC operations
    Ibc = 5,
    /// Sudo operation
    Sudo = 6,
    /// Reply callback operation
    Reply = 7,
}
impl ContractOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractOperation::Unspecified => "CONTRACT_OPERATION_UNSPECIFIED",
            ContractOperation::Instantiation => "CONTRACT_OPERATION_INSTANTIATION",
            ContractOperation::Execution => "CONTRACT_OPERATION_EXECUTION",
            ContractOperation::Query => "CONTRACT_OPERATION_QUERY",
            ContractOperation::Migrate => "CONTRACT_OPERATION_MIGRATE",
            ContractOperation::Ibc => "CONTRACT_OPERATION_IBC",
            ContractOperation::Sudo => "CONTRACT_OPERATION_SUDO",
            ContractOperation::Reply => "CONTRACT_OPERATION_REPLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_OPERATION_INSTANTIATION" => Some(Self::Instantiation),
            "CONTRACT_OPERATION_EXECUTION" => Some(Self::Execution),
            "CONTRACT_OPERATION_QUERY" => Some(Self::Query),
            "CONTRACT_OPERATION_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_OPERATION_IBC" => Some(Self::Ibc),
            "CONTRACT_OPERATION_SUDO" => Some(Self::Sudo),
            "CONTRACT_OPERATION_REPLY" => Some(Self::Reply),
            _ => None,
        }
    }
}
/// GenesisState defines the initial state of the tracking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// tx_info_last_id defines the last unique ID for a TxInfo objs.
    #[prost(uint64, tag = "1")]
    pub tx_info_last_id: u64,
    /// tx_infos defines a list of all the tracked transactions.
    #[prost(message, repeated, tag = "2")]
    pub tx_infos: ::prost::alloc::vec::Vec<TxInfo>,
    /// contract_op_info_last_id defines the last unique ID for
    /// ContractOperationInfo objs.
    #[prost(uint64, tag = "3")]
    pub contract_op_info_last_id: u64,
    /// contract_op_infos defines a list of all the tracked contract operations.
    #[prost(message, repeated, tag = "4")]
    pub contract_op_infos: ::prost::alloc::vec::Vec<ContractOperationInfo>,
}
/// QueryBlockGasTrackingRequest is the request for Query.BlockGasTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasTrackingRequest {}
/// QueryBlockGasTrackingResponse is the response for Query.BlockGasTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasTrackingResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockTracking>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query service for the tracking module.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// BlockGasTracking returns block gas tracking for the current block
        pub async fn block_gas_tracking(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBlockGasTrackingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBlockGasTrackingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.tracking.v1.Query/BlockGasTracking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("archway.tracking.v1.Query", "BlockGasTracking"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// BlockGasTracking returns block gas tracking for the current block
        async fn block_gas_tracking(
            &self,
            request: tonic::Request<super::QueryBlockGasTrackingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBlockGasTrackingResponse>,
            tonic::Status,
        >;
    }
    /// Query service for the tracking module.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/archway.tracking.v1.Query/BlockGasTracking" => {
                    #[allow(non_camel_case_types)]
                    struct BlockGasTrackingSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryBlockGasTrackingRequest>
                    for BlockGasTrackingSvc<T> {
                        type Response = super::QueryBlockGasTrackingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryBlockGasTrackingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).block_gas_tracking(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockGasTrackingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "archway.tracking.v1.Query";
    }
}
