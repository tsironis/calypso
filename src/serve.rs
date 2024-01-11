use axum_embed::ServeEmbed;
use rust_embed::RustEmbed;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::services::{ServeDir, ServeFile};

use anyhow::{Context, Result};

use axum::{routing::get_service, Router};

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

pub async fn start(report_dir: PathBuf) -> Result<()> {
    tracing_subscriber::fmt::init();
    let index = report_dir.join("index.html");
    let current_snapshots_dir = report_dir.join("current_snapshots");
    let original_snapshots_dir = report_dir.join("original_snapshots");
    let diff_snapshots_dir = report_dir.join("diff_snapshots");
    let app = Router::new()
        .route("/", get_service(ServeFile::new(index)))
        .nest_service("/assets", ServeEmbed::<Assets>::new())
        .nest_service("/original_snapshots", ServeDir::new(original_snapshots_dir))
        .nest_service("/diff_snapshots", ServeDir::new(diff_snapshots_dir))
        .nest_service("/current_snapshots", ServeDir::new(current_snapshots_dir));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to address {}", addr))?;
    axum::serve(listener, app).await?;
    Ok(())
}
