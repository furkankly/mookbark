use crate::web_server::{
    add_bookmark, add_container, check_auth, db, delete_bookmark, delete_container, get_bookmark,
    get_bookmarks, get_user, health_check, logout, serve_web_app, session, token,
};
use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    http::{HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
    BoxError, Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{cookie, Session as TowerSession, SessionManagerLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    pub db_conn: DatabaseConnection,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Session {
    user_id: String,
}

async fn protected_routes(
    session: TowerSession,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if session
        .get::<String>("user_id")
        .expect("Couldn't deserialize the session")
        .is_none()
    {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        let response = next.run(request).await;
        Ok(response)
    }
}

// cors
pub fn allow_cors() -> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST, Method::DELETE])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_credentials(true)
}

// ssl
pub async fn ssl_config() -> RustlsConfig {
    RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("0.0.0.0.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("self_signed_certs")
            .join("0.0.0.0-key.pem"),
    )
    .await
    .unwrap()
}

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "mookbark=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}

pub async fn create_server() {
    setup_tracing();

    let db_conn = db::create_db_conn_pool().await;
    let state = Arc::new(AppState { db_conn });

    let session_store = session::create_session_store().await;
    let mut session_manager = SessionManagerLayer::new(session_store).with_secure(true);
    if env::var("SERVER_ENV").unwrap() == "dev" {
        session_manager = session_manager
            .clone()
            // This is required in dev and when its set to None, Cookie must be secure
            // Thats why we need a tls enabled dev server
            // See https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
            .with_same_site(cookie::SameSite::None)
    }
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            let err = err.to_string();
            tracing::error!(err);
            StatusCode::BAD_REQUEST
        }))
        .layer(session_manager);

    let mut app = Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/bookmark", post(add_bookmark::add_bookmark))
                .route("/bookmark", get(get_bookmark::get_bookmark))
                .route("/bookmark", delete(delete_bookmark::delete_bookmark))
                .route("/container", post(add_container::add_container))
                .route("/container", delete(delete_container::delete_container))
                .route("/bookmarks", get(get_bookmarks::get_bookmarks))
                .route("/user", get(get_user::get_user))
                .route("/check-auth", get(check_auth::check_auth))
                .route_layer(middleware::from_fn(protected_routes))
                .route("/health-check", get(health_check::health_check))
                .route("/token", post(token::token))
                .route("/logout", post(logout::logout))
                .layer(Extension(state))
                .layer(session_service),
        )
        .nest(
            "/dashboard",
            Router::new().fallback_service(serve_web_app::serve_web_app()),
        )
        .layer(TraceLayer::new_for_http());

    if env::var("SERVER_ENV").unwrap() == "dev" {
        app = app.layer(allow_cors());
        axum_server::bind_rustls("0.0.0.0:8080".parse().unwrap(), ssl_config().await)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
        tracing::debug!("listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}
