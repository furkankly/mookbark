use axum::http::StatusCode;

// We just need a unprotected endpoint (so it doesn't consume upstash free limits on each hit)
pub async fn health_check() -> Result<(), StatusCode> {
    Ok(())
}
