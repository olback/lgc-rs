use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let timestamp = lgc.timestamp();

    println!("Timestamp: {}", timestamp); // 1563461711

}
