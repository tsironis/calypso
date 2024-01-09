use anyhow::{Context, Result};
use git2::{Reference, Repository};
use std::{
    env::current_dir,
    process::{Command, Output},
};

pub fn get_current_branch() -> Result<String> {
    let dir = current_dir()?;
    let repo = Repository::discover(&dir)
        .with_context(|| format!("failed to discover git repository from {}", dir.display()))?;
    let current_head = repo.head().context("failed to get current HEAD")?;
    let current_branch = Reference::shorthand(&current_head)
        .map(str::to_string)
        .context("failed to get current branch name from current HEAD reference shorthand")?;
    Ok(current_branch)
}

pub fn checkout_branch(refname: String) -> Result<Output> {
    println!("Checking out to {} branch...", refname);
    let dir = current_dir()?;
    let repo = Repository::discover(&dir)
        .with_context(|| format!("failed to discover git repository from {}", dir.display()))?;
    let (object, reference) = repo
        .revparse_ext(refname.as_str())
        .context("Object not found")?;

    repo.checkout_tree(&object, None)
        .context("Failed to checkout")?;

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .context("Failed to set head")?;

    Command::new("git")
        .args(["lfs", "pull"])
        .output()
        .context("failed to execute `git lfs pull`")
}
