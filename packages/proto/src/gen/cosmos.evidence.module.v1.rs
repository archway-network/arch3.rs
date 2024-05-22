// @generated
/// Module is the config object of the evidence module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.evidence.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.evidence.module.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.evidence.module.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd0, 0x02, 0x0a, 0x26, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x65, 0x76, 0x69, 0x64,
    0x65, 0x6e, 0x63, 0x65, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6d,
    0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x19, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2e, 0x65, 0x76, 0x69, 0x64, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x6d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x61,
    0x70, 0x70, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x39, 0x0a, 0x06, 0x4d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x3a, 0x2f, 0xba, 0xc0, 0x96, 0xda, 0x01, 0x29, 0x0a, 0x27, 0x67, 0x69, 0x74, 0x68,
    0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f,
    0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x65, 0x76, 0x69, 0x64, 0x65,
    0x6e, 0x63, 0x65, 0x4a, 0xa5, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0b, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x41, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0b, 0x01, 0x1a, 0x35, 0x20, 0x4d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x76, 0x69, 0x64, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x00, 0x07, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x07,
    0x87, 0xe8, 0xa2, 0x1b, 0x12, 0x04, 0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00,
    0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x01, 0x12, 0x03, 0x09, 0x04, 0x38, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
