#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "4")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    #[prost(message, optional, tag = "5")]
    pub object_group_version: ::core::option::Option<CreateObjectGroupRevisionRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupResponse {
    #[prost(string, tag = "1")]
    pub object_group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRevisionRequest {
    #[prost(message, repeated, tag = "4")]
    pub objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRevisionToObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub object_group_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub group_version: ::core::option::Option<CreateObjectGroupRevisionRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRevisionToObjectGroupResponse {
    #[prost(string, tag = "1")]
    pub revision_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub revision_number: u64,
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
pub struct GetObjectGroupRevisionRequest {
    #[prost(enumeration = "ObjectGroupRevisionReferenceType", tag = "1")]
    pub reference_type: i32,
    #[prost(int64, tag = "2")]
    pub revision: i64,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
    #[prost(string, tag = "4")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionResponse {
    #[prost(message, optional, tag = "1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroup>,
    #[prost(message, optional, tag = "2")]
    pub object_group_revision:
        ::core::option::Option<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentObjectGroupRevisionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentObjectGroupRevisionResponse {
    #[prost(message, optional, tag = "1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroup>,
    #[prost(message, optional, tag = "2")]
    pub object_group_revision:
        ::core::option::Option<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupRevisionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_group_revision:
        ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRevisionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRevisionResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ObjectGroupRevisionReferenceType {
    Version = 0,
    Revision = 1,
    Id = 2,
}
#[doc = r" Generated client implementations."]
pub mod dataset_objects_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
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
        T::ResponseBody: Body + Send + Sync + 'static,
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
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
        #[doc = "CreateObjectGroup Creates a new object group"]
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
        #[doc = "CreateObjectGroupVersion Creates a new object group version"]
        pub async fn add_revision_to_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::AddRevisionToObjectGroupRequest>,
        ) -> Result<tonic::Response<super::AddRevisionToObjectGroupResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/AddRevisionToObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "GetObjectGroup Returns the object group with the given ID"]
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
        #[doc = "GetObjectGroupCurrentVersion Returns the head version in the history of a given object group"]
        pub async fn get_current_object_group_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrentObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::GetCurrentObjectGroupRevisionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/GetCurrentObjectGroupRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_group_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupRevisionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/GetObjectGroupRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_group_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupRevisionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/GetObjectGroupRevisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "FinishObjectUpload Finishes the upload process for an object"]
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
        pub async fn delete_object_group_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupRevisionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetObjectsService/DeleteObjectGroupRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dataset_objects_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatasetObjectsServiceServer."]
    #[async_trait]
    pub trait DatasetObjectsService: Send + Sync + 'static {
        #[doc = "CreateObjectGroup Creates a new object group"]
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status>;
        #[doc = "CreateObjectGroupVersion Creates a new object group version"]
        async fn add_revision_to_object_group(
            &self,
            request: tonic::Request<super::AddRevisionToObjectGroupRequest>,
        ) -> Result<tonic::Response<super::AddRevisionToObjectGroupResponse>, tonic::Status>;
        #[doc = "GetObjectGroup Returns the object group with the given ID"]
        async fn get_object_group(
            &self,
            request: tonic::Request<super::GetObjectGroupRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupResponse>, tonic::Status>;
        #[doc = "GetObjectGroupCurrentVersion Returns the head version in the history of a given object group"]
        async fn get_current_object_group_revision(
            &self,
            request: tonic::Request<super::GetCurrentObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::GetCurrentObjectGroupRevisionResponse>, tonic::Status>;
        async fn get_object_group_revision(
            &self,
            request: tonic::Request<super::GetObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupRevisionResponse>, tonic::Status>;
        async fn get_object_group_revisions(
            &self,
            request: tonic::Request<super::GetObjectGroupRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupRevisionsResponse>, tonic::Status>;
        #[doc = "FinishObjectUpload Finishes the upload process for an object"]
        async fn finish_object_upload(
            &self,
            request: tonic::Request<super::FinishObjectUploadRequest>,
        ) -> Result<tonic::Response<super::FinishObjectUploadResponse>, tonic::Status>;
        async fn delete_object_group(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status>;
        async fn delete_object_group_revision(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRevisionRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupRevisionResponse>, tonic::Status>;
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for DatasetObjectsServiceServer<T>
    where
        T: DatasetObjectsService,
        B: Body + Send + Sync + 'static,
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
                "/api.services.v1.DatasetObjectsService/AddRevisionToObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct AddRevisionToObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::AddRevisionToObjectGroupRequest>
                        for AddRevisionToObjectGroupSvc<T>
                    {
                        type Response = super::AddRevisionToObjectGroupResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddRevisionToObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).add_revision_to_object_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddRevisionToObjectGroupSvc(inner);
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
                "/api.services.v1.DatasetObjectsService/GetCurrentObjectGroupRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentObjectGroupRevisionSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::GetCurrentObjectGroupRevisionRequest>
                        for GetCurrentObjectGroupRevisionSvc<T>
                    {
                        type Response = super::GetCurrentObjectGroupRevisionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCurrentObjectGroupRevisionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_current_object_group_revision(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCurrentObjectGroupRevisionSvc(inner);
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
                "/api.services.v1.DatasetObjectsService/GetObjectGroupRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupRevisionSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::GetObjectGroupRevisionRequest>
                        for GetObjectGroupRevisionSvc<T>
                    {
                        type Response = super::GetObjectGroupRevisionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupRevisionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_object_group_revision(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.services.v1.DatasetObjectsService/GetObjectGroupRevisions" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupRevisionsSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::GetObjectGroupRevisionsRequest>
                        for GetObjectGroupRevisionsSvc<T>
                    {
                        type Response = super::GetObjectGroupRevisionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupRevisionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_object_group_revisions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupRevisionsSvc(inner);
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
                "/api.services.v1.DatasetObjectsService/DeleteObjectGroupRevision" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectGroupRevisionSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::DeleteObjectGroupRevisionRequest>
                        for DeleteObjectGroupRevisionSvc<T>
                    {
                        type Response = super::DeleteObjectGroupRevisionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectGroupRevisionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_object_group_revision(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectGroupRevisionSvc(inner);
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
/// Dataset related Models
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Name of the dataset
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetObjectGroupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentObjectGroupRevisionsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentObjectGroupRevisionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_group_revisions:
        ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
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
    #[prost(string, repeated, tag = "4")]
    pub object_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<super::super::models::v1::Metadata>,
    #[prost(string, repeated, tag = "7")]
    pub revision_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
pub struct GetDatsetVersionRevisionsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatsetVersionRevisionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub object_group_revision:
        ::prost::alloc::vec::Vec<super::super::models::v1::ObjectGroupRevision>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionResponse {}
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
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinkResponse {
    #[prost(string, tag = "1")]
    pub upload_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
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
#[doc = r" Generated client implementations."]
pub mod object_load_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
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
        T::ResponseBody: Body + Send + Sync + 'static,
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectLoadServiceServer."]
    #[async_trait]
    pub trait ObjectLoadService: Send + Sync + 'static {
        async fn create_upload_link(
            &self,
            request: tonic::Request<super::CreateUploadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status>;
        async fn create_download_link(
            &self,
            request: tonic::Request<super::CreateDownloadLinkRequest>,
        ) -> Result<tonic::Response<super::CreateDownloadLinkResponse>, tonic::Status>;
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for ObjectLoadServiceServer<T>
    where
        T: ObjectLoadService,
        B: Body + Send + Sync + 'static,
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
    pub project: ::prost::alloc::string::String,
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
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::Label>,
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
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v1::Project>,
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
    #![allow(unused_variables, dead_code, missing_docs)]
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
        T::ResponseBody: Body + Send + Sync + 'static,
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
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
    #![allow(unused_variables, dead_code, missing_docs)]
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for ProjectServiceServer<T>
    where
        T: ProjectService,
        B: Body + Send + Sync + 'static,
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
    #![allow(unused_variables, dead_code, missing_docs)]
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
        T::ResponseBody: Body + Send + Sync + 'static,
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as Service<http::Request<tonic::body::BoxBody>>>::Error:
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
        pub async fn get_current_object_group_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrentObjectGroupRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetCurrentObjectGroupRevisionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetCurrentObjectGroupRevisions",
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
        #[doc = "ReleaseDatasetVersion Release a new dataset version"]
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
        pub async fn get_datset_version_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatsetVersionRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetDatsetVersionRevisionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.services.v1.DatasetService/GetDatsetVersionRevisions",
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
    #![allow(unused_variables, dead_code, missing_docs)]
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
        async fn get_dataset_object_groups(
            &self,
            request: tonic::Request<super::GetDatasetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetDatasetObjectGroupsResponse>, tonic::Status>;
        async fn get_current_object_group_revisions(
            &self,
            request: tonic::Request<super::GetCurrentObjectGroupRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetCurrentObjectGroupRevisionsResponse>, tonic::Status>;
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
        #[doc = "ReleaseDatasetVersion Release a new dataset version"]
        async fn release_dataset_version(
            &self,
            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::ReleaseDatasetVersionResponse>, tonic::Status>;
        async fn get_dataset_version(
            &self,
            request: tonic::Request<super::GetDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::GetDatasetVersionResponse>, tonic::Status>;
        async fn get_datset_version_revisions(
            &self,
            request: tonic::Request<super::GetDatsetVersionRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetDatsetVersionRevisionsResponse>, tonic::Status>;
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
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> Service<http::Request<B>> for DatasetServiceServer<T>
    where
        T: DatasetService,
        B: Body + Send + Sync + 'static,
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
                "/api.services.v1.DatasetService/GetCurrentObjectGroupRevisions" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentObjectGroupRevisionsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetCurrentObjectGroupRevisionsRequest>
                        for GetCurrentObjectGroupRevisionsSvc<T>
                    {
                        type Response = super::GetCurrentObjectGroupRevisionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCurrentObjectGroupRevisionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_current_object_group_revisions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCurrentObjectGroupRevisionsSvc(inner);
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
                "/api.services.v1.DatasetService/GetDatsetVersionRevisions" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatsetVersionRevisionsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::GetDatsetVersionRevisionsRequest>
                        for GetDatsetVersionRevisionsSvc<T>
                    {
                        type Response = super::GetDatsetVersionRevisionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatsetVersionRevisionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_datset_version_revisions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatsetVersionRevisionsSvc(inner);
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
