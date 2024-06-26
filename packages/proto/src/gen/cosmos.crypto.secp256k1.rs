// @generated
/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "cosmos.crypto.secp256k1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.secp256k1.{}", Self::NAME)
    }
}
/// PrivKey defines a secp256k1 private key.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivKey {
    const NAME: &'static str = "PrivKey";
    const PACKAGE: &'static str = "cosmos.crypto.secp256k1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.secp256k1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.crypto.secp256k1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x8c, 0x0a, 0x0a, 0x22, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x6f, 0x2f, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x2f, 0x6b, 0x65, 0x79,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2e, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31,
    0x1a, 0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2f, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67,
    0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4d, 0x0a, 0x06, 0x50, 0x75, 0x62,
    0x4b, 0x65, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x3a, 0x31, 0x98, 0xa0, 0x1f, 0x00, 0x8a, 0xe7, 0xb0, 0x2a, 0x1a,
    0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2f, 0x50, 0x75, 0x62, 0x4b, 0x65,
    0x79, 0x53, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x92, 0xe7, 0xb0, 0x2a, 0x09, 0x6b,
    0x65, 0x79, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x22, 0x4b, 0x0a, 0x07, 0x50, 0x72, 0x69, 0x76,
    0x4b, 0x65, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x3a, 0x2e, 0x8a, 0xe7, 0xb0, 0x2a, 0x1b, 0x74, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2f, 0x50, 0x72, 0x69, 0x76, 0x4b, 0x65, 0x79, 0x53, 0x65,
    0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x92, 0xe7, 0xb0, 0x2a, 0x09, 0x6b, 0x65, 0x79, 0x5f,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x42, 0x34, 0x5a, 0x32, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
    0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x2f, 0x6b, 0x65, 0x79,
    0x73, 0x2f, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x4a, 0xc9, 0x07, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x25, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x03, 0x00, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x1e,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x49, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b,
    0x12, 0x03, 0x06, 0x00, 0x49, 0x0a, 0xbd, 0x02, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0d, 0x00,
    0x1d, 0x01, 0x1a, 0xb0, 0x02, 0x20, 0x50, 0x75, 0x62, 0x4b, 0x65, 0x79, 0x20, 0x64, 0x65, 0x66,
    0x69, 0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31,
    0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x4b, 0x65, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x75, 0x62, 0x6b, 0x65, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x30, 0x78, 0x30, 0x32, 0x20, 0x62, 0x79, 0x74, 0x65, 0x0a, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x79, 0x2d, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x78, 0x69, 0x63, 0x6f, 0x67,
    0x72, 0x61, 0x70, 0x68, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65,
    0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x61, 0x73,
    0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x78, 0x2d, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x2e,
    0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x69, 0x72, 0x73, 0x74, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x30,
    0x78, 0x30, 0x33, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69,
    0x78, 0x20, 0x69, 0x73, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2d, 0x63, 0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e,
    0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x08,
    0x0e, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x0e, 0x02, 0x35, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x07, 0xf1, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x35, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x19, 0x02, 0x34, 0x0a, 0x86, 0x02, 0x0a, 0x07, 0x04, 0x00,
    0x07, 0xf2, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x19, 0x02, 0x34, 0x1a, 0xf5, 0x01, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x41, 0x6d, 0x69, 0x6e, 0x6f, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x69, 0x73, 0x20, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69,
    0x6e, 0x6e, 0x65, 0x72, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64,
    0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6d,
    0x69, 0x6e, 0x6f, 0x0a, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x68, 0x6f, 0x6c, 0x65, 0x20, 0x50, 0x75, 0x62, 0x4b, 0x65,
    0x79, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x78, 0x61, 0x6d,
    0x70, 0x6c, 0x65, 0x20, 0x28, 0x4a, 0x53, 0x4f, 0x4e, 0x29, 0x3a, 0x0a, 0x20, 0x73, 0x20, 0x3a,
    0x3d, 0x20, 0x50, 0x75, 0x62, 0x4b, 0x65, 0x79, 0x7b, 0x4b, 0x65, 0x79, 0x3a, 0x20, 0x5b, 0x5d,
    0x62, 0x79, 0x74, 0x65, 0x7b, 0x30, 0x78, 0x30, 0x31, 0x7d, 0x7d, 0x0a, 0x20, 0x6f, 0x75, 0x74,
    0x20, 0x3a, 0x3d, 0x20, 0x41, 0x6d, 0x69, 0x6e, 0x6f, 0x4a, 0x53, 0x4f, 0x4e, 0x45, 0x6e, 0x63,
    0x6f, 0x64, 0x65, 0x72, 0x28, 0x73, 0x29, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x6e, 0x20, 0x77,
    0x65, 0x20, 0x68, 0x61, 0x76, 0x65, 0x3a, 0x0a, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x3d, 0x3d, 0x20,
    0x60, 0x22, 0x4d, 0x51, 0x3d, 0x3d, 0x22, 0x60, 0x0a, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x21, 0x3d,
    0x20, 0x60, 0x7b, 0x22, 0x6b, 0x65, 0x79, 0x22, 0x3a, 0x22, 0x4d, 0x51, 0x3d, 0x3d, 0x22, 0x7d,
    0x60, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x1a, 0x02, 0x2e, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x1a, 0x02, 0x2e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1c, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1c, 0x0e, 0x0f, 0x0a, 0x36, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x20, 0x00, 0x25, 0x01,
    0x1a, 0x2a, 0x20, 0x50, 0x72, 0x69, 0x76, 0x4b, 0x65, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x65, 0x63, 0x70, 0x32, 0x35, 0x36, 0x6b, 0x31, 0x20, 0x70,
    0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12,
    0x03, 0x21, 0x02, 0x42, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x07, 0xf1, 0x8c, 0xa6, 0x05, 0x12,
    0x03, 0x21, 0x02, 0x42, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07, 0x12, 0x03, 0x22, 0x02, 0x30,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x07, 0xf2, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x22, 0x02, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x24, 0x0e, 0x0f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
