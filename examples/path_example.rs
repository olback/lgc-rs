extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Path};

fn main() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let path = lgc.path();

    println!("Path: {}", path); // "."

}
