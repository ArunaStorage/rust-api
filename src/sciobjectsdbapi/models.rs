/// A location in S3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Object storage endpoint
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    #[prost(enumeration = "LocationType", tag = "4")]
    pub location_type: i32,
    #[prost(message, optional, tag = "5")]
    pub index_location: ::core::option::Option<IndexLocation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexLocation {
    #[prost(int64, tag = "1")]
    pub start_byte: i64,
    #[prost(int64, tag = "2")]
    pub end_byte: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Origin {
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub object_storage_locatio: ::core::option::Option<Location>,
    #[prost(enumeration = "origin::OriginTypeEnum", tag = "4")]
    pub origin_type: i32,
}
/// Nested message and enum types in `Origin`.
pub mod origin {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OriginTypeEnum {
        ObjectStorage = 0,
        OriginLink = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(int32, tag = "1")]
    pub major: i32,
    #[prost(int32, tag = "2")]
    pub minor: i32,
    #[prost(int32, tag = "3")]
    pub patch: i32,
    #[prost(int32, tag = "4")]
    pub revision: i32,
    #[prost(enumeration = "version::VersionStage", tag = "5")]
    pub stage: i32,
}
/// Nested message and enum types in `Version`.
pub mod version {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VersionStage {
        Stable = 0,
        ReleaseCandidate = 1,
        Beta = 2,
        Alpha = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    /// An arbitrary ID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFieldsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub update_string_fields:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Right", repeated, tag = "2")]
    pub rights: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "Resource", tag = "3")]
    pub resource: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(string, tag = "3")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(oneof = "metadata::Schema", tags = "4")]
    pub schema: ::core::option::Option<metadata::Schema>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schema {
        #[prost(string, tag = "4")]
        SimpleSchema(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Initiating = 0,
    Available = 1,
    Updating = 2,
    Archived = 3,
    Deleting = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Right {
    Read = 0,
    Write = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Resource {
    Project = 0,
    Dataset = 1,
    DatasetVersion = 2,
    DatasetObject = 3,
    DatasetObjectGroupResource = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationType {
    Object = 0,
    Index = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectEntry {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub users: ::prost::alloc::vec::Vec<User>,
    #[prost(string, tag = "4")]
    pub project_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectHeritage {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Name: Name of the dataset object heritage entity e.g.: mydata
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "11")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetObjectGroup {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub version: ::core::option::Option<Version>,
    #[prost(int64, tag = "4")]
    pub initialized_objects: i64,
    #[prost(int64, tag = "5")]
    pub uploaded_objects: i64,
    #[prost(message, repeated, tag = "6")]
    pub objects: ::prost::alloc::vec::Vec<DatasetObjectEntry>,
    /// Additional metadata of the object
    #[prost(map = "string, message", tag = "7")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    #[prost(message, repeated, tag = "8")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(string, tag = "9")]
    pub object_heritage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "11")]
    pub status: i32,
    #[prost(message, repeated, tag = "12")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetObjectEntry {
    ///ID of the entity
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Filename: Name of the original file e.g.: mydata.json
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// Filetype: Type of the stored file, e.g.: json, gbff,...
    #[prost(string, tag = "3")]
    pub filetype: ::prost::alloc::string::String,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag = "4")]
    pub origin: ::core::option::Option<Origin>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag = "5")]
    pub content_len: i64,
    /// Location: Location of the data
    #[prost(message, optional, tag = "6")]
    pub location: ::core::option::Option<Location>,
    /// When the data object was created
    #[prost(message, optional, tag = "7")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Additional metadata of the object
    #[prost(map = "string, message", tag = "8")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    #[prost(string, tag = "9")]
    pub upload_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "11")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetEntry {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Name of the dataset
    #[prost(string, tag = "2")]
    pub datasetname: ::prost::alloc::string::String,
    /// Type of the stored data in the dataset
    #[prost(string, tag = "3")]
    pub datasettype: ::prost::alloc::string::String,
    ///Indicates if the dataset if publicly available
    #[prost(bool, tag = "4")]
    pub is_public: bool,
    /// When the datasets was created
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(enumeration = "Status", tag = "6")]
    pub status: i32,
    #[prost(string, tag = "7")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "9")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "10")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetVersionEntry {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub version: ::core::option::Option<Version>,
    /// When the datasets version was created
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Additional metadata for the dataset version
    #[prost(map = "string, message", tag = "6")]
    pub additional_metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Struct>,
    /// Message reference for the metadata
    #[prost(map = "string, string", tag = "7")]
    pub additional_metadata_message_ref:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Message reference for the metadata of the objects associated with this DatasetVersion
    #[prost(map = "string, string", tag = "8")]
    pub additional_object_metadata_message_ref:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Number of objects registered with this dataset version
    #[prost(int64, tag = "9")]
    pub object_count: i64,
    /// Indicates the status of a dataset
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
    #[prost(string, repeated, tag = "11")]
    pub object_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "12")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "13")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenList {
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub token: ::prost::alloc::vec::Vec<TokenEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenEntry {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub user_id: ::core::option::Option<User>,
    #[prost(string, tag = "3")]
    pub token: ::prost::alloc::string::String,
    #[prost(enumeration = "Resource", tag = "4")]
    pub resource: i32,
    /// When the token was created
    #[prost(message, optional, tag = "5")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// When the token expires
    #[prost(message, optional, tag = "6")]
    pub expires: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTokenRequest {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Right", repeated, tag = "2")]
    pub rights: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration = "Resource", tag = "3")]
    pub resource: i32,
    /// When the token expires
    #[prost(message, optional, tag = "4")]
    pub expires: ::core::option::Option<::prost_types::Timestamp>,
}
