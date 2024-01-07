// use glob::glob;
// use image;
// use lcs_image_diff::compare;

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use axum::{routing::get, Router};
use clap::Parser;

mod diff;
mod git;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the base branch to use for git diff
    /// against the currenct checked out branch
    #[arg(short, long)]
    branch: String,

    /// Relative path to run only specified test cases
    #[arg(short, long)]
    path: Option<String>,

    /// Enable the serving of the reporter web app
    #[arg(short, long)]
    serve: bool,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let dir: PathBuf = current_dir().unwrap();
    let base_dir: &Path = Path::new(&dir);
    let report_dir: PathBuf = base_dir.join("diff-report");

    let current_branch = match git::get_current_branch() {
        Some(branch) => branch,
        None => panic!("failed to get current branch name"),
    };

    util::copy_snaps(report_dir.as_path(), "current_snapshots");
    git::checkout_branch(args.branch);
    util::copy_snaps(report_dir.as_path(), "original_snapshots");
    git::checkout_branch(current_branch);
    util::compare_snaps(report_dir.as_path());
    if args.serve {
        serve().await;
    }
}

async fn serve() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running reported at http://localhost:3000");
    println!("TODO create diff-report/index.html");
    axum::serve(listener, app).await.unwrap();
}
