use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct NoNoDataValueError {}

impl Error for NoNoDataValueError {}

impl fmt::Display for NoNoDataValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}
