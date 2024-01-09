use std::net::SocketAddr;
use tower_http::services::ServeFile;

use anyhow::{Context, Result};

use axum::{routing::get_service, Router};
pub async fn start() -> Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get_service(ServeFile::new("diff-report/index.html")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to address {}", addr))?;
    axum::serve(listener, app).await?;
    Ok(())
}
