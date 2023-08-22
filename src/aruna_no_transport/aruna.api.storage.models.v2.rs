/// ------------- USERS & PERMISSIONS -----------------------
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// Internal Aruna UserID
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Oidc subject ID
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
    /// (optional) User display_name
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Is the user activated
    #[prost(bool, tag = "4")]
    pub active: bool,
    /// User email (empty if service account)
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    /// User attributes
    #[prost(message, optional, tag = "6")]
    pub attributes: ::core::option::Option<UserAttributes>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    #[prost(enumeration = "PermissionLevel", tag = "6")]
    pub permission_level: i32,
    #[prost(oneof = "permission::ResourceId", tags = "1, 2, 3, 4")]
    pub resource_id: ::core::option::Option<permission::ResourceId>,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceId {
        #[prost(string, tag = "1")]
        ProjectId(::prost::alloc::string::String),
        #[prost(string, tag = "2")]
        CollectionId(::prost::alloc::string::String),
        #[prost(string, tag = "3")]
        DatasetId(::prost::alloc::string::String),
        #[prost(string, tag = "4")]
        ObjectId(::prost::alloc::string::String),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub expires_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    /// Tokens can either be personal or resource "specific"
    #[prost(message, optional, tag = "5")]
    pub permission: ::core::option::Option<Permission>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttributes {
    #[prost(string, tag = "1")]
    pub attribute_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub attribute_value: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAttributes {
    #[prost(bool, tag = "1")]
    pub global_admin: bool,
    #[prost(bool, tag = "2")]
    pub service_account: bool,
    #[prost(message, repeated, tag = "3")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
    #[prost(string, repeated, tag = "4")]
    pub trusted_endpoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub custom_attributes: ::prost::alloc::vec::Vec<CustomAttributes>,
    #[prost(message, repeated, tag = "6")]
    pub personal_permissions: ::prost::alloc::vec::Vec<Permission>,
}
/// A key value pair for hooks and labels
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(enumeration = "KeyValueVariant", tag = "3")]
    pub variant: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    #[prost(oneof = "relation::Relation", tags = "1, 2")]
    pub relation: ::core::option::Option<relation::Relation>,
}
/// Nested message and enum types in `Relation`.
pub mod relation {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Relation {
        #[prost(message, tag = "1")]
        External(super::ExternalRelation),
        #[prost(message, tag = "2")]
        Internal(super::InternalRelation),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalRelation {
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(enumeration = "ExternalRelationVariant", tag = "2")]
    pub defined_variant: i32,
    /// Will only be filled if defined_variant == CUSTOM
    #[prost(string, optional, tag = "3")]
    pub custom_variant: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalRelation {
    #[prost(string, tag = "1")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ResourceVariant", tag = "2")]
    pub resource_variant: i32,
    #[prost(enumeration = "InternalRelationVariant", tag = "3")]
    pub defined_variant: i32,
    /// Will only be filled if defined_variant == CUSTOM
    #[prost(string, optional, tag = "4")]
    pub custom_variant: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "RelationDirection", tag = "5")]
    pub direction: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRequest {
    /// This is the last ID of the previous returned request
    #[prost(string, tag = "1")]
    pub start_after: ::prost::alloc::string::String,
    /// Default to 20, -1 for all
    #[prost(int64, tag = "2")]
    pub page_size: i64,
}
/// Stats for a set of objects
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stats {
    #[prost(int64, tag = "1")]
    pub count: i64,
    #[prost(int64, tag = "2")]
    pub size: i64,
    #[prost(message, optional, tag = "3")]
    pub last_updated: ::core::option::Option<::prost_wkt_types::Timestamp>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    #[prost(enumeration = "Hashalgorithm", tag = "1")]
    pub alg: i32,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointHostConfig {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_primary: bool,
    #[prost(bool, tag = "3")]
    pub ssl: bool,
    #[prost(bool, tag = "4")]
    pub public: bool,
    #[prost(enumeration = "EndpointHostVariant", tag = "5")]
    pub host_variant: i32,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "EndpointVariant", tag = "2")]
    pub ep_variant: i32,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub is_public: bool,
    #[prost(enumeration = "ComponentStatus", tag = "5")]
    pub status: i32,
    #[prost(message, repeated, tag = "6")]
    pub host_configs: ::prost::alloc::vec::Vec<EndpointHostConfig>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataEndpoint {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Hint if the objects' project
    /// is fully synced to the endpoint
    #[prost(bool, tag = "2")]
    pub full_synced: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Copy {
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_endpoint: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub push: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(oneof = "context::Context", tags = "1, 2")]
    pub context: ::core::option::Option<context::Context>,
}
/// Nested message and enum types in `Context`.
pub mod context {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Context {
        #[prost(bool, tag = "1")]
        S3Credentials(bool),
        #[prost(message, tag = "2")]
        Copy(super::Copy),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericResource {
    #[prost(oneof = "generic_resource::Resource", tags = "1, 2, 3, 4")]
    pub resource: ::core::option::Option<generic_resource::Resource>,
}
/// Nested message and enum types in `GenericResource`.
pub mod generic_resource {
    #[derive(serde::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        #[prost(message, tag = "1")]
        Project(super::Project),
        #[prost(message, tag = "2")]
        Collection(super::Collection),
        #[prost(message, tag = "3")]
        Dataset(super::Dataset),
        #[prost(message, tag = "4")]
        Object(super::Object),
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Short name according to BucketNamingRules
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Long name
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Project specific labels / hooks
    #[prost(message, repeated, tag = "4")]
    pub key_values: ::prost::alloc::vec::Vec<KeyValue>,
    /// Relations to internal and external sources
    #[prost(message, repeated, tag = "5")]
    pub relations: ::prost::alloc::vec::Vec<Relation>,
    #[prost(message, optional, tag = "6")]
    pub stats: ::core::option::Option<Stats>,
    #[prost(enumeration = "DataClass", tag = "7")]
    pub data_class: i32,
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
    #[prost(bool, tag = "11")]
    pub dynamic: bool,
    #[prost(message, repeated, tag = "12")]
    pub endpoints: ::prost::alloc::vec::Vec<DataEndpoint>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    /// ASDASDASDOPASKIDPO
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// my_mags
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// ENA asda234928349028 MAG 1293819203819028i V1
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Collection specific labels / hooks
    #[prost(message, repeated, tag = "4")]
    pub key_values: ::prost::alloc::vec::Vec<KeyValue>,
    /// Relations to internal and external sources
    #[prost(message, repeated, tag = "5")]
    pub relations: ::prost::alloc::vec::Vec<Relation>,
    #[prost(message, optional, tag = "6")]
    pub stats: ::core::option::Option<Stats>,
    #[prost(enumeration = "DataClass", tag = "7")]
    pub data_class: i32,
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
    #[prost(bool, tag = "11")]
    pub dynamic: bool,
    #[prost(message, repeated, tag = "12")]
    pub endpoints: ::prost::alloc::vec::Vec<DataEndpoint>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Dataset specific labels / hooks
    #[prost(message, repeated, tag = "4")]
    pub key_values: ::prost::alloc::vec::Vec<KeyValue>,
    /// Relations to internal and external sources
    #[prost(message, repeated, tag = "5")]
    pub relations: ::prost::alloc::vec::Vec<Relation>,
    #[prost(message, optional, tag = "6")]
    pub stats: ::core::option::Option<Stats>,
    #[prost(enumeration = "DataClass", tag = "7")]
    pub data_class: i32,
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
    #[prost(bool, tag = "11")]
    pub dynamic: bool,
    #[prost(message, repeated, tag = "12")]
    pub endpoints: ::prost::alloc::vec::Vec<DataEndpoint>,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Collection specific labels / hooks
    #[prost(message, repeated, tag = "4")]
    pub key_values: ::prost::alloc::vec::Vec<KeyValue>,
    /// Relations to internal and external sources
    #[prost(message, repeated, tag = "5")]
    pub relations: ::prost::alloc::vec::Vec<Relation>,
    /// Object only
    #[prost(int64, tag = "6")]
    pub content_len: i64,
    #[prost(enumeration = "DataClass", tag = "7")]
    pub data_class: i32,
    #[prost(message, optional, tag = "8")]
    pub created_at: ::core::option::Option<::prost_wkt_types::Timestamp>,
    #[prost(string, tag = "9")]
    pub created_by: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "10")]
    pub status: i32,
    #[prost(bool, tag = "11")]
    pub dynamic: bool,
    #[prost(message, repeated, tag = "12")]
    pub endpoints: ::prost::alloc::vec::Vec<DataEndpoint>,
    /// Object specific attributes
    #[prost(message, repeated, tag = "13")]
    pub hashes: ::prost::alloc::vec::Vec<Hash>,
}
/// Dataclass defines the confidentiality of the object
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataClass {
    Unspecified = 0,
    Public = 1,
    Private = 2,
    Workspace = 4,
    Confidential = 5,
}
impl DataClass {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataClass::Unspecified => "DATA_CLASS_UNSPECIFIED",
            DataClass::Public => "DATA_CLASS_PUBLIC",
            DataClass::Private => "DATA_CLASS_PRIVATE",
            DataClass::Workspace => "DATA_CLASS_WORKSPACE",
            DataClass::Confidential => "DATA_CLASS_CONFIDENTIAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATA_CLASS_UNSPECIFIED" => Some(Self::Unspecified),
            "DATA_CLASS_PUBLIC" => Some(Self::Public),
            "DATA_CLASS_PRIVATE" => Some(Self::Private),
            "DATA_CLASS_WORKSPACE" => Some(Self::Workspace),
            "DATA_CLASS_CONFIDENTIAL" => Some(Self::Confidential),
            _ => None,
        }
    }
}
/// Which kind of endpoint
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EndpointVariant {
    Unspecified = 0,
    Persistent = 1,
    Volatile = 2,
}
impl EndpointVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EndpointVariant::Unspecified => "ENDPOINT_VARIANT_UNSPECIFIED",
            EndpointVariant::Persistent => "ENDPOINT_VARIANT_PERSISTENT",
            EndpointVariant::Volatile => "ENDPOINT_VARIANT_VOLATILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENDPOINT_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "ENDPOINT_VARIANT_PERSISTENT" => Some(Self::Persistent),
            "ENDPOINT_VARIANT_VOLATILE" => Some(Self::Volatile),
            _ => None,
        }
    }
}
/// Which features does the endpoint have
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EndpointHostVariant {
    Unspecified = 0,
    Proxy = 1,
    Bundler = 2,
}
impl EndpointHostVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EndpointHostVariant::Unspecified => "ENDPOINT_HOST_VARIANT_UNSPECIFIED",
            EndpointHostVariant::Proxy => "ENDPOINT_HOST_VARIANT_PROXY",
            EndpointHostVariant::Bundler => "ENDPOINT_HOST_VARIANT_BUNDLER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENDPOINT_HOST_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "ENDPOINT_HOST_VARIANT_PROXY" => Some(Self::Proxy),
            "ENDPOINT_HOST_VARIANT_BUNDLER" => Some(Self::Bundler),
            _ => None,
        }
    }
}
/// Permission Levels
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PermissionLevel {
    Unspecified = 0,
    None = 2,
    Read = 3,
    Append = 4,
    Write = 5,
    Admin = 6,
}
impl PermissionLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PermissionLevel::Unspecified => "PERMISSION_LEVEL_UNSPECIFIED",
            PermissionLevel::None => "PERMISSION_LEVEL_NONE",
            PermissionLevel::Read => "PERMISSION_LEVEL_READ",
            PermissionLevel::Append => "PERMISSION_LEVEL_APPEND",
            PermissionLevel::Write => "PERMISSION_LEVEL_WRITE",
            PermissionLevel::Admin => "PERMISSION_LEVEL_ADMIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_LEVEL_NONE" => Some(Self::None),
            "PERMISSION_LEVEL_READ" => Some(Self::Read),
            "PERMISSION_LEVEL_APPEND" => Some(Self::Append),
            "PERMISSION_LEVEL_WRITE" => Some(Self::Write),
            "PERMISSION_LEVEL_ADMIN" => Some(Self::Admin),
            _ => None,
        }
    }
}
/// KeyValueVariants
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyValueVariant {
    Unspecified = 0,
    Label = 1,
    /// A Label that only admins can remove
    StaticLabel = 2,
    Hook = 3,
}
impl KeyValueVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyValueVariant::Unspecified => "KEY_VALUE_VARIANT_UNSPECIFIED",
            KeyValueVariant::Label => "KEY_VALUE_VARIANT_LABEL",
            KeyValueVariant::StaticLabel => "KEY_VALUE_VARIANT_STATIC_LABEL",
            KeyValueVariant::Hook => "KEY_VALUE_VARIANT_HOOK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KEY_VALUE_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "KEY_VALUE_VARIANT_LABEL" => Some(Self::Label),
            "KEY_VALUE_VARIANT_STATIC_LABEL" => Some(Self::StaticLabel),
            "KEY_VALUE_VARIANT_HOOK" => Some(Self::Hook),
            _ => None,
        }
    }
}
/// External Relations
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExternalRelationVariant {
    Unspecified = 0,
    Url = 1,
    Identifier = 2,
    Custom = 3,
}
impl ExternalRelationVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExternalRelationVariant::Unspecified => {
                "EXTERNAL_RELATION_VARIANT_UNSPECIFIED"
            }
            ExternalRelationVariant::Url => "EXTERNAL_RELATION_VARIANT_URL",
            ExternalRelationVariant::Identifier => "EXTERNAL_RELATION_VARIANT_IDENTIFIER",
            ExternalRelationVariant::Custom => "EXTERNAL_RELATION_VARIANT_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXTERNAL_RELATION_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "EXTERNAL_RELATION_VARIANT_URL" => Some(Self::Url),
            "EXTERNAL_RELATION_VARIANT_IDENTIFIER" => Some(Self::Identifier),
            "EXTERNAL_RELATION_VARIANT_CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
