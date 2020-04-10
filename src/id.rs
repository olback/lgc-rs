pub struct Id(pub(crate) String);

impl Id {

    /// Get all the 7 first characters of the SHA1 git hash.  
    /// This is what is shown on GitHub.
    pub fn short(&self) -> String {

        self.0.clone().get(0..7).unwrap().into()

    }

    /// Get all 40 characters of the SHA1 git hash.
    pub fn long(&self) -> String {

        self.0.clone()

    }

    /// Define your own range of the SHA1 git hash. None if the range is invalid.
    pub fn range(&self, range: std::ops::Range<usize>) -> Option<String> {

        self.0.clone().get(range).map(|id| id.into())

    }

}
