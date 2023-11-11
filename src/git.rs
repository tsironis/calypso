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
pub fn checkout_branch(refname: String) {
    println!("Checking out to {} branch...", refname);
    let dir = current_dir().unwrap();
    let repo = match Repository::discover(dir) {
        Ok(repo) => repo,
        Err(err) => panic!("failed to discover a git repository \n\n {}", err),
    };
    let (object, reference) = repo
        .revparse_ext(refname.as_str())
        .expect("Object not found");
    repo.checkout_tree(&object, None)
        .expect("Failed to checkout");

    match reference {
        // gref is an actual reference like branches or tags
        Some(gref) => repo.set_head(gref.name().unwrap()),
        // this is a commit, not a reference
        None => repo.set_head_detached(object.id()),
    }
    .expect("Failed to set HEAD");
}
