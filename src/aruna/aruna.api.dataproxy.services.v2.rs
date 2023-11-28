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
    #[derive(Debug, Clone)]
    pub struct BundlerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BundlerServiceClient<tonic::transport::Channel> {
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
        async fn create_bundle(
            &self,
            request: tonic::Request<super::CreateBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateBundleResponse>,
            tonic::Status,
        >;
        async fn delete_bundle(
            &self,
            request: tonic::Request<super::DeleteBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBundleResponse>,
            tonic::Status,
        >;
    }
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
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProxyInfo {
    #[prost(string, tag = "1")]
    pub dataproxy_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub available_space: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<DataProxyInfo>,
    #[prost(bool, tag = "2")]
    pub user_initialized: bool,
    #[prost(string, repeated, tag = "3")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
pub struct PullReplicationResponse {
    /// oneof response {
    ///
    ///     bool ack = 2;
    /// }
    #[prost(message, optional, tag = "1")]
    pub data_infos: ::core::option::Option<DataInfos>,
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
pub mod dataproxy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataproxyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataproxyServiceClient<tonic::transport::Channel> {
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
    impl<T> DataproxyServiceClient<T>
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
        ) -> DataproxyServiceClient<InterceptedService<T, F>>
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
            DataproxyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Status: BETA
        ///
        /// Creates a replication request
        pub async fn pull_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::PullReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PullReplicationResponse>,
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
                "/aruna.api.dataproxy.services.v2.DataproxyService/PullReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyService",
                        "PullReplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// InitReplication
        ///
        /// Status: BETA
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
                "/aruna.api.dataproxy.services.v2.DataproxyService/PushReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.dataproxy.services.v2.DataproxyService",
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
    impl DataproxyBackendServiceClient<tonic::transport::Channel> {
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
    impl DataproxyUserServiceClient<tonic::transport::Channel> {
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
        /// PushReplica
        ///
        /// Status: BETA
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
        /// Status: BETA
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
        /// PullReplica
        ///
        /// Status: BETA
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
/// Generated server implementations.
pub mod dataproxy_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataproxyServiceServer.
    #[async_trait]
    pub trait DataproxyService: Send + Sync + 'static {
        /// RequestReplication
        ///
        /// Status: BETA
        ///
        /// Creates a replication request
        async fn pull_replication(
            &self,
            request: tonic::Request<super::PullReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PullReplicationResponse>,
            tonic::Status,
        >;
        /// InitReplication
        ///
        /// Status: BETA
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
    #[derive(Debug)]
    pub struct DataproxyServiceServer<T: DataproxyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataproxyService> DataproxyServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataproxyServiceServer<T>
    where
        T: DataproxyService,
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
                "/aruna.api.dataproxy.services.v2.DataproxyService/PullReplication" => {
                    #[allow(non_camel_case_types)]
                    struct PullReplicationSvc<T: DataproxyService>(pub Arc<T>);
                    impl<
                        T: DataproxyService,
                    > tonic::server::UnaryService<super::PullReplicationRequest>
                    for PullReplicationSvc<T> {
                        type Response = super::PullReplicationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullReplicationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as DataproxyService>::pull_replication(&inner, request)
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
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.dataproxy.services.v2.DataproxyService/PushReplication" => {
                    #[allow(non_camel_case_types)]
                    struct PushReplicationSvc<T: DataproxyService>(pub Arc<T>);
                    impl<
                        T: DataproxyService,
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
                                <T as DataproxyService>::push_replication(&inner, request)
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
    impl<T: DataproxyService> Clone for DataproxyServiceServer<T> {
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
    impl<T: DataproxyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataproxyService> tonic::server::NamedService for DataproxyServiceServer<T> {
        const NAME: &'static str = "aruna.api.dataproxy.services.v2.DataproxyService";
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
        /// PushReplica
        ///
        /// Status: BETA
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
        /// Status: BETA
        ///
        /// Manually request data to be transferred to this data-proxy
        async fn pull_replica(
            &self,
            request: tonic::Request<super::PullReplicaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PullReplicaResponse>,
            tonic::Status,
        >;
        /// PullReplica
        ///
        /// Status: BETA
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
