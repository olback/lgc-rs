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

    /// Get the path to the repository.  
    /// (same as passed to new())
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Path};
    ///
    /// let path = LastGitCommit::new(None, None).unwrap().path();
    ///
    /// println!("Path: {}", path);
    /// ```
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

    /// Get the selected branch.  
    /// (same as passed to new())
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Branch};
    ///
    /// let branch = LastGitCommit::new(None, None).unwrap().branch();
    ///
    /// println!("Branch: {}", branch);
    /// ```
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

    /// Get last commit message.
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Message};
    ///
    /// let message = LastGitCommit::new(None, None).unwrap().message();
    ///
    /// println!("Message: {}", message);
    /// ```
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

    /// Get the name of the commit author.
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Author};
    ///
    /// let name = LastGitCommit::new(None, None).unwrap().author.name();
    ///
    /// println!("Name: {}", name);
    /// ```
    fn name(&self) -> String {
        self._name.clone()
    }

    /// Get the email of the commit author.
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Author};
    ///
    /// let email = LastGitCommit::new(None, None).unwrap().author.email();
    ///
    /// println!("Email: {}", email);
    /// ```
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

    /// Get all 40 characters of the SHA1 git hash.
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Id};
    ///
    /// let long = LastGitCommit::new(None, None).unwrap().id.long();
    ///
    /// println!("SHA1 Hash: {}", long);
    /// assert_eq!(long.len(), 40);
    /// ```
    fn long(&self) -> String {
        self._id.clone()
    }

    /// Get all the 7 first characters of the SHA1 git hash.  
    /// This is what is shown on GitHub.
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Id};
    ///
    /// let short = LastGitCommit::new(None, None).unwrap().id.short();
    ///
    /// println!("SHA1 Hash: {}", short);
    /// assert_eq!(short.len(), 7);
    /// ```
    fn short(&self) -> String {
        self._id.get(0..7).unwrap_or("<invalid git id/hash>").to_string()
    }

    /// Define your own range of the SHA1 git hash.  
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Id};
    ///
    /// // Get the middle 20 characters of the hash.
    /// let range = LastGitCommit::new(None, None).unwrap().id.range(10..30);
    ///
    /// println!("SHA1 Hash: {}", range);
    /// assert_eq!(range.len(), 20);
    /// ```
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

    /// A date and time formatted string.  
    /// Format: "%Y-%m-%d %H:%M:%S
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Date};
    ///
    /// let utc_string = LastGitCommit::new(None, None).unwrap().date.utc_string();
    ///
    /// println!("UTC String: {}", utc_string);
    /// ```
    fn utc_string(&self) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(self._timestamp as u64);
        let dt = DateTime::<Utc>::from(d);
        format!("{}", dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    /// Unix Timestamp
    ///
    /// # Examples
    /// ```rust
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit, Date};
    ///
    /// let timestamp = LastGitCommit::new(None, None).unwrap().date.timestamp();
    ///
    /// println!("Timestamp: {}", timestamp);
    /// ```
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
    ///
    /// `path`: Path to git repository. `None` defaults to current directory.
    ///
    /// `branch`: Branch to use. `None` defaults to `master`.
    ///
    /// # Examples
    /// ```rust,should_panic
    /// extern crate last_git_commit;
    /// use last_git_commit::{LastGitCommit};
    /// let lgc = LastGitCommit::new(None, None).unwrap(); // paht: ".", branch: "master"
    /// let lgc = LastGitCommit::new(Some("my/path/to/other/git/repo"), None).unwrap();
    /// let lgc = LastGitCommit::new(None, Some("my-other-branch")).unwrap();
    /// let lgc = LastGitCommit::new(Some("my/path/to/other/git/repo"), Some("my-other-branch")).unwrap();
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
            },
            date: LGCDate {
                _timestamp: commit.time().seconds()
            }
        };

        Ok(lgc)

    }

}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
