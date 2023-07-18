#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    #[prost(enumeration = "TriggerType", tag = "1")]
    pub trigger_type: i32,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalHook {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub credentials: ::core::option::Option<Credentials>,
    #[prost(string, tag = "3")]
    pub json_template: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalHook {
    #[prost(enumeration = "InternalAction", tag = "1")]
    pub internal_action: i32,
    /// Either key or target ID
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Optional value
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hook {
    #[prost(oneof = "hook::HookType", tags = "1, 2")]
    pub hook_type: ::core::option::Option<hook::HookType>,
}
/// Nested message and enum types in `Hook`.
pub mod hook {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HookType {
        #[prost(message, tag = "1")]
        ExternalHook(super::ExternalHook),
        #[prost(message, tag = "2")]
        InternalHook(super::InternalHook),
    }
}
/// Will be expanded with additional credential types
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credentials {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHookRequest {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
    #[prost(message, optional, tag = "2")]
    pub hook: ::core::option::Option<Hook>,
    #[prost(uint64, tag = "3")]
    pub timeout: u64,
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHookResponse {
    #[prost(string, tag = "1")]
    pub hook_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHookRequest {
    #[prost(string, tag = "1")]
    pub hook_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHookResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookCallbackRequest {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, repeated, tag = "2")]
    pub add_key_values: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::KeyValue,
    >,
    #[prost(message, repeated, tag = "3")]
    pub remove_key_values: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::KeyValue,
    >,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookCallbackResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHooksRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookInfo {
    #[prost(string, tag = "1")]
    pub hook_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub hook: ::core::option::Option<Hook>,
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<Trigger>,
    #[prost(uint64, tag = "4")]
    pub timeout: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHooksResponse {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<HookInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Unspecified = 0,
    HookAdded = 1,
    ObjectCreated = 2,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Unspecified => "TRIGGER_TYPE_UNSPECIFIED",
            TriggerType::HookAdded => "TRIGGER_TYPE_HOOK_ADDED",
            TriggerType::ObjectCreated => "TRIGGER_TYPE_OBJECT_CREATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRIGGER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRIGGER_TYPE_HOOK_ADDED" => Some(Self::HookAdded),
            "TRIGGER_TYPE_OBJECT_CREATED" => Some(Self::ObjectCreated),
            _ => None,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InternalAction {
    Unspecified = 0,
    AddLabel = 1,
    AddHook = 2,
    CreateRelation = 3,
}
impl InternalAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InternalAction::Unspecified => "INTERNAL_ACTION_UNSPECIFIED",
            InternalAction::AddLabel => "INTERNAL_ACTION_ADD_LABEL",
            InternalAction::AddHook => "INTERNAL_ACTION_ADD_HOOK",
            InternalAction::CreateRelation => "INTERNAL_ACTION_CREATE_RELATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTERNAL_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "INTERNAL_ACTION_ADD_LABEL" => Some(Self::AddLabel),
            "INTERNAL_ACTION_ADD_HOOK" => Some(Self::AddHook),
            "INTERNAL_ACTION_CREATE_RELATION" => Some(Self::CreateRelation),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod hooks_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// HooksService
    ///
    /// Status: ALPHA
    ///
    /// A service that enables automatic Hook scheduling
    #[derive(Debug, Clone)]
    pub struct HooksServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HooksServiceClient<tonic::transport::Channel> {
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
    impl<T> HooksServiceClient<T>
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
        ) -> HooksServiceClient<InterceptedService<T, F>>
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
            HooksServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateHookResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/CreateHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "CreateHook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_hooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHooksResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/ListHooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "ListHooks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteHookResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/DeleteHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "DeleteHook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn hook_callback(
            &mut self,
            request: impl tonic::IntoRequest<super::HookCallbackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HookCallbackResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/HookCallback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "HookCallback",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod hooks_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with HooksServiceServer.
    #[async_trait]
    pub trait HooksService: Send + Sync + 'static {
        async fn create_hook(
            &self,
            request: tonic::Request<super::CreateHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateHookResponse>,
            tonic::Status,
        >;
        async fn list_hooks(
            &self,
            request: tonic::Request<super::ListHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListHooksResponse>,
            tonic::Status,
        >;
        async fn delete_hook(
            &self,
            request: tonic::Request<super::DeleteHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteHookResponse>,
            tonic::Status,
        >;
        async fn hook_callback(
            &self,
            request: tonic::Request<super::HookCallbackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::HookCallbackResponse>,
            tonic::Status,
        >;
    }
    /// HooksService
    ///
    /// Status: ALPHA
    ///
    /// A service that enables automatic Hook scheduling
    #[derive(Debug)]
    pub struct HooksServiceServer<T: HooksService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: HooksService> HooksServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for HooksServiceServer<T>
    where
        T: HooksService,
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
                "/aruna.api.hooks.services.v2.HooksService/CreateHook" => {
                    #[allow(non_camel_case_types)]
                    struct CreateHookSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::CreateHookRequest>
                    for CreateHookSvc<T> {
                        type Response = super::CreateHookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateHookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_hook(request).await };
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
                        let method = CreateHookSvc(inner);
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
                "/aruna.api.hooks.services.v2.HooksService/ListHooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListHooksSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::ListHooksRequest>
                    for ListHooksSvc<T> {
                        type Response = super::ListHooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListHooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_hooks(request).await };
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
                        let method = ListHooksSvc(inner);
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
                "/aruna.api.hooks.services.v2.HooksService/DeleteHook" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteHookSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::DeleteHookRequest>
                    for DeleteHookSvc<T> {
                        type Response = super::DeleteHookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteHookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_hook(request).await };
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
                        let method = DeleteHookSvc(inner);
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
                "/aruna.api.hooks.services.v2.HooksService/HookCallback" => {
                    #[allow(non_camel_case_types)]
                    struct HookCallbackSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::HookCallbackRequest>
                    for HookCallbackSvc<T> {
                        type Response = super::HookCallbackResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HookCallbackRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).hook_callback(request).await
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
                        let method = HookCallbackSvc(inner);
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
    impl<T: HooksService> Clone for HooksServiceServer<T> {
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
    impl<T: HooksService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: HooksService> tonic::server::NamedService for HooksServiceServer<T> {
        const NAME: &'static str = "aruna.api.hooks.services.v2.HooksService";
    }
}
