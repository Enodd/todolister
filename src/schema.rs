// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        id -> Uuid,
        session_data -> Text,
        expires_at -> Timestamptz,
        user_id -> Uuid
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        #[max_length = 40]
        title -> Varchar,
        description -> Text,
        user_id -> Uuid,
        create_date -> Timestamptz,
        due_date -> Nullable<Timestamptz>,
        tags -> Array<Nullable<Text>>,
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

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    tasks,
    users,
);
