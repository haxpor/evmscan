use crate::prelude::*;
use crate::types::bsc_types::{
    BSCBnbLastPriceResponse,
    BSCBnbLastPriceResult,
    BSCBnbLastPrice
};
use crate::environ::Context;

use isahc::prelude::*;
use url::Url;

/// Stats namespace containing related APIs about stats
pub struct Stats;

impl Stats {
    /// Get BNB last price
    ///
    /// # Arguments
    /// * `ctx` - context instance
    pub fn get_bnb_last_price(&self, ctx: &Context) -> Result<BSCBnbLastPrice, BscError> {
        let raw_url_str = format!("https://api.bscscan.com/api?module=stats&action=bnbprice&apikey={api_key}", api_key=ctx.api_key);

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
                    return Err(BscError::ErrorApiResponse(format!("Error API resonse, with HTTP {code} returned", code=res.status().as_str())));
                }

                match res.json::<BSCBnbLastPriceResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                BSCBnbLastPriceResult::Success(lastprice_struct) => Ok(lastprice_struct),
                                BSCBnbLastPriceResult::Failed(result_msg) => Err(BscError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)))
                            }
                        }
                        else {
                            return Err(BscError::ErrorApiResponse(format!("Message:{message}", message=json.message)));
                        }
                    },
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(BscError::ErrorJsonParsing(None));
                    }
                }
            },
            Err(_) => {
                return Err(BscError::ErrorSendingHttpRequest);
            }
        }
    }
}
