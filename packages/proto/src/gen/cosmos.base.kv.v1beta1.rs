// @generated
/// Pairs defines a repeated slice of Pair objects.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
impl ::prost::Name for Pairs {
    const NAME: &'static str = "Pairs";
    const PACKAGE: &'static str = "cosmos.base.kv.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.kv.v1beta1.{}", Self::NAME)
    }
}
/// Pair defines a key/value bytes tuple.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Pair {
    const NAME: &'static str = "Pair";
    const PACKAGE: &'static str = "cosmos.base.kv.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.kv.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.base.kv.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x90, 0x05, 0x0a, 0x1f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x6b, 0x76, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x6b, 0x76, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73,
    0x65, 0x2e, 0x6b, 0x76, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f,
    0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x41, 0x0a, 0x05, 0x50, 0x61, 0x69, 0x72, 0x73, 0x12, 0x38, 0x0a, 0x05, 0x70,
    0x61, 0x69, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x6b, 0x76, 0x2e, 0x76, 0x31, 0x62, 0x65,
    0x74, 0x61, 0x31, 0x2e, 0x50, 0x61, 0x69, 0x72, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x05,
    0x70, 0x61, 0x69, 0x72, 0x73, 0x22, 0x2e, 0x0a, 0x04, 0x50, 0x61, 0x69, 0x72, 0x12, 0x10, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
    0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x27, 0x5a, 0x25, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x6b, 0x76, 0x4a, 0x9a,
    0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1f, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x1e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05,
    0x00, 0x3c, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x3c, 0x0a, 0x3d, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x1a, 0x31, 0x20, 0x50, 0x61, 0x69, 0x72,
    0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x70, 0x65,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x73, 0x6c, 0x69, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x50, 0x61,
    0x69, 0x72, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x09, 0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09, 0x0b,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x10, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09, 0x18, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x09, 0x1a, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x00, 0x02, 0x00, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x09, 0x1b, 0x37, 0x0a, 0x33, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x10, 0x01, 0x1a, 0x27, 0x20, 0x50, 0x61, 0x69, 0x72, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6b, 0x65, 0x79, 0x2f, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x74, 0x75, 0x70, 0x6c, 0x65, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x08, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x0e, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x0e, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x10, 0x11, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
