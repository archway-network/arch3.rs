// @generated
/// InterfaceDescriptor describes an interface type to be used with
/// accepts_interface and implements_interface and declared by declare_interface.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceDescriptor {
    /// name is the name of the interface. It should be a short-name (without
    /// a period) such that the fully qualified name of the interface will be
    /// package.name, ex. for the package a.b and interface named C, the
    /// fully-qualified name will be a.b.C.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description is a human-readable description of the interface and its
    /// purpose.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
impl ::prost::Name for InterfaceDescriptor {
    const NAME: &'static str = "InterfaceDescriptor";
    const PACKAGE: &'static str = "cosmos_proto";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos_proto.{}", Self::NAME)
    }
}
/// ScalarDescriptor describes an scalar type to be used with
/// the scalar field option and declared by declare_scalar.
/// Scalars extend simple protobuf built-in types with additional
/// syntax and semantics, for instance to represent big integers.
/// Scalars should ideally define an encoding such that there is only one
/// valid syntactical representation for a given semantic meaning,
/// i.e. the encoding should be deterministic.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalarDescriptor {
    /// name is the name of the scalar. It should be a short-name (without
    /// a period) such that the fully qualified name of the scalar will be
    /// package.name, ex. for the package a.b and scalar named C, the
    /// fully-qualified name will be a.b.C.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description is a human-readable description of the scalar and its
    /// encoding format. For instance a big integer or decimal scalar should
    /// specify precisely the expected encoding format.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// field_type is the type of field with which this scalar can be used.
    /// Scalars can be used with one and only one type of field so that
    /// encoding standards and simple and clear. Currently only string and
    /// bytes fields are supported for scalars.
    #[prost(enumeration = "ScalarType", repeated, tag = "3")]
    pub field_type: ::prost::alloc::vec::Vec<i32>,
}
impl ::prost::Name for ScalarDescriptor {
    const NAME: &'static str = "ScalarDescriptor";
    const PACKAGE: &'static str = "cosmos_proto";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos_proto.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScalarType {
    Unspecified = 0,
    String = 1,
    Bytes = 2,
}
impl ScalarType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScalarType::Unspecified => "SCALAR_TYPE_UNSPECIFIED",
            ScalarType::String => "SCALAR_TYPE_STRING",
            ScalarType::Bytes => "SCALAR_TYPE_BYTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SCALAR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SCALAR_TYPE_STRING" => Some(Self::String),
            "SCALAR_TYPE_BYTES" => Some(Self::Bytes),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `cosmos_proto` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x94, 0x26, 0x0a, 0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4b,
    0x0a, 0x13, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0x81, 0x01, 0x0a, 0x10,
    0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x37, 0x0a, 0x0a, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x09, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x54, 0x79, 0x70, 0x65, 0x2a,
    0x58, 0x0a, 0x0a, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1b, 0x0a,
    0x17, 0x53, 0x43, 0x41, 0x4c, 0x41, 0x52, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53,
    0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x16, 0x0a, 0x12, 0x53, 0x43,
    0x41, 0x4c, 0x41, 0x52, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53, 0x54, 0x52, 0x49, 0x4e, 0x47,
    0x10, 0x01, 0x12, 0x15, 0x0a, 0x11, 0x53, 0x43, 0x41, 0x4c, 0x41, 0x52, 0x5f, 0x54, 0x59, 0x50,
    0x45, 0x5f, 0x42, 0x59, 0x54, 0x45, 0x53, 0x10, 0x02, 0x3a, 0x54, 0x0a, 0x14, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63,
    0x65, 0x12, 0x1f, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x18, 0xc9, 0xd6, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x13, 0x69, 0x6d, 0x70, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x3a,
    0x4c, 0x0a, 0x11, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72,
    0x66, 0x61, 0x63, 0x65, 0x12, 0x1d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x18, 0xc9, 0xd6, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x61, 0x63, 0x63,
    0x65, 0x70, 0x74, 0x73, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x3a, 0x37, 0x0a,
    0x06, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x1d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0xca, 0xd6, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x3a, 0x6e, 0x0a, 0x11, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72,
    0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x12, 0x1c, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69,
    0x6c, 0x65, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0xbd, 0xb3, 0x30, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x21, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x52, 0x10, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x49, 0x6e, 0x74,
    0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x3a, 0x65, 0x0a, 0x0e, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72,
    0x65, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x1c, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x4f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0xbe, 0xb3, 0x30, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x63,
    0x61, 0x6c, 0x61, 0x72, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x52, 0x0d,
    0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x42, 0x2d, 0x5a,
    0x2b, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3b,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x4a, 0xb0, 0x1f, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x60, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x03, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x42,
    0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x42, 0x0a, 0x09, 0x0a, 0x01, 0x07,
    0x12, 0x04, 0x07, 0x00, 0x0f, 0x01, 0x0a, 0xb6, 0x02, 0x0a, 0x02, 0x07, 0x00, 0x12, 0x03, 0x0e,
    0x04, 0x31, 0x1a, 0xaa, 0x02, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x0a, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x61, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x69, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x20, 0x41, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x70, 0x6c, 0x65, 0x0a, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x73, 0x2e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61,
    0x63, 0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65,
    0x63, 0x6c, 0x61, 0x72, 0x65, 0x64, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x64,
    0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x0a, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x07, 0x00, 0x02, 0x12, 0x03, 0x07, 0x07, 0x25, 0x0a, 0x0a, 0x0a, 0x03, 0x07,
    0x00, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x05, 0x12, 0x03,
    0x0e, 0x0d, 0x13, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x14, 0x28, 0x0a,
    0x0a, 0x0a, 0x03, 0x07, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x2b, 0x30, 0x0a, 0x09, 0x0a, 0x01, 0x07,
    0x12, 0x04, 0x11, 0x00, 0x1d, 0x01, 0x0a, 0xd4, 0x01, 0x0a, 0x02, 0x07, 0x01, 0x12, 0x03, 0x16,
    0x04, 0x25, 0x1a, 0xc8, 0x01, 0x20, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x73, 0x5f, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x61, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x61, 0x63,
    0x63, 0x65, 0x70, 0x74, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65,
    0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x0a, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63,
    0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x63,
    0x6c, 0x61, 0x72, 0x65, 0x64, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x64, 0x65,
    0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x07, 0x01, 0x02, 0x12, 0x03, 0x11, 0x07, 0x23, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x05,
    0x12, 0x03, 0x16, 0x04, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x01, 0x12, 0x03, 0x16, 0x0b,
    0x1c, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x01, 0x03, 0x12, 0x03, 0x16, 0x1f, 0x24, 0x0a, 0x96, 0x02,
    0x0a, 0x02, 0x07, 0x02, 0x12, 0x03, 0x1c, 0x04, 0x1a, 0x1a, 0x8a, 0x02, 0x20, 0x73, 0x63, 0x61,
    0x6c, 0x61, 0x72, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x64, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x63, 0x6c,
    0x61, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72,
    0x65, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x2e, 0x20, 0x43, 0x6f, 0x64, 0x65, 0x0a, 0x20,
    0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63,
    0x68, 0x6f, 0x6f, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f,
    0x20, 0x6d, 0x61, 0x70, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x0a, 0x20, 0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x2d, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63,
    0x61, 0x6c, 0x61, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02, 0x02, 0x12, 0x03, 0x11,
    0x07, 0x23, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x04, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x07, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x02,
    0x03, 0x12, 0x03, 0x1c, 0x14, 0x19, 0x0a, 0x09, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x1f, 0x00, 0x30,
    0x01, 0x0a, 0x91, 0x03, 0x0a, 0x02, 0x07, 0x03, 0x12, 0x03, 0x27, 0x04, 0x3c, 0x1a, 0x85, 0x03,
    0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61,
    0x63, 0x65, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x61,
    0x63, 0x63, 0x65, 0x70, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x5f,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x66, 0x61, 0x63, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20,
    0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x6f, 0x6c, 0x6c,
    0x6f, 0x77, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67,
    0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x75, 0x63, 0x68,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20, 0x64, 0x65, 0x63, 0x6c,
    0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x6f,
    0x6f, 0x6c, 0x73, 0x3a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x61, 0x2e, 0x62, 0x2e, 0x43, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x65, 0x78,
    0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x64, 0x0a, 0x20, 0x61, 0x2f, 0x62, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63,
    0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20,
    0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x03, 0x02, 0x12, 0x03, 0x1f, 0x07,
    0x22, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x03, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0a, 0x0a,
    0x03, 0x07, 0x03, 0x06, 0x12, 0x03, 0x27, 0x0d, 0x20, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x03, 0x01,
    0x12, 0x03, 0x27, 0x21, 0x32, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x03, 0x03, 0x12, 0x03, 0x27, 0x35,
    0x3b, 0x0a, 0xee, 0x02, 0x0a, 0x02, 0x07, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x36, 0x1a, 0xe2, 0x02,
    0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20,
    0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61,
    0x72, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x61, 0x6c,
    0x61, 0x72, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x20, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x0a, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x66,
    0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77,
    0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x6e, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73,
    0x75, 0x63, 0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20, 0x64,
    0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x62, 0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x74, 0x6f, 0x6f, 0x6c, 0x73, 0x3a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x67, 0x69,
    0x76, 0x65, 0x6e, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x61, 0x2e, 0x62, 0x2e, 0x43, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x65, 0x78,
    0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x64, 0x0a, 0x20, 0x61, 0x2f, 0x62, 0x2f, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c,
    0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x73, 0x65, 0x74,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x04, 0x02, 0x12, 0x03, 0x1f, 0x07, 0x22, 0x0a, 0x0a,
    0x0a, 0x03, 0x07, 0x04, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x04,
    0x06, 0x12, 0x03, 0x2f, 0x0d, 0x1d, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x04, 0x01, 0x12, 0x03, 0x2f,
    0x1e, 0x2c, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x04, 0x03, 0x12, 0x03, 0x2f, 0x2f, 0x35, 0x0a, 0x9d,
    0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x34, 0x00, 0x3f, 0x01, 0x1a, 0x90, 0x01, 0x20, 0x49,
    0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20,
    0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x73, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63,
    0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x64,
    0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61,
    0x72, 0x65, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1b, 0x0a, 0x83, 0x02, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x3a, 0x04, 0x14, 0x1a, 0xf5, 0x01, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x2e, 0x20, 0x49,
    0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20, 0x73, 0x68,
    0x6f, 0x72, 0x74, 0x2d, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x28, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75,
    0x74, 0x0a, 0x20, 0x61, 0x20, 0x70, 0x65, 0x72, 0x69, 0x6f, 0x64, 0x29, 0x20, 0x73, 0x75, 0x63,
    0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x79,
    0x20, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67,
    0x65, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x2e, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20, 0x61, 0x2e, 0x62, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x64, 0x20, 0x43, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x75, 0x6c, 0x6c,
    0x79, 0x2d, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x2e, 0x62, 0x2e, 0x43, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x12, 0x13, 0x0a, 0x5d, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x3e, 0x04, 0x1b, 0x1a, 0x50, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x68, 0x75, 0x6d, 0x61, 0x6e,
    0x2d, 0x72, 0x65, 0x61, 0x64, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74, 0x73, 0x0a, 0x20,
    0x70, 0x75, 0x72, 0x70, 0x6f, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x3e, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x3e, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x3e, 0x19, 0x1a, 0x0a, 0xb2, 0x03, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x48, 0x00, 0x5a, 0x01,
    0x1a, 0xa5, 0x03, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61,
    0x6e, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61,
    0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x64, 0x65, 0x63, 0x6c, 0x61, 0x72, 0x65, 0x5f, 0x73,
    0x63, 0x61, 0x6c, 0x61, 0x72, 0x2e, 0x0a, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x73, 0x20,
    0x65, 0x78, 0x74, 0x65, 0x6e, 0x64, 0x20, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x62, 0x75, 0x69, 0x6c, 0x74, 0x2d, 0x69, 0x6e, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x0a, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x78, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x73, 0x2c, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x69, 0x67, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67,
    0x65, 0x72, 0x73, 0x2e, 0x0a, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x73, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x69, 0x64, 0x65, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x73, 0x75, 0x63, 0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x20, 0x76, 0x61, 0x6c,
    0x69, 0x64, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x61, 0x63, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x20, 0x72,
    0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74,
    0x69, 0x63, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x69, 0x6e, 0x67, 0x2c, 0x0a, 0x20, 0x69, 0x2e, 0x65,
    0x2e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69,
    0x6e, 0x69, 0x73, 0x74, 0x69, 0x63, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x48, 0x08, 0x18, 0x0a, 0xfa, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x4e,
    0x04, 0x14, 0x1a, 0xec, 0x01, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63,
    0x61, 0x6c, 0x61, 0x72, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x2d, 0x6e, 0x61, 0x6d, 0x65, 0x20,
    0x28, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x0a, 0x20, 0x61, 0x20, 0x70, 0x65, 0x72, 0x69,
    0x6f, 0x64, 0x29, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x79, 0x20, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63,
    0x61, 0x6c, 0x61, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x70, 0x61,
    0x63, 0x6b, 0x61, 0x67, 0x65, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x2c, 0x20, 0x65, 0x78, 0x2e, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x20,
    0x61, 0x2e, 0x62, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x64, 0x20, 0x43, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x75, 0x6c,
    0x6c, 0x79, 0x2d, 0x71, 0x75, 0x61, 0x6c, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x2e, 0x62, 0x2e, 0x43, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4e, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x12, 0x13, 0x0a, 0xc8, 0x01, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x53, 0x04, 0x1b, 0x1a, 0xba, 0x01, 0x20, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x68, 0x75,
    0x6d, 0x61, 0x6e, 0x2d, 0x72, 0x65, 0x61, 0x64, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74, 0x73, 0x0a, 0x20,
    0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x61, 0x20,
    0x62, 0x69, 0x67, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x64,
    0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x70, 0x72,
    0x65, 0x63, 0x69, 0x73, 0x65, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x70, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x53, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x53,
    0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x53, 0x19, 0x1a,
    0x0a, 0x81, 0x02, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x59, 0x04, 0x27, 0x1a, 0xf3,
    0x01, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x73, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x73,
    0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73,
    0x69, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x2e,
    0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x73, 0x75, 0x70,
    0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61,
    0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x59,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x59, 0x0d, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x59, 0x18, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x59, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x00, 0x12, 0x04, 0x5c, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12,
    0x03, 0x5c, 0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x04, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x5d, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x5e, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x5e, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x5f, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x04,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5f, 0x18, 0x19, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)