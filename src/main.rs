use std::process::{Command, Output};

use clap::Parser;

/// Simple program to greet a person
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

fn get_current_branch() -> Result<Vec<u8>, Vec<u8>> {
    let current_branch = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .expect("failed to execute git rev-parse");

    println!("{:?}", current_branch);

    match current_branch.stderr[..] {
        [] => Ok(current_branch.stdout),
        _ => Err(current_branch.stderr),
    }
}

fn main() {
    let args = Cli::parse();

    if let Some(path) = args.path.as_deref() {
        println!("path: {path}!");
    }
    println!("branch: {}!", args.branch);

    let current_branch = get_current_branch();
    println!("{:?}", current_branch)
}
