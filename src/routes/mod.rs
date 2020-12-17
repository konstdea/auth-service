mod version_one;

use actix_web::{web, Scope};

pub fn init() -> Scope {
    web::scope("/api").service(version_one::init())
}
