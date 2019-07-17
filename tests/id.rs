extern crate last_git_commit;
use last_git_commit::{LastGitCommit, Id};

/**
 * Id Tests
 */

#[test]
fn id_long() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let long = lgc.id.long();

    assert!(long.len() == 40);

}

#[test]
fn id_short() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let short = lgc.id.short();
    let long = lgc.id.long();

    assert!(short.len() == 7);
    assert_eq!(short, long.get(0..7).unwrap().to_string());

}

#[test]
fn id_custom_len() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let short = lgc.id.short();
    let long = lgc.id.long();
    let range = lgc.id.range(0..3);

    assert!(range.len() == 3);
    assert_eq!(short.get(0..3).unwrap().to_string(), range);
    assert_eq!(long.get(0..3).unwrap().to_string(), range);

}

#[test]
fn id_custom_our() {

    let lgc = LastGitCommit::new(None, None).unwrap();
    let range = lgc.id.range(30..41);

    assert_eq!(range, "out of range".to_string());

}