/// InternalRelations
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InternalRelationVariant {
    Unspecified = 0,
    BelongsTo = 1,
    Origin = 2,
    Version = 3,
    Metadata = 4,
    Policy = 5,
    Custom = 6,
}
impl InternalRelationVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InternalRelationVariant::Unspecified => {
                "INTERNAL_RELATION_VARIANT_UNSPECIFIED"
            }
            InternalRelationVariant::BelongsTo => "INTERNAL_RELATION_VARIANT_BELONGS_TO",
            InternalRelationVariant::Origin => "INTERNAL_RELATION_VARIANT_ORIGIN",
            InternalRelationVariant::Version => "INTERNAL_RELATION_VARIANT_VERSION",
            InternalRelationVariant::Metadata => "INTERNAL_RELATION_VARIANT_METADATA",
            InternalRelationVariant::Policy => "INTERNAL_RELATION_VARIANT_POLICY",
            InternalRelationVariant::Custom => "INTERNAL_RELATION_VARIANT_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTERNAL_RELATION_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "INTERNAL_RELATION_VARIANT_BELONGS_TO" => Some(Self::BelongsTo),
            "INTERNAL_RELATION_VARIANT_ORIGIN" => Some(Self::Origin),
            "INTERNAL_RELATION_VARIANT_VERSION" => Some(Self::Version),
            "INTERNAL_RELATION_VARIANT_METADATA" => Some(Self::Metadata),
            "INTERNAL_RELATION_VARIANT_POLICY" => Some(Self::Policy),
            "INTERNAL_RELATION_VARIANT_CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
