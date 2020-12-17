use crate::database::models::queryable::*;
use crate::database::models::users::{InsertUser, User};
use crate::errors::api::ApiError;
use crate::utils::structures::filterable::Limitable;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub async fn get(limitation: web::Query<Limitable>) -> Result<HttpResponse, ApiError> {
    let limitation = limitation.into_inner();
    let user_list = User::get_all(Box::new(limitation))?;
    Ok(HttpResponse::Ok().json(user_list))
}

pub async fn get_by_id(user_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = User::get_by_id(user_id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn post(user: web::Json<InsertUser>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn put(
    user_id: web::Path<Uuid>,
    user: web::Json<InsertUser>,
) -> Result<HttpResponse, ApiError> {
    let user = User::update(user_id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete(user_id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    User::delete(user_id.into_inner())?;
    Ok(HttpResponse::Ok().finish())
}
