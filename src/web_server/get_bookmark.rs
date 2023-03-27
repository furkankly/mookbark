use crate::web_server::{create_server::AppState, entities::bookmark, services};
use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tower_sessions::Session;

#[derive(Serialize)]
struct Bookmark {
    pub url: String,
    pub user_id: String,
    pub ingested: bool,
}

impl From<bookmark::Model> for Bookmark {
    fn from(value: bookmark::Model) -> Self {
        Bookmark {
            url: value.url,
            user_id: value.user_id,
            ingested: value.ingested != 0,
        }
    }
}

#[derive(Deserialize)]
pub struct Params {
    bookmark_url: String,
}

pub async fn get_bookmark(
    session: Session,
    Extension(app_state): Extension<Arc<AppState>>,
    Query(params): Query<Params>,
) -> Result<Json<Value>, StatusCode> {
    let Some(user_id) = session
        .get::<String>("user_id")
        .expect("Couldn't deserialize session")
    else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let result = services::get_bookmark(
        &app_state.db_conn,
        user_id.as_str(),
        params.bookmark_url.as_str(),
    )
    .await;
    match result {
        Ok(bookmark) => {
            if let Some(bookmark) = bookmark {
                let bookmark = Bookmark::from(bookmark);
                Ok(Json(json!(bookmark)))
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_err) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
