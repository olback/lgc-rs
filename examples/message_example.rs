extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Message};

fn main() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let message = lgc.message();

    println!("Message: {}", message); // "this is a commit message"

}
