extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Author};

fn main() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let name = lgc.author.name();
    let email = lgc.author.email();

    println!("Name: {}", name); // "Jon Doe"
    println!("Email: {}", email); // "jondoe@example.com"

}
