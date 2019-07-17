extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Path};
use std::env;

/**
 * Path Tests
 */

#[test]
fn path_none() {

    let path = LastGitCommit::new(None, None).unwrap().path();

    assert_eq!(path, ".".to_string())

}

#[test]
fn path_custom() {

    let working_dir = format!("{}", env::current_dir().unwrap().display());
    let path = LastGitCommit::new(Some(&working_dir), None).unwrap().path();

    assert_eq!(path, working_dir)

}
