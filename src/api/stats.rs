use crate::prelude::*;
use crate::types::evm_types::{
    EvmNativeTokenLastPriceResponse,
    EvmNativeTokenLastPriceResult,
    EvmNativeTokenLastPrice
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
    pub fn get_native_token_last_price(&self, ctx: &Context) -> Result<EvmNativeTokenLastPrice, EvmError> {
        let action = match ctx.chain {
            ChainType::BSC => "bnbprice",
            ChainType::Ethereum => "ethprice",
            ChainType::Polygon => "maticprice",
        };
        let raw_url_str = format!("{}/api?module=stats&action={}&apikey={api_key}", Context::get_prefix_url(ctx.chain), action, api_key=ctx.api_key);

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

                match res.json::<EvmNativeTokenLastPriceResponse>() {
                    Ok(json) => {
                        if json.status == "1" {
                            match json.result {
                                EvmNativeTokenLastPriceResult::Success(lastprice_struct) => Ok(lastprice_struct),
                                EvmNativeTokenLastPriceResult::Success_Polygon(lastprice_struct) => {
                                    // transfer field data into EvmNativeTokenLastPrice for
                                    // consistent
                                    Ok(EvmNativeTokenLastPrice {
                                        ethbtc: lastprice_struct.maticbtc,
                                        ethbtc_timestamp: lastprice_struct.maticbtc_timestamp,
                                        ethusd: lastprice_struct.maticusd,
                                        ethusd_timestamp: lastprice_struct.maticusd_timestamp,
                                    })
                                },
                                EvmNativeTokenLastPriceResult::Failed(result_msg) => Err(EvmError::ErrorApiResponse(format!("un-expected error for success case ({msg})", msg=result_msg)))
                            }
                        }
                        else {
                            return Err(EvmError::ErrorApiResponse(format!("Message:{message}", message=json.message)));
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
}
