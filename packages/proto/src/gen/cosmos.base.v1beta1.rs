// @generated
/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for Coin {
    const NAME: &'static str = "Coin";
    const PACKAGE: &'static str = "cosmos.base.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.v1beta1.{}", Self::NAME)
    }
}
/// DecCoin defines a token with a denomination and a decimal amount.
///
/// NOTE: The amount field is an Dec which implements the custom method
/// signatures required by gogoproto.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecCoin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for DecCoin {
    const NAME: &'static str = "DecCoin";
    const PACKAGE: &'static str = "cosmos.base.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.v1beta1.{}", Self::NAME)
    }
}
/// IntProto defines a Protobuf wrapper around an Int object.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntProto {
    #[prost(string, tag = "1")]
    pub int: ::prost::alloc::string::String,
}
impl ::prost::Name for IntProto {
    const NAME: &'static str = "IntProto";
    const PACKAGE: &'static str = "cosmos.base.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.v1beta1.{}", Self::NAME)
    }
}
/// DecProto defines a Protobuf wrapper around a Dec object.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecProto {
    #[prost(string, tag = "1")]
    pub dec: ::prost::alloc::string::String,
}
impl ::prost::Name for DecProto {
    const NAME: &'static str = "DecProto";
    const PACKAGE: &'static str = "cosmos.base.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.base.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa2, 0x0e, 0x0a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63, 0x6f, 0x69, 0x6e, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x13, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65,
    0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x73,
    0x6d, 0x6f, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f,
    0x2f, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5a, 0x0a, 0x04,
    0x43, 0x6f, 0x69, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x36, 0x0a, 0x06, 0x61, 0x6d,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x1e, 0xc8, 0xde, 0x1f, 0x00,
    0xda, 0xde, 0x1f, 0x03, 0x49, 0x6e, 0x74, 0xd2, 0xb4, 0x2d, 0x0a, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2e, 0x49, 0x6e, 0x74, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75,
    0x6e, 0x74, 0x3a, 0x04, 0xe8, 0xa0, 0x1f, 0x01, 0x22, 0x58, 0x0a, 0x07, 0x44, 0x65, 0x63, 0x43,
    0x6f, 0x69, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x31, 0x0a, 0x06, 0x61, 0x6d, 0x6f,
    0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x19, 0xc8, 0xde, 0x1f, 0x00, 0xda,
    0xde, 0x1f, 0x03, 0x44, 0x65, 0x63, 0xd2, 0xb4, 0x2d, 0x0a, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2e, 0x44, 0x65, 0x63, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x3a, 0x04, 0xe8, 0xa0,
    0x1f, 0x01, 0x22, 0x37, 0x0a, 0x08, 0x49, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2b,
    0x0a, 0x03, 0x69, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x19, 0xc8, 0xde, 0x1f,
    0x00, 0xda, 0xde, 0x1f, 0x03, 0x49, 0x6e, 0x74, 0xd2, 0xb4, 0x2d, 0x0a, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2e, 0x49, 0x6e, 0x74, 0x52, 0x03, 0x69, 0x6e, 0x74, 0x22, 0x37, 0x0a, 0x08, 0x44,
    0x65, 0x63, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2b, 0x0a, 0x03, 0x64, 0x65, 0x63, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x42, 0x19, 0xc8, 0xde, 0x1f, 0x00, 0xda, 0xde, 0x1f, 0x03, 0x44, 0x65,
    0x63, 0xd2, 0xb4, 0x2d, 0x0a, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x44, 0x65, 0x63, 0x52,
    0x03, 0x64, 0x65, 0x63, 0x42, 0x2c, 0xd8, 0xe1, 0x1e, 0x00, 0x80, 0xe2, 0x1e, 0x00, 0x5a, 0x22,
    0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
    0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x4a, 0xc8, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2f, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00,
    0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x1e, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x05,
    0x00, 0x1b, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x4f, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x0b, 0x12, 0x03, 0x07, 0x00, 0x4f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00,
    0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0x9b, 0xec, 0x03, 0x12, 0x03, 0x08, 0x00, 0x30, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xa0, 0xec, 0x03,
    0x12, 0x03, 0x09, 0x00, 0x30, 0x0a, 0xaf, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0f, 0x00,
    0x19, 0x01, 0x1a, 0xa2, 0x01, 0x20, 0x43, 0x6f, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x61, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x61, 0x20, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x20,
    0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x6e, 0x74,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x6d, 0x65, 0x74,
    0x68, 0x6f, 0x64, 0x0a, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x67, 0x6f, 0x67, 0x6f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x0f, 0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x10, 0x02, 0x22, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x07, 0x8d, 0xf4, 0x03, 0x12, 0x03, 0x10, 0x02, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x12, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x12, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x13,
    0x02, 0x18, 0x04, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x13, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x13, 0x09, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x13, 0x12, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x04, 0x13, 0x14, 0x18, 0x03, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x00, 0x02, 0x01, 0x08, 0xca, 0xd6, 0x05, 0x12, 0x03, 0x14, 0x04, 0x29, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xeb, 0xfb, 0x03, 0x12, 0x03, 0x15, 0x04, 0x22, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x16, 0x04, 0x22, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xf5, 0x8c, 0xa6, 0x05, 0x12, 0x03, 0x17, 0x04,
    0x21, 0x0a, 0xb9, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1f, 0x00, 0x25, 0x01, 0x1a, 0xac,
    0x01, 0x20, 0x44, 0x65, 0x63, 0x43, 0x6f, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61,
    0x20, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x61, 0x20, 0x64, 0x65, 0x63, 0x69, 0x6d, 0x61, 0x6c, 0x20, 0x61, 0x6d, 0x6f, 0x75,
    0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x6e, 0x20, 0x44, 0x65, 0x63, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x73, 0x74,
    0x6f, 0x6d, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61,
    0x74, 0x75, 0x72, 0x65, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62,
    0x79, 0x20, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x07,
    0x12, 0x03, 0x20, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x07, 0x8d, 0xf4, 0x03, 0x12,
    0x03, 0x20, 0x02, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x22, 0x02,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x09, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x04, 0x23, 0x02, 0x24, 0x6b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x23, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x23, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x23, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x24, 0x06,
    0x6a, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01, 0x08, 0xca, 0xd6, 0x05, 0x12, 0x03, 0x24,
    0x07, 0x2b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01, 0x08, 0xeb, 0xfb, 0x03, 0x12, 0x03,
    0x24, 0x2d, 0x4b, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x01, 0x08, 0xe9, 0xfb, 0x03, 0x12,
    0x03, 0x24, 0x4d, 0x69, 0x0a, 0x47, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01,
    0x1a, 0x3b, 0x20, 0x49, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69,
    0x6e, 0x65, 0x73, 0x20, 0x61, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x77,
    0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x61, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x61, 0x6e,
    0x20, 0x49, 0x6e, 0x74, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x28, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x29, 0x02, 0x76, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x29, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29,
    0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x0f, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x29, 0x11, 0x75, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xca, 0xd6, 0x05, 0x12, 0x03, 0x29, 0x12, 0x36, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xeb, 0xfb, 0x03, 0x12, 0x03, 0x29, 0x38, 0x56,
    0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x29, 0x58,
    0x74, 0x0a, 0x46, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x1a, 0x3a, 0x20,
    0x44, 0x65, 0x63, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73,
    0x20, 0x61, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x77, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x72, 0x20, 0x61, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x44, 0x65, 0x63,
    0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x2d, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x2e,
    0x02, 0x76, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2e, 0x11, 0x75, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03,
    0x02, 0x00, 0x08, 0xca, 0xd6, 0x05, 0x12, 0x03, 0x2e, 0x12, 0x36, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x03, 0x02, 0x00, 0x08, 0xeb, 0xfb, 0x03, 0x12, 0x03, 0x2e, 0x38, 0x56, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x03, 0x02, 0x00, 0x08, 0xe9, 0xfb, 0x03, 0x12, 0x03, 0x2e, 0x58, 0x74, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
