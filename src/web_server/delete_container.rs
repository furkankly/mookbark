use crate::web_server::{create_server::AppState, services};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct Params {
    container_name: String,
}

pub async fn delete_container(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
    params: Query<Params>,
) -> Result<(), StatusCode> {
    let Some(user_id) = session.get::<String>("user_id").expect("Couldn't deserialize session")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let result = services::delete_entity(
        &app_state.db_conn,
        user_id.as_str(),
        "container",
        params.container_name.as_str(),
    )
    .await;
    if result.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
