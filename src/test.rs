/**
 * Note that test module needs bscscan.com's API key in which it needs to be
 * defined via environment variable namedly BSCSCAN_TEST_APIKEY.
 *
 * FIXME: be careful to run these tests as logic code to respect rate-limit
 * is not yet implemented.
 */
#[cfg(test)]
use crate::environ::Context;
use crate::bscscan;
use lazy_static::lazy_static;
use std::env;
use std::sync::Mutex;

lazy_static! {
    // Each test function will be executed on a different thread
    // (as checked via thread's id)
    // so we will lock each thread to wait for one test involving one
    // API usage possibly in multiple calls to finish first before moving on.
    //
    // This will ease and avoid rate-limit problem which is 5 API calls/s
    // as imposed by bscscan.com.
    static ref LOCK: Mutex<()> = Mutex::new(());

    // target address which is "BSC: Token Hub"
    static ref ADDRESS1: &'static str = "0x0000000000000000000000000000000000001004";

    // another target address which is "BSC: Relayer Incentivize"
    static ref ADDRESS2: &'static str = "0x0000000000000000000000000000000000001005";
}

/// Thanks to upstream suggestion as seen from
/// https://users.rust-lang.org/t/run-tests-sequentially/16397/7?u=haxpor
///
/// This is to avoid importing serial_test crate if we can do it without it.
macro_rules! serial_test {
    (fn $name: ident() $body: block) => {
        #[test]
        fn $name() {
            // NOTE: don't use `let _ = ...` as this will get lock immediately
            // `drop` thus unlock immediately.
            let _guard = LOCK.lock().unwrap();

            $body
        }
    };
}

/// This function will panic if BSCSCAN_TEST_APIKEY is not defined.
fn get_api_key_or_panic() -> String {
    env::var("BSCSCAN_TEST_APIKEY").expect("Error: define 'BSCSCAN_TEST_APIKEY' environment variable for testing")
}

fn create_context() -> Context {
    Context {
        api_key: get_api_key_or_panic()
    }
}

serial_test! {
    fn test_get_balance() {
        let ctx = create_context();

        // this is "BSC: Token Hub" contract address
        let _bnb_balance = bscscan::accounts().get_balance_address(&ctx, &ADDRESS1).unwrap();
    }
}

serial_test! {
    fn test_get_balance_multi() {
        let ctx = create_context();

        let txs = bscscan::accounts().get_balance_addresses_multi(&ctx, &[&ADDRESS1, &ADDRESS2]).unwrap();
        assert!(txs.len() == 2);
    }
}

// NOTE: only downside here is the time it takes to wait for response
// as it will max out at 10000 which is the maximum limitation that this
// API can return.
serial_test! {
    fn test_get_list_normal_txs() {
        let ctx = create_context();

        let txs = bscscan::accounts().get_list_normal_transactions(&ctx, &ADDRESS1).unwrap();

        // as API limits the maximum returns of this type of API to exactly 10000,
        // so we use to assert against it
        // NOTE: this assert and others signify that API really returns max 10000
        // records, and we have data filled in.
        assert!(txs.len() == 10000);
    }
}

// NOTE: same as of `test_get_list_normal_txs()`. This API will return
// max of 10000 records.
serial_test! {
    fn test_get_list_internal_txs() {
        let ctx = create_context();

        let txs = bscscan::accounts().get_list_internal_transactions(&ctx, &ADDRESS1).unwrap();
        assert!(txs.len() == 10000);
    }
}

serial_test! {
    fn test_get_bep20_transfer_events() {
        let ctx = create_context();

        let res = bscscan::accounts().get_bep20_transfer_events_a(&ctx, &ADDRESS1);
        assert!(res.is_err());      // as we use non-EOA address, it will be error
    }
}

}
