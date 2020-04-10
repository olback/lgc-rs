use std::path::PathBuf;
use super::{LastGitCommit, author::Author, id::Id};

pub struct LastGitCommitBuilder {
    path: Option<PathBuf>,
    branch: Option<String>
}

impl LastGitCommitBuilder {

    pub fn new() -> Self {

        Self {
            path: None,
            branch: None
        }

    }

    pub fn set_path<P: Into<PathBuf>>(mut self, path: P) -> Self {

        self.path = Some(path.into());

        self

    }

    pub fn set_branch<B: Into<String>>(mut self, branch: B) -> Self {

        self.branch = Some(branch.into());

        self

    }

    pub fn build(self) -> Result<LastGitCommit, git2::Error> {

        let path = self.path.unwrap_or(PathBuf::from("."));
        let repo = git2::Repository::open(&path)?;
        let branch = self.branch.unwrap_or(Self::get_branch(&repo)?);
        let object = repo.revparse_single(&branch)?;
        let commit = object.peel_to_commit()?;

        let message = commit.message().map(|m| m.to_string());
        let name = commit.author().name().map(|n| n.to_string());
        let email = commit.author().email().map(|e| e.to_string());

        Ok(LastGitCommit {
            path: path,
            branch: branch,
            message: message,
            author: Author {
                name: name,
                email: email
            },
            id: Id(commit.id().to_string()),
            timestamp: commit.time().seconds()
        })

    }

    fn get_branch(repo: &git2::Repository) -> Result<String, git2::Error> {

        let branches = repo.branches(None)?;

        for branch in branches {

            match branch {

                Ok(b) => {

                    if b.0.is_head() {

                        let name = b.0.name()?.unwrap();
                        return Ok(String::from(name))

                    }

                },

                Err(_) => {}

            }

        }

        return Err(git2::Error::from_str("could not determine current branch"));

    }

}
