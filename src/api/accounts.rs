use crate::prelude::*;
use crate::types::bsc_types::*;
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
    pub fn get_list_normal_transactions(&self, ctx: &Context, address: &str) -> Result<Vec::<BSCNormalTransactionResponseSuccessVariantResult>, AppError>
    {
        type ResultType = BSCNormalTransactionResponseSuccessVariantResult;
        type JsonType = BSCTransactionResponse::<ResultType>;

        self.get_list_transactions::<ResultType, JsonType>(ctx, BSCApiResponseType::NormalTransaction, address)
    }

    /// Get list of internal transactions
    ///
    /// # Arguments
    /// * `ctx` - context instance
    /// * `address` - target wallet or contract address to get list of internal transactions
    pub fn get_list_internal_transactions(&self, ctx: &Context, address: &str) -> Result<Vec::<BSCInternalTransactionResponseSuccessVariantResult>, AppError>
    {
        type ResultType = BSCInternalTransactionResponseSuccessVariantResult;
        type JsonType = BSCTransactionResponse::<ResultType>;

        self.get_list_transactions::<ResultType, JsonType>(ctx, BSCApiResponseType::InternalTransaction, address)
    }

    /// Internal generic function supporting to get list of transactions for both
    /// normal and internal ones.
    ///
    /// __NOTE__: Get normal and internal transaction APIs are limited to maximum of
    /// 10,000 transactions per-se page * offset must be less than or equal to 10,000.
    /// So it doesn't make sense to use this API for address which has more than
    /// 10,000 transactions.
    fn get_list_transactions<R, J>(&self, ctx: &Context, api_req_type: BSCApiResponseType, address: &str) -> Result<Vec::<R>, AppError>
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
                BSCApiResponseType::NormalTransaction => "txlist",
                BSCApiResponseType::InternalTransaction => "txlistinternal"
            };
            let raw_url_str = format!("https://api.bscscan.com/api?module=account&action={action}&address={target_address}&startblock=0&endblock=99999999&page={page}&offset={offset}&sort=asc&apikey={api_key}", action=action, target_address=address, api_key=ctx.api_key, page=page_number, offset=OFFSET);

            let url = Url::parse(&raw_url_str);
            if let Err(_) = url {
                return Err(AppError::ErrorInternalUrlParsing);
            }

            match isahc::get(url.unwrap().as_str()) {
                Ok(mut res) => {
                    // early return for non-200 HTTP returned code
                    if res.status() != 200 {
                        return Err(AppError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                    }

                    // use the commented line, or just use what isahc provides conveniently
                    match res.json::<J>() {
                        Ok(json) => {
                            if json.status() == "1" {
                                // NOTE: unfortunate, we need to extract value from within enum
                                // https://stackoverflow.com/questions/34953711/unwrap-inner-type-when-enum-variant-is-known
                                match json.result() {
                                    GenericBSCTransactionResponseResult::Success(mut c) => {
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
                                    GenericBSCTransactionResponseResult::Failed(msg_opt) => {
                                        match msg_opt {
                                            Some(msg) => {
                                                return Err(AppError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=msg)));
                                            },
                                            None => {
                                                return Err(AppError::ErrorApiResponse(format!("un-expected error for success case")));
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
                                    return Err(AppError::ErrorApiResponse(format!("'{message}'", message=json.message())));
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return Err(AppError::ErrorJsonParsing(None));
                        }
                    }
                },
                Err(_) => {
                    return Err(AppError::ErrorSendingHttpRequest);
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
    pub fn get_balance_address(&self, ctx: &Context, address: &str) -> Result<U256, AppError> {
        let raw_url_str = format!("https://api.bscscan.com/api?module=account&action=balance&address={target_address}&apikey={api_key}", target_address=address, api_key=ctx.api_key);

        let url = Url::parse(&raw_url_str);
        if let Err(_) = url {
            return Err(AppError::ErrorInternalUrlParsing);
        }

        match isahc::get(url.unwrap().as_str()) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(AppError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<BSCBnbBalanceResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                GenericBSCBnbBalanceResponseResult::Success(bal) => Ok(bal),
                                GenericBSCBnbBalanceResponseResult::Failed(result_msg) => {
                                    return Err(AppError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)));
                                }
                            }
                        }
                        else {
                            return Err(AppError::ErrorApiResponse(format!("Message:{message}", message=json.message)));
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(AppError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(_) => {
                return Err(AppError::ErrorSendingHttpRequest);
            }
        }
    }

    /// Get BEP-20 transfer events for `address` API request.
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
    pub fn get_bep20_transfer_events_a(&self, ctx: &Context, address: &str) -> Result<Vec::<BSCBep20TokenTransferEventResponseSuccessVariantResult>, AppError> {
        let mut page_number = 1u8;
        let mut is_need_next_page = true;
        const OFFSET: usize = 2000;

        let mut ret_txs: Vec::<BSCBep20TokenTransferEventResponseSuccessVariantResult> = Vec::new();
     
        while is_need_next_page {
            let raw_url_str = format!("https://api.bscscan.com/api?module=account&action=tokentx&address={target_address}&page={page}&offset={offset}&startblock=0&endblock=999999999&sort=asc&apikey={api_key}", target_address=address, page={page_number}, offset=OFFSET, api_key=ctx.api_key);

            let url = Url::parse(&raw_url_str);
            if let Err(_) = url {
                return Err(AppError::ErrorInternalUrlParsing);
            }

            match isahc::get(url.unwrap().as_str()) {
                Ok(mut res) => {
                    // early return for non-200 HTTP returned code
                    if res.status() != 200 {
                        return Err(AppError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                    }

                    match res.json::<BSCBep20TokenTransferEventResponse>() {
                        Ok(json) => {
                            if json.status == "1" {
                                match json.result {
                                    GenericBSCBep20TokenTransferEventResponseResult::Success(mut c) => {
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
                                    GenericBSCBep20TokenTransferEventResponseResult::Failed(msg) => {
                                        return Err(AppError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=msg)));
                                    }
                                }
                            }
                            else {
                                // exact text as returned when empty "result" is returned
                                if json.message == "No transactions found" {
                                    break;
                                }
                                else {
                                    return Err(AppError::ErrorApiResponse(format!("'{message}'", message=json.message)));
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return Err(AppError::ErrorJsonParsing(None));
                        }
                    }
                },
                Err(_) => {
                    return Err(AppError::ErrorSendingHttpRequest);
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
