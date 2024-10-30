// @generated
/// Capability defines an implementation of an object capability. The index
/// provided to a Capability must be globally unique.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capability {
    #[prost(uint64, tag = "1")]
    pub index: u64,
}
impl ::prost::Name for Capability {
    const NAME: &'static str = "Capability";
    const PACKAGE: &'static str = "capability.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("capability.v1.{}", Self::NAME)
    }
}
/// Owner defines a single capability owner. An owner is defined by the name of
/// capability and the module name.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Owner {
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
impl ::prost::Name for Owner {
    const NAME: &'static str = "Owner";
    const PACKAGE: &'static str = "capability.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("capability.v1.{}", Self::NAME)
    }
}
/// CapabilityOwners defines a set of owners of a single Capability. The set of
/// owners must be unique.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityOwners {
    #[prost(message, repeated, tag = "1")]
    pub owners: ::prost::alloc::vec::Vec<Owner>,
}
impl ::prost::Name for CapabilityOwners {
    const NAME: &'static str = "CapabilityOwners";
    const PACKAGE: &'static str = "capability.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("capability.v1.{}", Self::NAME)
    }
}
/// GenesisOwners defines the capability owners with their corresponding index.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisOwners {
    /// index is the index of the capability owner.
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// index_owners are the owners at the given index.
    #[prost(message, optional, tag = "2")]
    pub index_owners: ::core::option::Option<CapabilityOwners>,
}
impl ::prost::Name for GenesisOwners {
    const NAME: &'static str = "GenesisOwners";
    const PACKAGE: &'static str = "capability.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("capability.v1.{}", Self::NAME)
    }
}
/// GenesisState defines the capability module's genesis state.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// index is the capability global index.
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// owners represents a map from index to owners of the capability index
    /// index key is string to allow amino marshalling.
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<GenesisOwners>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "capability.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("capability.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `capability.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9f, 0x09, 0x0a, 0x1e, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2f,
    0x76, 0x31, 0x2f, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e,
    0x76, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f,
    0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2f,
    0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x28, 0x0a, 0x0a, 0x43,
    0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x3a,
    0x04, 0x98, 0xa0, 0x1f, 0x00, 0x22, 0x3d, 0x0a, 0x05, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x12, 0x16,
    0x0a, 0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x3a, 0x08, 0x88, 0xa0, 0x1f, 0x00,
    0x98, 0xa0, 0x1f, 0x00, 0x22, 0x4b, 0x0a, 0x10, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69,
    0x74, 0x79, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x12, 0x37, 0x0a, 0x06, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x63, 0x61, 0x70, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x42, 0x09,
    0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x73, 0x42, 0x33, 0x5a, 0x31, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2d, 0x67, 0x6f, 0x2f, 0x6d, 0x6f,
    0x64, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79,
    0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xd1, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1f,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x1e,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x07, 0x00, 0x48, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00, 0x48,
    0x0a, 0x88, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x1a, 0x7c, 0x20,
    0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x0a, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c,
    0x6c, 0x79, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03,
    0x0c, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x0c,
    0x02, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x11, 0x12, 0x0a, 0x7a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x13, 0x00, 0x19, 0x01, 0x1a, 0x6e, 0x20, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x20, 0x64, 0x65,
    0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x63,
    0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x2e,
    0x20, 0x41, 0x6e, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x0a, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x0d, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x03, 0x14, 0x02, 0x2e, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x01, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x14, 0x02, 0x2e, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x07, 0x12, 0x03, 0x15, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x07, 0x81,
    0xf4, 0x03, 0x12, 0x03, 0x15, 0x02, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x17, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x09, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17, 0x12, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x18, 0x12, 0x13, 0x0a, 0x71, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1d, 0x00, 0x1f,
    0x01, 0x1a, 0x65, 0x20, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x4f, 0x77,
    0x6e, 0x65, 0x72, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x0a,
    0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20,
    0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x1d, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x02,
    0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1e, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x1e, 0x1c, 0x59, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08,
    0xe9, 0xfb, 0x03, 0x12, 0x03, 0x1e, 0x1d, 0x39, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00,
    0x08, 0xf5, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x1e, 0x3b, 0x58, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33, 0x0a, 0xf3, 0x09, 0x0a, 0x1b, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x2f, 0x76, 0x31, 0x2f, 0x67, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0d, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x76,
    0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67,
    0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74,
    0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2f, 0x61,
    0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x74, 0x0a, 0x0d, 0x47, 0x65,
    0x6e, 0x65, 0x73, 0x69, 0x73, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x12, 0x14, 0x0a, 0x05, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x12, 0x4d, 0x0a, 0x0c, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x5f, 0x6f, 0x77, 0x6e, 0x65, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69,
    0x6c, 0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69,
    0x74, 0x79, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x42, 0x09, 0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7,
    0xb0, 0x2a, 0x01, 0x52, 0x0b, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x73,
    0x22, 0x65, 0x0a, 0x0c, 0x47, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x3f, 0x0a, 0x06, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c,
    0x69, 0x74, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x4f, 0x77,
    0x6e, 0x65, 0x72, 0x73, 0x42, 0x09, 0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52,
    0x06, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x42, 0x33, 0x5a, 0x31, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63,
    0x2d, 0x67, 0x6f, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x63, 0x61, 0x70, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xe1, 0x06, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x1b, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00,
    0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x1b, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x08, 0x00, 0x48, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x08, 0x00,
    0x48, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x11, 0x01, 0x1a, 0x4d, 0x20,
    0x47, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x4f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x64, 0x65,
    0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69,
    0x6c, 0x69, 0x74, 0x79, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x15, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x0d, 0x02, 0x13, 0x1a, 0x2d, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x6f, 0x77, 0x6e,
    0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x11, 0x12, 0x0a, 0x3e,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x10, 0x02, 0x62, 0x1a, 0x31, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x5f, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x08, 0x12, 0x03, 0x10, 0x24, 0x61, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe9,
    0xfb, 0x03, 0x12, 0x03, 0x10, 0x25, 0x41, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08,
    0xf5, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x10, 0x43, 0x60, 0x0a, 0x49, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x14, 0x00, 0x1b, 0x01, 0x1a, 0x3d, 0x20, 0x47, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x27, 0x73, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x14,
    0x0a, 0x34, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x13, 0x1a, 0x27, 0x20,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x70,
    0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16,
    0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x11, 0x12,
    0x0a, 0x84, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x62, 0x1a, 0x77,
    0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20,
    0x69, 0x6e, 0x64, 0x65, 0x78, 0x0a, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x6b, 0x65, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x20, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x20, 0x6d, 0x61, 0x72, 0x73, 0x68, 0x61,
    0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x1a, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x19,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x1a, 0x24, 0x61, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x1a, 0x25, 0x41, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x02, 0x01, 0x08, 0xf5, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x1a, 0x43, 0x60,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
