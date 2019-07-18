extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Branch};

fn main() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let branch = lgc.branch();

    println!("Bath: {}", branch); // "master"

}
