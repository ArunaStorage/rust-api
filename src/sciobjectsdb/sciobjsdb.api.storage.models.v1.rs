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
    #[prost(enumeration = "origin::OriginType", tag = "3")]
    pub origin_type: i32,
    #[prost(oneof = "origin::Location", tags = "2")]
    pub location: ::core::option::Option<origin::Location>,
}
/// Nested message and enum types in `Origin`.
pub mod origin {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OriginType {
        Unspecified = 0,
        ObjectStorage = 1,
        OriginLink = 2,
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
        Unspecified = 0,
        Stable = 1,
        Rc = 2,
        Beta = 3,
        Alpha = 4,
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
    #[prost(string, tag = "2")]
    pub version_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiToken {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(enumeration = "Right", repeated, tag = "3")]
    pub rights: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
/// Request for paginated result retrival
/// If the page size is zero, the backends default page size will be used
/// If the page size exceeds the backends maximum, an error will be returned
/// Consistency of results will only be guaranteed for DatasetVersions
/// For the initial request leave last_uuid empty
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRequest {
    #[prost(string, tag = "1")]
    pub last_uuid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub page_size: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unspecified = 0,
    Initiating = 1,
    Available = 2,
    Updating = 3,
    Archived = 4,
    Deleting = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Right {
    Unspecified = 0,
    Read = 1,
    Write = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Resource {
    Unspecified = 0,
    Project = 1,
    Dataset = 2,
    DatasetVersion = 3,
    Object = 4,
    ObjectGroup = 5,
    ObjectGroupRevision = 6,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectGroup {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "6")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "7")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    #[prost(enumeration = "Status", tag = "8")]
    pub status: i32,
    #[prost(message, repeated, tag = "9")]
    pub objects: ::prost::alloc::vec::Vec<Object>,
    #[prost(message, optional, tag = "10")]
    pub generated: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "11")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    ///ID of the entity
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
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
    #[prost(message, optional, tag = "11")]
    pub generated: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "12")]
    pub object_group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "15")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
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
    #[prost(string, tag = "7")]
    pub project_id: ::prost::alloc::string::String,
    ///Indicates if the dataset if publicly available
    #[prost(bool, tag = "8")]
    pub is_public: bool,
    #[prost(enumeration = "Status", tag = "9")]
    pub status: i32,
    #[prost(string, tag = "10")]
    pub bucket: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetVersion {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::prost::alloc::vec::Vec<Metadata>,
    /// When the datasets version was created
    #[prost(message, optional, tag = "7")]
    pub created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "8")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, repeated, tag = "9")]
    pub object_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Number of objects registered with this dataset version
    #[prost(int64, tag = "10")]
    pub object_count: i64,
    /// Indicates the status of a dataset
    #[prost(enumeration = "Status", tag = "11")]
    pub status: i32,
    #[prost(string, tag = "12")]
    pub project_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
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
    #[prost(string, tag = "7")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "8")]
    pub status: i32,
}
