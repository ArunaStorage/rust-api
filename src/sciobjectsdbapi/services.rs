// These messages are used inside the DatasetAPI

/// Dataset related Models
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Name of the dataset
    #[prost(string, tag = "1")]
    pub dataset_name: ::prost::alloc::string::String,
    ///Datatype of the dataset, e.g. json, gbff, fasta
    #[prost(string, tag = "2")]
    pub datatype: ::prost::alloc::string::String,
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
    pub dataset_versions: ::prost::alloc::vec::Vec<super::models::DatasetVersionEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseDatasetVersionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<super::models::Version>,
    /// Additional metadata for the dataset version
    #[prost(map = "string, message", tag = "4")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    /// Message reference for the metadata
    #[prost(map = "string, string", tag = "5")]
    pub additional_metadata_message_ref:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Message reference for the metadata of the objects associated with this DatasetVersion
    #[prost(map = "string, string", tag = "6")]
    pub additional_object_metadata_message_ref:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub object_group_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupList {
    #[prost(message, repeated, tag = "1")]
    pub object_groups: ::prost::alloc::vec::Vec<super::models::DatasetObjectGroup>,
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
        ) -> Result<tonic::Response<super::super::models::DatasetEntry>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::models::DatasetEntry>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::models::DatasetEntry>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::models::DatasetVersionEntry>, tonic::Status>
        {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUploadLinkResponse {
    #[prost(string, tag = "1")]
    pub upload_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object: ::core::option::Option<super::models::DatasetObjectEntry>,
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
    pub object_heritage_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub objects: ::prost::alloc::vec::Vec<CreateObjectRequest>,
    #[prost(map = "string, message", tag = "4")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    #[prost(string, tag = "6")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectRequest {
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filetype: ::prost::alloc::string::String,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag = "4")]
    pub origin: ::core::option::Option<super::models::Origin>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag = "5")]
    pub content_len: i64,
    #[prost(message, repeated, tag = "6")]
    pub labels: ::prost::alloc::vec::Vec<super::models::Label>,
    /// Additional metadata of the object#
    #[prost(map = "string, message", tag = "7")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    #[prost(message, repeated, tag = "8")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
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
        ) -> Result<tonic::Response<super::super::models::DatasetObjectGroup>, tonic::Status>
        {
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
        ) -> Result<tonic::Response<super::super::models::DatasetObjectGroup>, tonic::Status>
        {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<super::models::Metadata>,
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
pub struct ProjectEntryList {
    #[prost(message, repeated, tag = "1")]
    pub projects: ::prost::alloc::vec::Vec<super::models::ProjectEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetList {
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<super::models::DatasetEntry>,
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
        ) -> Result<tonic::Response<super::super::models::ProjectEntry>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::super::models::ProjectEntry>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::ProjectEntryList>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status> {
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
        ) -> Result<tonic::Response<super::CreateUploadLinkResponse>, tonic::Status> {
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
