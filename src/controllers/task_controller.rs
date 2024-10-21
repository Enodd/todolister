use axum::{extract::{Path, State}, Json};
use uuid::Uuid;

use crate::{models::{app_state::AppState, response::{Response, ResponseType}, task::Tasks}, services::task_service::TaskService};

pub struct TaskController;

impl TaskController {
    pub async fn get_tasks_for_user(Path(user_id): Path<Uuid>, State(app_state): State<AppState>) -> Json<Response<Vec<Tasks>>> {
        let tasks = TaskService::get_tasks_for_user(app_state, user_id).await;
        let response = match tasks {
            Ok(t) => Response::<Vec<Tasks>>::new(
                ResponseType::Data,
                "Getting tasks ended successfully",
                200,
                Some(t)
            ),
            Err(_) => Response::new(ResponseType::Error, "failed getting tasks", 500, None)
        };
        Json(response)
    }
}