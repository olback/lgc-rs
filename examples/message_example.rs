use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let message = lgc.message().unwrap();

    println!("Message: {}", message); // "this is a commit message"

}
