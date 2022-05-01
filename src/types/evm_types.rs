use crate::prelude::*;
use crate::deserialize::{de_string_to_numeric,
                         de_string_to_U256,
                         de_string_to_bool,
                         de_constructor_arguments_string_to_vec_string};

/// Type of upstream server's API request
pub enum EvmApiResponseType {
    NormalTransaction,
    InternalTransaction
}

/// Structure that holds information from API response from bscscan.com
/// of normal transaction
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]  // source JSON response is in camelCase except
                                    // 'txreceipt_status' which we explicitly `rename` it.
pub struct EvmNormalTransactionResponseSuccessVariantResult {
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
pub struct EvmInternalTransactionResponseSuccessVariantResult {
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
pub struct EvmNativeTokenBalanceResponse {
    pub status: String,
    pub message: String,
    pub result: GenericEvmNativeTokenBalanceResponseResult,
}

/// Generic result for `result` field of `EvmNativeTokenBalanceResponse`.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericEvmNativeTokenBalanceResponseResult {
    #[serde(deserialize_with = "de_string_to_U256")]
    Success(U256),
    Failed(String),
}

/// Structure that holds balance for multiple addresses query via API
#[derive(Debug, serde::Deserialize)]
pub struct EvmNativeTokenBalanceMultiResponse {
    pub status: String,
    pub message: String,
    pub result: GenericEvmNativeTokenBalanceMultiResponseResult,
}

/// Generic result for `result` field of `EvmNativeTokenBalanceMultiResponse`.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericEvmNativeTokenBalanceMultiResponseResult {
    Success(Vec<EvmNativeTokenBalanceMulti>),
    Failed(String),
}

/// Structure which hold individual record of Getting native token balance for multiple
/// addresses API.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmNativeTokenBalanceMulti {
    /// Account address
    pub account: String,

    /// Balance in Wei
    #[serde(deserialize_with = "de_string_to_U256")]
    pub balance: U256,
}

/// Generic result as returned from `result` field from API response from bscscan.com
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericEvmTransactionResponseResult<T> {
    Success(Vec::<T>),
    Failed(Option<String>)
}

/// Common structure which has shared fields for API response from bscscan.com.
#[derive(Debug, serde::Deserialize)]
pub struct EvmTransactionResponse<T> {
    pub status: String,
    pub message: String,
    pub result: GenericEvmTransactionResponseResult::<T>,
}

/// Trait to satisfy implementing generic handling function for multiple API response
/// within one function.
pub trait CompatibleTransactionResponse<T> {
    fn status(&self) -> &str;
    fn message(&self) -> &str;
    fn result(&self) -> GenericEvmTransactionResponseResult::<T>;
}

/// Implementation of `CompatibleTransactionResponse` for
/// `EvmNormalTransactionResponseSuccessVariantResult`.
impl CompatibleTransactionResponse<EvmNormalTransactionResponseSuccessVariantResult> for EvmTransactionResponse<EvmNormalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericEvmTransactionResponseResult::<EvmNormalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

/// Implementation of `CompatibleTransactionResponse` for
/// `EvmInternalTransactionResponseSuccessVariantResult`.
impl CompatibleTransactionResponse<EvmInternalTransactionResponseSuccessVariantResult> for EvmTransactionResponse<EvmInternalTransactionResponseSuccessVariantResult>
{
    fn status(&self) -> &str {
        &self.status
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn result(&self) -> GenericEvmTransactionResponseResult::<EvmInternalTransactionResponseSuccessVariantResult> {
        self.result.clone()
    }
}

/// Structure holding returne API response of `result` field for ERC-20/BEP-20 tokens
/// transfer events
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvmErc20TokenTransferEventResponseSuccessVariantResult {
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

/// Structure holding information returned from API response for ERC-20/BEP-20 token
/// transfer event.
#[derive(Debug, serde::Deserialize)]
pub struct EvmErc20TokenTransferEventResponse {
    pub status: String,
    pub message: String,
    pub result: GenericEvmErc20TokenTransferEventResponseResult,
}

/// Structure holding variant of either success or failed returned for `result`
/// field of API response for Erc-20/BEP-20 token transfer event.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum GenericEvmErc20TokenTransferEventResponseResult {
    Success(Vec::<EvmErc20TokenTransferEventResponseSuccessVariantResult>),
    Failed(String)
}

/// Structure holding response back for Stats API's Get native token last price
#[derive(Debug, serde::Deserialize)]
pub struct EvmNativeTokenLastPriceResponse {
    pub status: String,
    pub message: String,
    pub result: EvmNativeTokenLastPriceResult,
}

/// Sturcture holding variant response for field 'result' of Stats API's
/// Get native token's last price.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum EvmNativeTokenLastPriceResult {
    /// Success case for main returning value of last price result.
    /// `EvmNativeTokanLastPrice_Polygon` will be mapped into this structure
    /// when querying last price on Polygon chain. That will be handled automatically
    /// and internally.
    Success(EvmNativeTokenLastPrice),

    /// Success case when querying for the price on Polygon chain. This is
    /// used internally only amidst can be accessed publicly.
    #[allow(non_camel_case_types)]
    Success_Polygon(EvmNativeTokenLastPrice_Polygon),

    Failed(String)
}

/// Actual structure holding a success response for Stats API's
/// Get native token's last price.
#[derive(Debug, serde::Deserialize)]
pub struct EvmNativeTokenLastPrice {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethbtc: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethbtc_timestamp: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethusd: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub ethusd_timestamp: u64,
}

/// Actual structure holding a success response for Stats API's
/// Get native token's last price for Polygon chain only.
///
/// NOTE: This is due to field names of response of this structure is not unique
/// compared to BSC, and Ethereum case. This library will internally handle
/// this unique case and reroute the data into main structure which is
/// `EvmNativeTokenLastPrice` structure in order to make it consistent in API.
///
/// NOTE2: Although accessiblity of this structure is public, but normally
/// users won't directly use this. It is used internally.
#[derive(Debug, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct EvmNativeTokenLastPrice_Polygon {
    #[serde(deserialize_with = "de_string_to_numeric")]
    pub maticbtc: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub maticbtc_timestamp: u64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub maticusd: f64,

    #[serde(deserialize_with = "de_string_to_numeric")]
    pub maticusd_timestamp: u64,
}

/// Contract ABI
#[derive(Debug, serde::Deserialize)]
pub struct EvmContractABIResponse {
    pub status: String,
    pub message: String,
    pub result: String,
}

/// Actual structure holding individual contract ABI.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EvmContractABIItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<EvmContractABIItemType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<EvmContractABIItemType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_mutability: Option<String>,

    pub r#type: String
}

/// Type definition for each ABI item
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EvmContractABIItemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_type: Option<String>,
    pub name: String,
    pub r#type: String,
}

/// Contract source code response
#[derive(Debug, serde::Deserialize)]
pub struct EvmContractSourceCodeResponse {
    pub status: String,
    pub message: String,
    pub result: EvmContractSourceCodeResult,
}

/// Structure holding variant response fro field `reuslt` of Contracts's
/// getting contract code API.
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum EvmContractSourceCodeResult {
    Success(Vec<EvmContractSourceCode>),

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
pub struct EvmContractSourceCode {
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
