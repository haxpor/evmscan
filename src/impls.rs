use crate::prelude::*;

// follow the pattern as seen in std::env https://doc.rust-lang.org/src/std/env.rs.html#263-299
impl std::fmt::Display for BscError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            BscError::ErrorInternalGeneric(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error internal operation ({})", msg),
                    None => write!(f, "Error internal operation"),
                }
            },
            BscError::ErrorInternalUrlParsing => write!(f, "Error internal from parsing Url"),
            BscError::ErrorSendingHttpRequest => write!(f, "Error in sending HTTP request"),
            BscError::ErrorJsonParsing(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error in parsing JSON string ({})", msg),
                    None => write!(f, "Error in parsing JSON string"),
                }
            },
            BscError::ErrorApiResponse(ref msg) => write!(f, "Error api response from bscscan.com: {}", msg),
        }
    }
}

impl std::error::Error for BscError {}
