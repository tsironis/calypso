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
    /// Enable logging of calculating time for each snapshot
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    // gather env info
    let dir: PathBuf = current_dir()?;
    let base_dir: &Path = Path::new(&dir);
    let report_dir: PathBuf = base_dir.join("diff-report");
    let current_branch = git::get_current_branch().context("failed to get current branch name")?;
    // prepare filesystem by coping snapshots from target branch
    util::copy_snaps(report_dir.as_path(), "current_snapshots")?;
    git::checkout_branch(&args.branch)?;
    util::copy_snaps(report_dir.as_path(), "original_snapshots")?;
    git::checkout_branch(&current_branch)?;
    // actual comparsion of snapshots and created diff image files
    let snaps = util::compare_snaps(report_dir.as_path(), &args)?;
    // create html reporter file
    util::create_report(report_dir.as_path(), snaps, current_branch, args.branch)?;
    // start axum server
    if args.serve {
        serve::start(report_dir).await?;
    }
    Ok(())
}
