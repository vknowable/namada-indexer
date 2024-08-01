use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Serialize, Deserialize, Validate)]
pub struct BlockRangeQueryParams {
    #[validate(range(min = 1, max = 10000))]
    pub tip: Option<u64>,
    #[validate(range(min = 1, max = 20))]
    pub length: Option<u64>,
}
