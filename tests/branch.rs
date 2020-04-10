use last_git_commit::LastGitCommit;

/**
 * Branch Tests
 */

#[test]
fn branch_none() {

    let lgc = LastGitCommit::new().build().unwrap();
    let branch = lgc.branch();

    assert_eq!(branch, &"0.2".to_string());

}

#[test]
fn branch_custom() {

    let custom_branch = "_tests_";
    let lgc = LastGitCommit::new().set_branch(custom_branch).build().unwrap();
    let branch = lgc.branch();

    assert_eq!(branch, &custom_branch.to_string());
    assert_ne!(branch, &"master".to_string());

}
