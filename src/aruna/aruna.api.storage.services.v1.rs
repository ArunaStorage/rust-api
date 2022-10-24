// Models
// These are the models for the above described requests and responses.
// gRPC best practises advice each Request and Response message in a RPC to be
// called {rpc_name}Request and {rpc_name}Response.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Url {
    /// URL response
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageObject {
    /// Filename
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
    /// File description
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Collection Id
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// Content length
    #[prost(int64, tag="4")]
    pub content_len: i64,
    /// Source of the object (e.g. wikipedia)
    #[prost(message, optional, tag="5")]
    pub source: ::core::option::Option<super::super::models::v1::Source>,
    /// Dataclass public / private
    #[prost(enumeration="super::super::models::v1::DataClass", tag="6")]
    pub dataclass: i32,
    /// List of label key-values
    #[prost(message, repeated, tag="7")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// List of hook key-values
    #[prost(message, repeated, tag="8")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNewObjectRequest {
    /// This describes the object to be initialized.
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<StageObject>,
    /// Collection id of the collection to which the object will be added.
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// (optional) Used to specify a preferred endpoint by id
    /// this can be used to specify which endpoint this object should use
    /// only needed if it is not the default endpoint for the current server
    /// instance
    #[prost(string, tag="4")]
    pub preferred_endpoint_id: ::prost::alloc::string::String,
    /// Should the object be uploaded via multipart?
    #[prost(bool, tag="5")]
    pub multipart: bool,
    /// Is specification ?
    /// Should this object contain a specification for the collection ?
    #[prost(bool, tag="6")]
    pub is_specification: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNewObjectResponse {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Upload ID, a ID used to identify the upload / staging object
    #[prost(string, tag="2")]
    pub upload_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlRequest {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Upload ID, a ID used to identify the upload / staging object
    #[prost(string, tag="2")]
    pub upload_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// Is this a multipart upload?
    #[prost(bool, tag="4")]
    pub multipart: bool,
    /// (optional) if multi was initialized
    #[prost(int32, tag="5")]
    pub part_number: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUploadUrlResponse {
    /// URL
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletedParts {
    /// Multipart identifier
    #[prost(string, tag="1")]
    pub etag: ::prost::alloc::string::String,
    /// Part number
    #[prost(int64, tag="2")]
    pub part: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadUrlResponse {
    /// URL
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadLinksBatchRequest {
    /// CollectionID
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// ObjectIds
    #[prost(string, repeated, tag="2")]
    pub objects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDownloadLinksBatchResponse {
    /// List of URLs
    #[prost(message, repeated, tag="1")]
    pub urls: ::prost::alloc::vec::Vec<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinksStreamRequest {
    /// CollectionID
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// ObjectIds
    #[prost(string, repeated, tag="2")]
    pub objects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDownloadLinksStreamResponse {
    #[prost(message, optional, tag="1")]
    pub url: ::core::option::Option<Url>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingRequest {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Always the S3 upload_id
    #[prost(string, tag="2")]
    pub upload_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// Hash of the uploaded data - used to verify the data integrity.
    /// This supports multiple hashing algorithms.
    #[prost(message, optional, tag="4")]
    pub hash: ::core::option::Option<super::super::models::v1::Hash>,
    /// If the staging object had no uploads
    /// Use this argument to skip the finish upload request
    #[prost(bool, tag="5")]
    pub no_upload: bool,
    /// If the upload was multipart, this is the list of parts that were uploaded.
    /// Should be empty if the upload was not multipart.
    /// (optional)
    #[prost(message, repeated, tag="6")]
    pub completed_parts: ::prost::alloc::vec::Vec<CompletedParts>,
    /// Should the object be auto-updated in the collection?
    /// default: false
    #[prost(bool, tag="7")]
    pub auto_update: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinishObjectStagingResponse {
    /// (new) Object overview
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectRequest {
    /// Existing object ID
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// collection ID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// New object data
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<StageObject>,
    /// Should new data be uploaded ?
    #[prost(bool, tag="4")]
    pub reupload: bool,
    /// If this is an reupload a preferred endpoint
    /// can be specified by id
    #[prost(string, tag="5")]
    pub preferred_endpoint_id: ::prost::alloc::string::String,
    /// Should a multipart upload be used?
    #[prost(bool, tag="6")]
    pub multi_part: bool,
    /// Is specification ?
    /// Should this object contain a specification for the collection ?
    #[prost(bool, tag="7")]
    pub is_specification: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectResponse {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Staging ID, a generic ID when multipart is not enabled, otherwise the
    /// multipart upload ID.
    #[prost(string, tag="2")]
    pub staging_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectReferenceRequest {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// OwnerCollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// BorrowerCollectionID
    #[prost(string, tag="3")]
    pub target_collection_id: ::prost::alloc::string::String,
    /// Should the other collection have permissions to edit the ressource
    #[prost(bool, tag="4")]
    pub writeable: bool,
    /// Should the borrowed ressource be automatically updated ?
    #[prost(bool, tag="5")]
    pub auto_update: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectReferenceResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectRequest {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// From CollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Target CollectionID
    #[prost(string, tag="3")]
    pub target_collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneObjectResponse {
    /// This describes the new object.
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// ObjectId
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// CollectionID
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Delete including revisions
    #[prost(bool, tag="3")]
    pub with_revisions: bool,
    /// Force delete including revisions
    #[prost(bool, tag="4")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectWithUrl {
    /// Description of a specified object
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
    /// This is a associated download URL
    /// Will be empty if request does not contain the associated with_url flag
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectByIdRequest {
    /// Collection Id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object Id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    /// With URL: Include URL in response ?
    #[prost(bool, tag="4")]
    pub with_url: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectByIdResponse {
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<ObjectWithUrl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Paginate the results: Default is 20
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Filter by Labels (optional) OR request a specific list of Objects
    #[prost(message, optional, tag="3")]
    pub label_id_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
    /// With URL: Include URL in response ?
    #[prost(bool, tag="4")]
    pub with_url: bool,
    /// Should this request consider older revisions of Objects ?
    #[prost(bool, tag="5")]
    pub include_history: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsResponse {
    /// A List of objects with (optional) associated URLs
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<ObjectWithUrl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRevisionsRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    /// Pagination info
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Should the response include download urls ?
    #[prost(bool, tag="4")]
    pub with_url: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRevisionsResponse {
    /// List of objects with (optional) URLs
    #[prost(message, repeated, tag="1")]
    pub objects: ::prost::alloc::vec::Vec<ObjectWithUrl>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestObjectRevisionRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestObjectRevisionResponse {
    /// The object with the latest revision
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectEndpointsRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectEndpointsResponse {
    /// List of endpoints
    #[prost(message, repeated, tag="1")]
    pub endpoints: ::prost::alloc::vec::Vec<super::super::models::v1::Endpoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLabelToObjectRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id 
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    /// List of labels that should be added to the list of labels
    #[prost(message, repeated, tag="3")]
    pub labels_to_add: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddLabelToObjectResponse {
    /// Returns the updated Object
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHooksOfObjectRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    /// This will overwrite all existing hooks
    /// Can be empty to remove all hooks
    #[prost(message, repeated, tag="3")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetHooksOfObjectResponse {
    /// Returns the updated Object
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencesRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Object id
    #[prost(string, tag="2")]
    pub object_id: ::prost::alloc::string::String,
    /// Should all revisions be included?
    #[prost(bool, tag="3")]
    pub with_revisions: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectReference {
    /// Object id
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Specific revision number
    #[prost(int64, tag="3")]
    pub revision_number: i64,
    /// Is the writeable?
    #[prost(bool, tag="4")]
    pub is_writeable: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReferencesResponse {
    /// List of object references
    #[prost(message, repeated, tag="1")]
    pub references: ::prost::alloc::vec::Vec<ObjectReference>,
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// InitializeNewObject
        ///
        /// This initializes a new object
        /// Initializing an object will put it in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        pub async fn initialize_new_object(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeNewObjectRequest>,
        ) -> Result<tonic::Response<super::InitializeNewObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/InitializeNewObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetUploadURL
        ///
        /// This method will return a (multi-part) url that can be used to upload a
        /// file to S3. Part is a optional query parameter that can be used to upload a
        /// part of the file / multipart upload.
        pub async fn get_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GetUploadUrlResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetUploadURL",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetDownloadUrl
        ///
        /// This method will return a url that can be used to download a file from S3.
        pub async fn get_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GetDownloadUrlResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadURL",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetDownloadLinksBatch
        ///
        /// This method can be used to get download urls for multiple objects.
        /// The order of the returned urls will be the same as the order of the object
        /// ids in the request.
        pub async fn get_download_links_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDownloadLinksBatchRequest>,
        ) -> Result<
            tonic::Response<super::GetDownloadLinksBatchResponse>,
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
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadLinksBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CreateDownloadLinksStream
        ///
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of Objects as a stream that would
        /// otherwise cause issues with the connection
        pub async fn create_download_links_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDownloadLinksStreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::CreateDownloadLinksStreamResponse>,
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
                "/aruna.api.storage.services.v1.ObjectService/CreateDownloadLinksStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// FinishObjectStaging
        ///
        /// This method completes the staging of an object.
        pub async fn finish_object_staging(
            &mut self,
            request: impl tonic::IntoRequest<super::FinishObjectStagingRequest>,
        ) -> Result<tonic::Response<super::FinishObjectStagingResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/FinishObjectStaging",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateObject
        ///
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        pub async fn update_object(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/UpdateObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CreateObjectReference
        ///
        /// Creates a new reference of this object in another collection
        pub async fn create_object_reference(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectReferenceRequest>,
        ) -> Result<
            tonic::Response<super::CreateObjectReferenceResponse>,
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
                "/aruna.api.storage.services.v1.ObjectService/CreateObjectReference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CloneObject
        ///
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from
        /// the original object.
        pub async fn clone_object(
            &mut self,
            request: impl tonic::IntoRequest<super::CloneObjectRequest>,
        ) -> Result<tonic::Response<super::CloneObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/CloneObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteObject
        ///
        /// Deletes the object with the complete revision history.
        /// This should be avoided if possible.
        /// This method allows the owner to cascade the deletion of all objects that
        /// were cloned from this object.
        /// -> GDPR compliant procedure.
        pub async fn delete_object(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/DeleteObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectByID
        ///
        /// gets a specific Object by ID that is associated to the
        /// current collection By default only the latest revision of an object will be
        /// returned Specify a revision_number to select an older revision With the
        /// optional with_url boolean a download link can automatically be requested
        pub async fn get_object_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectByIdResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjects
        ///
        /// GetObjects returns a (paginated) list of objects in a specific collection
        /// By default only the latest revisions of all objects will be shown
        /// This behaviour can be changed with the include_history flag
        /// With the optional with_url boolean a download link can automatically be
        /// requested for each Object This request contains a LabelOrIDQuery message,
        /// this is either a list of request ObjectIDs or a query filtered by Labels
        pub async fn get_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectRevisions
        ///
        /// This returns the full list of revisions of a specified object
        /// With the optional with_url boolean a download link can automatically be
        /// requested for each Object This is by default a paginated request
        pub async fn get_object_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetObjectRevisionsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectRevisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetLatestObjectRevision
        ///
        /// This returns the latest revision of a specific object
        /// The returned `latest` object will have a different id if the current
        /// object is not the latest revision
        pub async fn get_latest_object_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestObjectRevisionRequest>,
        ) -> Result<
            tonic::Response<super::GetLatestObjectRevisionResponse>,
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
                "/aruna.api.storage.services.v1.ObjectService/GetLatestObjectRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectEndpoints
        ///
        /// This returns a list of endpoints
        /// One endpoint will be the "default" endpoint
        pub async fn get_object_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectEndpointsRequest>,
        ) -> Result<tonic::Response<super::GetObjectEndpointsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddLabelToObject
        ///
        /// This is a specific request to add a new label
        /// to an existing object, in contrast to UpdateObject
        /// this will not create a new object in the staging area
        /// Instead it will directly add the specified label(s) to the object
        pub async fn add_label_to_object(
            &mut self,
            request: impl tonic::IntoRequest<super::AddLabelToObjectRequest>,
        ) -> Result<tonic::Response<super::AddLabelToObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/AddLabelToObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SetHooksOfObject
        ///
        /// This is a specific request to update the complete list
        /// of hooks for a specific object. This will not update the object
        /// and create a new id, instead it will overwrite all hooks of the existing
        /// object.
        pub async fn set_hooks_of_object(
            &mut self,
            request: impl tonic::IntoRequest<super::SetHooksOfObjectRequest>,
        ) -> Result<tonic::Response<super::SetHooksOfObjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/SetHooksOfObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetReferences
        ///
        /// Get a list of references for this object (optional) including all revisions
        pub async fn get_references(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReferencesRequest>,
        ) -> Result<tonic::Response<super::GetReferencesResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectService/GetReferences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ObjectServiceServer.
    #[async_trait]
    pub trait ObjectService: Send + Sync + 'static {
        /// InitializeNewObject
        ///
        /// This initializes a new object
        /// Initializing an object will put it in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        async fn initialize_new_object(
            &self,
            request: tonic::Request<super::InitializeNewObjectRequest>,
        ) -> Result<tonic::Response<super::InitializeNewObjectResponse>, tonic::Status>;
        /// GetUploadURL
        ///
        /// This method will return a (multi-part) url that can be used to upload a
        /// file to S3. Part is a optional query parameter that can be used to upload a
        /// part of the file / multipart upload.
        async fn get_upload_url(
            &self,
            request: tonic::Request<super::GetUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GetUploadUrlResponse>, tonic::Status>;
        /// GetDownloadUrl
        ///
        /// This method will return a url that can be used to download a file from S3.
        async fn get_download_url(
            &self,
            request: tonic::Request<super::GetDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GetDownloadUrlResponse>, tonic::Status>;
        /// GetDownloadLinksBatch
        ///
        /// This method can be used to get download urls for multiple objects.
        /// The order of the returned urls will be the same as the order of the object
        /// ids in the request.
        async fn get_download_links_batch(
            &self,
            request: tonic::Request<super::GetDownloadLinksBatchRequest>,
        ) -> Result<
            tonic::Response<super::GetDownloadLinksBatchResponse>,
            tonic::Status,
        >;
        ///Server streaming response type for the CreateDownloadLinksStream method.
        type CreateDownloadLinksStreamStream: futures_core::Stream<
                Item = Result<super::CreateDownloadLinksStreamResponse, tonic::Status>,
            >
            + Send
            + 'static;
        /// CreateDownloadLinksStream
        ///
        /// Creates a stream of objects and presigned links based on the provided query
        /// This can be used retrieve a large number of Objects as a stream that would
        /// otherwise cause issues with the connection
        async fn create_download_links_stream(
            &self,
            request: tonic::Request<super::CreateDownloadLinksStreamRequest>,
        ) -> Result<
            tonic::Response<Self::CreateDownloadLinksStreamStream>,
            tonic::Status,
        >;
        /// FinishObjectStaging
        ///
        /// This method completes the staging of an object.
        async fn finish_object_staging(
            &self,
            request: tonic::Request<super::FinishObjectStagingRequest>,
        ) -> Result<tonic::Response<super::FinishObjectStagingResponse>, tonic::Status>;
        /// UpdateObject
        ///
        /// Objects are immutable!
        /// Updating an object will create a new revision for the object
        /// This method will put the new revision in a staging area.
        /// Staged objects will get a separate staging id and need to be finished
        /// before they can be used.
        async fn update_object(
            &self,
            request: tonic::Request<super::UpdateObjectRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectResponse>, tonic::Status>;
        /// CreateObjectReference
        ///
        /// Creates a new reference of this object in another collection
        async fn create_object_reference(
            &self,
            request: tonic::Request<super::CreateObjectReferenceRequest>,
        ) -> Result<
            tonic::Response<super::CreateObjectReferenceResponse>,
            tonic::Status,
        >;
        /// CloneObject
        ///
        /// This method clones an object and creates a copy in the same collection.
        /// This copy has a new id and revision and will not receive any updates from
        /// the original object.
        async fn clone_object(
            &self,
            request: tonic::Request<super::CloneObjectRequest>,
        ) -> Result<tonic::Response<super::CloneObjectResponse>, tonic::Status>;
        /// DeleteObject
        ///
        /// Deletes the object with the complete revision history.
        /// This should be avoided if possible.
        /// This method allows the owner to cascade the deletion of all objects that
        /// were cloned from this object.
        /// -> GDPR compliant procedure.
        async fn delete_object(
            &self,
            request: tonic::Request<super::DeleteObjectRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectResponse>, tonic::Status>;
        /// GetObjectByID
        ///
        /// gets a specific Object by ID that is associated to the
        /// current collection By default only the latest revision of an object will be
        /// returned Specify a revision_number to select an older revision With the
        /// optional with_url boolean a download link can automatically be requested
        async fn get_object_by_id(
            &self,
            request: tonic::Request<super::GetObjectByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectByIdResponse>, tonic::Status>;
        /// GetObjects
        ///
        /// GetObjects returns a (paginated) list of objects in a specific collection
        /// By default only the latest revisions of all objects will be shown
        /// This behaviour can be changed with the include_history flag
        /// With the optional with_url boolean a download link can automatically be
        /// requested for each Object This request contains a LabelOrIDQuery message,
        /// this is either a list of request ObjectIDs or a query filtered by Labels
        async fn get_objects(
            &self,
            request: tonic::Request<super::GetObjectsRequest>,
        ) -> Result<tonic::Response<super::GetObjectsResponse>, tonic::Status>;
        /// GetObjectRevisions
        ///
        /// This returns the full list of revisions of a specified object
        /// With the optional with_url boolean a download link can automatically be
        /// requested for each Object This is by default a paginated request
        async fn get_object_revisions(
            &self,
            request: tonic::Request<super::GetObjectRevisionsRequest>,
        ) -> Result<tonic::Response<super::GetObjectRevisionsResponse>, tonic::Status>;
        /// GetLatestObjectRevision
        ///
        /// This returns the latest revision of a specific object
        /// The returned `latest` object will have a different id if the current
        /// object is not the latest revision
        async fn get_latest_object_revision(
            &self,
            request: tonic::Request<super::GetLatestObjectRevisionRequest>,
        ) -> Result<
            tonic::Response<super::GetLatestObjectRevisionResponse>,
            tonic::Status,
        >;
        /// GetObjectEndpoints
        ///
        /// This returns a list of endpoints
        /// One endpoint will be the "default" endpoint
        async fn get_object_endpoints(
            &self,
            request: tonic::Request<super::GetObjectEndpointsRequest>,
        ) -> Result<tonic::Response<super::GetObjectEndpointsResponse>, tonic::Status>;
        /// AddLabelToObject
        ///
        /// This is a specific request to add a new label
        /// to an existing object, in contrast to UpdateObject
        /// this will not create a new object in the staging area
        /// Instead it will directly add the specified label(s) to the object
        async fn add_label_to_object(
            &self,
            request: tonic::Request<super::AddLabelToObjectRequest>,
        ) -> Result<tonic::Response<super::AddLabelToObjectResponse>, tonic::Status>;
        /// SetHooksOfObject
        ///
        /// This is a specific request to update the complete list
        /// of hooks for a specific object. This will not update the object
        /// and create a new id, instead it will overwrite all hooks of the existing
        /// object.
        async fn set_hooks_of_object(
            &self,
            request: tonic::Request<super::SetHooksOfObjectRequest>,
        ) -> Result<tonic::Response<super::SetHooksOfObjectResponse>, tonic::Status>;
        /// GetReferences
        ///
        /// Get a list of references for this object (optional) including all revisions
        async fn get_references(
            &self,
            request: tonic::Request<super::GetReferencesRequest>,
        ) -> Result<tonic::Response<super::GetReferencesResponse>, tonic::Status>;
    }
    /// ObjectService
    ///
    /// Contains all methods that get/create or update Objects and associated resources
    #[derive(Debug)]
    pub struct ObjectServiceServer<T: ObjectService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.ObjectService/InitializeNewObject" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeNewObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::InitializeNewObjectRequest>
                    for InitializeNewObjectSvc<T> {
                        type Response = super::InitializeNewObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeNewObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).initialize_new_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InitializeNewObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetUploadURL" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_upload_url(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUploadURLSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadURL" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_download_url(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDownloadURLSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetDownloadLinksBatch" => {
                    #[allow(non_camel_case_types)]
                    struct GetDownloadLinksBatchSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetDownloadLinksBatchRequest>
                    for GetDownloadLinksBatchSvc<T> {
                        type Response = super::GetDownloadLinksBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDownloadLinksBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_download_links_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDownloadLinksBatchSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/CreateDownloadLinksStream" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDownloadLinksStreamSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::ServerStreamingService<
                        super::CreateDownloadLinksStreamRequest,
                    > for CreateDownloadLinksStreamSvc<T> {
                        type Response = super::CreateDownloadLinksStreamResponse;
                        type ResponseStream = T::CreateDownloadLinksStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateDownloadLinksStreamRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_download_links_stream(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateDownloadLinksStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aruna.api.storage.services.v1.ObjectService/FinishObjectStaging" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).finish_object_staging(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinishObjectStagingSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/UpdateObject" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/CreateObjectReference" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectReferenceSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::CreateObjectReferenceRequest>
                    for CreateObjectReferenceSvc<T> {
                        type Response = super::CreateObjectReferenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectReferenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_object_reference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateObjectReferenceSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/CloneObject" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clone_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CloneObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/DeleteObject" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectByIDSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectByIdRequest>
                    for GetObjectByIDSvc<T> {
                        type Response = super::GetObjectByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectByIDSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjects" => {
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
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_objects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectsSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectRevisions" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectRevisionsSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectRevisionsRequest>
                    for GetObjectRevisionsSvc<T> {
                        type Response = super::GetObjectRevisionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectRevisionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_revisions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectRevisionsSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetLatestObjectRevision" => {
                    #[allow(non_camel_case_types)]
                    struct GetLatestObjectRevisionSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetLatestObjectRevisionRequest>
                    for GetLatestObjectRevisionSvc<T> {
                        type Response = super::GetLatestObjectRevisionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetLatestObjectRevisionRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_latest_object_revision(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLatestObjectRevisionSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetObjectEndpoints" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectEndpointsSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetObjectEndpointsRequest>
                    for GetObjectEndpointsSvc<T> {
                        type Response = super::GetObjectEndpointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectEndpointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectEndpointsSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/AddLabelToObject" => {
                    #[allow(non_camel_case_types)]
                    struct AddLabelToObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::AddLabelToObjectRequest>
                    for AddLabelToObjectSvc<T> {
                        type Response = super::AddLabelToObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddLabelToObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_label_to_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddLabelToObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/SetHooksOfObject" => {
                    #[allow(non_camel_case_types)]
                    struct SetHooksOfObjectSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::SetHooksOfObjectRequest>
                    for SetHooksOfObjectSvc<T> {
                        type Response = super::SetHooksOfObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetHooksOfObjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).set_hooks_of_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetHooksOfObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectService/GetReferences" => {
                    #[allow(non_camel_case_types)]
                    struct GetReferencesSvc<T: ObjectService>(pub Arc<T>);
                    impl<
                        T: ObjectService,
                    > tonic::server::UnaryService<super::GetReferencesRequest>
                    for GetReferencesSvc<T> {
                        type Response = super::GetReferencesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReferencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_references(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReferencesSvc(inner);
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
            }
        }
    }
    impl<T: ObjectService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectService> tonic::server::NamedService for ObjectServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.ObjectService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    /// Project name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Description for the project
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectResponse {
    /// The freshly created project_id 
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectRequest {
    /// The id of the project to add the user to
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Permissions for the user
    #[prost(message, optional, tag="3")]
    pub user_permission: ::core::option::Option<super::super::models::v1::ProjectPermission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUserToProjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    /// The id of the project to get
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectResponse {
    /// Overview of the projectroject
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<super::super::models::v1::ProjectOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectsResponse {
    /// Overview of the projects
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<super::super::models::v1::ProjectOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyProjectRequest {
    /// The id of the project to destroy
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyProjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectRequest {
    /// Project id to update
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Updated name
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Update description
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectResponse {
    /// Updated project overview 
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<super::super::models::v1::ProjectOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserFromProjectRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// User that should be removed
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUserFromProjectResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPermissionsForProjectRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// User id
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserPermissionsForProjectResponse {
    /// Userpermission for a specific user
    #[prost(message, optional, tag="1")]
    pub user_permission: ::core::option::Option<super::super::models::v1::ProjectPermissionDisplayName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditUserPermissionsForProjectRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// This contains the user_id and the "new permission"
    #[prost(message, optional, tag="2")]
    pub user_permission: ::core::option::Option<super::super::models::v1::ProjectPermission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditUserPermissionsForProjectResponse {
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
        /// CreateProject
        ///
        /// Creates a new project all users and collections are bundled in a project.
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/CreateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddUserToProject
        ///
        /// Adds a new user to a given project by its id
        pub async fn add_user_to_project(
            &mut self,
            request: impl tonic::IntoRequest<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/AddUserToProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetProject
        ///
        /// Requests a project by id
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/GetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetProject
        ///
        /// Admin request to get all projects
        pub async fn get_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectsRequest>,
        ) -> Result<tonic::Response<super::GetProjectsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/GetProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DestroyProject
        ///
        /// Destroys the project and all its associated data. Must be empty
        /// (cannot contain any collections).
        pub async fn destroy_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyProjectRequest>,
        ) -> Result<tonic::Response<super::DestroyProjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/DestroyProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateProject
        ///
        /// Updates the project. All (meta) data will be overwritten.
        pub async fn update_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectRequest>,
        ) -> Result<tonic::Response<super::UpdateProjectResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ProjectService/UpdateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RemoveUserFromProject
        ///
        /// Removes a specified user from the project.
        pub async fn remove_user_from_project(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveUserFromProjectRequest>,
        ) -> Result<
            tonic::Response<super::RemoveUserFromProjectResponse>,
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
                "/aruna.api.storage.services.v1.ProjectService/RemoveUserFromProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetUserPermissionsForProject
        ///
        /// Get the user_permission of a specific user for the project.
        pub async fn get_user_permissions_for_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserPermissionsForProjectRequest>,
        ) -> Result<
            tonic::Response<super::GetUserPermissionsForProjectResponse>,
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
                "/aruna.api.storage.services.v1.ProjectService/GetUserPermissionsForProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// EditUserPermissionsForProject
        ///
        /// Modifies the user_permission of a specific user for the project.
        pub async fn edit_user_permissions_for_project(
            &mut self,
            request: impl tonic::IntoRequest<super::EditUserPermissionsForProjectRequest>,
        ) -> Result<
            tonic::Response<super::EditUserPermissionsForProjectResponse>,
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
                "/aruna.api.storage.services.v1.ProjectService/EditUserPermissionsForProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod project_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ProjectServiceServer.
    #[async_trait]
    pub trait ProjectService: Send + Sync + 'static {
        /// CreateProject
        ///
        /// Creates a new project all users and collections are bundled in a project.
        async fn create_project(
            &self,
            request: tonic::Request<super::CreateProjectRequest>,
        ) -> Result<tonic::Response<super::CreateProjectResponse>, tonic::Status>;
        /// AddUserToProject
        ///
        /// Adds a new user to a given project by its id
        async fn add_user_to_project(
            &self,
            request: tonic::Request<super::AddUserToProjectRequest>,
        ) -> Result<tonic::Response<super::AddUserToProjectResponse>, tonic::Status>;
        /// GetProject
        ///
        /// Requests a project by id
        async fn get_project(
            &self,
            request: tonic::Request<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::GetProjectResponse>, tonic::Status>;
        /// GetProject
        ///
        /// Admin request to get all projects
        async fn get_projects(
            &self,
            request: tonic::Request<super::GetProjectsRequest>,
        ) -> Result<tonic::Response<super::GetProjectsResponse>, tonic::Status>;
        /// DestroyProject
        ///
        /// Destroys the project and all its associated data. Must be empty
        /// (cannot contain any collections).
        async fn destroy_project(
            &self,
            request: tonic::Request<super::DestroyProjectRequest>,
        ) -> Result<tonic::Response<super::DestroyProjectResponse>, tonic::Status>;
        /// UpdateProject
        ///
        /// Updates the project. All (meta) data will be overwritten.
        async fn update_project(
            &self,
            request: tonic::Request<super::UpdateProjectRequest>,
        ) -> Result<tonic::Response<super::UpdateProjectResponse>, tonic::Status>;
        /// RemoveUserFromProject
        ///
        /// Removes a specified user from the project.
        async fn remove_user_from_project(
            &self,
            request: tonic::Request<super::RemoveUserFromProjectRequest>,
        ) -> Result<
            tonic::Response<super::RemoveUserFromProjectResponse>,
            tonic::Status,
        >;
        /// GetUserPermissionsForProject
        ///
        /// Get the user_permission of a specific user for the project.
        async fn get_user_permissions_for_project(
            &self,
            request: tonic::Request<super::GetUserPermissionsForProjectRequest>,
        ) -> Result<
            tonic::Response<super::GetUserPermissionsForProjectResponse>,
            tonic::Status,
        >;
        /// EditUserPermissionsForProject
        ///
        /// Modifies the user_permission of a specific user for the project.
        async fn edit_user_permissions_for_project(
            &self,
            request: tonic::Request<super::EditUserPermissionsForProjectRequest>,
        ) -> Result<
            tonic::Response<super::EditUserPermissionsForProjectResponse>,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.ProjectService/CreateProject" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_project(request).await
                            };
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
                "/aruna.api.storage.services.v1.ProjectService/AddUserToProject" => {
                    #[allow(non_camel_case_types)]
                    struct AddUserToProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::AddUserToProjectRequest>
                    for AddUserToProjectSvc<T> {
                        type Response = super::AddUserToProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddUserToProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_user_to_project(request).await
                            };
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
                "/aruna.api.storage.services.v1.ProjectService/GetProject" => {
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
                "/aruna.api.storage.services.v1.ProjectService/GetProjects" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_projects(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetProjectsSvc(inner);
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
                "/aruna.api.storage.services.v1.ProjectService/DestroyProject" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::DestroyProjectRequest>
                    for DestroyProjectSvc<T> {
                        type Response = super::DestroyProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).destroy_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DestroyProjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ProjectService/UpdateProject" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::UpdateProjectRequest>
                    for UpdateProjectSvc<T> {
                        type Response = super::UpdateProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateProjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ProjectService/RemoveUserFromProject" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveUserFromProjectSvc<T: ProjectService>(pub Arc<T>);
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<super::RemoveUserFromProjectRequest>
                    for RemoveUserFromProjectSvc<T> {
                        type Response = super::RemoveUserFromProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveUserFromProjectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_user_from_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveUserFromProjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ProjectService/GetUserPermissionsForProject" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserPermissionsForProjectSvc<T: ProjectService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<
                        super::GetUserPermissionsForProjectRequest,
                    > for GetUserPermissionsForProjectSvc<T> {
                        type Response = super::GetUserPermissionsForProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetUserPermissionsForProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_permissions_for_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserPermissionsForProjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ProjectService/EditUserPermissionsForProject" => {
                    #[allow(non_camel_case_types)]
                    struct EditUserPermissionsForProjectSvc<T: ProjectService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ProjectService,
                    > tonic::server::UnaryService<
                        super::EditUserPermissionsForProjectRequest,
                    > for EditUserPermissionsForProjectSvc<T> {
                        type Response = super::EditUserPermissionsForProjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::EditUserPermissionsForProjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).edit_user_permissions_for_project(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EditUserPermissionsForProjectSvc(inner);
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
    impl<T: ProjectService> tonic::server::NamedService for ProjectServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.ProjectService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupRequest {
    /// ObjectGroup name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Description for group
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Collection Id
    #[prost(string, tag="3")]
    pub collection_id: ::prost::alloc::string::String,
    /// This is the reference to the Objects that should be added to the group
    #[prost(string, repeated, tag="4")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This is a reference to the Objects that are associated with "meta" data
    /// about corresponding objects in the group
    #[prost(string, repeated, tag="5")]
    pub meta_object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of label key-value pairs
    #[prost(message, repeated, tag="6")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// List of hooks key-value pairs
    #[prost(message, repeated, tag="7")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateObjectGroupResponse {
    /// Overview of the new objectgroup
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroupOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupRequest {
    /// Old group id
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    /// New name
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// New description
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="4")]
    pub collection_id: ::prost::alloc::string::String,
    /// This is the reference to the Objects that should be added to the group
    #[prost(string, repeated, tag="5")]
    pub object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This is a reference to the Objects that are associated with "meta" data
    /// about corresponding objects in the group
    #[prost(string, repeated, tag="6")]
    pub meta_object_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of label key-value pairs
    #[prost(message, repeated, tag="7")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// List of hooks key-value pairs
    #[prost(message, repeated, tag="8")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateObjectGroupResponse {
    /// Overview of the updated objectgroup
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroupOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupByIdRequest {
    /// Object group id
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupByIdResponse {
    /// Overview of the objectgroup
    #[prost(message, optional, tag="1")]
    pub object_group: ::core::option::Option<super::super::models::v1::ObjectGroupOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsFromObjectRequest {
    /// Object id
    #[prost(string, tag="1")]
    pub object_id: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Page request
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsFromObjectResponse {
    /// Overviews of multiple objectgroups
    #[prost(message, optional, tag="1")]
    pub object_groups: ::core::option::Option<super::super::models::v1::ObjectGroupOverviews>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupRequest {
    /// Objectgroup id
    #[prost(string, tag="1")]
    pub group_id: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectGroupResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Paginate the results: Default is 20
    #[prost(message, optional, tag="2")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Filter by Labels (optional) OR request a specific list of ObjectGroups
    #[prost(message, optional, tag="3")]
    pub label_id_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupsResponse {
    /// Overviews of multiple objectgroups
    #[prost(message, optional, tag="1")]
    pub object_groups: ::core::option::Option<super::super::models::v1::ObjectGroupOverviews>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupHistoryRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Objectgroup id
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    /// Pagerequest
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupHistoryResponse {
    /// Overviews of multiple objectgroups
    #[prost(message, optional, tag="1")]
    pub object_groups: ::core::option::Option<super::super::models::v1::ObjectGroupOverviews>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupObjectsRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Objectgroup id
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    /// Pagerequest
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
    /// Include meta objects only
    ///
    /// Should only the "meta" objects be returned
    #[prost(bool, tag="4")]
    pub meta_only: bool,
}
/// Objectgroup objects are a combination of "object" and the boolean is_metadata
/// flag Returned as single list to allow for more precise queries
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupObject {
    /// Object
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::models::v1::Object>,
    /// Is this objet a meta object
    #[prost(bool, tag="2")]
    pub is_metadata: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectGroupObjectsResponse {
    /// List of associated object group objects
    #[prost(message, repeated, tag="1")]
    pub object_group_objects: ::prost::alloc::vec::Vec<ObjectGroupObject>,
}
/// Generated client implementations.
pub mod object_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ObjectService
    ///
    /// Contains all methods that get/create or update Objects and associated resource
    #[derive(Debug, Clone)]
    pub struct ObjectGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectGroupServiceClient<tonic::transport::Channel> {
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
    impl<T> ObjectGroupServiceClient<T>
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
        ) -> ObjectGroupServiceClient<InterceptedService<T, F>>
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
            ObjectGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateObjectGroup
        ///
        /// This creates a new ObjectGroup in the collection
        pub async fn create_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectGroupService/CreateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateObjectGroup
        ///
        /// This creates an updated ObjectGroup
        /// ObjectGroups are immutable
        /// Updating an ObjectGroup will create a new Revision of the ObjectGroup
        pub async fn update_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectGroupService/UpdateObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupById
        ///
        /// This gets a specific ObjectGroup by ID
        /// By default the latest revision is always returned, older revisions need to
        /// be specified separately
        pub async fn get_object_group_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupByIdResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupById",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupsFromObject
        ///
        /// This gets all ObjectGroups associated to a specific
        /// Object Objects can be part of multiple ObjectGroups at once
        pub async fn get_object_groups_from_object(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsFromObjectRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsFromObjectResponse>,
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupsFromObject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroups
        ///
        /// This is a request that returns a (paginated) list of
        /// ObjectGroups that contain a specific set of labels.
        pub async fn get_object_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupHistory
        ///
        /// This requests a full history with all objectgroups
        /// that are part of this objectgroups history
        pub async fn get_object_group_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupHistoryRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupHistoryResponse>,
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetObjectGroupObjects
        ///
        /// Requests a list of paginated objects associated with this
        /// specific objectgroup
        pub async fn get_object_group_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectGroupObjectsRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupObjectsResponse>,
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupObjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteObjectGroup
        ///
        /// This is a request that deletes a specified ObjectGroup
        /// This does not delete the associated Objects
        pub async fn delete_object_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.ObjectGroupService/DeleteObjectGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod object_group_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ObjectGroupServiceServer.
    #[async_trait]
    pub trait ObjectGroupService: Send + Sync + 'static {
        /// CreateObjectGroup
        ///
        /// This creates a new ObjectGroup in the collection
        async fn create_object_group(
            &self,
            request: tonic::Request<super::CreateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::CreateObjectGroupResponse>, tonic::Status>;
        /// UpdateObjectGroup
        ///
        /// This creates an updated ObjectGroup
        /// ObjectGroups are immutable
        /// Updating an ObjectGroup will create a new Revision of the ObjectGroup
        async fn update_object_group(
            &self,
            request: tonic::Request<super::UpdateObjectGroupRequest>,
        ) -> Result<tonic::Response<super::UpdateObjectGroupResponse>, tonic::Status>;
        /// GetObjectGroupById
        ///
        /// This gets a specific ObjectGroup by ID
        /// By default the latest revision is always returned, older revisions need to
        /// be specified separately
        async fn get_object_group_by_id(
            &self,
            request: tonic::Request<super::GetObjectGroupByIdRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupByIdResponse>, tonic::Status>;
        /// GetObjectGroupsFromObject
        ///
        /// This gets all ObjectGroups associated to a specific
        /// Object Objects can be part of multiple ObjectGroups at once
        async fn get_object_groups_from_object(
            &self,
            request: tonic::Request<super::GetObjectGroupsFromObjectRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupsFromObjectResponse>,
            tonic::Status,
        >;
        /// GetObjectGroups
        ///
        /// This is a request that returns a (paginated) list of
        /// ObjectGroups that contain a specific set of labels.
        async fn get_object_groups(
            &self,
            request: tonic::Request<super::GetObjectGroupsRequest>,
        ) -> Result<tonic::Response<super::GetObjectGroupsResponse>, tonic::Status>;
        /// GetObjectGroupHistory
        ///
        /// This requests a full history with all objectgroups
        /// that are part of this objectgroups history
        async fn get_object_group_history(
            &self,
            request: tonic::Request<super::GetObjectGroupHistoryRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupHistoryResponse>,
            tonic::Status,
        >;
        /// GetObjectGroupObjects
        ///
        /// Requests a list of paginated objects associated with this
        /// specific objectgroup
        async fn get_object_group_objects(
            &self,
            request: tonic::Request<super::GetObjectGroupObjectsRequest>,
        ) -> Result<
            tonic::Response<super::GetObjectGroupObjectsResponse>,
            tonic::Status,
        >;
        /// DeleteObjectGroup
        ///
        /// This is a request that deletes a specified ObjectGroup
        /// This does not delete the associated Objects
        async fn delete_object_group(
            &self,
            request: tonic::Request<super::DeleteObjectGroupRequest>,
        ) -> Result<tonic::Response<super::DeleteObjectGroupResponse>, tonic::Status>;
    }
    /// ObjectService
    ///
    /// Contains all methods that get/create or update Objects and associated resource
    #[derive(Debug)]
    pub struct ObjectGroupServiceServer<T: ObjectGroupService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectGroupService> ObjectGroupServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectGroupServiceServer<T>
    where
        T: ObjectGroupService,
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
                "/aruna.api.storage.services.v1.ObjectGroupService/CreateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::CreateObjectGroupRequest>
                    for CreateObjectGroupSvc<T> {
                        type Response = super::CreateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_object_group(request).await
                            };
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
                "/aruna.api.storage.services.v1.ObjectGroupService/UpdateObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::UpdateObjectGroupRequest>
                    for UpdateObjectGroupSvc<T> {
                        type Response = super::UpdateObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_object_group(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateObjectGroupSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupById" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupByIdSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupByIdRequest>
                    for GetObjectGroupByIdSvc<T> {
                        type Response = super::GetObjectGroupByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupByIdSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupsFromObject" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsFromObjectSvc<T: ObjectGroupService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<
                        super::GetObjectGroupsFromObjectRequest,
                    > for GetObjectGroupsFromObjectSvc<T> {
                        type Response = super::GetObjectGroupsFromObjectResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetObjectGroupsFromObjectRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups_from_object(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsFromObjectSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroups" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupsSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupsRequest>
                    for GetObjectGroupsSvc<T> {
                        type Response = super::GetObjectGroupsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_groups(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupsSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupHistory" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupHistorySvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupHistoryRequest>
                    for GetObjectGroupHistorySvc<T> {
                        type Response = super::GetObjectGroupHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupHistoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupHistorySvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/GetObjectGroupObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectGroupObjectsSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::GetObjectGroupObjectsRequest>
                    for GetObjectGroupObjectsSvc<T> {
                        type Response = super::GetObjectGroupObjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectGroupObjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_group_objects(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectGroupObjectsSvc(inner);
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
                "/aruna.api.storage.services.v1.ObjectGroupService/DeleteObjectGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteObjectGroupSvc<T: ObjectGroupService>(pub Arc<T>);
                    impl<
                        T: ObjectGroupService,
                    > tonic::server::UnaryService<super::DeleteObjectGroupRequest>
                    for DeleteObjectGroupSvc<T> {
                        type Response = super::DeleteObjectGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteObjectGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_object_group(request).await
                            };
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
    impl<T: ObjectGroupService> Clone for ObjectGroupServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectGroupService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectGroupService> tonic::server::NamedService
    for ObjectGroupServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.ObjectGroupService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEndpointRequest {
    /// Endpoint name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Endpoint type
    #[prost(enumeration="super::super::models::v1::EndpointType", tag="2")]
    pub ep_type: i32,
    /// Public hostname of the proxy
    #[prost(string, tag="3")]
    pub proxy_hostname: ::prost::alloc::string::String,
    /// Internal hostname for the proxy
    #[prost(string, tag="4")]
    pub internal_hostname: ::prost::alloc::string::String,
    /// (optional) URL to a offsite documentation 
    #[prost(string, tag="5")]
    pub documentation_path: ::prost::alloc::string::String,
    /// Is this endpoint public
    #[prost(bool, tag="6")]
    pub is_public: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEndpointResponse {
    /// Overview of the requested endpoint
    #[prost(message, optional, tag="1")]
    pub endpoint: ::core::option::Option<super::super::models::v1::Endpoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    /// Either endpoint_name or id
    #[prost(oneof="get_endpoint_request::Endpoint", tags="1, 2")]
    pub endpoint: ::core::option::Option<get_endpoint_request::Endpoint>,
}
/// Nested message and enum types in `GetEndpointRequest`.
pub mod get_endpoint_request {
    /// Either endpoint_name or id
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Endpoint {
        /// The name of the endpoint
        #[prost(string, tag="1")]
        EndpointName(::prost::alloc::string::String),
        /// Id of the endpoint
        #[prost(string, tag="2")]
        EndpointId(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointResponse {
    /// Overview of the requested endpoint
    #[prost(message, optional, tag="1")]
    pub endpoint: ::core::option::Option<super::super::models::v1::Endpoint>,
}
/// Requests all endpoints
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointsResponse {
    /// List of endpoints
    #[prost(message, repeated, tag="1")]
    pub endpoints: ::prost::alloc::vec::Vec<super::super::models::v1::Endpoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    /// Endpoint_id to delete
    #[prost(string, tag="1")]
    pub endpoint_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultEndpointRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultEndpointResponse {
    /// Default endpoint of the server instance
    #[prost(message, optional, tag="1")]
    pub endpoint: ::core::option::Option<super::super::models::v1::Endpoint>,
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// AddEndpoint
        ///
        /// Registers a new Endpoint (Aruna DataProxy) to the server
        /// Needs admin permissions
        pub async fn add_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::AddEndpointRequest>,
        ) -> Result<tonic::Response<super::AddEndpointResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.EndpointService/AddEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetEndpoint
        ///
        /// Gets an specific endpoint by ID or Name
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> Result<tonic::Response<super::GetEndpointResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.EndpointService/GetEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetEndpoints
        ///
        /// Gets all available endpoints
        pub async fn get_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointsRequest>,
        ) -> Result<tonic::Response<super::GetEndpointsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.EndpointService/GetEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteEndpoint
        ///
        /// Deletes a specific endpoint by id
        /// This needs admin permissions
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
        ) -> Result<tonic::Response<super::DeleteEndpointResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.EndpointService/DeleteEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetDefaultEndpoint
        ///
        /// This request returns the default endpoint for the current aruna_server
        /// It may produce different results depending on the used server
        pub async fn get_default_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultEndpointRequest>,
        ) -> Result<tonic::Response<super::GetDefaultEndpointResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.EndpointService/GetDefaultEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod endpoint_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with EndpointServiceServer.
    #[async_trait]
    pub trait EndpointService: Send + Sync + 'static {
        /// AddEndpoint
        ///
        /// Registers a new Endpoint (Aruna DataProxy) to the server
        /// Needs admin permissions
        async fn add_endpoint(
            &self,
            request: tonic::Request<super::AddEndpointRequest>,
        ) -> Result<tonic::Response<super::AddEndpointResponse>, tonic::Status>;
        /// GetEndpoint
        ///
        /// Gets an specific endpoint by ID or Name
        async fn get_endpoint(
            &self,
            request: tonic::Request<super::GetEndpointRequest>,
        ) -> Result<tonic::Response<super::GetEndpointResponse>, tonic::Status>;
        /// GetEndpoints
        ///
        /// Gets all available endpoints
        async fn get_endpoints(
            &self,
            request: tonic::Request<super::GetEndpointsRequest>,
        ) -> Result<tonic::Response<super::GetEndpointsResponse>, tonic::Status>;
        /// DeleteEndpoint
        ///
        /// Deletes a specific endpoint by id
        /// This needs admin permissions
        async fn delete_endpoint(
            &self,
            request: tonic::Request<super::DeleteEndpointRequest>,
        ) -> Result<tonic::Response<super::DeleteEndpointResponse>, tonic::Status>;
        /// GetDefaultEndpoint
        ///
        /// This request returns the default endpoint for the current aruna_server
        /// It may produce different results depending on the used server
        async fn get_default_endpoint(
            &self,
            request: tonic::Request<super::GetDefaultEndpointRequest>,
        ) -> Result<tonic::Response<super::GetDefaultEndpointResponse>, tonic::Status>;
    }
    /// EndpointService
    ///
    /// Contains all methods that get/create or update Endpoint and associated resources
    #[derive(Debug)]
    pub struct EndpointServiceServer<T: EndpointService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.EndpointService/AddEndpoint" => {
                    #[allow(non_camel_case_types)]
                    struct AddEndpointSvc<T: EndpointService>(pub Arc<T>);
                    impl<
                        T: EndpointService,
                    > tonic::server::UnaryService<super::AddEndpointRequest>
                    for AddEndpointSvc<T> {
                        type Response = super::AddEndpointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddEndpointRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_endpoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddEndpointSvc(inner);
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
                "/aruna.api.storage.services.v1.EndpointService/GetEndpoint" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_endpoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEndpointSvc(inner);
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
                "/aruna.api.storage.services.v1.EndpointService/GetEndpoints" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_endpoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEndpointsSvc(inner);
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
                "/aruna.api.storage.services.v1.EndpointService/DeleteEndpoint" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_endpoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteEndpointSvc(inner);
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
                "/aruna.api.storage.services.v1.EndpointService/GetDefaultEndpoint" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_default_endpoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDefaultEndpointSvc(inner);
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
            }
        }
    }
    impl<T: EndpointService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EndpointService> tonic::server::NamedService for EndpointServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.EndpointService";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpiresAt {
    /// Expiry time
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserRequest {
    /// Optional user_displayname
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterUserResponse {
    /// Created user id
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
/// Empty if token_type is personal, otherwise the id of the collection or
/// project to create the token for
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Collection id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// Token name
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// Token expiry
    #[prost(message, optional, tag="4")]
    pub expires_at: ::core::option::Option<ExpiresAt>,
    /// Token permissions
    #[prost(enumeration="super::super::models::v1::Permission", tag="5")]
    pub permission: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiTokenResponse {
    /// This contains only the token description
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<super::super::models::v1::Token>,
    /// This is the actual secret token
    /// Attention, this can not be recreated and needs to be stored securely
    /// New tokens will always contain a new secret
    #[prost(string, tag="2")]
    pub token_secret: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenRequest {
    /// The token id
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokenResponse {
    /// List of API tokens
    #[prost(message, optional, tag="1")]
    pub token: ::core::option::Option<super::super::models::v1::Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiTokensResponse {
    /// List of API tokens with redacted actual token
    #[prost(message, repeated, tag="1")]
    pub token: ::prost::alloc::vec::Vec<super::super::models::v1::Token>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenRequest {
    /// The token_id
    #[prost(string, tag="1")]
    pub token_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokenResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokensRequest {
    /// This request invalidates all tokens of a specific user
    /// usually the user_id is specified via the provided oidc or aruna token
    /// This user_id can be used by admins to invalidate all tokens of a specific
    /// user
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiTokensResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserRequest {
    /// Optional user_id
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserResponse {
    /// User info
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::models::v1::User>,
    /// User permissions per project
    #[prost(message, repeated, tag="2")]
    pub project_permissions: ::prost::alloc::vec::Vec<super::super::models::v1::ProjectPermission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDisplayNameRequest {
    /// New display name
    #[prost(string, tag="1")]
    pub new_display_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUserDisplayNameResponse {
    /// Updated user info
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<super::super::models::v1::User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsRequest {
    /// User id
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProject {
    /// Project id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Project name
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Project description
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserProjectsResponse {
    /// List of associated projects
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<UserProject>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateUserRequest {
    /// User to activate
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateUserResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotActivatedUsersRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotActivatedUsersResponse {
    /// List of users that are not yet activated
    #[prost(message, repeated, tag="1")]
    pub users: ::prost::alloc::vec::Vec<super::super::models::v1::User>,
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// RegisterUser
        ///
        /// This request should be called when a new user logs in for the first time
        pub async fn register_user(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/RegisterUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ActivateUser
        ///
        /// This activates a specific user (Admin request)
        pub async fn activate_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateUserRequest>,
        ) -> Result<tonic::Response<super::ActivateUserResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/ActivateUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CreateAPIToken
        ///
        /// Creates an API token to authenticate
        pub async fn create_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/CreateAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetAPIToken
        ///
        /// Returns one API token by id
        pub async fn get_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/GetAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetAPITokens
        ///
        /// Returns a list of API tokens for a specific user
        pub async fn get_api_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiTokensRequest>,
        ) -> Result<tonic::Response<super::GetApiTokensResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/GetAPITokens",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteAPIToken
        ///
        /// Deletes the specified API Token
        pub async fn delete_api_token(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/DeleteAPIToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteAPITokens
        ///
        /// Deletes the specified API Token
        pub async fn delete_api_tokens(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiTokensRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokensResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/DeleteAPITokens",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetUserRequest
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        pub async fn get_user(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserRequest>,
        ) -> Result<tonic::Response<super::GetUserResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/GetUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateUserDisplayName
        ///
        /// Updates the Displayname for the user (Personal only)
        pub async fn update_user_display_name(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUserDisplayNameRequest>,
        ) -> Result<
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
                "/aruna.api.storage.services.v1.UserService/UpdateUserDisplayName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetUserProjects
        ///
        /// Gets all project_ids a user is member of
        pub async fn get_user_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.UserService/GetUserProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetNotActivatedUsers
        ///
        /// Get all not activated users (Admin only)
        pub async fn get_not_activated_users(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotActivatedUsersRequest>,
        ) -> Result<
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
                "/aruna.api.storage.services.v1.UserService/GetNotActivatedUsers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        /// RegisterUser
        ///
        /// This request should be called when a new user logs in for the first time
        async fn register_user(
            &self,
            request: tonic::Request<super::RegisterUserRequest>,
        ) -> Result<tonic::Response<super::RegisterUserResponse>, tonic::Status>;
        /// ActivateUser
        ///
        /// This activates a specific user (Admin request)
        async fn activate_user(
            &self,
            request: tonic::Request<super::ActivateUserRequest>,
        ) -> Result<tonic::Response<super::ActivateUserResponse>, tonic::Status>;
        /// CreateAPIToken
        ///
        /// Creates an API token to authenticate
        async fn create_api_token(
            &self,
            request: tonic::Request<super::CreateApiTokenRequest>,
        ) -> Result<tonic::Response<super::CreateApiTokenResponse>, tonic::Status>;
        /// GetAPIToken
        ///
        /// Returns one API token by id
        async fn get_api_token(
            &self,
            request: tonic::Request<super::GetApiTokenRequest>,
        ) -> Result<tonic::Response<super::GetApiTokenResponse>, tonic::Status>;
        /// GetAPITokens
        ///
        /// Returns a list of API tokens for a specific user
        async fn get_api_tokens(
            &self,
            request: tonic::Request<super::GetApiTokensRequest>,
        ) -> Result<tonic::Response<super::GetApiTokensResponse>, tonic::Status>;
        /// DeleteAPIToken
        ///
        /// Deletes the specified API Token
        async fn delete_api_token(
            &self,
            request: tonic::Request<super::DeleteApiTokenRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokenResponse>, tonic::Status>;
        /// DeleteAPITokens
        ///
        /// Deletes the specified API Token
        async fn delete_api_tokens(
            &self,
            request: tonic::Request<super::DeleteApiTokensRequest>,
        ) -> Result<tonic::Response<super::DeleteApiTokensResponse>, tonic::Status>;
        /// GetUserRequest
        ///
        /// This is a request that returns the user information of the
        /// current user or if invoked by an admin from another user
        async fn get_user(
            &self,
            request: tonic::Request<super::GetUserRequest>,
        ) -> Result<tonic::Response<super::GetUserResponse>, tonic::Status>;
        /// UpdateUserDisplayName
        ///
        /// Updates the Displayname for the user (Personal only)
        async fn update_user_display_name(
            &self,
            request: tonic::Request<super::UpdateUserDisplayNameRequest>,
        ) -> Result<
            tonic::Response<super::UpdateUserDisplayNameResponse>,
            tonic::Status,
        >;
        /// GetUserProjects
        ///
        /// Gets all project_ids a user is member of
        async fn get_user_projects(
            &self,
            request: tonic::Request<super::GetUserProjectsRequest>,
        ) -> Result<tonic::Response<super::GetUserProjectsResponse>, tonic::Status>;
        /// GetNotActivatedUsers
        ///
        /// Get all not activated users (Admin only)
        async fn get_not_activated_users(
            &self,
            request: tonic::Request<super::GetNotActivatedUsersRequest>,
        ) -> Result<tonic::Response<super::GetNotActivatedUsersResponse>, tonic::Status>;
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.UserService/RegisterUser" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).register_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterUserSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/ActivateUser" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).activate_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ActivateUserSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/CreateAPIToken" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_api_token(request).await
                            };
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
                "/aruna.api.storage.services.v1.UserService/GetAPIToken" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_api_token(request).await
                            };
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
                "/aruna.api.storage.services.v1.UserService/GetAPITokens" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_api_tokens(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAPITokensSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/DeleteAPIToken" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_api_token(request).await
                            };
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
                "/aruna.api.storage.services.v1.UserService/DeleteAPITokens" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_api_tokens(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAPITokensSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/GetUser" => {
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
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUserSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/UpdateUserDisplayName" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_user_display_name(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateUserDisplayNameSvc(inner);
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
                "/aruna.api.storage.services.v1.UserService/GetUserProjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserProjectsSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::GetUserProjectsRequest>
                    for GetUserProjectsSvc<T> {
                        type Response = super::GetUserProjectsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserProjectsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_user_projects(request).await
                            };
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
                "/aruna.api.storage.services.v1.UserService/GetNotActivatedUsers" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_not_activated_users(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNotActivatedUsersSvc(inner);
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
            }
        }
    }
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.UserService";
    }
}
// Models:

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNewCollectionRequest {
    /// Collection name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Description
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Project id
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
    /// List of associated labels
    #[prost(message, repeated, tag="4")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// List of associated hooks
    #[prost(message, repeated, tag="5")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// Optional LabelOntology with required labels
    #[prost(message, optional, tag="6")]
    pub label_ontology: ::core::option::Option<super::super::models::v1::LabelOntology>,
    /// Optional Dataclass
    #[prost(enumeration="super::super::models::v1::DataClass", tag="7")]
    pub dataclass: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNewCollectionResponse {
    /// The new collection_id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByIdRequest {
    /// Requested id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionByIdResponse {
    /// Overview of the requested collection
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// (optional) Filter, label or ids
    #[prost(message, optional, tag="2")]
    pub label_or_id_filter: ::core::option::Option<super::super::models::v1::LabelOrIdQuery>,
    /// (optional) Pagerequest
    #[prost(message, optional, tag="3")]
    pub page_request: ::core::option::Option<super::super::models::v1::PageRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionsResponse {
    /// List of collection overviews
    #[prost(message, optional, tag="1")]
    pub collections: ::core::option::Option<super::super::models::v1::CollectionOverviews>,
}
/// This updates the collection
/// Updating a pinned collection will require a new version to be created
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionRequest {
    /// Project id
    #[prost(string, tag="1")]
    pub project_id: ::prost::alloc::string::String,
    /// Old collection_id
    #[prost(string, tag="2")]
    pub collection_id: ::prost::alloc::string::String,
    /// New name
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    /// New description
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// New list of labels
    #[prost(message, repeated, tag="5")]
    pub labels: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// New list of hooks
    #[prost(message, repeated, tag="6")]
    pub hooks: ::prost::alloc::vec::Vec<super::super::models::v1::KeyValue>,
    /// (optional) LabelOntology
    #[prost(message, optional, tag="7")]
    pub label_ontology: ::core::option::Option<super::super::models::v1::LabelOntology>,
    /// Optional update Dataclass, this will not overwrite
    /// the dataclass of all existing associated objects
    /// New objects can only have this dataclass
    #[prost(enumeration="super::super::models::v1::DataClass", tag="8")]
    pub dataclass: i32,
    /// If this is set, the collection will be automatically pinned to this version
    /// Similar to the more explicit Pin request
    /// Updating a pinned collection will make this field required
    /// (optional if unpinned || required if pinned)
    #[prost(message, optional, tag="9")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollectionResponse {
    /// New collection overview
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCollectionVersionRequest {
    /// Old collection_id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// New version
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<super::super::models::v1::Version>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PinCollectionVersionResponse {
    /// New collection overview
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<super::super::models::v1::CollectionOverview>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionRequest {
    /// Collection id
    #[prost(string, tag="1")]
    pub collection_id: ::prost::alloc::string::String,
    /// Project id
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
    /// Force delete
    #[prost(bool, tag="3")]
    pub force: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollectionResponse {
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
            D: std::convert::TryInto<tonic::transport::Endpoint>,
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
        /// CreateNewCollection
        ///
        /// creates a new Collection
        pub async fn create_new_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNewCollectionRequest>,
        ) -> Result<tonic::Response<super::CreateNewCollectionResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.CollectionService/CreateNewCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetCollectionByID
        ///
        /// Queries a specific Collection by ID
        /// The result can be one_of:
        /// CollectionOverview -> default
        /// CollectionWithID
        /// Collection (full)
        /// This can be modified with the optional OutputFormat parameter
        pub async fn get_collection_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionByIdRequest>,
        ) -> Result<tonic::Response<super::GetCollectionByIdResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.CollectionService/GetCollectionByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetCollections
        ///
        /// queries multiple collections by ID or by LabelFilter
        /// This returns by default a paginated result with 20 entries.
        /// Must specify a project_id as context
        pub async fn get_collections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetCollectionsResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.CollectionService/GetCollections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// UpdateCollection
        ///
        /// Updates the current collection
        /// This will update the collection in place if it is unversioned / latest
        /// A versioned (pinned) collection requires a new semantic version after the
        /// update This can be used to pin a collection to a specific version similar
        /// to the PinCollectionVersion request
        pub async fn update_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollectionRequest>,
        ) -> Result<tonic::Response<super::UpdateCollectionResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.CollectionService/UpdateCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// PinCollectionVersion
        ///
        /// This pins the current status of the version to a
        /// specific version. Effectively creating a copy of the collection with a
        /// stable version All objects will be pinned to an explicit revision number
        /// Pinned collections can not be updated in place
        pub async fn pin_collection_version(
            &mut self,
            request: impl tonic::IntoRequest<super::PinCollectionVersionRequest>,
        ) -> Result<
            tonic::Response<super::PinCollectionVersionResponse>,
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
                "/aruna.api.storage.services.v1.CollectionService/PinCollectionVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteCollection
        ///
        /// This request deletes the collection.
        /// If with_version is true, it deletes the collection and all its versions.
        /// If cascade is true, all objects that are owned by the collection will also
        /// deleted. This should be avoided
        pub async fn delete_collection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status> {
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
                "/aruna.api.storage.services.v1.CollectionService/DeleteCollection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod collection_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CollectionServiceServer.
    #[async_trait]
    pub trait CollectionService: Send + Sync + 'static {
        /// CreateNewCollection
        ///
        /// creates a new Collection
        async fn create_new_collection(
            &self,
            request: tonic::Request<super::CreateNewCollectionRequest>,
        ) -> Result<tonic::Response<super::CreateNewCollectionResponse>, tonic::Status>;
        /// GetCollectionByID
        ///
        /// Queries a specific Collection by ID
        /// The result can be one_of:
        /// CollectionOverview -> default
        /// CollectionWithID
        /// Collection (full)
        /// This can be modified with the optional OutputFormat parameter
        async fn get_collection_by_id(
            &self,
            request: tonic::Request<super::GetCollectionByIdRequest>,
        ) -> Result<tonic::Response<super::GetCollectionByIdResponse>, tonic::Status>;
        /// GetCollections
        ///
        /// queries multiple collections by ID or by LabelFilter
        /// This returns by default a paginated result with 20 entries.
        /// Must specify a project_id as context
        async fn get_collections(
            &self,
            request: tonic::Request<super::GetCollectionsRequest>,
        ) -> Result<tonic::Response<super::GetCollectionsResponse>, tonic::Status>;
        /// UpdateCollection
        ///
        /// Updates the current collection
        /// This will update the collection in place if it is unversioned / latest
        /// A versioned (pinned) collection requires a new semantic version after the
        /// update This can be used to pin a collection to a specific version similar
        /// to the PinCollectionVersion request
        async fn update_collection(
            &self,
            request: tonic::Request<super::UpdateCollectionRequest>,
        ) -> Result<tonic::Response<super::UpdateCollectionResponse>, tonic::Status>;
        /// PinCollectionVersion
        ///
        /// This pins the current status of the version to a
        /// specific version. Effectively creating a copy of the collection with a
        /// stable version All objects will be pinned to an explicit revision number
        /// Pinned collections can not be updated in place
        async fn pin_collection_version(
            &self,
            request: tonic::Request<super::PinCollectionVersionRequest>,
        ) -> Result<tonic::Response<super::PinCollectionVersionResponse>, tonic::Status>;
        /// DeleteCollection
        ///
        /// This request deletes the collection.
        /// If with_version is true, it deletes the collection and all its versions.
        /// If cascade is true, all objects that are owned by the collection will also
        /// deleted. This should be avoided
        async fn delete_collection(
            &self,
            request: tonic::Request<super::DeleteCollectionRequest>,
        ) -> Result<tonic::Response<super::DeleteCollectionResponse>, tonic::Status>;
    }
    /// CollectionService
    ///
    /// Contains all methods that get/create or update Collection and associated resources
    #[derive(Debug)]
    pub struct CollectionServiceServer<T: CollectionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
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
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/aruna.api.storage.services.v1.CollectionService/CreateNewCollection" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNewCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::CreateNewCollectionRequest>
                    for CreateNewCollectionSvc<T> {
                        type Response = super::CreateNewCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNewCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_new_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateNewCollectionSvc(inner);
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
                "/aruna.api.storage.services.v1.CollectionService/GetCollectionByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetCollectionByIDSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::GetCollectionByIdRequest>
                    for GetCollectionByIDSvc<T> {
                        type Response = super::GetCollectionByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_collection_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionByIDSvc(inner);
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
                "/aruna.api.storage.services.v1.CollectionService/GetCollections" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_collections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCollectionsSvc(inner);
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
                "/aruna.api.storage.services.v1.CollectionService/UpdateCollection" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCollectionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::UpdateCollectionRequest>
                    for UpdateCollectionSvc<T> {
                        type Response = super::UpdateCollectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCollectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateCollectionSvc(inner);
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
                "/aruna.api.storage.services.v1.CollectionService/PinCollectionVersion" => {
                    #[allow(non_camel_case_types)]
                    struct PinCollectionVersionSvc<T: CollectionService>(pub Arc<T>);
                    impl<
                        T: CollectionService,
                    > tonic::server::UnaryService<super::PinCollectionVersionRequest>
                    for PinCollectionVersionSvc<T> {
                        type Response = super::PinCollectionVersionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PinCollectionVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).pin_collection_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PinCollectionVersionSvc(inner);
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
                "/aruna.api.storage.services.v1.CollectionService/DeleteCollection" => {
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
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_collection(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteCollectionSvc(inner);
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
            }
        }
    }
    impl<T: CollectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CollectionService> tonic::server::NamedService
    for CollectionServiceServer<T> {
        const NAME: &'static str = "aruna.api.storage.services.v1.CollectionService";
    }
}
