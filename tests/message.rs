extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Message};

/**
 * Id Tests
 */
#[test]
fn message() {

    let message = LastGitCommit::new(None, None).unwrap().message();

    assert!(message.len() > 0);
    assert_ne!(message, "<no commit message>".to_string());

}
