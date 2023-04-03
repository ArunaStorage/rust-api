#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventStreamingGroupRequest {
    #[prost(
        enumeration = "super::super::super::storage::models::v1::ResourceType",
        tag = "1"
    )]
    pub resource: i32,
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub include_subresource: bool,
    #[prost(message, optional, tag = "7")]
    pub hierarchy: ::core::option::Option<EventStreamingGroupHierarchy>,
    #[prost(
        oneof = "create_event_streaming_group_request::StreamType",
        tags = "4, 5, 6"
    )]
    pub stream_type: ::core::option::Option<
        create_event_streaming_group_request::StreamType,
    >,
}
/// Nested message and enum types in `CreateEventStreamingGroupRequest`.
pub mod create_event_streaming_group_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamType {
        #[prost(message, tag = "4")]
        StreamAll(super::StreamAll),
        #[prost(message, tag = "5")]
        StreamFromDate(super::StreamFromDate),
        #[prost(message, tag = "6")]
        StreamFromSequence(super::StreamFromSequence),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventStreamingGroupHierarchy {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub object_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventStreamingGroupResponse {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchRequest {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub batch_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<EventNotificationMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchStreamRequest {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub batch_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchStreamResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<EventNotificationMessage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeMessageBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub replies: ::prost::alloc::vec::Vec<Reply>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeMessageBatchResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventStreamingGroupRequest {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventStreamingGroupResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromSequence {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromDate {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAll {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNotificationMessage {
    #[prost(
        enumeration = "super::super::super::storage::models::v1::ResourceType",
        tag = "1"
    )]
    pub resource: i32,
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(enumeration = "EventType", tag = "3")]
    pub updated_type: i32,
    #[prost(message, optional, tag = "4")]
    pub reply: ::core::option::Option<Reply>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reply {
    #[prost(string, tag = "1")]
    pub reply: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub salt: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub hmac: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Unspecified = 0,
    Created = 1,
    Available = 2,
    Updated = 3,
    MetadataUpdated = 4,
    Deleted = 5,
    All = 6,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
            EventType::Created => "EVENT_TYPE_CREATED",
            EventType::Available => "EVENT_TYPE_AVAILABLE",
            EventType::Updated => "EVENT_TYPE_UPDATED",
            EventType::MetadataUpdated => "EVENT_TYPE_METADATA_UPDATED",
            EventType::Deleted => "EVENT_TYPE_DELETED",
            EventType::All => "EVENT_TYPE_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_TYPE_CREATED" => Some(Self::Created),
            "EVENT_TYPE_AVAILABLE" => Some(Self::Available),
            "EVENT_TYPE_UPDATED" => Some(Self::Updated),
            "EVENT_TYPE_METADATA_UPDATED" => Some(Self::MetadataUpdated),
            "EVENT_TYPE_DELETED" => Some(Self::Deleted),
            "EVENT_TYPE_ALL" => Some(Self::All),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod event_notification_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// EventNotificationService
    ///
    /// A service to receive events in the AOS storage
    #[derive(Debug, Clone)]
    pub struct EventNotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventNotificationServiceClient<tonic::transport::Channel> {
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
    impl<T> EventNotificationServiceClient<T>
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
        ) -> EventNotificationServiceClient<InterceptedService<T, F>>
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
            EventNotificationServiceClient::new(
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
        /// CreateEventStreamingGroup
        ///
        /// Creates a new EventStreamingGroup
        pub async fn create_event_streaming_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventStreamingGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEventStreamingGroupResponse>,
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
                "/aruna.api.notification.services.v1.EventNotificationService/CreateEventStreamingGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v1.EventNotificationService",
                        "CreateEventStreamingGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message
        pub async fn get_event_message_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventMessageBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventMessageBatchResponse>,
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
                "/aruna.api.notification.services.v1.EventNotificationService/GetEventMessageBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v1.EventNotificationService",
                        "GetEventMessageBatch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message
        pub async fn get_event_message_batch_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventMessageBatchStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::GetEventMessageBatchStreamResponse>,
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
                "/aruna.api.notification.services.v1.EventNotificationService/GetEventMessageBatchStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v1.EventNotificationService",
                        "GetEventMessageBatchStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// AcknowledgeMessageBatch
        ///
        /// List of messages to acknowledge
        /// Each reply is protected by a salt and and hmac that verifies the message
        pub async fn acknowledge_message_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeMessageBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcknowledgeMessageBatchResponse>,
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
                "/aruna.api.notification.services.v1.EventNotificationService/AcknowledgeMessageBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v1.EventNotificationService",
                        "AcknowledgeMessageBatch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteEventStreamingGroup
        ///
        /// Deletes a existing EventStreamingGroup by ID
        pub async fn delete_event_streaming_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventStreamingGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEventStreamingGroupResponse>,
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
                "/aruna.api.notification.services.v1.EventNotificationService/DeleteEventStreamingGroup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v1.EventNotificationService",
                        "DeleteEventStreamingGroup",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod event_notification_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EventNotificationServiceServer.
    #[async_trait]
    pub trait EventNotificationService: Send + Sync + 'static {
        /// CreateEventStreamingGroup
        ///
        /// Creates a new EventStreamingGroup
        async fn create_event_streaming_group(
            &self,
            request: tonic::Request<super::CreateEventStreamingGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEventStreamingGroupResponse>,
            tonic::Status,
        >;
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message
        async fn get_event_message_batch(
            &self,
            request: tonic::Request<super::GetEventMessageBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventMessageBatchResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetEventMessageBatchStream method.
        type GetEventMessageBatchStreamStream: futures_core::Stream<
                Item = std::result::Result<
                    super::GetEventMessageBatchStreamResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message
        async fn get_event_message_batch_stream(
            &self,
            request: tonic::Request<super::GetEventMessageBatchStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetEventMessageBatchStreamStream>,
            tonic::Status,
        >;
        /// AcknowledgeMessageBatch
        ///
        /// List of messages to acknowledge
        /// Each reply is protected by a salt and and hmac that verifies the message
        async fn acknowledge_message_batch(
            &self,
            request: tonic::Request<super::AcknowledgeMessageBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcknowledgeMessageBatchResponse>,
            tonic::Status,
        >;
        /// DeleteEventStreamingGroup
        ///
        /// Deletes a existing EventStreamingGroup by ID
        async fn delete_event_streaming_group(
            &self,
            request: tonic::Request<super::DeleteEventStreamingGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEventStreamingGroupResponse>,
            tonic::Status,
        >;
    }
    /// EventNotificationService
    ///
    /// A service to receive events in the AOS storage
    #[derive(Debug)]
    pub struct EventNotificationServiceServer<T: EventNotificationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EventNotificationService> EventNotificationServiceServer<T> {
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
    for EventNotificationServiceServer<T>
    where
        T: EventNotificationService,
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
                "/aruna.api.notification.services.v1.EventNotificationService/CreateEventStreamingGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEventStreamingGroupSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<
                        super::CreateEventStreamingGroupRequest,
                    > for CreateEventStreamingGroupSvc<T> {
                        type Response = super::CreateEventStreamingGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateEventStreamingGroupRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_event_streaming_group(request).await
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
                        let method = CreateEventStreamingGroupSvc(inner);
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
                "/aruna.api.notification.services.v1.EventNotificationService/GetEventMessageBatch" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventMessageBatchSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<super::GetEventMessageBatchRequest>
                    for GetEventMessageBatchSvc<T> {
                        type Response = super::GetEventMessageBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEventMessageBatchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_event_message_batch(request).await
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
                        let method = GetEventMessageBatchSvc(inner);
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
                "/aruna.api.notification.services.v1.EventNotificationService/GetEventMessageBatchStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventMessageBatchStreamSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::ServerStreamingService<
                        super::GetEventMessageBatchStreamRequest,
                    > for GetEventMessageBatchStreamSvc<T> {
                        type Response = super::GetEventMessageBatchStreamResponse;
                        type ResponseStream = T::GetEventMessageBatchStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetEventMessageBatchStreamRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_event_message_batch_stream(request).await
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
                        let method = GetEventMessageBatchStreamSvc(inner);
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
                "/aruna.api.notification.services.v1.EventNotificationService/AcknowledgeMessageBatch" => {
                    #[allow(non_camel_case_types)]
                    struct AcknowledgeMessageBatchSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<super::AcknowledgeMessageBatchRequest>
                    for AcknowledgeMessageBatchSvc<T> {
                        type Response = super::AcknowledgeMessageBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AcknowledgeMessageBatchRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).acknowledge_message_batch(request).await
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
                        let method = AcknowledgeMessageBatchSvc(inner);
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
                "/aruna.api.notification.services.v1.EventNotificationService/DeleteEventStreamingGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEventStreamingGroupSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<
                        super::DeleteEventStreamingGroupRequest,
                    > for DeleteEventStreamingGroupSvc<T> {
                        type Response = super::DeleteEventStreamingGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteEventStreamingGroupRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_event_streaming_group(request).await
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
                        let method = DeleteEventStreamingGroupSvc(inner);
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
    impl<T: EventNotificationService> Clone for EventNotificationServiceServer<T> {
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
    impl<T: EventNotificationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventNotificationService> tonic::server::NamedService
    for EventNotificationServiceServer<T> {
        const NAME: &'static str = "aruna.api.notification.services.v1.EventNotificationService";
    }
}
