pub trait Contains<P: PartialEq> {
	/// Test if the target contains `other`.
	fn contains(&self, other: &P) -> bool;
}

impl <P: PartialEq> Contains<P> for Option<P> {
    fn contains(&self, other: &P) -> bool {
        match self.as_ref() {
            Some(s) => s.eq(other),
            None => false,
        }
    }
}
