// use glob::glob;
// use image;
// use lcs_image_diff::compare;

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::git::get_current_branch;

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

    if let Some(current_branch) = get_current_branch() {
    } else {
    }
    let current_branch = match git::get_current_branch() {
        Some(branch) => branch,
        None => panic!("failed to get current branch name"),
    };
    println!("{}", current_branch);

    util::copy_og_snaps(report_dir, args.branch);
}
