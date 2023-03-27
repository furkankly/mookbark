use axum::http::StatusCode;

// We just need a protected endpoint (behind auth checking middleware)
pub async fn check_auth() -> Result<(), StatusCode> {
    Ok(())
}
