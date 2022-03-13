pub mod bsc_types;

/// List of possible this program's error types.
#[derive(Debug)]
pub enum AppError {
    /// Internal error for generic error combined altogether
    /// Contain optional error message
    ErrorInternalGeneric(Option<String>),

    /// Internal error from parsing Url
    ErrorInternalUrlParsing,

    /// Error in sending HTTP request
    ErrorSendingHttpRequest,

    /// Error due to no api-key defined via environment variable HX_INOUTFLOW_API_KEY
    ErrorNoApiKey,

    /// Api key defined but it is not unicode
    ErrorApiKeyNotUnicode,

    /// Error JSON parsing
    /// Contain optional error message
    ErrorJsonParsing(Option<String>),

    /// Error from Api response back from bscscan.com containing the error message
    ErrorApiResponse(String),

	/// Error not enough arguments supplied at command line
    /// Contain optional message for error.
	ErrorNotEnoughArgumentsSuppliedAtCommandline(Option<String>),
}
