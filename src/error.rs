use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    Error(String )
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Error(e) => write!(f, "{}", e) 
        }
    }
}
