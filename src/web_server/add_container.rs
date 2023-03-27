use crate::web_server::{create_server::AppState, services};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
};
use sea_orm::TransactionTrait;
use serde::Deserialize;
use std::sync::Arc;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct Params {
    parent_container_name: String,
    container_name: String,
}

pub async fn add_container(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<Params>,
) -> Result<(), StatusCode> {
    let Some(user_id) = session.get::<String>("user_id").expect("Couldn't deserialize session")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let Ok(txn) = app_state.db_conn.begin().await
    else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let result = services::add_container(
        &txn,
        user_id.as_str(),
        params.parent_container_name.as_str(),
        params.container_name.as_str(),
    )
    .await;
    if result.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let result = txn.commit().await;
    if result.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}
