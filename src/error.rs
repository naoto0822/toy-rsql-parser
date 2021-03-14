use crate::annot::Annot;

#[derive(Clone, Debug, PartialEq)]
pub enum LexErrorType {
    InvalidChar(String),
}

pub type LexError = Annot<LexErrorType>;

impl LexError {
    pub fn invalid_char(c: String) -> Self {
        LexError::new(LexErrorType::InvalidChar(c))
    }
}
