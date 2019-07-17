extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Author};

/**
 * Author Tests
 */

#[test]
fn author_name() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let name = lgc.author.name();

    assert!(name.len() > 0);
    assert_ne!(name, "<unknown>".to_string());

}

#[test]
fn author_email() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let email = lgc.author.email();

    assert!(email.len() > 0);
    assert_ne!(email, "<unknown>".to_string());

}
