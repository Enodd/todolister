use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use uuid::Uuid;


#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(Pg))]
#[diesel(belongs_to(User))]
pub struct Session {
    pub id: Uuid,
    pub session_data: String,
    pub expires_at: NaiveDateTime,
    pub user_id: Uuid,
}