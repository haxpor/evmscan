use crate::types::ChainType;

pub(crate) static BSCSCAN_PREFIX_URL: &str = "https://api.bscscan.com";
pub(crate) static ETHERSCAN_PREFIX_URL: &str = "https://api.etherscan.io";
pub(crate) static POLYGONSCAN_PREFIX_URL: &str = "https://api.polygonscan.com";

/// Context in interacting with API
pub struct Context {
    /// Which chain to be working with
    pub chain: ChainType,

    /// API key to be used
    pub api_key: String,
}

impl Context {
    /// Create a context instance
    ///
    /// # Arguments
    /// * `chain` - type of chain to be working with
    /// * `api_key` - api key
    pub fn create(chain: ChainType, api_key: String) -> Context {
        Context { chain: chain, api_key: api_key }
    }

    /// Return prefixed URL of different blockchain.
    /// Result is static string.
    ///
    /// # Arguments
    /// * `chain` - type of chain
    pub fn get_prefix_url(chain: ChainType) -> &'static str {
        match chain {
            ChainType::BSC => BSCSCAN_PREFIX_URL,
            ChainType::Ethereum => ETHERSCAN_PREFIX_URL,
            ChainType::Polygon => POLYGONSCAN_PREFIX_URL,
        }
    }
}
