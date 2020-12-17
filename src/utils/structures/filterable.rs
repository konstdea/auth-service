use crate::utils::structures::Limitation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Limitable {
    #[serde(default = "default_offset")]
    offset: i64,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_offset() -> i64 {
    0
}
fn default_limit() -> i64 {
    10
}

impl Limitation for Limitable {
    fn offset(&self) -> i64 {
        *&self.offset
    }

    fn limit(&self) -> i64 {
        *&self.limit
    }
}
