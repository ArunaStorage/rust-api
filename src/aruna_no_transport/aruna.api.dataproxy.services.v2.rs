#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBundleRequest {
    #[prost(string, repeated, tag = "1")]
    pub resource_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// .tar.gz / .zip
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// Default 1 Month
    #[prost(message, optional, tag = "3")]
    pub expires_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Default false (expires after first download)
    #[prost(bool, tag = "4")]
    pub once: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBundleResponse {
    #[prost(string, tag = "1")]
    pub bundle_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bundle_url: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBundleRequest {
    #[prost(string, tag = "1")]
    pub bundle_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBundleResponse {}
/// Generated client implementations.
pub mod bundler_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// BundlerService
    ///
    /// Status: ALPHA
    ///
    /// Dataproxy specific service for creating and deleting bundles.
    #[derive(Debug, Clone)]
    pub struct BundlerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BundlerServiceClient<T>
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
        ) -> BundlerServiceClient<InterceptedService<T, F>>
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
            BundlerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateBundle
        ///
        /// Status: ALPHA
        ///
        /// Creates a bundle with multiple resources, dataproxy only.
        pub async fn create_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBundleResponse>,
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
                "/aruna.api.dataproxy.services.v2.BundlerService/CreateBundle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.BundlerService",
                        "CreateBundle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteBundle
        ///
        /// Status: ALPHA
        ///
        /// Delete an existing bundle, dataproxy only.
        pub async fn delete_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBundleResponse>,
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
                "/aruna.api.dataproxy.services.v2.BundlerService/DeleteBundle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.BundlerService",
                        "DeleteBundle",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod bundler_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BundlerServiceServer.
    #[async_trait]
    pub trait BundlerService: Send + Sync + 'static {
        /// CreateBundle
        ///
        /// Status: ALPHA
        ///
        /// Creates a bundle with multiple resources, dataproxy only.
        async fn create_bundle(
            &self,
            request: tonic::Request<super::CreateBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBundleResponse>,
            tonic::Status,
        >;
        /// DeleteBundle
        ///
        /// Status: ALPHA
        ///
        /// Delete an existing bundle, dataproxy only.
        async fn delete_bundle(
            &self,
            request: tonic::Request<super::DeleteBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBundleResponse>,
            tonic::Status,
        >;
    }
    /// BundlerService
    ///
    /// Status: ALPHA
    ///
    /// Dataproxy specific service for creating and deleting bundles.
    #[derive(Debug)]
    pub struct BundlerServiceServer<T: BundlerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BundlerService> BundlerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BundlerServiceServer<T>
    where
        T: BundlerService,
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
                "/aruna.api.dataproxy.services.v2.BundlerService/CreateBundle" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBundleSvc<T: BundlerService>(pub Arc<T>);
                    impl<
                        T: BundlerService,
                    > tonic::server::UnaryService<super::CreateBundleRequest>
                    for CreateBundleSvc<T> {
                        type Response = super::CreateBundleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBundleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BundlerService>::create_bundle(&inner, request).await
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
                        let method = CreateBundleSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.BundlerService/DeleteBundle" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBundleSvc<T: BundlerService>(pub Arc<T>);
                    impl<
                        T: BundlerService,
                    > tonic::server::UnaryService<super::DeleteBundleRequest>
                    for DeleteBundleSvc<T> {
                        type Response = super::DeleteBundleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBundleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as BundlerService>::delete_bundle(&inner, request).await
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
                        let method = DeleteBundleSvc(inner);
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
    impl<T: BundlerService> Clone for BundlerServiceServer<T> {
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
    impl<T: BundlerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BundlerService> tonic::server::NamedService for BundlerServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.BundlerService";
    }
}
/// Messages (requests) from PROXY B
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMessage {
    #[prost(string, tag = "1")]
    pub dataproxy_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoAckMessage {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkAckMessage {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub chunk_idx: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryChunkMessage {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub chunk_idx: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorMessage {
    #[prost(oneof = "error_message::Error", tags = "1, 2, 3")]
    pub error: ::core::option::Option<error_message::Error>,
}
/// Nested message and enum types in `ErrorMessage`.
pub mod error_message {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Error {
        #[prost(message, tag = "1")]
        RetryChunk(super::RetryChunkMessage),
        #[prost(message, tag = "2")]
        Abort(super::Empty),
        #[prost(string, tag = "3")]
        RetryObjectId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReplicationRequest {
    #[prost(oneof = "pull_replication_request::Message", tags = "1, 2, 3, 4, 5")]
    pub message: ::core::option::Option<pull_replication_request::Message>,
}
/// Nested message and enum types in `PullReplicationRequest`.
pub mod pull_replication_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        InitMessage(super::InitMessage),
        #[prost(message, tag = "2")]
        InfoAckMessage(super::InfoAckMessage),
        #[prost(message, tag = "3")]
        ChunkAckMessage(super::ChunkAckMessage),
        #[prost(message, tag = "4")]
        ErrorMessage(super::ErrorMessage),
        #[prost(message, tag = "5")]
        FinishMessage(super::Empty),
    }
}
/// Messages (responses) from PROXY A
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectInfo {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// == (Compressed_size / (65536 + 28)) + 1
    #[prost(int64, tag = "2")]
    pub chunks: i64,
    #[prost(int64, tag = "3")]
    pub raw_size: i64,
    #[prost(int64, tag = "4")]
    pub compressed_size: i64,
    /// JSON encoded proxy specific extra fields
    #[prost(string, optional, tag = "5")]
    pub extra: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub chunk_idx: i64,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub checksum: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReplicationResponse {
    #[prost(oneof = "pull_replication_response::Message", tags = "1, 2, 3")]
    pub message: ::core::option::Option<pull_replication_response::Message>,
}
/// Nested message and enum types in `PullReplicationResponse`.
pub mod pull_replication_response {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        ObjectInfo(super::ObjectInfo),
        /// If no ack is received, the chunk will be resent
        #[prost(message, tag = "2")]
        Chunk(super::Chunk),
        #[prost(message, tag = "3")]
        FinishMessage(super::Empty),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfo {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub download_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub encryption_key: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_compressed: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfos {
    #[prost(message, repeated, tag = "1")]
    pub data_info: ::prost::alloc::vec::Vec<DataInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub data_infos: ::core::option::Option<DataInfos>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReplicationResponse {
    #[prost(bool, tag = "1")]
    pub ack: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialsRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCredentialsResponse {
    #[prost(string, tag = "1")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub secret_key: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrUpdateCredentialsRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrUpdateCredentialsResponse {
    #[prost(string, tag = "1")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub secret_key: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeCredentialsRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeCredentialsResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3Path {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReplicaRequest {
    #[prost(string, tag = "3")]
    pub target_endpoint_id: ::prost::alloc::string::String,
    #[prost(oneof = "push_replica_request::Resource", tags = "1, 2")]
    pub resource: ::core::option::Option<push_replica_request::Resource>,
}
/// Nested message and enum types in `PushReplicaRequest`.
pub mod push_replica_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(string, tag = "1")]
        ResourceId(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        S3Path(super::S3Path),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushReplicaResponse {
    #[prost(string, tag = "1")]
    pub replication_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReplicaRequest {
    #[prost(oneof = "pull_replica_request::Resource", tags = "1, 2")]
    pub resource: ::core::option::Option<pull_replica_request::Resource>,
}
/// Nested message and enum types in `PullReplicaRequest`.
pub mod pull_replica_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(string, tag = "1")]
        ResourceId(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        S3Path(super::S3Path),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReplicaResponse {
    /// why ?
    #[prost(string, tag = "1")]
    pub replication_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatusRequest {
    /// why ?
    #[prost(string, tag = "1")]
    pub replication_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatusResponse {
    #[prost(enumeration = "ReplicationStatus", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectLocation {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub content_length: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutObjectRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutObjectResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadObjectRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeadObjectResponse {
    #[prost(string, tag = "1")]
    pub content_length: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub exists: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMultiPartUploadRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMultiPartUploadResponse {
    #[prost(string, tag = "1")]
    pub upload_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadPartRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
    #[prost(int32, tag = "3")]
    pub part_number: i32,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadPartResponse {
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedPart {
    #[prost(int32, tag = "1")]
    pub part_number: i32,
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultiPartUploadRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
    #[prost(message, repeated, tag = "2")]
    pub completed_parts: ::prost::alloc::vec::Vec<CompletedPart>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultiPartUploadResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketRequest {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketRequest {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBucketResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitLocationRequest {
    #[prost(string, tag = "1")]
    pub object_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub size: i64,
    #[prost(bool, tag = "3")]
    pub is_temporary: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<ObjectLocation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestResource {
    /// object name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// title
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// description
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Authors
    #[prost(message, repeated, tag = "4")]
    pub authors: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::Author,
    >,
    /// object specific labels / hooks
    #[prost(message, repeated, tag = "5")]
    pub key_values: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::KeyValue,
    >,
    /// Internal / External relations (URLs / IDs from external sources)
    #[prost(message, repeated, tag = "6")]
    pub relations: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::Relation,
    >,
    /// DataClass
    #[prost(
        enumeration = "super::super::super::storage::models::v2::DataClass",
        tag = "7"
    )]
    pub data_class: i32,
    /// Ignored if Collection | Dataset
    #[prost(message, repeated, tag = "8")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::super::storage::models::v2::Hash>,
    #[prost(string, tag = "9")]
    pub metadata_license_tag: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub data_license_tag: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestExistingObjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub object: ::core::option::Option<IngestResource>,
    /// "s3://bucket/key" or "file:///foo/bar/baz.txt" must be a valid file
    #[prost(string, tag = "7")]
    pub path: ::prost::alloc::string::String,
    #[prost(oneof = "ingest_existing_object_request::Collection", tags = "2, 3")]
    pub collection: ::core::option::Option<ingest_existing_object_request::Collection>,
    #[prost(oneof = "ingest_existing_object_request::Dataset", tags = "4, 5")]
    pub dataset: ::core::option::Option<ingest_existing_object_request::Dataset>,
}
/// Nested message and enum types in `IngestExistingObjectRequest`.
pub mod ingest_existing_object_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Collection {
        #[prost(string, tag = "2")]
        CollectionId(::prost::alloc::string::String),
        #[prost(message, tag = "3")]
        CollectionResource(super::IngestResource),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dataset {
        #[prost(string, tag = "4")]
        DatasetId(::prost::alloc::string::String),
        #[prost(message, tag = "5")]
        DatasetResource(super::IngestResource),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestExistingObjectResponse {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplicationStatus {
    Unspecified = 0,
    Pending = 1,
    Running = 2,
    Finished = 3,
    Error = 4,
}
impl ReplicationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplicationStatus::Unspecified => "REPLICATION_STATUS_UNSPECIFIED",
            ReplicationStatus::Pending => "REPLICATION_STATUS_PENDING",
            ReplicationStatus::Running => "REPLICATION_STATUS_RUNNING",
            ReplicationStatus::Finished => "REPLICATION_STATUS_FINISHED",
            ReplicationStatus::Error => "REPLICATION_STATUS_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REPLICATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "REPLICATION_STATUS_PENDING" => Some(Self::Pending),
            "REPLICATION_STATUS_RUNNING" => Some(Self::Running),
            "REPLICATION_STATUS_FINISHED" => Some(Self::Finished),
            "REPLICATION_STATUS_ERROR" => Some(Self::Error),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod dataproxy_replication_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// DataproxyService
    ///
    /// Status: ALPHA
    ///
    /// Service for data replication between data-proxies
    #[derive(Debug, Clone)]
    pub struct DataproxyReplicationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataproxyReplicationServiceClient<T>
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
        ) -> DataproxyReplicationServiceClient<InterceptedService<T, F>>
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
            DataproxyReplicationServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// RequestReplication
        ///
        /// Status: ALPHA
        ///
        /// Creates a replication stream
        pub async fn pull_replication(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::PullReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PullReplicationResponse>>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyReplicationService/PullReplication",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyReplicationService",
                        "PullReplication",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// InitReplication
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Provides the necessary url to init replication
        pub async fn push_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::PushReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PushReplicationResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyReplicationService/PushReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyReplicationService",
                        "PushReplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod dataproxy_backend_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataproxyBackendServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataproxyBackendServiceClient<T>
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
        ) -> DataproxyBackendServiceClient<InterceptedService<T, F>>
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
            DataproxyBackendServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        pub async fn put_object(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::PutObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutObjectResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/PutObject",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "PutObject",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GetObjectResponse>>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/GetObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "GetObject",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn head_object(
            &mut self,
            request: impl tonic::IntoRequest<super::HeadObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeadObjectResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/HeadObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "HeadObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn init_multi_part_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::InitMultiPartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitMultiPartUploadResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/InitMultiPartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "InitMultiPartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn upload_part(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UploadPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UploadPartResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/UploadPart",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "UploadPart",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn complete_multi_part_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteMultiPartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultiPartUploadResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/CompleteMultiPartUpload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "CompleteMultiPartUpload",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBucketResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/CreateBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "CreateBucket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBucketResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/DeleteBucket",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "DeleteBucket",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteObjectResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/DeleteObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "DeleteObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn init_location(
            &mut self,
            request: impl tonic::IntoRequest<super::InitLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitLocationResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/InitLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyBackendService",
                        "InitLocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod dataproxy_user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataproxyUserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataproxyUserServiceClient<T>
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
        ) -> DataproxyUserServiceClient<InterceptedService<T, F>>
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
            DataproxyUserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// GetCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token to exchange for dataproxy
        /// specific S3AccessKey and S3SecretKey
        pub async fn get_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCredentialsResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/GetCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "GetCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateOrUpdateCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token to exchange for dataproxy
        /// specific S3AccessKey and S3SecretKey
        pub async fn create_or_update_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrUpdateCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrUpdateCredentialsResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/CreateOrUpdateCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "CreateOrUpdateCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RevokeCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token
        /// Revokes the current credentials
        pub async fn revoke_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeCredentialsResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/RevokeCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "RevokeCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PushReplica
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Manually transfers a replica to another data-proxy
        pub async fn push_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::PushReplicaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PushReplicaResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/PushReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "PushReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PullReplica
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Manually request data to be transferred to this data-proxy
        pub async fn pull_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::PullReplicaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PullReplicaResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/PullReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "PullReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ReplicationStatus
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Status of the previous replication request
        pub async fn replication_status(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplicationStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReplicationStatusResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/ReplicationStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyUserService",
                        "ReplicationStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod dataproxy_ingestion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataproxyIngestionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataproxyIngestionServiceClient<T>
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
        ) -> DataproxyIngestionServiceClient<InterceptedService<T, F>>
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
            DataproxyIngestionServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// IngestExistingObject
        ///
        /// Status: ALPHA
        ///
        /// Ingest an existing object into backend
        pub async fn ingest_existing_object(
            &mut self,
            request: impl tonic::IntoRequest<super::IngestExistingObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IngestExistingObjectResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyIngestionService/IngestExistingObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyIngestionService",
                        "IngestExistingObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dataproxy_replication_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataproxyReplicationServiceServer.
    #[async_trait]
    pub trait DataproxyReplicationService: Send + Sync + 'static {
        /// Server streaming response type for the PullReplication method.
        type PullReplicationStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::PullReplicationResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// RequestReplication
        ///
        /// Status: ALPHA
        ///
        /// Creates a replication stream
        async fn pull_replication(
            &self,
            request: tonic::Request<tonic::Streaming<super::PullReplicationRequest>>,
        ) -> std::result::Result<
            tonic::Response<Self::PullReplicationStream>,
            tonic::Status,
        >;
        /// InitReplication
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Provides the necessary url to init replication
        async fn push_replication(
            &self,
            request: tonic::Request<super::PushReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PushReplicationResponse>,
            tonic::Status,
        >;
    }
    /// DataproxyService
    ///
    /// Status: ALPHA
    ///
    /// Service for data replication between data-proxies
    #[derive(Debug)]
    pub struct DataproxyReplicationServiceServer<T: DataproxyReplicationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataproxyReplicationService> DataproxyReplicationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DataproxyReplicationServiceServer<T>
    where
        T: DataproxyReplicationService,
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
                "/aruna.api.dataproxy.services.v2.DataproxyReplicationService/PullReplication" => {
                    #[allow(non_camel_case_types)]
                    struct PullReplicationSvc<T: DataproxyReplicationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyReplicationService,
                    > tonic::server::StreamingService<super::PullReplicationRequest>
                    for PullReplicationSvc<T> {
                        type Response = super::PullReplicationResponse;
                        type ResponseStream = T::PullReplicationStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PullReplicationRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyReplicationService>::pull_replication(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = PullReplicationSvc(inner);
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
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.dataproxy.services.v2.DataproxyReplicationService/PushReplication" => {
                    #[allow(non_camel_case_types)]
                    struct PushReplicationSvc<T: DataproxyReplicationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyReplicationService,
                    > tonic::server::UnaryService<super::PushReplicationRequest>
                    for PushReplicationSvc<T> {
                        type Response = super::PushReplicationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PushReplicationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyReplicationService>::push_replication(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = PushReplicationSvc(inner);
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
    impl<T: DataproxyReplicationService> Clone for DataproxyReplicationServiceServer<T> {
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
    impl<T: DataproxyReplicationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataproxyReplicationService> tonic::server::NamedService
    for DataproxyReplicationServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.DataproxyReplicationService";
    }
}
/// Generated server implementations.
pub mod dataproxy_backend_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataproxyBackendServiceServer.
    #[async_trait]
    pub trait DataproxyBackendService: Send + Sync + 'static {
        async fn put_object(
            &self,
            request: tonic::Request<tonic::Streaming<super::PutObjectRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::PutObjectResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetObject method.
        type GetObjectStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::GetObjectResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_object(
            &self,
            request: tonic::Request<super::GetObjectRequest>,
        ) -> std::result::Result<tonic::Response<Self::GetObjectStream>, tonic::Status>;
        async fn head_object(
            &self,
            request: tonic::Request<super::HeadObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HeadObjectResponse>,
            tonic::Status,
        >;
        async fn init_multi_part_upload(
            &self,
            request: tonic::Request<super::InitMultiPartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitMultiPartUploadResponse>,
            tonic::Status,
        >;
        async fn upload_part(
            &self,
            request: tonic::Request<tonic::Streaming<super::UploadPartRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::UploadPartResponse>,
            tonic::Status,
        >;
        async fn complete_multi_part_upload(
            &self,
            request: tonic::Request<super::CompleteMultiPartUploadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CompleteMultiPartUploadResponse>,
            tonic::Status,
        >;
        async fn create_bucket(
            &self,
            request: tonic::Request<super::CreateBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBucketResponse>,
            tonic::Status,
        >;
        async fn delete_bucket(
            &self,
            request: tonic::Request<super::DeleteBucketRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBucketResponse>,
            tonic::Status,
        >;
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteObjectResponse>,
            tonic::Status,
        >;
        async fn init_location(
            &self,
            request: tonic::Request<super::InitLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InitLocationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DataproxyBackendServiceServer<T: DataproxyBackendService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataproxyBackendService> DataproxyBackendServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DataproxyBackendServiceServer<T>
    where
        T: DataproxyBackendService,
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/PutObject" => {
                    #[allow(non_camel_case_types)]
                    struct PutObjectSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::ClientStreamingService<super::PutObjectRequest>
                    for PutObjectSvc<T> {
                        type Response = super::PutObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::PutObjectRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::put_object(&inner, request)
                                    .await
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
                        let method = PutObjectSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/GetObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::ServerStreamingService<super::GetObjectRequest>
                    for GetObjectSvc<T> {
                        type Response = super::GetObjectResponse;
                        type ResponseStream = T::GetObjectStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::get_object(&inner, request)
                                    .await
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
                        let method = GetObjectSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/HeadObject" => {
                    #[allow(non_camel_case_types)]
                    struct HeadObjectSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::HeadObjectRequest>
                    for HeadObjectSvc<T> {
                        type Response = super::HeadObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HeadObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::head_object(&inner, request)
                                    .await
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
                        let method = HeadObjectSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/InitMultiPartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct InitMultiPartUploadSvc<T: DataproxyBackendService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::InitMultiPartUploadRequest>
                    for InitMultiPartUploadSvc<T> {
                        type Response = super::InitMultiPartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitMultiPartUploadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::init_multi_part_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = InitMultiPartUploadSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/UploadPart" => {
                    #[allow(non_camel_case_types)]
                    struct UploadPartSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::ClientStreamingService<super::UploadPartRequest>
                    for UploadPartSvc<T> {
                        type Response = super::UploadPartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::UploadPartRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::upload_part(&inner, request)
                                    .await
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
                        let method = UploadPartSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/CompleteMultiPartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultiPartUploadSvc<T: DataproxyBackendService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::CompleteMultiPartUploadRequest>
                    for CompleteMultiPartUploadSvc<T> {
                        type Response = super::CompleteMultiPartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CompleteMultiPartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::complete_multi_part_upload(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = CompleteMultiPartUploadSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/CreateBucket" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBucketSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::CreateBucketRequest>
                    for CreateBucketSvc<T> {
                        type Response = super::CreateBucketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBucketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::create_bucket(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = CreateBucketSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/DeleteBucket" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBucketSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::DeleteBucketRequest>
                    for DeleteBucketSvc<T> {
                        type Response = super::DeleteBucketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBucketRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::delete_bucket(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = DeleteBucketSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::DeleteObjectRequest>
                    for DeleteObjectSvc<T> {
                        type Response = super::DeleteObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::delete_object(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = DeleteObjectSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyBackendService/InitLocation" => {
                    #[allow(non_camel_case_types)]
                    struct InitLocationSvc<T: DataproxyBackendService>(pub Arc<T>);
                    impl<
                        T: DataproxyBackendService,
                    > tonic::server::UnaryService<super::InitLocationRequest>
                    for InitLocationSvc<T> {
                        type Response = super::InitLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyBackendService>::init_location(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = InitLocationSvc(inner);
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
    impl<T: DataproxyBackendService> Clone for DataproxyBackendServiceServer<T> {
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
    impl<T: DataproxyBackendService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataproxyBackendService> tonic::server::NamedService
    for DataproxyBackendServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.DataproxyBackendService";
    }
}
/// Generated server implementations.
pub mod dataproxy_user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataproxyUserServiceServer.
    #[async_trait]
    pub trait DataproxyUserService: Send + Sync + 'static {
        /// GetCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token to exchange for dataproxy
        /// specific S3AccessKey and S3SecretKey
        async fn get_credentials(
            &self,
            request: tonic::Request<super::GetCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCredentialsResponse>,
            tonic::Status,
        >;
        /// CreateOrUpdateCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token to exchange for dataproxy
        /// specific S3AccessKey and S3SecretKey
        async fn create_or_update_credentials(
            &self,
            request: tonic::Request<super::CreateOrUpdateCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrUpdateCredentialsResponse>,
            tonic::Status,
        >;
        /// RevokeCredentials
        ///
        /// Status: BETA
        ///
        /// Authorized method that needs a aruna-token
        /// Revokes the current credentials
        async fn revoke_credentials(
            &self,
            request: tonic::Request<super::RevokeCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RevokeCredentialsResponse>,
            tonic::Status,
        >;
        /// PushReplica
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Manually transfers a replica to another data-proxy
        async fn push_replica(
            &self,
            request: tonic::Request<super::PushReplicaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PushReplicaResponse>,
            tonic::Status,
        >;
        /// PullReplica
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Manually request data to be transferred to this data-proxy
        async fn pull_replica(
            &self,
            request: tonic::Request<super::PullReplicaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PullReplicaResponse>,
            tonic::Status,
        >;
        /// ReplicationStatus
        ///
        /// Status: UNIMPLEMENTED
        ///
        /// Status of the previous replication request
        async fn replication_status(
            &self,
            request: tonic::Request<super::ReplicationStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReplicationStatusResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DataproxyUserServiceServer<T: DataproxyUserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataproxyUserService> DataproxyUserServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DataproxyUserServiceServer<T>
    where
        T: DataproxyUserService,
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/GetCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct GetCredentialsSvc<T: DataproxyUserService>(pub Arc<T>);
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<super::GetCredentialsRequest>
                    for GetCredentialsSvc<T> {
                        type Response = super::GetCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCredentialsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::get_credentials(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = GetCredentialsSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/CreateOrUpdateCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrUpdateCredentialsSvc<T: DataproxyUserService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<
                        super::CreateOrUpdateCredentialsRequest,
                    > for CreateOrUpdateCredentialsSvc<T> {
                        type Response = super::CreateOrUpdateCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateOrUpdateCredentialsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::create_or_update_credentials(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = CreateOrUpdateCredentialsSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/RevokeCredentials" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeCredentialsSvc<T: DataproxyUserService>(pub Arc<T>);
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<super::RevokeCredentialsRequest>
                    for RevokeCredentialsSvc<T> {
                        type Response = super::RevokeCredentialsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeCredentialsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::revoke_credentials(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = RevokeCredentialsSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/PushReplica" => {
                    #[allow(non_camel_case_types)]
                    struct PushReplicaSvc<T: DataproxyUserService>(pub Arc<T>);
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<super::PushReplicaRequest>
                    for PushReplicaSvc<T> {
                        type Response = super::PushReplicaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PushReplicaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::push_replica(&inner, request)
                                    .await
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
                        let method = PushReplicaSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/PullReplica" => {
                    #[allow(non_camel_case_types)]
                    struct PullReplicaSvc<T: DataproxyUserService>(pub Arc<T>);
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<super::PullReplicaRequest>
                    for PullReplicaSvc<T> {
                        type Response = super::PullReplicaResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullReplicaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::pull_replica(&inner, request)
                                    .await
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
                        let method = PullReplicaSvc(inner);
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
                "/aruna.api.dataproxy.services.v2.DataproxyUserService/ReplicationStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ReplicationStatusSvc<T: DataproxyUserService>(pub Arc<T>);
                    impl<
                        T: DataproxyUserService,
                    > tonic::server::UnaryService<super::ReplicationStatusRequest>
                    for ReplicationStatusSvc<T> {
                        type Response = super::ReplicationStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReplicationStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyUserService>::replication_status(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = ReplicationStatusSvc(inner);
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
    impl<T: DataproxyUserService> Clone for DataproxyUserServiceServer<T> {
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
    impl<T: DataproxyUserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataproxyUserService> tonic::server::NamedService
    for DataproxyUserServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.DataproxyUserService";
    }
}
/// Generated server implementations.
pub mod dataproxy_ingestion_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataproxyIngestionServiceServer.
    #[async_trait]
    pub trait DataproxyIngestionService: Send + Sync + 'static {
        /// IngestExistingObject
        ///
        /// Status: ALPHA
        ///
        /// Ingest an existing object into backend
        async fn ingest_existing_object(
            &self,
            request: tonic::Request<super::IngestExistingObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IngestExistingObjectResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DataproxyIngestionServiceServer<T: DataproxyIngestionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataproxyIngestionService> DataproxyIngestionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DataproxyIngestionServiceServer<T>
    where
        T: DataproxyIngestionService,
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
                "/aruna.api.dataproxy.services.v2.DataproxyIngestionService/IngestExistingObject" => {
                    #[allow(non_camel_case_types)]
                    struct IngestExistingObjectSvc<T: DataproxyIngestionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataproxyIngestionService,
                    > tonic::server::UnaryService<super::IngestExistingObjectRequest>
                    for IngestExistingObjectSvc<T> {
                        type Response = super::IngestExistingObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IngestExistingObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyIngestionService>::ingest_existing_object(
                                        &inner,
                                        request,
                                    )
                                    .await
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
                        let method = IngestExistingObjectSvc(inner);
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
    impl<T: DataproxyIngestionService> Clone for DataproxyIngestionServiceServer<T> {
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
    impl<T: DataproxyIngestionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataproxyIngestionService> tonic::server::NamedService
    for DataproxyIngestionServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.DataproxyIngestionService";
    }
}
