#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(oneof = "location::Location", tags = "1, 2")]
    pub location: ::core::option::Option<location::Location>,
}
/// Nested message and enum types in `Location`.
pub mod location {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        #[prost(message, tag = "1")]
        ObjectLocation(super::ObjectLocation),
        #[prost(message, tag = "2")]
        ObjectIndexLocation(super::IndexedObjectLocation),
    }
}
/// A location in S3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectLocation {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Object storage endpoint
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
}
/// A location in S3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexedObjectLocation {
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Object storage endpoint
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub index: ::core::option::Option<Index>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    #[prost(int64, tag = "1")]
    pub start_byte: i64,
    #[prost(int64, tag = "2")]
    pub end_byte: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Origin {
    #[prost(string, tag = "1")]
    pub link: ::prost::alloc::string::String,
    #[prost(enumeration = "origin::OriginTypeEnum", tag = "3")]
    pub origin_type: i32,
    #[prost(oneof = "origin::Location", tags = "2")]
    pub location: ::core::option::Option<origin::Location>,
}
/// Nested message and enum types in `Origin`.
pub mod origin {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OriginTypeEnum {
        ObjectStorage = 0,
        OriginLink = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        #[prost(message, tag = "2")]
        ObjectLocation(super::ObjectLocation),
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
        Rc = 1,
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
    pub updated_string_fields:
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
    #[prost(bytes = "vec", tag = "3")]
    pub metadata: ::prost::alloc::vec::Vec<u8>,
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
pub struct VersionTag {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub version_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiToken {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(enumeration = "Right", repeated, tag = "3")]
    pub rights: ::prost::alloc::vec::Vec<i32>,
    #[prost(uint64, tag = "4")]
    pub project_id: u64,
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
    ProjectResource = 0,
    DatasetResource = 1,
    DatasetVersionResource = 2,
    ObjectResource = 3,
    ObjectGroupResource = 4,
    ObjectGroupRevisionResource = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroup {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub dataset_id: u64,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    #[prost(enumeration = "Status", tag = "6")]
    pub status: i32,
    #[prost(uint64, tag = "7")]
    pub head_id: u64,
    #[prost(int64, tag = "8")]
    pub current_revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroupRevision {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "4")]
    pub dataset_id: u64,
    #[prost(message, optional, tag = "5")]
    pub version: ::core::option::Option<Version>,
    /// When the data object was created
    #[prost(message, optional, tag = "6")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "7")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "8")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    #[prost(message, repeated, tag = "9")]
    pub objects: ::prost::alloc::vec::Vec<Object>,
    #[prost(uint64, tag = "10")]
    pub object_group_id: u64,
    #[prost(int64, tag = "11")]
    pub revision: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    ///ID of the entity
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Filename: Name of the original file e.g.: mydata.json
    #[prost(string, tag = "2")]
    pub filename: ::prost::alloc::string::String,
    /// Filetype: Type of the stored file, e.g.: json, gbff,...
    #[prost(string, tag = "3")]
    pub filetype: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    /// When the data object was created
    #[prost(message, optional, tag = "6")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    /// Location: Location of the data
    #[prost(message, optional, tag = "7")]
    pub location: ::core::option::Option<Location>,
    /// Origin: Source of the dataset
    #[prost(message, optional, tag = "8")]
    pub origin: ::core::option::Option<Origin>,
    /// ContentLen: Lenght of the stored dataset
    #[prost(int64, tag = "9")]
    pub content_len: i64,
    #[prost(string, tag = "10")]
    pub upload_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Name of the dataset
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// When the datasets was created
    #[prost(message, optional, tag = "4")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    #[prost(uint64, tag = "7")]
    pub project_id: u64,
    ///Indicates if the dataset if publicly available
    #[prost(bool, tag = "8")]
    pub is_public: bool,
    #[prost(enumeration = "Status", tag = "9")]
    pub status: i32,
    #[prost(message, repeated, tag = "10")]
    pub version_tags: ::prost::alloc::vec::Vec<VersionTag>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetVersion {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub dataset_id: u64,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    /// When the datasets version was created
    #[prost(message, optional, tag = "6")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub version: ::core::option::Option<Version>,
    #[prost(uint64, repeated, tag = "8")]
    pub object_group_ids: ::prost::alloc::vec::Vec<u64>,
    /// Number of objects registered with this dataset version
    #[prost(int64, tag = "9")]
    pub object_count: i64,
    /// Indicates the status of a dataset
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "5")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    #[prost(message, repeated, tag = "6")]
    pub users: ::prost::alloc::vec::Vec<User>,
}
