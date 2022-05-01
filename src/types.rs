pub mod evm_types;

/// List of possible error as occurs from the operations
#[derive(Debug)]
pub enum Error {
    /// Internal error for generic error combined altogether
    /// Contain optional error message
    ErrorInternalGeneric(Option<String>),

    /// Internal error from parsing Url
    ErrorInternalUrlParsing,

    /// Error in sending HTTP request
    /// Contains optional error message
    ErrorSendingHttpRequest(Option<String>),

    /// Error JSON parsing
    /// Contain optional error message
    ErrorJsonParsing(Option<String>),

    /// Error from Api response back from upstream API server containing the error message
    ErrorApiResponse(String),

    /// Parameter to function error
    ErrorParameter(Option<String>),
}

/// Chain type
#[derive(Copy, Clone)]
pub enum ChainType {
    /// Binance Smart Chain
    BSC,

    /// Ethereum
    Ethereum,

    /// Polygon
    Polygon,
}
