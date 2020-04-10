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

    pub fn new() -> LastGitCommitBuilder {

        LastGitCommitBuilder::new()

    }

    pub fn path(&self) -> &PathBuf {

        &self.path

    }

    pub fn branch(&self) -> &String {

        &self.branch

    }

    pub fn message(&self) -> Option<&String> {

        self.message.as_ref()

    }

    pub fn author(&self) -> &author::Author {

        &self.author

    }

    pub fn id(&self) -> &id::Id {

        &self.id

    }

    pub fn timestamp(&self) -> i64 {

        self.timestamp

    }

}





