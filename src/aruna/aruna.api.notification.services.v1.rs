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
    #[prost(string, tag = "7")]
    pub stream_group_id: ::prost::alloc::string::String,
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
pub struct CreateEventStreamingGroupResponse {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStreamGroupMessagesRequest {
    #[prost(bool, tag = "3")]
    pub close: bool,
    #[prost(oneof = "read_stream_group_messages_request::StreamAction", tags = "1, 2")]
    pub stream_action: ::core::option::Option<
        read_stream_group_messages_request::StreamAction,
    >,
}
/// Nested message and enum types in `ReadStreamGroupMessagesRequest`.
pub mod read_stream_group_messages_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamAction {
        #[prost(message, tag = "1")]
        Init(super::NotificationStreamInit),
        #[prost(message, tag = "2")]
        Ack(super::NotficationStreamAck),
    }
}
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
pub struct NotificationStreamInit {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotficationStreamAck {
    #[prost(string, repeated, tag = "1")]
    pub ack_chunk_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadStreamGroupMessagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub notification: ::prost::alloc::vec::Vec<NotificationStreamResponse>,
    #[prost(string, tag = "2")]
    pub ack_chunk_id: ::prost::alloc::string::String,
}
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
pub struct NotificationStreamResponse {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<EventNotificationMessage>,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
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
pub mod update_notification_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// UpdateNotificationService
    ///
    /// A service to update streaminggroups in nats.io
    #[derive(Debug, Clone)]
    pub struct UpdateNotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UpdateNotificationServiceClient<tonic::transport::Channel> {
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
    impl<T> UpdateNotificationServiceClient<T>
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
        ) -> UpdateNotificationServiceClient<InterceptedService<T, F>>
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
            UpdateNotificationServiceClient::new(
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
        /// CreateEventStreamingGroup
        ///
        /// Creates a new EventStreamingGroup
        pub async fn create_event_streaming_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventStreamingGroupRequest>,
        ) -> Result<
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/CreateEventStreamingGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteEventStreamingGroup
        ///
        /// Deletes a existing EventStreamingGroup by ID
        pub async fn delete_event_streaming_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventStreamingGroupRequest>,
        ) -> Result<
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/DeleteEventStreamingGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ReadStreamGroupMessages
        ///
        /// Reads a stream of messages for a specific StreamGroup
        pub async fn read_stream_group_messages(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::ReadStreamGroupMessagesRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::ReadStreamGroupMessagesResponse>,
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/ReadStreamGroupMessages",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod update_notification_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UpdateNotificationServiceServer.
    #[async_trait]
    pub trait UpdateNotificationService: Send + Sync + 'static {
        /// CreateEventStreamingGroup
        ///
        /// Creates a new EventStreamingGroup
        async fn create_event_streaming_group(
            &self,
            request: tonic::Request<super::CreateEventStreamingGroupRequest>,
        ) -> Result<
            tonic::Response<super::CreateEventStreamingGroupResponse>,
            tonic::Status,
        >;
        /// DeleteEventStreamingGroup
        ///
        /// Deletes a existing EventStreamingGroup by ID
        async fn delete_event_streaming_group(
            &self,
            request: tonic::Request<super::DeleteEventStreamingGroupRequest>,
        ) -> Result<
            tonic::Response<super::DeleteEventStreamingGroupResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the ReadStreamGroupMessages method.
        type ReadStreamGroupMessagesStream: futures_core::Stream<
                Item = Result<super::ReadStreamGroupMessagesResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// ReadStreamGroupMessages
        ///
        /// Reads a stream of messages for a specific StreamGroup
        async fn read_stream_group_messages(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::ReadStreamGroupMessagesRequest>,
            >,
        ) -> Result<tonic::Response<Self::ReadStreamGroupMessagesStream>, tonic::Status>;
    }
    /// UpdateNotificationService
    ///
    /// A service to update streaminggroups in nats.io
    #[derive(Debug)]
    pub struct UpdateNotificationServiceServer<T: UpdateNotificationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UpdateNotificationService> UpdateNotificationServiceServer<T> {
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
    for UpdateNotificationServiceServer<T>
    where
        T: UpdateNotificationService,
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/CreateEventStreamingGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEventStreamingGroupSvc<T: UpdateNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UpdateNotificationService,
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_event_streaming_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateEventStreamingGroupSvc(inner);
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/DeleteEventStreamingGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEventStreamingGroupSvc<T: UpdateNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UpdateNotificationService,
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_event_streaming_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteEventStreamingGroupSvc(inner);
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
                "/aruna.api.notification.services.v1.UpdateNotificationService/ReadStreamGroupMessages" => {
                    #[allow(non_camel_case_types)]
                    struct ReadStreamGroupMessagesSvc<T: UpdateNotificationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: UpdateNotificationService,
                    > tonic::server::StreamingService<
                        super::ReadStreamGroupMessagesRequest,
                    > for ReadStreamGroupMessagesSvc<T> {
                        type Response = super::ReadStreamGroupMessagesResponse;
                        type ResponseStream = T::ReadStreamGroupMessagesStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::ReadStreamGroupMessagesRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).read_stream_group_messages(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadStreamGroupMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
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
    impl<T: UpdateNotificationService> Clone for UpdateNotificationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UpdateNotificationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UpdateNotificationService> tonic::server::NamedService
    for UpdateNotificationServiceServer<T> {
        const NAME: &'static str = "aruna.api.notification.services.v1.UpdateNotificationService";
    }
}
