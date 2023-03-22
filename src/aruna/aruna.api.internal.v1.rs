#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    /// Should include the APItoken
    #[prost(string, tag = "1")]
    pub secretkey: ::prost::alloc::string::String,
    /// Is the API-Token ID
    #[prost(string, tag = "2")]
    pub accesskey: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Identifier {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "IdType", tag = "2")]
    pub idtype: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeRequest {
    /// The resource type
    #[prost(enumeration = "super::super::storage::models::v1::ResourceType", tag = "1")]
    pub resource: i32,
    /// Id of the resource (PATH / OBJECT UUID)
    #[prost(message, optional, tag = "2")]
    pub identifier: ::core::option::Option<Identifier>,
    /// Which action should be performed (CRUD)
    #[prost(
        enumeration = "super::super::storage::models::v1::ResourceAction",
        tag = "3"
    )]
    pub resource_action: i32,
    /// Authorization
    #[prost(message, optional, tag = "4")]
    pub authorization: ::core::option::Option<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeResponse {
    /// Ok -> Authorization granted, empty or not ok -> dismiss
    #[prost(bool, tag = "1")]
    pub ok: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretRequest {
    #[prost(string, tag = "1")]
    pub accesskey: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretResponse {
    #[prost(message, optional, tag = "1")]
    pub authorization: ::core::option::Option<Authorization>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdType {
    Unspecified = 0,
    Uuid = 1,
    Path = 2,
}
impl IdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdType::Unspecified => "ID_TYPE_UNSPECIFIED",
            IdType::Uuid => "ID_TYPE_UUID",
            IdType::Path => "ID_TYPE_PATH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ID_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ID_TYPE_UUID" => Some(Self::Uuid),
            "ID_TYPE_PATH" => Some(Self::Path),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod internal_authorize_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InternalAuthorizeServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InternalAuthorizeServiceClient<tonic::transport::Channel> {
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
    impl<T> InternalAuthorizeServiceClient<T>
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
        ) -> InternalAuthorizeServiceClient<InterceptedService<T, F>>
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
            InternalAuthorizeServiceClient::new(
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
        pub async fn authorize(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeRequest>,
        ) -> Result<tonic::Response<super::AuthorizeResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalAuthorizeService/Authorize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::GetSecretResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalAuthorizeService/GetSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod internal_authorize_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InternalAuthorizeServiceServer.
    #[async_trait]
    pub trait InternalAuthorizeService: Send + Sync + 'static {
        async fn authorize(
            &self,
            request: tonic::Request<super::AuthorizeRequest>,
        ) -> Result<tonic::Response<super::AuthorizeResponse>, tonic::Status>;
        async fn get_secret(
            &self,
            request: tonic::Request<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::GetSecretResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct InternalAuthorizeServiceServer<T: InternalAuthorizeService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InternalAuthorizeService> InternalAuthorizeServiceServer<T> {
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
    for InternalAuthorizeServiceServer<T>
    where
        T: InternalAuthorizeService,
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
                "/aruna.api.internal.v1.InternalAuthorizeService/Authorize" => {
                    #[allow(non_camel_case_types)]
                    struct AuthorizeSvc<T: InternalAuthorizeService>(pub Arc<T>);
                    impl<
                        T: InternalAuthorizeService,
                    > tonic::server::UnaryService<super::AuthorizeRequest>
                    for AuthorizeSvc<T> {
                        type Response = super::AuthorizeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthorizeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).authorize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthorizeSvc(inner);
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
                "/aruna.api.internal.v1.InternalAuthorizeService/GetSecret" => {
                    #[allow(non_camel_case_types)]
                    struct GetSecretSvc<T: InternalAuthorizeService>(pub Arc<T>);
                    impl<
                        T: InternalAuthorizeService,
                    > tonic::server::UnaryService<super::GetSecretRequest>
                    for GetSecretSvc<T> {
                        type Response = super::GetSecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_secret(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSecretSvc(inner);
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
    impl<T: InternalAuthorizeService> Clone for InternalAuthorizeServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InternalAuthorizeService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InternalAuthorizeService> tonic::server::NamedService
    for InternalAuthorizeServiceServer<T> {
        const NAME: &'static str = "aruna.api.internal.v1.InternalAuthorizeService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmittedResource {
    #[prost(oneof = "emitted_resource::Resource", tags = "1, 2, 3, 4")]
    pub resource: ::core::option::Option<emitted_resource::Resource>,
}
/// Nested message and enum types in `EmittedResource`.
pub mod emitted_resource {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(message, tag = "1")]
        Project(super::ProjectResource),
        #[prost(message, tag = "2")]
        Collection(super::CollectionResource),
        #[prost(message, tag = "3")]
        Object(super::ObjectResource),
        #[prost(message, tag = "4")]
        ObjectGroup(super::ObjectGroupResource),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectResource {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionResource {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectResource {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shared_object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub object_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupResource {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shared_object_group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub object_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    /// The resource Type e.g. Collection / Object etc.
    #[prost(enumeration = "super::super::storage::models::v1::ResourceType", tag = "1")]
    pub event_resource: i32,
    /// The resource ID
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    /// Event type (CRUD)
    #[prost(
        enumeration = "super::super::notification::services::v1::EventType",
        tag = "3"
    )]
    pub event_type: i32,
    /// All relations of the resource, only parents are shown
    #[prost(message, repeated, tag = "4")]
    pub resources: ::prost::alloc::vec::Vec<EmittedResource>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamGroup {
    /// Stream group ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Event this streamgroup is listening for
    #[prost(
        enumeration = "super::super::notification::services::v1::EventType",
        tag = "2"
    )]
    pub event_type: i32,
    /// Type of the resource (Collection, Object etc.)
    #[prost(enumeration = "super::super::storage::models::v1::ResourceType", tag = "3")]
    pub resource_type: i32,
    /// Resource ID
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// Should all "sub" resources be included
    #[prost(bool, tag = "5")]
    pub notify_on_sub_resource: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamGroupRequest {
    /// Authorization for the user who wants to create this stream group
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Event type
    #[prost(
        enumeration = "super::super::notification::services::v1::EventType",
        tag = "2"
    )]
    pub event_type: i32,
    /// Type of the resource (Collection, Object etc.)
    #[prost(enumeration = "super::super::storage::models::v1::ResourceType", tag = "3")]
    pub resource_type: i32,
    /// Resource ID
    #[prost(string, tag = "4")]
    pub resource_id: ::prost::alloc::string::String,
    /// Should all "sub" resources be included
    #[prost(bool, tag = "5")]
    pub notify_on_sub_resource: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamGroupResponse {
    /// The stream_group
    #[prost(message, optional, tag = "1")]
    pub stream_group: ::core::option::Option<StreamGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamGroupRequest {
    /// User token
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Stream group ID
    #[prost(string, tag = "2")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStreamGroupResponse {
    /// Stream group
    #[prost(message, optional, tag = "1")]
    pub stream_group: ::core::option::Option<StreamGroup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamGroupRequest {
    /// User token
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Stream group ID
    #[prost(string, tag = "2")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamGroupResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedRevisionRequest {
    /// Resource Type (ObjectGroup or Object)
    #[prost(enumeration = "super::super::storage::models::v1::ResourceType", tag = "1")]
    pub resource_type: i32,
    /// Resource ID
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedRevisionResponse {
    /// Shared revision ID
    #[prost(string, tag = "1")]
    pub shared_revision_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod internal_event_emitter_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service hosted by the notification service application
    /// the API server emits events to the notification service
    /// Server --> Notification System
    #[derive(Debug, Clone)]
    pub struct InternalEventEmitterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InternalEventEmitterServiceClient<tonic::transport::Channel> {
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
    impl<T> InternalEventEmitterServiceClient<T>
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
        ) -> InternalEventEmitterServiceClient<InterceptedService<T, F>>
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
            InternalEventEmitterServiceClient::new(
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
        pub async fn emit_event(
            &mut self,
            request: impl tonic::IntoRequest<super::EmitEventRequest>,
        ) -> Result<tonic::Response<super::EmitEventResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalEventEmitterService/EmitEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod internal_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service that allows the notification service to issue requests
    /// to the server application
    /// Notification System --> Server
    #[derive(Debug, Clone)]
    pub struct InternalEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InternalEventServiceClient<tonic::transport::Channel> {
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
    impl<T> InternalEventServiceClient<T>
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
        ) -> InternalEventServiceClient<InterceptedService<T, F>>
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
            InternalEventServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_stream_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStreamGroupRequest>,
        ) -> Result<tonic::Response<super::CreateStreamGroupResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalEventService/CreateStreamGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_stream_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStreamGroupRequest>,
        ) -> Result<tonic::Response<super::GetStreamGroupResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalEventService/GetStreamGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_stream_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStreamGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteStreamGroupResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalEventService/DeleteStreamGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_shared_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSharedRevisionRequest>,
        ) -> Result<tonic::Response<super::GetSharedRevisionResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalEventService/GetSharedRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod internal_event_emitter_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InternalEventEmitterServiceServer.
    #[async_trait]
    pub trait InternalEventEmitterService: Send + Sync + 'static {
        async fn emit_event(
            &self,
            request: tonic::Request<super::EmitEventRequest>,
        ) -> Result<tonic::Response<super::EmitEventResponse>, tonic::Status>;
    }
    /// Service hosted by the notification service application
    /// the API server emits events to the notification service
    /// Server --> Notification System
    #[derive(Debug)]
    pub struct InternalEventEmitterServiceServer<T: InternalEventEmitterService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InternalEventEmitterService> InternalEventEmitterServiceServer<T> {
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
    for InternalEventEmitterServiceServer<T>
    where
        T: InternalEventEmitterService,
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
                "/aruna.api.internal.v1.InternalEventEmitterService/EmitEvent" => {
                    #[allow(non_camel_case_types)]
                    struct EmitEventSvc<T: InternalEventEmitterService>(pub Arc<T>);
                    impl<
                        T: InternalEventEmitterService,
                    > tonic::server::UnaryService<super::EmitEventRequest>
                    for EmitEventSvc<T> {
                        type Response = super::EmitEventResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmitEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).emit_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EmitEventSvc(inner);
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
    impl<T: InternalEventEmitterService> Clone for InternalEventEmitterServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InternalEventEmitterService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InternalEventEmitterService> tonic::server::NamedService
    for InternalEventEmitterServiceServer<T> {
        const NAME: &'static str = "aruna.api.internal.v1.InternalEventEmitterService";
    }
}
/// Generated server implementations.
pub mod internal_event_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InternalEventServiceServer.
    #[async_trait]
    pub trait InternalEventService: Send + Sync + 'static {
        async fn create_stream_group(
            &self,
            request: tonic::Request<super::CreateStreamGroupRequest>,
        ) -> Result<tonic::Response<super::CreateStreamGroupResponse>, tonic::Status>;
        async fn get_stream_group(
            &self,
            request: tonic::Request<super::GetStreamGroupRequest>,
        ) -> Result<tonic::Response<super::GetStreamGroupResponse>, tonic::Status>;
        async fn delete_stream_group(
            &self,
            request: tonic::Request<super::DeleteStreamGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteStreamGroupResponse>, tonic::Status>;
        async fn get_shared_revision(
            &self,
            request: tonic::Request<super::GetSharedRevisionRequest>,
        ) -> Result<tonic::Response<super::GetSharedRevisionResponse>, tonic::Status>;
    }
    /// Service that allows the notification service to issue requests
    /// to the server application
    /// Notification System --> Server
    #[derive(Debug)]
    pub struct InternalEventServiceServer<T: InternalEventService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InternalEventService> InternalEventServiceServer<T> {
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
    for InternalEventServiceServer<T>
    where
        T: InternalEventService,
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
                "/aruna.api.internal.v1.InternalEventService/CreateStreamGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStreamGroupSvc<T: InternalEventService>(pub Arc<T>);
                    impl<
                        T: InternalEventService,
                    > tonic::server::UnaryService<super::CreateStreamGroupRequest>
                    for CreateStreamGroupSvc<T> {
                        type Response = super::CreateStreamGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStreamGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_stream_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateStreamGroupSvc(inner);
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
                "/aruna.api.internal.v1.InternalEventService/GetStreamGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetStreamGroupSvc<T: InternalEventService>(pub Arc<T>);
                    impl<
                        T: InternalEventService,
                    > tonic::server::UnaryService<super::GetStreamGroupRequest>
                    for GetStreamGroupSvc<T> {
                        type Response = super::GetStreamGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStreamGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_stream_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStreamGroupSvc(inner);
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
                "/aruna.api.internal.v1.InternalEventService/DeleteStreamGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteStreamGroupSvc<T: InternalEventService>(pub Arc<T>);
                    impl<
                        T: InternalEventService,
                    > tonic::server::UnaryService<super::DeleteStreamGroupRequest>
                    for DeleteStreamGroupSvc<T> {
                        type Response = super::DeleteStreamGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteStreamGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_stream_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteStreamGroupSvc(inner);
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
                "/aruna.api.internal.v1.InternalEventService/GetSharedRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetSharedRevisionSvc<T: InternalEventService>(pub Arc<T>);
                    impl<
                        T: InternalEventService,
                    > tonic::server::UnaryService<super::GetSharedRevisionRequest>
                    for GetSharedRevisionSvc<T> {
                        type Response = super::GetSharedRevisionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSharedRevisionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_shared_revision(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSharedRevisionSvc(inner);
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
    impl<T: InternalEventService> Clone for InternalEventServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InternalEventService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InternalEventService> tonic::server::NamedService
    for InternalEventServiceServer<T> {
        const NAME: &'static str = "aruna.api.internal.v1.InternalEventService";
    }
}
/// Locations is the path to the requested data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(enumeration = "LocationType", tag = "1")]
    pub r#type: i32,
    /// This is the bucket name for S3. This is the folder name
    #[prost(string, tag = "2")]
    pub bucket: ::prost::alloc::string::String,
    /// for local file.
    ///
    /// This is the key name for S3. This is the file name for local file.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub endpoint_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_compressed: bool,
    #[prost(bool, tag = "6")]
    pub is_encrypted: bool,
    #[prost(string, tag = "7")]
    pub encryption_key: ::prost::alloc::string::String,
}
/// Etag / Part combination to finish a presigned multipart upload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartETag {
    #[prost(int64, tag = "1")]
    pub part_number: i64,
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub collection_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitMultipartUploadResponse {
    #[prost(string, tag = "1")]
    pub upload_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub part_etags: ::prost::alloc::vec::Vec<PartETag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishMultipartUploadResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeObjectRequest {
    /// This should be stored temporarily
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// This should be stored temporarily
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    /// This will be the final location of the object
    #[prost(message, optional, tag = "3")]
    pub location: ::core::option::Option<Location>,
    #[prost(message, repeated, tag = "4")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::storage::models::v1::Hash>,
    #[prost(int64, tag = "5")]
    pub content_length: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeObjectResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrCreateEncryptionKeyRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub endpoint_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrCreateEncryptionKeyResponse {
    #[prost(string, tag = "1")]
    pub encryption_key: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub created: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrCreateObjectByPathRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// Validate if the user has correct permissions
    #[prost(string, tag = "2")]
    pub access_key: ::prost::alloc::string::String,
    /// Will only be used if no staging object exists
    #[prost(message, optional, tag = "3")]
    pub object: ::core::option::Option<super::super::storage::services::v1::StageObject>,
    /// Should this only get the object NOT create -> fail
    #[prost(bool, tag = "4")]
    pub get_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrCreateObjectByPathResponse {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::storage::models::v1::DataClass", tag = "3")]
    pub dataclass: i32,
    #[prost(message, repeated, tag = "4")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::storage::models::v1::Hash>,
    #[prost(int64, tag = "5")]
    pub revision_number: i64,
    #[prost(bool, tag = "6")]
    pub created: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectLocationRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub endpoint_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::storage::models::v1::Object>,
    #[prost(message, optional, tag = "2")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByBucketRequest {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub access_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByBucketResponse {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
}
/// Enum to support multiple target Locations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationType {
    Unspecified = 0,
    S3 = 1,
    File = 2,
}
impl LocationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LocationType::Unspecified => "LOCATION_TYPE_UNSPECIFIED",
            LocationType::S3 => "LOCATION_TYPE_S3",
            LocationType::File => "LOCATION_TYPE_FILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LOCATION_TYPE_S3" => Some(Self::S3),
            "LOCATION_TYPE_FILE" => Some(Self::File),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod internal_proxy_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InternalProxyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InternalProxyServiceClient<tonic::transport::Channel> {
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
    impl<T> InternalProxyServiceClient<T>
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
        ) -> InternalProxyServiceClient<InterceptedService<T, F>>
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
            InternalProxyServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn init_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::InitMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::InitMultipartUploadResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalProxyService/InitMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn finish_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishMultipartUploadRequest>,
        ) -> Result<
            tonic::Response<super::FinishMultipartUploadResponse>,
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
                "/aruna.api.internal.v1.InternalProxyService/FinishMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalProxyService/DeleteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod internal_proxy_notifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// This service enables a "return" channel for dataproxy to aruna server communication
    /// Mainly used to notify the backend of validation / move events after the upload of new files
    #[derive(Debug, Clone)]
    pub struct InternalProxyNotifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InternalProxyNotifierServiceClient<tonic::transport::Channel> {
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
    impl<T> InternalProxyNotifierServiceClient<T>
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
        ) -> InternalProxyNotifierServiceClient<InterceptedService<T, F>>
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
            InternalProxyNotifierServiceClient::new(
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
        pub async fn get_or_create_object_by_path(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrCreateObjectByPathRequest>,
        ) -> Result<
            tonic::Response<super::GetOrCreateObjectByPathResponse>,
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetOrCreateObjectByPath",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn finalize_object(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeObjectRequest>,
        ) -> Result<tonic::Response<super::FinalizeObjectResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/FinalizeObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_or_create_encryption_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrCreateEncryptionKeyRequest>,
        ) -> Result<
            tonic::Response<super::GetOrCreateEncryptionKeyResponse>,
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetOrCreateEncryptionKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectLocationRequest>,
        ) -> Result<tonic::Response<super::GetObjectLocationResponse>, tonic::Status> {
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetObjectLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_collection_by_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionByBucketRequest>,
        ) -> Result<
            tonic::Response<super::GetCollectionByBucketResponse>,
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetCollectionByBucket",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod internal_proxy_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InternalProxyServiceServer.
    #[async_trait]
    pub trait InternalProxyService: Send + Sync + 'static {
        async fn init_multipart_upload(
            &self,
            request: tonic::Request<super::InitMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::InitMultipartUploadResponse>, tonic::Status>;
        async fn finish_multipart_upload(
            &self,
            request: tonic::Request<super::FinishMultipartUploadRequest>,
        ) -> Result<
            tonic::Response<super::FinishMultipartUploadResponse>,
            tonic::Status,
        >;
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct InternalProxyServiceServer<T: InternalProxyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InternalProxyService> InternalProxyServiceServer<T> {
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
    for InternalProxyServiceServer<T>
    where
        T: InternalProxyService,
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
                "/aruna.api.internal.v1.InternalProxyService/InitMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct InitMultipartUploadSvc<T: InternalProxyService>(pub Arc<T>);
                    impl<
                        T: InternalProxyService,
                    > tonic::server::UnaryService<super::InitMultipartUploadRequest>
                    for InitMultipartUploadSvc<T> {
                        type Response = super::InitMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitMultipartUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).init_multipart_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitMultipartUploadSvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyService/FinishMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct FinishMultipartUploadSvc<T: InternalProxyService>(pub Arc<T>);
                    impl<
                        T: InternalProxyService,
                    > tonic::server::UnaryService<super::FinishMultipartUploadRequest>
                    for FinishMultipartUploadSvc<T> {
                        type Response = super::FinishMultipartUploadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinishMultipartUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finish_multipart_upload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishMultipartUploadSvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyService/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: InternalProxyService>(pub Arc<T>);
                    impl<
                        T: InternalProxyService,
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectSvc(inner);
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
    impl<T: InternalProxyService> Clone for InternalProxyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InternalProxyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InternalProxyService> tonic::server::NamedService
    for InternalProxyServiceServer<T> {
        const NAME: &'static str = "aruna.api.internal.v1.InternalProxyService";
    }
}
/// Generated server implementations.
pub mod internal_proxy_notifier_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InternalProxyNotifierServiceServer.
    #[async_trait]
    pub trait InternalProxyNotifierService: Send + Sync + 'static {
        async fn get_or_create_object_by_path(
            &self,
            request: tonic::Request<super::GetOrCreateObjectByPathRequest>,
        ) -> Result<
            tonic::Response<super::GetOrCreateObjectByPathResponse>,
            tonic::Status,
        >;
        async fn finalize_object(
            &self,
            request: tonic::Request<super::FinalizeObjectRequest>,
        ) -> Result<tonic::Response<super::FinalizeObjectResponse>, tonic::Status>;
        async fn get_or_create_encryption_key(
            &self,
            request: tonic::Request<super::GetOrCreateEncryptionKeyRequest>,
        ) -> Result<
            tonic::Response<super::GetOrCreateEncryptionKeyResponse>,
            tonic::Status,
        >;
        async fn get_object_location(
            &self,
            request: tonic::Request<super::GetObjectLocationRequest>,
        ) -> Result<tonic::Response<super::GetObjectLocationResponse>, tonic::Status>;
        async fn get_collection_by_bucket(
            &self,
            request: tonic::Request<super::GetCollectionByBucketRequest>,
        ) -> Result<
            tonic::Response<super::GetCollectionByBucketResponse>,
            tonic::Status,
        >;
    }
    /// This service enables a "return" channel for dataproxy to aruna server communication
    /// Mainly used to notify the backend of validation / move events after the upload of new files
    #[derive(Debug)]
    pub struct InternalProxyNotifierServiceServer<T: InternalProxyNotifierService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InternalProxyNotifierService> InternalProxyNotifierServiceServer<T> {
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
    for InternalProxyNotifierServiceServer<T>
    where
        T: InternalProxyNotifierService,
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetOrCreateObjectByPath" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrCreateObjectByPathSvc<T: InternalProxyNotifierService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InternalProxyNotifierService,
                    > tonic::server::UnaryService<super::GetOrCreateObjectByPathRequest>
                    for GetOrCreateObjectByPathSvc<T> {
                        type Response = super::GetOrCreateObjectByPathResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOrCreateObjectByPathRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_or_create_object_by_path(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOrCreateObjectByPathSvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/FinalizeObject" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeObjectSvc<T: InternalProxyNotifierService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InternalProxyNotifierService,
                    > tonic::server::UnaryService<super::FinalizeObjectRequest>
                    for FinalizeObjectSvc<T> {
                        type Response = super::FinalizeObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizeObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finalize_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinalizeObjectSvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetOrCreateEncryptionKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrCreateEncryptionKeySvc<T: InternalProxyNotifierService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InternalProxyNotifierService,
                    > tonic::server::UnaryService<super::GetOrCreateEncryptionKeyRequest>
                    for GetOrCreateEncryptionKeySvc<T> {
                        type Response = super::GetOrCreateEncryptionKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOrCreateEncryptionKeyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_or_create_encryption_key(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOrCreateEncryptionKeySvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetObjectLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectLocationSvc<T: InternalProxyNotifierService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InternalProxyNotifierService,
                    > tonic::server::UnaryService<super::GetObjectLocationRequest>
                    for GetObjectLocationSvc<T> {
                        type Response = super::GetObjectLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectLocationSvc(inner);
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
                "/aruna.api.internal.v1.InternalProxyNotifierService/GetCollectionByBucket" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionByBucketSvc<T: InternalProxyNotifierService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: InternalProxyNotifierService,
                    > tonic::server::UnaryService<super::GetCollectionByBucketRequest>
                    for GetCollectionByBucketSvc<T> {
                        type Response = super::GetCollectionByBucketResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionByBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_collection_by_bucket(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionByBucketSvc(inner);
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
    impl<T: InternalProxyNotifierService> Clone
    for InternalProxyNotifierServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InternalProxyNotifierService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InternalProxyNotifierService> tonic::server::NamedService
    for InternalProxyNotifierServiceServer<T> {
        const NAME: &'static str = "aruna.api.internal.v1.InternalProxyNotifierService";
    }
}
