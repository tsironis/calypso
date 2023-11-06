use git2::{Reference, Repository};
use std::env::current_dir;
pub fn get_current_branch() -> Option<String> {
    let dir = current_dir().unwrap();
    let repo = match Repository::discover(dir) {
        Ok(repo) => repo,
        Err(err) => panic!("failed to discover a git repository \n\n {}", err),
    };
    let current_head = match repo.head() {
        Ok(reference) => reference,
        Err(err) => panic!("failed to get current HEAD \n\n {}", err),
    };
    let current_branch = Reference::shorthand(&current_head);
    return current_branch.map(str::to_string);
}
