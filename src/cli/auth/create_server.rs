use crate::cli::auth::{serve_token_page, token};
use axum::{routing::post, Router};
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    signal,
    sync::{
        oneshot::{self, Sender},
        Mutex,
    },
};

lazy_static! {
    /// Channel used to send shutdown signal - wrapped in an Option to allow
    /// it to be taken by value (since oneshot channels consume themselves on
    /// send) and an Arc<Mutex> to allow it to be safely shared between threads
    pub static ref SHUTDOWN_TX: Arc<Mutex<Option<Sender<()>>>> = <_>::default();
}

pub async fn create_server(tx: Sender<String>) {
    let app = Router::new()
        .nest("/api", Router::new().route("/token", post(token::token)))
        .fallback_service(serve_token_page::serve_token_page());
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let _ = tx.send(addr);
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
async fn shutdown_signal() {
    let (tx, rx) = oneshot::channel::<()>();
    SHUTDOWN_TX.lock().await.replace(tx);

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let one_shot = async { rx.await.expect("failed to install oneshot channel") };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = one_shot => {},
    }
}
