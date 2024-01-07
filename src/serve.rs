use std::net::SocketAddr;
use tower_http::services::ServeFile;

use axum::{routing::get_service, Router};
pub async fn start() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get_service(ServeFile::new("diff-report/index.html")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
