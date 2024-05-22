// @generated
/// MerkleRoot defines a merkle root hash.
/// In the Cosmos SDK, the AppHash of a block header becomes the root.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleRoot {
    #[prost(bytes = "vec", tag = "1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MerkleRoot {
    const NAME: &'static str = "MerkleRoot";
    const PACKAGE: &'static str = "ibc.core.commitment.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.commitment.v1.{}", Self::NAME)
    }
}
/// MerklePrefix is merkle path prefixed to the key.
/// The constructed key from the Path and the key will be append(Path.KeyPath,
/// append(Path.KeyPrefix, key...))
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePrefix {
    #[prost(bytes = "vec", tag = "1")]
    pub key_prefix: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MerklePrefix {
    const NAME: &'static str = "MerklePrefix";
    const PACKAGE: &'static str = "ibc.core.commitment.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.commitment.v1.{}", Self::NAME)
    }
}
/// MerklePath is the path used to verify commitment proofs, which can be an
/// arbitrary structured object (defined by a commitment type).
/// MerklePath is represented from root-to-leaf
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(string, repeated, tag = "1")]
    pub key_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MerklePath {
    const NAME: &'static str = "MerklePath";
    const PACKAGE: &'static str = "ibc.core.commitment.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.commitment.v1.{}", Self::NAME)
    }
}
/// MerkleProof is a wrapper type over a chain of CommitmentProofs.
/// It demonstrates membership or non-membership for an element or set of
/// elements, verifiable in conjunction with a known commitment root. Proofs
/// should be succinct.
/// MerkleProofs are ordered from leaf-to-root
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleProof {
    #[prost(message, repeated, tag = "1")]
    pub proofs:
        ::prost::alloc::vec::Vec<super::super::super::super::cosmos::ics23::v1::CommitmentProof>,
}
impl ::prost::Name for MerkleProof {
    const NAME: &'static str = "MerkleProof";
    const PACKAGE: &'static str = "ibc.core.commitment.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("ibc.core.commitment.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `ibc.core.commitment.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xae, 0x0d, 0x0a, 0x27, 0x69, 0x62, 0x63, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x63, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x69, 0x62,
    0x63, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e,
    0x74, 0x2e, 0x76, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f,
    0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2f, 0x69, 0x63, 0x73, 0x32, 0x33, 0x2f, 0x76, 0x31, 0x2f, 0x70, 0x72, 0x6f, 0x6f,
    0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x26, 0x0a, 0x0a, 0x4d, 0x65, 0x72, 0x6b,
    0x6c, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x3a, 0x04, 0x88, 0xa0, 0x1f, 0x00,
    0x22, 0x44, 0x0a, 0x0c, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78,
    0x12, 0x34, 0x0a, 0x0a, 0x6b, 0x65, 0x79, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x42, 0x15, 0xf2, 0xde, 0x1f, 0x11, 0x79, 0x61, 0x6d, 0x6c, 0x3a, 0x22,
    0x6b, 0x65, 0x79, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x22, 0x52, 0x09, 0x6b, 0x65, 0x79,
    0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x22, 0x42, 0x0a, 0x0a, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65,
    0x50, 0x61, 0x74, 0x68, 0x12, 0x2e, 0x0a, 0x08, 0x6b, 0x65, 0x79, 0x5f, 0x70, 0x61, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x42, 0x13, 0xf2, 0xde, 0x1f, 0x0f, 0x79, 0x61, 0x6d, 0x6c,
    0x3a, 0x22, 0x6b, 0x65, 0x79, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x22, 0x52, 0x07, 0x6b, 0x65, 0x79,
    0x50, 0x61, 0x74, 0x68, 0x3a, 0x04, 0x98, 0xa0, 0x1f, 0x00, 0x22, 0x47, 0x0a, 0x0b, 0x4d, 0x65,
    0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x12, 0x38, 0x0a, 0x06, 0x70, 0x72, 0x6f,
    0x6f, 0x66, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2e, 0x69, 0x63, 0x73, 0x32, 0x33, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x52, 0x06, 0x70, 0x72, 0x6f,
    0x6f, 0x66, 0x73, 0x42, 0x3e, 0x5a, 0x3c, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x69, 0x62, 0x63, 0x2d, 0x67, 0x6f, 0x2f,
    0x76, 0x37, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x73, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f,
    0x32, 0x33, 0x2d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x2f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x4a, 0xf3, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x28, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02,
    0x00, 0x1f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00, 0x53, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x04, 0x00, 0x53, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x06,
    0x00, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x07, 0x00, 0x26, 0x0a, 0x78, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x1a, 0x6c, 0x20, 0x4d, 0x65, 0x72, 0x6b,
    0x6c, 0x65, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x61,
    0x20, 0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x20, 0x68, 0x61, 0x73,
    0x68, 0x2e, 0x0a, 0x20, 0x49, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x20, 0x53, 0x44, 0x4b, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x70, 0x70, 0x48, 0x61,
    0x73, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x20, 0x62, 0x65, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x6f, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x0b, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x0c, 0x02, 0x2d, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x07, 0x81, 0xf4, 0x03, 0x12, 0x03, 0x0c, 0x02, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0e, 0x0f, 0x10, 0x0a, 0xac, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x14, 0x00,
    0x16, 0x01, 0x1a, 0x9f, 0x01, 0x20, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x20, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x75, 0x63, 0x74, 0x65, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x50, 0x61, 0x74, 0x68, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70,
    0x70, 0x65, 0x6e, 0x64, 0x28, 0x50, 0x61, 0x74, 0x68, 0x2e, 0x4b, 0x65, 0x79, 0x50, 0x61, 0x74,
    0x68, 0x2c, 0x0a, 0x20, 0x61, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x28, 0x50, 0x61, 0x74, 0x68, 0x2e,
    0x4b, 0x65, 0x79, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78, 0x2c, 0x20, 0x6b, 0x65, 0x79, 0x2e, 0x2e,
    0x2e, 0x29, 0x29, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x46, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x15, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x15, 0x17, 0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xee, 0xfb,
    0x03, 0x12, 0x03, 0x15, 0x18, 0x44, 0x0a, 0xc1, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1b,
    0x00, 0x1f, 0x01, 0x1a, 0xb4, 0x01, 0x20, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x61, 0x74,
    0x68, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x6f, 0x66, 0x73, 0x2c, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x0a,
    0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x28, 0x64, 0x65,
    0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x29, 0x2e, 0x0a, 0x20, 0x4d, 0x65,
    0x72, 0x6b, 0x6c, 0x65, 0x50, 0x61, 0x74, 0x68, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x72, 0x6f, 0x6f,
    0x74, 0x2d, 0x74, 0x6f, 0x2d, 0x6c, 0x65, 0x61, 0x66, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x1b, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x07, 0x12, 0x03, 0x1c,
    0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x07, 0x83, 0xf4, 0x03, 0x12, 0x03, 0x1c, 0x02,
    0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x02, 0x4c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x1e, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x1e, 0x1f, 0x4b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xee, 0xfb, 0x03,
    0x12, 0x03, 0x1e, 0x20, 0x4a, 0x0a, 0xa0, 0x02, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x26, 0x00,
    0x28, 0x01, 0x1a, 0x93, 0x02, 0x20, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x6f,
    0x66, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61, 0x69, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x50, 0x72,
    0x6f, 0x6f, 0x66, 0x73, 0x2e, 0x0a, 0x20, 0x49, 0x74, 0x20, 0x64, 0x65, 0x6d, 0x6f, 0x6e, 0x73,
    0x74, 0x72, 0x61, 0x74, 0x65, 0x73, 0x20, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6c, 0x65, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x65, 0x6c,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2c, 0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x69, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x6a, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x6f, 0x6f, 0x74, 0x2e, 0x20,
    0x50, 0x72, 0x6f, 0x6f, 0x66, 0x73, 0x0a, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x75, 0x63, 0x63, 0x69, 0x6e, 0x63, 0x74, 0x2e, 0x0a, 0x20, 0x4d, 0x65, 0x72,
    0x6b, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x6f, 0x66, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x72,
    0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6c, 0x65, 0x61, 0x66, 0x2d,
    0x74, 0x6f, 0x2d, 0x72, 0x6f, 0x6f, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x26, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02,
    0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x27, 0x0b, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x2b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x34, 0x35, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("ibc.core.commitment.v1.serde.rs");
// @@protoc_insertion_point(module)
