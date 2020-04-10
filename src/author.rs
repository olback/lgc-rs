pub struct Author {
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>
}

impl Author {

    /// Get commit authors name, if any
    pub fn name(&self) -> Option<&String> {

        self.name.as_ref()

    }

    /// Get commit authors email, if any
    pub fn email(&self) -> Option<&String> {

        self.email.as_ref()

    }

}
