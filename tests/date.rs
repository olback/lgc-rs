extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Date};

/**
 * Date Tests
 */

#[test]
fn date_timestamp() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let timestamp = lgc.date.timestamp();

    assert!(timestamp > 0);

}

#[test]
fn date_utc_string() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let utc_string = lgc.date.utc_string();

    assert_eq!(utc_string.len(), 19);

}
