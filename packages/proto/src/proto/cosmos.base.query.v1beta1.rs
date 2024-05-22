// @generated
/// PageRequest is to be embedded in gRPC request messages for efficient
/// pagination. Ex:
///
///   message SomeRequest {
///           Foo some_parameter = 1;
///           PageRequest pagination = 2;
///   }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageRequest {
    /// key is a value returned in PageResponse.next_key to begin
    /// querying the next page most efficiently. Only one of offset or key
    /// should be set.
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// offset is a numeric offset that can be used when key is unavailable.
    /// It is less efficient than using key. Only one of offset or key should
    /// be set.
    #[prost(uint64, tag = "2")]
    pub offset: u64,
    /// limit is the total number of results to be returned in the result page.
    /// If left empty it will default to a value to be set by each app.
    #[prost(uint64, tag = "3")]
    pub limit: u64,
    /// count_total is set to true  to indicate that the result set should include
    /// a count of the total number of items available for pagination in UIs.
    /// count_total is only respected when offset is used. It is ignored when key
    /// is set.
    #[prost(bool, tag = "4")]
    pub count_total: bool,
    /// reverse is set to true if results are to be returned in the descending order.
    ///
    /// Since: cosmos-sdk 0.43
    #[prost(bool, tag = "5")]
    pub reverse: bool,
}
impl ::prost::Name for PageRequest {
    const NAME: &'static str = "PageRequest";
    const PACKAGE: &'static str = "cosmos.base.query.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.query.v1beta1.{}", Self::NAME)
    }
}
/// PageResponse is to be embedded in gRPC response messages where the
/// corresponding request message has used PageRequest.
///
///   message SomeResponse {
///           repeated Bar results = 1;
///           PageResponse page = 2;
///   }
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageResponse {
    /// next_key is the key to be passed to PageRequest.key to
    /// query the next page most efficiently. It will be empty if
    /// there are no more results.
    #[prost(bytes = "vec", tag = "1")]
    pub next_key: ::prost::alloc::vec::Vec<u8>,
    /// total is total number of results available if PageRequest.count_total
    /// was set, its value is undefined otherwise
    #[prost(uint64, tag = "2")]
    pub total: u64,
}
impl ::prost::Name for PageResponse {
    const NAME: &'static str = "PageResponse";
    const PACKAGE: &'static str = "cosmos.base.query.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.base.query.v1beta1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.base.query.v1beta1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xe3, 0x11, 0x0a, 0x2a, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
    0x2f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x70,
    0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x22, 0x88, 0x01, 0x0a, 0x0b, 0x50,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x16, 0x0a, 0x06,
    0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6f, 0x66,
    0x66, 0x73, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x1f, 0x0a, 0x0b, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52,
    0x0a, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x72,
    0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x72, 0x65,
    0x76, 0x65, 0x72, 0x73, 0x65, 0x22, 0x3f, 0x0a, 0x0c, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x19, 0x0a, 0x08, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x07, 0x6e, 0x65, 0x78, 0x74, 0x4b, 0x65, 0x79,
    0x12, 0x14, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x42, 0x2a, 0x5a, 0x28, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x71, 0x75, 0x65,
    0x72, 0x79, 0x4a, 0x99, 0x0f, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x37, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x03, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x03, 0x00, 0x3f, 0x0a, 0xc9, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c,
    0x00, 0x25, 0x01, 0x1a, 0xbc, 0x01, 0x20, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x62, 0x65,
    0x64, 0x64, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x70, 0x61, 0x67,
    0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x45, 0x78, 0x3a, 0x0a, 0x0a, 0x20, 0x20,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x53, 0x6f, 0x6d, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x46, 0x6f, 0x6f, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74,
    0x65, 0x72, 0x20, 0x3d, 0x20, 0x31, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x70, 0x61,
    0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x3d, 0x20, 0x32, 0x3b, 0x0a, 0x20, 0x20,
    0x7d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x13, 0x0a, 0x9d,
    0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x10, 0x1a, 0x8f, 0x01, 0x20,
    0x6b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x50, 0x61, 0x67, 0x65, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2e, 0x6e, 0x65, 0x78, 0x74, 0x5f, 0x6b, 0x65, 0x79,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x0a, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x70, 0x61, 0x67,
    0x65, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x74,
    0x6c, 0x79, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x0e, 0x0f, 0x0a, 0xa4, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x15, 0x02, 0x14, 0x1a, 0x96, 0x01, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x75, 0x6d, 0x65, 0x72, 0x69, 0x63, 0x20, 0x6f, 0x66,
    0x66, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69,
    0x73, 0x20, 0x75, 0x6e, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x20,
    0x49, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x65, 0x66, 0x66, 0x69, 0x63,
    0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20,
    0x6b, 0x65, 0x79, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x12, 0x13, 0x0a, 0x98, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x19, 0x02, 0x13, 0x1a, 0x8a, 0x01, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x6e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x70, 0x61, 0x67,
    0x65, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x6c, 0x65, 0x66, 0x74, 0x20, 0x65, 0x6d, 0x70, 0x74,
    0x79, 0x20, 0x69, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x62, 0x79, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x61,
    0x70, 0x70, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x11, 0x12, 0x0a, 0xf5,
    0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1f, 0x02, 0x17, 0x1a, 0xe7, 0x01, 0x20,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x69, 0x73, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x6e, 0x64, 0x69, 0x63, 0x61, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x73, 0x65, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x0a, 0x20, 0x61, 0x20, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x6f, 0x74, 0x61, 0x6c,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x69, 0x74, 0x65, 0x6d, 0x73,
    0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x70,
    0x61, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x20, 0x55, 0x49, 0x73,
    0x2e, 0x0a, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x69,
    0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20, 0x69, 0x67, 0x6e, 0x6f,
    0x72, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x6b, 0x65, 0x79, 0x0a, 0x20, 0x69, 0x73,
    0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x1f, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1f,
    0x07, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1f, 0x15, 0x16,
    0x0a, 0x75, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x24, 0x02, 0x13, 0x1a, 0x68, 0x20,
    0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20,
    0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64,
    0x6b, 0x20, 0x30, 0x2e, 0x34, 0x33, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x24, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x24, 0x07, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x24, 0x11,
    0x12, 0x0a, 0xe9, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2e, 0x00, 0x37, 0x01, 0x1a, 0xdc,
    0x01, 0x20, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x62, 0x65, 0x64, 0x64, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x68, 0x61, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x50, 0x61, 0x67,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x20, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x53, 0x6f, 0x6d, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x20, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x72, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x42, 0x61, 0x72, 0x20, 0x72, 0x65, 0x73, 0x75, 0x6c,
    0x74, 0x73, 0x20, 0x3d, 0x20, 0x31, 0x3b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x70,
    0x61, 0x67, 0x65, 0x20, 0x3d, 0x20, 0x32, 0x3b, 0x0a, 0x20, 0x20, 0x7d, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x14, 0x0a, 0x9d, 0x01, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x15, 0x1a, 0x8f, 0x01, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x5f,
    0x6b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x50,
    0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x6b, 0x65, 0x79, 0x20, 0x74,
    0x6f, 0x0a, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x78,
    0x74, 0x20, 0x70, 0x61, 0x67, 0x65, 0x20, 0x6d, 0x6f, 0x73, 0x74, 0x20, 0x65, 0x66, 0x66, 0x69,
    0x63, 0x69, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x69, 0x66, 0x0a, 0x20, 0x74, 0x68,
    0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20,
    0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x32, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x32, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x32, 0x13, 0x14, 0x0a, 0x7f, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x36, 0x02, 0x13,
    0x1a, 0x72, 0x20, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x69, 0x73, 0x20, 0x74, 0x6f, 0x74, 0x61,
    0x6c, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x73, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x69, 0x66,
    0x20, 0x50, 0x61, 0x67, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x63, 0x6f, 0x75,
    0x6e, 0x74, 0x5f, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x0a, 0x20, 0x77, 0x61, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x2c, 0x20, 0x69, 0x74, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20,
    0x75, 0x6e, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x77,
    0x69, 0x73, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x36,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x36, 0x09, 0x0e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x36, 0x11, 0x12, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)