/// internal object relation type (direction)
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RelationDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}
impl RelationDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RelationDirection::Unspecified => "RELATION_DIRECTION_UNSPECIFIED",
            RelationDirection::Inbound => "RELATION_DIRECTION_INBOUND",
            RelationDirection::Outbound => "RELATION_DIRECTION_OUTBOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RELATION_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "RELATION_DIRECTION_INBOUND" => Some(Self::Inbound),
            "RELATION_DIRECTION_OUTBOUND" => Some(Self::Outbound),
            _ => None,
        }
    }
}
/// Used for the internal associated services to validate permissions
/// Actions are similar to HTTP verbs
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceAction {
    Unspecified = 0,
    Create = 1,
    Append = 2,
    Update = 3,
    Read = 4,
    Delete = 5,
}
impl ResourceAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceAction::Unspecified => "RESOURCE_ACTION_UNSPECIFIED",
            ResourceAction::Create => "RESOURCE_ACTION_CREATE",
            ResourceAction::Append => "RESOURCE_ACTION_APPEND",
            ResourceAction::Update => "RESOURCE_ACTION_UPDATE",
            ResourceAction::Read => "RESOURCE_ACTION_READ",
            ResourceAction::Delete => "RESOURCE_ACTION_DELETE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOURCE_ACTION_CREATE" => Some(Self::Create),
            "RESOURCE_ACTION_APPEND" => Some(Self::Append),
            "RESOURCE_ACTION_UPDATE" => Some(Self::Update),
            "RESOURCE_ACTION_READ" => Some(Self::Read),
            "RESOURCE_ACTION_DELETE" => Some(Self::Delete),
            _ => None,
        }
    }
}
/// An arbitrary status for Objects
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// Unspecified
    Unspecified = 0,
    /// This object is initializing -> Staging
    Initializing = 1,
    /// Data got uploaded and a validating hook got triggered
    Validating = 2,
    /// Data is available
    Available = 3,
    /// Data is temporarily not available
    Unavailable = 4,
    /// Validating failed or fatal error in data proxy
    Error = 5,
    /// Object got deleted
    Deleted = 6,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unspecified => "STATUS_UNSPECIFIED",
            Status::Initializing => "STATUS_INITIALIZING",
            Status::Validating => "STATUS_VALIDATING",
            Status::Available => "STATUS_AVAILABLE",
            Status::Unavailable => "STATUS_UNAVAILABLE",
            Status::Error => "STATUS_ERROR",
            Status::Deleted => "STATUS_DELETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "STATUS_INITIALIZING" => Some(Self::Initializing),
            "STATUS_VALIDATING" => Some(Self::Validating),
            "STATUS_AVAILABLE" => Some(Self::Available),
            "STATUS_UNAVAILABLE" => Some(Self::Unavailable),
            "STATUS_ERROR" => Some(Self::Error),
            "STATUS_DELETED" => Some(Self::Deleted),
            _ => None,
        }
    }
}
/// Status for endpoints
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComponentStatus {
    Unspecified = 0,
    Initializing = 1,
    Available = 2,
    Degraded = 3,
    Unavailable = 4,
    Maintenance = 5,
}
impl ComponentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComponentStatus::Unspecified => "COMPONENT_STATUS_UNSPECIFIED",
            ComponentStatus::Initializing => "COMPONENT_STATUS_INITIALIZING",
            ComponentStatus::Available => "COMPONENT_STATUS_AVAILABLE",
            ComponentStatus::Degraded => "COMPONENT_STATUS_DEGRADED",
            ComponentStatus::Unavailable => "COMPONENT_STATUS_UNAVAILABLE",
            ComponentStatus::Maintenance => "COMPONENT_STATUS_MAINTENANCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPONENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPONENT_STATUS_INITIALIZING" => Some(Self::Initializing),
            "COMPONENT_STATUS_AVAILABLE" => Some(Self::Available),
            "COMPONENT_STATUS_DEGRADED" => Some(Self::Degraded),
            "COMPONENT_STATUS_UNAVAILABLE" => Some(Self::Unavailable),
            "COMPONENT_STATUS_MAINTENANCE" => Some(Self::Maintenance),
            _ => None,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Hashalgorithm {
    Unspecified = 0,
    Md5 = 1,
    Sha256 = 2,
}
impl Hashalgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Hashalgorithm::Unspecified => "HASHALGORITHM_UNSPECIFIED",
            Hashalgorithm::Md5 => "HASHALGORITHM_MD5",
            Hashalgorithm::Sha256 => "HASHALGORITHM_SHA256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HASHALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "HASHALGORITHM_MD5" => Some(Self::Md5),
            "HASHALGORITHM_SHA256" => Some(Self::Sha256),
            _ => None,
        }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceVariant {
    Unspecified = 0,
    Project = 1,
    Collection = 2,
    Dataset = 3,
    Object = 4,
}
impl ResourceVariant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceVariant::Unspecified => "RESOURCE_VARIANT_UNSPECIFIED",
            ResourceVariant::Project => "RESOURCE_VARIANT_PROJECT",
            ResourceVariant::Collection => "RESOURCE_VARIANT_COLLECTION",
            ResourceVariant::Dataset => "RESOURCE_VARIANT_DATASET",
            ResourceVariant::Object => "RESOURCE_VARIANT_OBJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESOURCE_VARIANT_UNSPECIFIED" => Some(Self::Unspecified),
            "RESOURCE_VARIANT_PROJECT" => Some(Self::Project),
            "RESOURCE_VARIANT_COLLECTION" => Some(Self::Collection),
            "RESOURCE_VARIANT_DATASET" => Some(Self::Dataset),
            "RESOURCE_VARIANT_OBJECT" => Some(Self::Object),
            _ => None,
        }
    }
}
