use last_git_commit::LastGitCommit;

/**
 * Date Tests
 */

#[test]
fn date_timestamp() {

    let lgc = LastGitCommit::new().build().unwrap();
    let timestamp = lgc.timestamp();

    assert!(timestamp > 0);

}
