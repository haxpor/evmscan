use crate::prelude::*;
use crate::deserialize::{de_string_to_numeric,
                         de_string_to_U256,
                         de_string_to_bool,
                         de_constructor_arguments_string_to_vec_string};

/// Type of bscscan.com's API request
pub enum BSCApiResponseType {
    NormalTransaction,
    InternalTransaction
}

/// Structure that holds information from API response from bscscan.com
/// of normal transaction
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]  // source JSON response is in camelCase except
                                    // 'txreceipt_status' which we explicitly `rename` it.
pub struct BSCNormalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "timeStamp")]
    pub timestamp: u64,

    pub hash: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub nonce: u32,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub transaction_index: u64,

    pub from: String,

    pub to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    pub value: U256,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_price: u64,

    #[serde(deserialize_with = "de_string_to_bool")]
    pub is_error: bool,

    #[serde(rename = "txreceipt_status")]
    pub txreceipt_status: String,

    pub input: String,

    pub contract_address: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub cumulative_gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub confirmations: u32,
}

/// Structure that holds information from API response from bscscan.com
/// of internal transaction
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSCInternalTransactionResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "timeStamp")]
    pub timestamp: u64,

    pub hash: String,

    pub from: String,

    pub to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    pub value: U256,

    pub contract_address: String,

    pub input: String,

    // this is how to escape reserved keyword to use as identifier
    pub r#type: Option<String>,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_used: u64,

    pub trace_id: Option<String>,

    #[serde(deserialize_with = "de_string_to_bool")]
    pub is_error: bool,

    pub err_code: Option<String>
}

/// Structure that holds account balance
#[derive(Debug, serde::Deserialize)]
pub struct BSCBnbBalanceResponse {
    pub status: String,
    pub message: String,
    pub result: GenericBSCBnbBalanceResponseResult,
}

/// Generic result for `result` field of `BSCBnbBalanceResponse`.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericBSCBnbBalanceResponseResult {
    #[serde(deserialize_with = "de_string_to_U256")]
    Success(U256),
    Failed(String),
}

/// Structure that holds balance for multiple addresses query via API
#[derive(Debug, serde::Deserialize)]
pub struct BSCBnbBalanceMultiResponse {
    pub status: String,
    pub message: String,
    pub result: GenericBSCBnbBalanceMultiResponseResult,
}

/// Generic result for `result` field of `BSCBnbBalanceMultiResponse`.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericBSCBnbBalanceMultiResponseResult {
    Success(Vec<BSCBnbBalanceMulti>),
    Failed(String),
}

/// Structure which hold individual record of Getting BNB balance for multiple
/// addresses API.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSCBnbBalanceMulti {
    /// Account address
    pub account: String,

    /// Balance in Wei
    #[serde(deserialize_with = "de_string_to_U256")]
    pub balance: U256,
}

/// Generic result as returned from `result` field from API response from bscscan.com
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericBSCTransactionResponseResult<T> {
    Success(Vec::<T>),
    Failed(Option<String>)
}

/// Common structure which has shared fields for API response from bscscan.com.
#[derive(Debug, serde::Deserialize)]
pub struct BSCTransactionResponse<T> {
    pub status: String,
    pub message: String,
    pub result: GenericBSCTransactionResponseResult::<T>,
}

/// Trait to satisfy implementing generic handling function for multiple API response
/// within one function.
pub trait CompatibleTransactionResponse<T> {
    fn status(&self) -> &str;
    fn message(&self) -> &str;
    fn result(&self) -> GenericBSCTransactionResponseResult::<T>;
}

/// Implementation of `CompatibleTransactionResponse` for
/// `BSCNormalTransactionResponseSuccessVariantResult`.
impl CompatibleTransactionResponse<BSCNormalTransactionResponseSuccessVariantResult> for BSCTransactionResponse<BSCNormalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericBSCTransactionResponseResult::<BSCNormalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

/// Implementation of `CompatibleTransactionResponse` for
/// `BSCInternalTransactionResponseSuccessVariantResult`.
impl CompatibleTransactionResponse<BSCInternalTransactionResponseSuccessVariantResult> for BSCTransactionResponse<BSCInternalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericBSCTransactionResponseResult::<BSCInternalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

