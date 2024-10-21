use crate::models::{app_state::AppState, task::Tasks};
use diesel::prelude::*;
use uuid::Uuid;

pub struct TaskRepository;

impl TaskRepository {
    pub fn get_tasks_for_user_from_db(state: AppState, target_user: Uuid) -> Result<Vec<Tasks>, diesel::result::Error> {
        use crate::schema::tasks::dsl::*;
        let mut connection = state.pool.get().unwrap();
        let results = tasks
            .select(Tasks::as_select())
            .filter(user_id.eq(target_user))
            .load::<Tasks>(&mut *connection);
        results
    }
}