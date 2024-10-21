use chrono::NaiveDateTime;
use diesel::{pg::Pg, prelude::{Associations, Identifiable, Queryable, QueryableByName, Selectable}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, QueryableByName)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(Pg))]
#[diesel(belongs_to(User))]
pub struct Tasks {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub create_date: NaiveDateTime, // we will calculate severity basing on difference between current_date and due_date, and then we will have percentages
    pub due_date: Option<NaiveDateTime>,
    pub tags: Vec<Option<String>>,
    pub user_id: Uuid
}