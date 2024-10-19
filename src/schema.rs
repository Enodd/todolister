// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        #[max_length = 40]
        title -> Varchar,
        description -> Text,
        create_date -> Timestamp,
        due_date -> Nullable<Timestamp>,
        tags -> Nullable<Array<Nullable<Text>>>,
        creator -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        #[max_length = 40]
        login -> Varchar,
        #[max_length = 32]
        password -> Varchar,
        birthdate -> Timestamp,
    }
}

diesel::joinable!(tasks -> users (creator));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
