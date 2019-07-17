extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Branch};

/**
 * Branch Tests
 */

#[test]
fn branch_none() {

    let branch = LastGitCommit::new(None, None).unwrap().branch();

    assert_eq!(branch, "master".to_string())

}

#[test]
fn branch_custom() {

    let custom_branch = "_tests_";
    let branch = LastGitCommit::new(None, Some(&custom_branch)).unwrap().branch();

    assert_eq!(branch, custom_branch.to_string())

}
