pub struct Id(pub(crate) String);

impl Id {

    pub fn short(&self) -> String {

        self.0.clone().get(0..7).unwrap().into()

    }

    pub fn long(&self) -> String {

        self.0.clone()

    }

    pub fn range(&self, range: std::ops::Range<usize>) -> Option<String> {

        self.0.clone().get(range).map(|id| id.into())

    }

}
