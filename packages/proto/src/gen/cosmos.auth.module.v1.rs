// @generated
/// Module is the config object for the auth module.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// bech32_prefix is the bech32 account prefix for the app.
    #[prost(string, tag = "1")]
    pub bech32_prefix: ::prost::alloc::string::String,
    /// module_account_permissions are module account permissions.
    #[prost(message, repeated, tag = "2")]
    pub module_account_permissions: ::prost::alloc::vec::Vec<ModuleAccountPermission>,
    /// authority defines the custom module authority. If not set, defaults to the governance module.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
    const NAME: &'static str = "Module";
    const PACKAGE: &'static str = "cosmos.auth.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.auth.module.v1.{}", Self::NAME)
    }
}
/// ModuleAccountPermission represents permissions for a module account.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccountPermission {
    /// account is the name of the module.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// permissions are the permissions this module has. Currently recognized
    /// values are minter, burner and staking.
    #[prost(string, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for ModuleAccountPermission {
    const NAME: &'static str = "ModuleAccountPermission";
    const PACKAGE: &'static str = "cosmos.auth.module.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.auth.module.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.auth.module.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xca, 0x0a, 0x0a, 0x22, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x61, 0x75, 0x74, 0x68,
    0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e,
    0x61, 0x75, 0x74, 0x68, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x20,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x61, 0x70, 0x70, 0x2f, 0x76, 0x31, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x31, 0x2f, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0xe6, 0x01, 0x0a, 0x06, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x62,
    0x65, 0x63, 0x68, 0x33, 0x32, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0c, 0x62, 0x65, 0x63, 0x68, 0x33, 0x32, 0x50, 0x72, 0x65, 0x66, 0x69, 0x78,
    0x12, 0x6c, 0x0a, 0x1a, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x5f, 0x61, 0x63, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x61, 0x75,
    0x74, 0x68, 0x2e, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x52, 0x18, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x41, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x1c,
    0x0a, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x3a, 0x2b, 0xba, 0xc0,
    0x96, 0xda, 0x01, 0x25, 0x0a, 0x23, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73,
    0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x22, 0x55, 0x0a, 0x17, 0x4d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x20,
    0x0a, 0x0b, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x09, 0x52, 0x0b, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x4a, 0xa2, 0x07, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1e, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x2a, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x07, 0x00, 0x14, 0x01, 0x1a, 0x32, 0x20, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20,
    0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x75, 0x74,
    0x68, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x04, 0x08,
    0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x12, 0x04,
    0x08, 0x02, 0x0a, 0x04, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x07, 0x87, 0xe8, 0xa2, 0x1b, 0x01,
    0x12, 0x03, 0x09, 0x04, 0x34, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d,
    0x02, 0x1b, 0x1a, 0x39, 0x20, 0x62, 0x65, 0x63, 0x68, 0x33, 0x32, 0x5f, 0x70, 0x72, 0x65, 0x66,
    0x69, 0x78, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x65, 0x63, 0x68, 0x33, 0x32,
    0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x70, 0x72, 0x65, 0x66, 0x69, 0x78, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x70, 0x70, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0d, 0x19, 0x1a, 0x0a, 0x49, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x10, 0x02, 0x42, 0x1a, 0x3c, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x5f, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x10, 0x0b, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x10, 0x23, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x10, 0x40, 0x41, 0x0a, 0x6c, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x17, 0x1a, 0x5f, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72,
    0x69, 0x74, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x61, 0x75,
    0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x73, 0x65, 0x74, 0x2c, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x20,
    0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x13, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x13, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13,
    0x15, 0x16, 0x0a, 0x52, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x17, 0x00, 0x1e, 0x01, 0x1a, 0x46,
    0x20, 0x4d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x50, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x17,
    0x08, 0x1f, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x19, 0x02, 0x15, 0x1a,
    0x24, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x64,
    0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x09,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x13, 0x14, 0x0a,
    0x7c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x22, 0x1a, 0x6f, 0x20, 0x70,
    0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x68, 0x61, 0x73, 0x2e, 0x20,
    0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x67, 0x6e,
    0x69, 0x7a, 0x65, 0x64, 0x0a, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x6d, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x62, 0x75, 0x72, 0x6e, 0x65, 0x72, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x74, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1d, 0x20, 0x21, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
