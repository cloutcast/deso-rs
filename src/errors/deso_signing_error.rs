use std::fmt;


#[derive(Debug)]
pub struct DesoSigningError {
    message: String,
}

impl fmt::Display for DesoSigningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn produce(message: String) -> DesoSigningError {
    DesoSigningError { message: message.to_string() }
}


