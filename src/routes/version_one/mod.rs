mod user;

use actix_web::{web, Scope};

pub fn init() -> Scope {
    web::scope("/v1").service(user::init())
}
