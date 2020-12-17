table! {
    users {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created -> Timestamptz,
        enabled -> Bool,
    }
}
