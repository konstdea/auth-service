use crate::utils::structures::Limitation;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pageable {
    page: i32,
    per_page: i32,
}

impl Limitation for Pageable {
    fn offset(&self) -> i64 {
        let mut page = *&self.page as i64;
        if page == 0 {
            page = 1;
        }
        page * *&self.per_page as i64
    }
    fn limit(&self) -> i64 {
        *&self.per_page as i64
    }
}
