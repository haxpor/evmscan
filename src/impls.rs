use crate::types::*;

use std::env::VarError;

// follow the pattern as seen in std::env https://doc.rust-lang.org/src/std/env.rs.html#263-299
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            AppError::ErrorInternalGeneric(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error internal operation ({})", msg),
                    None => write!(f, "Error internal operation"),
                }
            },
            AppError::ErrorInternalUrlParsing => write!(f, "Error internal from parsing Url"),
            AppError::ErrorSendingHttpRequest => write!(f, "Error in sending HTTP request"),
            AppError::ErrorNoApiKey => write!(f, "Error no api-key defined via environment variable HX_INOUTFLOW_API_KEY"),
            AppError::ErrorApiKeyNotUnicode => write!(f, "Api key is defined, but it is not unicode."),
            AppError::ErrorJsonParsing(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error in parsing JSON string ({})", msg),
                    None => write!(f, "Error in parsing JSON string"),
                }
            },
            AppError::ErrorApiResponse(ref msg) => write!(f, "Error api response from bscscan.com: {}", msg),
			AppError::ErrorNotEnoughArgumentsSuppliedAtCommandline(ref msg) => {
                match msg {
                    Some(msg) => write!(f, "Error not enough arguments supplied at commandline ({})", msg),
                    None => write!(f, "Error not enough arguments supplied at commandline.")
                }
            }
        }
    }
}

impl std::error::Error for AppError {}

impl std::convert::From<VarError> for AppError {
    fn from(f: VarError) -> Self {
        match f {
            VarError::NotPresent => AppError::ErrorNoApiKey,
            // NOTE: can also use .. but it has different semantics
            // .. means range of all arguments, although in this case we just have one
            VarError::NotUnicode(_) => AppError::ErrorApiKeyNotUnicode,
        }
    }
}
