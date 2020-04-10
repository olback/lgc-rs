use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let branch = lgc.branch();

    println!("Branch: {}", branch); // "master"

}
