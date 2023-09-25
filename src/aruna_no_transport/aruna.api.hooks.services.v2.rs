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
    /// If empty a basic JSON template will be used
    #[prost(string, optional, tag = "3")]
    pub custom_template: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional Project/Collection/Dataset where hooks can upload results.
    #[prost(string, optional, tag = "4")]
    pub result_object: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "Method", tag = "5")]
    pub method: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLabel {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddHook {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalHook {
    #[prost(oneof = "internal_hook::InternalAction", tags = "1, 2, 3")]
    pub internal_action: ::core::option::Option<internal_hook::InternalAction>,
}
/// Nested message and enum types in `InternalHook`.
pub mod internal_hook {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InternalAction {
        #[prost(message, tag = "1")]
        AddLabel(super::AddLabel),
        #[prost(message, tag = "2")]
        AddHook(super::AddHook),
        #[prost(message, tag = "3")]
        AddRelation(super::super::super::super::storage::models::v2::Relation),
    }
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
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub trigger: ::core::option::Option<Trigger>,
    #[prost(message, optional, tag = "3")]
    pub hook: ::core::option::Option<Hook>,
    #[prost(uint64, tag = "4")]
    pub timeout: u64,
    #[prost(string, repeated, tag = "5")]
    pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
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
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub hook_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub pubkey_serial: i32,
    #[prost(oneof = "hook_callback_request::Status", tags = "1, 2")]
    pub status: ::core::option::Option<hook_callback_request::Status>,
}
/// Nested message and enum types in `HookCallbackRequest`.
pub mod hook_callback_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Status {
        #[prost(message, tag = "1")]
        Finished(super::Finished),
        #[prost(message, tag = "2")]
        Error(super::Error),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Finished {
    #[prost(message, repeated, tag = "1")]
    pub add_key_values: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::KeyValue,
    >,
    #[prost(message, repeated, tag = "2")]
    pub remove_key_values: ::prost::alloc::vec::Vec<
        super::super::super::storage::models::v2::KeyValue,
    >,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookCallbackResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectHooksRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOwnedHooksRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HookInfo {
    #[prost(string, tag = "1")]
    pub hook_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub hook: ::core::option::Option<Hook>,
    #[prost(message, optional, tag = "6")]
    pub trigger: ::core::option::Option<Trigger>,
    #[prost(uint64, tag = "7")]
    pub timeout: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectHooksResponse {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<HookInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOwnedHooksResponse {
    #[prost(message, repeated, tag = "1")]
    pub infos: ::prost::alloc::vec::Vec<HookInfo>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectsToHookRequest {
    #[prost(string, tag = "1")]
    pub hook_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProjectsToHookResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Unspecified = 0,
    HookAdded = 1,
    ObjectCreated = 2,
    LabelAdded = 3,
    StaticLabelAdded = 4,
    HookStatusChanged = 5,
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
            TriggerType::LabelAdded => "TRIGGER_TYPE_LABEL_ADDED",
            TriggerType::StaticLabelAdded => "TRIGGER_TYPE_STATIC_LABEL_ADDED",
            TriggerType::HookStatusChanged => "TRIGGER_TYPE_HOOK_STATUS_CHANGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRIGGER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TRIGGER_TYPE_HOOK_ADDED" => Some(Self::HookAdded),
            "TRIGGER_TYPE_OBJECT_CREATED" => Some(Self::ObjectCreated),
            "TRIGGER_TYPE_LABEL_ADDED" => Some(Self::LabelAdded),
            "TRIGGER_TYPE_STATIC_LABEL_ADDED" => Some(Self::StaticLabelAdded),
            "TRIGGER_TYPE_HOOK_STATUS_CHANGED" => Some(Self::HookStatusChanged),
            _ => None,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Method {
    Unspecified = 0,
    Put = 1,
    Post = 2,
}
impl Method {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Method::Unspecified => "METHOD_UNSPECIFIED",
            Method::Put => "METHOD_PUT",
            Method::Post => "METHOD_POST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "METHOD_UNSPECIFIED" => Some(Self::Unspecified),
            "METHOD_PUT" => Some(Self::Put),
            "METHOD_POST" => Some(Self::Post),
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
        /// Created Hooks are always associated with the owner that creates the hook
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
        pub async fn add_projects_to_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProjectsToHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectsToHookResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/AddProjectsToHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "AddProjectsToHook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_project_hooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectHooksResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/ListProjectHooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "ListProjectHooks",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_owned_hooks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOwnedHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOwnedHooksResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/ListOwnedHooks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.hooks.services.v2.HooksService",
                        "ListOwnedHooks",
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
        /// Created Hooks are always associated with the owner that creates the hook
        async fn create_hook(
            &self,
            request: tonic::Request<super::CreateHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateHookResponse>,
            tonic::Status,
        >;
        async fn add_projects_to_hook(
            &self,
            request: tonic::Request<super::AddProjectsToHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddProjectsToHookResponse>,
            tonic::Status,
        >;
        async fn list_project_hooks(
            &self,
            request: tonic::Request<super::ListProjectHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListProjectHooksResponse>,
            tonic::Status,
        >;
        async fn list_owned_hooks(
            &self,
            request: tonic::Request<super::ListOwnedHooksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOwnedHooksResponse>,
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
                "/aruna.api.hooks.services.v2.HooksService/AddProjectsToHook" => {
                    #[allow(non_camel_case_types)]
                    struct AddProjectsToHookSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::AddProjectsToHookRequest>
                    for AddProjectsToHookSvc<T> {
                        type Response = super::AddProjectsToHookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddProjectsToHookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_projects_to_hook(request).await
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
                        let method = AddProjectsToHookSvc(inner);
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
                "/aruna.api.hooks.services.v2.HooksService/ListProjectHooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListProjectHooksSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::ListProjectHooksRequest>
                    for ListProjectHooksSvc<T> {
                        type Response = super::ListProjectHooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProjectHooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_project_hooks(request).await
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
                        let method = ListProjectHooksSvc(inner);
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
                "/aruna.api.hooks.services.v2.HooksService/ListOwnedHooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListOwnedHooksSvc<T: HooksService>(pub Arc<T>);
                    impl<
                        T: HooksService,
                    > tonic::server::UnaryService<super::ListOwnedHooksRequest>
                    for ListOwnedHooksSvc<T> {
                        type Response = super::ListOwnedHooksResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOwnedHooksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_owned_hooks(request).await
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
                        let method = ListOwnedHooksSvc(inner);
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
