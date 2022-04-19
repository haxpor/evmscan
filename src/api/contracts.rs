use crate::prelude::*;
use crate::types::bsc_types::*;
use crate::environ::Context;

use isahc::prelude::*;
use url::Url;

/// Contracts namespace containing related APIs about contracts
pub struct Contracts;

impl Contracts {
    /// Get contract ABI.
    ///
    /// # Arguments
    /// * `ctx` - Context
    /// * `address` - contract address to get ABI
    /// * `is_pretty_print` - whether or not to pretty print
    pub fn get_abi(self, ctx: &Context, address: &str, is_pretty_print: bool) -> Result<String, BscError> {
        let raw_url_str = format!("https://api.bscscan.com/api?module=contract&action=getabi&address={address}&apikey={api_key}", address=address, api_key=ctx.api_key);

        let url = match Url::parse(&raw_url_str) {
            Ok(res) => res,
            Err(_) => return Err(BscError::ErrorInternalUrlParsing),
        };

        let request = match isahc::Request::get(url.as_str())
            .version_negotiation(isahc::config::VersionNegotiation::http2())
            .body(()) {
            Ok(res) => res,
            Err(e) => return Err(BscError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
        };

        match isahc::send(request) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(BscError::ErrorApiResponse(format!("Error API response, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<BSCContractABIResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            // clean the text e.g. \
                            let cleaned_res = str::replace(&json.result, "\\", "");

                            if is_pretty_print {
                                // deserialize json string into object
                                // in order to pretty print it later
                                match serde_json::from_str::<Vec<BSCContractABIItem>>(&cleaned_res) {
                                    Ok(json_obj) => {
                                        match serde_json::to_string_pretty(&json_obj) {
                                            Ok(pretty_json_string) => Ok(pretty_json_string),
                                            Err(e) => return Err(BscError::ErrorInternalGeneric(Some(format!("create pretty JSON string from JSON object; err={}", e)))),
                                        }
                                    },
                                    Err(e) => {
                                        return Err(BscError::ErrorInternalGeneric(Some(format!("create JSON object from string; err={}", e))));
                                    },
                                }
                            }
                            else {
                                Ok(cleaned_res)
                            }
                        }
                        else {
                            return Err(BscError::ErrorApiResponse(format!("message:{}", json.message)));
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(BscError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("{}", e);
                return Err(BscError::ErrorSendingHttpRequest(Some(err_msg)));
            }
        }
    }

    /// Get verified contract's source code from the specified address.
    ///
    /// # Arguments
    /// * `ctx` - Context
    /// * `address` - contract address to get verified source code
    ///
    /// # Result
    /// Return `Vec<BSCContractSourceCode>` for success case, but mostly you just
    /// need to access the first item. It won't be the case of empty in returned
    /// `Vec`. This is due to the way API returned back. Even with mutiple source
    /// files uploaded and verified but it will altogether combined into one
    /// long string of code. But in the future, thing may be changed.
    pub fn get_verified_source_code(self, ctx: &Context, address: &str) -> Result<Vec<BSCContractSourceCode>, BscError> {
        let raw_url_str = format!("https://api.bscscan.com/api?module=contract&action=getsourcecode&address={address}&apikey={api_key}", address=address, api_key=ctx.api_key);

        let url = match Url::parse(&raw_url_str) {
            Ok(res) => res,
            Err(_) => return Err(BscError::ErrorInternalUrlParsing),
        };

        let request = match isahc::Request::get(url.as_str())
            .version_negotiation(isahc::config::VersionNegotiation::http2())
            .body(()) {
            Ok(res) => res,
            Err(e) => return Err(BscError::ErrorInternalGeneric(Some(format!("Error creating a HTTP request; err={}", e)))),
        };

        match isahc::send(request) {
            Ok(mut res) => {
                // early return for non-200 HTTP returned code
                if res.status() != 200 {
                    return Err(BscError::ErrorApiResponse(format!("Error API response, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<BSCContractSourceCodeResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                BSCContractSourceCodeResult::Success(contracts) => {
                                    if contracts.is_empty() {
                                        return Err(BscError::ErrorApiResponse(format!("source code is empty")));
                                    }

                                    // also check whether we made query for un-verified source code
                                    // in which response back from server still
                                    // has `status` field as "1". We need to check
                                    // against `ABI` field for exact string of
                                    // "Contract source code not verified".
                                    //
                                    // We treat this as an error as it doesn't make
                                    // sense to return with no meta-detail information
                                    // at all for non-verified source code even
                                    // though its `status` is "1".
                                    //
                                    // NOTE: we only have interest towards the first
                                    // item of `contracts` as this is the way
                                    // API returns to us although there are
                                    // multiple of source files uploaded.
                                    if contracts.first().unwrap().abi == "Contract source code not verified" {
                                        return Err(BscError::ErrorApiResponse(format!("made query to un-verified contract source code")));
                                    }
                                    else {
                                        return Ok(contracts);
                                    }
                                },
                                BSCContractSourceCodeResult::Failed(result_msg) => {
                                    return Err(BscError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)));
                                },
                            }
                        }
                        else {
                            // safely get text from "result" field
                            // this will ensure that the type of `json.result` is
                            // actually BSCContractSourceCodeResult which is
                            // the failed case.
                            let result_text = match json.result {
                                BSCContractSourceCodeResult::Failed(txt) => Some(txt),
                                _ => None,
                            };

                            match result_text {
                                Some(txt) => {
                                    return Err(BscError::ErrorApiResponse(format!("message:{}, result:{}", json.message, txt)));
                                },
                                None => {
                                    return Err(BscError::ErrorApiResponse(format!("message:{}", json.message)));
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(BscError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(e) => {
                let err_msg = format!("{}", e);
                return Err(BscError::ErrorSendingHttpRequest(Some(err_msg)));
            }
        }
    }
}
