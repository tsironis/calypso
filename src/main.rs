// use glob::glob;
// use image;
// use lcs_image_diff::compare;

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use clap::Parser;

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
}

fn main() {
    let args = Cli::parse();

    let dir: PathBuf = current_dir().unwrap();
    let base_dir: &Path = Path::new(&dir);
    let report_dir: PathBuf = base_dir.join("diff-report");

    let current_branch = match git::get_current_branch() {
        Some(branch) => branch,
        None => panic!("failed to get current branch name"),
    };

    git::checkout_branch(args.branch);
    util::copy_snaps(&report_dir, "original_snapshots");
    git::checkout_branch(current_branch);
    util::copy_snaps(&report_dir, "current_snapshots");
    println!("diffing in place")
}
