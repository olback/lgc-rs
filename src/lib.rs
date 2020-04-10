/// ! Get information about the last git commit, such as
/// ! git hash, author and time. Please see take a look at
/// ! the [examples] as they probably provide better docs
/// ! than this!
/// !
/// ! [examples]: https://github.com/olback/lgc-rs/tree/master/examples

use std::path::PathBuf;

mod author;
mod id;
mod builder;
use builder::LastGitCommitBuilder;

pub struct LastGitCommit {
    path: PathBuf,
    branch: String,
    message: Option<String>,
    author: author::Author,
    id: id::Id,
    timestamp: i64
}

impl LastGitCommit {

    /// Create new builder
    pub fn new() -> LastGitCommitBuilder {

        LastGitCommitBuilder::new()

    }

    /// Get path
    pub fn path(&self) -> &PathBuf {

        &self.path

    }

    /// Get branch
    pub fn branch(&self) -> &String {

        &self.branch

    }

    /// Get commit message
    pub fn message(&self) -> Option<&String> {

        self.message.as_ref()

    }

    /// Get commit author
    pub fn author(&self) -> &author::Author {

        &self.author

    }

    /// Get commit id (hash)
    pub fn id(&self) -> &id::Id {

        &self.id

    }

    /// Get commit timestamp
    pub fn timestamp(&self) -> i64 {

        self.timestamp

    }

}
