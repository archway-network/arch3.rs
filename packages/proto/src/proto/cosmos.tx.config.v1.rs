// @generated
/// Config is the config object of the x/auth/tx package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// skip_ante_handler defines whether the ante handler registration should be skipped in case an app wants to override
    /// this functionality.
    #[prost(bool, tag = "1")]
    pub skip_ante_handler: bool,
    /// skip_post_handler defines whether the post handler registration should be skipped in case an app wants to override
    /// this functionality.
    #[prost(bool, tag = "2")]
    pub skip_post_handler: bool,
}
impl ::prost::Name for Config {
    const NAME: &'static str = "Config";
    const PACKAGE: &'static str = "cosmos.tx.config.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.tx.config.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.tx.config.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa6, 0x06, 0x0a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x74, 0x78, 0x2f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x74, 0x78,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x1a, 0x20, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x61, 0x70, 0x70, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f,
    0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x90, 0x01, 0x0a,
    0x06, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x2a, 0x0a, 0x11, 0x73, 0x6b, 0x69, 0x70, 0x5f,
    0x61, 0x6e, 0x74, 0x65, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x08, 0x52, 0x0f, 0x73, 0x6b, 0x69, 0x70, 0x41, 0x6e, 0x74, 0x65, 0x48, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x72, 0x12, 0x2a, 0x0a, 0x11, 0x73, 0x6b, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x73, 0x74,
    0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0f,
    0x73, 0x6b, 0x69, 0x70, 0x50, 0x6f, 0x73, 0x74, 0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x3a,
    0x2e, 0xba, 0xc0, 0x96, 0xda, 0x01, 0x28, 0x0a, 0x26, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x2f, 0x74, 0x78, 0x4a,
    0xaf, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x13, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x07, 0x00, 0x13, 0x01, 0x1a, 0x37, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x6f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2f, 0x61, 0x75, 0x74,
    0x68, 0x2f, 0x74, 0x78, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00,
    0x07, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x07, 0x87, 0xe8,
    0xa2, 0x1b, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x07, 0x87,
    0xe8, 0xa2, 0x1b, 0x01, 0x12, 0x03, 0x09, 0x04, 0x37, 0x0a, 0x97, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x1d, 0x1a, 0x89, 0x01, 0x20, 0x73, 0x6b, 0x69, 0x70, 0x5f,
    0x61, 0x6e, 0x74, 0x65, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x61, 0x6e, 0x74, 0x65, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72, 0x20, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x6b, 0x69, 0x70, 0x70, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x63, 0x61, 0x73, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x70, 0x70, 0x20, 0x77, 0x61, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64, 0x65, 0x0a, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x69, 0x74,
    0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x1b, 0x1c, 0x0a, 0x97, 0x01,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x1d, 0x1a, 0x89, 0x01, 0x20, 0x73,
    0x6b, 0x69, 0x70, 0x5f, 0x70, 0x6f, 0x73, 0x74, 0x5f, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x72,
    0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x6f, 0x73, 0x74, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65,
    0x72, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x6b, 0x69, 0x70, 0x70, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x63, 0x61, 0x73, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x70, 0x70, 0x20,
    0x77, 0x61, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x72, 0x69, 0x64,
    0x65, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x12, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x12, 0x07, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x12, 0x1b,
    0x1c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)