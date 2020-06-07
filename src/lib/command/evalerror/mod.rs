use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct EvalError {
    message: String,
}

impl Error for EvalError {}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl EvalError {
    pub fn new(message: &str) -> EvalError {
        EvalError {
            message: String::from(message),
        }
    }

    pub fn from_string(message: String) -> EvalError {
        EvalError { message }
    }

    pub fn message(&self) -> & String {
        & self.message
    }
}