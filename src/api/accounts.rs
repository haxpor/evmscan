use crate::prelude::*;
use crate::types::evm_types::*;
use crate::environ::Context;

use isahc::prelude::*;
use url::Url;

/// Accounts namespace containing related APIs about accounts
pub struct Accounts;

impl Accounts {
    /// Get list of normal transactions
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `address` - target wallet or contract address to get list of normal transactions
    pub fn get_list_normal_transactions(&self, ctx: &Context, address: &str) -> Result<Vec::<EvmNormalTransactionResponseSuccessVariantResult>, EvmError>
    {
        type ResultType = EvmNormalTransactionResponseSuccessVariantResult;
        type JsonType = EvmTransactionResponse::<ResultType>;

        self.get_list_transactions::<ResultType, JsonType>(ctx, EvmApiResponseType::NormalTransaction, address)
    }

    /// Get list of internal transactions
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `address` - target wallet or contract address to get list of internal transactions
    pub fn get_list_internal_transactions(&self, ctx: &Context, address: &str) -> Result<Vec::<EvmInternalTransactionResponseSuccessVariantResult>, EvmError>
    {
        type ResultType = EvmInternalTransactionResponseSuccessVariantResult;
        type JsonType = EvmTransactionResponse::<ResultType>;

        self.get_list_transactions::<ResultType, JsonType>(ctx, EvmApiResponseType::InternalTransaction, address)
    }

