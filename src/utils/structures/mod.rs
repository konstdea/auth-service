pub mod filterable;
pub mod pageable;

pub trait Limitation {
    fn offset(&self) -> i64;
    fn limit(&self) -> i64;
}
