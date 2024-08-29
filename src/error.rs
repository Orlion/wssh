use std::fmt;

#[derive(Debug)]
pub struct WsshError {
    message: String,
}

pub fn from_str(message: &str) -> WsshError {
    WsshError {
        message: message.to_string(),
    }
}

pub fn from_string(message: String) -> WsshError {
    WsshError { message }
}

impl fmt::Display for WsshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