/// Structure holding returne API response of `result` field for BEP-20 tokens
/// transfer events
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSCBep20TokenTransferEventResponseSuccessVariantResult {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub block_number: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "timeStamp")]
    pub timestamp: u64,

    pub hash: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub nonce: u32,

    pub block_hash: String,

    pub from: String,

    pub contract_address: String,

    pub to: String,

    #[serde(deserialize_with = "de_string_to_U256")]
    pub value: U256,

    pub token_name: String,

    pub token_symbol: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub token_decimal: u8,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub transaction_index: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_price: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub gas_used: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub cumulative_gas_used: u64,

    pub input: String,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub confirmations: u32,
}

/// Structure holding information returned from API response for BEP-20 token
/// transfer event.
#[derive(Debug, serde::Deserialize)]
pub struct BSCBep20TokenTransferEventResponse {
    pub status: String,
    pub message: String,
    pub result: GenericBSCBep20TokenTransferEventResponseResult,
}

/// Structure holding variant of either success or failed returned for `result`
/// field of API response for BEP-20 token transfer event.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericBSCBep20TokenTransferEventResponseResult {
    Success(Vec::<BSCBep20TokenTransferEventResponseSuccessVariantResult>),
    Failed(String)
}

/// Structure holding response back for Stats API's Get BNB last price
#[derive(Debug, serde::Deserialize)]
pub struct BSCBnbLastPriceResponse {
    pub status: String,
    pub message: String,
    pub result: BSCBnbLastPriceResult,
}

/// Sturcture holding variant response for field 'result' of Stats API's
/// Get BNB last price.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum BSCBnbLastPriceResult {
    Success(BSCBnbLastPrice),
    Failed(String)
}

/// Actual structure holding a success response for Stats API's
/// Get BNB last price.
#[derive(Debug, serde::Deserialize)]
pub struct BSCBnbLastPrice {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethbtc: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethbtc_timestamp: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethusd: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethusd_timestamp: u64,
}

/// Contract source code response
#[derive(Debug, serde::Deserialize)]
pub struct BSCContractSourceCodeResponse {
    pub status: String,
    pub message: String,
    pub result: BSCContractSourceCodeResult,
}

/// Structure holding variant response fro field `reuslt` of Contracts's
/// getting contract code API.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum BSCContractSourceCodeResult {
    Success(Vec<BSCContractSourceCode>),

    /// This also includes the case of querying for non-verified source code.
    /// Although it is not error / failed case per-se as its `abi` field will
    /// contain exactly "Contract source code not verified". But it is included
    /// as failed case as well.
    Failed(String),
}

/// Actual structure holding contract's verified source code
/// If such contract doesn't verify source code, then most fields will be empty.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BSCContractSourceCode {
    /// Actual smart contract source code
    #[serde(rename = "SourceCode")]
    pub source_code: String,

    /// Contract ABI
    #[serde(rename = "ABI")]
    pub abi: String,

    /// Contract name
    #[serde(rename = "ContractName")]
    pub contract_name: String,

    /// Compiler version
    #[serde(rename = "CompilerVersion")]
    pub compiler_version: String,

    /// Whether or not optimization has been applied
    #[serde(deserialize_with = "de_string_to_bool")]
    #[serde(rename = "OptimizationUsed")]
    pub optimization_used: bool,

    /// Number of runs as part of optimization
    #[serde(deserialize_with = "de_string_to_numeric")]
    #[serde(rename = "Runs")]
    pub runs: u32,

    /// Constructor's arguments
    #[serde(deserialize_with = "de_constructor_arguments_string_to_vec_string")]
    #[serde(rename = "ConstructorArguments")]
    pub constructor_arguments: Vec<String>,

    /// EVM version
    #[serde(rename = "EVMVersion")]
    pub evm_version: String,

    /// Library used by this constract
    /// FIXME: For now, returned as a whole as string, we need to find an example of
    /// contract which contains non-empty of this field.
    #[serde(rename = "Library")]
    pub library: String,

    /// License type
    #[serde(rename = "LicenseType")]
    pub license_type: String,

    /// Whether or not this contract is the proxy, if so then `implementation`
    /// field contains the actual implementation address.
    #[serde(deserialize_with = "de_string_to_bool")]
    #[serde(rename = "Proxy")]
    pub proxy: bool,

    /// Contract address that is the implementation for this contract as it is
    /// acting as a proxy.
    #[serde(rename = "Implementation")]
    pub implementation: String,

    /// URL to swarm source
    #[serde(rename = "SwarmSource")]
    pub swarm_source: String,
}
