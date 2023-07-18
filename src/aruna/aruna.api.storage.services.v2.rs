#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    /// Project name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Project specific labels / hooks
    #[prost(message, repeated, tag = "3")]
    pub key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// External relations (URLs / IDs from external sources)
    #[prost(message, repeated, tag = "4")]
    pub external_relations: ::prost::alloc::vec::Vec<
        super::super::models::v2::ExternalRelation,
    >,
    /// DataClass
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "5")]
    pub data_class: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    /// The freshly created project
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    /// The id of the project to get
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectResponse {
    /// Overview of the projectroject
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectsRequest {
    /// optional filter for specific ids
    #[prost(string, repeated, tag = "1")]
    pub project_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectsResponse {
    /// Overview of the projects
    #[prost(message, repeated, tag = "1")]
    pub projects: ::prost::alloc::vec::Vec<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectRequest {
    /// The id of the project to destroy
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectNameRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectNameResponse {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectDescriptionRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectDescriptionResponse {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectKeyValuesRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub add_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    #[prost(message, repeated, tag = "3")]
    pub remove_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectKeyValuesResponse {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectDataClassRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "2")]
    pub data_class: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectDataClassResponse {
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveProjectResponse {
    /// This project will be returned via an Persistent Identifier! Updates will be impossible
    #[prost(message, optional, tag = "1")]
    pub project: ::core::option::Option<super::super::models::v2::Project>,
}
/// Generated client implementations.
pub mod project_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ProjectService
    ///
    /// Contains all methods that get/create or update Projects and associated resources
    #[derive(Debug, Clone)]
    pub struct ProjectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectServiceClient<tonic::transport::Channel> {
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
        /// CreateProject
        ///
        /// Status: BETA
        ///
        /// Creates a new project. All subsequent resources are part of a project.
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateProjectResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/CreateProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "CreateProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetProject
        ///
        /// Status: BETA
        ///
        /// Requests a project (by id)
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/GetProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "GetProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetProjects
        ///
        /// Status: BETA
        ///
        /// Admin request to get all projects
        pub async fn get_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectsResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/GetProjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "GetProjects",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteProject
        ///
        /// Status: BETA
        ///
        /// Deletes the project and all its associated data. Must be empty!
        pub async fn delete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteProjectResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/DeleteProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "DeleteProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateProjectName
        ///
        /// Status: BETA
        ///
        /// Updates the project name. Caveat! Will rename the "s3 bucket" for data proxies!
        pub async fn update_project_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectNameResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "UpdateProjectName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateProjectDescription
        ///
        /// Status: BETA
        ///
        /// Updates the project name.
        pub async fn update_project_description(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectDescriptionResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectDescription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "UpdateProjectDescription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateProjectKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the project key values.
        pub async fn update_project_key_values(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectKeyValuesResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectKeyValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "UpdateProjectKeyValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateProjectDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the project name. All (meta) data will be overwritten.
        pub async fn update_project_data_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectDataClassResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectDataClass",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "UpdateProjectDataClass",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ArchiveProjectRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full project, rendering all downstream relations immutable
        pub async fn archive_project(
            &mut self,
            request: impl tonic::IntoRequest<super::ArchiveProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveProjectResponse>,
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
                "/aruna.api.storage.services.v2.ProjectService/ArchiveProject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ProjectService",
                        "ArchiveProject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod project_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ProjectServiceServer.
    #[async_trait]
    pub trait ProjectService: Send + Sync + 'static {
        /// CreateProject
        ///
        /// Status: BETA
        ///
        /// Creates a new project. All subsequent resources are part of a project.
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateProjectResponse>,
            tonic::Status,
        >;
        /// GetProject
        ///
        /// Status: BETA
        ///
        /// Requests a project (by id)
        async fn get_project(
            &self,
            request: tonic::Request<super::GetProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectResponse>,
            tonic::Status,
        >;
        /// GetProjects
        ///
        /// Status: BETA
        ///
        /// Admin request to get all projects
        async fn get_projects(
            &self,
            request: tonic::Request<super::GetProjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetProjectsResponse>,
            tonic::Status,
        >;
        /// DeleteProject
        ///
        /// Status: BETA
        ///
        /// Deletes the project and all its associated data. Must be empty!
        async fn delete_project(
            &self,
            request: tonic::Request<super::DeleteProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteProjectResponse>,
            tonic::Status,
        >;
        /// UpdateProjectName
        ///
        /// Status: BETA
        ///
        /// Updates the project name. Caveat! Will rename the "s3 bucket" for data proxies!
        async fn update_project_name(
            &self,
            request: tonic::Request<super::UpdateProjectNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectNameResponse>,
            tonic::Status,
        >;
        /// UpdateProjectDescription
        ///
        /// Status: BETA
        ///
        /// Updates the project name.
        async fn update_project_description(
            &self,
            request: tonic::Request<super::UpdateProjectDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectDescriptionResponse>,
            tonic::Status,
        >;
        /// UpdateProjectKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the project key values.
        async fn update_project_key_values(
            &self,
            request: tonic::Request<super::UpdateProjectKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectKeyValuesResponse>,
            tonic::Status,
        >;
        /// UpdateProjectDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the project name. All (meta) data will be overwritten.
        async fn update_project_data_class(
            &self,
            request: tonic::Request<super::UpdateProjectDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateProjectDataClassResponse>,
            tonic::Status,
        >;
        /// ArchiveProjectRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full project, rendering all downstream relations immutable
        async fn archive_project(
            &self,
            request: tonic::Request<super::ArchiveProjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ArchiveProjectResponse>,
            tonic::Status,
        >;
    }
    /// ProjectService
    ///
    /// Contains all methods that get/create or update Projects and associated resources
    #[derive(Debug)]
    pub struct ProjectServiceServer<T: ProjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v2.ProjectService/CreateProject" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_project(request).await
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
                        let method = CreateProjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/GetProject" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_project(request).await };
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
                        let method = GetProjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/GetProjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectsSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::GetProjectsRequest>
                    for GetProjectsSvc<T> {
                        type Response = super::GetProjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_projects(request).await
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
                        let method = GetProjectsSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/DeleteProject" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_project(request).await
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
                        let method = DeleteProjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectName" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectNameSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::UpdateProjectNameRequest>
                    for UpdateProjectNameSvc<T> {
                        type Response = super::UpdateProjectNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_project_name(request).await
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
                        let method = UpdateProjectNameSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectDescription" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectDescriptionSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::UpdateProjectDescriptionRequest>
                    for UpdateProjectDescriptionSvc<T> {
                        type Response = super::UpdateProjectDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateProjectDescriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_project_description(request).await
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
                        let method = UpdateProjectDescriptionSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectKeyValues" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectKeyValuesSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::UpdateProjectKeyValuesRequest>
                    for UpdateProjectKeyValuesSvc<T> {
                        type Response = super::UpdateProjectKeyValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectKeyValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_project_key_values(request).await
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
                        let method = UpdateProjectKeyValuesSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/UpdateProjectDataClass" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectDataClassSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::UpdateProjectDataClassRequest>
                    for UpdateProjectDataClassSvc<T> {
                        type Response = super::UpdateProjectDataClassResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectDataClassRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_project_data_class(request).await
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
                        let method = UpdateProjectDataClassSvc(inner);
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
                "/aruna.api.storage.services.v2.ProjectService/ArchiveProject" => {
                    #[allow(non_camel_case_types)]
                    struct ArchiveProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::ArchiveProjectRequest>
                    for ArchiveProjectSvc<T> {
                        type Response = super::ArchiveProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ArchiveProjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).archive_project(request).await
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
                        let method = ArchiveProjectSvc(inner);
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
    impl<T: ProjectService> Clone for ProjectServiceServer<T> {
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
    impl<T: ProjectService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProjectService> tonic::server::NamedService for ProjectServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.ProjectService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStorageVersionRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticVersion {
    /// Complete version string
    #[prost(string, tag = "1")]
    pub version_string: ::prost::alloc::string::String,
    /// Semver according to <https://semver.org/>
    #[prost(int32, tag = "2")]
    pub major: i32,
    #[prost(int32, tag = "3")]
    pub minor: i32,
    #[prost(int32, tag = "4")]
    pub patch: i32,
    #[prost(string, tag = "5")]
    pub labels: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationVersion {
    /// Status of a specific Location e.g Gießen / dataproxy / 0.5.0-beta.1
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub version: ::prost::alloc::vec::Vec<ComponentVersion>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentVersion {
    /// Name of a specific component e.g. server, dataproxy etc. and their status by location
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<SemanticVersion>,
}
/// Version of each component by location
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStorageVersionResponse {
    #[prost(message, repeated, tag = "1")]
    pub location_version: ::prost::alloc::vec::Vec<LocationVersion>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStorageStatusRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationStatus {
    /// Status of a specific Location e.g Gießen / AVAILABLE
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub component_status: ::prost::alloc::vec::Vec<ComponentStatus>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentStatus {
    /// Name of a specific component e.g. server, dataproxy etc. and their status by location
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::ComponentStatus", tag = "2")]
    pub status: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStorageStatusResponse {
    /// List of all locations and their component / status
    #[prost(message, repeated, tag = "1")]
    pub location_status: ::prost::alloc::vec::Vec<LocationStatus>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubkeysRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pubkey {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub location: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPubkeysResponse {
    #[prost(message, repeated, tag = "1")]
    pub pubkeys: ::prost::alloc::vec::Vec<Pubkey>,
}
/// Generated client implementations.
pub mod storage_status_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// StorageStatusService
    ///
    /// This is a generic service that contains utility functions
    /// these functions are used to query additional meta-information
    /// about the status of the overall storage architecture
    #[derive(Debug, Clone)]
    pub struct StorageStatusServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageStatusServiceClient<tonic::transport::Channel> {
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
    impl<T> StorageStatusServiceClient<T>
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
        ) -> StorageStatusServiceClient<InterceptedService<T, F>>
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
            StorageStatusServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// GetStorageVersion
        ///
        /// Status: BETA
        ///
        /// A request to get the current version of the server application
        /// String representation and https://semver.org/
        pub async fn get_storage_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStorageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStorageVersionResponse>,
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetStorageVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.StorageStatusService",
                        "GetStorageVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetStorageStatus
        ///
        /// Status: ALPHA
        ///
        /// A request to get the current status of the storage components by location(s)
        pub async fn get_storage_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStorageStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStorageStatusResponse>,
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetStorageStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.StorageStatusService",
                        "GetStorageStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_pubkeys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPubkeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPubkeysResponse>,
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetPubkeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.StorageStatusService",
                        "GetPubkeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod storage_status_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with StorageStatusServiceServer.
    #[async_trait]
    pub trait StorageStatusService: Send + Sync + 'static {
        /// GetStorageVersion
        ///
        /// Status: BETA
        ///
        /// A request to get the current version of the server application
        /// String representation and https://semver.org/
        async fn get_storage_version(
            &self,
            request: tonic::Request<super::GetStorageVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStorageVersionResponse>,
            tonic::Status,
        >;
        /// GetStorageStatus
        ///
        /// Status: ALPHA
        ///
        /// A request to get the current status of the storage components by location(s)
        async fn get_storage_status(
            &self,
            request: tonic::Request<super::GetStorageStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStorageStatusResponse>,
            tonic::Status,
        >;
        async fn get_pubkeys(
            &self,
            request: tonic::Request<super::GetPubkeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPubkeysResponse>,
            tonic::Status,
        >;
    }
    /// StorageStatusService
    ///
    /// This is a generic service that contains utility functions
    /// these functions are used to query additional meta-information
    /// about the status of the overall storage architecture
    #[derive(Debug)]
    pub struct StorageStatusServiceServer<T: StorageStatusService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StorageStatusService> StorageStatusServiceServer<T> {
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
    for StorageStatusServiceServer<T>
    where
        T: StorageStatusService,
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetStorageVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetStorageVersionSvc<T: StorageStatusService>(pub Arc<T>);
                    impl<
                        T: StorageStatusService,
                    > tonic::server::UnaryService<super::GetStorageVersionRequest>
                    for GetStorageVersionSvc<T> {
                        type Response = super::GetStorageVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStorageVersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_storage_version(request).await
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
                        let method = GetStorageVersionSvc(inner);
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetStorageStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStorageStatusSvc<T: StorageStatusService>(pub Arc<T>);
                    impl<
                        T: StorageStatusService,
                    > tonic::server::UnaryService<super::GetStorageStatusRequest>
                    for GetStorageStatusSvc<T> {
                        type Response = super::GetStorageStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStorageStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_storage_status(request).await
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
                        let method = GetStorageStatusSvc(inner);
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
                "/aruna.api.storage.services.v2.StorageStatusService/GetPubkeys" => {
                    #[allow(non_camel_case_types)]
                    struct GetPubkeysSvc<T: StorageStatusService>(pub Arc<T>);
                    impl<
                        T: StorageStatusService,
                    > tonic::server::UnaryService<super::GetPubkeysRequest>
                    for GetPubkeysSvc<T> {
                        type Response = super::GetPubkeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPubkeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_pubkeys(request).await };
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
                        let method = GetPubkeysSvc(inner);
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
    impl<T: StorageStatusService> Clone for StorageStatusServiceServer<T> {
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
    impl<T: StorageStatusService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageStatusService> tonic::server::NamedService
    for StorageStatusServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.StorageStatusService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollectionRequest {
    /// collection name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// collection specific labels / hooks
    #[prost(message, repeated, tag = "3")]
    pub key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// External relations (URLs / IDs from external sources)
    #[prost(message, repeated, tag = "4")]
    pub external_relations: ::prost::alloc::vec::Vec<
        super::super::models::v2::ExternalRelation,
    >,
    /// DataClass
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "5")]
    pub data_class: i32,
    /// Parent_id MUST be collection
    #[prost(oneof = "create_collection_request::Parent", tags = "6")]
    pub parent: ::core::option::Option<create_collection_request::Parent>,
}
/// Nested message and enum types in `CreateCollectionRequest`.
pub mod create_collection_request {
    /// Parent_id MUST be collection
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parent {
        #[prost(string, tag = "6")]
        ProjectId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollectionResponse {
    /// The new collection_id
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionRequest {
    /// Requested id
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionResponse {
    /// Overview of the requested collection
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsRequest {
    #[prost(string, repeated, tag = "1")]
    pub collection_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsResponse {
    /// List of collection overviews
    #[prost(message, repeated, tag = "1")]
    pub collections: ::prost::alloc::vec::Vec<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionNameRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionNameResponse {
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionDescriptionRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionDescriptionResponse {
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionKeyValuesRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub add_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    #[prost(message, repeated, tag = "3")]
    pub remove_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionKeyValuesResponse {
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionDataClassRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "2")]
    pub data_class: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionDataClassResponse {
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotCollectionRequest {
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotCollectionResponse {
    /// This collection will be returned via an Persistent Identifier! Updates will be impossible
    #[prost(message, optional, tag = "1")]
    pub collection: ::core::option::Option<super::super::models::v2::Collection>,
}
/// Generated client implementations.
pub mod collection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// CollectionService
    ///
    /// Contains all methods that get/create or update Collection and associated resources
    #[derive(Debug, Clone)]
    pub struct CollectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CollectionServiceClient<tonic::transport::Channel> {
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
    impl<T> CollectionServiceClient<T>
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
        ) -> CollectionServiceClient<InterceptedService<T, F>>
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
            CollectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateNewCollection
        ///
        /// Status: BETA
        ///
        /// creates a new Collection
        pub async fn create_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCollectionResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/CreateCollection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "CreateCollection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetCollection
        ///
        /// Status: BETA
        ///
        /// Request a specific collection by ID
        pub async fn get_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCollectionResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/GetCollection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "GetCollection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetCollections
        ///
        /// Status: BETA
        ///
        /// Queries multiple collections by ID
        pub async fn get_collections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCollectionsResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/GetCollections",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "GetCollections",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCollection
        ///
        /// Status: STABLE
        ///
        /// This request deletes the collection.
        pub async fn delete_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCollectionResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/DeleteCollection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "DeleteCollection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCollectionName
        ///
        /// Status: BETA
        ///
        /// Updates the collection name. Caveat! Will rename the "s3 bucket" for data proxies!
        pub async fn update_collection_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionNameResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "UpdateCollectionName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCollectionDescription
        ///
        /// Status: BETA
        ///
        /// Updates the collection description.
        pub async fn update_collection_description(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionDescriptionResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionDescription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "UpdateCollectionDescription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCollectionKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the collection key values.
        pub async fn update_collection_key_values(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionKeyValuesResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionKeyValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "UpdateCollectionKeyValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCollectionDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the collection name. All (meta) data will be overwritten.
        pub async fn update_collection_data_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionDataClassResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionDataClass",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "UpdateCollectionDataClass",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SnapshotCollectionRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full collection, rendering all downstream relations immutable
        pub async fn snapshot_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SnapshotCollectionResponse>,
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
                "/aruna.api.storage.services.v2.CollectionService/SnapshotCollection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.CollectionService",
                        "SnapshotCollection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod collection_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CollectionServiceServer.
    #[async_trait]
    pub trait CollectionService: Send + Sync + 'static {
        /// CreateNewCollection
        ///
        /// Status: BETA
        ///
        /// creates a new Collection
        async fn create_collection(
            &self,
            request: tonic::Request<super::CreateCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateCollectionResponse>,
            tonic::Status,
        >;
        /// GetCollection
        ///
        /// Status: BETA
        ///
        /// Request a specific collection by ID
        async fn get_collection(
            &self,
            request: tonic::Request<super::GetCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCollectionResponse>,
            tonic::Status,
        >;
        /// GetCollections
        ///
        /// Status: BETA
        ///
        /// Queries multiple collections by ID
        async fn get_collections(
            &self,
            request: tonic::Request<super::GetCollectionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCollectionsResponse>,
            tonic::Status,
        >;
        /// DeleteCollection
        ///
        /// Status: STABLE
        ///
        /// This request deletes the collection.
        async fn delete_collection(
            &self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteCollectionResponse>,
            tonic::Status,
        >;
        /// UpdateCollectionName
        ///
        /// Status: BETA
        ///
        /// Updates the collection name. Caveat! Will rename the "s3 bucket" for data proxies!
        async fn update_collection_name(
            &self,
            request: tonic::Request<super::UpdateCollectionNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionNameResponse>,
            tonic::Status,
        >;
        /// UpdateCollectionDescription
        ///
        /// Status: BETA
        ///
        /// Updates the collection description.
        async fn update_collection_description(
            &self,
            request: tonic::Request<super::UpdateCollectionDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionDescriptionResponse>,
            tonic::Status,
        >;
        /// UpdateCollectionKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the collection key values.
        async fn update_collection_key_values(
            &self,
            request: tonic::Request<super::UpdateCollectionKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionKeyValuesResponse>,
            tonic::Status,
        >;
        /// UpdateCollectionDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the collection name. All (meta) data will be overwritten.
        async fn update_collection_data_class(
            &self,
            request: tonic::Request<super::UpdateCollectionDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateCollectionDataClassResponse>,
            tonic::Status,
        >;
        /// SnapshotCollectionRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full collection, rendering all downstream relations immutable
        async fn snapshot_collection(
            &self,
            request: tonic::Request<super::SnapshotCollectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SnapshotCollectionResponse>,
            tonic::Status,
        >;
    }
    /// CollectionService
    ///
    /// Contains all methods that get/create or update Collection and associated resources
    #[derive(Debug)]
    pub struct CollectionServiceServer<T: CollectionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CollectionService> CollectionServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CollectionServiceServer<T>
    where
        T: CollectionService,
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
                "/aruna.api.storage.services.v2.CollectionService/CreateCollection" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::CreateCollectionRequest>
                    for CreateCollectionSvc<T> {
                        type Response = super::CreateCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCollectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_collection(request).await
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
                        let method = CreateCollectionSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/GetCollection" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::GetCollectionRequest>
                    for GetCollectionSvc<T> {
                        type Response = super::GetCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_collection(request).await
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
                        let method = GetCollectionSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/GetCollections" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionsSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::GetCollectionsRequest>
                    for GetCollectionsSvc<T> {
                        type Response = super::GetCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_collections(request).await
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
                        let method = GetCollectionsSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/DeleteCollection" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::DeleteCollectionRequest>
                    for DeleteCollectionSvc<T> {
                        type Response = super::DeleteCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCollectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_collection(request).await
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
                        let method = DeleteCollectionSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionName" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionNameSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::UpdateCollectionNameRequest>
                    for UpdateCollectionNameSvc<T> {
                        type Response = super::UpdateCollectionNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCollectionNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_collection_name(request).await
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
                        let method = UpdateCollectionNameSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionDescription" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionDescriptionSvc<T: CollectionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<
                        super::UpdateCollectionDescriptionRequest,
                    > for UpdateCollectionDescriptionSvc<T> {
                        type Response = super::UpdateCollectionDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateCollectionDescriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_collection_description(request).await
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
                        let method = UpdateCollectionDescriptionSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionKeyValues" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionKeyValuesSvc<T: CollectionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<
                        super::UpdateCollectionKeyValuesRequest,
                    > for UpdateCollectionKeyValuesSvc<T> {
                        type Response = super::UpdateCollectionKeyValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateCollectionKeyValuesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_collection_key_values(request).await
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
                        let method = UpdateCollectionKeyValuesSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/UpdateCollectionDataClass" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionDataClassSvc<T: CollectionService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<
                        super::UpdateCollectionDataClassRequest,
                    > for UpdateCollectionDataClassSvc<T> {
                        type Response = super::UpdateCollectionDataClassResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateCollectionDataClassRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_collection_data_class(request).await
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
                        let method = UpdateCollectionDataClassSvc(inner);
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
                "/aruna.api.storage.services.v2.CollectionService/SnapshotCollection" => {
                    #[allow(non_camel_case_types)]
                    struct SnapshotCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::SnapshotCollectionRequest>
                    for SnapshotCollectionSvc<T> {
                        type Response = super::SnapshotCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnapshotCollectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).snapshot_collection(request).await
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
                        let method = SnapshotCollectionSvc(inner);
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
    impl<T: CollectionService> Clone for CollectionServiceServer<T> {
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
    impl<T: CollectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CollectionService> tonic::server::NamedService
    for CollectionServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.CollectionService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// dataset name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// dataset specific labels / hooks
    #[prost(message, repeated, tag = "3")]
    pub key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// External relations (URLs / IDs from external sources)
    #[prost(message, repeated, tag = "4")]
    pub external_relations: ::prost::alloc::vec::Vec<
        super::super::models::v2::ExternalRelation,
    >,
    /// DataClass
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "5")]
    pub data_class: i32,
    /// Parent_id MUST be dataset
    #[prost(oneof = "create_dataset_request::Parent", tags = "6, 7")]
    pub parent: ::core::option::Option<create_dataset_request::Parent>,
}
/// Nested message and enum types in `CreateDatasetRequest`.
pub mod create_dataset_request {
    /// Parent_id MUST be dataset
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parent {
        #[prost(string, tag = "6")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag = "7")]
        CollectionId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetResponse {
    /// The new dataset_id
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Requested id
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetResponse {
    /// Overview of the requested dataset
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetsRequest {
    #[prost(string, repeated, tag = "1")]
    pub dataset_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetsResponse {
    /// List of dataset overviews
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetNameRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetNameResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetDescriptionRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetDescriptionResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetKeyValuesRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub add_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    #[prost(message, repeated, tag = "3")]
    pub remove_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetKeyValuesResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetDataClassRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "2")]
    pub data_class: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetDataClassResponse {
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotDatasetRequest {
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotDatasetResponse {
    /// This dataset will be returned via an Persistent Identifier! Updates will be impossible
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<super::super::models::v2::Dataset>,
}
/// Generated client implementations.
pub mod dataset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// DatasetService
    ///
    /// Contains all methods that get/create or update Dataset and associated resources
    #[derive(Debug, Clone)]
    pub struct DatasetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatasetServiceClient<tonic::transport::Channel> {
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
        /// CreateNewDataset
        ///
        /// Status: BETA
        ///
        /// creates a new Dataset
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDatasetResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/CreateDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "CreateDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetDataset
        ///
        /// Status: BETA
        ///
        /// Request a specific dataset by ID
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatasetResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/GetDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "GetDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetDatasets
        ///
        /// Status: BETA
        ///
        /// Queries multiple datasets by ID
        pub async fn get_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatasetsResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/GetDatasets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "GetDatasets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteDataset
        ///
        /// Status: STABLE
        ///
        /// This request deletes the dataset.
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteDatasetResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/DeleteDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "DeleteDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateDatasetName
        ///
        /// Status: BETA
        ///
        /// Updates the dataset name. Caveat! Will rename the "s3 bucket" for data proxies!
        pub async fn update_dataset_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetNameResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "UpdateDatasetName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateDatasetDescription
        ///
        /// Status: BETA
        ///
        /// Updates the dataset description.
        pub async fn update_dataset_description(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetDescriptionResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetDescription",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "UpdateDatasetDescription",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateDatasetKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the dataset key values.
        pub async fn update_dataset_key_values(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetKeyValuesResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetKeyValues",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "UpdateDatasetKeyValues",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateDatasetDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the dataset name. All (meta) data will be overwritten.
        pub async fn update_dataset_data_class(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetDataClassResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetDataClass",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "UpdateDatasetDataClass",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SnapshotDatasetRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full dataset, rendering all downstream relations immutable
        pub async fn snapshot_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SnapshotDatasetResponse>,
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
                "/aruna.api.storage.services.v2.DatasetService/SnapshotDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.DatasetService",
                        "SnapshotDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod dataset_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DatasetServiceServer.
    #[async_trait]
    pub trait DatasetService: Send + Sync + 'static {
        /// CreateNewDataset
        ///
        /// Status: BETA
        ///
        /// creates a new Dataset
        async fn create_dataset(
            &self,
            request: tonic::Request<super::CreateDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateDatasetResponse>,
            tonic::Status,
        >;
        /// GetDataset
        ///
        /// Status: BETA
        ///
        /// Request a specific dataset by ID
        async fn get_dataset(
            &self,
            request: tonic::Request<super::GetDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatasetResponse>,
            tonic::Status,
        >;
        /// GetDatasets
        ///
        /// Status: BETA
        ///
        /// Queries multiple datasets by ID
        async fn get_datasets(
            &self,
            request: tonic::Request<super::GetDatasetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatasetsResponse>,
            tonic::Status,
        >;
        /// DeleteDataset
        ///
        /// Status: STABLE
        ///
        /// This request deletes the dataset.
        async fn delete_dataset(
            &self,
            request: tonic::Request<super::DeleteDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteDatasetResponse>,
            tonic::Status,
        >;
        /// UpdateDatasetName
        ///
        /// Status: BETA
        ///
        /// Updates the dataset name. Caveat! Will rename the "s3 bucket" for data proxies!
        async fn update_dataset_name(
            &self,
            request: tonic::Request<super::UpdateDatasetNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetNameResponse>,
            tonic::Status,
        >;
        /// UpdateDatasetDescription
        ///
        /// Status: BETA
        ///
        /// Updates the dataset description.
        async fn update_dataset_description(
            &self,
            request: tonic::Request<super::UpdateDatasetDescriptionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetDescriptionResponse>,
            tonic::Status,
        >;
        /// UpdateDatasetKeyValues
        ///
        /// Status: BETA
        ///
        /// Updates the dataset key values.
        async fn update_dataset_key_values(
            &self,
            request: tonic::Request<super::UpdateDatasetKeyValuesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetKeyValuesResponse>,
            tonic::Status,
        >;
        /// UpdateDatasetDataClass
        ///
        /// Status: BETA
        ///
        /// Updates the dataset name. All (meta) data will be overwritten.
        async fn update_dataset_data_class(
            &self,
            request: tonic::Request<super::UpdateDatasetDataClassRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateDatasetDataClassResponse>,
            tonic::Status,
        >;
        /// SnapshotDatasetRequest
        ///
        /// Status: BETA
        ///
        /// Archives the full dataset, rendering all downstream relations immutable
        async fn snapshot_dataset(
            &self,
            request: tonic::Request<super::SnapshotDatasetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SnapshotDatasetResponse>,
            tonic::Status,
        >;
    }
    /// DatasetService
    ///
    /// Contains all methods that get/create or update Dataset and associated resources
    #[derive(Debug)]
    pub struct DatasetServiceServer<T: DatasetService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v2.DatasetService/CreateDataset" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_dataset(request).await
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
                        let method = CreateDatasetSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/GetDataset" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_dataset(request).await };
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
                        let method = GetDatasetSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/GetDatasets" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatasetsSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::GetDatasetsRequest>
                    for GetDatasetsSvc<T> {
                        type Response = super::GetDatasetsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatasetsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_datasets(request).await
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
                        let method = GetDatasetsSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/DeleteDataset" => {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_dataset(request).await
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
                        let method = DeleteDatasetSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetName" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetNameSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::UpdateDatasetNameRequest>
                    for UpdateDatasetNameSvc<T> {
                        type Response = super::UpdateDatasetNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatasetNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_dataset_name(request).await
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
                        let method = UpdateDatasetNameSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetDescription" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetDescriptionSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::UpdateDatasetDescriptionRequest>
                    for UpdateDatasetDescriptionSvc<T> {
                        type Response = super::UpdateDatasetDescriptionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateDatasetDescriptionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_dataset_description(request).await
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
                        let method = UpdateDatasetDescriptionSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetKeyValues" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetKeyValuesSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::UpdateDatasetKeyValuesRequest>
                    for UpdateDatasetKeyValuesSvc<T> {
                        type Response = super::UpdateDatasetKeyValuesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatasetKeyValuesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_dataset_key_values(request).await
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
                        let method = UpdateDatasetKeyValuesSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/UpdateDatasetDataClass" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatasetDataClassSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::UpdateDatasetDataClassRequest>
                    for UpdateDatasetDataClassSvc<T> {
                        type Response = super::UpdateDatasetDataClassResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatasetDataClassRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_dataset_data_class(request).await
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
                        let method = UpdateDatasetDataClassSvc(inner);
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
                "/aruna.api.storage.services.v2.DatasetService/SnapshotDataset" => {
                    #[allow(non_camel_case_types)]
                    struct SnapshotDatasetSvc<T: DatasetService>(pub Arc<T>);
                    impl<
                        T: DatasetService,
                    > tonic::server::UnaryService<super::SnapshotDatasetRequest>
                    for SnapshotDatasetSvc<T> {
                        type Response = super::SnapshotDatasetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SnapshotDatasetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).snapshot_dataset(request).await
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
                        let method = SnapshotDatasetSvc(inner);
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
    impl<T: DatasetService> Clone for DatasetServiceServer<T> {
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
    impl<T: DatasetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatasetService> tonic::server::NamedService for DatasetServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.DatasetService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceTemplateRequest {
    /// The user id of the template owner (will be automatically added as "admin" to each associated workspace)
    #[prost(string, tag = "1")]
    pub owner_id: ::prost::alloc::string::String,
    /// Short prefix for each workspace_project (will be prepended by a random id) example: test-i12ashj9g2
    #[prost(string, tag = "2")]
    pub prefix: ::prost::alloc::string::String,
    /// The name of the workspace template
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Key values / hooks that must be added to each participant of the workspace
    #[prost(message, repeated, tag = "4")]
    pub key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceTemplateResponse {
    #[prost(string, tag = "1")]
    pub template_name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub workspace_template: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceResponse {
    #[prost(string, tag = "1")]
    pub workspace_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub secret_key: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkspaceRequest {
    #[prost(string, tag = "1")]
    pub workspace_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkspaceResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimWorkspaceRequest {
    /// This can only be called by an registered user,
    /// that is in possesion of the workspace_id and workspace token
    /// It will remove the service account and claim all references "as" the user.
    #[prost(string, tag = "1")]
    pub workspace_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimWorkspaceResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveWorkspaceDataRequest {
    /// This will update all objects as "non_workspace" and move the root to another project
    #[prost(string, tag = "1")]
    pub workspace_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveWorkspaceDataResponse {}
/// Generated client implementations.
pub mod workspace_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage "special" anonymous collections / workspaces
    #[derive(Debug, Clone)]
    pub struct WorkspaceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkspaceServiceClient<tonic::transport::Channel> {
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
    impl<T> WorkspaceServiceClient<T>
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
        ) -> WorkspaceServiceClient<InterceptedService<T, F>>
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
            WorkspaceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreatesNewWorkspaceTemplate
        ///
        /// Status: ALPHA
        ///
        /// This will create a new template for workspaces (admin only)
        pub async fn create_workspace_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkspaceTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkspaceTemplateResponse>,
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
                "/aruna.api.storage.services.v2.WorkspaceService/CreateWorkspaceTemplate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.WorkspaceService",
                        "CreateWorkspaceTemplate",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateWorkspace
        ///
        /// Status: ALPHA
        ///
        /// A new request to create a personal anonymous workspace
        pub async fn create_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkspaceResponse>,
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
                "/aruna.api.storage.services.v2.WorkspaceService/CreateWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.WorkspaceService",
                        "CreateWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteWorkspace
        ///
        /// Status: ALPHA
        ///
        /// Delete a workspace
        pub async fn delete_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteWorkspaceResponse>,
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
                "/aruna.api.storage.services.v2.WorkspaceService/DeleteWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.WorkspaceService",
                        "DeleteWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteWorkspace
        ///
        /// Status: ALPHA
        ///
        /// Claims an anonymous workspace, and transfers the owner to a regular user account.
        pub async fn claim_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ClaimWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClaimWorkspaceResponse>,
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
                "/aruna.api.storage.services.v2.WorkspaceService/ClaimWorkspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.WorkspaceService",
                        "ClaimWorkspace",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// MoveWorkspaceData
        ///
        /// Status: ALPHA
        ///
        /// Claims an anonymous workspace
        pub async fn move_workspace_data(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveWorkspaceDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveWorkspaceDataResponse>,
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
                "/aruna.api.storage.services.v2.WorkspaceService/MoveWorkspaceData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.WorkspaceService",
                        "MoveWorkspaceData",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod workspace_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with WorkspaceServiceServer.
    #[async_trait]
    pub trait WorkspaceService: Send + Sync + 'static {
        /// CreatesNewWorkspaceTemplate
        ///
        /// Status: ALPHA
        ///
        /// This will create a new template for workspaces (admin only)
        async fn create_workspace_template(
            &self,
            request: tonic::Request<super::CreateWorkspaceTemplateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkspaceTemplateResponse>,
            tonic::Status,
        >;
        /// CreateWorkspace
        ///
        /// Status: ALPHA
        ///
        /// A new request to create a personal anonymous workspace
        async fn create_workspace(
            &self,
            request: tonic::Request<super::CreateWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateWorkspaceResponse>,
            tonic::Status,
        >;
        /// DeleteWorkspace
        ///
        /// Status: ALPHA
        ///
        /// Delete a workspace
        async fn delete_workspace(
            &self,
            request: tonic::Request<super::DeleteWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteWorkspaceResponse>,
            tonic::Status,
        >;
        /// DeleteWorkspace
        ///
        /// Status: ALPHA
        ///
        /// Claims an anonymous workspace, and transfers the owner to a regular user account.
        async fn claim_workspace(
            &self,
            request: tonic::Request<super::ClaimWorkspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ClaimWorkspaceResponse>,
            tonic::Status,
        >;
        /// MoveWorkspaceData
        ///
        /// Status: ALPHA
        ///
        /// Claims an anonymous workspace
        async fn move_workspace_data(
            &self,
            request: tonic::Request<super::MoveWorkspaceDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveWorkspaceDataResponse>,
            tonic::Status,
        >;
    }
    /// Service to manage "special" anonymous collections / workspaces
    #[derive(Debug)]
    pub struct WorkspaceServiceServer<T: WorkspaceService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: WorkspaceService> WorkspaceServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for WorkspaceServiceServer<T>
    where
        T: WorkspaceService,
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
                "/aruna.api.storage.services.v2.WorkspaceService/CreateWorkspaceTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkspaceTemplateSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateWorkspaceTemplateRequest>
                    for CreateWorkspaceTemplateSvc<T> {
                        type Response = super::CreateWorkspaceTemplateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateWorkspaceTemplateRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_workspace_template(request).await
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
                        let method = CreateWorkspaceTemplateSvc(inner);
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
                "/aruna.api.storage.services.v2.WorkspaceService/CreateWorkspace" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkspaceSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::CreateWorkspaceRequest>
                    for CreateWorkspaceSvc<T> {
                        type Response = super::CreateWorkspaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkspaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_workspace(request).await
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
                        let method = CreateWorkspaceSvc(inner);
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
                "/aruna.api.storage.services.v2.WorkspaceService/DeleteWorkspace" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWorkspaceSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::DeleteWorkspaceRequest>
                    for DeleteWorkspaceSvc<T> {
                        type Response = super::DeleteWorkspaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWorkspaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_workspace(request).await
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
                        let method = DeleteWorkspaceSvc(inner);
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
                "/aruna.api.storage.services.v2.WorkspaceService/ClaimWorkspace" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimWorkspaceSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::ClaimWorkspaceRequest>
                    for ClaimWorkspaceSvc<T> {
                        type Response = super::ClaimWorkspaceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClaimWorkspaceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).claim_workspace(request).await
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
                        let method = ClaimWorkspaceSvc(inner);
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
                "/aruna.api.storage.services.v2.WorkspaceService/MoveWorkspaceData" => {
                    #[allow(non_camel_case_types)]
                    struct MoveWorkspaceDataSvc<T: WorkspaceService>(pub Arc<T>);
                    impl<
                        T: WorkspaceService,
                    > tonic::server::UnaryService<super::MoveWorkspaceDataRequest>
                    for MoveWorkspaceDataSvc<T> {
                        type Response = super::MoveWorkspaceDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveWorkspaceDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).move_workspace_data(request).await
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
                        let method = MoveWorkspaceDataSvc(inner);
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
    impl<T: WorkspaceService> Clone for WorkspaceServiceServer<T> {
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
    impl<T: WorkspaceService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: WorkspaceService> tonic::server::NamedService for WorkspaceServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.WorkspaceService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    /// collection name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// collection specific labels / hooks
    #[prost(message, repeated, tag = "3")]
    pub key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// External relations (URLs / IDs from external sources)
    #[prost(message, repeated, tag = "4")]
    pub external_relations: ::prost::alloc::vec::Vec<
        super::super::models::v2::ExternalRelation,
    >,
    /// DataClass
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "5")]
    pub data_class: i32,
    #[prost(message, repeated, tag = "9")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::models::v2::Hash>,
    /// Parent can be one of all other resources
    #[prost(oneof = "create_object_request::Parent", tags = "6, 7, 8")]
    pub parent: ::core::option::Option<create_object_request::Parent>,
}
/// Nested message and enum types in `CreateObjectRequest`.
pub mod create_object_request {
    /// Parent can be one of all other resources
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parent {
        #[prost(string, tag = "6")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag = "7")]
        CollectionId(::prost::alloc::string::String),
        #[prost(string, tag = "8")]
        DatasetId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectResponse {
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlRequest {
    /// ObjectId
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// Is this a multipart upload?
    #[prost(bool, tag = "2")]
    pub multipart: bool,
    /// (optional) if multi was initialized
    #[prost(int32, tag = "3")]
    pub part_number: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlResponse {
    /// URL
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlRequest {
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlResponse {
    /// URL
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedParts {
    /// Multipart identifier
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
    /// Part number
    #[prost(int64, tag = "2")]
    pub part: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingRequest {
    /// ObjectId
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// Final content len
    #[prost(int64, tag = "2")]
    pub content_len: i64,
    /// Hash of the uploaded data - used to verify the data integrity.
    /// This supports multiple hashing algorithms.
    #[prost(message, repeated, tag = "3")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::models::v2::Hash>,
    /// If the upload was multipart, this is the list of parts that were uploaded.
    /// Should be empty if the upload was not multipart.
    /// (optional)
    #[prost(message, repeated, tag = "4")]
    pub completed_parts: ::prost::alloc::vec::Vec<CompletedParts>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingResponse {
    /// (new) Object overview
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    /// Existing object ID
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// object name
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    ///
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// key_values to add
    #[prost(message, repeated, tag = "4")]
    pub add_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// key_values to remove
    #[prost(message, repeated, tag = "5")]
    pub remove_key_values: ::prost::alloc::vec::Vec<super::super::models::v2::KeyValue>,
    /// New DataClass
    #[prost(enumeration = "super::super::models::v2::DataClass", tag = "7")]
    pub data_class: i32,
    #[prost(message, repeated, tag = "12")]
    pub hashes: ::prost::alloc::vec::Vec<super::super::models::v2::Hash>,
    /// Parent can be one of all other resources
    #[prost(oneof = "update_object_request::Parent", tags = "8, 9, 10")]
    pub parent: ::core::option::Option<update_object_request::Parent>,
}
/// Nested message and enum types in `UpdateObjectRequest`.
pub mod update_object_request {
    /// Parent can be one of all other resources
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parent {
        #[prost(string, tag = "8")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag = "9")]
        CollectionId(::prost::alloc::string::String),
        #[prost(string, tag = "10")]
        DatasetId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectResponse {
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
    #[prost(bool, tag = "2")]
    pub new_revision: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectRequest {
    /// ObjectId
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    ///
    #[prost(oneof = "clone_object_request::Parent", tags = "2, 3, 4")]
    pub parent: ::core::option::Option<clone_object_request::Parent>,
}
/// Nested message and enum types in `CloneObjectRequest`.
pub mod clone_object_request {
    ///
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Parent {
        #[prost(string, tag = "2")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        CollectionId(::prost::alloc::string::String),
        #[prost(string, tag = "4")]
        DatasetId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectResponse {
    /// This describes the new object.
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// ObjectId
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
    /// Delete including revisions
    #[prost(bool, tag = "2")]
    pub with_revisions: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRequest {
    /// Object Id
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectResponse {
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsRequest {
    /// Object ids
    #[prost(string, repeated, tag = "1")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsResponse {
    /// A List of objects
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRevisionsRequest {
    /// Object id
    #[prost(string, tag = "2")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRevisionsResponse {
    /// List of objects
    #[prost(message, repeated, tag = "1")]
    pub objects: ::prost::alloc::vec::Vec<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestObjectRevisionRequest {
    /// Object id
    #[prost(string, tag = "1")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestObjectRevisionResponse {
    /// The object with the latest revision
    #[prost(message, optional, tag = "1")]
    pub object: ::core::option::Option<super::super::models::v2::Object>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectEndpointsRequest {
    /// Collection id
    #[prost(string, tag = "1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag = "2")]
    pub object_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod object_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ObjectService
    ///
    /// Contains all methods that get/create or update Objects and associated resources
    #[derive(Debug, Clone)]
    pub struct ObjectServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectServiceClient<tonic::transport::Channel> {
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
    impl<T> ObjectServiceClient<T>
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
        ) -> ObjectServiceClient<InterceptedService<T, F>>
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
            ObjectServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateObject
        ///
        /// Status: BETA
        ///
        /// This creates a new object
        /// Initializing an object will put it in a staging area.
        pub async fn create_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateObjectResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/CreateObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "CreateObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetUploadURL
        ///
        /// Status: BETA
        ///
        /// This is a proxy method that will call the apropriate method at dataproxy level
        /// This method will return a (multi-part) url that can be used to upload a
        /// file to S3. Part is a optional query parameter that can be used to upload a
        /// part of the file / multipart upload.
        pub async fn get_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUploadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUploadUrlResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/GetUploadURL",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "GetUploadURL",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetDownloadUrl
        ///
        /// Status: BETA
        ///
        /// This is a proxy method that will call the apropriate method at dataproxy level
        /// will return a url that can be used to download a file from S3.
        pub async fn get_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDownloadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDownloadUrlResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/GetDownloadURL",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "GetDownloadURL",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FinishObjectStaging
        ///
        /// Status: BETA
        ///
        /// This method completes the staging of an object.
        pub async fn finish_object_staging(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishObjectStagingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FinishObjectStagingResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/FinishObjectStaging",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "FinishObjectStaging",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateObject
        ///
        /// Status: BETA
        ///
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateObjectResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/UpdateObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "UpdateObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CloneObject
        ///
        /// Status: BETA
        ///
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from
        /// the original object.
        pub async fn clone_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CloneObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloneObjectResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/CloneObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "CloneObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteObject
        ///
        /// Status: BETA
        ///
        /// Deletes the object with the complete revision history.
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteObjectResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/DeleteObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "DeleteObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetObject
        ///
        /// Status: BETA
        ///
        /// gets a specific Object by ID that is associated to the
        /// current collection By default only the latest revision of an object will be
        /// returned Specify a revision_number to select an older revision With the
        /// optional with_url boolean a download link can automatically be requested
        pub async fn get_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetObjectResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/GetObject",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "GetObject",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetObjects
        ///
        /// Status: BETA
        ///
        /// Get multiple objects by ID at once
        pub async fn get_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetObjectsResponse>,
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
                "/aruna.api.storage.services.v2.ObjectService/GetObjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ObjectService",
                        "GetObjects",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ObjectServiceServer.
    #[async_trait]
    pub trait ObjectService: Send + Sync + 'static {
        /// CreateObject
        ///
        /// Status: BETA
        ///
        /// This creates a new object
        /// Initializing an object will put it in a staging area.
        async fn create_object(
            &self,
            request: tonic::Request<super::CreateObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateObjectResponse>,
            tonic::Status,
        >;
        /// GetUploadURL
        ///
        /// Status: BETA
        ///
        /// This is a proxy method that will call the apropriate method at dataproxy level
        /// This method will return a (multi-part) url that can be used to upload a
        /// file to S3. Part is a optional query parameter that can be used to upload a
        /// part of the file / multipart upload.
        async fn get_upload_url(
            &self,
            request: tonic::Request<super::GetUploadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUploadUrlResponse>,
            tonic::Status,
        >;
        /// GetDownloadUrl
        ///
        /// Status: BETA
        ///
        /// This is a proxy method that will call the apropriate method at dataproxy level
        /// will return a url that can be used to download a file from S3.
        async fn get_download_url(
            &self,
            request: tonic::Request<super::GetDownloadUrlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDownloadUrlResponse>,
            tonic::Status,
        >;
        /// FinishObjectStaging
        ///
        /// Status: BETA
        ///
        /// This method completes the staging of an object.
        async fn finish_object_staging(
            &self,
            request: tonic::Request<super::FinishObjectStagingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FinishObjectStagingResponse>,
            tonic::Status,
        >;
        /// UpdateObject
        ///
        /// Status: BETA
        ///
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        async fn update_object(
            &self,
            request: tonic::Request<super::UpdateObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateObjectResponse>,
            tonic::Status,
        >;
        /// CloneObject
        ///
        /// Status: BETA
        ///
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from
        /// the original object.
        async fn clone_object(
            &self,
            request: tonic::Request<super::CloneObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloneObjectResponse>,
            tonic::Status,
        >;
        /// DeleteObject
        ///
        /// Status: BETA
        ///
        /// Deletes the object with the complete revision history.
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteObjectResponse>,
            tonic::Status,
        >;
        /// GetObject
        ///
        /// Status: BETA
        ///
        /// gets a specific Object by ID that is associated to the
        /// current collection By default only the latest revision of an object will be
        /// returned Specify a revision_number to select an older revision With the
        /// optional with_url boolean a download link can automatically be requested
        async fn get_object(
            &self,
            request: tonic::Request<super::GetObjectRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetObjectResponse>,
            tonic::Status,
        >;
        /// GetObjects
        ///
        /// Status: BETA
        ///
        /// Get multiple objects by ID at once
        async fn get_objects(
            &self,
            request: tonic::Request<super::GetObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetObjectsResponse>,
            tonic::Status,
        >;
    }
    /// ObjectService
    ///
    /// Contains all methods that get/create or update Objects and associated resources
    #[derive(Debug)]
    pub struct ObjectServiceServer<T: ObjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectService> ObjectServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectServiceServer<T>
    where
        T: ObjectService,
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
                "/aruna.api.storage.services.v2.ObjectService/CreateObject" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::CreateObjectRequest>
                    for CreateObjectSvc<T> {
                        type Response = super::CreateObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_object(request).await
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
                        let method = CreateObjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/GetUploadURL" => {
                    #[allow(non_camel_case_types)]
                    struct GetUploadURLSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetUploadUrlRequest>
                    for GetUploadURLSvc<T> {
                        type Response = super::GetUploadUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUploadUrlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_upload_url(request).await
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
                        let method = GetUploadURLSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/GetDownloadURL" => {
                    #[allow(non_camel_case_types)]
                    struct GetDownloadURLSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetDownloadUrlRequest>
                    for GetDownloadURLSvc<T> {
                        type Response = super::GetDownloadUrlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDownloadUrlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_download_url(request).await
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
                        let method = GetDownloadURLSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/FinishObjectStaging" => {
                    #[allow(non_camel_case_types)]
                    struct FinishObjectStagingSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::FinishObjectStagingRequest>
                    for FinishObjectStagingSvc<T> {
                        type Response = super::FinishObjectStagingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinishObjectStagingRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).finish_object_staging(request).await
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
                        let method = FinishObjectStagingSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/UpdateObject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::UpdateObjectRequest>
                    for UpdateObjectSvc<T> {
                        type Response = super::UpdateObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_object(request).await
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
                        let method = UpdateObjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/CloneObject" => {
                    #[allow(non_camel_case_types)]
                    struct CloneObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::CloneObjectRequest>
                    for CloneObjectSvc<T> {
                        type Response = super::CloneObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CloneObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).clone_object(request).await
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
                        let method = CloneObjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/DeleteObject" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_object(request).await
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
                        let method = DeleteObjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/GetObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectRequest>
                    for GetObjectSvc<T> {
                        type Response = super::GetObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_object(request).await };
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
                        let method = GetObjectSvc(inner);
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
                "/aruna.api.storage.services.v2.ObjectService/GetObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectsSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectsRequest>
                    for GetObjectsSvc<T> {
                        type Response = super::GetObjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_objects(request).await };
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
                        let method = GetObjectsSvc(inner);
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
    impl<T: ObjectService> Clone for ObjectServiceServer<T> {
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
    impl<T: ObjectService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectService> tonic::server::NamedService for ObjectServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.ObjectService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    /// user_displayname
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Mail address (optional)
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// Project hint description string (optional)
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    /// Created user id
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenRequest {
    /// Token name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Personal or resource specific
    #[prost(message, optional, tag = "2")]
    pub permission: ::core::option::Option<super::super::models::v2::Permission>,
    /// Token expiry
    #[prost(message, optional, tag = "3")]
    pub expires_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenResponse {
    /// This contains only the token description
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::models::v2::Token>,
    /// This is the actual secret token
    /// Attention, this can not be recreated and needs to be stored securely
    /// New tokens will always contain a new secret
    #[prost(string, tag = "2")]
    pub token_secret: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenRequest {
    /// The token id
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenResponse {
    /// List of API tokens
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::models::v2::Token>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensResponse {
    /// List of API tokens with redacted actual token
    #[prost(message, repeated, tag = "1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v2::Token>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenRequest {
    /// The token_id
    #[prost(string, tag = "1")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokensRequest {
    /// This request invalidates all tokens of a specific user
    /// usually the user_id is specified via the provided oidc or aruna token
    /// This user_id can be used by admins to invalidate all tokens of a specific
    /// user
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokensResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    /// Optional user_id
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    /// User info
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRedactedRequest {
    /// Optional user_id
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRedactedResponse {
    /// User info
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDisplayNameRequest {
    /// New display name
    #[prost(string, tag = "1")]
    pub new_display_name: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDisplayNameResponse {
    /// Updated user info
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateUserRequest {
    /// User to activate
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateUserResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotActivatedUsersRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotActivatedUsersResponse {
    /// List of users that are not yet activated
    #[prost(message, repeated, tag = "1")]
    pub users: ::prost::alloc::vec::Vec<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllUsersRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllUsersResponse {
    #[prost(message, repeated, tag = "1")]
    pub user: ::prost::alloc::vec::Vec<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserRequest {
    /// User to activate
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeactivateUserResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserEmailRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// If new email is empty == unsubscribe
    #[prost(string, tag = "2")]
    pub new_email: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserEmailResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::super::models::v2::User>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeUserAccountRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from_user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeUserAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<super::super::models::v2::User>,
}
/// Generated client implementations.
pub mod user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// UserService
    ///
    /// Contains all methods that get/create or update Users and associated resource
    /// including all autorization methods
    #[derive(Debug, Clone)]
    pub struct UserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserServiceClient<tonic::transport::Channel> {
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
    impl<T> UserServiceClient<T>
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
        ) -> UserServiceClient<InterceptedService<T, F>>
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
            UserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// RegisterUser
        ///
        /// Status: BETA
        ///
        /// This request should be called when a new user logs in for the first time
        pub async fn register_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterUserResponse>,
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
                "/aruna.api.storage.services.v2.UserService/RegisterUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "RegisterUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeActivateUser
        ///
        /// Status: BETA
        ///
        /// This deactivates a specific user (Admin request)
        pub async fn deactivate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserResponse>,
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
                "/aruna.api.storage.services.v2.UserService/DeactivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "DeactivateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ActivateUser
        ///
        /// Status: BETA
        ///
        /// This activates a specific user (Admin request)
        pub async fn activate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateUserResponse>,
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
                "/aruna.api.storage.services.v2.UserService/ActivateUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "ActivateUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateAPIToken
        ///
        /// Status: BETA
        ///
        /// Creates an API token to authenticate
        pub async fn create_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiTokenResponse>,
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
                "/aruna.api.storage.services.v2.UserService/CreateAPIToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "CreateAPIToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetAPIToken
        ///
        /// Status: BETA
        ///
        /// Returns one API token by id
        pub async fn get_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApiTokenResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetAPIToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetAPIToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetAPITokens
        ///
        /// Status: STABLE
        ///
        /// Returns a list of API tokens for a specific user
        pub async fn get_api_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApiTokensResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetAPITokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetAPITokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteAPIToken
        ///
        /// Status: STABLE
        ///
        /// Deletes the specified API Token
        pub async fn delete_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApiTokenResponse>,
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
                "/aruna.api.storage.services.v2.UserService/DeleteAPIToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "DeleteAPIToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteAPITokens
        ///
        /// Status: BETA
        ///
        /// Deletes the specified API Token
        pub async fn delete_api_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApiTokensResponse>,
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
                "/aruna.api.storage.services.v2.UserService/DeleteAPITokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "DeleteAPITokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetUserRequest
        ///
        /// Status: STABLE
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetUserRequestRedacted
        ///
        /// Status: STABLE
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        /// Redacts personal information like name or email
        pub async fn get_user_redacted(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRedactedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserRedactedResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetUserRedacted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetUserRedacted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateUserDisplayName
        ///
        /// Status: STABLE
        ///
        /// Updates the Displayname for the user (Personal only)
        pub async fn update_user_display_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserDisplayNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserDisplayNameResponse>,
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
                "/aruna.api.storage.services.v2.UserService/UpdateUserDisplayName",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "UpdateUserDisplayName",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateUserDisplayName
        ///
        /// Status: ALPHA
        ///
        /// Updates the email for the user (Personal only)
        pub async fn update_user_email(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserEmailResponse>,
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
                "/aruna.api.storage.services.v2.UserService/UpdateUserEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "UpdateUserEmail",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetNotActivatedUsers
        ///
        /// Status: STABLE
        ///
        /// Get all not activated users (Admin only)
        pub async fn get_not_activated_users(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotActivatedUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNotActivatedUsersResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetNotActivatedUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetNotActivatedUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetAllUsers
        ///
        /// Status: ALPHA
        ///
        /// Get all users including permissions (Admin only)
        pub async fn get_all_users(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAllUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAllUsersResponse>,
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
                "/aruna.api.storage.services.v2.UserService/GetAllUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "GetAllUsers",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// MergeUserAccount
        ///
        /// Status: ALPHA
        ///
        /// Get all users including permissions (Admin only)
        pub async fn merge_user_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeUserAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MergeUserAccountResponse>,
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
                "/aruna.api.storage.services.v2.UserService/MergeUserAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.UserService",
                        "MergeUserAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        /// RegisterUser
        ///
        /// Status: BETA
        ///
        /// This request should be called when a new user logs in for the first time
        async fn register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterUserResponse>,
            tonic::Status,
        >;
        /// DeActivateUser
        ///
        /// Status: BETA
        ///
        /// This deactivates a specific user (Admin request)
        async fn deactivate_user(
            &self,
            request: tonic::Request<super::DeactivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeactivateUserResponse>,
            tonic::Status,
        >;
        /// ActivateUser
        ///
        /// Status: BETA
        ///
        /// This activates a specific user (Admin request)
        async fn activate_user(
            &self,
            request: tonic::Request<super::ActivateUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ActivateUserResponse>,
            tonic::Status,
        >;
        /// CreateAPIToken
        ///
        /// Status: BETA
        ///
        /// Creates an API token to authenticate
        async fn create_api_token(
            &self,
            request: tonic::Request<super::CreateApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateApiTokenResponse>,
            tonic::Status,
        >;
        /// GetAPIToken
        ///
        /// Status: BETA
        ///
        /// Returns one API token by id
        async fn get_api_token(
            &self,
            request: tonic::Request<super::GetApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApiTokenResponse>,
            tonic::Status,
        >;
        /// GetAPITokens
        ///
        /// Status: STABLE
        ///
        /// Returns a list of API tokens for a specific user
        async fn get_api_tokens(
            &self,
            request: tonic::Request<super::GetApiTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetApiTokensResponse>,
            tonic::Status,
        >;
        /// DeleteAPIToken
        ///
        /// Status: STABLE
        ///
        /// Deletes the specified API Token
        async fn delete_api_token(
            &self,
            request: tonic::Request<super::DeleteApiTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApiTokenResponse>,
            tonic::Status,
        >;
        /// DeleteAPITokens
        ///
        /// Status: BETA
        ///
        /// Deletes the specified API Token
        async fn delete_api_tokens(
            &self,
            request: tonic::Request<super::DeleteApiTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteApiTokensResponse>,
            tonic::Status,
        >;
        /// GetUserRequest
        ///
        /// Status: STABLE
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserRequest>,
        ) -> std::result::Result<tonic::Response<super::GetUserResponse>, tonic::Status>;
        /// GetUserRequestRedacted
        ///
        /// Status: STABLE
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        /// Redacts personal information like name or email
        async fn get_user_redacted(
            &self,
            request: tonic::Request<super::GetUserRedactedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserRedactedResponse>,
            tonic::Status,
        >;
        /// UpdateUserDisplayName
        ///
        /// Status: STABLE
        ///
        /// Updates the Displayname for the user (Personal only)
        async fn update_user_display_name(
            &self,
            request: tonic::Request<super::UpdateUserDisplayNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserDisplayNameResponse>,
            tonic::Status,
        >;
        /// UpdateUserDisplayName
        ///
        /// Status: ALPHA
        ///
        /// Updates the email for the user (Personal only)
        async fn update_user_email(
            &self,
            request: tonic::Request<super::UpdateUserEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateUserEmailResponse>,
            tonic::Status,
        >;
        /// GetNotActivatedUsers
        ///
        /// Status: STABLE
        ///
        /// Get all not activated users (Admin only)
        async fn get_not_activated_users(
            &self,
            request: tonic::Request<super::GetNotActivatedUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetNotActivatedUsersResponse>,
            tonic::Status,
        >;
        /// GetAllUsers
        ///
        /// Status: ALPHA
        ///
        /// Get all users including permissions (Admin only)
        async fn get_all_users(
            &self,
            request: tonic::Request<super::GetAllUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAllUsersResponse>,
            tonic::Status,
        >;
        /// MergeUserAccount
        ///
        /// Status: ALPHA
        ///
        /// Get all users including permissions (Admin only)
        async fn merge_user_account(
            &self,
            request: tonic::Request<super::MergeUserAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MergeUserAccountResponse>,
            tonic::Status,
        >;
    }
    /// UserService
    ///
    /// Contains all methods that get/create or update Users and associated resource
    /// including all autorization methods
    #[derive(Debug)]
    pub struct UserServiceServer<T: UserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserService> UserServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
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
                "/aruna.api.storage.services.v2.UserService/RegisterUser" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::RegisterUserRequest>
                    for RegisterUserSvc<T> {
                        type Response = super::RegisterUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_user(request).await
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
                        let method = RegisterUserSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/DeactivateUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeactivateUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeactivateUserRequest>
                    for DeactivateUserSvc<T> {
                        type Response = super::DeactivateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeactivateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).deactivate_user(request).await
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
                        let method = DeactivateUserSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/ActivateUser" => {
                    #[allow(non_camel_case_types)]
                    struct ActivateUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ActivateUserRequest>
                    for ActivateUserSvc<T> {
                        type Response = super::ActivateUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ActivateUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).activate_user(request).await
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
                        let method = ActivateUserSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/CreateAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAPITokenSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_api_token(request).await
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
                        let method = CreateAPITokenSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetAPITokenSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_api_token(request).await
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
                        let method = GetAPITokenSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetAPITokens" => {
                    #[allow(non_camel_case_types)]
                    struct GetAPITokensSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetApiTokensRequest>
                    for GetAPITokensSvc<T> {
                        type Response = super::GetApiTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApiTokensRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_api_tokens(request).await
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
                        let method = GetAPITokensSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/DeleteAPIToken" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPITokenSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_api_token(request).await
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
                        let method = DeleteAPITokenSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/DeleteAPITokens" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAPITokensSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::DeleteApiTokensRequest>
                    for DeleteAPITokensSvc<T> {
                        type Response = super::DeleteApiTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApiTokensRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_api_tokens(request).await
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
                        let method = DeleteAPITokensSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetUserRequest>
                    for GetUserSvc<T> {
                        type Response = super::GetUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_user(request).await };
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
                        let method = GetUserSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetUserRedacted" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserRedactedSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetUserRedactedRequest>
                    for GetUserRedactedSvc<T> {
                        type Response = super::GetUserRedactedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserRedactedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_user_redacted(request).await
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
                        let method = GetUserRedactedSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/UpdateUserDisplayName" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserDisplayNameSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::UpdateUserDisplayNameRequest>
                    for UpdateUserDisplayNameSvc<T> {
                        type Response = super::UpdateUserDisplayNameResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserDisplayNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_user_display_name(request).await
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
                        let method = UpdateUserDisplayNameSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/UpdateUserEmail" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUserEmailSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::UpdateUserEmailRequest>
                    for UpdateUserEmailSvc<T> {
                        type Response = super::UpdateUserEmailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUserEmailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_user_email(request).await
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
                        let method = UpdateUserEmailSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetNotActivatedUsers" => {
                    #[allow(non_camel_case_types)]
                    struct GetNotActivatedUsersSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetNotActivatedUsersRequest>
                    for GetNotActivatedUsersSvc<T> {
                        type Response = super::GetNotActivatedUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNotActivatedUsersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_not_activated_users(request).await
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
                        let method = GetNotActivatedUsersSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/GetAllUsers" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllUsersSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetAllUsersRequest>
                    for GetAllUsersSvc<T> {
                        type Response = super::GetAllUsersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAllUsersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_all_users(request).await
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
                        let method = GetAllUsersSvc(inner);
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
                "/aruna.api.storage.services.v2.UserService/MergeUserAccount" => {
                    #[allow(non_camel_case_types)]
                    struct MergeUserAccountSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::MergeUserAccountRequest>
                    for MergeUserAccountSvc<T> {
                        type Response = super::MergeUserAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeUserAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).merge_user_account(request).await
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
                        let method = MergeUserAccountSvc(inner);
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
    impl<T: UserService> Clone for UserServiceServer<T> {
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
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.UserService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPermission {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::PermissionLevel", tag = "3")]
    pub permission_level: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAuthorization {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub user_permission: ::prost::alloc::vec::Vec<UserPermission>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizationRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// Can also include "deny"
    #[prost(enumeration = "super::super::models::v2::PermissionLevel", tag = "3")]
    pub permission_level: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAuthorizationResponse {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user_name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::PermissionLevel", tag = "4")]
    pub permission_level: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizationsRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub recursive: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub authorizations: ::prost::alloc::vec::Vec<ResourceAuthorization>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizationRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAuthorizationResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizationsRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::models::v2::PermissionLevel", tag = "3")]
    pub permission_level: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthorizationsResponse {
    #[prost(message, optional, tag = "1")]
    pub user_permission: ::core::option::Option<UserPermission>,
}
/// Generated client implementations.
pub mod authorization_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// AuthorizationService
    ///
    /// Contains all methods to edit and change user authorization
    #[derive(Debug, Clone)]
    pub struct AuthorizationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthorizationServiceClient<tonic::transport::Channel> {
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
    impl<T> AuthorizationServiceClient<T>
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
        ) -> AuthorizationServiceClient<InterceptedService<T, F>>
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
            AuthorizationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        pub async fn create_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAuthorizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAuthorizationResponse>,
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
                "/aruna.api.storage.services.v2.AuthorizationService/CreateAuthorization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.AuthorizationService",
                        "CreateAuthorization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetAuthorization
        ///
        /// Status: BETA
        ///
        /// This gets resource specific user authorizations
        pub async fn get_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAuthorizationsResponse>,
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
                "/aruna.api.storage.services.v2.AuthorizationService/GetAuthorizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.AuthorizationService",
                        "GetAuthorizations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        pub async fn delete_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAuthorizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAuthorizationResponse>,
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
                "/aruna.api.storage.services.v2.AuthorizationService/DeleteAuthorization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.AuthorizationService",
                        "DeleteAuthorization",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        pub async fn update_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAuthorizationsResponse>,
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
                "/aruna.api.storage.services.v2.AuthorizationService/UpdateAuthorizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.AuthorizationService",
                        "UpdateAuthorizations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod authorization_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AuthorizationServiceServer.
    #[async_trait]
    pub trait AuthorizationService: Send + Sync + 'static {
        /// CreateAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        async fn create_authorization(
            &self,
            request: tonic::Request<super::CreateAuthorizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateAuthorizationResponse>,
            tonic::Status,
        >;
        /// GetAuthorization
        ///
        /// Status: BETA
        ///
        /// This gets resource specific user authorizations
        async fn get_authorizations(
            &self,
            request: tonic::Request<super::GetAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAuthorizationsResponse>,
            tonic::Status,
        >;
        /// DeleteAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        async fn delete_authorization(
            &self,
            request: tonic::Request<super::DeleteAuthorizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteAuthorizationResponse>,
            tonic::Status,
        >;
        /// UpdateAuthorization
        ///
        /// Status: BETA
        ///
        /// This creates a user-specific attribute that handles permission for a
        /// specific resource
        async fn update_authorizations(
            &self,
            request: tonic::Request<super::UpdateAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAuthorizationsResponse>,
            tonic::Status,
        >;
    }
    /// AuthorizationService
    ///
    /// Contains all methods to edit and change user authorization
    #[derive(Debug)]
    pub struct AuthorizationServiceServer<T: AuthorizationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthorizationService> AuthorizationServiceServer<T> {
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
    for AuthorizationServiceServer<T>
    where
        T: AuthorizationService,
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
                "/aruna.api.storage.services.v2.AuthorizationService/CreateAuthorization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAuthorizationSvc<T: AuthorizationService>(pub Arc<T>);
                    impl<
                        T: AuthorizationService,
                    > tonic::server::UnaryService<super::CreateAuthorizationRequest>
                    for CreateAuthorizationSvc<T> {
                        type Response = super::CreateAuthorizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAuthorizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_authorization(request).await
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
                        let method = CreateAuthorizationSvc(inner);
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
                "/aruna.api.storage.services.v2.AuthorizationService/GetAuthorizations" => {
                    #[allow(non_camel_case_types)]
                    struct GetAuthorizationsSvc<T: AuthorizationService>(pub Arc<T>);
                    impl<
                        T: AuthorizationService,
                    > tonic::server::UnaryService<super::GetAuthorizationsRequest>
                    for GetAuthorizationsSvc<T> {
                        type Response = super::GetAuthorizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAuthorizationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_authorizations(request).await
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
                        let method = GetAuthorizationsSvc(inner);
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
                "/aruna.api.storage.services.v2.AuthorizationService/DeleteAuthorization" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAuthorizationSvc<T: AuthorizationService>(pub Arc<T>);
                    impl<
                        T: AuthorizationService,
                    > tonic::server::UnaryService<super::DeleteAuthorizationRequest>
                    for DeleteAuthorizationSvc<T> {
                        type Response = super::DeleteAuthorizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAuthorizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_authorization(request).await
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
                        let method = DeleteAuthorizationSvc(inner);
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
                "/aruna.api.storage.services.v2.AuthorizationService/UpdateAuthorizations" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAuthorizationsSvc<T: AuthorizationService>(pub Arc<T>);
                    impl<
                        T: AuthorizationService,
                    > tonic::server::UnaryService<super::UpdateAuthorizationsRequest>
                    for UpdateAuthorizationsSvc<T> {
                        type Response = super::UpdateAuthorizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAuthorizationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_authorizations(request).await
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
                        let method = UpdateAuthorizationsSvc(inner);
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
    impl<T: AuthorizationService> Clone for AuthorizationServiceServer<T> {
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
    impl<T: AuthorizationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthorizationService> tonic::server::NamedService
    for AuthorizationServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.AuthorizationService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointRequest {
    /// Endpoint name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Endpoint type
    #[prost(enumeration = "super::super::models::v2::EndpointVariant", tag = "2")]
    pub ep_variant: i32,
    /// Is this endpoint public
    #[prost(bool, tag = "3")]
    pub is_public: bool,
    /// required public_key
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    /// List of EndpointHostConfigs
    #[prost(message, repeated, tag = "5")]
    pub host_configs: ::prost::alloc::vec::Vec<
        super::super::models::v2::EndpointHostConfig,
    >,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointResponse {
    /// Overview of the created endpoint
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<super::super::models::v2::Endpoint>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullSyncEndpointRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullSyncEndpointResponse {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    /// Either endpoint_name or id
    #[prost(oneof = "get_endpoint_request::Endpoint", tags = "1, 2")]
    pub endpoint: ::core::option::Option<get_endpoint_request::Endpoint>,
}
/// Nested message and enum types in `GetEndpointRequest`.
pub mod get_endpoint_request {
    /// Either endpoint_name or id
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Endpoint {
        /// The name of the endpoint
        #[prost(string, tag = "1")]
        EndpointName(::prost::alloc::string::String),
        /// Id of the endpoint
        #[prost(string, tag = "2")]
        EndpointId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointResponse {
    /// Overview of the requested endpoint
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<super::super::models::v2::Endpoint>,
}
/// Requests all endpoints
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsResponse {
    /// List of endpoints
    #[prost(message, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<super::super::models::v2::Endpoint>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    /// Endpoint_id to delete
    #[prost(string, tag = "1")]
    pub endpoint_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultEndpointRequest {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultEndpointResponse {
    /// Default endpoint of the server instance
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<super::super::models::v2::Endpoint>,
}
/// Generated client implementations.
pub mod endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// EndpointService
    ///
    /// Contains all methods that get/create or update Endpoint and associated resources
    #[derive(Debug, Clone)]
    pub struct EndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EndpointServiceClient<tonic::transport::Channel> {
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
    impl<T> EndpointServiceClient<T>
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
        ) -> EndpointServiceClient<InterceptedService<T, F>>
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
            EndpointServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateEndpoint
        ///
        /// Status: BETA
        ///
        /// Registers a new Endpoint (Aruna DataProxy) to the server
        /// Needs admin permissions
        pub async fn create_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEndpointResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/CreateEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "CreateEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FullSyncEndpoint
        ///
        /// Status: BETA
        ///
        /// Requests a full sync of all endpoint related data
        pub async fn full_sync_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::FullSyncEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FullSyncEndpointResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/FullSyncEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "FullSyncEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEndpoint
        ///
        /// Status: BETA
        ///
        /// Gets an specific endpoint by ID or Name
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndpointResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/GetEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "GetEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetEndpoints
        ///
        /// Status: BETA
        ///
        /// Gets all available endpoints
        pub async fn get_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndpointsResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/GetEndpoints",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "GetEndpoints",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteEndpoint
        ///
        /// Status: BETA
        ///
        /// Deletes a specific endpoint by id
        /// This needs admin permissions
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEndpointResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/DeleteEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "DeleteEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetDefaultEndpoint
        ///
        /// Status: BETA
        ///
        /// This request returns the default endpoint for the current aruna_server
        /// It may produce different results depending on the used server
        pub async fn get_default_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultEndpointResponse>,
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
                "/aruna.api.storage.services.v2.EndpointService/GetDefaultEndpoint",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.EndpointService",
                        "GetDefaultEndpoint",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod endpoint_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EndpointServiceServer.
    #[async_trait]
    pub trait EndpointService: Send + Sync + 'static {
        /// CreateEndpoint
        ///
        /// Status: BETA
        ///
        /// Registers a new Endpoint (Aruna DataProxy) to the server
        /// Needs admin permissions
        async fn create_endpoint(
            &self,
            request: tonic::Request<super::CreateEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateEndpointResponse>,
            tonic::Status,
        >;
        /// FullSyncEndpoint
        ///
        /// Status: BETA
        ///
        /// Requests a full sync of all endpoint related data
        async fn full_sync_endpoint(
            &self,
            request: tonic::Request<super::FullSyncEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FullSyncEndpointResponse>,
            tonic::Status,
        >;
        /// GetEndpoint
        ///
        /// Status: BETA
        ///
        /// Gets an specific endpoint by ID or Name
        async fn get_endpoint(
            &self,
            request: tonic::Request<super::GetEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndpointResponse>,
            tonic::Status,
        >;
        /// GetEndpoints
        ///
        /// Status: BETA
        ///
        /// Gets all available endpoints
        async fn get_endpoints(
            &self,
            request: tonic::Request<super::GetEndpointsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetEndpointsResponse>,
            tonic::Status,
        >;
        /// DeleteEndpoint
        ///
        /// Status: BETA
        ///
        /// Deletes a specific endpoint by id
        /// This needs admin permissions
        async fn delete_endpoint(
            &self,
            request: tonic::Request<super::DeleteEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteEndpointResponse>,
            tonic::Status,
        >;
        /// GetDefaultEndpoint
        ///
        /// Status: BETA
        ///
        /// This request returns the default endpoint for the current aruna_server
        /// It may produce different results depending on the used server
        async fn get_default_endpoint(
            &self,
            request: tonic::Request<super::GetDefaultEndpointRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDefaultEndpointResponse>,
            tonic::Status,
        >;
    }
    /// EndpointService
    ///
    /// Contains all methods that get/create or update Endpoint and associated resources
    #[derive(Debug)]
    pub struct EndpointServiceServer<T: EndpointService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EndpointService> EndpointServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EndpointServiceServer<T>
    where
        T: EndpointService,
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
                "/aruna.api.storage.services.v2.EndpointService/CreateEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::CreateEndpointRequest>
                    for CreateEndpointSvc<T> {
                        type Response = super::CreateEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEndpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_endpoint(request).await
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
                        let method = CreateEndpointSvc(inner);
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
                "/aruna.api.storage.services.v2.EndpointService/FullSyncEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct FullSyncEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::FullSyncEndpointRequest>
                    for FullSyncEndpointSvc<T> {
                        type Response = super::FullSyncEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FullSyncEndpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).full_sync_endpoint(request).await
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
                        let method = FullSyncEndpointSvc(inner);
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
                "/aruna.api.storage.services.v2.EndpointService/GetEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct GetEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::GetEndpointRequest>
                    for GetEndpointSvc<T> {
                        type Response = super::GetEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEndpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_endpoint(request).await
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
                        let method = GetEndpointSvc(inner);
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
                "/aruna.api.storage.services.v2.EndpointService/GetEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct GetEndpointsSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::GetEndpointsRequest>
                    for GetEndpointsSvc<T> {
                        type Response = super::GetEndpointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEndpointsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_endpoints(request).await
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
                        let method = GetEndpointsSvc(inner);
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
                "/aruna.api.storage.services.v2.EndpointService/DeleteEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::DeleteEndpointRequest>
                    for DeleteEndpointSvc<T> {
                        type Response = super::DeleteEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEndpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_endpoint(request).await
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
                        let method = DeleteEndpointSvc(inner);
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
                "/aruna.api.storage.services.v2.EndpointService/GetDefaultEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct GetDefaultEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::GetDefaultEndpointRequest>
                    for GetDefaultEndpointSvc<T> {
                        type Response = super::GetDefaultEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDefaultEndpointRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_default_endpoint(request).await
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
                        let method = GetDefaultEndpointSvc(inner);
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
    impl<T: EndpointService> Clone for EndpointServiceServer<T> {
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
    impl<T: EndpointService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EndpointService> tonic::server::NamedService for EndpointServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.EndpointService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResourcesRequest {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub offset: i64,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResourcesResponse {
    /// Json list for each found resource
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<super::super::models::v2::GenericResource>,
    /// How many results are expected to be found ?
    #[prost(int64, tag = "2")]
    pub estimated_total: i64,
    /// The last index returned
    #[prost(int64, tag = "3")]
    pub last_index: i64,
}
/// Generated client implementations.
pub mod search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchServiceClient<tonic::transport::Channel> {
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
    impl<T> SearchServiceClient<T>
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
        ) -> SearchServiceClient<InterceptedService<T, F>>
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
            SearchServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// SearchResources
        ///
        /// Status: BETA
        ///
        /// Searches the index for applicable resources (only public + private can be searched)
        pub async fn search_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchResourcesResponse>,
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
                "/aruna.api.storage.services.v2.SearchService/SearchResources",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.SearchService",
                        "SearchResources",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod search_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SearchServiceServer.
    #[async_trait]
    pub trait SearchService: Send + Sync + 'static {
        /// SearchResources
        ///
        /// Status: BETA
        ///
        /// Searches the index for applicable resources (only public + private can be searched)
        async fn search_resources(
            &self,
            request: tonic::Request<super::SearchResourcesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchResourcesResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SearchServiceServer<T: SearchService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SearchService> SearchServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchServiceServer<T>
    where
        T: SearchService,
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
                "/aruna.api.storage.services.v2.SearchService/SearchResources" => {
                    #[allow(non_camel_case_types)]
                    struct SearchResourcesSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::SearchResourcesRequest>
                    for SearchResourcesSvc<T> {
                        type Response = super::SearchResourcesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchResourcesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).search_resources(request).await
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
                        let method = SearchResourcesSvc(inner);
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
    impl<T: SearchService> Clone for SearchServiceServer<T> {
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
    impl<T: SearchService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SearchService> tonic::server::NamedService for SearchServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.SearchService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyRelationsRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub add_relations: ::prost::alloc::vec::Vec<super::super::models::v2::Relation>,
    #[prost(message, repeated, tag = "3")]
    pub remove_relations: ::prost::alloc::vec::Vec<super::super::models::v2::Relation>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyRelationsResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHierachyRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetRelations {
    #[prost(string, tag = "1")]
    pub origin: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub object_children: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionRelations {
    #[prost(string, tag = "1")]
    pub origin: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub dataset_children: ::prost::alloc::vec::Vec<DatasetRelations>,
    #[prost(string, repeated, tag = "3")]
    pub object_children: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectRelations {
    #[prost(string, tag = "1")]
    pub origin: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub collection_children: ::prost::alloc::vec::Vec<CollectionRelations>,
    #[prost(message, repeated, tag = "3")]
    pub dataset_children: ::prost::alloc::vec::Vec<DatasetRelations>,
    #[prost(string, repeated, tag = "4")]
    pub object_children: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHierachyResponse {
    #[prost(oneof = "get_hierachy_response::Graph", tags = "1, 2, 3")]
    pub graph: ::core::option::Option<get_hierachy_response::Graph>,
}
/// Nested message and enum types in `GetHierachyResponse`.
pub mod get_hierachy_response {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Graph {
        #[prost(message, tag = "1")]
        Project(super::ProjectRelations),
        #[prost(message, tag = "2")]
        Collection(super::CollectionRelations),
        #[prost(message, tag = "3")]
        Dataset(super::DatasetRelations),
    }
}
/// Generated client implementations.
pub mod relations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// RelationsService
    ///
    /// Contains all methods to edit and change resource relations
    #[derive(Debug, Clone)]
    pub struct RelationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RelationsServiceClient<tonic::transport::Channel> {
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
    impl<T> RelationsServiceClient<T>
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
        ) -> RelationsServiceClient<InterceptedService<T, F>>
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
            RelationsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// ModifyRelation
        ///
        /// Status: BETA
        ///
        /// Modifys all relations to / from a resource
        pub async fn modify_relations(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyRelationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyRelationsResponse>,
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
                "/aruna.api.storage.services.v2.RelationsService/ModifyRelations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.RelationsService",
                        "ModifyRelations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetHierachy
        ///
        /// Status: BETA
        ///
        /// Gets all downstream hierarchy relations from a resource
        pub async fn get_hierachy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHierachyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHierachyResponse>,
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
                "/aruna.api.storage.services.v2.RelationsService/GetHierachy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.RelationsService",
                        "GetHierachy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod relations_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RelationsServiceServer.
    #[async_trait]
    pub trait RelationsService: Send + Sync + 'static {
        /// ModifyRelation
        ///
        /// Status: BETA
        ///
        /// Modifys all relations to / from a resource
        async fn modify_relations(
            &self,
            request: tonic::Request<super::ModifyRelationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ModifyRelationsResponse>,
            tonic::Status,
        >;
        /// GetHierachy
        ///
        /// Status: BETA
        ///
        /// Gets all downstream hierarchy relations from a resource
        async fn get_hierachy(
            &self,
            request: tonic::Request<super::GetHierachyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetHierachyResponse>,
            tonic::Status,
        >;
    }
    /// RelationsService
    ///
    /// Contains all methods to edit and change resource relations
    #[derive(Debug)]
    pub struct RelationsServiceServer<T: RelationsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RelationsService> RelationsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RelationsServiceServer<T>
    where
        T: RelationsService,
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
                "/aruna.api.storage.services.v2.RelationsService/ModifyRelations" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyRelationsSvc<T: RelationsService>(pub Arc<T>);
                    impl<
                        T: RelationsService,
                    > tonic::server::UnaryService<super::ModifyRelationsRequest>
                    for ModifyRelationsSvc<T> {
                        type Response = super::ModifyRelationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyRelationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).modify_relations(request).await
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
                        let method = ModifyRelationsSvc(inner);
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
                "/aruna.api.storage.services.v2.RelationsService/GetHierachy" => {
                    #[allow(non_camel_case_types)]
                    struct GetHierachySvc<T: RelationsService>(pub Arc<T>);
                    impl<
                        T: RelationsService,
                    > tonic::server::UnaryService<super::GetHierachyRequest>
                    for GetHierachySvc<T> {
                        type Response = super::GetHierachyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetHierachyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_hierachy(request).await
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
                        let method = GetHierachySvc(inner);
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
    impl<T: RelationsService> Clone for RelationsServiceServer<T> {
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
    impl<T: RelationsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RelationsService> tonic::server::NamedService for RelationsServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.RelationsService";
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub permission: ::core::option::Option<super::super::models::v2::Permission>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub permission: ::core::option::Option<super::super::models::v2::Permission>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub service_account: ::core::option::Option<ServiceAccount>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountTokenRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
    /// Identify the associated project (should always be provided)
    #[prost(message, optional, tag = "2")]
    pub permission: ::core::option::Option<super::super::models::v2::Permission>,
    /// (optional) Token name
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// (optional) Token expiry
    #[prost(message, optional, tag = "4")]
    pub expires_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountTokenResponse {
    /// This contains only the token description
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::models::v2::Token>,
    /// This is the actual secret API token
    #[prost(string, tag = "2")]
    pub token_secret: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetServiceAccountPermissionRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub permission: ::core::option::Option<super::super::models::v2::Permission>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetServiceAccountPermissionResponse {
    #[prost(message, optional, tag = "1")]
    pub service_account: ::core::option::Option<ServiceAccount>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountTokenRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountTokenResponse {
    /// This contains only the token description
    #[prost(message, optional, tag = "1")]
    pub token: ::core::option::Option<super::super::models::v2::Token>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountTokensRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountTokensResponse {
    /// This contains only the token description
    #[prost(message, repeated, tag = "1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v2::Token>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountTokenRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountTokenResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountTokensRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountTokensResponse {}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountRequest {
    #[prost(string, tag = "1")]
    pub svc_account_id: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountResponse {}
/// Generated client implementations.
pub mod service_account_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ServiceAccountService
    ///
    /// Service that contains all methods for service_accounts, these are Accounts that are
    /// project specific (or global) and can be used for automation.
    /// Service account users will always contain (bot) behind their name
    #[derive(Debug, Clone)]
    pub struct ServiceAccountServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceAccountServiceClient<tonic::transport::Channel> {
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
    impl<T> ServiceAccountServiceClient<T>
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
        ) -> ServiceAccountServiceClient<InterceptedService<T, F>>
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
            ServiceAccountServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateServiceAccount
        ///
        /// Status: BETA
        ///
        /// Creates a service account for a given project
        /// If the service account has permissions for the global Admin project
        /// it will be a global service account that can interact with any resource
        pub async fn create_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateServiceAccountResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/CreateServiceAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "CreateServiceAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// Creates a token for a service account
        /// Each service account can only have one permission -> The token will have the same permission as the
        /// service account
        pub async fn create_service_account_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateServiceAccountTokenResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/CreateServiceAccountToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "CreateServiceAccountToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SetServiceAccountPermission
        ///
        /// Status: BETA
        ///
        /// Overwrites the project specific permissions for a service account
        pub async fn set_service_account_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::SetServiceAccountPermissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetServiceAccountPermissionResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/SetServiceAccountPermission",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "SetServiceAccountPermission",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// This requests the overall information about a specifc service account token (by id)
        /// it will not contain the token itself.
        pub async fn get_service_account_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetServiceAccountTokenResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/GetServiceAccountToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "GetServiceAccountToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetServiceAccountTokens
        ///
        /// Status: BETA
        ///
        /// This requests the overall information about all service account tokens
        /// it will not contain the token itself.
        pub async fn get_service_account_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceAccountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetServiceAccountTokensResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/GetServiceAccountTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "GetServiceAccountTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// Deletes one service account token by ID
        pub async fn delete_service_account_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountTokenResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccountToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "DeleteServiceAccountToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteServiceAccountTokens
        ///
        /// Status: BETA
        ///
        /// Deletes all service account tokens
        pub async fn delete_service_account_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceAccountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountTokensResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccountTokens",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "DeleteServiceAccountTokens",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DeleteServiceAccount
        ///
        /// Status: BETA
        ///
        /// Deletes a service account (by id)
        pub async fn delete_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountResponse>,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "aruna.api.storage.services.v2.ServiceAccountService",
                        "DeleteServiceAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod service_account_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ServiceAccountServiceServer.
    #[async_trait]
    pub trait ServiceAccountService: Send + Sync + 'static {
        /// CreateServiceAccount
        ///
        /// Status: BETA
        ///
        /// Creates a service account for a given project
        /// If the service account has permissions for the global Admin project
        /// it will be a global service account that can interact with any resource
        async fn create_service_account(
            &self,
            request: tonic::Request<super::CreateServiceAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateServiceAccountResponse>,
            tonic::Status,
        >;
        /// CreateServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// Creates a token for a service account
        /// Each service account can only have one permission -> The token will have the same permission as the
        /// service account
        async fn create_service_account_token(
            &self,
            request: tonic::Request<super::CreateServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateServiceAccountTokenResponse>,
            tonic::Status,
        >;
        /// SetServiceAccountPermission
        ///
        /// Status: BETA
        ///
        /// Overwrites the project specific permissions for a service account
        async fn set_service_account_permission(
            &self,
            request: tonic::Request<super::SetServiceAccountPermissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetServiceAccountPermissionResponse>,
            tonic::Status,
        >;
        /// GetServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// This requests the overall information about a specifc service account token (by id)
        /// it will not contain the token itself.
        async fn get_service_account_token(
            &self,
            request: tonic::Request<super::GetServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetServiceAccountTokenResponse>,
            tonic::Status,
        >;
        /// GetServiceAccountTokens
        ///
        /// Status: BETA
        ///
        /// This requests the overall information about all service account tokens
        /// it will not contain the token itself.
        async fn get_service_account_tokens(
            &self,
            request: tonic::Request<super::GetServiceAccountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetServiceAccountTokensResponse>,
            tonic::Status,
        >;
        /// DeleteServiceAccountToken
        ///
        /// Status: BETA
        ///
        /// Deletes one service account token by ID
        async fn delete_service_account_token(
            &self,
            request: tonic::Request<super::DeleteServiceAccountTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountTokenResponse>,
            tonic::Status,
        >;
        /// DeleteServiceAccountTokens
        ///
        /// Status: BETA
        ///
        /// Deletes all service account tokens
        async fn delete_service_account_tokens(
            &self,
            request: tonic::Request<super::DeleteServiceAccountTokensRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountTokensResponse>,
            tonic::Status,
        >;
        /// DeleteServiceAccount
        ///
        /// Status: BETA
        ///
        /// Deletes a service account (by id)
        async fn delete_service_account(
            &self,
            request: tonic::Request<super::DeleteServiceAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteServiceAccountResponse>,
            tonic::Status,
        >;
    }
    /// ServiceAccountService
    ///
    /// Service that contains all methods for service_accounts, these are Accounts that are
    /// project specific (or global) and can be used for automation.
    /// Service account users will always contain (bot) behind their name
    #[derive(Debug)]
    pub struct ServiceAccountServiceServer<T: ServiceAccountService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServiceAccountService> ServiceAccountServiceServer<T> {
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
    for ServiceAccountServiceServer<T>
    where
        T: ServiceAccountService,
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
                "/aruna.api.storage.services.v2.ServiceAccountService/CreateServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceAccountSvc<T: ServiceAccountService>(pub Arc<T>);
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<super::CreateServiceAccountRequest>
                    for CreateServiceAccountSvc<T> {
                        type Response = super::CreateServiceAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_service_account(request).await
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
                        let method = CreateServiceAccountSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/CreateServiceAccountToken" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceAccountTokenSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<
                        super::CreateServiceAccountTokenRequest,
                    > for CreateServiceAccountTokenSvc<T> {
                        type Response = super::CreateServiceAccountTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateServiceAccountTokenRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_service_account_token(request).await
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
                        let method = CreateServiceAccountTokenSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/SetServiceAccountPermission" => {
                    #[allow(non_camel_case_types)]
                    struct SetServiceAccountPermissionSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<
                        super::SetServiceAccountPermissionRequest,
                    > for SetServiceAccountPermissionSvc<T> {
                        type Response = super::SetServiceAccountPermissionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::SetServiceAccountPermissionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_service_account_permission(request).await
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
                        let method = SetServiceAccountPermissionSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/GetServiceAccountToken" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceAccountTokenSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<super::GetServiceAccountTokenRequest>
                    for GetServiceAccountTokenSvc<T> {
                        type Response = super::GetServiceAccountTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceAccountTokenRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_service_account_token(request).await
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
                        let method = GetServiceAccountTokenSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/GetServiceAccountTokens" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceAccountTokensSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<super::GetServiceAccountTokensRequest>
                    for GetServiceAccountTokensSvc<T> {
                        type Response = super::GetServiceAccountTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetServiceAccountTokensRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_service_account_tokens(request).await
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
                        let method = GetServiceAccountTokensSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccountToken" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceAccountTokenSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<
                        super::DeleteServiceAccountTokenRequest,
                    > for DeleteServiceAccountTokenSvc<T> {
                        type Response = super::DeleteServiceAccountTokenResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteServiceAccountTokenRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_service_account_token(request).await
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
                        let method = DeleteServiceAccountTokenSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccountTokens" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceAccountTokensSvc<T: ServiceAccountService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<
                        super::DeleteServiceAccountTokensRequest,
                    > for DeleteServiceAccountTokensSvc<T> {
                        type Response = super::DeleteServiceAccountTokensResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteServiceAccountTokensRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_service_account_tokens(request).await
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
                        let method = DeleteServiceAccountTokensSvc(inner);
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
                "/aruna.api.storage.services.v2.ServiceAccountService/DeleteServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceAccountSvc<T: ServiceAccountService>(pub Arc<T>);
                    impl<
                        T: ServiceAccountService,
                    > tonic::server::UnaryService<super::DeleteServiceAccountRequest>
                    for DeleteServiceAccountSvc<T> {
                        type Response = super::DeleteServiceAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_service_account(request).await
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
                        let method = DeleteServiceAccountSvc(inner);
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
    impl<T: ServiceAccountService> Clone for ServiceAccountServiceServer<T> {
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
    impl<T: ServiceAccountService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceAccountService> tonic::server::NamedService
    for ServiceAccountServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v2.ServiceAccountService";
    }
}
