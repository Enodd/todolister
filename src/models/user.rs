use chrono::{DateTime, Utc};
use diesel::{prelude::Queryable, sql_types::Uuid, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub login: String,
    pub password: String,
    pub birthdate: DateTime<Utc>
}