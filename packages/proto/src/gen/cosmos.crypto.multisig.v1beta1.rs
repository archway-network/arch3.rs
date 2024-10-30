// @generated
/// MultiSignature wraps the signatures from a multisig.LegacyAminoPubKey.
/// See cosmos.tx.v1betata1.ModeInfo.Multi for how to specify which signers
/// signed and with which modes.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiSignature {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
impl ::prost::Name for MultiSignature {
    const NAME: &'static str = "MultiSignature";
    const PACKAGE: &'static str = "cosmos.crypto.multisig.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.multisig.v1beta1.{}", Self::NAME)
    }
}
/// CompactBitArray is an implementation of a space efficient bit array.
/// This is used to ensure that the encoded data takes up a minimal amount of
/// space after proto encoding.
/// This is not thread safe, and is not intended for concurrent usage.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactBitArray {
    #[prost(uint32, tag = "1")]
    pub extra_bits_stored: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub elems: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for CompactBitArray {
    const NAME: &'static str = "CompactBitArray";
    const PACKAGE: &'static str = "cosmos.crypto.multisig.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.multisig.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.crypto.multisig.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xae, 0x08, 0x0a, 0x2d, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x73, 0x69, 0x67, 0x2f, 0x76, 0x31, 0x62, 0x65,
    0x74, 0x61, 0x31, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x73, 0x69, 0x67, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x6f, 0x2e, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x73, 0x69, 0x67, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74,
    0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f,
    0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x36, 0x0a, 0x0e, 0x4d, 0x75, 0x6c, 0x74,
    0x69, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x12, 0x1e, 0x0a, 0x0a, 0x73, 0x69,
    0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x0a,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x3a, 0x04, 0xd0, 0xa1, 0x1f, 0x01,
    0x22, 0x59, 0x0a, 0x0f, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x42, 0x69, 0x74, 0x41, 0x72,
    0x72, 0x61, 0x79, 0x12, 0x2a, 0x0a, 0x11, 0x65, 0x78, 0x74, 0x72, 0x61, 0x5f, 0x62, 0x69, 0x74,
    0x73, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0f,
    0x65, 0x78, 0x74, 0x72, 0x61, 0x42, 0x69, 0x74, 0x73, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x12,
    0x14, 0x0a, 0x05, 0x65, 0x6c, 0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05,
    0x65, 0x6c, 0x65, 0x6d, 0x73, 0x3a, 0x04, 0x98, 0xa0, 0x1f, 0x00, 0x42, 0x2b, 0x5a, 0x29, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xfe, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x18, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x00, 0x1e, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x40, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x40, 0x0a, 0xbc, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x0a, 0x00, 0x0d, 0x01, 0x1a, 0xaf, 0x01, 0x20, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x77, 0x72, 0x61, 0x70, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x61, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x73, 0x69, 0x67, 0x2e, 0x4c, 0x65, 0x67, 0x61,
    0x63, 0x79, 0x41, 0x6d, 0x69, 0x6e, 0x6f, 0x50, 0x75, 0x62, 0x4b, 0x65, 0x79, 0x2e, 0x0a, 0x20,
    0x53, 0x65, 0x65, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x74, 0x78, 0x2e, 0x76, 0x31,
    0x62, 0x65, 0x74, 0x61, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x2e, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x74,
    0x6f, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x73, 0x69, 0x67, 0x6e, 0x65, 0x72, 0x73, 0x0a, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d,
    0x6f, 0x64, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x0b, 0x02, 0x31, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x07, 0x9a, 0xf4, 0x03, 0x12, 0x03, 0x0b, 0x02, 0x31, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0c, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c,
    0x2c, 0x2d, 0x0a, 0xff, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x13, 0x00, 0x18, 0x01, 0x1a,
    0xf2, 0x01, 0x20, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x42, 0x69, 0x74, 0x41, 0x72, 0x72,
    0x61, 0x79, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x73, 0x70, 0x61,
    0x63, 0x65, 0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x69, 0x74,
    0x20, 0x61, 0x72, 0x72, 0x61, 0x79, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x73, 0x75, 0x72, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x73, 0x20, 0x75, 0x70, 0x20, 0x61,
    0x20, 0x6d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x20, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20,
    0x6f, 0x66, 0x0a, 0x20, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x2e, 0x0a,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x72,
    0x65, 0x61, 0x64, 0x20, 0x73, 0x61, 0x66, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x75, 0x73, 0x61,
    0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x13, 0x08, 0x17,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x03, 0x14, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x01, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x14, 0x02, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x16, 0x09, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16,
    0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
// @@protoc_insertion_point(module)