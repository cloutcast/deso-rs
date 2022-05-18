use std::fmt;


#[derive(Debug)]
pub struct FromMnemonicError {
    message: String,
}

impl fmt::Display for FromMnemonicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn produce(message: String) -> FromMnemonicError {
    FromMnemonicError { message: message.to_string() }
}


