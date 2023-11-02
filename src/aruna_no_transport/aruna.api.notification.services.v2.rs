#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub persistent_resource_id: bool,
    #[prost(string, tag = "3")]
    pub checksum: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::storage::models::v2::ResourceVariant",
        tag = "4"
    )]
    pub resource_variant: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceTarget {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::storage::models::v2::ResourceVariant",
        tag = "2"
    )]
    pub resource_variant: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamConsumerRequest {
    #[prost(bool, tag = "5")]
    pub include_subresources: bool,
    #[prost(oneof = "create_stream_consumer_request::Target", tags = "1, 2, 3, 4")]
    pub target: ::core::option::Option<create_stream_consumer_request::Target>,
    #[prost(oneof = "create_stream_consumer_request::StreamType", tags = "6, 7, 8")]
    pub stream_type: ::core::option::Option<create_stream_consumer_request::StreamType>,
}
/// Nested message and enum types in `CreateStreamConsumerRequest`.
pub mod create_stream_consumer_request {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(message, tag = "1")]
        Resource(super::ResourceTarget),
        #[prost(string, tag = "2")]
        User(::prost::alloc::string::String),
        #[prost(bool, tag = "3")]
        Announcements(bool),
        #[prost(bool, tag = "4")]
        All(bool),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamType {
        #[prost(message, tag = "6")]
        StreamAll(super::StreamAll),
        #[prost(message, tag = "7")]
        StreamFromDate(super::StreamFromDate),
        #[prost(message, tag = "8")]
        StreamFromSequence(super::StreamFromSequence),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStreamConsumerResponse {
    #[prost(string, tag = "1")]
    pub stream_consumer: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchRequest {
    #[prost(string, tag = "1")]
    pub stream_consumer: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub batch_size: u32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::prost::alloc::vec::Vec<EventMessage>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageStreamRequest {
    #[prost(string, tag = "1")]
    pub stream_consumer: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventMessageStreamResponse {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<EventMessage>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeMessageBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub replies: ::prost::alloc::vec::Vec<Reply>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeMessageBatchResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamConsumerRequest {
    #[prost(string, tag = "1")]
    pub stream_consumer: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStreamConsumerResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromSequence {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromDate {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAll {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMessage {
    #[prost(oneof = "event_message::MessageVariant", tags = "1, 2, 3")]
    pub message_variant: ::core::option::Option<event_message::MessageVariant>,
}
/// Nested message and enum types in `EventMessage`.
pub mod event_message {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MessageVariant {
        #[prost(message, tag = "1")]
        ResourceEvent(super::ResourceEvent),
        #[prost(message, tag = "2")]
        UserEvent(super::UserEvent),
        #[prost(message, tag = "3")]
        AnnouncementEvent(super::AnnouncementEvent),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceEvent {
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<Resource>,
    #[prost(enumeration = "EventVariant", tag = "2")]
    pub event_variant: i32,
    #[prost(message, optional, tag = "3")]
    pub reply: ::core::option::Option<Reply>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "EventVariant", tag = "2")]
    pub event_variant: i32,
    #[prost(string, tag = "3")]
    pub checksum: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub reply: ::core::option::Option<Reply>,
}
#[derive(serde::Deserialize, serde::Serialize)]
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
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduledDowntime {
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub component: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewVersion {
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_version: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPubkey {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncementEvent {
    #[prost(message, optional, tag = "8")]
    pub reply: ::core::option::Option<Reply>,
    #[prost(oneof = "announcement_event::EventVariant", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub event_variant: ::core::option::Option<announcement_event::EventVariant>,
}
/// Nested message and enum types in `AnnouncementEvent`.
pub mod announcement_event {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EventVariant {
        #[prost(string, tag = "1")]
        NewDataProxyId(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        RemoveDataProxyId(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        UpdateDataProxyId(::prost::alloc::string::String),
        #[prost(int32, tag = "4")]
        NewPubkey(i32),
        #[prost(int32, tag = "5")]
        RemovePubkey(i32),
        #[prost(message, tag = "6")]
        Downtime(super::ScheduledDowntime),
        #[prost(message, tag = "7")]
        Version(super::NewVersion),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventVariant {
    Unspecified = 0,
    Created = 1,
    Available = 2,
    Updated = 3,
    Deleted = 4,
    Snapshotted = 5,
}
impl EventVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventVariant::Unspecified => "EVENT_VARIANT_UNSPECIFIED",
            EventVariant::Created => "EVENT_VARIANT_CREATED",
            EventVariant::Available => "EVENT_VARIANT_AVAILABLE",
            EventVariant::Updated => "EVENT_VARIANT_UPDATED",
            EventVariant::Deleted => "EVENT_VARIANT_DELETED",
            EventVariant::Snapshotted => "EVENT_VARIANT_SNAPSHOTTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_VARIANT_CREATED" => Some(Self::Created),
            "EVENT_VARIANT_AVAILABLE" => Some(Self::Available),
            "EVENT_VARIANT_UPDATED" => Some(Self::Updated),
            "EVENT_VARIANT_DELETED" => Some(Self::Deleted),
            "EVENT_VARIANT_SNAPSHOTTED" => Some(Self::Snapshotted),
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
        /// CreateStreamConsumer
        ///
        /// Creates a new event stream consumer.
        pub async fn create_stream_consumer(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStreamConsumerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamConsumerResponse>,
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
                "/aruna.api.notification.services.v2.EventNotificationService/CreateStreamConsumer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v2.EventNotificationService",
                        "CreateStreamConsumer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message thatis protected by a salt and an hmac for verification.
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message.
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
                "/aruna.api.notification.services.v2.EventNotificationService/GetEventMessageBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v2.EventNotificationService",
                        "GetEventMessageBatch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEventMessageBatch
        ///
        /// Opens a stream which pushes each received notification individual and just-in-time.
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification.
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message.
        pub async fn get_event_message_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventMessageStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::GetEventMessageStreamResponse>,
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
                "/aruna.api.notification.services.v2.EventNotificationService/GetEventMessageStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v2.EventNotificationService",
                        "GetEventMessageStream",
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
                "/aruna.api.notification.services.v2.EventNotificationService/AcknowledgeMessageBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v2.EventNotificationService",
                        "AcknowledgeMessageBatch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteEventStreamingGroup
        ///
        /// Deletes an existing event stream consumer by ID.
        pub async fn delete_stream_consumer(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStreamConsumerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteStreamConsumerResponse>,
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
                "/aruna.api.notification.services.v2.EventNotificationService/DeleteStreamConsumer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.notification.services.v2.EventNotificationService",
                        "DeleteStreamConsumer",
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
        /// CreateStreamConsumer
        ///
        /// Creates a new event stream consumer.
        async fn create_stream_consumer(
            &self,
            request: tonic::Request<super::CreateStreamConsumerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStreamConsumerResponse>,
            tonic::Status,
        >;
        /// GetEventMessageBatch
        ///
        /// Reads a set of messages from a given stream group
        /// Each message contains a separate acknowledgement message thatis protected by a salt and an hmac for verification.
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message.
        async fn get_event_message_batch(
            &self,
            request: tonic::Request<super::GetEventMessageBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEventMessageBatchResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetEventMessageStream method.
        type GetEventMessageStreamStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<
                    super::GetEventMessageStreamResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// GetEventMessageBatch
        ///
        /// Opens a stream which pushes each received notification individual and just-in-time.
        /// Each message contains a separate acknowledgement message that is protected by a salt and an hmac for verification.
        /// The message can be send directly through the AcknowledgeMessageBatch call to acknowledge the message.
        async fn get_event_message_stream(
            &self,
            request: tonic::Request<super::GetEventMessageStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetEventMessageStreamStream>,
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
        /// Deletes an existing event stream consumer by ID.
        async fn delete_stream_consumer(
            &self,
            request: tonic::Request<super::DeleteStreamConsumerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteStreamConsumerResponse>,
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
                "/aruna.api.notification.services.v2.EventNotificationService/CreateStreamConsumer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStreamConsumerSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<super::CreateStreamConsumerRequest>
                    for CreateStreamConsumerSvc<T> {
                        type Response = super::CreateStreamConsumerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStreamConsumerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EventNotificationService>::create_stream_consumer(
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
                        let method = CreateStreamConsumerSvc(inner);
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
                "/aruna.api.notification.services.v2.EventNotificationService/GetEventMessageBatch" => {
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
                                <T as EventNotificationService>::get_event_message_batch(
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
                "/aruna.api.notification.services.v2.EventNotificationService/GetEventMessageStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventMessageStreamSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::ServerStreamingService<
                        super::GetEventMessageStreamRequest,
                    > for GetEventMessageStreamSvc<T> {
                        type Response = super::GetEventMessageStreamResponse;
                        type ResponseStream = T::GetEventMessageStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEventMessageStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EventNotificationService>::get_event_message_stream(
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
                        let method = GetEventMessageStreamSvc(inner);
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
                "/aruna.api.notification.services.v2.EventNotificationService/AcknowledgeMessageBatch" => {
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
                                <T as EventNotificationService>::acknowledge_message_batch(
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
                "/aruna.api.notification.services.v2.EventNotificationService/DeleteStreamConsumer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteStreamConsumerSvc<T: EventNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: EventNotificationService,
                    > tonic::server::UnaryService<super::DeleteStreamConsumerRequest>
                    for DeleteStreamConsumerSvc<T> {
                        type Response = super::DeleteStreamConsumerResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteStreamConsumerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as EventNotificationService>::delete_stream_consumer(
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
                        let method = DeleteStreamConsumerSvc(inner);
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
        const NAME: &'static str = "aruna.api.notification.services.v2.EventNotificationService";
    }
}
