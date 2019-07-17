extern crate last_git_commit;
use last_git_commit::{LastGitCommit, /*Path,*/ Branch, Message, Author, Id, Date};


/**
 * Id Tests
 */
#[test]
fn id_long() {

    let lgc = LastGitCommit::new(None, None).unwrap();

    let long = lgc.id.long();

    assert_eq!(long.len(), 40)

}
