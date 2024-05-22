// @generated
/// Module is the config object of the capability module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// seal_keeper defines if keeper.Seal() will run on BeginBlock() to prevent further modules from creating a scoped
    /// keeper. For more details check x/capability/keeper.go.
    #[prost(bool, tag = "1")]
    pub seal_keeper: bool,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.capability.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.capability.module.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.capability.module.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xdd, 0x04, 0x0a, 0x28, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x61, 0x70, 0x61,
    0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31,
    0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x63,
    0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79,
    0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x20, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x61, 0x70, 0x70, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x31, 0x2f,
    0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5c, 0x0a, 0x06,
    0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x73, 0x65, 0x61, 0x6c, 0x5f, 0x6b,
    0x65, 0x65, 0x70, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x73, 0x65, 0x61,
    0x6c, 0x4b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x3a, 0x31, 0xba, 0xc0, 0x96, 0xda, 0x01, 0x2b, 0x0a,
    0x29, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f,
    0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x4a, 0x8b, 0x03, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x24, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x00, 0x2a, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0f, 0x01,
    0x1a, 0x37, 0x20, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79,
    0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x08, 0x02,
    0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x12, 0x04, 0x08,
    0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x01, 0x12,
    0x03, 0x09, 0x04, 0x3a, 0x0a, 0xb7, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e,
    0x02, 0x17, 0x1a, 0xa9, 0x01, 0x20, 0x73, 0x65, 0x61, 0x6c, 0x5f, 0x6b, 0x65, 0x65, 0x70, 0x65,
    0x72, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x69, 0x66, 0x20, 0x6b, 0x65, 0x65,
    0x70, 0x65, 0x72, 0x2e, 0x53, 0x65, 0x61, 0x6c, 0x28, 0x29, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x72, 0x75, 0x6e, 0x20, 0x6f, 0x6e, 0x20, 0x42, 0x65, 0x67, 0x69, 0x6e, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x28, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x66,
    0x75, 0x72, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73,
    0x63, 0x6f, 0x70, 0x65, 0x64, 0x0a, 0x20, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x2e, 0x20, 0x46,
    0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x20,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x78, 0x2f, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69,
    0x74, 0x79, 0x2f, 0x6b, 0x65, 0x65, 0x70, 0x65, 0x72, 0x2e, 0x67, 0x6f, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x15, 0x16, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("cosmos.capability.module.v1.serde.rs");
// @@protoc_insertion_point(module)
