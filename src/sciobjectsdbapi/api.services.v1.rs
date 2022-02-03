#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventStreamingGroupRequest {
    #[prost(
        enumeration = "create_event_streaming_group_request::EventResources",
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
    pub stream_type: ::core::option::Option<create_event_streaming_group_request::StreamType>,
}
/// Nested message and enum types in `CreateEventStreamingGroupRequest`.
pub mod create_event_streaming_group_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventResources {
        Unspecified = 0,
        ProjectResource = 1,
        DatasetResource = 2,
        DatasetVersionResource = 3,
        ObjectGroupResource = 4,
        AllResource = 5,
    }
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventStreamingGroupResponse {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationStreamGroupRequest {
    #[prost(bool, tag = "3")]
    pub close: bool,
    #[prost(
        oneof = "notification_stream_group_request::StreamAction",
        tags = "1, 2"
    )]
    pub stream_action: ::core::option::Option<notification_stream_group_request::StreamAction>,
}
/// Nested message and enum types in `NotificationStreamGroupRequest`.
pub mod notification_stream_group_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StreamAction {
        #[prost(message, tag = "1")]
        Init(super::NotificationStreamInit),
        #[prost(message, tag = "2")]
        Ack(super::NotficationStreamAck),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationStreamInit {
    #[prost(string, tag = "1")]
    pub stream_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotficationStreamAck {
    #[prost(string, repeated, tag = "1")]
    pub ack_chunk_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationStreamGroupResponse {
    #[prost(message, repeated, tag = "1")]
    pub notification: ::prost::alloc::vec::Vec<NotificationStreamResponse>,
    #[prost(string, tag = "2")]
    pub ack_chunk_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromSequence {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamFromDate {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAll {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationStreamResponse {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<EventNotificationMessage>,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNotificationMessage {
    #[prost(enumeration = "super::super::models::v1::Resource", tag = "1")]
    pub resource: i32,
    #[prost(string, tag = "2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(enumeration = "event_notification_message::UpdateType", tag = "3")]
    pub updated_type: i32,
}
/// Nested message and enum types in `EventNotificationMessage`.
pub mod event_notification_message {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UpdateType {
        Unspecified = 0,
        Created = 1,
        Available = 2,
        Updated = 3,
        MetadataUpdated = 4,
        Deleted = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod update_notification_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UpdateNotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UpdateNotificationServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UpdateNotificationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UpdateNotificationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn create_event_streaming_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventStreamingGroupRequest>,
        ) -> Result<tonic::Response<super::CreateEventStreamingGroupResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.UpdateNotificationService/CreateEventStreamingGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn notification_stream_group(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::NotificationStreamGroupRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::NotificationStreamGroupResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.UpdateNotificationService/NotificationStreamGroup",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod update_notification_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UpdateNotificationServiceServer."]
    #[async_trait]
    pub trait UpdateNotificationService: Send + Sync + 'static {
        async fn create_event_streaming_group(
            &self,
            request: tonic::Request<super::CreateEventStreamingGroupRequest>,
        ) -> Result<tonic::Response<super::CreateEventStreamingGroupResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the NotificationStreamGroup method."]
        type NotificationStreamGroupStream: futures_core::Stream<
                Item = Result<super::NotificationStreamGroupResponse, tonic::Status>,
            > + Send
            + 'static;
        async fn notification_stream_group(
            &self,
            request: tonic::Request<tonic::Streaming<super::NotificationStreamGroupRequest>>,
        ) -> Result<tonic::Response<Self::NotificationStreamGroupStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UpdateNotificationServiceServer<T: UpdateNotificationService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UpdateNotificationService> UpdateNotificationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UpdateNotificationServiceServer<T>
    where
        T: UpdateNotificationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.services.v1.UpdateNotificationService/CreateEventStreamingGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEventStreamingGroupSvc<T: UpdateNotificationService>(pub Arc<T>);
                    impl<T: UpdateNotificationService>
                        tonic::server::UnaryService<super::CreateEventStreamingGroupRequest>
                        for CreateEventStreamingGroupSvc<T>
                    {
                        type Response = super::CreateEventStreamingGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEventStreamingGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_event_streaming_group(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.UpdateNotificationService/NotificationStreamGroup" => {
                    #[allow(non_camel_case_types)]
                    struct NotificationStreamGroupSvc<T: UpdateNotificationService>(pub Arc<T>);
                    impl<T: UpdateNotificationService>
                        tonic::server::StreamingService<super::NotificationStreamGroupRequest>
                        for NotificationStreamGroupSvc<T>
                    {
                        type Response = super::NotificationStreamGroupResponse;
                        type ResponseStream = T::NotificationStreamGroupStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::NotificationStreamGroupRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).notification_stream_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NotificationStreamGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: UpdateNotificationService> tonic::transport::NamedService
        for UpdateNotificationServiceServer<T>
    {
        const NAME: &'static str = "api.services.v1.UpdateNotificationService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    #[prost(message, repeated, tag = "6")]
    pub objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(bool, tag = "8")]
    pub include_object_link: bool,
    #[prost(message, optional, tag = "10")]
    pub generated: ::core::option::Option<::prost_types::Timestamp>,
    /// A user defined uuid that is used to identify requests in chunked workloads
    #[prost(string, tag = "11")]
    pub uuid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<CreateObjectGroupRequest>,
    #[prost(bool, tag = "2")]
    pub include_object_link: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<CreateObjectGroupResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupResponse {
    #[prost(string, tag = "1")]
    pub object_group_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub object_links: ::prost::alloc::vec::Vec<create_object_group_response::ObjectLinks>,
    #[prost(string, tag = "3")]
    pub object_group_name: ::prost::alloc::string::String,
    /// A user defined uuid that is used to identify requests in chunked/streamed workloads
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CreateObjectGroupResponse`.
pub mod create_object_group_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ObjectLinks {
        #[prost(string, tag = "1")]
        pub filename: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub link: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub object_id: ::prost::alloc::string::String,
        #[prost(int64, tag = "4")]
        pub index: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filetype: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "4")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag = "5")]
    pub content_len: i64,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag = "6")]
    pub origin: ::core::option::Option<super::super::models::v1::Origin>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupResponse {
    #[prost(message, optional, tag = "1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectUploadRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectUploadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupResponse {}
#[doc = r" Generated client implementations."]
pub mod dataset_objects_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DatasetObjectsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatasetObjectsServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatasetObjectsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DatasetObjectsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a new object group"]
        pub async fn create_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/CreateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch request of CreateObjectGroup"]
        #[doc = " The call will preserve the ordering of the request in the response"]
        pub async fn create_object_group_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupBatchRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupBatchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/CreateObjectGroupBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "Returns the object group with the given ID"]
        pub async fn get_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/GetObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finishes the upload process for an object"]
        #[doc = " This will change the status of the objects to \"available\""]
        #[doc = " Experimental, might change this to FinishObjectGroupUpload"]
        pub async fn finish_object_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishObjectUploadRequest>,
        ) -> Result<tonic::Response<super::FinishObjectUploadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/FinishObjectUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the given object group"]
        #[doc = " This will also delete all associated objects both as metadata objects and the actual objects in the object storage"]
        pub async fn delete_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/DeleteObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dataset_objects_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatasetObjectsServiceServer."]
    #[async_trait]
    pub trait DatasetObjectsService: Send + Sync + 'static {
        #[doc = " Creates a new object group"]
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status>;
        #[doc = " Batch request of CreateObjectGroup"]
        #[doc = " The call will preserve the ordering of the request in the response"]
        async fn create_object_group_batch(
            &self,
            request: tonic::Request<super::CreateObjectGroupBatchRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupBatchResponse>, tonic::Status>;
        #[doc = "Returns the object group with the given ID"]
        async fn get_object_group(
            &self,
            request: tonic::Request<super::GetObjectGroupRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupResponse>, tonic::Status>;
        #[doc = " Finishes the upload process for an object"]
        #[doc = " This will change the status of the objects to \"available\""]
        #[doc = " Experimental, might change this to FinishObjectGroupUpload"]
        async fn finish_object_upload(
            &self,
            request: tonic::Request<super::FinishObjectUploadRequest>,
        ) -> Result<tonic::Response<super::FinishObjectUploadResponse>, tonic::Status>;
        #[doc = " Deletes the given object group"]
        #[doc = " This will also delete all associated objects both as metadata objects and the actual objects in the object storage"]
        async fn delete_object_group(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DatasetObjectsServiceServer<T: DatasetObjectsService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DatasetObjectsService> DatasetObjectsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DatasetObjectsServiceServer<T>
    where
        T: DatasetObjectsService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.services.v1.DatasetObjectsService/CreateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::CreateObjectGroupRequest>
                        for CreateObjectGroupSvc<T>
                    {
                        type Response = super::CreateObjectGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_object_group(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetObjectsService/CreateObjectGroupBatch" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupBatchSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::CreateObjectGroupBatchRequest>
                        for CreateObjectGroupBatchSvc<T>
                    {
                        type Response = super::CreateObjectGroupBatchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_object_group_batch(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetObjectsService/GetObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::GetObjectGroupRequest>
                        for GetObjectGroupSvc<T>
                    {
                        type Response = super::GetObjectGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object_group(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetObjectsService/FinishObjectUpload" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectUploadSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::FinishObjectUploadRequest>
                        for FinishObjectUploadSvc<T>
                    {
                        type Response = super::FinishObjectUploadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinishObjectUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finish_object_upload(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetObjectsService/DeleteObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::DeleteObjectGroupRequest>
                        for DeleteObjectGroupSvc<T>
                    {
                        type Response = super::DeleteObjectGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_object_group(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: DatasetObjectsService> tonic::transport::NamedService for DatasetObjectsServiceServer<T> {
        const NAME: &'static str = "api.services.v1.DatasetObjectsService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLink {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLinkRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLinkResponse {
    #[prost(string, tag = "1")]
    pub upload_link: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkRequest {
    ///Object id the download is requested for
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///Optional; Byte range of the data object
    #[prost(message, optional, tag = "2")]
    pub range: ::core::option::Option<create_download_link_request::Range>,
}
/// Nested message and enum types in `CreateDownloadLinkRequest`.
pub mod create_download_link_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        #[prost(int64, tag = "1")]
        pub start_byte: i64,
        #[prost(int64, tag = "2")]
        pub end_byte: i64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkResponse {
    #[prost(string, tag = "1")]
    pub download_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<CreateDownloadLinkRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<CreateDownloadLinkResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMultipartUploadResponse {
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultipartUploadLinkResponse {
    #[prost(string, tag = "1")]
    pub upload_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMultipartUploadLinkRequest {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub upload_part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadRequest {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub parts: ::prost::alloc::vec::Vec<CompletedParts>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteMultipartUploadResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedParts {
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkStreamRequest {
    #[prost(oneof = "create_download_link_stream_request::Query", tags = "1, 3, 4")]
    pub query: ::core::option::Option<create_download_link_stream_request::Query>,
}
/// Nested message and enum types in `CreateDownloadLinkStreamRequest`.
pub mod create_download_link_stream_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateRangeQuery {
        #[prost(string, tag = "3")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "1")]
        pub start: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "2")]
        pub end: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetQuery {
        #[prost(string, tag = "1")]
        pub dataset_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetVersionQuery {
        #[prost(string, tag = "1")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub dataset_version_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag = "1")]
        Dataset(DatasetQuery),
        #[prost(message, tag = "3")]
        DatasetVersion(DatasetVersionQuery),
        #[prost(message, tag = "4")]
        DateRange(DateRangeQuery),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkStreamResponse {
    #[prost(message, optional, tag = "1")]
    pub links: ::core::option::Option<LinksResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinksResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
    #[prost(message, repeated, tag = "2")]
    pub object_group_links: ::prost::alloc::vec::Vec<InnerLinksResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerLinksResponse {
    #[prost(string, repeated, tag = "1")]
    pub object_links: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod object_load_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Handles object up and downloads"]
    #[derive(Debug, Clone)]
    pub struct ObjectLoadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectLoadServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ObjectLoadServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ObjectLoadServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates an upload link for an object to upload the actual data object"]
        #[doc = " Returns a presigned https PUT request"]
        #[doc = " Can only be used to upload objects < 4GB"]
        pub async fn create_upload_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUploadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/CreateUploadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a download link for an object"]
        #[doc = " Returns a presigned https GET request"]
        pub async fn create_download_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/CreateDownloadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates links for multiple objects at once"]
        #[doc = " The order of the requested objects is preserved in the response"]
        pub async fn create_download_link_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkBatchRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkBatchResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/CreateDownloadLinkBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a stream of objects and presigned links based on the provided query"]
        #[doc = " This can be used retrieve a large number of ObjectGroups as a stream that would otherwise cause issues with the connection"]
        pub async fn create_download_link_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinkStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::CreateDownloadLinkStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/CreateDownloadLinkStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Initiates a multipart upload for an object"]
        #[doc = " This is intended to be used for larger objects"]
        #[doc = " For further information please read the Amazon S3 documentation on multipart uploads"]
        #[doc = " Has to be used together with GetMultipartUploadLink and CompleteMultipartUpload"]
        pub async fn start_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::StartMultipartUploadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/StartMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_multipart_upload_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMultipartUploadLinkRequest>,
        ) -> Result<tonic::Response<super::GetMultipartUploadLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/GetMultipartUploadLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn complete_multipart_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::CompleteMultipartUploadResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ObjectLoadService/CompleteMultipartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_load_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectLoadServiceServer."]
    #[async_trait]
    pub trait ObjectLoadService: Send + Sync + 'static {
        #[doc = " Creates an upload link for an object to upload the actual data object"]
        #[doc = " Returns a presigned https PUT request"]
        #[doc = " Can only be used to upload objects < 4GB"]
        async fn create_upload_link(
            &self,
            request: tonic::Request<super::CreateUploadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status>;
        #[doc = " Creates a download link for an object"]
        #[doc = " Returns a presigned https GET request"]
        async fn create_download_link(
            &self,
            request: tonic::Request<super::CreateDownloadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkResponse>, tonic::Status>;
        #[doc = " Creates links for multiple objects at once"]
        #[doc = " The order of the requested objects is preserved in the response"]
        async fn create_download_link_batch(
            &self,
            request: tonic::Request<super::CreateDownloadLinkBatchRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkBatchResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the CreateDownloadLinkStream method."]
        type CreateDownloadLinkStreamStream: futures_core::Stream<
                Item = Result<super::CreateDownloadLinkStreamResponse, tonic::Status>,
            > + Send
            + 'static;
        #[doc = " Creates a stream of objects and presigned links based on the provided query"]
        #[doc = " This can be used retrieve a large number of ObjectGroups as a stream that would otherwise cause issues with the connection"]
        async fn create_download_link_stream(
            &self,
            request: tonic::Request<super::CreateDownloadLinkStreamRequest>,
        ) -> Result<tonic::Response<Self::CreateDownloadLinkStreamStream>, tonic::Status>;
        #[doc = " Initiates a multipart upload for an object"]
        #[doc = " This is intended to be used for larger objects"]
        #[doc = " For further information please read the Amazon S3 documentation on multipart uploads"]
        #[doc = " Has to be used together with GetMultipartUploadLink and CompleteMultipartUpload"]
        async fn start_multipart_upload(
            &self,
            request: tonic::Request<super::StartMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::StartMultipartUploadResponse>, tonic::Status>;
        async fn get_multipart_upload_link(
            &self,
            request: tonic::Request<super::GetMultipartUploadLinkRequest>,
        ) -> Result<tonic::Response<super::GetMultipartUploadLinkResponse>, tonic::Status>;
        async fn complete_multipart_upload(
            &self,
            request: tonic::Request<super::CompleteMultipartUploadRequest>,
        ) -> Result<tonic::Response<super::CompleteMultipartUploadResponse>, tonic::Status>;
    }
    #[doc = " Handles object up and downloads"]
    #[derive(Debug)]
    pub struct ObjectLoadServiceServer<T: ObjectLoadService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectLoadService> ObjectLoadServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectLoadServiceServer<T>
    where
        T: ObjectLoadService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.services.v1.ObjectLoadService/CreateUploadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUploadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::CreateUploadLinkRequest>
                        for CreateUploadLinkSvc<T>
                    {
                        type Response = super::CreateUploadLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUploadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_upload_link(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/CreateDownloadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::CreateDownloadLinkRequest>
                        for CreateDownloadLinkSvc<T>
                    {
                        type Response = super::CreateDownloadLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDownloadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_download_link(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/CreateDownloadLinkBatch" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkBatchSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::CreateDownloadLinkBatchRequest>
                        for CreateDownloadLinkBatchSvc<T>
                    {
                        type Response = super::CreateDownloadLinkBatchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDownloadLinkBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_download_link_batch(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/CreateDownloadLinkStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkStreamSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::ServerStreamingService<
                            super::CreateDownloadLinkStreamRequest,
                        > for CreateDownloadLinkStreamSvc<T>
                    {
                        type Response = super::CreateDownloadLinkStreamResponse;
                        type ResponseStream = T::CreateDownloadLinkStreamStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDownloadLinkStreamRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_download_link_stream(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/StartMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct StartMultipartUploadSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::StartMultipartUploadRequest>
                        for StartMultipartUploadSvc<T>
                    {
                        type Response = super::StartMultipartUploadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartMultipartUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_multipart_upload(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/GetMultipartUploadLink" => {
                    #[allow(non_camel_case_types)]
                    struct GetMultipartUploadLinkSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::GetMultipartUploadLinkRequest>
                        for GetMultipartUploadLinkSvc<T>
                    {
                        type Response = super::GetMultipartUploadLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMultipartUploadLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_multipart_upload_link(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ObjectLoadService/CompleteMultipartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteMultipartUploadSvc<T: ObjectLoadService>(pub Arc<T>);
                    impl<T: ObjectLoadService>
                        tonic::server::UnaryService<super::CompleteMultipartUploadRequest>
                        for CompleteMultipartUploadSvc<T>
                    {
                        type Response = super::CompleteMultipartUploadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteMultipartUploadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).complete_multipart_upload(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: ObjectLoadService> tonic::transport::NamedService for ObjectLoadServiceServer<T> {
        const NAME: &'static str = "api.services.v1.ObjectLoadService";
    }
}
/// Dataset related Models
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Name of the dataset
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    ///ProjectID of the project the dataset is associated with
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v1::Dataset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dataset_versions: ::prost::alloc::vec::Vec<super::super::models::v1::DatasetVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetObjectGroupsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetObjectGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsInDateRangeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsInDateRangeResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
}
/// GetObjectGroupsStreamLinkRequest a request for a get link to stream a set of object groups
/// The query defines what object groups should be part of the stream
/// The steam type defines how the individual objects are packed
/// ZIP and TARGZ will bundle objectgroups into subfolders
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsStreamLinkRequest {
    #[prost(
        enumeration = "get_object_groups_stream_link_request::StreamType",
        tag = "3"
    )]
    pub stream_type: i32,
    /// Expiry time of the link
    /// This is the maximum expiry time, implementations can set maximum durations that can be shorter than this
    #[prost(message, optional, tag = "8")]
    pub expiry: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(
        oneof = "get_object_groups_stream_link_request::Query",
        tags = "4, 5, 6, 7"
    )]
    pub query: ::core::option::Option<get_object_groups_stream_link_request::Query>,
}
/// Nested message and enum types in `GetObjectGroupsStreamLinkRequest`.
pub mod get_object_groups_stream_link_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateRangeQuery {
        #[prost(string, tag = "3")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "1")]
        pub start: ::core::option::Option<::prost_types::Timestamp>,
        #[prost(message, optional, tag = "2")]
        pub end: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupIDsQuery {
        #[prost(string, tag = "2")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "1")]
        pub object_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetQuery {
        #[prost(string, tag = "1")]
        pub dataset_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetVersionQuery {
        #[prost(string, tag = "1")]
        pub dataset_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub dataset_version_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StreamType {
        Base64newline = 0,
        Zip = 1,
        Targz = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        #[prost(message, tag = "4")]
        GroupIds(GroupIDsQuery),
        #[prost(message, tag = "5")]
        DateRange(DateRangeQuery),
        #[prost(message, tag = "6")]
        Dataset(DatasetQuery),
        #[prost(message, tag = "7")]
        DatasetVersion(DatasetVersionQuery),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsStreamLinkResponse {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetFieldRequest {
    #[prost(message, optional, tag = "1")]
    pub update_request: ::core::option::Option<super::super::models::v1::UpdateFieldsRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetFieldResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetResponse {}
// DatasetVersion related models

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    #[prost(string, repeated, tag = "7")]
    pub object_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset_version: ::core::option::Option<super::super::models::v1::DatasetVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionObjectGroupsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetVersionObjectGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_group: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionResponse {}
/// Request to create a project
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v1::Right", repeated, tag = "2")]
    pub scope: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenResponse {
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::models::v1::ApiToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDatasetsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectDatasetsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dataset: ::prost::alloc::vec::Vec<super::super::models::v1::Dataset>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsResponse {
    #[prost(message, repeated, tag = "1")]
    pub projects: ::prost::alloc::vec::Vec<super::super::models::v1::Project>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectResponse {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v1::Project>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenResponse {
    #[prost(message, repeated, tag = "1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v1::ApiToken>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectResponse {}
#[doc = r" Generated client implementations."]
pub mod project_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ProjectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProjectServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ProjectServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "CreateProject creates a new projects"]
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/CreateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "AddUserToProject Adds a new user to a given project"]
        pub async fn add_user_to_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/AddUserToProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/CreateAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "GetProjectDatasets Returns all datasets that belong to a certain project"]
        pub async fn get_project_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectDatasetsRequest>,
        ) -> Result<tonic::Response<super::GetProjectDatasetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/GetProjectDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "GetUserProjects Returns all projects that a specified user has access to"]
        pub async fn get_user_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/GetUserProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.services.v1.ProjectService/GetProject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all API token for a specific user, based on the provided oauth2 token"]
        pub async fn get_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.services.v1.ProjectService/GetAPIToken");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "DeleteProject Deletes a specific project"]
        #[doc = "Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database"]
        pub async fn delete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProjectRequest>,
        ) -> Result<tonic::Response<super::DeleteProjectResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/DeleteProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.ProjectService/DeleteAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod project_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProjectServiceServer."]
    #[async_trait]
    pub trait ProjectService: Send + Sync + 'static {
        #[doc = "CreateProject creates a new projects"]
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status>;
        #[doc = "AddUserToProject Adds a new user to a given project"]
        async fn add_user_to_project(
            &self,
            request: tonic::Request<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status>;
        async fn create_api_token(
            &self,
            request: tonic::Request<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status>;
        #[doc = "GetProjectDatasets Returns all datasets that belong to a certain project"]
        async fn get_project_datasets(
            &self,
            request: tonic::Request<super::GetProjectDatasetsRequest>,
        ) -> Result<tonic::Response<super::GetProjectDatasetsResponse>, tonic::Status>;
        #[doc = "GetUserProjects Returns all projects that a specified user has access to"]
        async fn get_user_projects(
            &self,
            request: tonic::Request<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status>;
        async fn get_project(
            &self,
            request: tonic::Request<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status>;
        #[doc = " Returns all API token for a specific user, based on the provided oauth2 token"]
        async fn get_api_token(
            &self,
            request: tonic::Request<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status>;
        #[doc = "DeleteProject Deletes a specific project"]
        #[doc = "Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database"]
        async fn delete_project(
            &self,
            request: tonic::Request<super::DeleteProjectRequest>,
        ) -> Result<tonic::Response<super::DeleteProjectResponse>, tonic::Status>;
        async fn delete_api_token(
            &self,
            request: tonic::Request<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProjectServiceServer<T: ProjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ProjectService> ProjectServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ProjectServiceServer<T>
    where
        T: ProjectService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.services.v1.ProjectService/CreateProject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService> tonic::server::UnaryService<super::CreateProjectRequest>
                        for CreateProjectSvc<T>
                    {
                        type Response = super::CreateProjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_project(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/AddUserToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService>
                        tonic::server::UnaryService<super::AddUserToProjectRequest>
                        for AddUserToProjectSvc<T>
                    {
                        type Response = super::AddUserToProjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUserToProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_user_to_project(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/CreateAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService>
                        tonic::server::UnaryService<super::CreateApiTokenRequest>
                        for CreateAPITokenSvc<T>
                    {
                        type Response = super::CreateApiTokenResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_api_token(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/GetProjectDatasets" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectDatasetsSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService>
                        tonic::server::UnaryService<super::GetProjectDatasetsRequest>
                        for GetProjectDatasetsSvc<T>
                    {
                        type Response = super::GetProjectDatasetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectDatasetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_project_datasets(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/GetUserProjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserProjectsSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService>
                        tonic::server::UnaryService<super::GetUserProjectsRequest>
                        for GetUserProjectsSvc<T>
                    {
                        type Response = super::GetUserProjectsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserProjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user_projects(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/GetProject" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService> tonic::server::UnaryService<super::GetProjectRequest> for GetProjectSvc<T> {
                        type Response = super::GetProjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/GetAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService> tonic::server::UnaryService<super::GetApiTokenRequest>
                        for GetAPITokenSvc<T>
                    {
                        type Response = super::GetApiTokenResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_api_token(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/DeleteProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService> tonic::server::UnaryService<super::DeleteProjectRequest>
                        for DeleteProjectSvc<T>
                    {
                        type Response = super::DeleteProjectResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_project(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.ProjectService/DeleteAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPITokenSvc<T: ProjectService>(pub Arc<T>);
                    impl<T: ProjectService>
                        tonic::server::UnaryService<super::DeleteApiTokenRequest>
                        for DeleteAPITokenSvc<T>
                    {
                        type Response = super::DeleteApiTokenResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApiTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_api_token(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: ProjectService> tonic::transport::NamedService for ProjectServiceServer<T> {
        const NAME: &'static str = "api.services.v1.ProjectService";
    }
}
#[doc = r" Generated client implementations."]
pub mod dataset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Dataset management service"]
    #[doc = " Manages all dataset related services"]
    #[doc = " All data objects are associated with one data dataset"]
    #[doc = " Dataset versions group these data objects, which makes them reusable"]
    #[derive(Debug, Clone)]
    pub struct DatasetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatasetServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
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
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DatasetServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DatasetServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " CreateNewDataset Creates a new dataset and associates it with a dataset"]
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::CreateDatasetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Dataset Returns a specific dataset"]
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::GetDatasetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/api.services.v1.DatasetService/GetDataset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Versions of a dataset"]
        pub async fn get_dataset_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetDatasetVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all object groups of a dataset"]
        pub async fn get_dataset_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetObjectGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetDatasetObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a signed link that can be used to download all objects from the"]
        #[doc = " specified request The link is signed using hmac and the resulting data can"]
        #[doc = " be shared without exposing any secrets"]
        pub async fn get_object_groups_stream_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsStreamLinkRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsStreamLinkResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetObjectGroupsStreamLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a field of a dataset"]
        pub async fn update_dataset_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetFieldRequest>,
        ) -> Result<tonic::Response<super::UpdateDatasetFieldResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/UpdateDatasetField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DeleteDataset Delete a dataset"]
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all object groups that were created within a specific date range"]
        #[doc = " The date range is not the date when the data was created in the system but"]
        #[doc = " byte the externally date that indicates the actual creation of the data"]
        #[doc = " rather than the date the data was ingested into the system"]
        pub async fn get_object_groups_in_date_range(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsInDateRangeRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsInDateRangeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetObjectGroupsInDateRange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReleaseDatasetVersion Release a new dataset version"]
        pub async fn release_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::ReleaseDatasetVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/ReleaseDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_dataset_version_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetVersionObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionObjectGroupsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetDatasetVersionObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/DeleteDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dataset_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatasetServiceServer."]
    #[async_trait]
    pub trait DatasetService: Send + Sync + 'static {
        #[doc = " CreateNewDataset Creates a new dataset and associates it with a dataset"]
        async fn create_dataset(
            &self,
            request: tonic::Request<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::CreateDatasetResponse>, tonic::Status>;
        #[doc = " Dataset Returns a specific dataset"]
        async fn get_dataset(
            &self,
            request: tonic::Request<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::GetDatasetResponse>, tonic::Status>;
        #[doc = " Lists Versions of a dataset"]
        async fn get_dataset_versions(
            &self,
            request: tonic::Request<super::GetDatasetVersionsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionsResponse>, tonic::Status>;
        #[doc = " Lists all object groups of a dataset"]
        async fn get_dataset_object_groups(
            &self,
            request: tonic::Request<super::GetDatasetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetObjectGroupsResponse>, tonic::Status>;
        #[doc = " Returns a signed link that can be used to download all objects from the"]
        #[doc = " specified request The link is signed using hmac and the resulting data can"]
        #[doc = " be shared without exposing any secrets"]
        async fn get_object_groups_stream_link(
            &self,
            request: tonic::Request<super::GetObjectGroupsStreamLinkRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsStreamLinkResponse>, tonic::Status>;
        #[doc = " Updates a field of a dataset"]
        async fn update_dataset_field(
            &self,
            request: tonic::Request<super::UpdateDatasetFieldRequest>,
        ) -> Result<tonic::Response<super::UpdateDatasetFieldResponse>, tonic::Status>;
        #[doc = " DeleteDataset Delete a dataset"]
        async fn delete_dataset(
            &self,
            request: tonic::Request<super::DeleteDatasetRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetResponse>, tonic::Status>;
        #[doc = " Returns all object groups that were created within a specific date range"]
        #[doc = " The date range is not the date when the data was created in the system but"]
        #[doc = " byte the externally date that indicates the actual creation of the data"]
        #[doc = " rather than the date the data was ingested into the system"]
        async fn get_object_groups_in_date_range(
            &self,
            request: tonic::Request<super::GetObjectGroupsInDateRangeRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsInDateRangeResponse>, tonic::Status>;
        #[doc = " ReleaseDatasetVersion Release a new dataset version"]
        async fn release_dataset_version(
            &self,
            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::ReleaseDatasetVersionResponse>, tonic::Status>;
        async fn get_dataset_version(
            &self,
            request: tonic::Request<super::GetDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionResponse>, tonic::Status>;
        async fn get_dataset_version_object_groups(
            &self,
            request: tonic::Request<super::GetDatasetVersionObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionObjectGroupsResponse>, tonic::Status>;
        async fn delete_dataset_version(
            &self,
            request: tonic::Request<super::DeleteDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::DeleteDatasetVersionResponse>, tonic::Status>;
    }
    #[doc = " Dataset management service"]
    #[doc = " Manages all dataset related services"]
    #[doc = " All data objects are associated with one data dataset"]
    #[doc = " Dataset versions group these data objects, which makes them reusable"]
    #[derive(Debug)]
    pub struct DatasetServiceServer<T: DatasetService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DatasetService> DatasetServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DatasetServiceServer<T>
    where
        T: DatasetService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.services.v1.DatasetService/CreateDataset" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::CreateDatasetRequest>
                        for CreateDatasetSvc<T>
                    {
                        type Response = super::CreateDatasetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_dataset(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetDataset" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::GetDatasetRequest> for GetDatasetSvc<T> {
                        type Response = super::GetDatasetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetDatasetVersions" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetDatasetVersionsRequest>
                        for GetDatasetVersionsSvc<T>
                    {
                        type Response = super::GetDatasetVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_dataset_versions(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetDatasetObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetObjectGroupsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetDatasetObjectGroupsRequest>
                        for GetDatasetObjectGroupsSvc<T>
                    {
                        type Response = super::GetDatasetObjectGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetObjectGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_dataset_object_groups(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetObjectGroupsStreamLink" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsStreamLinkSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetObjectGroupsStreamLinkRequest>
                        for GetObjectGroupsStreamLinkSvc<T>
                    {
                        type Response = super::GetObjectGroupsStreamLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupsStreamLinkRequest>,
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/UpdateDatasetField" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetFieldSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::UpdateDatasetFieldRequest>
                        for UpdateDatasetFieldSvc<T>
                    {
                        type Response = super::UpdateDatasetFieldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatasetFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_dataset_field(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/DeleteDataset" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::DeleteDatasetRequest>
                        for DeleteDatasetSvc<T>
                    {
                        type Response = super::DeleteDatasetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_dataset(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetObjectGroupsInDateRange" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsInDateRangeSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetObjectGroupsInDateRangeRequest>
                        for GetObjectGroupsInDateRangeSvc<T>
                    {
                        type Response = super::GetObjectGroupsInDateRangeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupsInDateRangeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups_in_date_range(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsInDateRangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/ReleaseDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::ReleaseDatasetVersionRequest>
                        for ReleaseDatasetVersionSvc<T>
                    {
                        type Response = super::ReleaseDatasetVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).release_dataset_version(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetDatasetVersionRequest>
                        for GetDatasetVersionSvc<T>
                    {
                        type Response = super::GetDatasetVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_dataset_version(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/GetDatasetVersionObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetVersionObjectGroupsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetDatasetVersionObjectGroupsRequest>
                        for GetDatasetVersionObjectGroupsSvc<T>
                    {
                        type Response = super::GetDatasetVersionObjectGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetVersionObjectGroupsRequest>,
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetService/DeleteDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::DeleteDatasetVersionRequest>
                        for DeleteDatasetVersionSvc<T>
                    {
                        type Response = super::DeleteDatasetVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDatasetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_dataset_version(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
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
    impl<T: DatasetService> tonic::transport::NamedService for DatasetServiceServer<T> {
        const NAME: &'static str = "api.services.v1.DatasetService";
    }
}
