use last_git_commit::LastGitCommit;

/**
 * Id Tests
 */
#[test]
fn message() {

    let lgc = LastGitCommit::new().build().unwrap();
    let message = lgc.message();

    assert!(message.is_some());

}
