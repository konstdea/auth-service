use crate::database::models::queryable::Delete;
use crate::database::models::queryable::Update;
use crate::database::models::queryable::{Connectable, Create, GetAllLimited, GetById};
use crate::database::schema::users;
use crate::diesel::{Insertable, Queryable, associations::HasTable, prelude::*, Identifiable};
use crate::errors::api::ApiError;
use crate::utils::structures::Limitation;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created: NaiveDateTime,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "users"]
pub struct InsertUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset)]
#[table_name = "users"]
pub struct LoadUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created: NaiveDateTime,
    pub enabled: bool,
}

impl From<User> for LoadUser {
    fn from(user: User) -> Self {
        LoadUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created: user.created,
            enabled: user.enabled,
        }
    }
}

impl HasTable for User {
    type Table = users::table;

    fn table() -> Self::Table {
        users::table
    }
}

impl Identifiable for User {
    type Id = Uuid;

    fn id(self) -> Self::Id {
        self.id
    }
}

impl Connectable for User {}


impl GetAllLimited<LoadUser> for User {
    fn get_all(limitation: Box<dyn Limitation>) -> Result<Vec<LoadUser>, ApiError> {
        let connection = Self::get_connection()?;
        let list = Self::table()
            .select((
                users::id,
                users::username,
                users::email,
                users::created,
                users::enabled,
            ))
            .limit(limitation.limit())
            .offset(limitation.offset())
            .load::<LoadUser>(&connection)?;
        Ok(list)
    }
}

impl GetById<LoadUser, User> for User {
    fn get_by_id(id: Self::Id) -> Result<LoadUser, ApiError> {
        let connection = Self::get_connection()?;
        let user = Self::table().find(id).get_result::<User>(&connection)?;
        let user = LoadUser::from(user);
        Ok(user)
    }
}

impl Create<LoadUser, InsertUser, User> for User {
    fn create(user: InsertUser) -> Result<LoadUser, ApiError> {
        let connection = Self::get_connection()?;
        let user = diesel::insert_into(Self::table())
            .values(user)
            .get_result::<User>(&connection)?;
        let user = LoadUser::from(user);
        Ok(user)
    }
}

impl Update<LoadUser, InsertUser, User> for User {
    fn update(
        id: <Self as diesel::Identifiable>::Id,
        user: InsertUser,
    ) -> Result<LoadUser, ApiError> {
        let connection = Self::get_connection()?;
        let user = diesel::update(Self::table())
            .filter(users::id.eq(id))
            .set(user)
            .get_result::<User>(&connection)?;
        let user = LoadUser::from(user);
        Ok(user)
    }
}

impl Delete for User {
    fn delete(id: <Self as diesel::Identifiable>::Id) -> Result<(), ApiError> {
        let connection = Self::get_connection()?;
        diesel::delete(Self::table().filter(users::id.eq(id))).execute(&connection)?;
        Ok(())
    }
}
