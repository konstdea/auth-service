use actix_web::{web, Scope};

use crate::controllers::version_one::user;

pub fn init() -> Scope {
    web::scope("/user")
        .service(
            web::resource("")
                .route(web::get().to(user::get))
                .route(web::post().to(user::post)),
        )
        .service(
            web::resource("/{id}")
                .route(web::get().to(user::get_by_id))
                .route(web::put().to(user::put))
                .route(web::delete().to(user::delete)),
        )
}
