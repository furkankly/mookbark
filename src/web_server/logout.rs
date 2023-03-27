use axum::http::StatusCode;
use tower_sessions::Session;

pub async fn logout(session: Session) -> Result<(), StatusCode> {
    session.clear();
    Ok(())
}
