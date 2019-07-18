extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Date};

fn main() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let timestamp = lgc.date.timestamp();
    let utc_string = lgc.date.utc_string();

    println!("Timestamp: {}", timestamp); // 1563461711
    println!("UTC String: {}", utc_string); // "2019-07-18 14:55:18"

}
