// @generated
/// Module is the config object of the gov module.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// max_metadata_len defines the maximum proposal metadata length.
    /// Defaults to 255 if not explicitly set.
    #[prost(uint64, tag = "1")]
    pub max_metadata_len: u64,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.gov.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.gov.module.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.gov.module.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xbe, 0x05, 0x0a, 0x21, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x67, 0x6f, 0x76, 0x2f,
    0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x67,
    0x6f, 0x76, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x20, 0x63, 0x6f,
    0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x61, 0x70, 0x70, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x31, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7c,
    0x0a, 0x06, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x12, 0x28, 0x0a, 0x10, 0x6d, 0x61, 0x78, 0x5f,
    0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6c, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0e, 0x6d, 0x61, 0x78, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x4c,
    0x65, 0x6e, 0x12, 0x1c, 0x0a, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x3a, 0x2a, 0xba, 0xc0, 0x96, 0xda, 0x01, 0x24, 0x0a, 0x22, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x67, 0x6f, 0x76, 0x4a, 0xda, 0x03, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x12, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1d, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00,
    0x12, 0x01, 0x1a, 0x30, 0x20, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x20, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x01, 0x12, 0x03, 0x09, 0x04, 0x33, 0x0a,
    0x76, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1e, 0x1a, 0x69, 0x20, 0x6d,
    0x61, 0x78, 0x5f, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x5f, 0x6c, 0x65, 0x6e, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x69,
    0x6d, 0x75, 0x6d, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x61, 0x6c, 0x20, 0x6d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x2e, 0x20, 0x0a, 0x20,
    0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x32, 0x35, 0x35, 0x20,
    0x69, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x6c,
    0x79, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0e, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x1c,
    0x1d, 0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x17, 0x1a, 0x5f,
    0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x6d, 0x6f,
    0x64, 0x75, 0x6c, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x64, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65,
    0x72, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
// @@protoc_insertion_point(module)
