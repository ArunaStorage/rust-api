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
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetVersionList {
    #[prost(message, repeated, tag = "1")]
    pub dataset_version: ::prost::alloc::vec::Vec<super::models::DatasetVersion>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<super::models::Version>,
    #[prost(string, repeated, tag = "4")]
    pub object_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupList {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::models::ObjectGroup>,
}
#[doc = r" Generated client implementations."]
pub mod dataset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Dataset management service"]
    #[doc = " Manages all dataset related services"]
    #[doc = " All data objects are associated with one data dataset"]
    #[doc = " Dataset versions group these data objects, which makes them reusable"]
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " CreateNewDataset Creates a new dataset and associates it with a dataset"]
        pub async fn create_new_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.DatasetService/CreateNewDataset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Dataset Returns a specific dataset"]
        pub async fn dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/services.DatasetService/Dataset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Versions of a dataset"]
        pub async fn dataset_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::DatasetVersionList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.DatasetService/DatasetVersions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn dataset_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::ObjectGroupList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetService/DatasetObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a field of a dataset"]
        pub async fn update_dataset_field(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::UpdateFieldsRequest>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.DatasetService/UpdateDatasetField");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DeleteDataset Delete a dataset"]
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.DatasetService/DeleteDataset");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "ReleaseDatasetVersion Release a new dataset version"]
        pub async fn release_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ReleaseDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::super::models::DatasetVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetService/ReleaseDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn dataset_version_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::ObjectGroupList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetService/DatasetVersionObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatasetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatasetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatasetServiceClient {{ ... }}")
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
        async fn create_new_dataset(
            &self,
            request: tonic::Request<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status>;
        #[doc = " Dataset Returns a specific dataset"]
        async fn dataset(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status>;
        #[doc = " Lists Versions of a dataset"]
        async fn dataset_versions(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::DatasetVersionList>, tonic::Status>;
        async fn dataset_object_groups(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::ObjectGroupList>, tonic::Status>;
        #[doc = " Updates a field of a dataset"]
        async fn update_dataset_field(
            &self,
            request: tonic::Request<super::super::models::UpdateFieldsRequest>,
        ) -> Result<tonic::Response<super::super::models::Dataset>, tonic::Status>;
        #[doc = " DeleteDataset Delete a dataset"]
        async fn delete_dataset(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status>;
        #[doc = "ReleaseDatasetVersion Release a new dataset version"]
        async fn release_dataset_version(
            &self,
            request: tonic::Request<super::ReleaseDatasetVersionRequest>,
        ) -> Result<tonic::Response<super::super::models::DatasetVersion>, tonic::Status>;
        async fn dataset_version_object_groups(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::ObjectGroupList>, tonic::Status>;
    }
    #[doc = " Dataset management service"]
    #[doc = " Manages all dataset related services"]
    #[doc = " All data objects are associated with one data dataset"]
    #[doc = " Dataset versions group these data objects, which makes them reusable"]
    #[derive(Debug)]
    pub struct DatasetServiceServer<T: DatasetService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DatasetService> DatasetServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DatasetServiceServer<T>
    where
        T: DatasetService,
        B: HttpBody + Send + Sync + 'static,
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
                "/services.DatasetService/CreateNewDataset" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNewDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::CreateDatasetRequest>
                        for CreateNewDatasetSvc<T>
                    {
                        type Response = super::super::models::Dataset;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_new_dataset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateNewDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/Dataset" => {
                    #[allow(non_camel_case_types)]
                    struct DatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::super::models::Id> for DatasetSvc<T> {
                        type Response = super::super::models::Dataset;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).dataset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/DatasetVersions" => {
                    #[allow(non_camel_case_types)]
                    struct DatasetVersionsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::super::models::Id>
                        for DatasetVersionsSvc<T>
                    {
                        type Response = super::DatasetVersionList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).dataset_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DatasetVersionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/DatasetObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct DatasetObjectGroupsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::super::models::Id>
                        for DatasetObjectGroupsSvc<T>
                    {
                        type Response = super::ObjectGroupList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).dataset_object_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DatasetObjectGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/UpdateDatasetField" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetFieldSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::super::models::UpdateFieldsRequest>
                        for UpdateDatasetFieldSvc<T>
                    {
                        type Response = super::super::models::Dataset;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::UpdateFieldsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_dataset_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDatasetFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/DeleteDataset" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::super::models::Id>
                        for DeleteDatasetSvc<T>
                    {
                        type Response = super::super::models::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_dataset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/ReleaseDatasetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct ReleaseDatasetVersionSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService>
                        tonic::server::UnaryService<super::ReleaseDatasetVersionRequest>
                        for ReleaseDatasetVersionSvc<T>
                    {
                        type Response = super::super::models::DatasetVersion;
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReleaseDatasetVersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetService/DatasetVersionObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct DatasetVersionObjectGroupsSvc<T: DatasetService>(pub Arc<T>);
                    impl<T: DatasetService> tonic::server::UnaryService<super::super::models::Id>
                        for DatasetVersionObjectGroupsSvc<T>
                    {
                        type Response = super::ObjectGroupList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).dataset_version_object_groups(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DatasetVersionObjectGroupsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DatasetService> Clone for DatasetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DatasetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatasetService> tonic::transport::NamedService for DatasetServiceServer<T> {
        const NAME: &'static str = "services.DatasetService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLinkResponse {
    #[prost(string, tag = "1")]
    pub upload_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::models::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectHeritageRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(message, repeated, tag = "4")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub object_heritage_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filetype: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(message, repeated, tag = "4")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag = "5")]
    pub content_len: i64,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag = "6")]
    pub origin: ::core::option::Option<super::models::Origin>,
}
#[doc = r" Generated client implementations."]
pub mod dataset_objects_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
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
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "CreateObjectHeritage Creates a new object heritage"]
        pub async fn create_object_heritage(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectHeritageRequest>,
        ) -> Result<tonic::Response<super::super::models::ObjectHeritage>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetObjectsService/CreateObjectHeritage",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "CreateObjectGroup Creates a new object group"]
        pub async fn create_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::super::models::ObjectGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetObjectsService/CreateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::ObjectGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetObjectsService/GetObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "FinishObjectUpload Finishes the upload process for an object"]
        pub async fn finish_object_upload(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.DatasetObjectsService/FinishObjectUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatasetObjectsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatasetObjectsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatasetObjectsServiceClient {{ ... }}")
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
        #[doc = "CreateObjectHeritage Creates a new object heritage"]
        async fn create_object_heritage(
            &self,
            request: tonic::Request<super::CreateObjectHeritageRequest>,
        ) -> Result<tonic::Response<super::super::models::ObjectHeritage>, tonic::Status>;
        #[doc = "CreateObjectGroup Creates a new object group"]
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::super::models::ObjectGroup>, tonic::Status>;
        async fn get_object_group(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::ObjectGroup>, tonic::Status>;
        #[doc = "FinishObjectUpload Finishes the upload process for an object"]
        async fn finish_object_upload(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct DatasetObjectsServiceServer<T: DatasetObjectsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DatasetObjectsService> DatasetObjectsServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for DatasetObjectsServiceServer<T>
    where
        T: DatasetObjectsService,
        B: HttpBody + Send + Sync + 'static,
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
                "/services.DatasetObjectsService/CreateObjectHeritage" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectHeritageSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::CreateObjectHeritageRequest>
                        for CreateObjectHeritageSvc<T>
                    {
                        type Response = super::super::models::ObjectHeritage;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectHeritageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_object_heritage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateObjectHeritageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetObjectsService/CreateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::CreateObjectGroupRequest>
                        for CreateObjectGroupSvc<T>
                    {
                        type Response = super::super::models::ObjectGroup;
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetObjectsService/GetObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::super::models::Id>
                        for GetObjectGroupSvc<T>
                    {
                        type Response = super::super::models::ObjectGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_object_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetObjectGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.DatasetObjectsService/FinishObjectUpload" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectUploadSvc<T: DatasetObjectsService>(pub Arc<T>);
                    impl<T: DatasetObjectsService>
                        tonic::server::UnaryService<super::super::models::Id>
                        for FinishObjectUploadSvc<T>
                    {
                        type Response = super::super::models::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finish_object_upload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FinishObjectUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: DatasetObjectsService> Clone for DatasetObjectsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DatasetObjectsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatasetObjectsService> tonic::transport::NamedService for DatasetObjectsServiceServer<T> {
        const NAME: &'static str = "services.DatasetObjectsService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::models::Right", repeated, tag = "2")]
    pub scope: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectList {
    #[prost(message, repeated, tag = "1")]
    pub projects: ::prost::alloc::vec::Vec<super::models::Project>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetList {
    #[prost(message, repeated, tag = "1")]
    pub dataset: ::prost::alloc::vec::Vec<super::models::Dataset>,
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
}
#[doc = r" Generated client implementations."]
pub mod project_api_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ProjectApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectApiClient<tonic::transport::Channel> {
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
    impl<T> ProjectApiClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "CreateProject creates a new projects"]
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::super::models::Project>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/services.ProjectAPI/CreateProject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "AddUserToProject Adds a new user to a given project"]
        pub async fn add_user_to_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::super::models::Project>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.ProjectAPI/AddUserToProject");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "GetProjectDatasets Returns all datasets that belong to a certain project"]
        pub async fn get_project_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::DatasetList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.ProjectAPI/GetProjectDatasets");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "GetUserProjects Returns all projects that a specified user has access to"]
        pub async fn get_user_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Empty>,
        ) -> Result<tonic::Response<super::ProjectList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/services.ProjectAPI/GetUserProjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "DeleteProject Deletes a specific project"]
        #[doc = "Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database"]
        pub async fn delete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/services.ProjectAPI/DeleteProject");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProjectApiClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProjectApiClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProjectApiClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod project_api_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProjectApiServer."]
    #[async_trait]
    pub trait ProjectApi: Send + Sync + 'static {
        #[doc = "CreateProject creates a new projects"]
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::super::models::Project>, tonic::Status>;
        #[doc = "AddUserToProject Adds a new user to a given project"]
        async fn add_user_to_project(
            &self,
            request: tonic::Request<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::super::models::Project>, tonic::Status>;
        #[doc = "GetProjectDatasets Returns all datasets that belong to a certain project"]
        async fn get_project_datasets(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::DatasetList>, tonic::Status>;
        #[doc = "GetUserProjects Returns all projects that a specified user has access to"]
        async fn get_user_projects(
            &self,
            request: tonic::Request<super::super::models::Empty>,
        ) -> Result<tonic::Response<super::ProjectList>, tonic::Status>;
        #[doc = "DeleteProject Deletes a specific project"]
        #[doc = "Will also delete all associated resources (Datasets/Objects/etc...) both from objects storage and the database"]
        async fn delete_project(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::super::models::Empty>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ProjectApiServer<T: ProjectApi> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProjectApi> ProjectApiServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ProjectApiServer<T>
    where
        T: ProjectApi,
        B: HttpBody + Send + Sync + 'static,
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
                "/services.ProjectAPI/CreateProject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProjectSvc<T: ProjectApi>(pub Arc<T>);
                    impl<T: ProjectApi> tonic::server::UnaryService<super::CreateProjectRequest>
                        for CreateProjectSvc<T>
                    {
                        type Response = super::super::models::Project;
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.ProjectAPI/AddUserToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToProjectSvc<T: ProjectApi>(pub Arc<T>);
                    impl<T: ProjectApi> tonic::server::UnaryService<super::AddUserToProjectRequest>
                        for AddUserToProjectSvc<T>
                    {
                        type Response = super::super::models::Project;
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
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddUserToProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.ProjectAPI/GetProjectDatasets" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectDatasetsSvc<T: ProjectApi>(pub Arc<T>);
                    impl<T: ProjectApi> tonic::server::UnaryService<super::super::models::Id>
                        for GetProjectDatasetsSvc<T>
                    {
                        type Response = super::DatasetList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_project_datasets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProjectDatasetsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.ProjectAPI/GetUserProjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserProjectsSvc<T: ProjectApi>(pub Arc<T>);
                    impl<T: ProjectApi> tonic::server::UnaryService<super::super::models::Empty>
                        for GetUserProjectsSvc<T>
                    {
                        type Response = super::ProjectList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Empty>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user_projects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetUserProjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.ProjectAPI/DeleteProject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProjectSvc<T: ProjectApi>(pub Arc<T>);
                    impl<T: ProjectApi> tonic::server::UnaryService<super::super::models::Id> for DeleteProjectSvc<T> {
                        type Response = super::super::models::Empty;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_project(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteProjectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ProjectApi> Clone for ProjectApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProjectApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProjectApi> tonic::transport::NamedService for ProjectApiServer<T> {
        const NAME: &'static str = "services.ProjectAPI";
    }
}
#[doc = r" Generated client implementations."]
pub mod object_load_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ObjectLoadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectLoadClient<tonic::transport::Channel> {
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
    impl<T> ObjectLoadClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create_upload_link(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::CreateLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.ObjectLoad/CreateUploadLink");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_download_link(
            &mut self,
            request: impl tonic::IntoRequest<super::super::models::Id>,
        ) -> Result<tonic::Response<super::CreateLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/services.ObjectLoad/CreateDownloadLink");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ObjectLoadClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ObjectLoadClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ObjectLoadClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_load_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectLoadServer."]
    #[async_trait]
    pub trait ObjectLoad: Send + Sync + 'static {
        async fn create_upload_link(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::CreateLinkResponse>, tonic::Status>;
        async fn create_download_link(
            &self,
            request: tonic::Request<super::super::models::Id>,
        ) -> Result<tonic::Response<super::CreateLinkResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ObjectLoadServer<T: ObjectLoad> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ObjectLoad> ObjectLoadServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ObjectLoadServer<T>
    where
        T: ObjectLoad,
        B: HttpBody + Send + Sync + 'static,
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
                "/services.ObjectLoad/CreateUploadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUploadLinkSvc<T: ObjectLoad>(pub Arc<T>);
                    impl<T: ObjectLoad> tonic::server::UnaryService<super::super::models::Id>
                        for CreateUploadLinkSvc<T>
                    {
                        type Response = super::CreateLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_upload_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateUploadLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.ObjectLoad/CreateDownloadLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinkSvc<T: ObjectLoad>(pub Arc<T>);
                    impl<T: ObjectLoad> tonic::server::UnaryService<super::super::models::Id>
                        for CreateDownloadLinkSvc<T>
                    {
                        type Response = super::CreateLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::models::Id>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_download_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDownloadLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ObjectLoad> Clone for ObjectLoadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ObjectLoad> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectLoad> tonic::transport::NamedService for ObjectLoadServer<T> {
        const NAME: &'static str = "services.ObjectLoad";
    }
}
