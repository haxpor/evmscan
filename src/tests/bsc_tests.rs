/**
 * Note that test module needs bscscan.com's API key in which it needs to be
 * defined via environment variable namely 'EVMSCAN_TEST_BSCSCAN_APIKEY'.
 *
 * NOTE: Logic code in these tests adhere to rate-limit by using serial_test! with
 * guard lock to test individual API method call which respects free tier rate-limit
 * internally.
 */
#[cfg(test)]
use crate::prelude::*;
use crate::environ::Context;
use crate::evmscan;
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
            // NOTE: don't use `let _ = ...` as this will get unlocked (drop) immediately.
            let _guard = LOCK.lock().unwrap();
            $body
        }
    };
}

/// This function will panic if EVMSCAN_TEST_BSCSCAN_APIKEY is not defined.
fn get_api_key_or_panic() -> String {
    env::var("EVMSCAN_TEST_BSCSCAN_APIKEY").expect("Error: define 'EVMSCAN_TEST_BSCSCAN_APIKEY' environment variable for testing")
}

serial_test! {
    fn test_get_balance() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let _bnb_balance = evmscan::accounts().get_balance_address(&ctx, &ADDRESS1).unwrap();
    }
}

serial_test! {
    fn test_get_balance_multi() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let txs = evmscan::accounts().get_balance_addresses_multi(&ctx, &[&ADDRESS1, &ADDRESS2]).unwrap();
        assert!(txs.len() == 2);
    }
}

// NOTE: only downside here is the time it takes to wait for response
// as it will max out at 10000 which is the maximum limitation that this
// API can return.
serial_test! {
    fn test_get_list_normal_txs() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let txs = evmscan::accounts().get_list_normal_transactions(&ctx, &ADDRESS1).unwrap();

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
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let txs = evmscan::accounts().get_list_internal_transactions(&ctx, &ADDRESS1).unwrap();
        assert!(txs.len() == 10000);
    }
}

// TODO: separate this Stats related API into separate test file
serial_test! {
    fn test_get_native_token_last_price() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        match evmscan::stats().get_native_token_last_price(&ctx) {
            Ok(res) => println!("{:#?}", res),
            Err(e) => panic!("{:?}:", e)
        }
    }
}

// TODO: separate this Contracts related API into seperate test file
serial_test! {
    fn test_contracts_get_abi_with_no_pretty_print() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, false);
        assert!(res.is_ok());
        assert!(res.unwrap().len() == 11842);   // exact number of character
                                                // from cleaned '\' char
    }
}

serial_test! {
    fn test_contracts_get_abi_with_pretty_print() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        let res = evmscan::contracts().get_abi(&ctx, &ADDRESS1, true);
        assert!(res.is_ok());
        assert!(res.unwrap().len() > 11842);
    }
}

serial_test! {
    fn test_contracts_get_verified_source_code() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        match evmscan::contracts().get_verified_source_code(&ctx, "0x1bA8D3C4c219B124d351F603060663BD1bcd9bbF") {
            Err(e) => panic!("{:?}", e),
            Ok(res) => {
                assert!(res.0.len() > 0);
                assert_eq!(res.1, false);       // not submitted as JSON format

                assert_eq!(res.0[0].constructor_arguments.len(), 5);
                assert_eq!(res.0[0].constructor_arguments[0], "000000000000000000000000ba5fe23f8a3a24bed3236f05f2fcf35fd0bf0b5c");
                assert_eq!(res.0[0].constructor_arguments[1], "000000000000000000000000Ee9546E92e6876EdF6a234eFFbD72d75360d91f0");
                assert_eq!(res.0[0].constructor_arguments[2], "0000000000000000000000000000000000000000000000000000000000000060");
                assert_eq!(res.0[0].constructor_arguments[3], "0000000000000000000000000000000000000000000000000000000000000000");
                assert_eq!(res.0[0].constructor_arguments[4], "0000000000000000000000000000000000000000000000000000000000000000");

                assert_eq!(res.0[0].optimization_used, true);
                assert_eq!(res.0[0].compiler_version, "v0.6.4+commit.1dca32f3");
                assert_eq!(res.0[0].runs, 200);
                assert_eq!(res.0[0].contract_name, "BEP20UpgradeableProxy");
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "Apache-2.0");
                assert_eq!(res.0[0].proxy, true);
                assert_eq!(res.0[0].implementation, "0xba5fe23f8a3a24bed3236f05f2fcf35fd0bf0b5c");
                assert_eq!(res.0[0].swarm_source, "ipfs://647a4fea61bb23cbda141d2cf5cadbd9ec022ccc2ffffaaa1b59b91259cfb8a1");
            }
        }
    }
}

serial_test! {
    // The verification step can be in json format
    // https://docs.soliditylang.org/en/v0.5.8/using-the-compiler.html#compiler-input-and-output-json-description
    // thus it can contain multiple files with the optional of settings.
    fn test_contracts_get_verified_source_code_json_format() {
        let ctx = Context::create(ChainType::BSC, get_api_key_or_panic());

        // ALERT: this address we used has been attacked due to vulnerability in the migrate()
        // function (gymdefi) as reported by BlockSec
        // https://twitter.com/BlockSecTeam/status/1512832398643265537?s=20&t=n5hETJrbgTAANKTpiwiMeg.
        //
        // Such address is used in our test as it's not relatively easy to find
        // such contract that has submitted as part of code verification onto bscscan
        // that used JSON format which allows multiple files to be there.
        //
        // So be vigilant, and careful not to interact with such contract address.
        match evmscan::contracts().get_verified_source_code(&ctx, "0x1befe6f3f0e8edd2d4d15cae97baee01e51ea4a4")         {
            Err(e) => panic!("{}", e),
            Ok(res) => {
                assert_eq!(res.0.len(), 7);     // 1 + 6 (1 is raw combined altogether, and 6 is other files there as part of JSON format)

                assert_eq!(res.1, true);    // this is submitted as JSON format, so it's true

                assert_eq!(res.0[0].constructor_arguments.len(), 0);
                assert_eq!(res.0[0].compiler_version, "v0.8.12+commit.f00d7308");
                assert_eq!(res.0[0].runs, 200);
                assert_eq!(res.0[0].evm_version, "Default");
                assert_eq!(res.0[0].license_type, "");
                assert_eq!(res.0[0].proxy, false);
                assert_eq!(res.0[0].implementation, "");
                assert_eq!(res.0[0].swarm_source, "");

                assert_eq!(res.0[1].contract_name, "contracts/LpMigration.sol");
                assert_eq!(res.0[2].contract_name, "@openzeppelin/contracts/access/Ownable.sol");
                assert_eq!(res.0[3].contract_name, "@openzeppelin/contracts/utils/Context.sol");
                assert_eq!(res.0[4].contract_name, "@openzeppelin/contracts/utils/math/SafeMath.sol");
                assert_eq!(res.0[5].contract_name, "@openzeppelin/contracts/security/ReentrancyGuard.sol");
                assert_eq!(res.0[6].contract_name, "@openzeppelin/contracts/token/ERC20/IERC20.sol");
            }
        }
    }
}
