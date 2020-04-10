use last_git_commit::LastGitCommit;
use std::{env, path::PathBuf};

/**
 * Path Tests
 */

#[test]
fn path_none() {

    let lgc = LastGitCommit::new().build().unwrap();
    let path = lgc.path();

    assert_eq!(path, &PathBuf::from("."));

}

#[test]
fn path_custom() {

    let working_dir = format!("{}", env::current_dir().unwrap().display());
    let lgc = LastGitCommit::new().set_path(&working_dir).build().unwrap();
    let path = lgc.path();

    assert_eq!(path, &PathBuf::from(working_dir));
    assert_ne!(path, &PathBuf::from("."));

}
