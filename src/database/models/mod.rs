pub mod users;

pub mod queryable {
    use crate::database;
    use crate::database::DbConnection;
    use crate::errors::api::ApiError;
    use crate::utils::structures::Limitation;
    use diesel::associations::HasTable;
    use diesel::Identifiable;

    pub trait Connectable {
        fn get_connection() -> Result<DbConnection, ApiError> {
            database::connection()
        }
    }

    pub trait GetAll<T>: HasTable + Identifiable {
        fn get_all(&self) -> Result<Vec<T>, ApiError>;
    }

    pub trait GetAllLimited<T>: Identifiable {
        fn get_all(limitation: Box<dyn Limitation>) -> Result<Vec<T>, ApiError>;
    }

    pub trait GetById<T: From<U>, U>: Identifiable {
        fn get_by_id(id: Self::Id) -> Result<T, ApiError>;
    }

    pub trait Create<T: From<U>, I, U>: Identifiable {
        fn create(item: I) -> Result<T, ApiError>;
    }

    pub trait Update<T: From<U>, I, U>: Identifiable {
        fn update(id: Self::Id, item: I) -> Result<T, ApiError>;
    }

    pub trait Delete: Identifiable {
        fn delete(id: Self::Id) -> Result<(), ApiError>;
    }
}
