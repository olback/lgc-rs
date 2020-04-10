use last_git_commit::LastGitCommit;

/**
 * Author Tests
 */

#[test]
fn author_name() {

    let lgc = LastGitCommit::new().build().unwrap();
    let name = lgc.author().name();

    assert!(name.is_some());

}

#[test]
fn author_email() {

    let lgc = LastGitCommit::new().build().unwrap();
    let email = lgc.author().email();

    assert!(email.is_some());

}
