#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLink {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLinkRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLinkResponse {
    #[prost(string, tag="1")]
    pub upload_link: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkRequest {
    /// Object id the download is requested for
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Optional; Byte range of the data object
    #[prost(message, optional, tag="2")]
    pub range: ::core::option::Option<create_download_link_request::Range>,
}
/// Nested message and enum types in `CreateDownloadLinkRequest`.
pub mod create_download_link_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(int64, tag="1")]
        pub start_byte: i64,
        #[prost(int64, tag="2")]
        pub end_byte: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkResponse {
    #[prost(string, tag="1")]
    pub download_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkBatchRequest {
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<CreateDownloadLinkRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkBatchResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<CreateDownloadLinkResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMultipartUploadRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMultipartUploadResponse {
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultipartUploadLinkResponse {
    #[prost(string, tag="1")]
    pub upload_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultipartUploadLinkRequest {
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub upload_part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadRequest {
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub parts: ::prost::alloc::vec::Vec<CompletedParts>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedParts {
    #[prost(string, tag="1")]
    pub etag: ::prost::alloc::string::String,
    #[prost(int64, tag="2")]
    pub part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkStreamRequest {
    #[prost(oneof="create_download_link_stream_request::Query", tags="1, 3, 4")]
    pub query: ::core::option::Option<create_download_link_stream_request::Query>,
}
/// Nested message and enum types in `CreateDownloadLinkStreamRequest`.
pub mod create_download_link_stream_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateRangeQuery {
        #[prost(string, tag="3")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag="1")]
        pub start: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag="2")]
        pub end: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetQuery {
        #[prost(string, tag="1")]
        pub dataset_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetVersionQuery {
        #[prost(string, tag="1")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub dataset_version_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="1")]
        Dataset(DatasetQuery),
        #[prost(message, tag="3")]
        DatasetVersion(DatasetVersionQuery),
        #[prost(message, tag="4")]
        DateRange(DateRangeQuery),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkStreamResponse {
    #[prost(message, optional, tag="1")]
    pub links: ::core::option::Option<LinksResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinksResponse {
    #[prost(message, repeated, tag="1")]
    pub object_group_revisions: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
    #[prost(message, repeated, tag="2")]
    pub object_group_revision_links: ::prost::alloc::vec::Vec<InnerLinksResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerLinksResponse {
    #[prost(string, repeated, tag="1")]
    pub object_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod object_load_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Handles object up and downloads
    #[derive(Debug, Clone)]
    pub struct ObjectLoadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectLoadServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ObjectLoadServiceClient<T>
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
        ) -> ObjectLoadServiceClient<InterceptedService<T, F>>
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
            ObjectLoadServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates an upload link for an object to upload the actual data object
        /// Returns a presigned https PUT request
        /// Can only be used to upload objects < 4GB
        pub async fn create_upload_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUploadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateUploadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a download link for an object
        /// Returns a presigned https GET request
        pub async fn create_download_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates links for multiple objects at once
        /// The order of the requested objects is preserved in the response
        pub async fn create_download_link_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkBatchRequest>,
        ) -> Result<
            tonic::Response<super::CreateDownloadLinkBatchResponse>,
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLinkBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of ObjectGroups as a stream that would otherwise cause issues with the connection
        pub async fn create_download_link_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::CreateDownloadLinkStreamResponse>,
            >,
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLinkStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// Initiates a multipart upload for an object
        /// This is intended to be used for larger objects
        /// For further information please read the Amazon S3 documentation on multipart uploads
        /// Has to be used together with GetMultipartUploadLink and CompleteMultipartUpload
        pub async fn start_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMultipartUploadRequest>,
        ) -> Result<
            tonic::Response<super::StartMultipartUploadResponse>,
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/StartMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a part of an multipart upload.
        /// Each but the last part needs to be bigger than 5MB in order to use this functionality
        pub async fn get_multipart_upload_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMultipartUploadLinkRequest>,
        ) -> Result<
            tonic::Response<super::GetMultipartUploadLinkResponse>,
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/GetMultipartUploadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///CompleteMultipartUploadRequest Finishes a multipart upload after all parts have been uploaded
        pub async fn complete_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteMultipartUploadRequest>,
        ) -> Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
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
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CompleteMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_load_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ObjectLoadServiceServer.
    #[async_trait]
    pub trait ObjectLoadService: Send + Sync + 'static {
        /// Creates an upload link for an object to upload the actual data object
        /// Returns a presigned https PUT request
        /// Can only be used to upload objects < 4GB
        async fn create_upload_link(
            &self,
            request: tonic::Request<super::CreateUploadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status>;
        /// Creates a download link for an object
        /// Returns a presigned https GET request
        async fn create_download_link(
            &self,
            request: tonic::Request<super::CreateDownloadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkResponse>, tonic::Status>;
        /// Creates links for multiple objects at once
        /// The order of the requested objects is preserved in the response
        async fn create_download_link_batch(
            &self,
            request: tonic::Request<super::CreateDownloadLinkBatchRequest>,
        ) -> Result<
            tonic::Response<super::CreateDownloadLinkBatchResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the CreateDownloadLinkStream method.
        type CreateDownloadLinkStreamStream: futures_core::Stream<
                Item = Result<super::CreateDownloadLinkStreamResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of ObjectGroups as a stream that would otherwise cause issues with the connection
        async fn create_download_link_stream(
            &self,
            request: tonic::Request<super::CreateDownloadLinkStreamRequest>,
        ) -> Result<
            tonic::Response<Self::CreateDownloadLinkStreamStream>,
            tonic::Status,
        >;
        /// Initiates a multipart upload for an object
        /// This is intended to be used for larger objects
        /// For further information please read the Amazon S3 documentation on multipart uploads
        /// Has to be used together with GetMultipartUploadLink and CompleteMultipartUpload
        async fn start_multipart_upload(
            &self,
            request: tonic::Request<super::StartMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::StartMultipartUploadResponse>, tonic::Status>;
        /// Returns a part of an multipart upload.
        /// Each but the last part needs to be bigger than 5MB in order to use this functionality
        async fn get_multipart_upload_link(
            &self,
            request: tonic::Request<super::GetMultipartUploadLinkRequest>,
        ) -> Result<
            tonic::Response<super::GetMultipartUploadLinkResponse>,
            tonic::Status,
        >;
        ///CompleteMultipartUploadRequest Finishes a multipart upload after all parts have been uploaded
        async fn complete_multipart_upload(
            &self,
            request: tonic::Request<super::CompleteMultipartUploadRequest>,
        ) -> Result<
            tonic::Response<super::CompleteMultipartUploadResponse>,
            tonic::Status,
        >;
    }
    /// Handles object up and downloads
    #[derive(Debug)]
    pub struct ObjectLoadServiceServer<T: ObjectLoadService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectLoadService> ObjectLoadServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectLoadServiceServer<T>
    where
        T: ObjectLoadService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateUploadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUploadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::CreateUploadLinkRequest>
                    for CreateUploadLinkSvc<T> {
                        type Response = super::CreateUploadLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUploadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_upload_link(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateUploadLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::CreateDownloadLinkRequest>
                    for CreateDownloadLinkSvc<T> {
                        type Response = super::CreateDownloadLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDownloadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_download_link(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDownloadLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLinkBatch" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkBatchSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::CreateDownloadLinkBatchRequest>
                    for CreateDownloadLinkBatchSvc<T> {
                        type Response = super::CreateDownloadLinkBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDownloadLinkBatchRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_download_link_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDownloadLinkBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CreateDownloadLinkStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkStreamSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::ServerStreamingService<
                        super::CreateDownloadLinkStreamRequest,
                    > for CreateDownloadLinkStreamSvc<T> {
                        type Response = super::CreateDownloadLinkStreamResponse;
                        type ResponseStream = T::CreateDownloadLinkStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDownloadLinkStreamRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_download_link_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDownloadLinkStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/StartMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct StartMultipartUploadSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::StartMultipartUploadRequest>
                    for StartMultipartUploadSvc<T> {
                        type Response = super::StartMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartMultipartUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).start_multipart_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartMultipartUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/GetMultipartUploadLink" => {
                    #[allow(non_camel_case_types)]
                    struct GetMultipartUploadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::GetMultipartUploadLinkRequest>
                    for GetMultipartUploadLinkSvc<T> {
                        type Response = super::GetMultipartUploadLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMultipartUploadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_multipart_upload_link(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMultipartUploadLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ObjectLoadService/CompleteMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultipartUploadSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<
                        T: ObjectLoadService,
                    > tonic::server::UnaryService<super::CompleteMultipartUploadRequest>
                    for CompleteMultipartUploadSvc<T> {
                        type Response = super::CompleteMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CompleteMultipartUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).complete_multipart_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CompleteMultipartUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: ObjectLoadService> Clone for ObjectLoadServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectLoadService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectLoadService> tonic::server::NamedService
    for ObjectLoadServiceServer<T> {
        const NAME: &'static str = "sciobjsdb.api.storage.services.v1.ObjectLoadService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag="5")]
    pub annotations: ::prost::alloc::vec::Vec<super::super::models::v1::Annotation>,
    #[prost(message, repeated, tag="6")]
    pub objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(message, repeated, tag="7")]
    pub metadata_objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    /// If true, an upload link for each generated object and metadata object will be created. The order of the objects will be preserved in the returned list
    #[prost(bool, tag="8")]
    pub include_object_link: bool,
    #[prost(message, optional, tag="10")]
    pub generated: ::core::option::Option<::prost_types::Timestamp>,
    /// A user defined uuid that is used to identify requests in chunked workloads
    #[prost(string, tag="11")]
    pub uuid: ::prost::alloc::string::String,
    /// External path that the objectgroup contents should be stored under.
    #[prost(message, optional, tag="12")]
    pub subpath: ::core::option::Option<super::super::models::v1::Subpath>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupBatchRequest {
    #[prost(message, repeated, tag="1")]
    pub requests: ::prost::alloc::vec::Vec<CreateObjectGroupRequest>,
    #[prost(bool, tag="2")]
    pub include_object_link: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupBatchResponse {
    #[prost(message, repeated, tag="1")]
    pub responses: ::prost::alloc::vec::Vec<CreateObjectGroupResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupResponse {
    #[prost(string, tag="1")]
    pub object_group_id: ::prost::alloc::string::String,
    /// upload link for the individual objects of the objectgroup. Will only be used if include_object_link in the create request is specified.
    #[prost(message, repeated, tag="2")]
    pub object_links: ::prost::alloc::vec::Vec<create_object_group_response::ObjectLinks>,
    #[prost(message, repeated, tag="3")]
    pub metadata_object_links: ::prost::alloc::vec::Vec<create_object_group_response::ObjectLinks>,
    #[prost(string, tag="4")]
    pub object_group_name: ::prost::alloc::string::String,
    /// A user defined uuid that is used to identify requests in chunked/streamed workloads
    #[prost(string, tag="5")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub object_group_revision_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CreateObjectGroupResponse`.
pub mod create_object_group_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectLinks {
        #[prost(string, tag="1")]
        pub filename: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub link: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub object_id: ::prost::alloc::string::String,
        #[prost(int64, tag="4")]
        pub index: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub filetype: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag="4")]
    pub annotations: ::prost::alloc::vec::Vec<super::super::models::v1::Annotation>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag="5")]
    pub content_len: i64,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag="6")]
    pub origin: ::core::option::Option<super::super::models::v1::Origin>,
    /// External path that the object content should be stored under.
    #[prost(message, optional, tag="12")]
    pub subpath: ::core::option::Option<super::super::models::v1::Subpath>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// If left empty, all revisions will be added to the response
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<get_object_group_request::ObjectGroupRevisionPagination>,
}
/// Nested message and enum types in `GetObjectGroupRequest`.
pub mod get_object_group_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectGroupRevisionPagination {
        #[prost(int64, tag="1")]
        pub start_revision_number: i64,
        #[prost(int64, tag="2")]
        pub last_revision_number: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupResponse {
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroup>,
    #[prost(message, repeated, tag="2")]
    pub object_group_revisions: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionResponse {
    #[prost(message, optional, tag="1")]
    pub object_group_revision: ::core::option::Option<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub parent_revision_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub update_objects: ::core::option::Option<UpdateObjectsRequests>,
    #[prost(message, optional, tag="4")]
    pub update_meta_objects: ::core::option::Option<UpdateObjectsRequests>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectsRequests {
    #[prost(message, repeated, tag="1")]
    pub add_objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(message, repeated, tag="2")]
    pub update_object: ::prost::alloc::vec::Vec<UpdateObjectRequest>,
    #[prost(string, repeated, tag="3")]
    pub delete_objects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof="update_object_request::UpdateObject", tags="2")]
    pub update_object: ::core::option::Option<update_object_request::UpdateObject>,
}
/// Nested message and enum types in `UpdateObjectRequest`.
pub mod update_object_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UpdateObject {
        #[prost(message, tag="2")]
        UpdatedObject(super::CreateObjectRequest),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectUploadRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectUploadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectGroupRevisionUploadRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectGroupRevisionUploadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupResponse {
}
/// Dataset related Models
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Name of the dataset
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// ProjectID of the project the dataset is associated with
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag="5")]
    pub annotations: ::prost::alloc::vec::Vec<super::super::models::v1::Annotation>,
    #[prost(message, repeated, tag="6")]
    pub metadata_objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetResponse {
    #[prost(message, optional, tag="1")]
    pub dataset: ::core::option::Option<super::super::models::v1::Dataset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionsResponse {
    #[prost(message, repeated, tag="1")]
    pub dataset_versions: ::prost::alloc::vec::Vec<super::super::models::v1::DatasetVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetObjectGroupsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetObjectGroupsResponse {
    #[prost(message, repeated, tag="1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionsInDateRangeRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionsInDateRangeResponse {
    #[prost(message, repeated, tag="1")]
    pub object_group_revisions: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
}
/// GetObjectGroupsStreamLinkRequest a request for a get link to stream a set of object groups
/// The query defines what object groups should be part of the stream
/// The steam type defines how the individual objects are packed
/// ZIP and TARGZ will bundle objectgroups into subfolders
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsStreamLinkRequest {
    #[prost(enumeration="get_object_groups_stream_link_request::StreamType", tag="3")]
    pub stream_type: i32,
    /// Expiry time of the link
    /// This is the maximum expiry time, implementations can set maximum durations that can be shorter than this
    #[prost(message, optional, tag="8")]
    pub expiry: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof="get_object_groups_stream_link_request::Query", tags="4, 5, 6, 7")]
    pub query: ::core::option::Option<get_object_groups_stream_link_request::Query>,
}
/// Nested message and enum types in `GetObjectGroupsStreamLinkRequest`.
pub mod get_object_groups_stream_link_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateRangeQuery {
        #[prost(string, tag="3")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag="1")]
        pub start: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag="2")]
        pub end: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupIDsQuery {
        #[prost(string, tag="2")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="1")]
        pub object_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetQuery {
        #[prost(string, tag="1")]
        pub dataset_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetVersionQuery {
        #[prost(string, tag="1")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub dataset_version_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StreamType {
        Unspecified = 0,
        Base64newline = 1,
        Zip = 2,
        Targz = 3,
    }
    impl StreamType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StreamType::Unspecified => "STREAM_TYPE_UNSPECIFIED",
                StreamType::Base64newline => "STREAM_TYPE_BASE64NEWLINE",
                StreamType::Zip => "STREAM_TYPE_ZIP",
                StreamType::Targz => "STREAM_TYPE_TARGZ",
            }
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag="4")]
        GroupIds(GroupIDsQuery),
        #[prost(message, tag="5")]
        DateRange(DateRangeQuery),
        #[prost(message, tag="6")]
        Dataset(DatasetQuery),
        #[prost(message, tag="7")]
        DatasetVersion(DatasetVersionQuery),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsStreamLinkResponse {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetFieldRequest {
    #[prost(message, optional, tag="1")]
    pub update_request: ::core::option::Option<super::super::models::v1::UpdateFieldsRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetFieldResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetResponse {
}
// DatasetVersion related models

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag="5")]
    pub annotations: ::prost::alloc::vec::Vec<super::super::models::v1::Annotation>,
    #[prost(string, repeated, tag="7")]
    pub object_group_revision_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionResponse {
    #[prost(message, optional, tag="1")]
    pub dataset_version: ::core::option::Option<super::super::models::v1::DatasetVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionObjectGroupsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionObjectGroupsResponse {
    #[prost(message, repeated, tag="1")]
    pub object_group_revisions: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionResponse {
}
/// Generated client implementations.
pub mod dataset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Dataset management service
    /// Manages all dataset related services
    /// All data objects are associated with one data dataset
    /// Dataset versions group these data objects, which makes them reusable
    #[derive(Debug, Clone)]
    pub struct DatasetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatasetServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatasetServiceClient<T>
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
        ) -> DatasetServiceClient<InterceptedService<T, F>>
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
            DatasetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateNewDataset Creates a new dataset and associates it with a dataset
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::CreateDatasetResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Dataset Returns a specific dataset
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::GetDatasetResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Versions of a dataset
        pub async fn get_dataset_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionsResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all object groups of a dataset
        pub async fn get_dataset_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetObjectGroupsRequest>,
        ) -> Result<
            tonic::Response<super::GetDatasetObjectGroupsResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a signed link that can be used to download all objects from the
        /// specified request The link is signed using hmac and the resulting data can
        /// be shared without exposing any secrets
        pub async fn get_object_groups_stream_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsStreamLinkRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsStreamLinkResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetObjectGroupsStreamLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a field of a dataset
        pub async fn update_dataset_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetFieldRequest>,
        ) -> Result<tonic::Response<super::UpdateDatasetFieldResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/UpdateDatasetField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteDataset Delete a dataset
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all object groups that were created within a specific date range
        /// The date range is not the date when the data was created in the system but
        /// byte the externally date that indicates the actual creation of the data
        /// rather than the date the data was ingested into the system
        pub async fn get_object_group_revisions_in_date_range(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetObjectGroupRevisionsInDateRangeRequest,
            >,
        ) -> Result<
            tonic::Response<super::GetObjectGroupRevisionsInDateRangeResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetObjectGroupRevisionsInDateRange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ReleaseDatasetVersion Release a new dataset version
        pub async fn release_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseDatasetVersionRequest>,
        ) -> Result<
            tonic::Response<super::ReleaseDatasetVersionResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/ReleaseDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_dataset_version_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionObjectGroupsRequest>,
        ) -> Result<
            tonic::Response<super::GetDatasetVersionObjectGroupsResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersionObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetVersionRequest>,
        ) -> Result<
            tonic::Response<super::DeleteDatasetVersionResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetService/DeleteDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dataset_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DatasetServiceServer.
    #[async_trait]
    pub trait DatasetService: Send + Sync + 'static {
        /// CreateNewDataset Creates a new dataset and associates it with a dataset
        async fn create_dataset(
            &self,
            request: tonic::Request<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::CreateDatasetResponse>, tonic::Status>;
        /// Dataset Returns a specific dataset
        async fn get_dataset(
            &self,
            request: tonic::Request<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::GetDatasetResponse>, tonic::Status>;
        /// Lists Versions of a dataset
        async fn get_dataset_versions(
            &self,
            request: tonic::Request<super::GetDatasetVersionsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionsResponse>, tonic::Status>;
        /// Lists all object groups of a dataset
        async fn get_dataset_object_groups(
            &self,
            request: tonic::Request<super::GetDatasetObjectGroupsRequest>,
        ) -> Result<
            tonic::Response<super::GetDatasetObjectGroupsResponse>,
            tonic::Status,
        >;
        /// Returns a signed link that can be used to download all objects from the
        /// specified request The link is signed using hmac and the resulting data can
        /// be shared without exposing any secrets
        async fn get_object_groups_stream_link(
            &self,
            request: tonic::Request<super::GetObjectGroupsStreamLinkRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsStreamLinkResponse>,
            tonic::Status,
        >;
        /// Updates a field of a dataset
        async fn update_dataset_field(
            &self,
            request: tonic::Request<super::UpdateDatasetFieldRequest>,
        ) -> Result<tonic::Response<super::UpdateDatasetFieldResponse>, tonic::Status>;
        /// DeleteDataset Delete a dataset
        async fn delete_dataset(
            &self,
            request: tonic::Request<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetResponse>, tonic::Status>;
        /// Returns all object groups that were created within a specific date range
        /// The date range is not the date when the data was created in the system but
        /// byte the externally date that indicates the actual creation of the data
        /// rather than the date the data was ingested into the system
        async fn get_object_group_revisions_in_date_range(
            &self,
            request: tonic::Request<super::GetObjectGroupRevisionsInDateRangeRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupRevisionsInDateRangeResponse>,
            tonic::Status,
        >;
        /// ReleaseDatasetVersion Release a new dataset version
        async fn release_dataset_version(
            &self,
            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
        ) -> Result<
            tonic::Response<super::ReleaseDatasetVersionResponse>,
            tonic::Status,
        >;
        async fn get_dataset_version(
            &self,
            request: tonic::Request<super::GetDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionResponse>, tonic::Status>;
        async fn get_dataset_version_object_groups(
            &self,
            request: tonic::Request<super::GetDatasetVersionObjectGroupsRequest>,
        ) -> Result<
            tonic::Response<super::GetDatasetVersionObjectGroupsResponse>,
            tonic::Status,
        >;
        async fn delete_dataset_version(
            &self,
            request: tonic::Request<super::DeleteDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetVersionResponse>, tonic::Status>;
    }
    /// Dataset management service
    /// Manages all dataset related services
    /// All data objects are associated with one data dataset
    /// Dataset versions group these data objects, which makes them reusable
    #[derive(Debug)]
    pub struct DatasetServiceServer<T: DatasetService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DatasetService> DatasetServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DatasetServiceServer<T>
    where
        T: DatasetService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sciobjsdb.api.storage.services.v1.DatasetService/CreateDataset" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::CreateDatasetRequest>
                    for CreateDatasetSvc<T> {
                        type Response = super::CreateDatasetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_dataset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDataset" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::GetDatasetRequest>
                    for GetDatasetSvc<T> {
                        type Response = super::GetDatasetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_dataset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersions" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionsSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::GetDatasetVersionsRequest>
                    for GetDatasetVersionsSvc<T> {
                        type Response = super::GetDatasetVersionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_dataset_versions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatasetVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetObjectGroupsSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::GetDatasetObjectGroupsRequest>
                    for GetDatasetObjectGroupsSvc<T> {
                        type Response = super::GetDatasetObjectGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetObjectGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_dataset_object_groups(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatasetObjectGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetObjectGroupsStreamLink" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsStreamLinkSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<
                        super::GetObjectGroupsStreamLinkRequest,
                    > for GetObjectGroupsStreamLinkSvc<T> {
                        type Response = super::GetObjectGroupsStreamLinkResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetObjectGroupsStreamLinkRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups_stream_link(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsStreamLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/UpdateDatasetField" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetFieldSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::UpdateDatasetFieldRequest>
                    for UpdateDatasetFieldSvc<T> {
                        type Response = super::UpdateDatasetFieldResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatasetFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_dataset_field(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDatasetFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/DeleteDataset" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::DeleteDatasetRequest>
                    for DeleteDatasetSvc<T> {
                        type Response = super::DeleteDatasetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_dataset(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetObjectGroupRevisionsInDateRange" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupRevisionsInDateRangeSvc<T: DatasetService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<
                        super::GetObjectGroupRevisionsInDateRangeRequest,
                    > for GetObjectGroupRevisionsInDateRangeSvc<T> {
                        type Response = super::GetObjectGroupRevisionsInDateRangeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetObjectGroupRevisionsInDateRangeRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .get_object_group_revisions_in_date_range(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupRevisionsInDateRangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/ReleaseDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::ReleaseDatasetVersionRequest>
                    for ReleaseDatasetVersionSvc<T> {
                        type Response = super::ReleaseDatasetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).release_dataset_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReleaseDatasetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::GetDatasetVersionRequest>
                    for GetDatasetVersionSvc<T> {
                        type Response = super::GetDatasetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_dataset_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatasetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/GetDatasetVersionObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionObjectGroupsSvc<T: DatasetService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<
                        super::GetDatasetVersionObjectGroupsRequest,
                    > for GetDatasetVersionObjectGroupsSvc<T> {
                        type Response = super::GetDatasetVersionObjectGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDatasetVersionObjectGroupsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_dataset_version_object_groups(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatasetVersionObjectGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetService/DeleteDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::DeleteDatasetVersionRequest>
                    for DeleteDatasetVersionSvc<T> {
                        type Response = super::DeleteDatasetVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_dataset_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteDatasetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: DatasetService> Clone for DatasetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DatasetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatasetService> tonic::server::NamedService for DatasetServiceServer<T> {
        const NAME: &'static str = "sciobjsdb.api.storage.services.v1.DatasetService";
    }
}
/// Request to create a project
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag="5")]
    pub annotations: ::prost::alloc::vec::Vec<super::super::models::v1::Annotation>,
}
/// CreateProjectResponse Returns the id of the created project
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// AddUserToProjectRequest Request to add a user to a project
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectRequest {
    /// oauth2 id of the added user
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    /// rights of the user
    #[prost(enumeration="super::super::models::v1::Right", repeated, tag="2")]
    pub scope: ::prost::alloc::vec::Vec<i32>,
    /// projectid of the project the user should be granted access to
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenResponse {
    /// API token with some additional information
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<super::super::models::v1::ApiToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDatasetsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDatasetsResponse {
    /// Queried datasets 
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<super::super::models::v1::Dataset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsResponse {
    /// Queried projects
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<super::super::models::v1::Project>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectResponse {
    /// Quried project
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<super::super::models::v1::Project>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenResponse {
    /// Queried API token
    #[prost(message, repeated, tag="1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v1::ApiToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectResponse {
}
/// Generated client implementations.
pub mod project_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ProjectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProjectServiceClient<T>
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
        ) -> ProjectServiceClient<InterceptedService<T, F>>
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
            ProjectServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///CreateProject creates a new projects
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/CreateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///AddUserToProject Adds a new user to a given project by its id
        pub async fn add_user_to_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/AddUserToProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///CreateAPIToken Creates an API token to authenticate
        pub async fn create_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/CreateAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetProjectDatasets Returns all datasets that belong to a certain project
        pub async fn get_project_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectDatasetsRequest>,
        ) -> Result<tonic::Response<super::GetProjectDatasetsResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetProjectDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetUserProjects Returns all projects that a specified user has access to
        pub async fn get_user_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetUserProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///GetProject Returns the specified project
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all API token for a specific user, based on the provided oauth2 token
        pub async fn get_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///DeleteProject Deletes a specific project
        ///Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database
        pub async fn delete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProjectRequest>,
        ) -> Result<tonic::Response<super::DeleteProjectResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/DeleteProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///DeleteAPITokenRequest Deletes the specified API Token
        pub async fn delete_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.ProjectService/DeleteAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod project_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ProjectServiceServer.
    #[async_trait]
    pub trait ProjectService: Send + Sync + 'static {
        ///CreateProject creates a new projects
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status>;
        ///AddUserToProject Adds a new user to a given project by its id
        async fn add_user_to_project(
            &self,
            request: tonic::Request<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status>;
        ///CreateAPIToken Creates an API token to authenticate
        async fn create_api_token(
            &self,
            request: tonic::Request<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status>;
        ///GetProjectDatasets Returns all datasets that belong to a certain project
        async fn get_project_datasets(
            &self,
            request: tonic::Request<super::GetProjectDatasetsRequest>,
        ) -> Result<tonic::Response<super::GetProjectDatasetsResponse>, tonic::Status>;
        ///GetUserProjects Returns all projects that a specified user has access to
        async fn get_user_projects(
            &self,
            request: tonic::Request<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status>;
        ///GetProject Returns the specified project
        async fn get_project(
            &self,
            request: tonic::Request<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status>;
        /// Returns all API token for a specific user, based on the provided oauth2 token
        async fn get_api_token(
            &self,
            request: tonic::Request<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status>;
        ///DeleteProject Deletes a specific project
        ///Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database
        async fn delete_project(
            &self,
            request: tonic::Request<super::DeleteProjectRequest>,
        ) -> Result<tonic::Response<super::DeleteProjectResponse>, tonic::Status>;
        ///DeleteAPITokenRequest Deletes the specified API Token
        async fn delete_api_token(
            &self,
            request: tonic::Request<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProjectServiceServer<T: ProjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProjectService> ProjectServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProjectServiceServer<T>
    where
        T: ProjectService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sciobjsdb.api.storage.services.v1.ProjectService/CreateProject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::CreateProjectRequest>
                    for CreateProjectSvc<T> {
                        type Response = super::CreateProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/AddUserToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::AddUserToProjectRequest>
                    for AddUserToProjectSvc<T> {
                        type Response = super::AddUserToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUserToProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_user_to_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddUserToProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/CreateAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::CreateApiTokenRequest>
                    for CreateAPITokenSvc<T> {
                        type Response = super::CreateApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_api_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAPITokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetProjectDatasets" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectDatasetsSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::GetProjectDatasetsRequest>
                    for GetProjectDatasetsSvc<T> {
                        type Response = super::GetProjectDatasetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectDatasetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_project_datasets(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectDatasetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetUserProjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserProjectsSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::GetUserProjectsRequest>
                    for GetUserProjectsSvc<T> {
                        type Response = super::GetUserProjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserProjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_projects(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserProjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetProject" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::GetProjectRequest>
                    for GetProjectSvc<T> {
                        type Response = super::GetProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/GetAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::GetApiTokenRequest>
                    for GetAPITokenSvc<T> {
                        type Response = super::GetApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_api_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAPITokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/DeleteProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::DeleteProjectRequest>
                    for DeleteProjectSvc<T> {
                        type Response = super::DeleteProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.ProjectService/DeleteAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::DeleteApiTokenRequest>
                    for DeleteAPITokenSvc<T> {
                        type Response = super::DeleteApiTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_api_token(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAPITokenSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: ProjectService> Clone for ProjectServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ProjectService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProjectService> tonic::server::NamedService for ProjectServiceServer<T> {
        const NAME: &'static str = "sciobjsdb.api.storage.services.v1.ProjectService";
    }
}
/// Generated client implementations.
pub mod dataset_objects_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DatasetObjectsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatasetObjectsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatasetObjectsServiceClient<T>
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
        ) -> DatasetObjectsServiceClient<InterceptedService<T, F>>
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
            DatasetObjectsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new object group
        pub async fn create_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/CreateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Batch request of CreateObjectGroup
        /// The call will preserve the ordering of the request in the response
        pub async fn create_object_group_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupBatchRequest>,
        ) -> Result<
            tonic::Response<super::CreateObjectGroupBatchResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/CreateObjectGroupBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/GetObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///Returns the object group with the given ID
        pub async fn get_object_group_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupRevisionRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupRevisionResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/GetObjectGroupRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an ObjectGroup
        /// This creates a new ObjectGroupRevisions
        /// It needs to be finished via FinishObjectGroupRevisionUpload before it is actually available
        /// Currently experimental
        pub async fn update_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/UpdateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finishes the upload process for an object
        /// This will change the status of the objects to "available"
        pub async fn finish_object_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishObjectUploadRequest>,
        ) -> Result<tonic::Response<super::FinishObjectUploadResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/FinishObjectUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Finishes the upload process for an objectgroup
        /// This will change the status of the objectgroup to "available"
        pub async fn finish_object_group_revision_upload(
            &mut self,
            request: impl tonic::IntoRequest<
                super::FinishObjectGroupRevisionUploadRequest,
            >,
        ) -> Result<
            tonic::Response<super::FinishObjectGroupRevisionUploadResponse>,
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/FinishObjectGroupRevisionUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the given object group
        /// This will also delete all associated objects both as metadata objects and the actual objects in the object storage
        pub async fn delete_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status> {
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
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/DeleteObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dataset_objects_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DatasetObjectsServiceServer.
    #[async_trait]
    pub trait DatasetObjectsService: Send + Sync + 'static {
        /// Creates a new object group
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status>;
        /// Batch request of CreateObjectGroup
        /// The call will preserve the ordering of the request in the response
        async fn create_object_group_batch(
            &self,
            request: tonic::Request<super::CreateObjectGroupBatchRequest>,
        ) -> Result<
            tonic::Response<super::CreateObjectGroupBatchResponse>,
            tonic::Status,
        >;
        async fn get_object_group(
            &self,
            request: tonic::Request<super::GetObjectGroupRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupResponse>, tonic::Status>;
        ///Returns the object group with the given ID
        async fn get_object_group_revision(
            &self,
            request: tonic::Request<super::GetObjectGroupRevisionRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupRevisionResponse>,
            tonic::Status,
        >;
        /// Updates an ObjectGroup
        /// This creates a new ObjectGroupRevisions
        /// It needs to be finished via FinishObjectGroupRevisionUpload before it is actually available
        /// Currently experimental
        async fn update_object_group(
            &self,
            request: tonic::Request<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status>;
        /// Finishes the upload process for an object
        /// This will change the status of the objects to "available"
        async fn finish_object_upload(
            &self,
            request: tonic::Request<super::FinishObjectUploadRequest>,
        ) -> Result<tonic::Response<super::FinishObjectUploadResponse>, tonic::Status>;
        /// Finishes the upload process for an objectgroup
        /// This will change the status of the objectgroup to "available"
        async fn finish_object_group_revision_upload(
            &self,
            request: tonic::Request<super::FinishObjectGroupRevisionUploadRequest>,
        ) -> Result<
            tonic::Response<super::FinishObjectGroupRevisionUploadResponse>,
            tonic::Status,
        >;
        /// Deletes the given object group
        /// This will also delete all associated objects both as metadata objects and the actual objects in the object storage
        async fn delete_object_group(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DatasetObjectsServiceServer<T: DatasetObjectsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DatasetObjectsService> DatasetObjectsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
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
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for DatasetObjectsServiceServer<T>
    where
        T: DatasetObjectsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/CreateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::CreateObjectGroupRequest>
                    for CreateObjectGroupSvc<T> {
                        type Response = super::CreateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/CreateObjectGroupBatch" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupBatchSvc<T: DatasetObjectsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::CreateObjectGroupBatchRequest>
                    for CreateObjectGroupBatchSvc<T> {
                        type Response = super::CreateObjectGroupBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_object_group_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateObjectGroupBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/GetObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::GetObjectGroupRequest>
                    for GetObjectGroupSvc<T> {
                        type Response = super::GetObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/GetObjectGroupRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupRevisionSvc<T: DatasetObjectsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::GetObjectGroupRevisionRequest>
                    for GetObjectGroupRevisionSvc<T> {
                        type Response = super::GetObjectGroupRevisionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupRevisionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_revision(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupRevisionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/UpdateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::UpdateObjectGroupRequest>
                    for UpdateObjectGroupSvc<T> {
                        type Response = super::UpdateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/FinishObjectUpload" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectUploadSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::FinishObjectUploadRequest>
                    for FinishObjectUploadSvc<T> {
                        type Response = super::FinishObjectUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinishObjectUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finish_object_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishObjectUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/FinishObjectGroupRevisionUpload" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectGroupRevisionUploadSvc<T: DatasetObjectsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<
                        super::FinishObjectGroupRevisionUploadRequest,
                    > for FinishObjectGroupRevisionUploadSvc<T> {
                        type Response = super::FinishObjectGroupRevisionUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::FinishObjectGroupRevisionUploadRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finish_object_group_revision_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishObjectGroupRevisionUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sciobjsdb.api.storage.services.v1.DatasetObjectsService/DeleteObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<
                        T: DatasetObjectsService,
                    > tonic::server::UnaryService<super::DeleteObjectGroupRequest>
                    for DeleteObjectGroupSvc<T> {
                        type Response = super::DeleteObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
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
    impl<T: DatasetObjectsService> Clone for DatasetObjectsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DatasetObjectsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatasetObjectsService> tonic::server::NamedService
    for DatasetObjectsServiceServer<T> {
        const NAME: &'static str = "sciobjsdb.api.storage.services.v1.DatasetObjectsService";
    }
}
