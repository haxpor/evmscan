use crate::prelude::*;

// follow the pattern as seen in std::env https://doc.rust-lang.org/src/std/env.rs.html#263-299
impl std::fmt::Display for EvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            EvmError::ErrorInternalGeneric(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error internal operation ({})", msg),
                    None => write!(f, "Error internal operation"),
                }
            },
            EvmError::ErrorInternalUrlParsing => write!(f, "Error internal from parsing Url"),
            EvmError::ErrorSendingHttpRequest(ref msg_opt) => {
                match msg_opt {
                    Some(msg) => write!(f, "Error in sending HTTP request; err={}", msg),
                    None => write!(f, "Error in sending HTTP request"),
                }
            },
            EvmError::ErrorJsonParsing(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error in parsing JSON string ({})", msg),
                    None => write!(f, "Error in parsing JSON string"),
                }
            },
            EvmError::ErrorApiResponse(ref msg) => write!(f, "Error api response from upstream server: {}", msg),
            EvmError::ErrorParameter(ref msg_optional) => {
                match msg_optional {
                    Some(msg) => write!(f, "Invalid parameter: {}", msg),
                    None => write!(f, "Invalid parameter"),
                }
            },
        }
    }
}

impl std::error::Error for EvmError {}
