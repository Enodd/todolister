use crate::{models::{app_state::AppState, response::Error, task::Tasks}, repositories::task_repository::TaskRepository};

pub struct TaskService;

impl TaskService {
    pub async fn get_tasks_for_user(state: AppState, target_user: uuid::Uuid) -> Result<Vec<Tasks>, Error> {
        let tasks = TaskRepository::get_tasks_for_user_from_db(state, target_user);
        let mut is_error = false;
        let query_result = match tasks {
            Ok(tasks) => tasks,
            Err(_) => {
                is_error = true;
                vec![]
            }
        };
        if is_error {
            return Err(Error { code: 500, message: "Failed to query".to_string() })
        }
        Ok(query_result)
    }
}