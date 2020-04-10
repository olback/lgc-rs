use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let path = lgc.path();

    println!("Path: {:?}", path); // "."

}
