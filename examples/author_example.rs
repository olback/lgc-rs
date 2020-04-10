use last_git_commit::LastGitCommit;

fn main() {

    let lgc = LastGitCommit::new().build().unwrap();
    let name = lgc.author().name().unwrap();
    let email = lgc.author().email().unwrap();

    println!("Name: {}", name); // "Jon Doe"
    println!("Email: {}", email); // "jondoe@example.com"

}
