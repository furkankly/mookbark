use crate::web_server::{create_server::AppState, entities::user, services};
use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tower_sessions::Session;

#[derive(Serialize)]
struct User {
    pub email: String,
    pub username: String,
    pub avatar: String,
}

impl From<user::Model> for User {
    fn from(value: user::Model) -> Self {
        User {
            email: value.email,
            username: value.username,
            avatar: value.avatar,
        }
    }
}

pub async fn get_user(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<Json<Value>, StatusCode> {
    let Some(user_id) = session
        .get::<String>("user_id")
        .expect("Couldn't deserialize session")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let result = services::get_user(&app_state.db_conn, user_id.as_str()).await;
    match result {
        Ok(user) => {
            if let Some(user) = user {
                let user = User::from(user);
                Ok(Json(json!(user)))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
