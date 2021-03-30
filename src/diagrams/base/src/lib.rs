use std::fmt;

#[derive(Debug)]
pub enum TransformError {
    ParseError(String),
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransformError::ParseError(msg) => write!(f, "Invalid syntax:\n{}", msg),
        }
    }
}