    /// Internal generic function supporting to get list of transactions for both
    /// normal and internal ones.
    ///
    /// __NOTE__: Get normal and internal transaction APIs are limited to maximum of
    /// 10,000 transactions per-se page * offset must be less than or equal to 10,000.
    /// So it doesn't make sense to use this API for address which has more than
    /// 10,000 transactions.
    fn get_list_transactions<R, J>(&self, ctx: &Context, api_req_type: EvmApiResponseType, address: &str) -> Result<Vec::<R>, EvmError>
    where
        R: serde::de::DeserializeOwned,
        J: CompatibleTransactionResponse::<R> + serde::de::DeserializeOwned
    {
        let mut page_number = 1usize;
        let mut is_need_next_page = true;

        // with this number, we would max out at 5 pages
        // which is reasonable as the free rate limit is 5 requests per seconds.
        // It has high chance that < 5 requests will be made per seconds.
        const OFFSET: usize = 2000;

        // rate limit for free tier
        // See https://docs.bscscan.com/support/rate-limits
        const RATE_LIMIT: usize = 10_000;

        let mut ret_txs: Vec::<R> = Vec::new();

        while is_need_next_page {
            if page_number * OFFSET > RATE_LIMIT {
                eprintln!("{}", format!("WARNING: Address has more than {txs_limit} txs limit!", txs_limit=RATE_LIMIT));
                break;
            }

            // beware to always use fully qualified here for type of api_req_type
            let action = match &api_req_type {
                EvmApiResponseType::NormalTransaction => "txlist",
                EvmApiResponseType::InternalTransaction => "txlistinternal"
            };
            let raw_url_str = format!("{}/api?module=account&action={action}&address={target_address}&startblock=0&endblock=99999999&page={page}&offset={offset}&sort=asc&apikey={api_key}", Context::get_prefix_url(ctx.chain), action=action, target_address=address, api_key=ctx.api_key, page=page_number, offset=OFFSET);

            let url = match Url::parse(&raw_url_str) {
                Ok(res) => res,
                Err(_) => return Err(EvmError::ErrorInternalUrlParsing),
            };

            let request = match isahc::Request::get(url.as_str())
                .version_negotiation(isahc::config::VersionNegotiation::http2())
                .body(()) {
                Ok(res) => res,
                Err(e) => return Err(EvmError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
            };

            match isahc::send(request) {
                Ok(mut res) => {
                    // early return for non-200 HTTP returned code
                    if res.status() != 200 {
                        return Err(EvmError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                    }

                    // use the commented line, or just use what isahc provides conveniently
                    match res.json::<J>() {
                        Ok(json) => {
                            if json.status() == "1" {
                                // NOTE: unfortunate, we need to extract value from within enum
                                // https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
                                match json.result() {
                                    GenericEvmTransactionResponseResult::Success(mut c) => {
                                        if c.len() == 0 {
                                            is_need_next_page = false;
                                        }
                                        else if c.len() > 0 && c.len() < OFFSET {
                                            ret_txs.append(&mut c);
                                            is_need_next_page = false;
                                        }
                                        else {
                                            ret_txs.append(&mut c);
                                        }
                                    },
                                    // this case should not happen
                                    GenericEvmTransactionResponseResult::Failed(msg_opt) => {
                                        match msg_opt {
                                            Some(msg) => {
                                                return Err(EvmError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=msg)));
                                            },
                                            None => {
                                                return Err(EvmError::ErrorApiResponse(format!("un-expected error for success case")));
                                            }
                                        }
                                    }
                                }
                            }
                            else {
                                // exact text as returned when empty "result" is returned
                                if json.message() == "No transactions found" {
                                    break;
                                }
                                else {
                                    return Err(EvmError::ErrorApiResponse(format!("'{message}'", message=json.message())));
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return Err(EvmError::ErrorJsonParsing(None));
                        }
                    }
                },
                Err(e) => {
                    let err_msg = format!("{}", e);
                    return Err(EvmError::ErrorSendingHttpRequest(Some(err_msg)));
                }
            }

            if is_need_next_page {
                page_number = page_number + 1;
            }
            else {
                break;
            }
        }

        Ok(ret_txs)
    }

    /// Get balance of specified address.
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `address` - target wallet or contract address to get balance of
    pub fn get_balance_address(&self, ctx: &Context, address: &str) -> Result<U256, EvmError> {
        let raw_url_str = format!("{}/api?module=account&action=balance&address={target_address}&tag=latest&apikey={api_key}", Context::get_prefix_url(ctx.chain), target_address=address, api_key=ctx.api_key);

        let url = match Url::parse(&raw_url_str) {
            Ok(res) => res,
            Err(_) => return Err(EvmError::ErrorInternalUrlParsing),
        };

        let request = match isahc::Request::get(url.as_str())
            .version_negotiation(isahc::config::VersionNegotiation::http2())
            .body(()) {
            Ok(res) => res,
            Err(e) => return Err(EvmError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
        };

        match isahc::send(request) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(EvmError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<EvmNativeTokenBalanceResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                GenericEvmNativeTokenBalanceResponseResult::Success(bal) => Ok(bal),
                                GenericEvmNativeTokenBalanceResponseResult::Failed(result_msg) => {
                                    return Err(EvmError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)));
                                }
                            }
                        }
                        else {
                            // safely get text from "result" field
                            // this will ensure that the type of `json.result` is
                            // actually GenericEvmNativeTokenBalanceRespnseResult which is
                            // the failed case.
                            let result_text = match json.result {
                                GenericEvmNativeTokenBalanceResponseResult::Failed(txt) => Some(txt),
                                _ => None,
                            };

                            match result_text {
                                Some(txt) => {
                                    return Err(EvmError::ErrorApiResponse(format!("message:{}, result:{}", json.message, txt)));
                                },
                                None => {
                                    return Err(EvmError::ErrorApiResponse(format!("message:{}", json.message)));
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(EvmError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("{}", e);
                return Err(EvmError::ErrorSendingHttpRequest(Some(err_msg)));
            }
        }
    }

    /// Get balance from multiple addresses.
    /// It has internal check and will return `Err` accordingly if length of
    /// specified `addresses` slice is 0 or more than 20.
    ///
    /// It's better to error out instead of process only up to 20 to notify user
    /// using this function that the input might not be as expected.
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `addresses` - slice of literal string addresses.
    pub fn get_balance_addresses_multi(&self, ctx: &Context, addresses: &[&str]) -> Result<Vec<EvmNativeTokenBalanceMulti>, EvmError> {
        let addrs_len = addresses.len();

        if addrs_len == 0 {
            return Err(EvmError::ErrorParameter(Some("'count' needs to be more than 0".to_owned())));
        }
        if addrs_len > 20 {
            return Err(EvmError::ErrorParameter(Some("'count' cannot be more than 20".to_owned())));
        }

        // build string of addresses, up to 20 addresses
        let mut addresses_str: String = String::new(); 
        for i in 1..addrs_len {
            addresses_str.push_str(addresses[i-1]);
            addresses_str.push(',');
        }
        addresses_str.push_str(addresses[addrs_len-1]);

        let raw_url_str = format!("{}/api?module=account&action=balancemulti&address={addresses_str}&tag=latest&apikey={api_key}", Context::get_prefix_url(ctx.chain), addresses_str=&addresses_str, api_key=ctx.api_key);

        let url = match Url::parse(&raw_url_str) {
            Ok(res) => res,
            Err(_) => return Err(EvmError::ErrorInternalUrlParsing),
        };

        let request = match isahc::Request::get(url.as_str())
            .version_negotiation(isahc::config::VersionNegotiation::http2())
            .body(()) {
            Ok(res) => res,
            Err(e) => return Err(EvmError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
        };

        match isahc::send(request) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(EvmError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<EvmNativeTokenBalanceMultiResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                GenericEvmNativeTokenBalanceMultiResponseResult::Success(bal_records) => Ok(bal_records),
                                GenericEvmNativeTokenBalanceMultiResponseResult::Failed(result_msg) => Err(EvmError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)))
                            }
                        }
                        else {
                            // safely get text from "result" field
                            // this will ensure that the type of `json.result` is
                            // actually GenericEvmNativeTokenBalanceRespnseResult which is
                            // the failed case.
                            let result_text = match json.result {
                                GenericEvmNativeTokenBalanceMultiResponseResult::Failed(txt) => Some(txt),
                                _ => None,
                            };

                            match result_text {
                                Some(txt) => {
                                    return Err(EvmError::ErrorApiResponse(format!("message:{}, result:{}", json.message, txt)));
                                },
                                None => {
                                    return Err(EvmError::ErrorApiResponse(format!("message:{}", json.message)));
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(EvmError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("{}", e);
                return Err(EvmError::ErrorSendingHttpRequest(Some(err_msg)));
            }
        }
    }

    /// Get ERC-20/BEP-20 transfer events for `address` API request.
    /// This will return only records of transfer from `address`.
    ///
    /// **NOTE**: This function **doesn't** internally check whether the specified address is
    /// in fact EOA address, and not contract address. Thus it will return error
    /// instead.
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `address` - target wallet address. It should not be contract address as
    ///               internally it use `address` parameter to make a request.
    pub fn get_erc20_transfer_events_a(&self, ctx: &Context, address: &str) -> Result<Vec::<EvmErc20TokenTransferEventResponseSuccessVariantResult>, EvmError> {
        let mut page_number = 1u8;
        let mut is_need_next_page = true;
        const OFFSET: usize = 2000;

        let mut ret_txs: Vec::<EvmErc20TokenTransferEventResponseSuccessVariantResult> = Vec::new();
     
        while is_need_next_page {
            let raw_url_str = format!("{}/api?module=account&action=tokentx&address={target_address}&page={page}&offset={offset}&startblock=0&endblock=999999999&sort=asc&apikey={api_key}", Context::get_prefix_url(ctx.chain), target_address=address, page={page_number}, offset=OFFSET, api_key=ctx.api_key);

            let url = match Url::parse(&raw_url_str) {
                Ok(res) => res,
                Err(_) => return Err(EvmError::ErrorInternalUrlParsing),
            };

            let request = match isahc::Request::get(url.as_str())
                .version_negotiation(isahc::config::VersionNegotiation::http2())
                .body(()) {
                Ok(res) => res,
                Err(e) => return Err(EvmError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
            };

            match isahc::send(request) {
                Ok(mut res) => {
                    // early return for non-200 HTTP returned code
                    if res.status() != 200 {
                        return Err(EvmError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                    }

                    match res.json::<EvmErc20TokenTransferEventResponse>() {
                        Ok(json) => {
                            if json.status == "1" {
                                match json.result {
                                    GenericEvmErc20TokenTransferEventResponseResult::Success(mut c) => {
                                        if c.len() == 0 {
                                            is_need_next_page = false;
                                        }
                                        else if c.len() > 0 && c.len() < OFFSET {
                                            ret_txs.append(&mut c);
                                            is_need_next_page = false;
                                        }
                                        else {
                                            ret_txs.append(&mut c);
                                        }
                                    },
                                    // this case should not happen
                                    GenericEvmErc20TokenTransferEventResponseResult::Failed(msg) => {
                                        return Err(EvmError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=msg)));
                                    }
                                }
                            }
                            else {
                                // exact text as returned when empty "result" is returned
                                if json.message == "No transactions found" {
                                    break;
                                }
                                else {
                                    return Err(EvmError::ErrorApiResponse(format!("'{message}'", message=json.message)));
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return Err(EvmError::ErrorJsonParsing(None));
                        }
                    }
                },
                Err(e) => {
                    let err_msg = format!("{}", e);
                    return Err(EvmError::ErrorSendingHttpRequest(Some(err_msg)));
                }
            }

            if is_need_next_page {
                page_number = page_number + 1;
            }
            else {
                break;
            }
        }

        Ok(ret_txs)
    }
}
