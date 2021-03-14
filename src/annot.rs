#[derive(Clone, Debug, PartialEq)]
pub struct Annot<T> {
    pub value: T,
}

// TODO: add Location(line, position)
impl<T> Annot<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
