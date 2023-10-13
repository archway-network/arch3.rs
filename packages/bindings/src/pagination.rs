use cosmwasm_schema::cw_serde;

/// WASM binding version for the Cosmos SDK [`query.PageRequest`](https://github.com/cosmos/cosmos-sdk/blob/v0.45.16/proto/cosmos/base/query/v1beta1/pagination.proto),
/// embedded in request messages for efficient pagination.
#[cw_serde]
#[derive(Default)]
pub struct PageRequest {
    key: Option<String>,
    offset: Option<u64>,
    limit: Option<u64>,
    count_total: Option<bool>,
    reverse: Option<bool>,
}

impl PageRequest {
    pub fn new() -> Self {
        Self::default()
    }

    /// The `key` is a value returned in `PageResponse.next_key` to begin querying the next page
    /// most efficiently. Only one of offset or key should be set.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// A numeric offset that can be used when key is unavailable. It is less efficient than using
    /// key. Only one of offset or key should be set.
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Total number of results to be returned in the result page. If left empty, it will default
    /// to a value to be set by each app.
    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// When set to `true`, indicates that the result set should include a count of the total number
    /// of items available for pagination. `count_total` is only respected when `offset` is used.
    /// It is ignored when `key` is set.
    pub fn count_total(mut self) -> Self {
        self.count_total = Some(true);
        self
    }

    /// If set to `true`, the results will be returned in the descending order.
    pub fn reverse(mut self) -> Self {
        self.reverse = Some(true);
        self
    }
}

/// Embedded in response messages where the corresponding request message has used a [`PageRequest`].
#[cw_serde]
pub struct PageResponse {
    /// Key to be passed to `PageRequest.key` to query the next page most efficiently.
    pub next_key: Option<String>,
    /// Total number of results available if `PageRequest.count_total` was set; its value is
    /// `None` otherwise
    pub total: Option<u64>,
}
