extern crate git2;
extern crate chrono;

use git2::{Repository};
use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

/**
 * Path
 */
pub trait Path {
    fn path(&self) -> String;
}

impl Path for LastGitCommit {
    fn path(&self) -> String {
        self._path.clone()
    }
}


/**
 * Branch
 */
pub trait Branch {
    fn branch(&self) -> String;
}

impl Branch for LastGitCommit {
    fn branch(&self) -> String {
        self._branch.clone()
    }
}


/**
 * Message
 */
pub trait Message {
    fn message(&self) -> String;
}

impl Message for LastGitCommit {
    fn message(&self) -> String {
        self._message.clone()
    }
}


/**
 * Author
 */
pub struct LGCAuthor {
    _name: String,
    _email: String
}

pub trait Author {
    fn name(&self) -> String;
    fn email(&self) -> String;
}

impl Author for LGCAuthor {

    fn name(&self) -> String {
        self._name.clone()
    }

    fn email(&self) -> String {
        self._email.clone()
    }

}


/**
 * Id
 */
pub struct LGCId {
    _id: String
}

pub trait Id {
    fn long(&self) -> String;
    fn short(&self) -> String;
    fn range(&self, range: std::ops::Range<usize>) -> String;
}

impl Id for LGCId {

    fn long(&self) -> String {
        self._id.clone()
    }

    fn short(&self) -> String {
        self._id.get(0..7).unwrap_or("<invalid git id/hash>").to_string()
    }

    fn range(&self, range: std::ops::Range<usize>) -> String {
        self._id.get(range).unwrap_or("out of range").to_string()
    }

}


/**
 * Date
 */
pub struct LGCDate {
    _timestamp: i64
}

pub trait Date {
    fn utc_string(&self) -> String;
    fn timestamp(&self) -> i64;
}

impl Date for LGCDate {

    fn utc_string(&self) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(self._timestamp as u64);
        let dt = DateTime::<Utc>::from(d);
        format!("{}", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn timestamp(&self) -> i64 {
        self._timestamp
    }

}


/**
 * LastGitCommit
 */
pub struct LastGitCommit {
    _path: String,
    _branch: String,
    _message: String,
    pub author: LGCAuthor,
    pub id: LGCId,
    pub date: LGCDate
}

impl LastGitCommit {

    /// # LastGitCommit
    /// A simple wrapper arround git2-rs
    ///
    /// `path`: Path to git repository. `None` defaults to current directory.
    ///
    /// `branch`: Branch to use. `None` defaults to `master`.
    ///
    /// # Examples
    /// ```
    /// let lgc = LastGitCommit::new(None, None).unwrap();
    /// let lgc = LastGitCommit::new("my/path/to/other/git/repo", None).unwrap();
    /// let lgc = LastGitCommit::new(None, "my-other-branch").unwrap();
    /// let lgc = LastGitCommit::new("my/path/to/other/git/repo", "my-other-branch").unwrap();
    /// ```
    pub fn new(path: Option<&str>, branch: Option<&str>) -> Result<LastGitCommit, git2::Error> {

        let path = path.unwrap_or(".");
        let branch = branch.unwrap_or("master");

        let repo = match Repository::open(path) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let object = match repo.revparse_single(branch) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let commit = match object.peel_to_commit() {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        let lgc = LastGitCommit {
            _path: path.to_string(),
            _branch: branch.to_string(),
            _message: commit.message().unwrap_or("<no commit message>").to_string(),
            author: LGCAuthor {
                _name: commit.author().name().unwrap_or("<unknown>").to_string(),
                _email: commit.author().email().unwrap_or("<unknown>").to_string()
            },
            id: LGCId {
                _id: format!("{}", commit.id())
                // long: format!("{}", commit.id()),
                // short: format!("{}", commit.id()).get(0..7).unwrap_or("<invalid git id/hash>").to_string()
            },
            date: LGCDate {
                _timestamp: commit.time().seconds()
            }
        };

        Ok(lgc)

    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
