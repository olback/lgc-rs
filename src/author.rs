pub struct Author {
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>
}

impl Author {

    pub fn name(&self) -> Option<&String> {

        self.name.as_ref()

    }

    pub fn email(&self) -> Option<&String> {

        self.email.as_ref()

    }

}
