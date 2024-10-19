use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::user::User;


pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub create_date: DateTime<Utc>, // we will calculate severity basing on difference between current_date and due_date, and then we will have percentages
    pub due_date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub creator: User
}