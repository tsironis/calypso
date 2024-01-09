use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::Parser;

mod diff;
mod git;
mod serve;
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
async fn main() -> Result<()> {
    let args = Cli::parse();

    let dir: PathBuf = current_dir()?;
    let base_dir: &Path = Path::new(&dir);
    let report_dir: PathBuf = base_dir.join("diff-report");

    let current_branch = git::get_current_branch().context("failed to get current branch name")?;

    util::copy_snaps(report_dir.as_path(), "current_snapshots")?;
    git::checkout_branch(args.branch)?;
    util::copy_snaps(report_dir.as_path(), "original_snapshots")?;
    git::checkout_branch(current_branch)?;
    util::compare_snaps(report_dir.as_path())?;
    util::create_report(report_dir.as_path())?;
    if args.serve {
        serve::start().await?;
    }
    Ok(())
}
