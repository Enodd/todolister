// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        #[max_length = 40]
        title -> Varchar,
        description -> Text,
        create_date -> Timestamptz,
        due_date -> Nullable<Timestamptz>,
        tags -> Array<Text>,
        user_id -> Uuid,
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
        birthdate -> Timestamptz,
    }
}

diesel::joinable!(tasks -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
