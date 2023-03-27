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
    bookmark_url: String,
}

pub async fn delete_bookmark(
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
        "bookmark",
        params.bookmark_url.as_str(),
    )
    .await;
    if result.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
